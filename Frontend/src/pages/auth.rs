use yew::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use js_sys::Date;
use base64::engine::general_purpose;
use base64::Engine as _;
use serde::{Deserialize};
use serde_json::json;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct UserData {
    pub id: usize,
    pub username: String,
    pub email: String,
}

#[derive(Deserialize)]
struct RefreshResponse {
    access: String,
}

#[hook]
pub fn use_auth_check() -> (UseStateHandle<bool>, UseStateHandle<Option<String>>) {
    let is_logged_in = use_state(|| false);
    let username = use_state(|| None);

    {
        let is_logged_in = is_logged_in.clone();
        let username = username.clone();

        use_effect_with((), move |_| {
            let is_logged_in = is_logged_in.clone();
            let username = username.clone();

            spawn_local(async move {
                if let Ok(mut access_token) = LocalStorage::get::<String>("access_token") {
                    if is_token_expired(&access_token) {
                        console::log_1(&"🔄 Access token expired, trying refresh...".into());

                        if let Ok(refresh_token) = LocalStorage::get::<String>("refresh_token") {
                            let refresh_body = json!({ "refresh_token": refresh_token });

                            match Request::post("http://127.0.0.1:8000/api/refresh/")
                                .header("Content-Type", "application/json")
                                .body(serde_json::to_string(&refresh_body).unwrap())
                                .expect("Failed to build request")  // ✅ FIX: unwrap before calling `.send()`
                                .send()
                                .await
                            {
                                Ok(response) if response.status() == 200 => {
                                    match response.json::<RefreshResponse>().await {
                                        Ok(new_tokens) => {
                                            console::log_1(&"✅ Access token refreshed.".into());
                                            LocalStorage::set("access_token", new_tokens.access.clone()).unwrap();
                                            access_token = new_tokens.access;
                                        }
                                        Err(_) => {
                                            console::log_1(&"❌ Failed to parse refresh response.".into());
                                            return;
                                        }
                                    }
                                }
                                _ => {
                                    console::log_1(&"❌ Refresh token invalid or expired.".into());
                                    return;
                                }
                            }
                        } else {
                            console::log_1(&"⚠️ No refresh token found".into());
                            return;
                        }
                    }

                    // 🧠 Use current access token to fetch user info
                    match Request::get("http://127.0.0.1:8000/api/user/")
                        .header("Authorization", &format!("Bearer {}", access_token))
                        .send()
                        .await
                    {
                        Ok(response) if response.status() == 200 => {
                            if let Ok(data) = response.json::<serde_json::Value>().await {
                                if let Some(name) = data["username"].as_str() {
                                    username.set(Some(name.to_string()));
                                    is_logged_in.set(true);
                                }
                            }
                        }
                        _ => {
                            console::log_1(&"❌ Failed to fetch user info.".into());
                            is_logged_in.set(false);
                            username.set(None);
                        }
                    }
                } else {
                    console::log_1(&"⚠️ No access token found".into());
                    is_logged_in.set(false);
                    username.set(None);
                }
            });

            || () // cleanup
        });
    }

    (is_logged_in, username)
}

/// Checks if a JWT is expired by parsing the `exp` field.
fn is_token_expired(token: &str) -> bool {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return true;
    }

    if let Ok(decoded) = general_purpose::STANDARD.decode(parts[1]) {
        if let Ok(json_str) = String::from_utf8(decoded) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&json_str) {
                if let Some(exp) = json["exp"].as_u64() {
                    let now = Date::now() / 1000.0;
                    return now > exp as f64;
                }
            }
        }
    }
    true // Default to expired if anything fails
}