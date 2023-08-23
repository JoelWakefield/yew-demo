use yew::{function_component, html, Callback, Html};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[at("/post/:id")]
    Post { id: String },
    #[at("/*path")]
    Misc { path: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
pub fn secure() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
      <div>
        <h1>{ "Secure" }</h1>
        <button {onclick}>{ "Go home" }</button>
        <Link<Route> to={Route::Home}>
          { "click here to go home" }
        </Link<Route>>
      </div>
    }
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "home" }</h1> },
        Route::Secure => html! { <Secure /> },
        Route::Post { id } => html! {
          <p>{format!("post: {}", id)}</p>
        },
        Route::Misc { path } => html! {
          <p>{format!("matched path: {}", path)}</p>
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
