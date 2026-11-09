use crate::components::Icon;
use dioxus::prelude::*;

use crate::{
    components::{AmbientBackground, Footer},
    config::COMMUNITY_GITHUB_URL,
    route::Route,
};

/// One plugin in the catalog, transcribed from its `manifest.json` (the
/// `plugin-repos/*` checkouts). `frontend`/`backend` mirror whether the
/// manifest declares the matching entry point.
#[derive(Clone, PartialEq)]
pub struct PluginEntry {
    pub id: &'static str,
    /// GitHub repo name under the community org — the card links there.
    /// Not derivable from `id` (see `companion-plugin-l2d`), so it is stored.
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

/// The official plugin catalog, embedded at build time because the Pages
/// deployment is a static client-only bundle — there is no server to fetch
/// a registry from (same reason the docs markdown uses `include_str!`).
pub static PLUGIN_CATALOG: [PluginEntry; 8] = [
    PluginEntry {
        id: "chat",
        repo: "companion-plugin-chat",
        name: "Chat",
        version: "0.1.0",
        description: "The avatar's AI conversation: provider chat with tool loop, streaming TTS/STT speech, wake word, clarifying questions, and the Persona/Connection settings sections.",
        icon: "comments",
        color: "purple",
        permissions: &["storage", "events", "tools", "secrets"],
        frontend: true,
        backend: false,
    },
    PluginEntry {
        id: "vrm",
        repo: "companion-plugin-vrm",
        name: "VRM Avatar",
        version: "0.1.0",
        description: "VRM avatar models (with the built-in demo asset), motion and emotion clips, and graphics settings.",
        icon: "cube",
        color: "blue",
        permissions: &["storage", "events"],
        frontend: true,
        backend: true,
    },
    PluginEntry {
        id: "companion-plugin-l2d",
        repo: "companion-plugin-l2d",
        name: "Live2D Avatar",
        version: "0.1.0",
        description: "Live2D avatar for the desktop overlay: renders the bundled Cubism model and answers the avatar bus (chat emotions, TTS lip sync, click-through).",
        icon: "masks-theater",
        color: "indigo",
        permissions: &["storage", "events", "tools"],
        frontend: true,
        backend: true,
    },
    PluginEntry {
        id: "stt",
        repo: "companion-plugin-stt",
        name: "Speech Recognition",
        version: "0.1.1",
        description: "On-device STT (Moonshine) + cloud transcription, with a Speech Recognition settings tab and the cloud STT endpoint card.",
        icon: "microphone-lines",
        color: "pink",
        permissions: &["events", "secrets", "microphone"],
        frontend: true,
        backend: true,
    },
    PluginEntry {
        id: "tts",
        repo: "companion-plugin-tts",
        name: "Speech Synthesis",
        version: "0.1.0",
        description: "On-device TTS (Supertonic) + cloud synthesis, with a Speech Synthesis settings tab and the cloud TTS endpoint card.",
        icon: "volume-high",
        color: "purple",
        permissions: &["events", "secrets"],
        frontend: true,
        backend: true,
    },
    PluginEntry {
        id: "alarms",
        repo: "companion-plugin-alarms",
        name: "Alarms",
        version: "0.1.0",
        description: "Timed alarms and reminders: a settings tab, the ringing banner, and alarm tools the avatar can call.",
        icon: "bell",
        color: "blue",
        permissions: &["storage", "events", "tools"],
        frontend: true,
        backend: true,
    },
    PluginEntry {
        id: "calendar",
        repo: "companion-plugin-calendar",
        name: "Calendar",
        version: "0.1.0",
        description: "Dated events, birthdays, and appointments: a settings tab, reminder banners, and calendar tools the avatar can call.",
        icon: "calendar-days",
        color: "indigo",
        permissions: &["storage", "events", "tools"],
        frontend: true,
        backend: true,
    },
    PluginEntry {
        id: "mcp",
        repo: "companion-plugin-mcp",
        name: "MCP Tools",
        version: "0.1.0",
        description: "Model Context Protocol client: connect MCP servers and expose their tools to the avatar.",
        icon: "plug",
        color: "pink",
        permissions: &["storage", "events", "tools"],
        frontend: true,
        backend: true,
    },
];

/// Every permission declared across the catalog, in chip order. Kept in sync
/// with `PLUGIN_CATALOG` — a new permission needs a chip here to be filterable.
static CATALOG_PERMISSIONS: [&str; 5] = ["storage", "events", "tools", "secrets", "microphone"];

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
        let second = if need_lg >= 2 { "hidden sm:hidden lg:block" } else { "hidden" };
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
                        span { class: "text-gray-400", "{PLUGIN_CATALOG.len()} modules loaded" }
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
                        Icon {
                            class: "pointer-events-none absolute left-4 top-1/2 -translate-y-1/2 text-gray-400",
                            icon: "magnifying-glass",
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
                            class: "mr-1 font-mono text-[11px] uppercase tracking-[0.22em] text-gray-400",
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
                                class: "ml-1 font-mono text-xs text-gray-400 underline-offset-4 transition-colors hover:text-white hover:underline",
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
                    class: "mb-4 font-mono text-[11px] uppercase tracking-[0.22em] text-gray-400 reveal active",
                    "showing {filtered().len()} of {PLUGIN_CATALOG.len()} plugins"
                }
                if filtered().is_empty() {
                    div {
                        class: "border border-dashed border-white/15 px-6 py-16 text-center reveal active",
                        Icon {
                            class: "mb-4 text-2xl text-gray-400",
                            icon: "magnifying-glass",
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
                        Icon { icon: "arrow-right", class: "text-xs" }
                    }
                }
            }
        }
        // Footer sits outside the padded min-h-screen wrapper — a bottom
        // padding on this wrapper would render as an empty strip below it.
        Footer {}
    }
}

/// One cell of the plugin data-grid: square hairline icon badge, version tag,
/// declared permissions as mono chips, and the shipped entry points. Indexes
/// like a register (`p.01`), matching the feature cells' `f.01`. The whole
/// cell is a link to the plugin's repository, opened in a new tab.
#[component]
fn PluginCard(plugin: &'static PluginEntry, index: String) -> Element {
    let icon_class = match plugin.color {
        "blue" => "text-lg text-blue-400",
        "indigo" => "text-lg text-indigo-400",
        "pink" => "text-lg text-pink-400",
        _ => "text-lg text-purple-400",
    };
    let kind = match (plugin.frontend, plugin.backend) {
        (true, true) => "frontend + rust backend",
        (true, false) => "frontend only",
        (false, true) => "rust backend only",
        (false, false) => "metadata only",
    };

    rsx! {
        a {
            href: "{COMMUNITY_GITHUB_URL}/{plugin.repo}",
            target: "_blank",
            rel: "noopener noreferrer",
            "aria-label": "View the {plugin.name} plugin repository on GitHub",
            class: "group relative block bg-[#08040f]/85 p-6 transition-colors hover:bg-[#140a26]",
            span {
                class: "absolute top-3 right-4 font-mono text-[10px] tracking-[0.2em] text-gray-400 transition-colors group-hover:text-purple-400/70",
                "p.{index}"
            }
            div {
                class: "mb-5 flex items-center gap-4",
                div {
                    class: "flex h-10 w-10 shrink-0 items-center justify-center border border-white/10 bg-white/[0.03] transition-colors group-hover:border-purple-500/40",
                    Icon { icon: plugin.icon, class: icon_class }
                }
                div {
                    h3 { class: "text-base font-semibold text-white", "{plugin.name}" }
                    span {
                        class: "font-mono text-[10px] tracking-[0.2em] text-gray-400",
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
                class: "flex items-center justify-between gap-2",
                span { class: "font-mono text-[10px] uppercase tracking-[0.2em] text-gray-400", "{kind}" }
                span {
                    class: "flex items-center gap-1.5 font-mono text-[10px] tracking-[0.2em] text-gray-400 transition-colors group-hover:text-purple-300",
                    Icon { icon: "github", class: "text-xs" }
                    "source"
                }
            }
        }
    }
}
