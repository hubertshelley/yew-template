use crate::routes::MainRoute;
use yew::prelude::*;
use yew_router::prelude::*;
#[function_component(Navigator)]
pub fn navigator() -> Html {
    html! {
        <Link<MainRoute> to={MainRoute::Home}>{ "click here to go home" }</Link<MainRoute>>
    }
}
