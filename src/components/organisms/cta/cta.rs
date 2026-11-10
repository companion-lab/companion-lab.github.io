use dioxus::prelude::*;

use crate::{components::molecules::DownloadButtons, route::Route};

#[component]
pub fn Cta() -> Element {
    rsx! {
        section {
            class: "py-16 sm:py-24 relative overflow-hidden",
            id: "download",
            div {
                class: "max-w-4xl mx-auto px-6 relative z-10",
                div {
                    class: "hud-frame hud-grid reveal active border border-white/10 bg-[#08040f]/70 px-6 py-12 text-center sm:px-14 sm:py-16",
                    div { class: "hud-frame-inner",
                        div {
                            class: "flex items-center justify-center gap-2 font-mono text-[11px] uppercase tracking-[0.22em] text-purple-300/90",
                            span { class: "hud-dot", "aria-hidden": "true" }
                            "action // deploy companion"
                        }
                        h2 {
                            class: "mt-6 text-3xl sm:text-4xl md:text-5xl font-black uppercase tracking-tight text-white",
                            "Ready to meet your "
                            span { class: "text-purple-500", "companion" }
                            "?"
                        }
                        p { class: "mt-5 text-lg sm:text-xl text-gray-300", "Download for your platform and run the setup." }
                        div { class: "mt-10", DownloadButtons { centered: true } }
                        p {
                            class: "mt-8 font-mono text-[11px] uppercase tracking-[0.15em] text-gray-500",
                            "new here? read the "
                            Link {
                                to: Route::Docs {},
                                class: "text-purple-400 underline decoration-purple-500/40 underline-offset-4 transition-colors hover:text-purple-300",
                                "documentation"
                            }
                            " to get started"
                        }
                    }
                }
            }
        }
    }
}
