//! Download links resolved from the GitHub releases list.
//!
//! Release assets embed the version in their filenames (`companion-0.0.2-
//! macos-arm64.zip`), so stable per-platform links cannot be baked in: the
//! wasm client asks the GitHub Releases API for the published releases and
//! picks the asset matching each platform by filename suffix. The catalog
//! lives in one root-provided [Signal] — [crate::components::DownloadButtons]
//! renders a version selector from it (letting visitors pin an older release)
//! and falls back to the releases page whenever a platform link is not (yet)
//! resolved, which covers bots, SSR renders, rate limits and offline visits.

use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::config::RELEASES_API_URL;

/// Per-platform download URLs of one release, in DownloadButtons' platform
/// order (0 = Windows, 1 = macOS, 2 = Linux); `None` means the release ships
/// no asset for that platform.
#[derive(Clone, PartialEq, Debug)]
pub struct ReleaseAssets {
    /// Release tag as shown on GitHub (e.g. `v0.3.0`).
    pub tag: String,
    /// GitHub marks pre-releases; they stay selectable but never default.
    pub prerelease: bool,
    pub urls: [Option<String>; 3],
}

/// The shared download catalog: every published release (newest first) plus
/// the visitor's version pick. `selected` is `None` until the visitor picks
/// one, resolving to the latest stable release.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct DownloadCatalog {
    pub releases: Vec<ReleaseAssets>,
    pub selected: Option<String>,
}

impl DownloadCatalog {
    /// The default release: the newest stable one, falling back to the first
    /// entry (the API serves releases newest-first).
    pub fn latest(&self) -> Option<&ReleaseAssets> {
        self.releases
            .iter()
            .find(|release| !release.prerelease)
            .or_else(|| self.releases.first())
    }

    /// The release the download buttons currently point at: the visitor's
    /// pick, or the latest stable release when nothing is picked (yet). A
    /// stale pick (release gone since it was made) falls back to the latest.
    pub fn resolved(&self) -> Option<&ReleaseAssets> {
        match &self.selected {
            Some(tag) => self
                .releases
                .iter()
                .find(|release| &release.tag == tag)
                .or_else(|| self.latest()),
            None => self.latest(),
        }
    }
}

/// Asset filename suffix per platform index (resolved on the web only).
#[cfg(target_arch = "wasm32")]
const PLATFORM_SUFFIXES: [&str; 3] = [
    "-windows-x86_64.zip", // Windows
    "-macos-arm64.zip",    // macOS (Apple Silicon is the only published target)
    "-linux-x86_64.zip",   // Linux
];

/// Provide the shared [DownloadCatalog] signal before the router mounts, and
/// on the web kick off the one resolver fetch per page load (hero + CTA both
/// render download buttons from this single shared signal). Non-web targets
/// simply keep the fallback links, leaving the binding unused — hence the
/// cfg-gated allows.
pub fn provide() {
    #[cfg_attr(not(target_arch = "wasm32"), allow(unused_mut, unused_variables))]
    let mut catalog: Signal<DownloadCatalog> =
        use_context_provider(|| Signal::new(DownloadCatalog::default()));

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        spawn(async move {
            if let Ok(releases) = fetch_releases().await {
                catalog.with_mut(|catalog| catalog.releases = releases);
            }
        });
    });
}

/// GitHub release JSON the resolver needs (subset of the Releases API shape).
#[cfg(target_arch = "wasm32")]
mod release_api {
    #[derive(serde::Deserialize)]
    pub struct Release {
        pub tag_name: String,
        #[serde(default)]
        pub prerelease: bool,
        #[serde(default)]
        pub draft: bool,
        pub assets: Vec<Asset>,
    }
    #[derive(serde::Deserialize)]
    pub struct Asset {
        pub name: String,
        pub browser_download_url: String,
    }
}

/// Query the GitHub Releases API for the published releases and pick each
/// release's installer zip per platform by filename suffix. Unauthenticated
/// request: api.github.com serves open CORS here, and one request per page
/// load sits well under its 60 req/h per-IP limit.
#[cfg(target_arch = "wasm32")]
async fn fetch_releases() -> Result<Vec<ReleaseAssets>, gloo_net::Error> {
    let releases: Vec<release_api::Release> = gloo_net::http::Request::get(RELEASES_API_URL)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await?
        .json() // parses the body with serde (T inferred from this binding)
        .await?;

    Ok(releases
        .into_iter()
        // Unauthenticated callers never see drafts, but stay safe regardless.
        .filter(|release| !release.draft)
        .map(|release| ReleaseAssets {
            tag: release.tag_name,
            prerelease: release.prerelease,
            urls: platform_urls(&release.assets),
        })
        .collect())
}

/// Match each asset filename against the per-platform suffixes
/// (`companion-0.0.2-macos-arm64.zip`); a release may ship no asset for a
/// platform (e.g. macOS-only builds early on), leaving that link `None`.
#[cfg(target_arch = "wasm32")]
fn platform_urls(assets: &[release_api::Asset]) -> [Option<String>; 3] {
    let mut urls: [Option<String>; 3] = Default::default();
    for asset in assets {
        for (idx, suffix) in PLATFORM_SUFFIXES.iter().enumerate() {
            if urls[idx].is_none()
                && asset.name.starts_with("companion-")
                && asset.name.ends_with(suffix)
            {
                urls[idx] = Some(asset.browser_download_url.clone());
            }
        }
    }
    urls
}
