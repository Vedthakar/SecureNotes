use yew::prelude::*;
use yew_router::prelude::Link;
use yew_router::prelude::use_navigator;
use crate::Route;
use gloo_net::http::Request;
use web_sys::{console, HtmlInputElement};
use wasm_bindgen_futures::spawn_local;
use serde::Serialize;
use gloo_storage::{Storage, LocalStorage};
use web_sys::window;


#[derive(Clone, Default, Debug, PartialEq, serde::Serialize)]
struct LoginData {
    username: String,
    email: String,
    password: String,
}

#[function_component(Login)]
pub fn login() -> Html {
    let form_data = use_state(LoginData::default);
    let is_loading = use_state(|| false);
    let on_input = {
        let form_data = form_data.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let name = input.name();
            let value = input.value();

            let mut data = (*form_data).clone();
            match name.as_str() {
                "username" => data.username = value,
                "email" => data.email = value,
                "password" => data.password = value,
                _ => {}
            }
            console::log_1(&format!("Form state: {:?}", data).into());

            form_data.set(data);
        })
    };
    let on_submit = {
        let form_data = form_data.clone();
        let is_loading = is_loading.clone();
        let navigator = use_navigator().unwrap();

        Callback::from({
            let is_loading_outer = is_loading.clone();
            move |e: SubmitEvent| {
                e.prevent_default();

                if *is_loading_outer {
                    return;
                }

                is_loading_outer.set(true);
                let data = (*form_data).clone();
                let is_loading_inner = is_loading_outer.clone();
                let navigator = navigator.clone();

                spawn_local(async move {
                    let res = Request::post("http://127.0.0.1:8000/api/login/")
                        .header("Content-Type", "application/json")
                        .json(&data)
                        .unwrap()
                        .send()
                        .await;

                    match res {
                        Ok(resp) => {
                            console::log_1(&format!("✅ Response Status: {}", resp.status()).into());

                            let json_result = resp.json::<serde_json::Value>().await;
                            match json_result {
                                Ok(body) => {
                                    if let Some(tokens) = body.get("tokens") {
                                        if let Some(access_token) = tokens.get("access").and_then(|v| v.as_str()) {
                                            match LocalStorage::set("access_token", access_token) {
                                                Ok(_) => console::log_1(&"✅ Stored access token".into()),
                                                Err(e) => console::error_1(&format!("❌ Failed to store access token: {:?}", e).into()),
                                            }
                                        }

                                        if let Some(refresh_token) = tokens.get("refresh").and_then(|v| v.as_str()) {
                                            match LocalStorage::set("refresh_token", refresh_token) {
                                                Ok(_) => console::log_1(&"✅ Stored refresh token".into()),
                                                Err(e) => console::error_1(&format!("❌ Failed to store refresh token: {:?}", e).into()),
                                            }
                                        }
                                    }
                                     navigator.push(&Route::Notes);
                                }
                                Err(e) => {
                                    console::error_1(&format!("❌ JSON parse error: {:?}", e).into());
                                }
                            }

                        }
                        Err(err) => {
                            console::error_1(&format!("❌ Request failed: {:?}", err).into());
                        }
                    }

                    is_loading_inner.set(false);
                });
            }
        })
};
    html! {
        <div class = "auth-page">
        <div class="bg-gradient">
            <div class="bg-decoration-1"></div>
            <div class="bg-decoration-2"></div>
            
            <div class="card">
                <Link<Route> to={Route::Home}>
                    <p class="footer-text"> {"<- back"} </p>
                </Link<Route>>
                <div class="header">
                    <h1 class="title">{ "Welcom Back" }</h1>
                    <p class="subtitle">{ "Please Login" }</p>
                </div>

                <form onsubmit={on_submit} action="#" method="POST">
                    <div class="form-group">
                        <label for="username" class="form-label">
                            { "Username" }
                        </label>
                        <input
                            type="text"
                            id="username"
                            name="username"
                            class="form-input"
                            oninput={on_input.clone()}
                            placeholder="Enter your username"
                            required={true}
                        />
                    </div>

                    <div class="form-group">
                        <label for="email" class="form-label">
                            { "Email Address" }
                        </label>
                        <input
                            type="email"
                            id="email"
                            name="email"
                            class="form-input"
                            oninput={on_input.clone()}
                            placeholder="Enter your email address"
                            required={true}
                        />
                    </div>

                    <div class="form-group">
                        <label for="password" class="form-label">
                            { "Password" }
                        </label>
                        <input
                            type="password"
                            id="password"
                            name="password"
                            class="form-input"
                            oninput={on_input.clone()}
                            placeholder="Enter your password"
                            required={true}
                        />
                    </div>

                    <button type="submit" class="btn btn-primary">
                       <span class="btn-content">{ "Login" }</span>
                    </button>
                </form>
                    <div class="footer">
                        <p class="footer-text">
                            { "Don't have an account? " }
                            <Link<Route> to={Route::Register}>
                            <p>{ "Sign Up" }</p>
                            </Link<Route>>
                        </p>
                    </div>
            </div>
        </div>
        </div>
    }
}
