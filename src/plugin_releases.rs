//! Per-plugin download options, resolved from the registered plugins' GitHub
//! releases (the catalog comes from the community plugin registry — see
//! src/plugin_catalog.rs).
//!
//! Every registered plugin repo publishes installable zips on its GitHub
//! releases when a `v*` tag is pushed: backend plugins get one zip per
//! platform (`<id>-<version>-macos-arm64.zip`, …), frontend-only plugins a
//! single universal zip (`<id>-<version>.zip`). The zips import into the
//! Companion app via Settings → Plugins → Import plugin…
//!
//! The Pages deployment is a static client-only bundle, so — like
//! [crate::latest_release] — the lookup runs in the wasm client. Unlike the
//! shared app-download catalog, plugin releases are fetched lazily, one repo
//! at a time, only when a visitor opens a plugin card's download panel: eight
//! upfront requests would burn a meaningful share of the unauthenticated
//! 60 req/h per-IP budget.
//!
//! Two sources, tried in order:
//! 1. the GitHub Releases API — exact zip assets and pre-release flags;
//! 2. the jsDelivr data API — used when the GitHub API rate-limits the
//!    visitor's IP (shared networks exhaust the 60 req/h easily). jsDelivr
//!    serves each repo's version tags CORS-open without meaningful limits;
//!    zip names are derived from the repos' uniform release convention
//!    (`<id>-<version>[-<platform>].zip`, tag `v<version>` — enforced by the
//!    release workflow), so it degrades gracefully: no pre-release flags,
//!    and a repo whose tags were pushed without the release job shows
//!    versions whose zips 404 (the panel keeps the GitHub releases page one
//!    click away for that case).

use crate::pages::PluginEntry;

/// Platform key parsed from a release asset's filename suffix; `None` on
/// [PluginZip] marks a universal (frontend-only) zip that runs everywhere.
/// Off the web nothing is ever parsed into it (the lookup has a stub), so
/// its variants would read as dead.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub enum PluginPlatform {
    Windows,
    MacosArm64,
    MacosX86_64,
    Linux,
}

impl PluginPlatform {
    /// Filename suffix this platform's zip carries
    /// (`<id>-<version><suffix>`); frontend-only universal zips have none.
    #[cfg(target_arch = "wasm32")]
    pub fn suffix(self) -> &'static str {
        match self {
            Self::Windows => "-windows-x86_64.zip",
            Self::MacosArm64 => "-macos-arm64.zip",
            Self::MacosX86_64 => "-macos-x86_64.zip",
            Self::Linux => "-linux-x86_64.zip",
        }
    }

    /// Parse the platform from an asset filename's suffix
    /// (`<id>-<version>-<platform>.zip`); `None` when the name carries no
    /// platform suffix (frontend-only universal zip).
    #[cfg(target_arch = "wasm32")]
    pub fn from_filename(name: &str) -> Option<Self> {
        [
            Self::Windows,
            Self::MacosArm64,
            Self::MacosX86_64,
            Self::Linux,
        ]
        .into_iter()
        .find(|platform| name.ends_with(platform.suffix()))
    }

    /// Display label in the download panel's platform row.
    pub fn label(self) -> &'static str {
        match self {
            Self::Windows => "Windows",
            Self::MacosArm64 => "macOS (Apple silicon)",
            Self::MacosX86_64 => "macOS (Intel)",
            Self::Linux => "Linux",
        }
    }

    /// FontAwesome icon for the platform row.
    pub fn icon(self) -> &'static str {
        match self {
            Self::Windows => "fa-brands fa-windows",
            Self::MacosArm64 | Self::MacosX86_64 => "fa-brands fa-apple",
            Self::Linux => "fa-brands fa-linux",
        }
    }
}

/// One downloadable zip on a plugin release.
#[derive(Clone, PartialEq, Debug)]
pub struct PluginZip {
    /// `None` for a universal (frontend-only) zip.
    pub platform: Option<PluginPlatform>,
    pub url: String,
}

/// One published release of a plugin repo (the sources serve them
/// newest-first).
#[derive(Clone, PartialEq, Debug)]
pub struct PluginRelease {
    /// Release tag as shown on GitHub (e.g. `v0.3.0`).
    pub tag: String,
    /// GitHub marks pre-releases; they stay selectable but never default.
    /// The CDN fallback cannot tell them apart and reports `false`.
    pub prerelease: bool,
    /// The release's installable zips (source archives are not assets, so
    /// they never appear here).
    pub zips: Vec<PluginZip>,
}

/// The visitor's platform, sniffed from the user agent — the download panel
/// orders and highlights that platform's zip first. `None` when detection
/// fails or runs off the web; the panel then lists every platform as-is.
pub fn detect_platform() -> Option<PluginPlatform> {
    #[cfg(target_arch = "wasm32")]
    {
        let ua = web_sys::window()?.navigator().user_agent().ok()?;
        if ua.contains("Mac") {
            // Apple Silicon is the only published macOS target (same
            // assumption as the app download buttons).
            Some(PluginPlatform::MacosArm64)
        } else if ua.contains("Linux") || ua.contains("X11") {
            Some(PluginPlatform::Linux)
        } else {
            Some(PluginPlatform::Windows)
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}

/// Query one plugin's published releases, newest first: the GitHub Releases
/// API first, the jsDelivr mirror when the API rate-limits the visitor (see
/// the module docs for the trade-offs).
pub async fn fetch_releases(entry: &PluginEntry) -> Result<Vec<PluginRelease>, String> {
    fetch_impl(entry).await
}

#[cfg(target_arch = "wasm32")]
async fn fetch_impl(entry: &PluginEntry) -> Result<Vec<PluginRelease>, String> {
    match fetch_releases_via_api(entry.repo).await {
        Ok(releases) => Ok(releases),
        // The unauthenticated API is capped per IP (60 req/h) — shared
        // networks exhaust it and every visitor there sees HTTP 403. The
        // CDN mirror keeps the panel working regardless of the IP's budget.
        Err(api_error) => fetch_releases_via_cdn(entry)
            .await
            .map_err(|cdn_error| format!("{api_error}; CDN mirror failed: {cdn_error}")),
    }
}

/// Off the web there is no runtime release lookup (the static wasm client
/// never asks the server) — the panel falls back to a link to the GitHub
/// releases page.
#[cfg(not(target_arch = "wasm32"))]
async fn fetch_impl(_entry: &PluginEntry) -> Result<Vec<PluginRelease>, String> {
    Err("The release listing is only available on the web build.".to_string())
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

/// Query the GitHub Releases API for one plugin repo's published releases,
/// newest first, mapping each zip asset to its platform by filename suffix.
/// `repo` is the registry entry's full `owner/name` slug. Unauthenticated
/// request: api.github.com serves open CORS here.
#[cfg(target_arch = "wasm32")]
async fn fetch_releases_via_api(repo: &str) -> Result<Vec<PluginRelease>, String> {
    let url = format!(
        "{}/{}/releases?per_page=25",
        crate::config::PLUGIN_RELEASES_API_URL, repo
    );
    let response = gloo_net::http::Request::get(&url)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    // An API error body is a JSON object, which would fail the Vec parse
    // below with a cryptic message — surface the status instead (403 is the
    // unauthenticated rate limit).
    if response.status() != 200 {
        return Err(format!("GitHub API returned HTTP {}", response.status()));
    }
    let releases: Vec<release_api::Release> = response
        .json() // parses the body with serde (T inferred from this binding)
        .await
        .map_err(|e| e.to_string())?;

    Ok(releases
        .into_iter()
        // Unauthenticated callers never see drafts, but stay safe regardless.
        .filter(|release| !release.draft)
        .map(|release| PluginRelease {
            tag: release.tag_name,
            prerelease: release.prerelease,
            zips: release
                .assets
                .into_iter()
                .filter(|asset| asset.name.ends_with(".zip"))
                .map(|asset| PluginZip {
                    platform: PluginPlatform::from_filename(&asset.name),
                    url: asset.browser_download_url,
                })
                .collect(),
        })
        .collect())
}

/// jsDelivr package JSON the fallback needs (subset of the data-API shape).
#[cfg(target_arch = "wasm32")]
mod cdn_api {
    #[derive(serde::Deserialize)]
    pub struct Package {
        /// Version tags of the repo, newest first, normalized without the
        /// leading `v` (jsDelivr rewrites `v0.3.0` to `0.3.0`).
        #[serde(default)]
        pub versions: Vec<Version>,
    }
    #[derive(serde::Deserialize)]
    pub struct Version {
        pub version: String,
    }
}

/// Query the jsDelivr data API for one plugin repo's version tags and derive
/// each version's installable zips from the release naming convention. Only
/// reached when the Releases API fails (rate limit); see the module docs.
#[cfg(target_arch = "wasm32")]
async fn fetch_releases_via_cdn(entry: &PluginEntry) -> Result<Vec<PluginRelease>, String> {
    let url = format!("{}/{}/", crate::config::PLUGIN_RELEASES_CDN_URL, entry.repo);
    let response = gloo_net::http::Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if response.status() != 200 {
        return Err(format!("CDN returned HTTP {}", response.status()));
    }
    let package: cdn_api::Package = response
        .json() // parses the body with serde (T inferred from this binding)
        .await
        .map_err(|e| e.to_string())?;

    Ok(package
        .versions
        .into_iter()
        .map(|version| {
            let tag = format!("v{}", version.version);
            PluginRelease {
                zips: derived_zips(entry, &version.version, &tag),
                tag,
                // Not exposed by the CDN; no chip renders, and the "latest"
                // marker lands on the first (newest) row.
                prerelease: false,
            }
        })
        .collect())
}

/// Derive one release's installable zips from the repos' uniform naming:
/// frontend-only plugins ship one `<id>-<version>.zip`, backend plugins one
/// `<id>-<version>-<platform>.zip` per platform, attached to the release's
/// `v`-tagged download URLs.
#[cfg(target_arch = "wasm32")]
fn derived_zips(entry: &PluginEntry, version: &str, tag: &str) -> Vec<PluginZip> {
    let base = format!(
        "{}/{}/releases/download/{tag}/{}-{version}",
        crate::config::GITHUB_URL,
        entry.repo,
        entry.id
    );
    if entry.backend {
        [
            PluginPlatform::Windows,
            PluginPlatform::MacosArm64,
            PluginPlatform::MacosX86_64,
            PluginPlatform::Linux,
        ]
        .into_iter()
        .map(|platform| PluginZip {
            platform: Some(platform),
            // suffix already carries the .zip extension (it doubles as the
            // filename matcher in `from_filename`).
            url: format!("{base}{}", platform.suffix()),
        })
        .collect()
    } else {
        vec![PluginZip {
            platform: None,
            url: format!("{base}.zip"),
        }]
    }
}
