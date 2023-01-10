use crate::routes::routes_switch;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::MainRoute;

#[function_component(Container)]
pub fn container() -> Html {
    html! {
        <BrowserRouter>
            <Switch<MainRoute> render={routes_switch} />
        </BrowserRouter>
    }
}
