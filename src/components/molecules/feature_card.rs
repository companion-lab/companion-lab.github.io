use dioxus::prelude::*;

/// One cell of the features data-grid: square hairline icon badge, indexed
/// like a module register (`f.01`), lightens on hover instead of lifting.
#[component]
pub fn FeatureCard(
    icon: &'static str,
    title: &'static str,
    desc: &'static str,
    /// Theme color key: "purple", "blue", "indigo", or "pink".
    color: &'static str,
    /// Two-digit register index, rendered mono in the cell corner.
    index: String,
) -> Element {
    let icon_color = match color {
        "blue" => "text-blue-400",
        "indigo" => "text-indigo-400",
        "pink" => "text-pink-400",
        _ => "text-purple-400",
    };

    rsx! {
        div {
            class: "group relative bg-[#08040f]/85 p-6 sm:p-7 transition-colors hover:bg-[#140a26] reveal active",
            span {
                class: "absolute top-3 right-4 font-mono text-[10px] tracking-[0.2em] text-gray-600 transition-colors group-hover:text-purple-400/70",
                "f.{index}"
            }
            div {
                class: "mb-5 flex h-10 w-10 items-center justify-center border border-white/10 bg-white/[0.03] transition-colors group-hover:border-purple-500/40",
                i { class: "fa-solid {icon} text-lg {icon_color}", "aria-hidden": "true" }
            }
            h3 { class: "mb-2 text-base font-semibold uppercase tracking-wide text-white", "{title}" }
            p { class: "text-sm leading-relaxed text-gray-400", "{desc}" }
        }
    }
}
