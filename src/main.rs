use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
mod utils;
use pages::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/server/:n")]
    Server { n: String },
}
fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Server { n } => html! { <Server n={n}/> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>

            <main class="p-4">
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
