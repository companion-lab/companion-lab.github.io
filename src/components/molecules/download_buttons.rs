use dioxus::prelude::*;

use crate::{
    components::atoms::{ActionButton, ButtonVariant},
    config::RELEASES_URL,
};

/// Platform download buttons, with the detected platform first and highlighted.
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

    let platforms: [(i32, &str, &str); 3] = [
        (0, "fa-brands fa-windows", "Windows"),
        (1, "fa-brands fa-apple", "macOS"),
        (2, "fa-brands fa-linux", "Linux"),
    ];
    let mut ordered = platforms.to_vec();
    ordered.sort_by(|a, b| (a.0 != detected).cmp(&(b.0 != detected)));

    let layout = if centered {
        "sm:items-center sm:justify-center"
    } else {
        "sm:items-center"
    };

    rsx! {
        div {
            class: "w-full flex flex-col sm:flex-row items-stretch {layout} gap-3 flex-wrap",
            for (code, icon, label) in ordered {
                ActionButton {
                    variant: if code == detected { ButtonVariant::Primary } else { ButtonVariant::Glass },
                    href: RELEASES_URL,
                    class: "w-full sm:w-auto",
                    i { class: "{icon} text-xl group-hover:-translate-y-1 transition-transform" }
                    span { "{label}" }
                }
            }
        }
    }
}
