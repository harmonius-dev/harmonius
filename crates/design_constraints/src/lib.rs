//! Automated checks for verifiable rules in `docs/design/constraints.md`.
#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const SKIP_DIRS: &[&str] = &[".git", "target", ".cursor"];

const FORBIDDEN_SOURCE_EXTS: &[&str] = &[
    "c", "cc", "cpp", "cxx", "h", "hpp", "hxx", "m", "mm", "swift",
];

const FORBIDDEN_MANIFEST_KEYS: &[&str] = &["serde", "tokio", "winit"];

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
pub fn scan_forbidden_source_files(root: &Path) -> io::Result<Vec<PathBuf>> {
    let mut hits = Vec::new();
    visit_tree(root, root, &mut hits)?;
    Ok(hits)
}

/// Scans Cargo manifests for dependencies forbidden in the engine workspace.
pub fn scan_forbidden_manifest_dependencies(root: &Path) -> io::Result<Vec<String>> {
    let mut hits = Vec::new();
    visit_cargo_files(root, root, &mut hits)?;
    Ok(hits)
}

fn visit_tree(root: &Path, dir: &Path, hits: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        if path.is_dir() {
            if should_skip_dir(&file_name) {
                continue;
            }
            visit_tree(root, &path, hits)?;
        } else if path.is_file() && is_forbidden_source_file(&path) {
            hits.push(path.strip_prefix(root).unwrap_or(&path).to_path_buf());
        }
    }
    Ok(())
}

fn should_skip_dir(name: &OsStr) -> bool {
    name.to_str().is_some_and(|n| SKIP_DIRS.contains(&n))
}

fn is_forbidden_source_file(path: &Path) -> bool {
    let Some(ext) = path.extension().and_then(OsStr::to_str) else {
        return false;
    };
    let ext = ext.to_ascii_lowercase();
    FORBIDDEN_SOURCE_EXTS.contains(&ext.as_str())
}

fn visit_cargo_files(root: &Path, dir: &Path, hits: &mut Vec<String>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        if path.is_dir() {
            if should_skip_dir(&file_name) {
                continue;
            }
            visit_cargo_files(root, &path, hits)?;
        } else if file_name == "Cargo.toml" {
            scan_manifest_file(root, &path, hits)?;
        }
    }
    Ok(())
}

fn scan_manifest_file(root: &Path, manifest_path: &Path, hits: &mut Vec<String>) -> io::Result<()> {
    let text = fs::read_to_string(manifest_path)?;
    let mut in_dep_section = false;
    for raw_line in text.lines() {
        let line = raw_line.split('#').next().unwrap_or("").trim();
        if line.starts_with('[') && line.ends_with(']') {
            in_dep_section = is_dependency_section_header(line);
            continue;
        }
        if !in_dep_section {
            continue;
        }
        if let Some(key) = forbidden_manifest_key(line) {
            let rel = manifest_path.strip_prefix(root).unwrap_or(manifest_path);
            hits.push(format!("{}: forbidden dependency `{key}`", rel.display()));
        }
    }
    Ok(())
}

fn is_dependency_section_header(header: &str) -> bool {
    matches!(
        header,
        "[dependencies]"
            | "[build-dependencies]"
            | "[dev-dependencies]"
            | "[workspace.dependencies]"
    ) || (header.starts_with("[target.") && header.ends_with(".dependencies]"))
}

fn forbidden_manifest_key(line: &str) -> Option<&'static str> {
    let line = line.trim();
    for key in FORBIDDEN_MANIFEST_KEYS {
        let Some(rest) = line.strip_prefix(key) else {
            continue;
        };
        let rest = rest.trim_start();
        if rest.starts_with('=') {
            return Some(key);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repo_has_no_forbidden_source_files() {
        let root = repo_root_from_manifest(Path::new(env!("CARGO_MANIFEST_DIR")));
        let hits = scan_forbidden_source_files(&root).expect("scan sources");
        assert!(hits.is_empty(), "unexpected forbidden sources: {hits:#?}");
    }

    #[test]
    fn repo_has_no_forbidden_manifest_dependencies() {
        let root = repo_root_from_manifest(Path::new(env!("CARGO_MANIFEST_DIR")));
        let hits = scan_forbidden_manifest_dependencies(&root).expect("scan manifests");
        assert!(
            hits.is_empty(),
            "unexpected forbidden manifest deps: {hits:#?}"
        );
    }
}
