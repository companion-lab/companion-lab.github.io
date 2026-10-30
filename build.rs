use std::fs;
use std::path::Path;

fn main() {
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
        .map(|name| {
            format!(
                "    dioxus::prelude::asset!(\"/assets/vrma/{name}\").to_string(),\n"
            )
        })
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
