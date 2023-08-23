use yew::{function_component, html, Callback, Html, Properties};

mod generic;
use generic::GenericComponent;

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
    let on_name_entry: Callback<String> = Callback::from(move |name: String| {
        let greeting = format!("HEY, {}!", name);
        web_sys::console::log_1(&greeting.into());
    });

    html! {
        <>
            <HelloWorld {on_name_entry} />
            <GenericComponent<i32> data=123 />
            <GenericComponent<String> data={"foo".to_string()} />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
