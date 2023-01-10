use yew::prelude::*;
mod container;
mod navs;

use container::Container;
use navs::Navigator;

#[function_component(LayoutComponent)]
pub fn layout() -> Html {
    html! {
        <>
        <Navigator/>
        <Container/>
        </>
    }
}
