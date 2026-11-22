#!/usr/bin/env node
// Regenerate src/plugin_catalog.rs from the community plugin registry
// (companion-lab/plugin-registry): plugins.json merged with the generated
// versions.json.
//
//   node scripts/sync-registry.mjs [--dry-run] [--check] [--registry <base-url-or-dir>]
//
// The registry repo is the source of truth: contributors edit plugins.json
// there (entries carry no version — the registry's sync-versions.mjs
// resolves each repo's latest GitHub release into versions.json), its
// publish workflow dispatches this repo after every merge, and deploy-pages
// runs this script before building so the static wasm ships the fresh
// catalog (the wasm client cannot fetch one at runtime — same reason the
// docs markdown uses include_str!). The committed plugin_catalog.rs also
// keeps local builds hermetic: no network needed unless you re-sync.
//
// Source resolution, in order:
//   --registry flag / PLUGIN_REGISTRY_URL env (a base URL or a local
//   directory containing plugins.json + versions.json)
//   → https://raw.githubusercontent.com/companion-lab/plugin-registry/main
//   SYNC_REGISTRY_FIXTURE=<path> replaces the fetch entirely with a merged
//   {"plugins": […], "versions": {repo: version}} JSON — offline tests.
//
// A versions.json fetch failure only downgrades the chips: the catalog
// still builds, with plugins without a resolved version rendering as
// "unreleased". A plugins.json fetch failure keeps the committed catalog —
// a registry outage must never take the site down.
//
// --check exits 1 when the committed file is stale (used by CI); --dry-run
// prints instead of writing.
//
// This script owns src/plugin_catalog.rs.
// DO NOT EDIT — edit companion-lab/plugin-registry instead.

import { readFileSync, writeFileSync } from "node:fs";

const OUTPUT_PATH = "src/plugin_catalog.rs";
const DEFAULT_REGISTRY =
  "https://raw.githubusercontent.com/companion-lab/plugin-registry/main";

const args = process.argv.slice(2);
const dryRun = args.includes("--dry-run");
const check = args.includes("--check");
const registryIdx = args.indexOf("--registry");
const registryBase =
  registryIdx !== -1
    ? args[registryIdx + 1]
    : (process.env.PLUGIN_REGISTRY_URL || DEFAULT_REGISTRY).replace(/\/$/, "");

function escapeRust(text) {
  return text
    .replace(/\\/g, "\\\\")
    .replace(/"/g, '\\"')
    .replace(/\n/g, "\\n")
    .replace(/\r/g, "\\r")
    .replace(/\t/g, "\\t");
}

function rustStr(text) {
  return `"${escapeRust(text)}"`;
}

function renderPlugin(plugin) {
  const permissions = plugin.permissions.map((perm) => rustStr(perm)).join(", ");
  return [
    "    PluginEntry {",
    `        id: ${rustStr(plugin.id)},`,
    `        repo: ${rustStr(plugin.repo)},`,
    `        name: ${rustStr(plugin.name)},`,
    `        version: ${rustStr(plugin.version)},`,
    `        description: ${rustStr(plugin.description)},`,
    `        icon: ${rustStr(plugin.icon)},`,
    `        color: ${rustStr(plugin.color)},`,
    `        permissions: &[${permissions}],`,
    `        frontend: ${plugin.frontend},`,
    `        backend: ${plugin.backend},`,
    "    },",
  ].join("\n");
}

// The permission filter's chips, in first-appearance order across the
// catalog (what the hand-written list used to be kept in sync with).
function catalogPermissions(plugins) {
  const seen = [];
  for (const plugin of plugins) {
    for (const perm of plugin.permissions) {
      if (!seen.includes(perm)) seen.push(perm);
    }
  }
  return seen;
}

function render(plugins) {
  const permissions = catalogPermissions(plugins);
  const catalog = plugins.length
    ? `[\n${plugins.map(renderPlugin).join("\n")}\n]`
    : "[]";
  return [
    "//! The `/plugins` page's catalog, generated from the community plugin",
    "//! registry (companion-lab/plugin-registry's plugins.json + versions.json)",
    "//! by scripts/sync-registry.mjs.",
    "//!",
    "//! DO NOT EDIT — edit the registry instead; the next Pages build embeds",
    "//! the fresh catalog (deploy-pages.yml runs the sync before building).",
    "//! `version` is the repo's latest published release resolved by the",
    '//! registry; "" means none yet (the card shows "unreleased").',
    "",
    "use crate::pages::PluginEntry;",
    "",
    `pub static PLUGIN_CATALOG: [PluginEntry; ${plugins.length}] = ${catalog};`,
    "",
    `pub static CATALOG_PERMISSIONS: [&str; ${permissions.length}] = [${permissions
      .map((perm) => rustStr(perm))
      .join(", ")}];`,
  ].join("\n");
}

async function fetchJson(name) {
  if (/^https?:\/\//.test(registryBase)) {
    const url = `${registryBase}/${name}`;
    const response = await fetch(url, {
      headers: { "User-Agent": "companion-landing-page-sync" },
    });
    if (!response.ok) {
      throw new Error(`${url} returned HTTP ${response.status}`);
    }
    return response.json();
  }
  return JSON.parse(readFileSync(`${registryBase}/${name}`, "utf8"));
}

async function main() {
  let registry;
  try {
    registry = await fetchJson("plugins.json");
  } catch (error) {
    // Same stance as the old version-sync: a registry outage must never
    // break the site build — keep the committed catalog and warn loudly.
    console.warn(`warn: registry fetch failed (${error.message}) — keeping the committed catalog`);
    if (check) process.exitCode = 1;
    return;
  }

  // Versions are a separate generated file (the registry resolves each
  // repo's latest release); a failure there only downgrades chips.
  let versions = {};
  try {
    versions = await fetchJson("versions.json");
    delete versions.$comment;
  } catch (error) {
    console.warn(`warn: versions fetch failed (${error.message}) — chips render as "unreleased"`);
  }

  const plugins = (registry.plugins ?? []).map((plugin) => ({
    ...plugin,
    version: versions[plugin.repo] ?? "",
  }));
  const rendered = render(plugins);

  if (dryRun) {
    process.stdout.write(rendered);
    return;
  }
  if (check) {
    let current = null;
    try {
      current = readFileSync(OUTPUT_PATH, "utf8");
    } catch {
      // missing file → stale by definition
    }
    if (current !== rendered) {
      console.error(`${OUTPUT_PATH} is stale — run 'node scripts/sync-registry.mjs' and commit the result.`);
      process.exitCode = 1;
      return;
    }
    console.log(`${OUTPUT_PATH} is up to date (${plugins.length} plugin(s))`);
    return;
  }

  writeFileSync(OUTPUT_PATH, rendered);
  console.log(
    `wrote ${OUTPUT_PATH} (${plugins.length} plugin(s), ${Object.keys(versions).length} version(s) resolved) from ${registryBase}`,
  );
}

await main();
