use yew::prelude::*;
use yew_router::prelude::*;

use crate::views::secure::Secure;
use crate::views::home::Home;
use crate::components::not_found::NotFound;

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn routes_switch(routes: MainRoute) -> Html {
    match routes {
        MainRoute::Home => html! {
            <Home />
        },
        MainRoute::Secure => html! {
            <Secure />
        },
        MainRoute::NotFound => html! {
            <NotFound />
        },
    }
}
