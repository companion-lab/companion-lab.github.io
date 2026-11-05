use dioxus::prelude::*;

use crate::{
    components::{
        atoms::IconButton,
        molecules::{FooterColumn, FooterLink},
    },
    route::Route,
};

const GITHUB_URL: &str = "https://github.com/companion-vr";
const DISCORD_URL: &str = "https://discord.gg/G46H7B9x";

#[component]
pub fn Footer() -> Element {
    let columns: [(&str, Vec<FooterLink>); 4] = [
        (
            "Docs",
            vec![
                FooterLink { label: "Installation", href: "/docs/installation", external: false },
                FooterLink { label: "Usage", href: "/docs/usage", external: false },
                FooterLink { label: "Creating plugins", href: "/docs/creating-plugins", external: false },
                FooterLink { label: "Troubleshooting", href: "/docs/troubleshooting", external: false },
            ],
        ),
        (
            "Product",
            vec![
                FooterLink { label: "Download", href: "#download", external: false },
                FooterLink { label: "Features", href: "#features", external: false },
                FooterLink { label: "Settings", href: "#", external: false },
            ],
        ),
        (
            "Tech",
            vec![
                FooterLink { label: "Tauri v2", href: "https://github.com/tauri-apps/tauri", external: true },
                FooterLink { label: "Three.js", href: "https://github.com/mrdoob/three.js", external: true },
                FooterLink { label: "Moonshine", href: "https://github.com/moonshine-ai/moonshine", external: true },
                FooterLink { label: "Supertonic", href: "https://github.com/supertone-oss-archive/supertonic", external: true },
            ],
        ),
        (
            "Legal",
            vec![
                FooterLink { label: "Privacy", href: "#", external: false },
                FooterLink { label: "Terms", href: "#", external: false },
            ],
        ),
    ];

    rsx! {
        footer {
            class: "border-t border-white/10 bg-[#02000a] pt-12 sm:pt-16 pb-8",
            div {
                class: "max-w-7xl mx-auto px-6",
                div {
                    class: "grid grid-cols-2 md:grid-cols-5 gap-8 sm:gap-12 mb-12",
                    div {
                        class: "col-span-2 md:col-span-1",
                        Link {
                            to: Route::Home {},
                            class: "flex items-center gap-2 mb-4",
                            i { class: "fa-solid fa-cube text-purple-400" }
                            span { class: "text-xl font-bold text-white", "Companion" }
                        }
                        p { class: "text-sm text-gray-500 mb-6", "A transparent VRM avatar on your desktop — the visual body of your AI agent." }
                        div {
                            class: "flex gap-4",
                            IconButton { href: GITHUB_URL, icon: "fab fa-github", label: "GitHub" }
                            IconButton { href: DISCORD_URL, icon: "fab fa-discord", label: "Discord" }
                        }
                    }
                    for (title, links) in columns {
                        FooterColumn { title, links }
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
