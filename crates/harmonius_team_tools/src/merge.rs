//! Asset-aware three-way structural merge (**TC-15.10.3.1**, **TC-15.10.3.2**, R-15.10.3).
//!
//! Inputs are UTF-8 `path=value` lines sorted deterministically for tests; production wiring swaps
//! the body for scene archives without changing the merge contract.

use std::collections::BTreeMap;

use git2::Repository;

use crate::git_status::VcError;

/// Outcome of [`MergeDriver::invoke`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MergeResult {
    /// Clean merge with merged bytes ready for Git.
    Merged(Vec<u8>),
    /// Conflicting property keys requiring the conflict UI.
    Conflicts(Vec<String>),
}

/// Three-way structural merge driver (see design `MergeDriver`).
#[derive(Clone, Copy, Debug, Default)]
pub struct MergeDriver;

impl MergeDriver {
    /// New merge driver instance.
    pub fn new() -> Self {
        Self
    }

    /// Register a local `.git/info/attributes` entry for Harmonius structural merge drivers.
    pub fn register_gitattributes(&self, repo: &Repository) -> Result<(), VcError> {
        let wd = repo.workdir().ok_or_else(|| VcError::Git {
            message: "bare repository has no workdir for attributes".to_string(),
        })?;
        let info = wd.join(".git/info");
        std::fs::create_dir_all(&info).map_err(|e| VcError::Git {
            message: format!("create .git/info: {e}"),
        })?;
        let line = "*.harmon merge=harmon-structural\n";
        std::fs::write(info.join("attributes"), line).map_err(|e| VcError::Git {
            message: format!("write attributes: {e}"),
        })?;
        Ok(())
    }

    /// Invoke three-way merge over line-oriented property maps.
    pub fn invoke(&self, base: &[u8], ours: &[u8], theirs: &[u8]) -> MergeResult {
        let Ok(b) = parse_prop_map(base) else {
            return MergeResult::Conflicts(Vec::new());
        };
        let Ok(o) = parse_prop_map(ours) else {
            return MergeResult::Conflicts(Vec::new());
        };
        let Ok(t) = parse_prop_map(theirs) else {
            return MergeResult::Conflicts(Vec::new());
        };

        let keys = union_keys(&b, &o, &t);
        let mut merged = BTreeMap::new();
        let mut conflicts = Vec::new();

        for k in keys {
            let bv = b.get(&k).map(String::as_str).unwrap_or("");
            let ov = o.get(&k).map(String::as_str).unwrap_or("");
            let tv = t.get(&k).map(String::as_str).unwrap_or("");

            if ov == tv {
                merged.insert(k.clone(), ov.to_string());
                continue;
            }
            if ov == bv {
                merged.insert(k.clone(), tv.to_string());
                continue;
            }
            if tv == bv {
                merged.insert(k.clone(), ov.to_string());
                continue;
            }
            conflicts.push(k);
        }

        if conflicts.is_empty() {
            MergeResult::Merged(serialize_prop_map(&merged))
        } else {
            conflicts.sort();
            MergeResult::Conflicts(conflicts)
        }
    }
}

fn parse_prop_map(data: &[u8]) -> Result<BTreeMap<String, String>, ()> {
    let text = std::str::from_utf8(data).map_err(|_| ())?;
    let mut map = BTreeMap::new();
    for line in text.lines() {
        if line.is_empty() {
            continue;
        }
        let (k, v) = line.split_once('=').ok_or(())?;
        if k.is_empty() {
            return Err(());
        }
        map.insert(k.to_string(), v.to_string());
    }
    Ok(map)
}

fn serialize_prop_map(map: &BTreeMap<String, String>) -> Vec<u8> {
    let mut s = String::new();
    for (k, v) in map {
        if !s.is_empty() {
            s.push('\n');
        }
        s.push_str(k);
        s.push('=');
        s.push_str(v);
    }
    s.into_bytes()
}

fn union_keys(
    b: &BTreeMap<String, String>,
    o: &BTreeMap<String, String>,
    t: &BTreeMap<String, String>,
) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for k in b.keys() {
        out.push(k.clone());
    }
    for k in o.keys() {
        if !b.contains_key(k) {
            out.push(k.clone());
        }
    }
    for k in t.keys() {
        if !b.contains_key(k) && !o.contains_key(k) {
            out.push(k.clone());
        }
    }
    out.sort();
    out.dedup();
    out
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::process::Command;

    use tempfile::TempDir;

    use super::*;

    fn init_repo(dir: &std::path::Path) {
        let st = Command::new("git")
            .args(["init", "-q"])
            .current_dir(dir)
            .status()
            .expect("git init");
        assert!(st.success());
    }

    /// **TC-15.10.3.1** — Structural merge with no conflicts produces clean merge.
    #[test]
    fn tc_15_10_3_1_clean_structural_merge() {
        let base = b"a=1\nb=2";
        let ours = b"a=1\nb=3";
        let theirs = b"a=1\nb=3";
        let md = MergeDriver::new();
        match md.invoke(base, ours, theirs) {
            MergeResult::Merged(bytes) => {
                let got = String::from_utf8(bytes).expect("utf8");
                assert_eq!(got, "a=1\nb=3");
            }
            MergeResult::Conflicts(c) => panic!("unexpected conflicts {c:?}"),
        }
    }

    /// **TC-15.10.3.2** — Structural merge with conflicts opens conflict UI.
    #[test]
    fn tc_15_10_3_2_structural_merge_conflicts() {
        let base = b"k=1";
        let ours = b"k=2";
        let theirs = b"k=3";
        let md = MergeDriver::new();
        match md.invoke(base, ours, theirs) {
            MergeResult::Conflicts(keys) => assert_eq!(keys, vec!["k".to_string()]),
            MergeResult::Merged(b) => panic!("unexpected merge {b:?}"),
        }
    }

    /// `register_gitattributes` writes `.git/info/attributes` merge driver mapping.
    #[test]
    fn register_gitattributes_writes_info_attributes() {
        let dir = TempDir::new().expect("tempdir");
        init_repo(dir.path());
        let repo = Repository::open(dir.path()).expect("open");
        MergeDriver::new()
            .register_gitattributes(&repo)
            .expect("register");
        let text = fs::read_to_string(dir.path().join(".git/info/attributes")).expect("read");
        assert!(text.contains("*.harmon"));
    }
}
