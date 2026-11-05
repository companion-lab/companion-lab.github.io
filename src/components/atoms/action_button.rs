use dioxus::prelude::*;

/// Visual style of an [ActionButton].
#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    /// Filled purple→blue gradient with a glow — the primary call to action.
    Primary,
    /// Subtle translucent panel — secondary actions.
    Glass,
}

/// The large rounded call-to-action control. Renders an `<a>` when `href` is
/// set, otherwise a `<button>`.
#[component]
pub fn ActionButton(
    variant: ButtonVariant,
    href: Option<&'static str>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    class: Option<&'static str>,
    children: Element,
) -> Element {
    let variant_class = match variant {
        ButtonVariant::Primary => "bg-gradient-to-r from-purple-600 to-blue-600 font-semibold text-white shadow-[0_0_30px_rgba(168,85,247,0.4)] hover:shadow-[0_0_40px_rgba(59,130,246,0.6)] hover:scale-105",
        ButtonVariant::Glass => "glass-panel font-medium text-gray-300 hover:text-white hover:bg-white/5",
    };
    let class = format!(
        "px-6 py-3 rounded-xl {variant_class} transition-all duration-300 flex items-center justify-center gap-3 group {}",
        class.unwrap_or_default()
    );

    // Element listeners can't take an Option handler directly — wrap it
    // (same pattern as dioxus' own `Link` component).
    let onmousedown = move |event: MouseEvent| {
        if let Some(handler) = onmousedown {
            handler.call(event);
        }
    };

    match href {
        Some(href) => rsx! {
            a { href, class, onmousedown, {children} }
        },
        None => rsx! {
            button { class, onmousedown, {children} }
        },
    }
}
