mod pages;

use yew::prelude::*;
use yew_router::prelude::*;

use pages::login::Login;
use pages::register::Register;
use pages::notes::Notes;
use pages::home::Home;

#[derive(Routable, PartialEq, Clone, Debug)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/notes")]
    Notes,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Login => html! { <Login /> },
        Route::Register => html! { <Register /> },
        Route::Notes => html! { <Notes /> },
        Route::NotFound => html! { <h1>{ "404 Not Found" }</h1> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
