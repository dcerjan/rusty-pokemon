use yew::{function_component, html};

#[function_component(PageNotFound)]
pub fn page_not_found() -> Html {
    html! {
        <div>
          { "Page Not Found" }
        </div>
    }
}
