use dioxus::prelude::*;

use crate::{
    components::{DiscordButton, GithubButton},
    route::Route,
};

const LOGO_IMAGE: Asset = asset!("/assets/icon.png");

#[component]
pub fn Navbar() -> Element {
    let mut menu_open = use_signal(|| false);

    rsx! {
        header {
            id: "navbar",
            class: "fixed w-full top-0 z-50 bg-[#05010f]/90 backdrop-blur-md border-b border-white/10 transition-all duration-300",
            div {
                class: "max-w-7xl mx-auto px-6 h-16 sm:h-20 flex items-center justify-between",

                // Logo
                a {
                    href: "#",
                    class: "flex items-center gap-2 group",
                    img {
                        src: LOGO_IMAGE,
                        class: "h-10 w-10 sm:h-12 sm:w-12",
                    }
                    span {
                        class: "text-lg sm:text-xl font-bold tracking-tight text-white",
                        "Companion "
                    }
                }

                // Desktop Nav
                nav {
                    class: "hidden md:flex items-center gap-8",
                    a { href: "#features", class: "text-sm font-medium text-gray-300 hover:text-white transition-colors", "Features" }
                    a { href: "#specs", class: "text-sm font-medium text-gray-300 hover:text-white transition-colors", "Tech Specs" }
                    a { href: "#download", class: "text-sm font-medium text-gray-300 hover:text-white transition-colors", "Download" }
                    div {
                        class: "flex gap-4",
                        GithubButton {},
                        DiscordButton {}
                    }
                }

                // Mobile Menu Toggle
                button {
                    class: "md:hidden w-10 h-10 flex items-center justify-center text-gray-300 hover:text-white transition-colors",
                    "aria-label": "Toggle navigation menu",
                    onclick: move |_| menu_open.set(!menu_open()),
                    i { class: if menu_open() { "fa-solid fa-xmark text-xl" } else { "fa-solid fa-bars text-xl" } }
                }
            }

            // Mobile Menu
            if menu_open() {
                nav {
                    class: "md:hidden px-6 pb-6 pt-2 flex flex-col border-t border-white/10 bg-[#05010f]/95 backdrop-blur-md",
                    a {
                        href: "#features",
                        class: "py-3 text-base font-medium text-gray-300 hover:text-white transition-colors",
                        onclick: move |_| menu_open.set(false),
                        "Features"
                    }
                    a {
                        href: "#specs",
                        class: "py-3 text-base font-medium text-gray-300 hover:text-white transition-colors",
                        onclick: move |_| menu_open.set(false),
                        "Tech Specs"
                    }
                    a {
                        href: "#download",
                        class: "py-3 text-base font-medium text-gray-300 hover:text-white transition-colors",
                        onclick: move |_| menu_open.set(false),
                        "Download"
                    }
                    div {
                        class: "flex gap-4 pt-4 border-t border-white/10",
                        GithubButton {},
                        DiscordButton {}
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
