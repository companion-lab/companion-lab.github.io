use dioxus::prelude::*;

/// One feature tile of the features grid: tinted icon badge, title, description.
#[component]
pub fn FeatureCard(
    icon: &'static str,
    title: &'static str,
    desc: &'static str,
    /// Theme color key: "purple", "blue", "indigo", or "pink".
    color: &'static str,
) -> Element {
    let (badge_bg, icon_color) = match color {
        "blue" => ("bg-blue-500/20", "text-blue-400"),
        "indigo" => ("bg-indigo-500/20", "text-indigo-400"),
        "pink" => ("bg-pink-500/20", "text-pink-400"),
        _ => ("bg-purple-500/20", "text-purple-400"),
    };

    rsx! {
        div {
            class: "glass-card p-6 sm:p-8 rounded-2xl group reveal active",
            div {
                class: "w-12 h-12 rounded-xl flex items-center justify-center mb-6 group-hover:scale-110 transition-all duration-300 {badge_bg}",
                i { class: "fa-solid {icon} text-2xl {icon_color}" }
            }
            h3 { class: "text-xl font-semibold text-white mb-3", "{title}" }
            p { class: "text-gray-400 text-sm leading-relaxed", "{desc}" }
        }
    }
}
