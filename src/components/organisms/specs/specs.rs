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

#[component]
pub fn Specs() -> Element {
    rsx! {
        section {
            id: "specs",
            class: "py-16 sm:py-24 border-t border-white/5 relative bg-gradient-to-b from-transparent to-purple-900/10",
            div {
                class: "max-w-7xl mx-auto px-6",
                div {
                    class: "grid grid-cols-1 lg:grid-cols-2 gap-16 items-center",
                    // Left: Making the avatar act
                    div {
                        class: "reveal active",
                        h2 { class: "text-3xl md:text-4xl font-bold mb-6", "Making the avatar " br {} "act" }
                        p { class: "text-gray-400 mb-8", "The provider returns a motion and an expression with every reply. Request them directly, or use inline tags for providers answering in plain prose." }
                        ul {
                            class: "space-y-3",
                            li { class: "flex items-start gap-3 text-gray-300", i { class: "fa-solid fa-arrow-right text-purple-400 mt-1" } " \"do a nod\" → plays the built-in nod" }
                            li { class: "flex items-start gap-3 text-gray-300", i { class: "fa-solid fa-arrow-right text-purple-400 mt-1" } " \"do the VRMA_02 motion\" → plays your imported clip" }
                            li { class: "flex items-start gap-3 text-gray-300", i { class: "fa-solid fa-arrow-right text-purple-400 mt-1" } " \"look angry\" → sets the angry expression" }
                            li { class: "flex items-start gap-3 text-gray-300", i { class: "fa-solid fa-arrow-right text-purple-400 mt-1" } " Tags are stripped before the text is spoken" }
                        }
                        p { class: "text-xs text-gray-400 mt-6", "Motion names come from three built-ins plus every clip you import; the model is told the full list on every request, so custom clip names work as soon as they're imported." }
                    }

                    // Right: Tech Specs
                    div {
                        class: "glass-panel p-1 rounded-2xl reveal active",
                        div {
                            class: "bg-[#0a0a1a] rounded-xl p-8 h-full",
                            h3 {
                                class: "text-xl font-semibold mb-6 flex items-center gap-2",
                                i { class: "fa-solid fa-microchip text-purple-400" }
                                "Technical Specifications"
                            }
                            ul {
                                class: "space-y-4 divide-y divide-white/5",
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
