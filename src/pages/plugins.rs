use dioxus::prelude::*;

use crate::{
    components::{AmbientBackground, Footer},
    config::GITHUB_URL,
    plugin_catalog::{CATALOG_PERMISSIONS, PLUGIN_CATALOG},
    plugin_releases::{detect_platform, fetch_releases, PluginPlatform, PluginRelease, PluginZip},
    route::Route,
};

/// One plugin in the catalog. The data comes from the community plugin
/// registry (companion-lab/plugin-registry) via the generated
/// `src/plugin_catalog.rs`; this struct is just its Rust shape.
/// `frontend`/`backend` mirror whether the plugin manifest declares the
/// matching entry point.
#[derive(Clone, PartialEq)]
pub struct PluginEntry {
    pub id: &'static str,
    /// Full GitHub slug (`owner/name`) hosting the plugin's source and
    /// releases — the card links there. Any public repo can register, not
    /// just companion-lab's, so the owner is part of the value.
    pub repo: &'static str,
    pub name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
    /// Theme color key: "purple", "blue", "indigo", or "pink".
    pub color: &'static str,
    /// The manifest's declared permission list.
    pub permissions: &'static [&'static str],
    pub frontend: bool,
    pub backend: bool,
}

/// `/plugins` — searchable catalog of the official plugins.
#[component]
pub fn Plugins() -> Element {
    let mut query = use_signal(String::new);
    let mut permission = use_signal(|| Option::<&'static str>::None);

    // Recomputed (and subscribed) whenever the query or the permission
    // filter changes; holds refs into the static catalog, so cloning it is
    // copying pointers, not descriptions.
    let filtered = use_memo(move || {
        let q = query().to_lowercase();
        let selected = permission();
        PLUGIN_CATALOG
            .iter()
            .filter(|p| {
                let matches_query = q.is_empty()
                    || p.name.to_lowercase().contains(&q)
                    || p.id.to_lowercase().contains(&q)
                    || p.description.to_lowercase().contains(&q)
                    || p.permissions.iter().any(|perm| perm.contains(&q));
                let matches_permission = match selected {
                    Some(sel) => p.permissions.contains(&sel),
                    None => true,
                };
                matches_query && matches_permission
            })
            .collect::<Vec<_>>()
    });

    let filters_active = !query().is_empty() || permission().is_some();

    // The data-grid draws its hairlines with a `gap-px` seam over a white/10
    // wrapper, so an unfilled slot in the final row shows as a lighter block.
    // Complete the row with card-colored filler cells — the column count is
    // responsive (1 base, 2 at sm, 3 at lg), so each filler's visibility
    // tracks the breakpoint it patches.
    let fillers = {
        let n = filtered().len();
        let need_sm = (2 - n % 2) % 2;
        let need_lg = (3 - n % 3) % 3;
        let first = match (need_sm >= 1, need_lg >= 1) {
            (true, true) => "hidden sm:block lg:block",
            (true, false) => "hidden sm:block lg:hidden",
            (false, true) => "hidden sm:hidden lg:block",
            (false, false) => "hidden",
        };
        let second = if need_lg >= 2 {
            "hidden sm:hidden lg:block"
        } else {
            "hidden"
        };
        vec![first, second]
    };

    rsx! {
        div {
            class: "relative min-h-screen pb-24 pt-24 sm:pt-32",
            AmbientBackground {}
            div {
                class: "relative z-10 mx-auto max-w-7xl px-6 pb-16",

                // ── Header ────────────────────────────────────────────
                div {
                    class: "mb-10 sm:mb-14 reveal active",
                    div {
                        class: "flex items-center justify-between gap-4 border-b border-white/10 pb-3 font-mono text-[11px] uppercase tracking-[0.22em] text-purple-300/90",
                        span { "mod // plugin registry" }
                        span { class: "text-gray-500", "{PLUGIN_CATALOG.len()} modules loaded" }
                    }
                    h1 {
                        class: "mt-8 text-4xl md:text-6xl font-black uppercase tracking-tight text-white",
                        "Plugin "
                        span { class: "text-purple-500", "library" }
                    }
                    p {
                        class: "mt-4 max-w-2xl text-gray-400 sm:text-lg",
                        "Everything Companion does beyond the core window ships as a plugin. Search the official catalog by name, capability, or permission."
                    }
                }

                // ── Search + permission filter ────────────────────────
                div {
                    class: "mb-8 flex flex-col gap-4 reveal active",
                    div {
                        class: "relative",
                        i {
                            class: "fa-solid fa-magnifying-glass pointer-events-none absolute left-4 top-1/2 -translate-y-1/2 text-gray-500",
                            "aria-hidden": "true"
                        }
                        input {
                            class: "w-full rounded-xl glass-panel py-3.5 pl-11 pr-4 text-sm text-white placeholder-gray-500 outline-none transition-colors focus:border-purple-500/50",
                            r#type: "text",
                            placeholder: "Search plugins — try \u{201C}tts\u{201D}, \u{201C}avatar\u{201D}, or \u{201C}secrets\u{201D}",
                            value: query(),
                            oninput: move |e| query.set(e.value()),
                            "aria-label": "Search plugins",
                        }
                    }
                    div {
                        class: "flex flex-wrap items-center gap-2",
                        span {
                            class: "mr-1 font-mono text-[11px] uppercase tracking-[0.22em] text-gray-500",
                            "permission:"
                        }
                        for perm in CATALOG_PERMISSIONS {
                            button {
                                key: "{perm}",
                                class: if permission() == Some(perm) {
                                    "rounded-full border border-purple-500/60 bg-purple-600/25 px-3 py-1 font-mono text-xs text-purple-200"
                                } else {
                                    "rounded-full border border-white/10 px-3 py-1 font-mono text-xs text-gray-400 transition-colors hover:border-purple-500/40 hover:text-purple-300"
                                },
                                onclick: move |_| {
                                    if permission() == Some(perm) {
                                        permission.set(None);
                                    } else {
                                        permission.set(Some(perm));
                                    }
                                },
                                "{perm}"
                            }
                        }
                        if filters_active {
                            button {
                                class: "ml-1 font-mono text-xs text-gray-500 underline-offset-4 transition-colors hover:text-white hover:underline",
                                onclick: move |_| {
                                    query.set(String::new());
                                    permission.set(None);
                                },
                                "clear all"
                            }
                        }
                    }
                }

                // ── Result grid ───────────────────────────────────────
                p {
                    class: "mb-4 font-mono text-[11px] uppercase tracking-[0.22em] text-gray-500 reveal active",
                    "showing {filtered().len()} of {PLUGIN_CATALOG.len()} plugins"
                }
                if filtered().is_empty() {
                    div {
                        class: "border border-dashed border-white/15 px-6 py-16 text-center reveal active",
                        i {
                            class: "fa-solid fa-magnifying-glass mb-4 text-2xl text-gray-600",
                            "aria-hidden": "true"
                        }
                        if query().is_empty() {
                            p { class: "mb-1 font-semibold text-white", "No plugins with this permission" }
                        } else {
                            p { class: "mb-1 font-semibold text-white", "No plugins match \u{201C}{query()}\u{201D}" }
                        }
                        p {
                            class: "mb-6 text-sm text-gray-400",
                            "Try a different name, capability, or permission."
                        }
                        button {
                            class: "rounded-xl border border-purple-500/40 bg-purple-600/15 px-5 py-2.5 text-sm font-medium text-purple-200 transition-colors hover:bg-purple-600/30",
                            onclick: move |_| {
                                query.set(String::new());
                                permission.set(None);
                            },
                            "Clear search"
                        }
                    }
                } else {
                    // Hairline data-grid: cells share 1px seams drawn by the
                    // gap over the wrapper background (same trick as Features).
                    div {
                        class: "grid grid-cols-1 gap-px border border-white/10 bg-white/10 sm:grid-cols-2 lg:grid-cols-3",
                        for (i, plugin) in filtered().iter().enumerate() {
                            PluginCard {
                                key: "{plugin.id}",
                                plugin: *plugin,
                                index: format!("{:02}", i + 1),
                            }
                        }
                        for (i, filler_class) in fillers.iter().enumerate() {
                            div {
                                key: "filler-{i}",
                                class: "bg-[#08040f]/85 {filler_class}",
                            }
                        }
                    }
                }

                // ── Build-your-own CTA ────────────────────────────────
                div {
                    class: "mt-14 flex flex-col items-start justify-between gap-4 rounded-2xl glass-panel p-6 sm:flex-row sm:items-center sm:p-8 reveal active",
                    div {
                        h2 { class: "text-lg font-semibold text-white", "Building your own?" }
                        p {
                            class: "mt-1 text-sm text-gray-400",
                            "Every plugin is a manifest plus an optional React frontend and Rust backend — the CLI scaffolds one for you."
                        }
                    }
                    Link {
                        to: Route::DocsArticle { slug: "creating-plugins".to_string() },
                        class: "flex items-center gap-2 rounded-xl border border-purple-500/40 bg-purple-600/15 px-5 py-2.5 text-sm font-medium text-purple-200 transition-colors hover:bg-purple-600/30",
                        "Creating plugins"
                        i { class: "fa-solid fa-arrow-right text-xs", "aria-hidden": "true" }
                    }
                }
            }
        }
        // Footer sits outside the padded min-h-screen wrapper — a bottom
        // padding on this wrapper would render as an empty strip below it.
        Footer {}
    }
}

/// Fetch state of a card's download panel. Kept per card — not reset when
/// the panel closes — so toggling the panel never re-requests; a failed
/// lookup (rate limit, offline) stays until reload and keeps its fallback
/// link to the GitHub releases page.
#[derive(Clone, PartialEq)]
enum ReleasesState {
    Loading,
    Loaded(Vec<PluginRelease>),
    Failed(String),
}

/// One cell of the plugin data-grid: square hairline icon badge, version tag,
/// declared permissions as mono chips, and the shipped entry points. Indexes
/// like a register (`p.01`), matching the feature cells' `f.01`. The header
/// links to the plugin's repository (opened in a new tab); the download
/// button opens a panel listing every published release's installable zips
/// (see [DownloadPanel]).
#[component]
fn PluginCard(plugin: &'static PluginEntry, index: String) -> Element {
    let mut panel_open = use_signal(|| false);
    // `None` until the first open kicks off the (one-time) release lookup.
    let mut releases = use_signal(|| Option::<ReleasesState>::None);
    let detected = detect_platform();

    let icon_color = match plugin.color {
        "blue" => "text-blue-400",
        "indigo" => "text-indigo-400",
        "pink" => "text-pink-400",
        _ => "text-purple-400",
    };
    let kind = match (plugin.frontend, plugin.backend) {
        (true, true) => "frontend + rust backend",
        (true, false) => "frontend only",
        (false, true) => "rust backend only",
        (false, false) => "metadata only",
    };

    let toggle = move |_| {
        let opening = !panel_open();
        panel_open.set(opening);
        if opening && releases().is_none() {
            releases.set(Some(ReleasesState::Loading));
            let mut releases = releases;
            spawn(async move {
                let state = match fetch_releases(plugin).await {
                    Ok(list) => ReleasesState::Loaded(list),
                    Err(error) => ReleasesState::Failed(error),
                };
                // Only land the result in a still-loading panel: a later
                // open/close cycle may already have superseded this request
                // (the panel keeps its state across toggles, so there is
                // nothing to refresh).
                if releases() == Some(ReleasesState::Loading) {
                    releases.set(Some(state));
                }
            });
        }
    };

    rsx! {
        div {
            class: "group relative bg-[#08040f]/85 p-6 transition-colors hover:bg-[#140a26]",
            span {
                class: "absolute top-3 right-4 font-mono text-[10px] tracking-[0.2em] text-gray-600 transition-colors group-hover:text-purple-400/70",
                "p.{index}"
            }
            a {
                href: "{GITHUB_URL}/{plugin.repo}",
                target: "_blank",
                rel: "noopener noreferrer",
                "aria-label": "View the {plugin.name} plugin repository on GitHub",
                class: "mb-5 flex items-center gap-4",
                div {
                    class: "flex h-10 w-10 shrink-0 items-center justify-center border border-white/10 bg-white/[0.03] transition-colors group-hover:border-purple-500/40",
                    i { class: "fa-solid {plugin.icon} text-lg {icon_color}", "aria-hidden": "true" }
                }
                div {
                    h3 { class: "text-base font-semibold text-white", "{plugin.name}" }
                    span {
                        class: "font-mono text-[10px] tracking-[0.2em] text-gray-500",
                        "v{plugin.version}"
                    }
                }
            }
            p { class: "mb-4 text-sm leading-relaxed text-gray-400", "{plugin.description}" }
            div {
                class: "mb-4 flex flex-wrap gap-1.5",
                for perm in plugin.permissions {
                    span {
                        key: "{perm}",
                        class: "rounded-full border border-white/10 px-2 py-0.5 font-mono text-[10px] text-gray-400",
                        "{perm}"
                    }
                }
            }
            div {
                class: "flex items-center justify-between gap-3",
                span { class: "font-mono text-[10px] uppercase tracking-[0.2em] text-gray-600", "{kind}" }
                div {
                    class: "flex items-center gap-3",
                    a {
                        href: "{GITHUB_URL}/{plugin.repo}",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "aria-label": "View the {plugin.name} plugin repository on GitHub",
                        class: "flex items-center gap-1.5 font-mono text-[10px] tracking-[0.2em] text-gray-600 transition-colors hover:text-purple-300",
                        i { class: "fa-brands fa-github text-xs", "aria-hidden": "true" }
                        "source"
                    }
                    button {
                        "aria-expanded": "{panel_open()}",
                        "aria-controls": "download-{plugin.id}",
                        onclick: toggle,
                        class: "flex items-center gap-1.5 rounded-lg border border-purple-500/40 bg-purple-600/15 px-3 py-1.5 font-mono text-[10px] uppercase tracking-[0.2em] text-purple-200 transition-colors hover:bg-purple-600/30",
                        i {
                            class: if panel_open() { "fa-solid fa-chevron-up text-xs" } else { "fa-solid fa-download text-xs" },
                            "aria-hidden": "true"
                        }
                        if panel_open() { "close" } else { "download" }
                    }
                }
            }
            if panel_open() {
                DownloadPanel {
                    key: "panel-{plugin.id}",
                    plugin,
                    state: releases().unwrap_or(ReleasesState::Loading),
                    detected,
                }
            }
        }
    }
}

/// The version chooser below a card's footer: every published release of the
/// plugin repo (newest first), each with its installable zips — one per
/// platform for backend plugins, a single universal zip for frontend-only
/// ones. The visitor's platform (when detected) sorts and highlights first.
#[component]
fn DownloadPanel(
    plugin: &'static PluginEntry,
    state: ReleasesState,
    detected: Option<PluginPlatform>,
) -> Element {
    let latest_idx = match &state {
        ReleasesState::Loaded(list) => list
            .iter()
            .position(|release| !release.prerelease)
            .unwrap_or(0),
        _ => 0,
    };

    rsx! {
        div {
            id: "download-{plugin.id}",
            class: "mt-4 border-t border-white/10 pt-4",
            {match &state {
                ReleasesState::Loading => rsx! {
                    p {
                        class: "flex items-center gap-2 font-mono text-[11px] uppercase tracking-[0.2em] text-gray-500",
                        i { class: "fa-solid fa-circle-notch fa-spin", "aria-hidden": "true" }
                        "loading releases…"
                    }
                },
                ReleasesState::Loaded(list) => rsx! {
                    if list.is_empty() {
                        p { class: "mb-2 text-xs text-gray-400", "No published releases yet." }
                        ReleasesFallbackLink { plugin }
                    } else {
                        for (idx, release) in list.iter().enumerate() {
                            div {
                                key: "{release.tag}",
                                class: "mb-3 last:mb-0",
                                div {
                                    class: "flex flex-wrap items-center gap-x-3 gap-y-1",
                                    span { class: "font-mono text-xs font-semibold text-white", "{release.tag}" }
                                    if idx == latest_idx {
                                        span {
                                            class: "rounded-full border border-purple-500/40 bg-purple-600/20 px-2 py-0.5 font-mono text-[9px] uppercase tracking-[0.2em] text-purple-200",
                                            "latest"
                                        }
                                    }
                                    if release.prerelease {
                                        span {
                                            class: "rounded-full border border-white/15 px-2 py-0.5 font-mono text-[9px] uppercase tracking-[0.2em] text-gray-400",
                                            "pre-release"
                                        }
                                    }
                                }
                                div {
                                    class: "mt-1.5 flex flex-wrap gap-x-4 gap-y-1.5",
                                    for zip in ordered_zips(release, detected) {
                                        a {
                                            key: "{zip.url}",
                                            href: "{zip.url}",
                                            class: "flex items-center gap-1.5 font-mono text-[11px] {zip_class(&zip, detected)}",
                                            i { class: "{zip_icon(&zip)} text-[10px]", "aria-hidden": "true" }
                                            "{zip_label(&zip)}"
                                        }
                                    }
                                }
                            }
                        }
                        // The lookup may be the CDN fallback (whose zip URLs
                        // are derived and can dead-end on repos whose tags
                        // were pushed without the release job) or simply be
                        // missing older versions — keep the source of truth
                        // one click away.
                        div {
                            class: "mt-3",
                            ReleasesFallbackLink { plugin }
                        }
                    }
                },
                ReleasesState::Failed(error) => rsx! {
                    p { class: "mb-2 text-xs text-red-300/90", "Couldn't load the release list ({error})." }
                    ReleasesFallbackLink { plugin }
                },
            }}
            p {
                class: "mt-3 border-t border-white/5 pt-3 font-mono text-[10px] leading-relaxed tracking-wide text-gray-500",
                "Import into Companion: Settings → Plugins → Import plugin… — pick the zip for your platform. "
                Link {
                    to: Route::DocsArticle { slug: "importing-plugins".to_string() },
                    class: "text-purple-400 transition-colors hover:text-purple-300",
                    "More on importing plugins"
                }
            }
        }
    }
}

/// The repo's releases page on GitHub — the panel's escape hatch when a
/// lookup cannot serve zips (rate-limited mirror, no releases yet), and the
/// way to older versions and source archives in any state.
#[component]
fn ReleasesFallbackLink(plugin: &'static PluginEntry) -> Element {
    rsx! {
        a {
            href: "{GITHUB_URL}/{plugin.repo}/releases",
            target: "_blank",
            rel: "noopener noreferrer",
            class: "flex items-center gap-1.5 font-mono text-[11px] text-gray-400 transition-colors hover:text-purple-300",
            i { class: "fa-brands fa-github text-xs", "aria-hidden": "true" }
            "Browse releases on GitHub"
        }
    }
}

/// The release's zips, the detected platform's first (the rest keep their
/// filename order).
fn ordered_zips(release: &PluginRelease, detected: Option<PluginPlatform>) -> Vec<PluginZip> {
    let mut zips = release.zips.clone();
    zips.sort_by_key(|zip| if zip.platform == detected { 0 } else { 1 });
    zips
}

/// Download-link styling: accent for the detected platform and for universal
/// (frontend-only) zips, quiet gray otherwise.
fn zip_class(zip: &PluginZip, detected: Option<PluginPlatform>) -> &'static str {
    if zip.platform == detected || zip.platform.is_none() {
        "text-purple-300 transition-colors hover:text-purple-200"
    } else {
        "text-gray-400 transition-colors hover:text-purple-300"
    }
}

fn zip_icon(zip: &PluginZip) -> &'static str {
    zip.platform
        .map(|platform| platform.icon())
        .unwrap_or("fa-solid fa-file-zipper")
}

fn zip_label(zip: &PluginZip) -> String {
    match zip.platform {
        Some(platform) => platform.label().to_string(),
        None => "All platforms".to_string(),
    }
}
