use dioxus::prelude::*;

/// The app preview recording. Drop the real screen recording (H.264 mp4) at
/// `assets/video/app-preview.mp4` — `asset!` hashes and bundles whatever is
/// there at build time, so replacing the placeholder file needs no code
/// changes. The placeholder currently in place is not a playable video; it
/// only exists so the build compiles before the recording lands.
const PREVIEW_VIDEO: Asset = asset!("/assets/video/demo.mp4");

/// App preview section: a screen recording of the actual desktop app so
/// visitors can see what Companion is before downloading it.
///
/// Until a real recording ships, a missing or undecodable file fires the
/// video element's `error` event and the player swaps to a styled "coming
/// soon" panel instead of a dead player. Once the real file is in place the
/// panel never appears.
#[component]
pub fn Preview() -> Element {
    let mut failed = use_signal(|| false);

    rsx! {
        section {
            id: "preview",
            class: "py-16 sm:py-24 relative",
            div {
                class: "max-w-7xl mx-auto px-6",
                div {
                    class: "mb-10 sm:mb-16 reveal active",
                    div {
                        class: "flex items-center justify-between gap-4 border-b border-white/10 pb-3 font-mono text-[11px] uppercase tracking-[0.22em] text-purple-300/90",
                        span { "mod.00 // app preview" }
                        span { class: "flex items-center gap-2 text-gray-500",
                            span { class: "hud-dot", "aria-hidden": "true" }
                            "screen.recording"
                        }
                    }
                    h2 {
                        class: "mt-8 text-4xl md:text-6xl font-black uppercase tracking-tight text-white",
                        "See it "
                        span { class: "text-purple-500", "in action" }
                    }
                    p {
                        class: "mt-4 max-w-2xl text-gray-400 sm:text-lg",
                        "The actual desktop app: a transparent VRM avatar floating over your screen, chatting with your agent, reacting with expressions and motions, and running tools in real time."
                    }
                }

                // Player wrapped in the same HUD frame as the hero viewport,
                // so the recording reads as "the app itself".
                div {
                    class: "hud-frame hud-grid reveal active border border-white/10 bg-[#08040f]/70 p-4 sm:p-6",
                    div { class: "hud-frame-inner",
                        div {
                            class: "flex items-center justify-between pb-3 font-mono text-[10px] uppercase tracking-[0.2em] text-gray-500",
                            span { class: "text-purple-300/80", "screen.recording // desktop overlay" }
                            span { "mp4" }
                        }

                        if failed() {
                            div {
                                class: "flex aspect-video w-full flex-col items-center justify-center gap-3 bg-black/40",
                                i { class: "fa-solid fa-clapperboard text-4xl text-white/60", "aria-hidden": "true" }
                                span { class: "font-mono text-xs uppercase tracking-[0.2em] text-gray-400", "app preview // coming soon" }
                                span { class: "font-mono text-[10px] uppercase tracking-[0.15em] text-gray-600", "assets/video/app-preview.mp4" }
                            }
                        } else {
                            video {
                                class: "aspect-video w-full bg-black/40",
                                src: PREVIEW_VIDEO,
                                controls: true,
                                preload: "metadata",
                                playsinline: true,
                                "aria-label": "Companion app preview recording",
                                onerror: move |_| failed.set(true),
                            }
                        }

                        div {
                            class: "flex items-center justify-between pt-3 font-mono text-[10px] uppercase tracking-[0.2em] text-gray-600",
                            span { "transparent overlay // voice + chat + tools" }
                            span { "companion // desktop" }
                        }
                    }
                }
            }
        }
    }
}
