use dioxus::prelude::*;

use crate::components::{DiscordButton, GithubButton};

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "border-t border-white/10 bg-[#02000a] pt-12 sm:pt-16 pb-8",
            div {
                class: "max-w-7xl mx-auto px-6",
                div {
                    class: "grid grid-cols-2 md:grid-cols-4 gap-8 sm:gap-12 mb-12",
                    div {
                        class: "col-span-1 md:col-span-1",
                        a {
                            href: "#",
                            class: "flex items-center gap-2 mb-4",
                            i { class: "fa-solid fa-cube text-purple-400" }
                            span { class: "text-xl font-bold text-white", "Companion" }
                        }
                        p { class: "text-sm text-gray-500 mb-6", "A transparent VRM avatar on your desktop — the visual body of your AI agent." }
                        div {
                            class: "flex gap-4",
                            GithubButton {},
                            DiscordButton {}
                        }
                    }
                    div {
                        h4 { class: "text-white font-semibold mb-4", "Product" }
                        ul {
                            class: "space-y-2 text-sm text-gray-500",
                            li { a { href: "#", class: "hover:text-purple-400 transition-colors", "Download" } }
                            li { a { href: "#features", class: "hover:text-purple-400 transition-colors", "Features" } }
                            li { a { href: "#", class: "hover:text-purple-400 transition-colors", "Settings" } }
                        }
                    }
                    div {
                        h4 { class: "text-white font-semibold mb-4", "Tech" }
                        ul {
                            class: "space-y-2 text-sm text-gray-500",
                            li { a { target: "_blank", href: "https://github.com/tauri-apps/tauri", class: "hover:text-purple-400 transition-colors", "Tauri v2" } }
                            li { a { target: "_blank", href: "https://github.com/mrdoob/three.js", class: "hover:text-purple-400 transition-colors", "Three.js" } }
                            li { a { target: "_blank", href: "https://github.com/moonshine-ai/moonshine", class: "hover:text-purple-400 transition-colors", "Moonshine" } }
                            li { a { target: "_blank", href: "https://github.com/supertone-oss-archive/supertonic", class: "hover:text-purple-400 transition-colors", "Supertonic" } }
                        }
                    }
                    div {
                        h4 { class: "text-white font-semibold mb-4", "Legal" }
                        ul {
                            class: "space-y-2 text-sm text-gray-500",
                            li { a { href: "#", class: "hover:text-purple-400 transition-colors", "Privacy" } }
                            li { a { href: "#", class: "hover:text-purple-400 transition-colors", "Terms" } }
                        }
                    }
                }
                div {
                    class: "border-t border-white/10 pt-8 flex flex-col md:flex-row items-center justify-between gap-4 text-xs text-gray-600",
                    p { "\u{00A9} 2026 Companion. All rights reserved." }
                    div {
                        class: "flex gap-4",
                        span {
                            "Built with "
                            i { class: "fa-solid fa-heart text-purple-500/50 mx-1" }
                            " for the desktop."
                        }
                    }
                }
            }
        }
    }
}
