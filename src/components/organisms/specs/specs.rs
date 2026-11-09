use dioxus::prelude::*;

use crate::components::molecules::SpecRow;

const SPECS: [(&str, &str); 7] = [
    ("Desktop Shell", "Tauri v2 (Rust)"),
    ("3D Rendering", "Three.js + three-vrm 3.5"),
    ("STT", "Moonshine Voice (on-device)"),
    ("TTS", "Supertonic-3 ONNX"),
    ("MCP", "rmcp (Rust SDK)"),
    ("Providers", "OpenAI-compat / Anthropic / Gemini"),
    ("Frontend", "React + TypeScript"),
];

/// One console row of the behavior examples: `[>]` prompt, quoted command,
/// dimmed result.
#[component]
fn CommandRow(command: &'static str, result: &'static str) -> Element {
    rsx! {
        div {
            class: "flex flex-wrap items-baseline gap-x-2 px-4 py-2.5 font-mono text-xs sm:text-sm",
            span { class: "text-purple-400", "[>]" }
            span { class: "text-gray-200", "{command}" }
            span { class: "text-gray-400", "→ {result}" }
        }
    }
}

#[component]
pub fn Specs() -> Element {
    rsx! {
        section {
            id: "specs",
            class: "border-t border-white/10 py-16 sm:py-24 relative",
            div {
                class: "max-w-7xl mx-auto px-6",
                div {
                    class: "grid grid-cols-1 lg:grid-cols-2 gap-12 lg:gap-16 items-center",
                    // Left: Making the avatar act
                    div {
                        class: "reveal active",
                        div {
                            class: "flex items-center justify-between gap-4 border-b border-white/10 pb-3 font-mono text-[11px] uppercase tracking-[0.22em] text-purple-300/90",
                            span { "mod.02 // behavior" }
                            span { class: "text-gray-400", "mcp + inline tags" }
                        }
                        h2 {
                            class: "mt-8 text-3xl md:text-5xl font-black uppercase tracking-tight text-white",
                            "Making the avatar "
                            br {}
                            span { class: "text-purple-500", "act" }
                        }
                        p {
                            class: "mt-5 text-gray-400 leading-relaxed",
                            "The provider returns a motion and an expression with every reply. Request them directly, or use inline tags for providers answering in plain prose."
                        }
                        div {
                            class: "mt-6 divide-y divide-white/5 border border-white/10 bg-[#08040f]/80",
                            CommandRow { command: "\"do a nod\"", result: "plays the built-in nod" }
                            CommandRow { command: "\"do the VRMA_02 motion\"", result: "plays your imported clip" }
                            CommandRow { command: "\"look angry\"", result: "sets the angry expression" }
                            CommandRow { command: "inline tags", result: "stripped before the text is spoken" }
                        }
                        p {
                            class: "mt-6 font-mono text-[11px] uppercase tracking-[0.15em] leading-relaxed text-gray-400",
                            "// motion names come from three built-ins plus every clip you import — the model is told the full list on every request, so custom clip names work as soon as they're imported"
                        }
                    }

                    // Right: Tech specs readout
                    div {
                        class: "hud-frame hud-grid reveal active border border-white/10 bg-[#08040f]/80 p-6 sm:p-8",
                        div { class: "hud-frame-inner",
                            h3 {
                                class: "mb-6 flex flex-wrap items-center justify-between gap-2 font-mono text-[11px] uppercase tracking-[0.22em] text-purple-300/90",
                                span { "sys.spec // readout" }
                                span { class: "text-gray-400", "7 entries" }
                            }
                            ul {
                                class: "divide-y divide-white/5",
                                for (label, value) in SPECS {
                                    SpecRow { label, value }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
