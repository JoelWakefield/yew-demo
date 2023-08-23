use yew::{function_component, html, Callback, Html, Properties};
use yew_router::prelude::*;

mod generic;
mod router;

use router::{switch, Route};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_name_entry: Callback<String>,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    props.on_name_entry.emit(String::from("Bob"));
    html! { "HELLO?" }
}

// Then supply the prop
#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
