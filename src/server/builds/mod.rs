//! Build-upload API server functions.
//!
//! `upload_build` receives artifacts from GitHub CI/CD (authenticated multipart
//! upload) and stores them in Postgres + S3; `latest_build` serves the newest
//! build for a platform with a presigned download URL. All Postgres/S3
//! machinery lives in [`storage`], which compiles only for the server target —
//! the function signatures and client stubs stay wasm-safe.

#[cfg(feature = "server")]
mod storage;

#[cfg(feature = "server")]
pub(crate) use storage::AppState;

// The prelude glob is load-bearing here: the method-macro expansion refers to
// the `dioxus_fullstack` and `dioxus_server` crates, which it re-exports.
use dioxus::prelude::*;

#[cfg(feature = "server")]
use dioxus::fullstack::{http::HeaderMap, MultipartFormData};

/// Metadata for one stored build artifact, as returned by the API.
// Only constructed server-side; on client builds it exists for the stub's type.
#[cfg_attr(not(feature = "server"), allow(dead_code))]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BuildResponse {
    pub id: i64,
    pub platform: String,
    pub version: String,
    pub channel: String,
    pub filename: String,
    pub size_bytes: i64,
    pub sha256: String,
    pub s3_key: String,
    pub content_type: Option<String>,
    pub commit_sha: Option<String>,
    pub ci_run_id: Option<String>,
    pub created_at: String,
}

/// The newest build for a platform, with an expiring presigned download URL.
#[cfg_attr(not(feature = "server"), allow(dead_code))]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LatestBuildResponse {
    #[serde(flatten)]
    pub build: BuildResponse,
    pub download_url: String,
    pub url_expires_in_secs: u64,
}

/// POST /api/builds — authenticated multipart upload of one build artifact
/// from CI. See `docs/upload-api.md` for the field list and curl examples.
#[post("/api/builds", headers: HeaderMap, multipart: MultipartFormData)]
pub async fn upload_build() -> Result<BuildResponse, ServerFnError> {
    storage::upload_build(headers, multipart).await
}

/// GET /api/builds/latest/:platform?channel=stable — public lookup of the most
/// recent build for a platform with a presigned, expiring download URL.
#[get("/api/builds/latest/:platform?channel")]
pub async fn latest_build(
    platform: String,
    channel: Option<String>,
) -> Result<LatestBuildResponse, ServerFnError> {
    storage::latest_build(&platform, channel).await
}
