use dioxus::prelude::*;

use crate::{
    components::{
        AmbientBackground, Footer, SECTION_SCROLL_TARGET, scroll_to_entry_hash, scroll_to_id,
    },
    config::{COMMUNITY_GITHUB_URL, DISCORD_URL},
};

/// `/legal` — Privacy & Terms: one page, two anchored sections. The footer
/// links point at `/legal#privacy` and `/legal#terms`, and the in-page tabs
/// jump between the sections.
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
                class: "relative z-10 mx-auto max-w-3xl px-6 pb-16",
                div {
                    class: "mb-8 reveal active",
                    p { class: "mb-2 text-sm font-semibold uppercase tracking-wider text-purple-400", "Legal" }
                    h1 { class: "text-3xl font-bold text-white sm:text-4xl", "Privacy & Terms" }
                    p { class: "mt-3 max-w-2xl text-gray-400", "How Companion handles your data, and the terms that apply to using the app and this site. Last updated November 8, 2026." }
                }

                div {
                    class: "mb-8 flex flex-wrap gap-3 reveal active",
                    // Same interception as the navbar links: the router eats
                    // plain `#hash` clicks, so scroll via SECTION_SCROLL_TARGET
                    // and let the effect at the top of this component do it.
                    a {
                        href: "#privacy",
                        class: "flex items-center gap-2 rounded-xl border border-white/10 bg-white/5 px-4 py-2 text-sm font-medium text-gray-300 transition-colors hover:border-purple-500/40 hover:bg-purple-600/10 hover:text-white",
                        onclick: move |event: MouseEvent| {
                            event.prevent_default();
                            SECTION_SCROLL_TARGET.with_mut(|target| *target = Some("privacy"));
                        },
                        i { class: "fa-solid fa-shield-halved w-4 text-center text-purple-400", "aria-hidden": "true" }
                        "Privacy Policy"
                    }
                    a {
                        href: "#terms",
                        class: "flex items-center gap-2 rounded-xl border border-white/10 bg-white/5 px-4 py-2 text-sm font-medium text-gray-300 transition-colors hover:border-purple-500/40 hover:bg-purple-600/10 hover:text-white",
                        onclick: move |event: MouseEvent| {
                            event.prevent_default();
                            SECTION_SCROLL_TARGET.with_mut(|target| *target = Some("terms"));
                        },
                        i { class: "fa-solid fa-file-contract w-4 text-center text-purple-400", "aria-hidden": "true" }
                        "Terms of Service"
                    }
                }

                div {
                    class: "glass-panel rounded-2xl p-6 sm:p-10 reveal active",

                    section {
                        id: "privacy",
                        div {
                            class: "mb-4 flex items-center gap-3",
                            i { class: "fa-solid fa-shield-halved text-purple-400", "aria-hidden": "true" }
                            h2 { class: "text-2xl font-semibold text-white", "Privacy Policy" }
                        }
                        LegalP { "Companion is built to keep your data on your machine. This policy explains what the app and this website process, store, and share." }

                        SubHeading { text: "The short version" }
                        ul {
                            class: "mb-4 list-disc space-y-1 pl-5 marker:text-purple-500/60",
                            LegalLi { "Conversations, settings, and API keys are stored on your device only." }
                            LegalLi { "The app ships with no telemetry — nothing is reported back to us." }
                            LegalLi { "Requests to AI providers travel directly from your device to the provider you connected; we never see them." }
                            LegalLi { "This website is static: no cookies, no analytics, no tracking." }
                        }

                        SubHeading { text: "What the app processes locally" }
                        LegalP { "Companion is an open-source desktop application that puts a VRM avatar on your desktop and connects it to an AI provider you configure. Everything it needs to run — your settings, chat history, avatar files, plugin data, and the audio used for speech-to-text — is processed and stored locally on your machine. We run no servers that receive any of it." }

                        SubHeading { text: "AI providers and API keys" }
                        LegalP {
                            "Companion generates responses by talking to the AI provider you connect in Settings → Connection — for example an OpenAI-compatible endpoint, Anthropic, or Gemini. Your API key is kept on your device and sent only to that provider, directly from your machine. Once a prompt leaves your device, it is covered by that provider's own privacy policy; review it before connecting."
                        }

                        SubHeading { text: "Voice" }
                        LegalP { "The bundled speech-to-text (Moonshine) and speech synthesis (Supertonic) run locally inside the app. Companion itself never uploads your microphone audio; only plugins or provider features you explicitly enable could route voice data elsewhere, and then to that service under its own terms." }

                        SubHeading { text: "Plugins" }
                        LegalP {
                            "The bundled plugins (chat, voice, alarms, calendar, MCP tools, avatars) run locally and can be removed any time in Settings → Plugins. Third-party plugins can add their own capabilities, including network access — review a plugin's source before installing it. See "
                            DocLink { to: "/docs/creating-plugins", "Creating plugins" }
                            "."
                        }

                        SubHeading { text: "This website" }
                        LegalP {
                            "This site is a static page served from GitHub Pages. It sets no cookies and loads no analytics or tracking scripts. GitHub may process standard request logs for your visit — see "
                            ExternalLink { to: "https://docs.github.com/en/site-policy/privacy-policies/github-general-privacy-statement", "GitHub's privacy statement" }
                            " for details."
                        }

                        SubHeading { text: "Community" }
                        LegalP {
                            "If you open an issue on "
                            ExternalLink { to: COMMUNITY_GITHUB_URL, "GitHub" }
                            " or join the "
                            ExternalLink { to: DISCORD_URL, "Discord" }
                            ", those platforms process your account and message data under their own policies — and anything you post there is public."
                        }

                        SubHeading { text: "Changes to this policy" }
                        LegalP { "If the app or site starts handling data differently, this page will say so — the date under the title marks the latest revision." }

                        ContactBox {
                            "Questions about your data? Open an issue on "
                            ExternalLink { to: COMMUNITY_GITHUB_URL, "GitHub" }
                            " or join the "
                            ExternalLink { to: DISCORD_URL, "Discord" }
                            "."
                        }
                    }

                    div { class: "my-12 border-t border-white/10", "aria-hidden": "true" }

                    section {
                        id: "terms",
                        div {
                            class: "mb-4 flex items-center gap-3",
                            i { class: "fa-solid fa-file-contract text-purple-400", "aria-hidden": "true" }
                            h2 { class: "text-2xl font-semibold text-white", "Terms of Service" }
                        }
                        LegalP { "These terms apply when you use the Companion application or this website." }

                        SubHeading { text: "The short version" }
                        ul {
                            class: "mb-4 list-disc space-y-1 pl-5 marker:text-purple-500/60",
                            LegalLi { "Companion is open-source software under the MIT license — use it freely under those conditions." }
                            LegalLi { "The app is provided \u{201C}as is\u{201D}, without warranty of any kind." }
                            LegalLi { "Don't use it to break the law or other people's rights." }
                            LegalLi { "AI providers, plugins, and platforms you connect are governed by their own terms." }
                        }

                        SubHeading { text: "Acceptance" }
                        LegalP { "By using the Companion app or this website you agree to these terms. If you don't agree, please don't use them." }

                        SubHeading { text: "License" }
                        LegalP { "Companion is open-source software released under the MIT license. You may use, copy, modify, merge, publish, and distribute it under that license's conditions — the full text ships with the app and lives in the source repository." }

                        SubHeading { text: "No warranty" }
                        LegalP { "The app is provided \u{201C}as is\u{201D}, without warranty of any kind, express or implied — including the warranties of merchantability, fitness for a particular purpose, and non-infringement." }

                        SubHeading { text: "Acceptable use" }
                        LegalP { "Use Companion in ways that are legal and respect others' rights. You are responsible for the prompts you write, the avatar assets and plugins you load, and what you do with the app's output." }

                        SubHeading { text: "AI-generated content" }
                        LegalP { "Responses come from the third-party models behind the provider you connect. They can be wrong or out of date — verify anything important before relying on it, and follow your provider's usage policies." }

                        SubHeading { text: "Third-party services and plugins" }
                        LegalP { "AI providers, plugins, and platforms such as GitHub and Discord are operated by third parties under their own terms. Companion isn't responsible for what those services do." }

                        SubHeading { text: "Limitation of liability" }
                        LegalP { "To the extent permitted by law, the authors and copyright holders of Companion are not liable for any claim, damages, or other liability arising from, out of, or in connection with the app or this site — the same limitation the MIT license applies." }

                        SubHeading { text: "Changes to these terms" }
                        LegalP { "These terms may change as the project evolves; the date under the title marks the latest revision. Continued use after a change means you accept the updated terms." }

                        ContactBox {
                            "Questions about these terms? Open an issue on "
                            ExternalLink { to: COMMUNITY_GITHUB_URL, "GitHub" }
                            " or join the "
                            ExternalLink { to: DISCORD_URL, "Discord" }
                            "."
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

/// A body paragraph of the legal text.
#[component]
fn LegalP(children: Element) -> Element {
    rsx! { p { class: "mb-3 leading-relaxed text-gray-400", {children} } }
}

/// One bullet of the "short version" lists.
#[component]
fn LegalLi(children: Element) -> Element {
    rsx! { li { class: "leading-relaxed text-gray-400", {children} } }
}

/// A subsection heading inside a legal section.
#[component]
fn SubHeading(text: &'static str) -> Element {
    rsx! { h3 { class: "mb-2 mt-8 text-base font-semibold text-purple-300", "{text}" } }
}

/// A link to a page of this site.
#[component]
fn DocLink(to: &'static str, children: Element) -> Element {
    rsx! {
        Link { to, class: "text-purple-400 transition-colors hover:text-purple-300", {children} }
    }
}

/// A link to an external site, opened in a new tab.
#[component]
fn ExternalLink(to: &'static str, children: Element) -> Element {
    rsx! {
        a {
            href: to,
            target: "_blank",
            class: "text-purple-400 transition-colors hover:text-purple-300",
            {children}
        }
    }
}

/// The small callout both sections end with.
#[component]
fn ContactBox(children: Element) -> Element {
    rsx! {
        div {
            class: "mt-8 rounded-xl border border-white/10 bg-white/5 p-4 text-sm leading-relaxed text-gray-400",
            {children}
        }
    }
}
