mod app;
mod pages;
mod theme;

use leptos::{logging::log, *};

use crate::app::App;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    log!("csr mode - mounting to body");

    leptos::mount_to_body(|| view! { <App/> })
}
