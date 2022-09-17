use yew::{function_component, html};

#[function_component(Spinner)]
pub fn spinner() -> Html {
    let classes = "spinner";

    html! {
        <div class={classes}>
        </div>
    }
}
