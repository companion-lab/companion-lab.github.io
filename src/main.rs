mod components;
mod config;
mod pages;
mod route;

// The upload API is native-only: its server functions and storage stack
// (sqlx, S3) don't build for wasm, and the static web client never calls
// them — so it is compiled out of every wasm build.
#[cfg(all(feature = "server", not(target_arch = "wasm32")))]
mod server;

use crate::route::Route;
use dioxus::prelude::*;

// The site head is rendered here through `document::` components — with the
// fullstack document model, the client rebuild replaces the document head
// on hydration, so anything only present as static tags in `index.html`
// (title, meta description, stylesheets, …) disappears once the wasm boots.
// Rendering them through `document::` makes the SSR output and the
// post-hydration head identical. The static copies in `index.html` keep the
// prerendered page styled before the wasm arrives and cover no-JS clients.
const APP_TITLE: &str = "Companion — a transparent VRM avatar on your desktop";
const SITE_DESCRIPTION: &str = "Companion puts a transparent VRM avatar on your desktop — the visual body of your AI agent. Open-source, plugin-based, and built with Tauri v2.";
const SOCIAL_DESCRIPTION: &str = "The visual body of your AI agent: a transparent VRM avatar that lives on your desktop. Open-source, plugin-based, built with Tauri v2.";

fn main() {
    #[cfg(not(all(feature = "server", not(target_arch = "wasm32"))))]
    dioxus::launch(App);

    #[cfg(all(feature = "server", not(target_arch = "wasm32")))]
    dioxus::serve(|| async move {
        // Incremental rendering is what makes `dx build --ssg` work: when
        // dx GETs each route from the static_routes list, the renderer
        // caches the final HTML into the deploy's public/ dir — the static
        // pages the Pages deployment serves. The cache lives next to the
        // server binary (target/dx/…/web/public in a release build).
        let cfg = dioxus::server::ServeConfig::builder()
            .incremental(
                dioxus::server::IncrementalRendererConfig::new()
                    .static_dir(
                        std::env::current_exe()
                            .unwrap()
                            .parent()
                            .unwrap()
                            .join("public"),
                    )
                    // The public dir holds the whole deploy artifact (wasm,
                    // assets, prerendered pages) — never wipe it on startup.
                    .clear_cache(false),
            )
            .enable_out_of_order_streaming();

        let mut router = dioxus::server::axum::Router::new()
            .serve_dioxus_application(cfg.clone(), App);

        // The build-upload API needs Postgres + S3 (DATABASE_URL, UPLOAD_TOKEN,
        // S3_BUCKET, AWS_*). When they are not configured — e.g. the `--ssg`
        // prerender run or a plain `dx serve` of the landing page — degrade
        // gracefully: the site works and the API endpoints answer 503 (see
        // builds::storage::current_state).
        let build_state = crate::server::builds::AppState::init().await.ok();
        if let Some(state) = build_state {
            // The extension carries the storage state into the server
            // functions; DefaultBodyLimit raises the default 2 MB request
            // limit so multipart uploads can pass.
            let max_upload_bytes = state.max_upload_bytes;
            router = router
                .layer(dioxus::server::axum::Extension(state))
                .layer(dioxus::server::axum::extract::DefaultBodyLimit::max(
                    max_upload_bytes,
                ));
        }

        // Server functions register through `inventory`, which only keeps
        // submissions that survive dead-code elimination. Nothing else in
        // the binary calls `static_routes` — reference it so the endpoint
        // exists for dx's SSG prerender to query.
        let _ = crate::server::static_routes;

        Ok(router)
    });
}

#[component]
fn App() -> Element {
    rsx! {
        // ── Document head (re-applied by the client on hydration) ──
        document::Title { {APP_TITLE} }
        document::Meta { name: "description", content: SITE_DESCRIPTION }
        document::Meta { name: "theme-color", content: "#05010f" }
        // Social preview cards
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:site_name", content: "Companion" }
        document::Meta { property: "og:title", content: APP_TITLE }
        document::Meta { property: "og:description", content: SOCIAL_DESCRIPTION }
        document::Meta { property: "og:url", content: "https://companion-lab.github.io/" }
        document::Meta { property: "og:image", content: "https://companion-lab.github.io/icon.png" }
        document::Meta { name: "twitter:card", content: "summary" }
        document::Meta { name: "twitter:title", content: APP_TITLE }
        document::Meta { name: "twitter:description", content: SOCIAL_DESCRIPTION }
        document::Meta { name: "twitter:image", content: "https://companion-lab.github.io/icon.png" }
        // Icons (static fallbacks also live in index.html so browsers never
        // hit the /favicon.ico 404 fallback while loading)
        document::Link { rel: "icon", r#type: "image/png", href: "/favicon.png" }
        document::Link { rel: "icon", r#type: "image/x-icon", href: "/favicon.ico" }
        document::Link { rel: "apple-touch-icon", href: "/icon.png" }
        // Stylesheets (unhashed, from public/ — see index.html)
        document::Link { rel: "stylesheet", href: "/main.css" }
        document::Link { rel: "stylesheet", href: "/tailwind.css" }
        // Fonts — non-blocking (media=print swap); see index.html for why.
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "anonymous" }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&display=swap",
            media: "print",
            onload: "this.media='all'",
        }
        // The document model clears the <html> attributes on hydration; the
        // language must be re-stated after it (and the same script keeps the
        // prerendered page correct for no-JS clients).
        document::Script { "document.documentElement.lang = 'en'" }

        Router::<Route> {}
    }
}
