use yew::prelude::{function_component, html};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div>{ "404 Buddy" }</div>
    }
}
