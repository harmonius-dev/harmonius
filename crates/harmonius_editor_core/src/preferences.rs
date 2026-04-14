//! Versioned editor preferences (`TC-15.1.7.*`).

use std::collections::HashMap;
use std::fmt;

/// Typed preference key wrapper.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PreferenceKey(pub String);

/// Preference store errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PreferenceError {
    /// Key missing for `get`.
    MissingKey,
    /// Migration failed for the requested target version.
    MigrationFailed,
}

impl fmt::Display for PreferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PreferenceError::MissingKey => write!(f, "missing preference key"),
            PreferenceError::MigrationFailed => write!(f, "preference migration failed"),
        }
    }
}

impl std::error::Error for PreferenceError {}

/// In-memory preference bag with integer schema versions.
#[derive(Debug, Clone)]
pub struct PreferenceStore {
    schema_version: u16,
    data: HashMap<String, String>,
}

impl PreferenceStore {
    /// Creates an empty store at `schema_version`.
    pub fn new(schema_version: u16) -> Self {
        Self {
            schema_version,
            data: HashMap::new(),
        }
    }

    /// Current schema version.
    pub fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Reads a string value.
    pub fn get(&self, key: &PreferenceKey) -> Result<String, PreferenceError> {
        self.data
            .get(&key.0)
            .cloned()
            .ok_or(PreferenceError::MissingKey)
    }

    /// Writes a string value.
    pub fn set(&mut self, key: PreferenceKey, value: String) {
        self.data.insert(key.0, value);
    }

    /// Runs built-in migrations up to `target` version.
    pub fn migrate(&mut self, target: u16) -> Result<(), PreferenceError> {
        while self.schema_version < target {
            match self.schema_version {
                0 => {
                    if let Some(old) = self.data.remove("legacy_theme") {
                        self.data.insert("ui.theme".to_string(), old);
                    }
                    self.schema_version = 1;
                }
                1 => {
                    if let Some(v) = self.data.get("ui.theme").cloned() {
                        self.data.insert("ui.theme.v2".to_string(), v);
                    }
                    self.schema_version = 2;
                }
                _ => return Err(PreferenceError::MigrationFailed),
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_15_1_7_1_preference_get_set() {
        let mut p = PreferenceStore::new(2);
        let k = PreferenceKey("ui.scale".to_string());
        p.set(k.clone(), "1.25".to_string());
        assert_eq!(p.get(&k).unwrap(), "1.25");
    }

    #[test]
    fn tc_15_1_7_2_preference_migration() {
        let mut p = PreferenceStore::new(0);
        p.set(
            PreferenceKey("legacy_theme".to_string()),
            "dark".to_string(),
        );
        p.migrate(2).unwrap();
        assert_eq!(p.schema_version(), 2);
        assert_eq!(
            p.get(&PreferenceKey("ui.theme.v2".to_string())).unwrap(),
            "dark"
        );
    }
}
