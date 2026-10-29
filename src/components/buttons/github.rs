use dioxus::prelude::*;

#[component]
pub fn GithubButton() -> Element {
    rsx! {
        a {
            href: "https://github.com/companion-vr",
            class: "w-8 h-8 rounded-full bg-white/5 flex items-center justify-center text-gray-400 hover:text-white hover:bg-white/10 transition-colors",
            i { class: "fab fa-github" }
        }
    }
}
