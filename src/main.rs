use blog::{App, AppProps};
use leptos::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug).expect("[Init log level] Error");
    console_error_panic_hook::set_once();

    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
