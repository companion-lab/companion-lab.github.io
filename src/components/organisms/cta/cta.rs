use dioxus::prelude::*;

use crate::{
    components::molecules::DownloadButtons,
    route::Route,
};

#[component]
pub fn Cta() -> Element {
    rsx! {
        section {
            class: "py-16 sm:py-24 relative overflow-hidden",
            id: "download",
            div { class: "absolute inset-0 bg-gradient-to-r from-purple-900/20 to-blue-900/20" }
            div {
                class: "max-w-4xl mx-auto px-6 relative z-10 text-center reveal active",
                h2 { class: "text-3xl sm:text-4xl md:text-5xl font-bold mb-6", "Ready to meet your companion?" }
                p { class: "text-lg sm:text-xl text-gray-300 mb-10", "Download for your platform and run the setup." }
                DownloadButtons { centered: true }
                p { class: "mt-8 text-sm text-gray-400",
                    "New here? Read the "
                    Link {
                        to: Route::Docs {},
                        class: "text-purple-400 underline decoration-purple-400/40 underline-offset-2 transition-colors hover:text-purple-300",
                        "documentation"
                    }
                    " to get started."
                }
            }
        }
    }
}
