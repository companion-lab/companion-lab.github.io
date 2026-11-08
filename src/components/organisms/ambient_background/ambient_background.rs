use dioxus::prelude::*;

/// Ambient background lighting behind the page content. Fixed to the viewport
/// and clipped, so the blobs can never widen the page and enable horizontal
/// scrolling.
///
/// Futuristic motion (defined in `assets/main.css`): the three blobs drift
/// slowly, a faint radar glow rotates behind the content, and two parallax
/// dot layers rise like data motes. All of it is transform-only CSS
/// animation, disabled under `prefers-reduced-motion` — the layers then just
/// render their static look.
#[component]
pub fn AmbientBackground() -> Element {
    rsx! {
        div {
            class: "fixed inset-0 overflow-hidden pointer-events-none",
            div { class: "bg-blob blob-a bg-purple-600/30 w-[300px] h-[300px] sm:w-[500px] sm:h-[500px] top-[-100px] left-[-100px]" }
            div { class: "bg-blob blob-b bg-blue-600/20 w-[350px] h-[350px] sm:w-[600px] sm:h-[600px] top-[20%] right-[-150px] sm:right-[-200px]" }
            div { class: "bg-blob blob-c bg-indigo-600/20 w-[250px] h-[250px] sm:w-[400px] sm:h-[400px] bottom-[10%] left-[10%]" }
            div { class: "motes-layer motes-far", "aria-hidden": "true" }
            div { class: "motes-layer motes-near", "aria-hidden": "true" }
            div { class: "radar-glow", "aria-hidden": "true" }
        }
    }
}
