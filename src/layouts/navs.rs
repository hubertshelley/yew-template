use crate::routes::MainRoute;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct Navigator {
    is_active: bool,
}

pub enum NavigatorMsg {
    ToggleNavbar,
}

impl Component for Navigator {
    type Message = NavigatorMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            is_active: false,
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NavigatorMsg::ToggleNavbar => {
                self.is_active = !self.is_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self { is_active, .. } = *self;

        let active_class = if !is_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "Yew Blog" }</h1>

                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={ctx.link().callback(|_| NavigatorMsg::ToggleNavbar)}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <Link<MainRoute> classes={classes!("navbar-item")} to={MainRoute::Home}>
                            { "Home" }
                        </Link<MainRoute>>
                        <Link<MainRoute> classes={classes!("navbar-item")} to={MainRoute::Secure}>
                            { "Secure" }
                        </Link<MainRoute>>

                        // <div class="navbar-item has-dropdown is-hoverable">
                        //     <div class="navbar-link">
                        //         { "More" }
                        //     </div>
                        //     <div class="navbar-dropdown">
                        //         <Link<MainRoute> classes={classes!("navbar-item")} to={MainRoute::NotFound}>
                        //             { "NotFound" }
                        //         </Link<MainRoute>>
                        //     </div>
                        // </div>
                    </div>
                </div>
            </nav>
        }
    }
}
