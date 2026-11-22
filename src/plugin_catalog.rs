//! The `/plugins` page's catalog, generated from the community plugin
//! registry (companion-lab/plugin-registry's plugins.json + versions.json)
//! by scripts/sync-registry.mjs.
//!
//! DO NOT EDIT — edit the registry instead; the next Pages build embeds
//! the fresh catalog (deploy-pages.yml runs the sync before building).
//! `version` is the repo's latest published release resolved by the
//! registry; "" means none yet (the card shows "unreleased").

use crate::pages::PluginEntry;

pub static PLUGIN_CATALOG: [PluginEntry; 8] = [
    PluginEntry {
        id: "chat",
        repo: "companion-lab/companion-plugin-chat",
        name: "Chat",
        version: "0.3.2",
        description: "The avatar's AI conversation: provider chat with tool loop, streaming TTS/STT speech, wake word, clarifying questions, and the Persona/Connection settings sections.",
        icon: "fa-comments",
        color: "purple",
        permissions: &["storage", "events", "tools", "secrets"],
        frontend: true,
        backend: false,
    },
    PluginEntry {
        id: "vrm",
        repo: "companion-lab/companion-plugin-vrm",
        name: "VRM Avatar",
        version: "0.3.2",
        description: "VRM avatar models (with the built-in demo asset), motion and emotion clips, and graphics settings.",
        icon: "fa-cube",
        color: "blue",
        permissions: &["storage", "events"],
        frontend: true,
        backend: true,
    },
    PluginEntry {
        id: "companion-plugin-l2d",
        repo: "companion-lab/companion-plugin-l2d",
        name: "Live2D Avatar",
        version: "0.3.2",
        description: "Live2D avatar for the desktop overlay: renders the bundled Cubism model and answers the avatar bus (chat emotions, TTS lip sync, click-through).",
        icon: "fa-masks-theater",
        color: "indigo",
        permissions: &["storage", "events", "tools"],
        frontend: true,
        backend: true,
    },
    PluginEntry {
        id: "stt",
        repo: "companion-lab/companion-plugin-stt",
        name: "Speech Recognition",
        version: "0.3.2",
        description: "On-device STT (Moonshine) + cloud transcription, with a Speech Recognition settings tab and the cloud STT endpoint card.",
        icon: "fa-microphone-lines",
        color: "pink",
        permissions: &["events", "secrets", "microphone"],
        frontend: true,
        backend: true,
    },
    PluginEntry {
        id: "tts",
        repo: "companion-lab/companion-plugin-tts",
        name: "Speech Synthesis",
        version: "0.3.3",
        description: "On-device TTS (Supertonic) + cloud synthesis, with a Speech Synthesis settings tab and the cloud TTS endpoint card.",
        icon: "fa-volume-high",
        color: "purple",
        permissions: &["events", "secrets"],
        frontend: true,
        backend: true,
    },
    PluginEntry {
        id: "alarms",
        repo: "companion-lab/companion-plugin-alarms",
        name: "Alarms",
        version: "0.3.2",
        description: "Timed alarms and reminders: a settings tab, the ringing banner, and alarm tools the avatar can call.",
        icon: "fa-bell",
        color: "blue",
        permissions: &["storage", "events", "tools"],
        frontend: true,
        backend: true,
    },
    PluginEntry {
        id: "calendar",
        repo: "companion-lab/companion-plugin-calendar",
        name: "Calendar",
        version: "0.3.2",
        description: "Dated events, birthdays, and appointments: a settings tab, reminder banners, and calendar tools the avatar can call.",
        icon: "fa-calendar-days",
        color: "indigo",
        permissions: &["storage", "events", "tools"],
        frontend: true,
        backend: true,
    },
    PluginEntry {
        id: "mcp",
        repo: "companion-lab/companion-plugin-mcp",
        name: "MCP Tools",
        version: "0.3.2",
        description: "Model Context Protocol client: connect MCP servers and expose their tools to the avatar.",
        icon: "fa-plug",
        color: "pink",
        permissions: &["storage", "events", "tools"],
        frontend: true,
        backend: true,
    },
];

pub static CATALOG_PERMISSIONS: [&str; 5] = ["storage", "events", "tools", "secrets", "microphone"];