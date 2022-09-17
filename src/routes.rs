use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Routable, Switch};

use crate::pages::{Home, PageNotFound, Pokemon};

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/pokemon/:name")]
    Pokemon { name: String },
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Pokemon { name } => {
            html! { <Pokemon name={name.clone()} /> }
        }
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}

#[function_component(AppRouter)]
pub fn app_router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
