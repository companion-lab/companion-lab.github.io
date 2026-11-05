//! Server-only implementation of the build-upload API: Postgres storage, S3
//! uploads, auth and validation. This module is compiled only for the server
//! target; the server function signatures in `super` stay wasm-safe.

use std::time::Duration;

use anyhow::Context;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::primitives::ByteStream;
use dioxus::fullstack::{
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    FullstackContext, HttpError, MultipartFormData, ServerFnError,
};
use dioxus::server::axum::extract::multipart::Field;
use sha2::{Digest, Sha256};
use tokio::io::AsyncWriteExt;

use super::{BuildResponse, LatestBuildResponse};

fn api_err(status: StatusCode, message: impl Into<String>) -> ServerFnError {
    ServerFnError::from(HttpError::new(status, message))
}

type ApiResult<T> = Result<T, ServerFnError>;

#[derive(Clone)]
pub(crate) struct AppState {
    pub pool: sqlx::PgPool,
    pub s3: aws_sdk_s3::Client,
    pub bucket: String,
    pub upload_token: String,
    pub max_upload_bytes: usize,
    pub presign_expires_secs: u64,
}

impl AppState {
    pub async fn init() -> anyhow::Result<Self> {
        let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
        let upload_token = std::env::var("UPLOAD_TOKEN").context("UPLOAD_TOKEN must be set")?;
        let bucket = std::env::var("S3_BUCKET").context("S3_BUCKET must be set")?;

        let max_upload_mb = std::env::var("MAX_UPLOAD_MB")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(500);
        let presign_expires_secs = std::env::var("PRESIGN_EXPIRES_SECS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(3600);

        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .context("failed to connect to Postgres")?;
        sqlx::migrate!()
            .run(&pool)
            .await
            .context("failed to run migrations")?;

        // Honors the standard AWS_* variables: AWS_REGION, AWS_ACCESS_KEY_ID,
        // AWS_SECRET_ACCESS_KEY, and AWS_ENDPOINT_URL (for MinIO / R2).
        let aws_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .load()
            .await;
        // S3-compatible servers (MinIO, SeaweedFS, R2) need path-style
        // addressing, which the SDK only uses for custom endpoints.
        let s3_config = if std::env::var_os("AWS_ENDPOINT_URL").is_some() {
            aws_sdk_s3::Config::from(&aws_config)
                .to_builder()
                .force_path_style(true)
                .build()
        } else {
            aws_sdk_s3::Config::from(&aws_config)
        };
        let s3 = aws_sdk_s3::Client::from_conf(s3_config);

        // Ensure the bucket exists so the first upload works out of the box.
        // With real AWS this is a no-op when the bucket is already there.
        if let Err(head_err) = s3.head_bucket().bucket(&bucket).send().await {
            tracing::info!(bucket = %bucket, error = %head_err, "S3 bucket not found, attempting to create it");
            match s3.create_bucket().bucket(&bucket).send().await {
                Ok(_) => {}
                Err(e) => {
                    let already_exists = e
                        .as_service_error()
                        .is_some_and(|se| {
                            se.is_bucket_already_exists() || se.is_bucket_already_owned_by_you()
                        });
                    if !already_exists {
                        return Err(anyhow::Error::new(e).context(format!(
                            "S3 bucket '{bucket}' does not exist and could not be created"
                        )));
                    }
                }
            }
        }

        Ok(Self {
            pool,
            s3,
            bucket,
            upload_token,
            max_upload_bytes: max_upload_mb * 1024 * 1024,
            presign_expires_secs,
        })
    }
}

/// Column list shared by the insert and lookup queries; `created_at` is
/// formatted as an RFC3339 string in UTC so no extra datetime crate is needed.
const BUILD_COLUMNS: &str = r#"id, platform, version, channel, filename, size_bytes, sha256, s3_key, content_type, commit_sha, ci_run_id,
    to_char(created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS created_at"#;

#[derive(sqlx::FromRow)]
struct BuildRow {
    id: i64,
    platform: String,
    version: String,
    channel: String,
    filename: String,
    size_bytes: i64,
    sha256: String,
    s3_key: String,
    content_type: Option<String>,
    commit_sha: Option<String>,
    ci_run_id: Option<String>,
    created_at: String,
}

impl From<BuildRow> for BuildResponse {
    fn from(row: BuildRow) -> Self {
        Self {
            id: row.id,
            platform: row.platform,
            version: row.version,
            channel: row.channel,
            filename: row.filename,
            size_bytes: row.size_bytes,
            sha256: row.sha256,
            s3_key: row.s3_key,
            content_type: row.content_type,
            commit_sha: row.commit_sha,
            ci_run_id: row.ci_run_id,
            created_at: row.created_at,
        }
    }
}

/// Reads the storage state injected as an `Extension` on the router in `main`.
/// The extension is absent when the API is disabled (storage env vars not
/// configured — see `main.rs`), which turns every endpoint into a 503.
fn current_state() -> ApiResult<AppState> {
    FullstackContext::current()
        .and_then(|ctx| ctx.extension::<AppState>())
        .ok_or_else(|| {
            api_err(
                StatusCode::SERVICE_UNAVAILABLE,
                "build-upload API is not available: storage is not configured \
                 (requires DATABASE_URL, UPLOAD_TOKEN, S3_BUCKET and AWS_* env vars)",
            )
        })
}

#[allow(clippy::too_many_lines)]
pub(super) async fn upload_build(
    headers: HeaderMap,
    mut multipart: MultipartFormData,
) -> ApiResult<BuildResponse> {
    let state = current_state()?;
    check_auth(&state, &headers)?;

    let mut meta = UploadMeta::default();
    let mut file: Option<StoredUpload> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| api_err(StatusCode::BAD_REQUEST, format!("invalid multipart body: {e}")))?
    {
        match field.name() {
            Some("file") => file = Some(store_upload(field, &state).await?),
            Some("platform") => meta.platform = Some(read_text(field, 32, "platform").await?),
            Some("version") => meta.version = Some(read_text(field, 64, "version").await?),
            Some("channel") => meta.channel = Some(read_text(field, 32, "channel").await?),
            Some("commit_sha") => {
                let value = read_text(field, 64, "commit_sha").await?;
                if !value.is_empty() {
                    meta.commit_sha = Some(value);
                }
            }
            Some("run_id") => {
                let value = read_text(field, 64, "run_id").await?;
                if !value.is_empty() {
                    meta.run_id = Some(value);
                }
            }
            _ => {}
        }
    }

    let platform = meta
        .platform
        .ok_or_else(|| api_err(StatusCode::BAD_REQUEST, "missing 'platform' field"))?
        .to_lowercase();
    validate_slug(&platform, "platform")?;
    let version = meta
        .version
        .ok_or_else(|| api_err(StatusCode::BAD_REQUEST, "missing 'version' field"))?;
    validate_version(&version)?;
    let channel = meta
        .channel
        .unwrap_or_else(|| "stable".to_string())
        .to_lowercase();
    validate_slug(&channel, "channel")?;
    let upload = file.ok_or_else(|| api_err(StatusCode::BAD_REQUEST, "missing 'file' field"))?;

    let s3_key = format!("builds/{channel}/{platform}/{version}/{}", upload.filename);

    let body = ByteStream::from_path(upload.temp.path())
        .await
        .map_err(|e| api_err(StatusCode::INTERNAL_SERVER_ERROR, format!("failed to stage upload: {e}")))?;
    state
        .s3
        .put_object()
        .bucket(&state.bucket)
        .key(&s3_key)
        .body(body)
        .content_type(
            upload
                .content_type
                .clone()
                .unwrap_or_else(|| "application/octet-stream".to_string()),
        )
        .send()
        .await
        .map_err(|e| api_err(StatusCode::BAD_GATEWAY, format!("S3 upload failed: {e}")))?;

    let row = sqlx::query_as::<_, BuildRow>(&format!(
        "INSERT INTO builds
            (platform, version, channel, filename, size_bytes, sha256, s3_key, content_type, commit_sha, ci_run_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
         RETURNING {BUILD_COLUMNS}"
    ))
    .bind(&platform)
    .bind(&version)
    .bind(&channel)
    .bind(&upload.filename)
    .bind(upload.size_bytes as i64)
    .bind(&upload.sha256)
    .bind(&s3_key)
    .bind(upload.content_type.as_deref())
    .bind(meta.commit_sha.as_deref())
    .bind(meta.run_id.as_deref())
    .fetch_one(&state.pool)
    .await;

    if let Err(e) = &row {
        // The object is already in S3 but not recorded in Postgres; an
        // orphan is preferable to failing the whole upload silently.
        tracing::warn!(%e, s3_key, "build uploaded to S3 but not recorded in Postgres");
    }
    let row = row
        .map_err(|e| api_err(StatusCode::INTERNAL_SERVER_ERROR, format!("failed to record build: {e}")))?;

    Ok(row.into())
}

pub(super) async fn latest_build(
    platform: &str,
    channel: Option<String>,
) -> ApiResult<LatestBuildResponse> {
    let state = current_state()?;

    let platform = platform.to_lowercase();
    validate_slug(&platform, "platform")?;
    let channel = channel.unwrap_or_else(|| "stable".to_string()).to_lowercase();
    validate_slug(&channel, "channel")?;

    let row = sqlx::query_as::<_, BuildRow>(&format!(
        "SELECT {BUILD_COLUMNS} FROM builds
         WHERE platform = $1 AND channel = $2
         ORDER BY created_at DESC
         LIMIT 1"
    ))
    .bind(&platform)
    .bind(&channel)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| api_err(StatusCode::INTERNAL_SERVER_ERROR, format!("failed to query builds: {e}")))?
    .ok_or_else(|| {
        api_err(
            StatusCode::NOT_FOUND,
            format!("no build found for platform '{platform}' on channel '{channel}'"),
        )
    })?;

    let presigning = PresigningConfig::expires_in(Duration::from_secs(state.presign_expires_secs))
        .map_err(|e| api_err(StatusCode::INTERNAL_SERVER_ERROR, format!("invalid presign expiry: {e}")))?;
    let url = state
        .s3
        .get_object()
        .bucket(&state.bucket)
        .key(&row.s3_key)
        .presigned(presigning)
        .await
        .map_err(|e| api_err(StatusCode::INTERNAL_SERVER_ERROR, format!("failed to presign download URL: {e}")))?;

    Ok(LatestBuildResponse {
        build: row.into(),
        download_url: url.uri().to_string(),
        url_expires_in_secs: state.presign_expires_secs,
    })
}

#[derive(Default)]
struct UploadMeta {
    platform: Option<String>,
    version: Option<String>,
    channel: Option<String>,
    commit_sha: Option<String>,
    run_id: Option<String>,
}

struct StoredUpload {
    /// Held so the temp file stays alive until after the S3 upload; deleted
    /// from disk when dropped.
    temp: tempfile::NamedTempFile,
    filename: String,
    content_type: Option<String>,
    size_bytes: u64,
    sha256: String,
}

/// Stream the file field to a self-deleting temp file, hashing it as it goes
/// and enforcing the configured size limit.
async fn store_upload(field: Field<'_>, state: &AppState) -> ApiResult<StoredUpload> {
    let filename = field
        .file_name()
        .ok_or_else(|| api_err(StatusCode::BAD_REQUEST, "'file' field must include a filename"))?
        .to_string();
    let filename = sanitize_filename(&filename)?;
    let content_type = field.content_type().map(str::to_owned);

    let temp = tempfile::NamedTempFile::new()
        .map_err(|e| api_err(StatusCode::INTERNAL_SERVER_ERROR, format!("failed to stage upload: {e}")))?;
    let mut out = tokio::fs::File::from_std(
        temp.reopen()
            .map_err(|e| api_err(StatusCode::INTERNAL_SERVER_ERROR, format!("failed to stage upload: {e}")))?,
    );

    let mut hasher = Sha256::new();
    let mut size: u64 = 0;
    let mut field = field;
    while let Some(chunk) = field
        .chunk()
        .await
        .map_err(|e| api_err(StatusCode::BAD_REQUEST, format!("failed to read upload: {e}")))?
    {
        size += chunk.len() as u64;
        if size > state.max_upload_bytes as u64 {
            return Err(api_err(
                StatusCode::PAYLOAD_TOO_LARGE,
                format!(
                    "upload exceeds the {} MB limit",
                    state.max_upload_bytes / (1024 * 1024)
                ),
            ));
        }
        hasher.update(&chunk);
        out.write_all(&chunk)
            .await
            .map_err(|e| api_err(StatusCode::INTERNAL_SERVER_ERROR, format!("failed to stage upload: {e}")))?;
    }
    out.flush()
        .await
        .map_err(|e| api_err(StatusCode::INTERNAL_SERVER_ERROR, format!("failed to stage upload: {e}")))?;

    Ok(StoredUpload {
        temp,
        filename,
        content_type,
        size_bytes: size,
        sha256: hex::encode(hasher.finalize()),
    })
}

async fn read_text(field: Field<'_>, max_len: usize, name: &str) -> ApiResult<String> {
    let value = field
        .text()
        .await
        .map_err(|e| api_err(StatusCode::BAD_REQUEST, format!("invalid form field '{name}': {e}")))?;
    if value.len() > max_len {
        return Err(api_err(
            StatusCode::BAD_REQUEST,
            format!("'{name}' exceeds {max_len} characters"),
        ));
    }
    Ok(value.trim().to_string())
}

fn check_auth(state: &AppState, headers: &HeaderMap) -> ApiResult<()> {
    let provided = headers
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let expected = format!("Bearer {}", state.upload_token);
    if constant_time_eq(provided.as_bytes(), expected.as_bytes()) {
        Ok(())
    } else {
        Err(api_err(
            StatusCode::UNAUTHORIZED,
            "invalid or missing bearer token",
        ))
    }
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

fn validate_slug(value: &str, name: &str) -> ApiResult<()> {
    if value.is_empty()
        || value.len() > 32
        || !value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err(api_err(
            StatusCode::BAD_REQUEST,
            format!("'{name}' must be 1-32 ASCII letters, digits, '-' or '_'"),
        ));
    }
    Ok(())
}

fn validate_version(value: &str) -> ApiResult<()> {
    let valid = !value.is_empty()
        && value.len() <= 64
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_' | '+' | '~'));
    if !valid {
        return Err(api_err(
            StatusCode::BAD_REQUEST,
            "'version' must be 1-64 characters of letters, digits and . _ - + ~",
        ));
    }
    Ok(())
}

fn sanitize_filename(name: &str) -> ApiResult<String> {
    let base = std::path::Path::new(name)
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| api_err(StatusCode::BAD_REQUEST, "invalid filename"))?;
    if base.is_empty()
        || base == ".."
        || base.contains('\\')
        || base.len() > 255
        || base.chars().any(|c| c.is_control())
    {
        return Err(api_err(StatusCode::BAD_REQUEST, "invalid filename"));
    }
    Ok(base.to_string())
}
