use dioxus::prelude::*;

#[component]
pub fn DownloadButtons(centered: bool) -> Element {
    let detected = {
        #[cfg(feature = "server")]
        {
            let res = use_server_future(move || async move {
                use crate::server::detect_os;
                detect_os().await.unwrap_or(0)
            })?;
            res().unwrap_or(0)
        }
        #[cfg(not(feature = "server"))]
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

    let order: [(i32, &str, &str); 3] = [
        (0, "fa-brands fa-windows", "Windows"),
        (1, "fa-brands fa-apple", "macOS"),
        (2, "fa-brands fa-linux", "Linux"),
    ];
    let mut ordered = order.to_vec();
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
                a {
                    href: "https://github.com/companion/companion/releases/latest",
                    class: if code == detected {
                        "w-full sm:w-auto px-6 py-3 rounded-xl bg-gradient-to-r from-purple-600 to-blue-600 font-semibold text-white shadow-[0_0_30px_rgba(168,85,247,0.4)] hover:shadow-[0_0_40px_rgba(59,130,246,0.6)] hover:scale-105 transition-all duration-300 flex items-center justify-center gap-3 group"
                    } else {
                        "w-full sm:w-auto px-6 py-3 rounded-xl glass-panel font-medium text-gray-300 hover:text-white hover:bg-white/5 transition-all duration-300 flex items-center justify-center gap-3 group"
                    },
                    i { class: "{icon} text-xl group-hover:-translate-y-1 transition-transform" }
                    span { "{label}" }
                }
            }
        }
    }
}
