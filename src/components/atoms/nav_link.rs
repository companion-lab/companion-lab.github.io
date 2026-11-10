use dioxus::prelude::*;

// Used by the wasm-only entry-hash retry below; unused on the server.
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::Closure, JsCast};

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
pub static SECTION_SCROLL_TARGET: GlobalSignal<Option<&'static str>> = GlobalSignal::new(|| None);

/// Scroll the element with `id` into view (no-op on the server).
///
/// Scrolls instantly: a smooth `scrollIntoView()` can be interrupted or
/// silently dropped (e.g. when the router scrolls back to the top of the
/// page during navigation), which would leave the user at the wrong place.
/// The element's `scroll-margin-top` (see `main.css`) still clears the fixed
/// navbar.
#[cfg_attr(not(target_arch = "wasm32"), allow(unused_variables))]
pub fn scroll_to_id(id: &str) {
    #[cfg(target_arch = "wasm32")]
    if let Some(element) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(id))
    {
        let options = web_sys::ScrollIntoViewOptions::new();
        options.set_behavior(web_sys::ScrollBehavior::Instant);
        element.scroll_into_view_with_scroll_into_view_options(&options);
    }
}

/// Land on the URL's `#hash` target after the first render, reliably.
///
/// Three things fight a plain "read the hash and scroll once" effect:
/// - the browser's native fragment scroll runs while the document is still
///   empty, long before the WASM app renders;
/// - the router rewrites the URL to the canonical route — no fragment — while
///   booting, so `location.hash` can be empty by effect time (reload is a
///   reliable loser here), and scrolls to the top;
/// - that scroll-to-top can also land *after* an effect-driven scroll.
///
/// `index.html` therefore stashes the entry hash in `window.__initialHash`
/// before the WASM bundle runs, and this re-applies the scroll for ~3.5s:
/// each pass is a no-op while the target is already in view (so late layout
/// shifts get corrected), an unknown hash just times out, and the loop stops
/// the moment the user scrolls themselves. No-op on the server.
pub fn scroll_to_entry_hash() {
    #[cfg(target_arch = "wasm32")]
    {
        let Some(window) = web_sys::window() else {
            return;
        };
        // Mark user-initiated scrolling (wheel, touch, keyboard) so the retry
        // loop below stops instead of fighting the user for the viewport.
        let listener = Closure::<dyn FnMut()>::new(|| USER_SCROLLED.with(|flag| flag.set(true)));
        for event in ["wheel", "touchstart", "keydown"] {
            let _ =
                window.add_event_listener_with_callback(event, listener.as_ref().unchecked_ref());
        }
        // Lives for the page (one tiny closure, leaked on purpose).
        listener.forget();
        retry_entry_hash_scroll(20);
    }
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    /// Set as soon as the user scrolls during [scroll_to_entry_hash]'s window.
    static USER_SCROLLED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// One pass of [scroll_to_entry_hash]'s bounded retry loop.
#[cfg(target_arch = "wasm32")]
fn retry_entry_hash_scroll(remaining: u8) {
    const RETRY_DELAY_MS: i32 = 300;

    let Some(window) = web_sys::window() else {
        return;
    };
    if USER_SCROLLED.with(|flag| flag.get()) {
        return;
    }

    // Prefer the pre-wasm stash; fall back to the (possibly rewritten) URL.
    let stash = js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str("__initialHash"))
        .ok()
        .and_then(|value| value.as_string());
    let hash = stash
        .filter(|hash| hash.len() > 1)
        .or_else(|| window.location().hash().ok())
        .unwrap_or_default();

    if hash.len() > 1 {
        // The router rewrites the URL without the fragment while booting;
        // put it back so the address bar matches where the page is heading.
        if window
            .location()
            .hash()
            .map(|current| current.is_empty())
            .unwrap_or(false)
        {
            let _ = window.location().set_hash(&hash);
        }

        let target = &hash[1..];
        let in_view = window
            .document()
            .and_then(|document| document.get_element_by_id(target))
            .map(|element| {
                let top = element.get_bounding_client_rect().top();
                // Landed state: the target rests just below the fixed navbar
                // (`scroll-margin-top`, see main.css). Late layout shifts move
                // it up (content above shrank) — anything clearly higher than
                // the margin means "not landed yet", so keep correcting.
                top > 60.0 && top < 120.0
            })
            .unwrap_or(false);
        if !in_view {
            scroll_to_id(target);
        }
    }

    if remaining > 1 {
        // Keep the timer alive until it fires (tiny one-off leak, one per
        // retry) — same pattern as the copy-button revert in markdown.rs.
        let callback = Closure::<dyn FnMut()>::new(move || retry_entry_hash_scroll(remaining - 1));
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            RETRY_DELAY_MS,
        );
        callback.forget();
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
