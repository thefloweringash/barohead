use std::panic;

mod app;
mod data;
mod widgets;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    yew::Renderer::<crate::app::App>::new().render();
}
