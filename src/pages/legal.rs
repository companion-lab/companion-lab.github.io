use dioxus::prelude::*;

use crate::{
    components::{
        AmbientBackground, Footer, SECTION_SCROLL_TARGET, scroll_to_entry_hash, scroll_to_id,
    },
    config::{COMMUNITY_GITHUB_URL, DISCORD_URL},
};

/// `/legal` — Privacy & Terms, presented as a dossier-style document: a mono
/// HUD header, a sticky index rail, and two corner-bracketed record panels.
/// The footer links point at `/legal#privacy` and `/legal#terms`, and the
/// index rail jumps between the sections.
#[component]
pub fn Legal() -> Element {
    // Scroll to a section requested from another page (footer links) or
    // re-requested while already here — same mechanism as Home. The tracked
    // read subscribes the effect to the signal; `take` clears it in the same
    // step so the effect settles after one extra run instead of looping.
    use_effect(move || {
        let target = SECTION_SCROLL_TARGET();
        if target.is_some() {
            SECTION_SCROLL_TARGET.with_mut(|target| target.take());
            scroll_to_id(target.unwrap());
        }
    });

    // Honor `#section` URLs on first load, reliably — see scroll_to_entry_hash
    // for why a single "read the hash and scroll" pass loses races.
    use_effect(scroll_to_entry_hash);

    rsx! {
        div {
            class: "relative min-h-screen pb-24 pt-24 sm:pt-32",
            AmbientBackground {}
            div {
                class: "relative z-10 mx-auto max-w-6xl px-6 pb-16",

                // ── HUD header ──────────────────────────────────────────
                header { class: "reveal active",
                    div {
                        class: "flex items-center justify-between gap-4 border-b border-white/10 pb-3 font-mono text-[11px] uppercase tracking-[0.22em] text-purple-300/90",
                        span { "cmp://legal-dossier" }
                        span {
                            class: "flex items-center gap-2 text-gray-400",
                            span { class: "hud-dot", "aria-hidden": "true" }
                            "rev 2026.11.08 · live document"
                        }
                    }

                    h1 {
                        class: "mt-8 text-5xl font-black uppercase leading-[0.95] tracking-tight text-white sm:text-7xl",
                        "Privacy"
                        span { class: "text-purple-500", " &" }
                        br {}
                        "Terms"
                        span { class: "hud-cursor text-purple-400", "_" }
                    }

                    p {
                        class: "mt-5 max-w-2xl text-sm leading-relaxed text-gray-400 sm:text-base",
                        "How Companion handles your data, and the terms that apply to using the app and this site. Two records, one dossier — read both before you connect a provider."
                    }
                }

                // ── Index rail + dossier records ────────────────────────
                div {
                    class: "mt-10 flex flex-col gap-8 lg:mt-14 lg:flex-row lg:gap-10",

                    aside {
                        class: "w-full shrink-0 lg:sticky lg:top-28 lg:w-60 lg:self-start",
                        nav {
                            class: "flex gap-2 overflow-x-auto pb-2 font-mono text-[11px] uppercase tracking-[0.18em] lg:flex-col lg:pb-0",
                            // Same interception as the navbar links: the router
                            // eats plain `#hash` clicks, so scroll via
                            // SECTION_SCROLL_TARGET and let the effect at the
                            // top of this component do it.
                            a {
                                href: "#privacy",
                                class: "flex shrink-0 items-center gap-3 border border-white/10 bg-white/[0.02] px-3 py-2.5 text-gray-400 transition-colors hover:border-purple-500/60 hover:text-purple-300",
                                onclick: move |event: MouseEvent| {
                                    event.prevent_default();
                                    SECTION_SCROLL_TARGET.with_mut(|target| *target = Some("privacy"));
                                },
                                span { class: "text-purple-400", "01" }
                                "Privacy Policy"
                            }
                            a {
                                href: "#terms",
                                class: "flex shrink-0 items-center gap-3 border border-white/10 bg-white/[0.02] px-3 py-2.5 text-gray-400 transition-colors hover:border-purple-500/60 hover:text-purple-300",
                                onclick: move |event: MouseEvent| {
                                    event.prevent_default();
                                    SECTION_SCROLL_TARGET.with_mut(|target| *target = Some("terms"));
                                },
                                span { class: "text-purple-400", "02" }
                                "Terms of Service"
                            }
                        }

                        // Document metadata readout (desktop only — the rail
                        // collapses to a horizontal strip on small screens).
                        div {
                            class: "mt-6 hidden border border-white/10 bg-white/[0.02] p-4 font-mono text-[11px] leading-relaxed lg:block",
                            div { class: "space-y-2",
                                MetaRow { label: "doc", value: "cmp-legal-001" }
                                MetaRow { label: "rev", value: "2026.11.08" }
                                MetaRow { label: "records", value: "02" }
                                MetaRow { label: "owner", value: "companion lab" }
                            }
                            div {
                                class: "mt-3 flex items-center gap-2 border-t border-white/10 pt-3 text-purple-300",
                                span { class: "hud-dot", "aria-hidden": "true" }
                                "classification: public"
                            }
                        }
                    }

                    div {
                        class: "hud-grid min-w-0 flex-1 space-y-8 border border-white/5 p-4 sm:p-6 lg:p-8",

                        // ── Record 01: Privacy Policy ────────────────────
                        section {
                            id: "privacy",
                            class: "hud-frame reveal active border border-white/10 bg-[#08040f]/80 p-6 sm:p-10",
                            div { class: "hud-frame-inner",

                                SectionHeader {
                                    id: "sec.01",
                                    title: "Privacy Policy",
                                    meta: "08 subsections",
                                }

                                LegalP { "Companion is built to keep your data on your machine. This policy explains what the app and this website process, store, and share." }

                                SubSection { index: "01.1", title: "The short version" }
                                ReadoutList {
                                    ReadoutRow { "Conversations, settings, and API keys are stored on your device only." }
                                    ReadoutRow { "The app ships with no telemetry — nothing is reported back to us." }
                                    ReadoutRow { "Requests to AI providers travel directly from your device to the provider you connected; we never see them." }
                                    ReadoutRow { "This website is static: no cookies, no analytics, no tracking." }
                                }

                                SubSection { index: "01.2", title: "What the app processes locally" }
                                LegalP { "Companion is an open-source desktop application that puts a VRM avatar on your desktop and connects it to an AI provider you configure. Everything it needs to run — your settings, chat history, avatar files, plugin data, and the audio used for speech-to-text — is processed and stored locally on your machine. We run no servers that receive any of it." }

                                SubSection { index: "01.3", title: "AI providers and API keys" }
                                LegalP {
                                    "Companion generates responses by talking to the AI provider you connect in Settings → Connection — for example an OpenAI-compatible endpoint, Anthropic, or Gemini. Your API key is kept on your device and sent only to that provider, directly from your machine. Once a prompt leaves your device, it is covered by that provider's own privacy policy; review it before connecting."
                                }

                                SubSection { index: "01.4", title: "Voice" }
                                LegalP { "The bundled speech-to-text (Moonshine) and speech synthesis (Supertonic) run locally inside the app. Companion itself never uploads your microphone audio; only plugins or provider features you explicitly enable could route voice data elsewhere, and then to that service under its own terms." }

                                SubSection { index: "01.5", title: "Plugins" }
                                LegalP {
                                    "The bundled plugins (chat, voice, alarms, calendar, MCP tools, avatars) run locally and can be removed any time in Settings → Plugins. Third-party plugins can add their own capabilities, including network access — review a plugin's source before installing it. See "
                                    DocLink { to: "/docs/creating-plugins", "Creating plugins" }
                                    "."
                                }

                                SubSection { index: "01.6", title: "This website" }
                                LegalP {
                                    "This site is a static page served from GitHub Pages. It sets no cookies and loads no analytics or tracking scripts. GitHub may process standard request logs for your visit — see "
                                    ExternalLink { to: "https://docs.github.com/en/site-policy/privacy-policies/github-general-privacy-statement", "GitHub's privacy statement" }
                                    " for details."
                                }

                                SubSection { index: "01.7", title: "Community" }
                                LegalP {
                                    "If you open an issue on "
                                    ExternalLink { to: COMMUNITY_GITHUB_URL, "GitHub" }
                                    " or join the "
                                    ExternalLink { to: DISCORD_URL, "Discord" }
                                    ", those platforms process your account and message data under their own policies — and anything you post there is public."
                                }

                                SubSection { index: "01.8", title: "Changes to this policy" }
                                LegalP { "If the app or site starts handling data differently, this page will say so — the date in the header marks the latest revision." }

                                Channel {
                                    "Questions about your data? Open an issue on "
                                    ExternalLink { to: COMMUNITY_GITHUB_URL, "GitHub" }
                                    " or join the "
                                    ExternalLink { to: DISCORD_URL, "Discord" }
                                    "."
                                }
                            }
                        }

                        // ── Record 02: Terms of Service ───────────────────
                        section {
                            id: "terms",
                            class: "hud-frame reveal active border border-white/10 bg-[#08040f]/80 p-6 sm:p-10",
                            div { class: "hud-frame-inner",

                                SectionHeader {
                                    id: "sec.02",
                                    title: "Terms of Service",
                                    meta: "09 subsections",
                                }

                                LegalP { "These terms apply when you use the Companion application or this website." }

                                SubSection { index: "02.1", title: "The short version" }
                                ReadoutList {
                                    ReadoutRow { "Companion is open-source software under the MIT license — use it freely under those conditions." }
                                    ReadoutRow { "The app is provided \u{201C}as is\u{201D}, without warranty of any kind." }
                                    ReadoutRow { "Don't use it to break the law or other people's rights." }
                                    ReadoutRow { "AI providers, plugins, and platforms you connect are governed by their own terms." }
                                }

                                SubSection { index: "02.2", title: "Acceptance" }
                                LegalP { "By using the Companion app or this website you agree to these terms. If you don't agree, please don't use them." }

                                SubSection { index: "02.3", title: "License" }
                                LegalP { "Companion is open-source software released under the MIT license. You may use, copy, modify, merge, publish, and distribute it under that license's conditions — the full text ships with the app and lives in the source repository." }

                                SubSection { index: "02.4", title: "No warranty" }
                                LegalP { "The app is provided \u{201C}as is\u{201D}, without warranty of any kind, express or implied — including the warranties of merchantability, fitness for a particular purpose, and non-infringement." }

                                SubSection { index: "02.5", title: "Acceptable use" }
                                LegalP { "Use Companion in ways that are legal and respect others' rights. You are responsible for the prompts you write, the avatar assets and plugins you load, and what you do with the app's output." }

                                SubSection { index: "02.6", title: "AI-generated content" }
                                LegalP { "Responses come from the third-party models behind the provider you connect. They can be wrong or out of date — verify anything important before relying on it, and follow your provider's usage policies." }

                                SubSection { index: "02.7", title: "Third-party services and plugins" }
                                LegalP { "AI providers, plugins, and platforms such as GitHub and Discord are operated by third parties under their own terms. Companion isn't responsible for what those services do." }

                                SubSection { index: "02.8", title: "Limitation of liability" }
                                LegalP { "To the extent permitted by law, the authors and copyright holders of Companion are not liable for any claim, damages, or other liability arising from, out of, or in connection with the app or this site — the same limitation the MIT license applies." }

                                SubSection { index: "02.9", title: "Changes to these terms" }
                                LegalP { "These terms may change as the project evolves; the date in the header marks the latest revision. Continued use after a change means you accept the updated terms." }

                                Channel {
                                    "Questions about these terms? Open an issue on "
                                    ExternalLink { to: COMMUNITY_GITHUB_URL, "GitHub" }
                                    " or join the "
                                    ExternalLink { to: DISCORD_URL, "Discord" }
                                    "."
                                }
                            }
                        }

                        // ── Dossier end strip ─────────────────────────────
                        div {
                            class: "flex flex-wrap items-center justify-between gap-2 px-1 font-mono text-[11px] uppercase tracking-[0.18em] text-gray-400",
                            span { "end of dossier" }
                            span { "doc // cmp-legal-001 — classification: public" }
                        }
                    }
                }
            }
        }
        // Footer sits outside the padded min-h-screen wrapper — a bottom
        // padding on this wrapper would render as an empty strip below it.
        Footer {}
    }
}

/// A `label / value` row of the document metadata readout.
#[component]
fn MetaRow(label: &'static str, value: &'static str) -> Element {
    rsx! {
        div { class: "flex items-baseline justify-between gap-4",
            span { class: "uppercase tracking-[0.18em] text-gray-400", "{label}" }
            span { class: "text-gray-300", "{value}" }
        }
    }
}

/// The header row of a record: mono id, title, hairline rule, meta.
#[component]
fn SectionHeader(id: &'static str, title: &'static str, meta: &'static str) -> Element {
    rsx! {
        div { class: "mb-6",
            div { class: "flex flex-wrap items-center gap-x-4 gap-y-1",
                span {
                    class: "border border-purple-500/40 bg-purple-500/10 px-2 py-0.5 font-mono text-[11px] uppercase tracking-[0.2em] text-purple-300",
                    "{id}"
                }
                h2 { class: "text-2xl font-bold uppercase tracking-tight text-white sm:text-3xl", "{title}" }
                span {
                    class: "ml-auto font-mono text-[11px] uppercase tracking-[0.15em] text-gray-400",
                    "{meta}"
                }
            }
            div {
                class: "mt-4 h-px w-full bg-gradient-to-r from-purple-500/70 via-white/10 to-transparent",
                "aria-hidden": "true"
            }
        }
    }
}

/// A numbered subsection: accent bar, mono index, spaced-uppercase title.
#[component]
fn SubSection(index: &'static str, title: &'static str) -> Element {
    rsx! {
        div { class: "mt-8 mb-3 flex items-baseline gap-3 border-l-2 border-purple-500/70 pl-3",
            span { class: "font-mono text-xs text-purple-400/80", "{index}" }
            h3 { class: "text-sm font-semibold uppercase tracking-[0.18em] text-gray-200", "{title}" }
        }
    }
}

/// A body paragraph of the legal text.
#[component]
fn LegalP(children: Element) -> Element {
    rsx! { p { class: "mb-3 leading-relaxed text-gray-400", {children} } }
}

/// A readout row of the "short version" lists.
#[component]
fn ReadoutRow(children: Element) -> Element {
    rsx! {
        li {
            class: "flex items-baseline gap-3 py-2.5 text-sm leading-relaxed text-gray-400",
            span { class: "font-mono text-xs text-purple-400/70", "▸" }
            span { {children} }
        }
    }
}

/// Hairline-divided list of readout rows.
#[component]
fn ReadoutList(children: Element) -> Element {
    rsx! {
        ul { class: "divide-y divide-white/5 border-y border-white/5", {children} }
    }
}

/// A link to a page of this site.
#[component]
fn DocLink(to: &'static str, children: Element) -> Element {
    rsx! {
        Link {
            to,
            class: "text-purple-400 underline decoration-purple-500/40 underline-offset-4 transition-colors hover:text-purple-300 hover:decoration-purple-300",
            {children}
        }
    }
}

/// A link to an external site, opened in a new tab.
#[component]
fn ExternalLink(to: &'static str, children: Element) -> Element {
    rsx! {
        a {
            href: to,
            target: "_blank",
            class: "text-purple-400 underline decoration-purple-500/40 underline-offset-4 transition-colors hover:text-purple-300 hover:decoration-purple-300",
            {children}
        }
    }
}

/// The comms callout both records end with.
#[component]
fn Channel(children: Element) -> Element {
    rsx! {
        div {
            class: "mt-8 flex flex-wrap items-baseline gap-x-2 border border-white/10 bg-white/[0.03] px-4 py-3 text-sm leading-relaxed text-gray-400",
            span {
                class: "font-mono text-[11px] uppercase tracking-[0.2em] text-purple-400/80",
                "comms //"
            }
            span { {children} }
        }
    }
}
