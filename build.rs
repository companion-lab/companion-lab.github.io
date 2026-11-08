use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// External links baked into the site (read by src/config.rs via `env!`),
/// resolved with dotenv precedence: an exported shell variable wins, then the
/// `.env` value, then the hardcoded default. The defaults also cover CI
/// builds (GitHub Pages) that run without a `.env`.
const SITE_LINKS: [(&str, &str); 3] = [
    ("COMMUNITY_GITHUB_URL", "https://github.com/companion-vr"),
    ("DISCORD_URL", "https://discord.gg/G46H7B9x"),
    (
        "RELEASES_URL",
        "https://github.com/companion/companion/releases/latest",
    ),
];

fn main() {
    // Rerun whenever a link source changes, so the values emitted via
    // `cargo:rustc-env` below are re-resolved instead of going stale.
    println!("cargo:rerun-if-changed=.env");
    for (key, _) in SITE_LINKS {
        println!("cargo:rerun-if-env-changed={key}");
    }

    let file_vars = parse_env_file(&Path::new(env!("CARGO_MANIFEST_DIR")).join(".env"));
    for (key, default) in SITE_LINKS {
        let from_shell = std::env::var(key)
            .ok()
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty());
        let from_file = file_vars
            .get(key)
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty());
        let value = from_shell
            .or(from_file)
            .unwrap_or_else(|| default.to_string());
        println!("cargo:rustc-env={key}={value}");
    }

    // Tell cargo to rerun this script when the vrma folder or its contents
    // change, so newly added/removed clips are picked up on the next build.
    println!("cargo:rerun-if-changed=assets/vrma");

    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/vrma");
    let mut names: Vec<String> = Vec::new();
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "vrma") {
                if let Some(name) = path.file_name() {
                    let name = name.to_string_lossy().to_string();
                    println!("cargo:rerun-if-changed=assets/vrma/{name}");
                    names.push(name);
                }
            }
        }
    }
    names.sort();

    // One hashed asset URL per clip (asset! requires the file to exist, which
    // it does — we just scanned it). The prelude glob is needed because the
    // asset! expansion refers to `manganis` via the prelude re-export. The
    // list is empty when no clips exist.
    let decls: String = names
        .iter()
        .map(|name| format!("    dioxus::prelude::asset!(\"/assets/vrma/{name}\").to_string(),\n"))
        .collect();

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR is set by cargo");
    fs::write(
        format!("{out_dir}/vrma_list.rs"),
        format!(
            "pub fn vrma_url_list() -> Vec<String> {{\n    use dioxus::prelude::*;\n    (&[\n{decls}]).to_vec()\n}}"
        ),
    )
    .expect("failed to write vrma_list.rs");
}

/// Minimal `.env` reader — enough for URL constants without pulling dotenvy
/// into the build graph: `KEY=VALUE` lines with an optional `export` prefix,
/// surrounding single/double quotes, `#` full-line comments, and unquoted
/// inline `#` comments. Unparseable lines are skipped.
fn parse_env_file(path: &Path) -> BTreeMap<String, String> {
    let mut vars = BTreeMap::new();
    let Ok(content) = fs::read_to_string(path) else {
        return vars; // no .env — the defaults apply
    };
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let line = line.strip_prefix("export ").unwrap_or(line).trim_start();
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let mut value = value.trim();
        let quoted = value.len() >= 2
            && matches!(value.as_bytes().first(), Some(b'"') | Some(b'\''))
            && value.ends_with(&value[..1]);
        if quoted {
            value = &value[1..value.len() - 1];
        } else if let Some(comment) = value.find(" #") {
            value = value[..comment].trim_end();
        }
        if !key.is_empty() {
            vars.insert(key.to_string(), value.to_string());
        }
    }
    vars
}
