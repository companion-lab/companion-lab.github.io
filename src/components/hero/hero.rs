use dioxus::prelude::*;

use crate::components::DownloadButtons;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section {
            class: "relative min-h-screen flex items-center pt-24 pb-12 overflow-hidden",
            div {
                class: "max-w-7xl mx-auto px-6 w-full grid grid-cols-1 lg:grid-cols-2 gap-12 items-center",
                // Left: Text Content
                div {
                    class: "flex flex-col items-start z-10 reveal active",
                    h1 {
                        class: "text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-bold leading-[1.1] tracking-tight mb-6",
                        "A 3D VRM avatar "
                        br {}
                        span { class: "text-gradient", "on your desktop" }
                        br {}
                        "for your AI agent"
                    }
                    p {
                        class: "text-base sm:text-lg text-gray-400 mb-8 max-w-xl leading-relaxed",
                        "Puts a 3D VRM avatar on your desktop that acts as the visual body of your AI agent. It sits as a transparent overlay — no window chrome, no background — just your character floating on screen, expressions and motion in real time."
                    }
                    DownloadButtons { centered: false }
                    p {
                        class: "text-xs text-gray-500 mt-2",
                        "Desktop build via Tauri v2. Requires models: pnpm models + Moonshine STT."
                    }
                    p {
                        class: "text-xs text-gray-500 mt-4",
                        i { class: "fa-solid fa-shield-check mr-1" }
                        " Tauri v2 desktop shell. OpenAI / Anthropic / Gemini providers."
                    }
                }

                // Right: Hero Graphic
                div {
                    class: "relative w-full h-[420px] sm:h-[500px] flex items-center justify-center z-10 reveal active lg:justify-end",
                    div {
                        class: "relative w-full max-w-[450px] h-[380px] sm:h-[450px] glass-panel rounded-3xl p-6 sm:p-8 flex flex-col items-center justify-center border-t border-l border-white/20 shadow-2xl",
                        div {
                            class: "absolute top-4 left-4 flex gap-2",
                            div { class: "w-3 h-3 rounded-full bg-red-500/80" }
                            div { class: "w-3 h-3 rounded-full bg-yellow-500/80" }
                            div { class: "w-3 h-3 rounded-full bg-green-500/80" }
                        }
                        div { class: "absolute top-4 right-4 text-xs font-mono text-gray-500", "COMPANION.EXE_ACTIVE" }

                        div {
                            class: "relative w-48 h-48 sm:w-64 sm:h-64 animate-float mt-4",
                            div {
                                class: "absolute inset-0 rounded-full border border-purple-500/30 animate-spin-slow shadow-[inset_0_0_20px_rgba(168,85,247,0.1)]",
                                div { class: "absolute top-0 left-1/2 w-2 h-2 bg-purple-400 rounded-full blur-[1px]" }
                            }
                            div {
                                class: "absolute inset-6 rounded-full border border-blue-500/40 animate-spin-slow-reverse",
                                div { class: "absolute bottom-0 right-1/4 w-3 h-3 bg-blue-400 rounded-full blur-[2px]" }
                            }
                            div { class: "absolute inset-12 rounded-full bg-gradient-to-br from-purple-600 to-blue-600 animate-pulse-glow z-0" }
                            div {
                                class: "absolute inset-16 rounded-full bg-gradient-to-tr from-indigo-900 via-purple-500 to-blue-300 shadow-[0_0_40px_rgba(168,85,247,0.8)] z-10 flex items-center justify-center overflow-hidden border border-white/30",
                                div { class: "absolute top-2 left-4 w-12 h-6 bg-white/30 rounded-full rotate-[-45deg] blur-[2px]" }
                                i { class: "fa-solid fa-robot text-3xl sm:text-4xl text-white/90 drop-shadow-[0_0_10px_rgba(255,255,255,0.8)]" }
                            }
                        }

                        div {
                            class: "mt-8 sm:mt-12 flex items-end justify-center gap-1 h-8",
                            div { class: "w-1.5 bg-purple-500 rounded-t-sm h-full animate-[ping_1s_ease-in-out_infinite]" }
                            div { class: "w-1.5 bg-blue-500 rounded-t-sm h-3/4 animate-[ping_1.2s_ease-in-out_infinite_0.1s]" }
                            div { class: "w-1.5 bg-purple-400 rounded-t-sm h-1/2 animate-[ping_0.8s_ease-in-out_infinite_0.2s]" }
                            div { class: "w-1.5 bg-blue-400 rounded-t-sm h-full animate-[ping_1.1s_ease-in-out_infinite_0.3s]" }
                            div { class: "w-1.5 bg-purple-500 rounded-t-sm h-2/3 animate-[ping_0.9s_ease-in-out_infinite_0.4s]" }
                        }
                        p { class: "text-xs text-gray-400 mt-3 tracking-widest uppercase", "Listening..." }
                    }

                    div {
                        class: "absolute -right-6 top-1/4 glass-card p-3 rounded-xl flex items-center gap-3 animate-float-delayed z-20 hidden md:flex",
                        div {
                            class: "w-8 h-8 rounded-full bg-green-500/20 flex items-center justify-center text-green-400",
                            i { class: "fa-solid fa-check" }
                        }
                        div {
                            p { class: "text-xs font-semibold text-white", "Transparent Overlay" }
                            p { class: "text-[10px] text-gray-400", "Frameless, always-on-top" }
                        }
                    }
                }
            }
        }
    }
}
