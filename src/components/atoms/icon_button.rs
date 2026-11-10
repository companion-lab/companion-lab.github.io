use dioxus::prelude::*;

/// Small round social-icon link (GitHub, Discord) used in the navbar and footer.
#[component]
pub fn IconButton(href: &'static str, icon: &'static str, label: &'static str) -> Element {
    rsx! {
        a {
            href,
            "aria-label": label,
            class: "w-8 h-8 rounded-full bg-white/5 flex items-center justify-center text-gray-400 hover:text-white hover:bg-white/10 transition-colors",
            i { class: "{icon}" }
        }
    }
}
