mod api;
mod components;
mod hooks;
mod model;
mod pages;
mod routes;

use yew::prelude::*;

use routes::AppRouter;

#[function_component(App)]
fn app() -> Html {
    html! {
        <AppRouter />
    }
}

fn main() {
    yew::start_app::<App>();
}
