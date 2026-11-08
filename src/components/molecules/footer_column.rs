use dioxus::prelude::*;

use crate::{
    components::atoms::SECTION_SCROLL_TARGET,
    route::Route,
};

/// A single link in a [FooterColumn].
#[derive(Clone, PartialEq)]
pub struct FooterLink {
    pub label: &'static str,
    pub href: &'static str,
    /// External links open in a new tab.
    pub external: bool,
}

/// One footer column: a heading above a stacked list of links.
///
/// - Links starting with `/` are internal routes and render as a router
///   `Link`.
/// - Links starting with a `#section` anchor point at the home page: the click
///   is intercepted so it navigates home and scrolls to the section instead
///   of appending a dead `#hash` to the current URL.
/// - Links shaped `/legal#section` do the same for the legal page (the router
///   drops fragments when it parses a link target).
/// - Everything else is a plain anchor.
#[component]
pub fn FooterColumn(title: &'static str, links: Vec<FooterLink>) -> Element {
    let nav = use_navigator();
    let is_home = matches!(use_route::<Route>(), Route::Home {});
    let is_legal = matches!(use_route::<Route>(), Route::Legal {});

    rsx! {
        div {
            h3 { class: "text-white font-semibold mb-4", "{title}" }
            ul {
                class: "space-y-2 text-sm text-gray-400",
                for link in links {
                    li {
                        if link.href.len() > 1 && link.href.starts_with('#') {
                            a {
                                href: link.href,
                                class: "hover:text-purple-400 transition-colors",
                                onclick: move |event: MouseEvent| {
                                    event.prevent_default();
                                    SECTION_SCROLL_TARGET.with_mut(|target| {
                                        *target = Some(&link.href[1..]);
                                    });
                                    if !is_home {
                                        nav.push(Route::Home {});
                                    }
                                },
                                "{link.label}"
                            }
                        } else if let Some(fragment) = link.href.strip_prefix("/legal#") {
                            // Same trick as the home `#section` links above,
                            // pointed at the legal page: the router would drop
                            // the fragment of `/legal#privacy`, so navigate by
                            // hand and let the page scroll via the signal.
                            a {
                                href: link.href,
                                class: "hover:text-purple-400 transition-colors",
                                onclick: move |event: MouseEvent| {
                                    event.prevent_default();
                                    SECTION_SCROLL_TARGET.with_mut(|target| {
                                        *target = Some(fragment);
                                    });
                                    if !is_legal {
                                        nav.push(Route::Legal {});
                                    }
                                },
                                "{link.label}"
                            }
                        } else if link.href.starts_with('/') {
                            Link {
                                to: link.href,
                                class: "hover:text-purple-400 transition-colors",
                                "{link.label}"
                            }
                        } else {
                            a {
                                href: link.href,
                                target: if link.external { "_blank" },
                                class: "hover:text-purple-400 transition-colors",
                                "{link.label}"
                            }
                        }
                    }
                }
            }
        }
    }
}
