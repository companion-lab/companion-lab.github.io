use dioxus::prelude::*;

use crate::{
    components::atoms::{ActionButton, ButtonVariant},
    config::RELEASES_URL,
    latest_release::DownloadCatalog,
};

/// Platform download buttons plus a version selector, with the detected
/// platform first and highlighted.
///
/// Each button links directly to the matching installer zip of the release
/// picked in the version selector — the latest stable release until the
/// visitor pins another one — resolved at runtime and shared via
/// [DownloadCatalog], with the releases page as the fallback link. The
/// selector itself only renders once the resolver fetch knows more than one
/// release, so bots, SSR renders and rate-limited visits just see the three
/// platform buttons.
///
/// On mobile the buttons stack full-width; on desktop they sit in a row —
/// centered when `centered` is set (the CTA section), flush-left otherwise
/// (the hero).
#[component]
pub fn DownloadButtons(centered: bool) -> Element {
    let detected = {
        // Server-rendered detection (fullstack builds, native binary only).
        #[cfg(all(feature = "server", not(target_arch = "wasm32")))]
        {
            let res = use_server_future(move || async move {
                use crate::server::detect_os;
                detect_os().await.unwrap_or(0)
            })?;
            res().unwrap_or(0)
        }
        #[cfg(not(all(feature = "server", not(target_arch = "wasm32"))))]
        {
            #[cfg(target_arch = "wasm32")]
            {
                let ua = web_sys::window()
                    .and_then(|w| w.navigator().user_agent().ok())
                    .unwrap_or_default();
                if ua.contains("Mac") {
                    1
                } else if ua.contains("Linux") || ua.contains("X11") {
                    2
                } else {
                    0
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                0
            }
        }
    };

    let mut catalog: Signal<DownloadCatalog> = use_context();

    // Work on a plain clone instead of holding the read guard: it keeps the
    // borrows out of rsx and lets the onchange handler below write the same
    // signal when the visitor picks a version.
    let snapshot = catalog.read().clone();
    let current = snapshot.resolved();
    let current_tag = current
        .map(|release| release.tag.clone())
        .unwrap_or_default();
    let latest_tag = snapshot
        .latest()
        .map(|release| release.tag.clone())
        .unwrap_or_default();
    let pickable = snapshot.releases.len() > 1;

    let platforms: [(i32, &str, &str); 3] = [
        (0, "fa-brands fa-windows", "Windows"),
        (1, "fa-brands fa-apple", "macOS"),
        (2, "fa-brands fa-linux", "Linux"),
    ];
    let mut ordered = platforms.to_vec();
    ordered.sort_by_key(|platform| platform.0 != detected);

    // Resolve each platform's link (selected release asset, or the releases
    // page as the fallback) outside rsx — `let` is not an rsx statement, and
    // the shared [DownloadCatalog] keeps hrefs in sync with it.
    let items = ordered
        .into_iter()
        .map(|(code, icon, label)| {
            let href = current
                .and_then(|release| release.urls[code as usize].clone())
                .unwrap_or_else(|| RELEASES_URL.to_string());
            (code, icon, label, href)
        })
        .collect::<Vec<_>>();

    // One dropdown row per release: its tag, marked "latest" (the default
    // pick) or "pre-release" where GitHub flags one.
    let versions = snapshot
        .releases
        .iter()
        .map(|release| {
            let label = if release.tag == latest_tag {
                format!("{} · latest", release.tag)
            } else if release.prerelease {
                format!("{} · pre-release", release.tag)
            } else {
                release.tag.clone()
            };
            (release.tag.clone(), release.tag == current_tag, label)
        })
        .collect::<Vec<_>>();

    let layout = if centered {
        "sm:items-center sm:justify-center"
    } else {
        "sm:items-center"
    };

    rsx! {
        div {
            class: "w-full flex flex-col sm:flex-row items-stretch {layout} gap-3 flex-wrap",
            for (code, icon, label, href) in items {
                ActionButton {
                    variant: if code == detected { ButtonVariant::Primary } else { ButtonVariant::Glass },
                    href: href,
                    class: "w-full sm:w-auto",
                    i { class: "{icon} text-xl group-hover:-translate-y-1 transition-transform" }
                    span { "{label}" }
                }
            }
            if pickable {
                div {
                    class: "w-full sm:w-auto px-6 py-3 rounded-xl glass-panel font-medium text-gray-300 hover:bg-white/5 transition-all duration-300 flex items-center justify-center gap-3",
                    i { class: "fa-solid fa-code-branch text-xl text-gray-500" }
                    select {
                        class: "bg-transparent font-medium text-gray-300 cursor-pointer outline-none",
                        "aria-label": "App version",
                        title: "Choose which release the download links point at",
                        onchange: move |event| {
                            let tag = event.value();
                            catalog.with_mut(|catalog| {
                                catalog.selected = (!tag.is_empty()).then_some(tag);
                            });
                        },
                        for (tag, is_current, label) in versions {
                            option {
                                value: "{tag}",
                                selected: is_current,
                                "{label}"
                            }
                        }
                    }
                }
            }
        }
    }
}
