# Importing plugins

Everything Companion does beyond the core window ships as a plugin. The
feature plugins — chat, speech recognition and synthesis, alarms, calendar,
MCP tools, and the Live2D and VRM avatars — come pre-installed. Anything
else you add by importing a plugin zip in **Settings → Plugins**.

## Where to get plugins

- **The plugin library** on this site — every official plugin's card has a
  download panel listing its published releases. Backend plugins ship one
  zip per platform (`macos-arm64`, `macos-x86_64`, `windows-x86_64`,
  `linux-x86_64`); frontend-only plugins ship a single zip that runs
  everywhere. Pick the zip for your platform — the panel highlights it when
  it can detect what you are on.
- **GitHub releases** — each official plugin lives in its own repository
  under [github.com/companion-lab](https://github.com/companion-lab) (for
  example `companion-plugin-alarms`); the installable zips are attached to
  the repo's releases.
- **Your own build** — `companion-cli build` produces a ready-to-import zip;
  see the Creating Plugins article.

## Import a plugin zip

1. Open **Settings → Plugins**.
2. Click **Import plugin…** and pick the zip.
3. The app validates the manifest, copies the package into the app data
   folder, and activates it. The plugin starts enabled, and its declared
   permissions are shown on its row.

The importer accepts two zip layouts — `manifest.json` at the zip root, or
a single top-level folder named after the plugin containing it. Both are
what `companion-cli build` produces:

```
notes.zip
  manifest.json
  frontend.js
  libnotes_backend.dylib
```

Imported plugins are **copied** into the app data directory, so the
downloaded zip can be deleted afterwards. The original zip is also worth
keeping around — re-importing it is the only way back if you delete the
plugin.

## Updating a plugin

Importing a zip whose plugin id already exists replaces the previous
package in place — to update a plugin, just import its newer zip.

Plugins whose manifest declares an update URL show an **Update** button on
their row when a newer release is available. Clicking it downloads the new
package and re-imports it, with a progress bar (model-heavy plugins reach
hundreds of megabytes, so this can take a while).

Frontend changes apply immediately in the open windows. A replaced Rust
backend is the exception: backends stay loaded while the app runs, so the
new backend takes over after an app restart.

## Managing installed plugins

- **Enable / Disable** — applies immediately in the open windows: disabling
  removes the plugin's settings tabs, widgets, and tools until it is
  re-enabled.
- **Delete** — removes the package from disk. For the bundled feature
  plugins the data is kept and the plugin can be reinstalled with
  **Restore bundled features**; a deleted bundled feature also stays
  uninstalled across restarts. For imported plugins there is no such
  safeguard — deleting one is final.

## When an import fails

A failed import leaves the plugin out of the list in **Settings → Plugins**;
a plugin that imports but cannot load its backend shows the error on its
row. The usual causes:

- The zip's layout matches neither accepted form. Repackage it with
  `companion-cli build` (see the Creating Plugins article).
- The manifest's `abi` doesn't match the host — the zip was built against
  an older SDK. Rebuild it with a current `companion-cli`.
- A backend plugin shows *"load failed: …"* on its row, usually because the
  zip was built for a different platform. Re-download the zip for yours
  from the plugin library or the repo's releases page.

A plugin that imports cleanly but misbehaves afterwards is covered in the
Troubleshooting article.
