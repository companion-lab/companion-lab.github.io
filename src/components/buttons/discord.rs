use dioxus::prelude::*;

#[component]
pub fn DiscordButton() -> Element {
    rsx! {
        a {
            href: "https://discord.gg/G46H7B9x",
            class: "w-8 h-8 rounded-full bg-white/5 flex items-center justify-center text-gray-400 hover:text-white hover:bg-white/10 transition-colors",
            i { class: "fab fa-discord" }
        }
    }
}
