use yew::prelude::{function_component, html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct PageProps {
    pub id: String,
}

#[function_component(Page)]
pub fn page(props: &PageProps) -> Html {
    let id = &props.id;
    html! {
        <div>{ format!("Hello Jndl Page: {}", id) }</div>
    }
}
