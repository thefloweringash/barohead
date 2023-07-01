use std::panic;

mod app;
mod widgets;

use crate::app::App;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    yew::Renderer::<App>::new().render();
}
