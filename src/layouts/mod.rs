use yew::prelude::*;
mod container;
mod navs;

#[warn(unused_imports)]
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
