//! Download links resolved from the latest GitHub release.
//!
//! Release assets embed the version in their filenames (`companion-0.0.2-
//! macos-arm64.zip`), so a stable per-platform link cannot be baked in: the
//! wasm client asks the GitHub Releases API for the latest release and picks
//! the asset matching each platform by filename suffix. The result lives in
//! one root-provided [Signal] — [crate::components::DownloadButtons] falls
//! back to the releases page whenever a platform link is not (yet) resolved,
//! which covers bots, SSR renders, rate limits and offline visits.

use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::config::RELEASES_API_URL;

/// Per-platform direct download URLs in DownloadButtons' platform order
/// (0 = Windows, 1 = macOS, 2 = Linux); `None` means "not resolved (yet)".
#[derive(Clone, PartialEq, Debug, Default)]
pub struct LatestAssets {
    pub urls: [Option<String>; 3],
}

/// Asset filename suffix per platform index (resolved on the web only).
#[cfg(target_arch = "wasm32")]
const PLATFORM_SUFFIXES: [&str; 3] = [
    "-windows-x86_64.zip", // Windows
    "-macos-arm64.zip",    // macOS (Apple Silicon is the only published target)
    "-linux-x86_64.zip",   // Linux
];

/// Provide the shared [LatestAssets] signal before the router mounts, and on
/// the web kick off the one resolver fetch per page load (hero + CTA both
/// render download buttons from this single shared signal).
pub fn provide() {
    let latest: Signal<LatestAssets> =
        use_context_provider(|| Signal::new(LatestAssets::default()));

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        let mut latest = latest.clone();
        spawn(async move {
            if let Ok(urls) = fetch_latest_assets().await {
                latest.set(LatestAssets { urls });
            }
        });
    });
    #[allow(unused_variables)]
    let latest = latest; // Non-web targets simply keep the fallback links.
}

/// Query the GitHub Releases API for the latest release and pick each
/// platform's installer zip by filename suffix. Unauthenticated request:
/// api.github.com serves open CORS here, and one request per page load sits
/// well under its 60 req/h per-IP limit.
#[cfg(target_arch = "wasm32")]
async fn fetch_latest_assets() -> Result<[Option<String>; 3], gloo_net::Error> {
    #[derive(serde::Deserialize)]
    struct Release {
        #[allow(dead_code)]
        tag_name: String,
        assets: Vec<Asset>,
    }
    #[derive(serde::Deserialize)]
    struct Asset {
        name: String,
        browser_download_url: String,
    }

    let release: Release = gloo_net::http::Request::get(RELEASES_API_URL)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await?
        .json() // parses the body with serde (T inferred from this binding)
        .await?;

    let mut urls: [Option<String>; 3] = Default::default();
    for asset in release.assets {
        for (idx, suffix) in PLATFORM_SUFFIXES.iter().enumerate() {
            if urls[idx].is_none()
                && asset.name.starts_with("companion-")
                && asset.name.ends_with(suffix)
            {
                urls[idx] = Some(asset.browser_download_url.clone());
            }
        }
    }
    Ok(urls)
}
