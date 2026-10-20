mod actions;
mod app;
mod components;
mod data;
mod i18n;
mod icons;
mod settings;
mod stream;
mod util;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| leptos::view! { <App /> });
}
