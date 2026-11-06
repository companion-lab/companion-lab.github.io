use dioxus::prelude::*;

use crate::{
    components::{scroll_to_id, AmbientBackground, Footer, MarkdownView},
    route::Route,
};

/// One documentation article.
pub struct DocArticle {
    pub slug: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
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
        icon: "fa-solid fa-download",
        markdown: include_str!("../../docs/content/installation.md"),
    },
    DocArticle {
        slug: "usage",
        title: "Usage",
        description: "Talk to your companion, ask for tools, and make the avatar act.",
        icon: "fa-solid fa-comments",
        markdown: include_str!("../../docs/content/usage.md"),
    },
    DocArticle {
        slug: "creating-plugins",
        title: "Creating Plugins",
        description: "Extend Companion with your own React frontend and Rust backend.",
        icon: "fa-solid fa-plug",
        markdown: include_str!("../../docs/content/creating-plugins.md"),
    },
    DocArticle {
        slug: "troubleshooting",
        title: "Troubleshooting",
        description: "Common problems and where to report the rest.",
        icon: "fa-solid fa-wrench",
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

    // Honor `#heading` URLs on first load: the browser's own hash scroll runs
    // before the WASM app has rendered anything, so it needs a second try.
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        if let Some(window) = web_sys::window() {
            if let Ok(hash) = window.location().hash() {
                if hash.len() > 1 {
                    scroll_to_id(&hash[1..]);
                }
            }
        }
    });

    rsx! {
        div {
            class: "relative min-h-screen pb-24 pt-24 sm:pt-32",
            AmbientBackground {}
            div {
                class: "relative z-10 mx-auto max-w-7xl px-6 pb-16",
                div {
                    class: "mb-10 reveal active",
                    p { class: "mb-2 text-sm font-semibold uppercase tracking-wider text-purple-400", "Documentation" }
                    h1 { class: "text-3xl font-bold text-white sm:text-4xl", "Everything about using Companion" }
                    p { class: "mt-3 max-w-2xl text-gray-400", "Install the app, drive your avatar, and build plugins — one page per topic." }
                }

                div {
                    class: "flex flex-col items-stretch gap-8 lg:flex-row lg:items-start",
                    aside {
                        class: "w-full lg:sticky lg:top-28 lg:w-64 lg:shrink-0",
                        nav {
                            class: "flex gap-2 overflow-x-auto pb-2 lg:flex-col lg:pb-0",
                            for article in DOC_ARTICLES.iter() {
                                Link {
                                    key: "{article.slug}",
                                    to: Route::DocsArticle { slug: article.slug.to_string() },
                                    class: if active.is_some_and(|a| a.slug == article.slug) {
                                        "flex items-center gap-3 rounded-xl border border-purple-500/30 bg-purple-600/20 px-4 py-3 text-sm font-medium text-purple-300"
                                    } else {
                                        "flex items-center gap-3 rounded-xl border border-transparent px-4 py-3 text-sm font-medium text-gray-400 transition-colors hover:bg-white/5 hover:text-white"
                                    },
                                    i { class: "{article.icon} w-4 text-center", "aria-hidden": "true" }
                                    span { class: "whitespace-nowrap", "{article.title}" }
                                }
                            }
                        }
                    }

                    div {
                        class: "min-w-0 w-full flex-1 rounded-2xl glass-panel p-6 sm:p-10",
                        match active {
                            Some(article) => rsx! {
                                MarkdownView { markdown: article.markdown }
                                p { class: "-mt-2 mb-8 text-sm text-gray-500", "{article.description}" }

                                div {
                                    class: "mt-12 flex flex-col justify-between gap-3 border-t border-white/10 pt-6 sm:flex-row",
                                    match prev {
                                        Some(article) => rsx! {
                                            Link {
                                                to: Route::DocsArticle { slug: article.slug.to_string() },
                                                class: "flex items-center gap-2 text-sm text-gray-400 transition-colors hover:text-white",
                                                i { class: "fa-solid fa-arrow-left text-xs", "aria-hidden": "true" }
                                                "{article.title}"
                                            }
                                        },
                                        None => rsx! { span {} },
                                    }
                                    match next {
                                        Some(article) => rsx! {
                                            Link {
                                                to: Route::DocsArticle { slug: article.slug.to_string() },
                                                class: "flex items-center gap-2 text-sm text-gray-400 transition-colors hover:text-white sm:text-right",
                                                "{article.title}"
                                                i { class: "fa-solid fa-arrow-right text-xs", "aria-hidden": "true" }
                                            }
                                        },
                                        None => rsx! { span {} },
                                    }
                                }
                            },
                            None => rsx! {
                                h2 { class: "mb-3 text-2xl font-bold text-white", "Page not found" }
                                p { class: "mb-6 text-gray-400", "There is no documentation page called \u{201C}{active_slug}\u{201D}." }
                                Link {
                                    to: Route::Docs {},
                                    class: "text-purple-400 transition-colors hover:text-purple-300",
                                    i { class: "fa-solid fa-arrow-left mr-2", "aria-hidden": "true" }
                                    "Back to the documentation"
                                }
                            }
                        }
                    }
                }
            }
            Footer {}
        }
    }
}
