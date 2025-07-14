use yew::prelude::*;
use yew_router::prelude::Link;
use crate::Route;

use gloo_net::http::Request;
use web_sys::{console, HtmlInputElement};
use wasm_bindgen_futures::spawn_local;
use serde::Serialize;


#[derive(Clone, Default, Debug, PartialEq, serde::Serialize)]
struct RegisterData {
    username: String,
    email: String,
    password1: String,
    password2: String,
}

#[function_component(Register)]
pub fn register() -> Html {
    let form_data = use_state(RegisterData::default);
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
                "password1" => data.password1 = value,
                "password2" => data.password2 = value,
                _ => {}
            }
            console::log_1(&format!("Form state: {:?}", data).into());

            form_data.set(data);
        })
    };
    let on_submit = {
    let form_data = form_data.clone();
    let is_loading = is_loading.clone();

    Callback::from({
        let is_loading_outer = is_loading.clone(); // ✅ cloned before the closure
        move |e: SubmitEvent| {
            e.prevent_default();

            if *is_loading_outer {
                return;
            }

            is_loading_outer.set(true);
            let data = (*form_data).clone();
            let is_loading_inner = is_loading_outer.clone(); // ✅ for use inside async

            spawn_local(async move {
                let res = Request::post("http://127.0.0.1:8000/api/register/")
                    .header("Content-Type", "application/json")
                    .json(&data)
                    .unwrap()
                    .send()
                    .await;

                    match res {
                        Ok(response) => {
                        console::log_1(&format!("✅ Response Status: {}", response.status()).into());
                    }
                    Err(err) => {
                        console::error_1(&format!("❌ Network error: {:?}", err).into());
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
                    <h1 class="title">{ "Welcom" }</h1>
                    <p class="subtitle">{ "Please Sign Up" }</p>
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
                            id="password1"
                            name="password1"
                            class="form-input"
                            oninput={on_input.clone()}
                            placeholder="Enter your password"
                            required={true}
                        />
                    </div>

                    <div class="form-group">
                        <label for="confirmPassword" class="form-label">
                            { "Confirm Password" }
                        </label>
                        <input
                            type="password"
                            id="password2"
                            name="password2"
                            class="form-input"
                            oninput={on_input.clone()}
                            placeholder="Confirm your password"
                            required={true}
                        />
                    </div>

                    <button type="submit" class="btn btn-primary">
                       <span class="btn-content">{ "Create Account" }</span>
                    </button>
                </form>
                    <div class="footer">
                        <p class="footer-text">
                            { "Already have an account? " }
                            <Link<Route> to={Route::Login}>
                            <p>{ "Sign in" }</p>
                            </Link<Route>>
                        </p>
                    </div>
            </div>
        </div>
        </div>
    }
}
