use dioxus::prelude::*;

use crate::{
    components::{AmbientBackground, Footer, MarkdownView},
    route::Route,
};
use crate::components::scroll_to_entry_hash;

/// One documentation article.
pub struct DocArticle {
    pub slug: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub markdown: &'static str,
}

/// The docs shipped with the site. The markdown is embedded at build time
/// (`include_str!`) because the Pages deployment is a static client-only
/// bundle — there is no server to fetch content from.
pub static DOC_ARTICLES: [DocArticle; 4] = [
    DocArticle {
        slug: "installation",
        title: "Installation",
        description: "Download the app, run it for the first time, and connect an AI provider.",
        markdown: include_str!("../../docs/content/installation.md"),
    },
    DocArticle {
        slug: "usage",
        title: "Usage",
        description: "Talk to your companion, ask for tools, and make the avatar act.",
        markdown: include_str!("../../docs/content/usage.md"),
    },
    DocArticle {
        slug: "creating-plugins",
        title: "Creating Plugins",
        description: "Extend Companion with your own React frontend and Rust backend.",
        markdown: include_str!("../../docs/content/creating-plugins.md"),
    },
    DocArticle {
        slug: "troubleshooting",
        title: "Troubleshooting",
        description: "Common problems and where to report the rest.",
        markdown: include_str!("../../docs/content/troubleshooting.md"),
    },
];

/// `/docs` — opens the first article.
#[component]
pub fn Docs() -> Element {
    rsx! { DocsView { slug: None } }
}

/// `/docs/:slug` — opens the article matching the slug.
#[component]
pub fn DocsArticle(slug: String) -> Element {
    rsx! { DocsView { slug: Some(slug) } }
}

#[component]
fn DocsView(slug: Option<String>) -> Element {
    let active_slug = slug.unwrap_or_else(|| DOC_ARTICLES[0].slug.to_string());
    let active = DOC_ARTICLES.iter().find(|a| a.slug == active_slug);
    let position = active
        .and_then(|a| DOC_ARTICLES.iter().position(|c| c.slug == a.slug))
        .filter(|p| *p > 0);
    let prev = position.map(|p| &DOC_ARTICLES[p - 1]);
    let next = active
        .and_then(|a| DOC_ARTICLES.iter().position(|c| c.slug == a.slug))
        .and_then(|p| DOC_ARTICLES.get(p + 1));

    // Honor `#heading` URLs on first load, reliably — see scroll_to_entry_hash
    // for why a single "read the hash and scroll" pass loses races.
    use_effect(scroll_to_entry_hash);

    let slug_label = format!("/docs/{active_slug}");
    // `record` labels are precomputed because rsx string templates only take
    // named bindings — a bare `{:02}` format spec fails to parse there.
    let records = DOC_ARTICLES
        .iter()
        .enumerate()
        .map(|(index, article)| (format!("{:02}", index + 1), article));

    rsx! {
        div {
            class: "relative min-h-screen pb-24 pt-24 sm:pt-32",
            AmbientBackground {}
            div {
                class: "relative z-10 mx-auto max-w-7xl px-6 pb-16",

                // ── HUD header ──────────────────────────────────────────
                header { class: "mb-10 reveal active",
                    div {
                        class: "flex items-center justify-between gap-4 border-b border-white/10 pb-3 font-mono text-[11px] uppercase tracking-[0.22em] text-purple-300/90",
                        span { "cmp://docs-dossier" }
                        span { class: "flex items-center gap-2 text-gray-500",
                            span { class: "hud-dot", "aria-hidden": "true" }
                            "{DOC_ARTICLES.len()} records · live document"
                        }
                    }
                    h1 {
                        class: "mt-8 text-5xl font-black uppercase leading-[0.95] tracking-tight text-white sm:text-7xl",
                        "Documentation"
                        span { class: "hud-cursor text-purple-400", "_" }
                    }
                    p {
                        class: "mt-5 max-w-2xl text-sm leading-relaxed text-gray-400 sm:text-base",
                        "Everything about using Companion — install the app, drive your avatar, and build plugins. One record per topic."
                    }
                }

                // ── Index rail + record panel ───────────────────────────
                div {
                    class: "flex flex-col items-stretch gap-8 lg:flex-row lg:items-start lg:gap-10",
                    aside {
                        class: "w-full lg:sticky lg:top-28 lg:w-64 lg:shrink-0",
                        nav {
                            class: "flex gap-2 overflow-x-auto pb-2 font-mono text-[11px] uppercase tracking-[0.18em] lg:flex-col lg:pb-0",
                            for (record, article) in records {
                                Link {
                                    key: "{article.slug}",
                                    to: Route::DocsArticle { slug: article.slug.to_string() },
                                    class: if active.is_some_and(|a| a.slug == article.slug) {
                                        "flex shrink-0 items-center gap-3 border border-purple-500/60 bg-purple-500/10 px-3 py-2.5 text-purple-300"
                                    } else {
                                        "flex shrink-0 items-center gap-3 border border-white/10 bg-white/[0.02] px-3 py-2.5 text-gray-400 transition-colors hover:border-purple-500/40 hover:text-purple-300"
                                    },
                                    span { class: "text-purple-400", "{record}" }
                                    span { class: "whitespace-nowrap", "{article.title}" }
                                }
                            }
                        }
                    }

                    div {
                        class: "hud-frame hud-grid min-w-0 w-full flex-1 border border-white/10 bg-[#08040f]/80 p-6 sm:p-10",
                        div { class: "hud-frame-inner",
                            div {
                                class: "mb-8 flex flex-wrap items-center justify-between gap-2 border-b border-white/10 pb-3 font-mono text-[11px] uppercase tracking-[0.22em] text-purple-300/90",
                                span { "doc // {slug_label}" }
                                span { class: "flex items-center gap-2 text-gray-500",
                                    span { class: "hud-dot", "aria-hidden": "true" }
                                    "markdown // embedded"
                                }
                            }

                            match active {
                                Some(article) => rsx! {
                                    MarkdownView { markdown: article.markdown }
                                    p {
                                        class: "-mt-2 mb-8 font-mono text-[11px] uppercase tracking-[0.15em] text-gray-500",
                                        "// {article.description}"
                                    }

                                    div {
                                        class: "mt-12 flex flex-col justify-between gap-3 border-t border-white/10 pt-6 sm:flex-row",
                                        match prev {
                                            Some(article) => rsx! {
                                                Link {
                                                    to: Route::DocsArticle { slug: article.slug.to_string() },
                                                    class: "flex items-center gap-2 font-mono text-[11px] uppercase tracking-[0.15em] text-gray-400 transition-colors hover:text-purple-300",
                                                    i { class: "fa-solid fa-arrow-left text-[10px]", "aria-hidden": "true" }
                                                    "prev // {article.title}"
                                                }
                                            },
                                            None => rsx! { span {} },
                                        }
                                        match next {
                                            Some(article) => rsx! {
                                                Link {
                                                    to: Route::DocsArticle { slug: article.slug.to_string() },
                                                    class: "flex items-center gap-2 font-mono text-[11px] uppercase tracking-[0.15em] text-gray-400 transition-colors hover:text-purple-300 sm:text-right",
                                                    "next // {article.title}"
                                                    i { class: "fa-solid fa-arrow-right text-[10px]", "aria-hidden": "true" }
                                                }
                                            },
                                            None => rsx! { span {} },
                                        }
                                    }
                                },
                                None => rsx! {
                                    h2 { class: "mb-3 text-2xl font-bold uppercase tracking-tight text-white", "Record not found" }
                                    p { class: "mb-6 font-mono text-sm text-gray-400", "// no documentation record called \u{201C}{active_slug}\u{201D}" }
                                    Link {
                                        to: Route::Docs {},
                                        class: "font-mono text-[11px] uppercase tracking-[0.15em] text-purple-400 transition-colors hover:text-purple-300",
                                        i { class: "fa-solid fa-arrow-left mr-2 text-[10px]", "aria-hidden": "true" }
                                        "back to the documentation"
                                    }
                                }
                            }
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
