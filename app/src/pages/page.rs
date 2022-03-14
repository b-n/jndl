use yew::prelude::{function_component, html, Properties};

use crate::lib::api::{use_api};

#[derive(Clone, PartialEq, Properties)]
pub struct PageProps {
    pub id: String,
}

#[function_component(Page)]
pub fn page(props: &PageProps) -> Html {
    let id = &props.id;
    let client = use_api(); 

    html! {
        <div>
            <p>{ format!("Hello Jndl Page: {}", id) }</p>
            <p>{ format!("Page retrieved from client: {}", client.find_page(&id)) }</p>
        </div>
    }
}
