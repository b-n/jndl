use core::fmt;
use yew::{ContextProvider, Children, html, function_component, Properties, use_context};

#[derive(Clone, PartialEq, Properties)]
pub struct ApiProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(APIProvider)]
pub fn api_provider(props: &ApiProviderProps) -> Html {
    let ctx = API::new();
    html! {
        <ContextProvider<API> context={ctx}>
            { for props.children.iter() }
        </ContextProvider<API>>
    }
}

pub fn use_api() -> API {
    use_context::<API>().expect("no ctx found")
}

pub struct Page {
    pub id: String
}

impl fmt::Display for Page {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
       f.debug_struct("Page")
           .field("id", &self.id)
           .finish()
   }
}


#[derive(Clone)]
pub struct API {
    user_id: Option<String>,
}

impl PartialEq for API {
    fn eq(&self, rhs: &Self) -> bool { 
        self.user_id == rhs.user_id
    }
}

impl API {
    pub fn new() -> Self {
        API {
           user_id: None, 
        }
    }

    pub fn find_page<'a>(&self, id: &'a str) -> Page {
        Page {
            id: id.to_string()
        } 
    }

    pub fn get<'a, F>(&self, uri: &'a str, callback: CallbackFn) -> Result<(), ()> 
        where F: FnOnce()
    {
        let req = Request::builder()
            .method(Method::GET)
            .uri(uri);

        self.client.request(req)
    }
}
