#!/usr/bin/env node
// Sync the plugin catalog's hardcoded version chips with the plugins'
// published GitHub releases.
//
//   node scripts/sync-plugin-versions.mjs [--dry-run] [--catalog <path>]
//
// Reads every PluginEntry's `repo` from src/pages/plugins.rs, queries
// GitHub's releases/latest endpoint for it, and rewrites the entry's
// `version: "..."` line when the published release differs. Deploy-pages
// runs this before building, so the static wasm ships fresh chips without
// burning any visitor-side API budget.
//
// Never fails the build: a repo whose lookup errors (rate limit, no
// releases, transient 5xx) keeps its current version and prints a warning.
//
// SYNC_PLUGIN_FIXTURE=<path to {"repo": "version", …} JSON> replaces the
// GitHub lookups entirely — used to test the rewrite path offline.

import { readFileSync, writeFileSync } from "node:fs";

const args = process.argv.slice(2);
const dryRun = args.includes("--dry-run");
const catalogIdx = args.indexOf("--catalog");
const catalogPath =
  catalogIdx !== -1 ? args[catalogIdx + 1] : "src/pages/plugins.rs";

const catalog = readFileSync(catalogPath, "utf8");

// Locate each PluginEntry block: from its `{` to the next entry or the
// array's closing `];`. Each block carries `repo: "…"` and `version: "…"`.
const ENTRY_START = /PluginEntry \{/g;
const blocks = [];
let starts = [...catalog.matchAll(ENTRY_START)].map((m) => m.index);
for (let i = 0; i < starts.length; i++) {
  const end =
    i + 1 < starts.length
      ? starts[i + 1]
      : catalog.indexOf("];", starts[i]) + 2;
  const text = catalog.slice(starts[i], end);
  const repo = text.match(/repo: "([^"]+)"/)?.[1];
  const version = text.match(/version: "([^"]+)"/)?.[1];
  if (!repo || !version) {
    console.error(`skip unparseable entry at offset ${starts[i]}`);
    continue;
  }
  blocks.push({ start: starts[i], end, repo, version });
}

// One repo's published release version, or null on any failure (the caller
// keeps the current version — see the header note about never failing).
async function latestReleaseVersion(repo, currentVersion) {
  const url = `https://api.github.com/repos/companion-lab/${repo}/releases/latest`;
  const headers = { Accept: "application/vnd.github+json", "User-Agent": "companion-landing-page-sync" };
  if (process.env.GITHUB_TOKEN) {
    headers.Authorization = `Bearer ${process.env.GITHUB_TOKEN}`;
  }
  try {
    const response = await fetch(url, { headers });
    if (!response.ok) {
      console.warn(`warn: ${repo}: GitHub API returned HTTP ${response.status} — keeping ${currentVersion}`);
      return null;
    }
    const release = await response.json();
    if (release.draft || release.prerelease) return null; // unexpected from /latest, stay safe
    return release.tag_name.replace(/^v/, "");
  } catch (error) {
    console.warn(`warn: ${repo}: lookup failed (${error.message})`);
    return null;
  }
}

async function main() {
  const fixturePath = process.env.SYNC_PLUGIN_FIXTURE;
  const published = fixturePath
    ? JSON.parse(readFileSync(fixturePath, "utf8"))
    : {};

  const resolved = new Map();
  for (const block of blocks) {
    if (fixturePath) {
      // Fixture mode: absent repo = keep current (mirrors an API failure).
      resolved.set(block.repo, published[block.repo] ?? null);
    } else {
      resolved.set(block.repo, await latestReleaseVersion(block.repo, block.version));
    }
  }

  // Rebuild the catalog with per-entry version swaps (only where changed).
  let out = catalog;
  let changed = 0;
  for (let i = blocks.length - 1; i >= 0; i--) {
    const { start, end, repo, version } = blocks[i];
    const fresh = resolved.get(repo);
    if (!fresh || fresh === version) continue;
    const blockText = catalog.slice(start, end);
    const newBlockText = blockText.replace(
      `version: "${version}"`,
      `version: "${fresh}"`,
    );
    if (newBlockText === blockText) {
      console.warn(`warn: ${repo}: could not rewrite version line`);
      continue;
    }
    out = out.slice(0, start) + newBlockText + out.slice(end);
    console.log(`${repo}: ${version} -> ${fresh}`);
    changed++;
  }

  if (dryRun) {
    console.log(`dry-run: ${changed} version(s) would change`);
  } else if (changed > 0) {
    writeFileSync(catalogPath, out);
    console.log(`wrote ${changed} version(s) to ${catalogPath}`);
  } else {
    console.log("catalog already up to date");
  }
}

await main();
