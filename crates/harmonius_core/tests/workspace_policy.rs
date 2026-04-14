//! Workspace policy checks (crate-type, layout).

use std::fs;
use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn read_toml(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn manifest_paths_under_crates() -> Vec<PathBuf> {
    let crates_dir = workspace_root().join("crates");
    let mut out = Vec::new();
    if let Ok(entries) = fs::read_dir(&crates_dir) {
        for entry in entries.flatten() {
            let manifest = entry.path().join("Cargo.toml");
            if manifest.is_file() {
                out.push(manifest);
            }
        }
    }
    out
}

fn declares_cdylib(manifest: &str) -> bool {
    let mut in_lib = false;
    for line in manifest.lines() {
        let trimmed = line.trim();
        if trimmed == "[lib]" {
            in_lib = true;
            continue;
        }
        if trimmed.starts_with('[') && trimmed != "[lib]" {
            in_lib = false;
        }
        if !in_lib {
            continue;
        }
        let normalized: String = trimmed.chars().filter(|c| !c.is_whitespace()).collect();
        if normalized.contains("crate-type=[\"cdylib\"]")
            || normalized.contains("crate-type=[\"cdylib\",\"rlib\"]")
            || normalized.contains("crate-type=[\"rlib\",\"cdylib\"]")
        {
            return true;
        }
    }
    false
}

#[test]
fn test_public_crates_have_no_cdylib() {
    for manifest in manifest_paths_under_crates() {
        let text = read_toml(&manifest);
        assert!(
            !declares_cdylib(&text),
            "cdylib is forbidden in public console crates: {}",
            manifest.display()
        );
    }
}
