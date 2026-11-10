//! External links, resolved from the environment (`.env`) at build time.
//!
//! `build.rs` reads each link with dotenv precedence — an exported shell
//! variable wins, then the `.env` value in the workspace root, then the
//! hardcoded default it also owns — and injects it with `cargo:rustc-env`,
//! which the `env!` calls below read. `build.rs` declares rerun rules for
//! `.env` and the variables, so edits are picked up on the next build.
//!
//! This is compile-time (not `std::env::var`) because the navbar/footer
//! render inside the wasm client, which has no runtime environment; baking
//! the values keeps SSR and hydration consistent. `env!` cannot fail: the
//! build script unconditionally emits all four.
//!
//! These are public site links. Server-side secrets (DATABASE_URL etc.) are
//! a different story — see `src/server/builds/storage.rs`, which reads them
//! at runtime instead.

/// GitHub organization URL (navbar + footer icon buttons).
pub const COMMUNITY_GITHUB_URL: &str = env!("COMMUNITY_GITHUB_URL");

/// Discord invite URL (navbar + footer icon buttons).
pub const DISCORD_URL: &str = env!("DISCORD_URL");

/// GitHub releases page of the app repo — the download buttons' fallback
/// link, shown until a runtime lookup resolves the asset URLs (or on errors).
pub const RELEASES_URL: &str = env!("RELEASES_URL");

/// GitHub releases API of the app repo, queried at runtime to resolve the
/// latest release's per-platform installer zip asset URLs (see
/// src/latest_release.rs).
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub const RELEASES_API_URL: &str = env!("RELEASES_API_URL");
