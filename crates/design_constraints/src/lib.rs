//! Automated checks for verifiable rules in `docs/design/constraints.md`.
#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

use std::path::{Path, PathBuf};

/// Returns the repository root that contains `docs/design/constraints.md`.
#[must_use]
pub fn repo_root_from_manifest(manifest_dir: &Path) -> PathBuf {
    let mut dir = manifest_dir.to_path_buf();
    loop {
        if dir.join("docs/design/constraints.md").is_file() {
            return dir;
        }
        assert!(dir.pop(), "Harmonius repo root not found from manifest dir");
    }
}

/// Scans the repository for source files forbidden by `docs/design/constraints.md`.
#[must_use]
pub fn scan_forbidden_source_files(_root: &Path) -> Vec<PathBuf> {
    vec![PathBuf::from("__design_constraints_red_stub__")]
}

/// Scans Cargo manifests for dependencies forbidden in the engine workspace.
#[must_use]
pub fn scan_forbidden_manifest_dependencies(_root: &Path) -> Vec<String> {
    vec!["__design_constraints_red_stub__".to_string()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repo_has_no_forbidden_source_files() {
        let root = repo_root_from_manifest(Path::new(env!("CARGO_MANIFEST_DIR")));
        let hits = scan_forbidden_source_files(&root);
        assert!(hits.is_empty(), "unexpected forbidden sources: {hits:#?}");
    }

    #[test]
    fn repo_has_no_forbidden_manifest_dependencies() {
        let root = repo_root_from_manifest(Path::new(env!("CARGO_MANIFEST_DIR")));
        let hits = scan_forbidden_manifest_dependencies(&root);
        assert!(hits.is_empty(), "unexpected forbidden manifest deps: {hits:#?}");
    }
}
