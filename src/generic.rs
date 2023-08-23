use std::fmt::Display;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props<T>
where
    T: PartialEq,
{
    pub data: T,
}

#[function_component]
pub fn GenericComponent<T>(props: &Props<T>) -> Html
where
    T: PartialEq + Display,
{
    html! {
      <p>
      { &props.data }
      </p>
    }
}
