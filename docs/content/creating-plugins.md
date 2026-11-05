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
- `permissions` — declarative in v1: `storage`, `events`, `tools`. Storage
  namespacing and tool routing are enforced; the rest is informational.
- `frontend` and `backend` are both optional — a plugin can be
  frontend-only, backend-only, or both.

## Frontend (React)

Plugin frontends are React bundles activated with a scoped `PluginContext`.
They never bundle React — the host injects its own copies at activation time.

```tsx
import { definePlugin } from "@companion/plugin";

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
| `main.registerWidget` / `overrideWidget` | Add or replace widgets in the avatar window |
| `main.registerContextMenuItem` | Entries in the avatar's context menu |
| `tools.register` | A tool the avatar's model can call |
| `backend.call(method, payload)` | Call the plugin's Rust backend |
| `events.on` / `events.emit` | Namespaced event bus |
| `storage.get/set/remove` | Namespaced key/value storage |
| `avatar` | Drive the avatar: emotion, motion, visemes, model loading |
| `services` | Register or consume TTS/STT engines |
| `secrets.set/get/remove` | OS-keychain secrets scoped to the plugin |
| `ui` | The host's UI kit so plugins match the app design |

The reference implementation covering every extension point is the
[`notes` example plugin](https://github.com/companion-vr) in the repository.

## Backend (Rust, optional)

A backend is a `cdylib` crate depending on `companion-plugin-sdk` and
exporting the ABI via `export_plugin!`:

```rust
use companion_plugin_sdk::{export_plugin, Plugin, PluginCtx, ToolDef};
use serde_json::{json, Value};

struct MyBackend;

impl Plugin for MyBackend {
    fn init(&mut self, ctx: PluginCtx) -> Result<(), String> {
        // Writable per-plugin dir: {app_data}/plugins-data/{id}/
        let data_dir = ctx.data_dir().ok_or("no data dir")?;
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
- Backends are never unloaded while the app runs; disabling a plugin only
  stops routing calls to it. Fully unloading needs an app restart.

## Build and install

Build the package with the companion CLI and zip it for distribution:

```bash
node plugin-cli/bin/companion-plugin.mjs build examples/notes-plugin
# -> dist/notes-0.1.0.zip
```

Then import it via **Settings → Plugins → Import plugin…**. For development,
drop the built package into the repo's `plugins/` staging directory — debug
builds discover it automatically.
