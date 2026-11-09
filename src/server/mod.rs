mod presentations;

pub(crate) mod builds;

pub use presentations::detect_os;

use dioxus::prelude::*;

/// Route list for `dx build --ssg`. dx spins up the built server binary,
/// calls this endpoint, and prerenders each returned path into static HTML
/// (hydration data included) in the deploy artifact.
///
/// Keep in sync with `Route` in `src/route.rs`: the router's site map has no
/// dynamic-segment enumeration, so the doc slugs are listed here explicitly
/// from [`DOC_ARTICLES`] — a new article must be added there (its slug then
/// flows into this list and the prerender automatically).
/// Route list for `dx build --ssg`. dx spins up the built server binary,
/// POSTs to this endpoint (see the CLI's `builder.rs` — POST with an empty
/// JSON object), and prerenders each returned path into static HTML
/// (hydration data included) in the deploy artifact.
///
/// Keep in sync with `Route` in `src/route.rs`: the router's site map has no
/// dynamic-segment enumeration, so the doc slugs are listed here explicitly
/// from [`crate::pages::docs::DOC_ARTICLES`] — a new article must be added
/// there (its slug then flows into this list and the prerender
/// automatically). The path must stay exactly `/api/static_routes`: dx
/// requests it verbatim (a default `#[server]` would append a hash).
#[post("/api/static_routes")]
pub async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    let mut routes: Vec<String> = vec![
        "/".into(),
        "/docs".into(),
        "/plugins".into(),
        "/legal".into(),
    ];
    for article in crate::pages::docs::DOC_ARTICLES.iter() {
        routes.push(format!("/docs/{}", article.slug));
    }
    // Rendered NotFound page, saved as `404.html` — GitHub Pages serves it
    // for unknown paths, so the SPA fallback hydrates matching content
    // instead of hydrating the home page under a wrong route.
    routes.push("/404.html".into());
    Ok(routes)
}
