#!/usr/bin/env node
// Regenerate src/plugin_catalog.rs from the community plugin registry
// (companion-lab/plugin-registry's plugins.json).
//
//   node scripts/sync-registry.mjs [--dry-run] [--check] [--registry <url-or-path>]
//
// The registry repo is the source of truth: contributors edit plugins.json
// there, its publish workflow dispatches this repo after every merge, and
// deploy-pages runs this script before building so the static wasm ships the
// fresh catalog (the wasm client cannot fetch one at runtime — same reason
// the docs markdown uses include_str!). The committed plugin_catalog.rs also
// keeps local builds hermetic: no network needed unless you re-sync.
//
// Source resolution, in order:
//   --registry flag / PLUGIN_REGISTRY_URL env (a URL or a local file path)
//   → https://raw.githubusercontent.com/companion-lab/plugin-registry/main/plugins.json
//   SYNC_REGISTRY_FIXTURE=<path> replaces the fetch entirely — offline tests.
//
// --check exits 1 when the committed file is stale (used by CI); --dry-run
// prints instead of writing. Never fails on a fetch error: the committed
// catalog stays as-is and the build proceeds (a registry outage must not
// take the site down).
//
// This script owns src/plugin_catalog.rs.
// DO NOT EDIT — edit companion-lab/plugin-registry instead.

import { readFileSync, writeFileSync } from "node:fs";

const OUTPUT_PATH = "src/plugin_catalog.rs";
const DEFAULT_SOURCE =
  "https://raw.githubusercontent.com/companion-lab/plugin-registry/main/plugins.json";

const args = process.argv.slice(2);
const dryRun = args.includes("--dry-run");
const check = args.includes("--check");
const registryIdx = args.indexOf("--registry");
const source =
  registryIdx !== -1
    ? args[registryIdx + 1]
    : (process.env.PLUGIN_REGISTRY_URL || DEFAULT_SOURCE);

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
    "//! registry (companion-lab/plugin-registry's plugins.json) by",
    "//! scripts/sync-registry.mjs.",
    "//!",
    "//! DO NOT EDIT — edit the registry instead; the next Pages build embeds",
    "//! the fresh catalog (deploy-pages.yml runs the sync before building).",
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

async function fetchRegistry() {
  const fixturePath = process.env.SYNC_REGISTRY_FIXTURE;
  if (fixturePath) {
    return JSON.parse(readFileSync(fixturePath, "utf8"));
  }
  if (/^https?:\/\//.test(source)) {
    const response = await fetch(source, {
      headers: { "User-Agent": "companion-landing-page-sync" },
    });
    if (!response.ok) {
      throw new Error(`${source} returned HTTP ${response.status}`);
    }
    return response.json();
  }
  return JSON.parse(readFileSync(source, "utf8"));
}

async function main() {
  let registry;
  try {
    registry = await fetchRegistry();
  } catch (error) {
    // Same stance as the old version-sync: a registry outage must never
    // break the site build — keep the committed catalog and warn loudly.
    console.warn(`warn: registry fetch failed (${error.message}) — keeping the committed catalog`);
    if (check) process.exitCode = 1;
    return;
  }

  const plugins = registry.plugins ?? [];
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
  console.log(`wrote ${OUTPUT_PATH} (${plugins.length} plugin(s)) from ${source}`);
}

await main();
