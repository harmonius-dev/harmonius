//! Partial clone / sparse checkout path filtering (**TC-15.10.6.1**, R-15.10.6).

use std::path::Path;

/// Path-prefix rules describing which worktree paths stay materialized after sparse checkout.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SparseCheckoutConfig {
    /// Repository-relative directory prefixes (POSIX separators, trailing `/` optional).
    pub include_prefixes: Vec<String>,
}

impl SparseCheckoutConfig {
    /// Build a config from normalized prefixes (slashes, no leading `./`).
    pub fn new(prefixes: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let include_prefixes = prefixes.into_iter().map(|p| normalize_prefix(p.into())).collect();
        Self { include_prefixes }
    }

    /// Whether `path` should exist in the sparse worktree under this config.
    pub fn path_included(&self, path: &Path) -> bool {
        let s = path.to_string_lossy();
        let posix = s.replace('\\', "/");
        self.include_prefixes
            .iter()
            .any(|p| posix == *p.as_str() || posix.starts_with(&format!("{p}/")))
    }
}

fn normalize_prefix(mut p: String) -> String {
    p = p.replace('\\', "/");
    while p.starts_with("./") {
        p = p.trim_start_matches("./").to_string();
    }
    p.trim_end_matches('/').to_string()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    /// **TC-15.10.6.1** — Sparse checkout config filters worktree paths correctly.
    #[test]
    fn tc_15_10_6_1_sparse_filters_paths() {
        let cfg = SparseCheckoutConfig::new(["src", "assets/textures/"]);
        assert!(cfg.path_included(Path::new("src/lib.rs")));
        assert!(cfg.path_included(Path::new("assets/textures/a.png")));
        assert!(!cfg.path_included(Path::new("tools/build.rs")));
        assert!(cfg.path_included(Path::new("src")));
    }
}
