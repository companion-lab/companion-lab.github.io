use dioxus::prelude::*;

use crate::components::{ActionButton, ButtonVariant};

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    let navigator = use_navigator();
    let url = format!("/{}", segments.join("/"));

    rsx! {
        div {
            class: "flex flex-col items-center justify-center h-screen",
            h1 { class: "text-6xl font-bold mb-4", "404" }
            p { class: "text-xl text-gray-400 pb-5", "Page {url} not found" }
            ActionButton {
                variant: ButtonVariant::Primary,
                onmousedown: move |_| navigator.go_back(),
                "Go Back"
            }
        }
    }
}
