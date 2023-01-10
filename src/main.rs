mod app;
mod components;
mod layouts;
mod routes;
mod views;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
