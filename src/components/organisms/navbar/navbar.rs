use dioxus::prelude::*;

use crate::{
    components::atoms::{IconButton, NavLink, NavLinkVariant},
    route::Route,
};

const LOGO_IMAGE: Asset = asset!("/assets/icon.png");

const GITHUB_URL: &str = "https://github.com/companion-vr";
const DISCORD_URL: &str = "https://discord.gg/G46H7B9x";

const NAV_ITEMS: [(&str, &str); 3] = [
    ("#features", "Features"),
    ("#specs", "Tech Specs"),
    ("#download", "Download"),
];

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
                Link {
                    to: Route::Home {},
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
                    for (href, label) in NAV_ITEMS {
                        NavLink { href, label, variant: NavLinkVariant::Desktop }
                    }
                    Link {
                        to: Route::Docs {},
                        class: "text-sm font-medium text-gray-300 hover:text-white transition-colors",
                        "Docs"
                    }
                    div {
                        class: "flex gap-4",
                        IconButton { href: GITHUB_URL, icon: "fab fa-github", label: "GitHub" }
                        IconButton { href: DISCORD_URL, icon: "fab fa-discord", label: "Discord" }
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
                    for (href, label) in NAV_ITEMS {
                        NavLink {
                            href,
                            label,
                            variant: NavLinkVariant::Mobile,
                            onclick: move |_| menu_open.set(false),
                        }
                    }
                    Link {
                        to: Route::Docs {},
                        class: "py-3 text-base font-medium text-gray-300 hover:text-white transition-colors",
                        onclick: move |_| menu_open.set(false),
                        "Docs"
                    }
                    div {
                        class: "flex gap-4 pt-4 border-t border-white/10",
                        IconButton { href: GITHUB_URL, icon: "fab fa-github", label: "GitHub" }
                        IconButton { href: DISCORD_URL, icon: "fab fa-discord", label: "Discord" }
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
