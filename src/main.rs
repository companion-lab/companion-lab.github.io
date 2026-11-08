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

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    #[cfg(not(all(feature = "server", not(target_arch = "wasm32"))))]
    dioxus::launch(App);

    #[cfg(all(feature = "server", not(target_arch = "wasm32")))]
    dioxus::serve(|| async move {
        // The build-upload API needs Postgres + S3 (DATABASE_URL, UPLOAD_TOKEN,
        // S3_BUCKET, AWS_*). When they are not configured — e.g. a plain
        // `dx serve` of the landing page — degrade gracefully: the site works
        // and the API endpoints answer 503 (see builds::storage::current_state).
        let build_state = match crate::server::builds::AppState::init().await {
            Ok(state) => Some(state),
            Err(e) => {
                tracing::error!(
                    error = %e,
                    "build-upload API disabled: build storage failed to initialize"
                );
                None
            }
        };

        let mut router = dioxus::server::router(App);
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

        Ok(router)
    })
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        // Fonts
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "anonymous" }
        document::Link { rel: "stylesheet", href: "https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;800&display=swap" }
        // FontAwesome
        document::Link { rel: "stylesheet", href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css" }

        // Body attributes handled by wrapper? Use html class scroll-smooth via document? skip
        Router::<Route> {}
    }
}
