use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::MainRoute;

#[function_component(Secure)]
pub fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&MainRoute::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}
