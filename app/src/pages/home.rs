use yew::prelude::{function_component, html};

use crate::components::button::Button;

#[function_component(Home)]
pub fn home() -> Html {
    // This comment never ends up in the binary
    html! {
        <div>
            <div>{ "Hello Capabilities!" }</div>
            <Button />
        </div>
    }
}
