use dioxus::prelude::*;

use crate::{
    components::atoms::{Icon, IconButton, NavLink, NavLinkVariant},
    config::{COMMUNITY_GITHUB_URL, DISCORD_URL},
    route::Route,
};

// The 128px rendition: the nav displays it at 40-48px, so serving
// the 512px icon.png would triple the bytes for no visible gain
// (Lighthouse image-size-responsive).
const LOGO_IMAGE: Asset = asset!("/assets/logo-128.png");

const NAV_ITEMS: [(&str, &str); 3] = [
    ("#features", "Features"),
    ("#specs", "Tech Specs"),
    ("#download", "Download"),
];

#[component]
pub fn Navbar() -> Element {
    let mut menu_open = use_signal(|| false);
    let toggle_icon = if menu_open() { "xmark" } else { "bars" };

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
                        alt: "Companion logo",
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
                        to: Route::Plugins {},
                        class: "text-sm font-medium text-gray-300 hover:text-white transition-colors",
                        "Plugins"
                    }
                    Link {
                        to: Route::Docs {},
                        class: "text-sm font-medium text-gray-300 hover:text-white transition-colors",
                        "Docs"
                    }
                    div {
                        class: "flex gap-4",
                        IconButton { href: COMMUNITY_GITHUB_URL, icon: "github", label: "GitHub" }
                        IconButton { href: DISCORD_URL, icon: "discord", label: "Discord" }
                    }
                }

                // Mobile Menu Toggle
                button {
                    class: "md:hidden w-10 h-10 flex items-center justify-center text-gray-300 hover:text-white transition-colors",
                    "aria-label": "Toggle navigation menu",
                    onclick: move |_| menu_open.set(!menu_open()),
                    Icon { icon: toggle_icon, class: "text-xl" }
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
                        to: Route::Plugins {},
                        class: "py-3 text-base font-medium text-gray-300 hover:text-white transition-colors",
                        onclick: move |_| menu_open.set(false),
                        "Plugins"
                    }
                    Link {
                        to: Route::Docs {},
                        class: "py-3 text-base font-medium text-gray-300 hover:text-white transition-colors",
                        onclick: move |_| menu_open.set(false),
                        "Docs"
                    }
                    div {
                        class: "flex gap-4 pt-4 border-t border-white/10",
                        IconButton { href: COMMUNITY_GITHUB_URL, icon: "github", label: "GitHub" }
                        IconButton { href: DISCORD_URL, icon: "discord", label: "Discord" }
                    }
                }
            }
        }

        main {
            Outlet::<Route> {}
        }
    }
}
