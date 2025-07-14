use yew::prelude::*;
use yew_router::prelude::Link;
use crate::Route;


#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class = "home-page">
        <div class="bg-gradient">
            <div class="bg-decoration-1"></div>
            <div class="bg-decoration-2"></div>

            <div class="card">
                <div class="header">
                    <h1 class="title">{"Welcome"}</h1>
                    <p class="subtitle">{"Choose your path to get started"}</p>
                </div>
                
                <div class="button-container">
                    <Link<Route> to={Route::Register} classes="btn btn-primary">
                        <div class="btn-content">
                            <svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                                      d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                            </svg>
                            <span>{"Create Account"}</span>
                        </div>
                    </Link<Route>>

                    <Link<Route> to={Route::Login} classes="btn btn-secondary">
                        <div class="btn-content">
                            <svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                      d="M11 16l-4-4m0 0l4-4m-4 4h14m-5 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h7a3 3 0 013 3v1" />
                            </svg>
                            <span>{"Sign In"}</span>
                        </div>
                    </Link<Route>>
                </div>

                <div class="divider">
                    <div class="divider-line"></div>
                    <span class="divider-text">{"or"}</span>
                </div>

                <div class="guest-section">
                    <p class="guest-text">{"Continue as guest?"}</p>
                    <button class="btn-ghost">{"Browse without account"}</button>
                </div>

                <div class="footer">
                    <p class="footer-text">{"Secure • Private • Modern"}</p>
                </div>
            </div>
        </div>
        </div>
    }
}
