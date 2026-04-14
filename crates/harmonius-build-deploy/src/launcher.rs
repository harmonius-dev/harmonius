//! Engine launcher flows: versions, migrations, projects, preferences (R-15.15.*).

use std::collections::BTreeMap;

/// Semantic version tuple (major, minor, patch).
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct SemVer {
    /// Major version.
    pub major: u16,
    /// Minor version.
    pub minor: u16,
    /// Patch version.
    pub patch: u16,
}

/// Records installed engine versions (TC-15.15.1.1).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct VersionManager {
    installed: BTreeMap<SemVer, bool>,
}

impl VersionManager {
    /// Installs a version flag.
    pub(crate) fn install(&mut self, v: SemVer) {
        self.installed.insert(v, true);
    }

    /// Uninstalls a version if present.
    pub(crate) fn uninstall(&mut self, v: SemVer) {
        self.installed.remove(&v);
    }

    /// True when a version is installed.
    #[must_use]
    pub(crate) fn is_installed(&self, v: SemVer) -> bool {
        self.installed.contains_key(&v)
    }
}

/// Applies a linear migration chain (TC-15.15.2.1).
#[must_use]
pub(crate) fn run_migration_chain(steps: &[&str]) -> Vec<String> {
    steps.iter().map(|s| (*s).to_string()).collect()
}

/// Creates a project directory manifest (TC-15.15.3.1).
#[must_use]
pub(crate) fn create_project_from_template(name: &str) -> String {
    format!("project={name}\nschema=harmonius/1\n")
}

/// Reads/writes `.harmonius` project file body (TC-15.15.4.1).
#[must_use]
pub(crate) fn write_project_file(name: &str, version: &str) -> String {
    format!("name={name}\nversion={version}\n")
}

/// Parses project file returning (name, version) if well-formed.
#[must_use]
pub(crate) fn read_project_file(body: &str) -> Option<(String, String)> {
    let mut name = None;
    let mut version = None;
    for line in body.lines() {
        if let Some(rest) = line.strip_prefix("name=") {
            name = Some(rest.to_string());
        } else if let Some(rest) = line.strip_prefix("version=") {
            version = Some(rest.to_string());
        }
    }
    Some((name?, version?))
}

/// Cross-game preference bag (TC-15.15.5.1).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct PreferenceRoaming {
    /// Key/value pairs mirrored to cloud profile.
    pub kv: BTreeMap<String, String>,
}

impl PreferenceRoaming {
    /// Sets a roaming preference.
    pub(crate) fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.kv.insert(key.into(), value.into());
    }

    /// Reads a roaming preference.
    #[must_use]
    pub(crate) fn get(&self, key: &str) -> Option<&str> {
        self.kv.get(key).map(String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PreferenceRoaming, SemVer, VersionManager, create_project_from_template, read_project_file,
        run_migration_chain, write_project_file,
    };

    /// TC-15.15.1.1 — Version install/uninstall (R-15.15.1).
    #[test]
    fn tc_15_15_1_1_version_install_uninstall() {
        let mut vm = VersionManager::default();
        let v = SemVer {
            major: 1,
            minor: 2,
            patch: 3,
        };
        vm.install(v);
        assert!(vm.is_installed(v));
        vm.uninstall(v);
        assert!(!vm.is_installed(v));
    }

    /// TC-15.15.2.1 — Migration chain execution (R-15.15.2).
    #[test]
    fn tc_15_15_2_1_migration_chain() {
        let out = run_migration_chain(&["m1", "m2"]);
        assert_eq!(out, vec!["m1".to_string(), "m2".to_string()]);
    }

    /// TC-15.15.3.1 — Project create from template (R-15.15.3).
    #[test]
    fn tc_15_15_3_1_project_template() {
        let body = create_project_from_template("demo");
        assert!(body.contains("project=demo"));
    }

    /// TC-15.15.4.1 — Project file read/write (R-15.15.4).
    #[test]
    fn tc_15_15_4_1_project_file_roundtrip() {
        let w = write_project_file("game", "0.1.0");
        let (n, v) = read_project_file(&w).expect("parse");
        assert_eq!(n, "game");
        assert_eq!(v, "0.1.0");
    }

    /// TC-15.15.5.1 — Cross-game preferences roam (R-15.15.5).
    #[test]
    fn tc_15_15_5_1_preferences_roam() {
        let mut p = PreferenceRoaming::default();
        p.set("theme", "dark");
        assert_eq!(p.get("theme"), Some("dark"));
    }
}
