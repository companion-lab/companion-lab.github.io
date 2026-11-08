use dioxus::prelude::*;

use crate::components::molecules::FeatureCard;

#[derive(Clone, PartialEq)]
struct Feature {
    icon: &'static str,
    title: &'static str,
    desc: &'static str,
    color: &'static str,
}

#[component]
pub fn Features() -> Element {
    let features = vec![
        Feature { icon: "fa-cube", title: "VRM Rendering", desc: "Three.js + @pixiv/three-vrm 3.5 avatar with blendshapes and expressions.", color: "purple" },
        Feature { icon: "fa-window-maximize", title: "Transparent Overlay", desc: "Frameless always-on-top window, scalable 50–200%.", color: "blue" },
        Feature { icon: "fa-face-smile", title: "Expressions", desc: "6 expressions — neutral, happy, sad, angry, surprised, relaxed — lerped over time.", color: "indigo" },
        Feature { icon: "fa-masks-theater", title: "Motion Clips", desc: "3 built-in motions (nod, shake, think) plus your own imported VRMA clips.", color: "pink" },
        Feature { icon: "fa-microphone-lines", title: "Voice Input", desc: "On-device STT via Moonshine Voice (Rust FFI + cpal), no cloud.", color: "purple" },
        Feature { icon: "fa-volume-high", title: "Voice Output", desc: "Local ONNX TTS (Supertonic-3) with viseme timeline.", color: "blue" },
        Feature { icon: "fa-plug", title: "MCP Tools", desc: "Model tool-calling over stdio / streamable HTTP servers (Playwright, etc.).", color: "indigo" },
        Feature { icon: "fa-bell", title: "Alarms", desc: "4 built-in tools — list, create, delete, toggle — plus a background scheduler.", color: "pink" },
        Feature { icon: "fa-brain", title: "3 Providers", desc: "OpenAI-compatible, Anthropic, Gemini.", color: "purple" },
        Feature { icon: "fa-microphone", title: "Wake Word", desc: "Configurable wake phrase; a lone wake word arms the mic.", color: "blue" },
        Feature { icon: "fa-keyboard", title: "Global Hotkeys", desc: "Push-to-talk shortcut.", color: "indigo" },
        Feature { icon: "fa-download", title: "Custom Models", desc: "Import any .vrm character, .vrma motion clip, or expression preset.", color: "pink" },
    ];

    rsx! {
        section {
            id: "features",
            class: "py-16 sm:py-24 relative",
            div {
                class: "max-w-7xl mx-auto px-6",
                div {
                    class: "mb-10 sm:mb-16 reveal active",
                    div {
                        class: "flex items-center justify-between gap-4 border-b border-white/10 pb-3 font-mono text-[11px] uppercase tracking-[0.22em] text-purple-300/90",
                        span { "mod.01 // capabilities" }
                        span { class: "text-gray-500", "{features.len()} modules loaded" }
                    }
                    h2 {
                        class: "mt-8 text-4xl md:text-6xl font-black uppercase tracking-tight text-white",
                        "What "
                        span { class: "text-purple-500", "Companion" }
                        " does"
                    }
                    p {
                        class: "mt-4 max-w-2xl text-gray-400 sm:text-lg",
                        "A transparent VRM avatar, speech, tool-calling, alarms, and chat — running on your desktop."
                    }
                }
                // Hairline data-grid: cells share 1px seams drawn by the gap
                // over the wrapper background.
                div {
                    class: "grid grid-cols-1 gap-px border border-white/10 bg-white/10 md:grid-cols-2 lg:grid-cols-4",
                    for (i, f) in features.iter().enumerate() {
                        FeatureCard {
                            key: "{f.title}",
                            icon: f.icon,
                            title: f.title,
                            desc: f.desc,
                            color: f.color,
                            index: format!("{:02}", i + 1),
                        }
                    }
                }
            }
        }
    }
}
