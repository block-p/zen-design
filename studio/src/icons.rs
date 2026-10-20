use icondata_core::IconData;
use leptos::prelude::*;

/// Renders a Lucide icon from its static SVG data. Keeping icons as embedded
/// Rust data removes the JavaScript icon package from the runtime entirely.
#[component]
pub fn Icon(
    data: &'static IconData,
    #[prop(default = 16)] size: u32,
    #[prop(default = 2.0)] stroke_width: f64,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=size
            height=size
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width=stroke_width
            stroke-linecap="round"
            stroke-linejoin="round"
            class=class
            aria-hidden="true"
            inner_html=data.data
        ></svg>
    }
}
