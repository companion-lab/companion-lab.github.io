use dioxus::prelude::*;

use crate::components::{Cta, Features, Footer, Hero, Specs};

/// Home page
#[component]
pub fn Home() -> Element {
    rsx! {
        // Ambient Background Lighting (clipped to the viewport so the blobs
        // can never widen the page and enable horizontal scrolling)
        div {
            class: "fixed inset-0 overflow-hidden pointer-events-none",
            div { class: "bg-blob bg-purple-600/30 w-[300px] h-[300px] sm:w-[500px] sm:h-[500px] top-[-100px] left-[-100px]" }
            div { class: "bg-blob bg-blue-600/20 w-[350px] h-[350px] sm:w-[600px] sm:h-[600px] top-[20%] right-[-150px] sm:right-[-200px]" }
            div { class: "bg-blob bg-indigo-600/20 w-[250px] h-[250px] sm:w-[400px] sm:h-[400px] bottom-[10%] left-[10%]" }
        }

        Hero {}
        Features {}
        Specs {}
        Cta {}
        Footer {}
    }
}
