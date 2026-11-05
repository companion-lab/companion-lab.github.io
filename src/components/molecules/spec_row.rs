use dioxus::prelude::*;

/// One label/value row of the tech-specs panel. The parent `ul` draws the
/// row dividers (`divide-y divide-white/5`).
#[component]
pub fn SpecRow(label: &'static str, value: &'static str) -> Element {
    rsx! {
        li {
            class: "flex flex-col gap-1 py-3 sm:flex-row sm:items-center sm:justify-between",
            span { class: "text-gray-400", "{label}" }
            span { class: "font-medium text-white sm:text-right", "{value}" }
        }
    }
}
