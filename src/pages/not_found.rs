use dioxus::prelude::*;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    let navigator = use_navigator();
    let url = format!("/{}", segments.join("/"));

    rsx! {
        div {
            class: "flex flex-col items-center justify-center h-screen",
            h1 { class: "text-6xl font-bold mb-4", "404" }
            p { class: "text-xl text-gray-600 pb-5", "Page {url} not found" }
            button {
                class: "px-6 py-3 rounded-xl bg-gradient-to-r from-purple-600 to-blue-600 font-semibold text-white shadow-[0_0_30px_rgba(168,85,247,0.4)] hover:shadow-[0_0_40px_rgba(59,130,246,0.6)] hover:scale-105 transition-all duration-300 flex items-center justify-center gap-3 group",
                onmousedown: move |_| {
                    navigator.go_back();
                },
                "Go Back"
            }
        }
    }
}
