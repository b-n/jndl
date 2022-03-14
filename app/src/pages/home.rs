use yew::prelude::{function_component, html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>{ "Hello App!" }</div>
    }
}
