use dioxus::prelude::*;

use crate::route::Route;

/// Where a [NavLink] is rendered; picks the styling.
#[derive(Clone, PartialEq)]
pub enum NavLinkVariant {
    /// The desktop header row.
    Desktop,
    /// The mobile dropdown menu.
    Mobile,
}

/// A section anchor (`#features`, …) requested from another page. The home
/// page watches this signal and scrolls to the section once it has rendered,
/// because a plain `#hash` link only works when the section is on the current
/// page.
pub static SECTION_SCROLL_TARGET: GlobalSignal<Option<&'static str>> =
    GlobalSignal::new(|| None);

/// Scroll the element with `id` into view (no-op on the server).
///
/// Scrolls instantly: a smooth `scrollIntoView()` can be interrupted or
/// silently dropped (e.g. when the router scrolls back to the top of the
/// page during navigation), which would leave the user at the wrong place.
/// The element's `scroll-margin-top` (see `main.css`) still clears the fixed
/// navbar.
pub fn scroll_to_id(id: &str) {
    #[cfg(target_arch = "wasm32")]
    if let Some(element) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(id))
    {
        let mut options = web_sys::ScrollIntoViewOptions::new();
        options.behavior(web_sys::ScrollBehavior::Instant);
        element.scroll_into_view_with_scroll_into_view_options(&options);
    }
}

/// One section link of the navbar.
#[component]
pub fn NavLink(
    href: &'static str,
    label: &'static str,
    variant: NavLinkVariant,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let class = match variant {
        NavLinkVariant::Desktop => {
            "text-sm font-medium text-gray-300 hover:text-white transition-colors"
        }
        NavLinkVariant::Mobile => {
            "py-3 text-base font-medium text-gray-300 hover:text-white transition-colors"
        }
    };

    let nav = use_navigator();
    let is_home = matches!(use_route::<Route>(), Route::Home {});

    // Element listeners can't take an Option handler directly — wrap it
    // (same pattern as dioxus' own `Link` component). Section anchors point
    // at the home page: from any other route, remember the target and
    // navigate home instead of letting the browser append a dead `#hash` to
    // the current URL.
    let onclick = move |event: MouseEvent| {
        if href.len() > 1 && href.starts_with('#') {
            event.prevent_default();
            SECTION_SCROLL_TARGET.with_mut(|target| *target = Some(&href[1..]));
            if !is_home {
                nav.push(Route::Home {});
            }
        }
        if let Some(handler) = onclick {
            handler.call(event);
        }
    };

    rsx! {
        a { href, class, onclick, "{label}" }
    }
}
