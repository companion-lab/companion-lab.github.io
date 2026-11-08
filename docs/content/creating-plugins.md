# Creating a plugin

Everything in Companion beyond the core window shell is a plugin — and you
can write your own. A plugin is a directory containing a `manifest.json`, an
optional bundled React frontend (`frontend.js`), and an optional Rust backend
(`*.dylib` / `*.dll` / `*.so`).

## Package layout

```
my-plugin/
  manifest.json            # metadata + entry points
  frontend.js              # bundled ESM (produced by the build script)
  libmy_backend.dylib      # optional Rust cdylib
  src/
    index.tsx              # your frontend source (React)
  backend/                 # optional Rust crate (crate-type = ["cdylib"])
    Cargo.toml
    src/lib.rs
```

## The manifest

```json
{
  "id": "my-plugin",
  "name": "My Plugin",
  "version": "0.1.0",
  "description": "What it does",
  "abi": 2,
  "permissions": ["storage", "events", "tools"],
  "frontend": { "entry": "frontend.js" },
  "backend": { "library": "libmy_backend.dylib" }
}
```

- `id` — lowercase alphanumeric with `-`/`_`; must not contain `__` (that
  separator qualifies tool names as `{id}__{tool}`).
- `abi` — must match the host ABI (currently `2`). Plugins built against an
  older SDK are refused at load time.
- `permissions` — declarative in v1: `storage`, `events`, `tools`,
  `secrets`. Storage namespacing and tool routing are enforced; the rest is
  informational. The declared list is shown in **Settings → Plugins**.
- `frontend` and `backend` are both optional — a plugin can be
  frontend-only, backend-only, or both.

## Frontend (React)

Plugin frontends are React bundles activated with a scoped `PluginContext`.
They never bundle React — the host injects its own copies at activation time.

```tsx
import { definePlugin } from "@companion-lab/plugin";

export default definePlugin((ctx) => {
  ctx.log.info("activating");

  ctx.settings.registerTab({
    id: "my-tab",
    label: "My Plugin",
    group: "Plugins",
    desc: "Plugin settings",
    component: () => <div>hello {ctx.id}</div>,
  });

  ctx.tools.register({
    name: "ping",
    description: "Ping the plugin",
    inputSchema: { type: "object" },
    execute: async () => "pong",
  });
});
```

### Extension points

| API | What it does |
| --- | --- |
| `settings.registerTab` / `overrideTab` | Add or replace a settings tab |
| `settings.registerSection` / `replaceSection` | Insert into or replace sections of existing tabs |
| `main.registerWidget` / `overrideWidget` | Add or replace widgets in the avatar window — slots `main.backdrop`, `main.top`, `main.bottom`, `main.overlay` |
| `main.registerContextMenuItem` | Entries in the avatar's context menu |
| `tools.register` | A tool the avatar's model can call |
| `systemPrompt.register` | Contribute instruction text to the chat system prompt (live across turns while the plugin is enabled) |
| `backend.call(method, payload)` | Call the plugin's Rust backend |
| `events.on` / `events.emit` | Namespaced event bus |
| `storage.get/set/remove` | Namespaced key/value storage |
| `avatar` | Drive the avatar: emotion, motion, visemes, model loading |
| `services` | Register or consume TTS/STT engines |
| `secrets.set/get/remove` | OS-keychain secrets scoped to the plugin |
| `ui` | The host's UI kit so plugins match the app design |
| `useSettingsState()` | Imported from the SDK — read/write the real settings-page state inside a settings tab or section |

The official feature plugins — alarms, calendar, chat, Live2D avatar, MCP,
STT, TTS, and VRM — are working reference implementations at
https://github.com/companion-lab (e.g. `companion-plugin-alarms`). They ship
with the app, so **Settings → Plugins** doubles as a gallery of real-world
plugin code.

## Backend (Rust, optional)

A backend is a `cdylib` crate depending on `companion-plugin-sdk` (published
on crates.io) and exporting the ABI via `export_plugin!`:

```rust
use companion_plugin_sdk::{export_plugin, Plugin, PluginCtx, ToolDef};
// Cargo.toml: companion-plugin-sdk = "0.1"
use serde_json::{json, Value};

struct MyBackend;

impl Plugin for MyBackend {
    fn init(&mut self, ctx: PluginCtx) -> Result<(), String> {
        // Writable per-plugin dir: {app_data}/plugins-data/{id}/
        let data_dir = ctx.data_dir().ok_or("no data dir")?;
        // Read-only package dir (manifest, bundles, bundled assets/):
        let _package_dir = ctx.plugin_dir();
        std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
        ctx.info("ready");
        Ok(())
    }

    fn handle(&mut self, method: &str, payload: Value) -> Result<Value, String> {
        match method {
            "ping" => Ok(json!({ "pong": true })),
            "tool" => {
                // chat-model tool calls arrive as {"name": "...", "args": {...}}
                let name = payload["name"].as_str().unwrap_or_default();
                match name {
                    "my_tool" => Ok(json!("done")),
                    _ => Err(format!("unknown tool: {name}")),
                }
            }
            _ => Err(format!("unknown method: {method}")),
        }
    }

    fn tools(&mut self) -> Vec<ToolDef> {
        vec![ToolDef {
            name: "my_tool".into(),
            description: "Does the thing".into(),
            input_schema: json!({ "type": "object" }),
        }]
    }
}

export_plugin!(MyBackend::new());
```

A few ABI-safety rules worth knowing:

- The host checks the ABI version before calling anything.
- Every call into the dylib is wrapped in `catch_unwind` — a panicking
  backend degrades to "every call errors" instead of crashing the app.
- `tools()` is queried live on every tool-list refresh, so dynamic tool sets
  (e.g. an MCP plugin whose servers connect over time) stay current. Keep it
  fast: read a cache, never block on I/O.
- Backends are never unloaded while the app runs; disabling a plugin only
  stops routing calls to it. Fully unloading needs an app restart.

## Build and install

### Package with the `companion-cli` CLI

The `companion-cli` CLI bundles your frontend, builds your backend, and
zips everything into a package ready for the app's import flow. The zip
contains only package files — `manifest.json`, the bundled frontend entry,
and the backend library. Sources, `node_modules/`, and cargo `target/` never
ship.

Install the CLI globally from npm:

```bash
npm install -g companion-cli
```

Or use it without installing via `npx companion-cli <command>`.

Then scaffold (optional) and build your plugin:

```bash
companion-cli init my-plugin    # optional scaffold (frontend + backend)
companion-cli build my-plugin
# -> my-plugin/dist/my-plugin-0.1.0.zip
```

Inside your plugin project, install dependencies first (the frontend SDK
comes from npm as `@companion-lab/plugin`):

```bash
npm install
```

Running from a companion-tauri checkout, the CLI also works straight from the
repository without any install:

```bash
node plugin-cli/bin/companion-cli.mjs build my-plugin
```

esbuild is resolved from the CLI's own dependencies (or `app/node_modules` as
a fallback), so the CLI works even without the install step. The frontend is
bundled with esbuild (React is aliased to the host's copies via the SDK
shims — never bundled). The Rust backend is built with `cargo build
--release` whenever `backend/` and `manifest.backend` both exist; use
`--no-backend` to skip it.

### CLI options

| Option | Meaning |
| --- | --- |
| `--out <dir>` | Directory for the zip (default `./dist`) |
| `--name <file>` | Zip filename (default `<id>-<version>.zip`) |
| `--flat` | Put `manifest.json` at the zip root instead of inside an `<id>/` folder — the importer accepts both layouts |
| `--no-backend` | Skip the Rust build even if `backend/` exists |
| `--no-frontend` | Skip the JS bundle (backend-only plugin) |
| `--sdk <dir>` | Path to the plugin JS SDK (defaults to the shims bundled with the CLI; inside a companion-tauri checkout, the repo's own `sdk/js/@companion/plugin`) |

Zip entries use a fixed timestamp, so identical inputs produce
byte-identical archives.

### Import into the app

Open **Settings → Plugins → Import plugin…** and pick the zip. Both layouts
are accepted — one top-level folder named after the plugin id:

```
notes/
  manifest.json
  frontend.js
  libnotes_backend.dylib
```

or the flat layout from `--flat` (`manifest.json` at the zip root).

