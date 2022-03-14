use yew::prelude::{function_component, html, Html};
use yew_router::prelude::{BrowserRouter, Routable, Switch};

mod pages;

use pages::home::Home;
use pages::page::Page;
use pages::errors::NotFound;

#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/page/:id")]
    Page { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Page { id } => html! { <Page id={id.clone()}/> },
        Route::NotFound => html! { <NotFound /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
