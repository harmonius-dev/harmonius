//! TOML preferences with dirty tracking (`R-14.5.8`).

use std::collections::HashMap;
use std::fs;
use crate::filesystem::{CanonicalPath, FsError};

use super::error::PrefsError;

/// Strongly typed preference value.
#[derive(Clone, Debug, PartialEq)]
pub enum PrefValue {
    /// Boolean flag.
    Bool(bool),
    /// Floating-point scalar.
    Float(f64),
    /// UTF-8 string.
    String(String),
}

/// Key plus default used for `get`.
#[derive(Clone, Debug, PartialEq)]
pub struct PrefKey {
    /// Logical key name.
    pub key: &'static str,
    /// Default when unset.
    pub default: PrefValue,
}

/// Cloud sync hook (stub for tests).
#[derive(Debug, Default)]
pub struct NoCloud;

/// Mutable preferences map.
#[derive(Debug)]
pub struct PreferencesStore {
    path: CanonicalPath,
    values: HashMap<String, PrefValue>,
    defaults: HashMap<String, PrefValue>,
    dirty: bool,
}

impl PreferencesStore {
    /// Loads preferences from disk (creates empty if missing).
    pub fn load(path: CanonicalPath) -> Result<Self, PrefsError> {
        let mut values = HashMap::new();
        if path.to_path_buf().exists() {
            let raw = fs::read_to_string(path.to_path_buf()).map_err(|e| {
                PrefsError::IoError(FsError::Platform {
                    code: e.raw_os_error().unwrap_or(-1),
                    message: e.to_string(),
                })
            })?;
            let table: toml::Value = toml::from_str(&raw).map_err(|e| PrefsError::ParseError {
                line: 0,
                message: e.to_string(),
            })?;
            if let Some(map) = table.as_table() {
                for (k, v) in map {
                    if let Some(b) = v.as_bool() {
                        values.insert(k.clone(), PrefValue::Bool(b));
                    } else if let Some(f) = v.as_float() {
                        values.insert(k.clone(), PrefValue::Float(f));
                    } else if let Some(s) = v.as_str() {
                        values.insert(k.clone(), PrefValue::String(s.into()));
                    }
                }
            }
        }
        Ok(Self {
            path,
            values,
            defaults: HashMap::new(),
            dirty: false,
        })
    }

    /// Reads a value or its default.
    pub fn get(&self, key: &PrefKey) -> PrefValue {
        self.values
            .get(key.key)
            .cloned()
            .unwrap_or_else(|| key.default.clone())
    }

    /// Writes a value and marks the store dirty.
    pub fn set(&mut self, key: &'static str, value: PrefValue) {
        self.values.insert(key.into(), value);
        self.dirty = true;
    }

    /// Returns whether there are unsaved edits.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Registers defaults used by [`PreferencesStore::reset_to_defaults`].
    pub fn register_default(&mut self, key: &'static str, value: PrefValue) {
        self.defaults.insert(key.into(), value);
    }

    /// Resets listed keys to their registered defaults.
    pub fn reset_to_defaults(&mut self, keys: &[&'static str]) {
        for k in keys {
            if let Some(v) = self.defaults.get(*k).cloned() {
                self.values.insert((*k).into(), v);
            }
        }
        self.dirty = true;
    }

    /// Atomically saves preferences to the backing path.
    pub fn save(&mut self, _cloud: &NoCloud) -> Result<(), PrefsError> {
        let mut table = toml::map::Map::new();
        for (k, v) in &self.values {
            let tv = match v {
                PrefValue::Bool(b) => toml::Value::Boolean(*b),
                PrefValue::Float(f) => toml::Value::Float(*f),
                PrefValue::String(s) => toml::Value::String(s.clone()),
            };
            table.insert(k.clone(), tv);
        }
        let body = toml::to_string(&toml::Value::Table(table))
            .map_err(|e| PrefsError::ParseError {
                line: 0,
                message: e.to_string(),
            })?;
        let pb = self.path.to_path_buf();
        if let Some(parent) = pb.parent() {
            fs::create_dir_all(parent).map_err(|e| PrefsError::IoError(FsError::Platform {
                code: e.raw_os_error().unwrap_or(-1),
                message: e.to_string(),
            }))?;
        }
        let tmp = pb.with_extension("toml.tmp");
        fs::write(&tmp, body).map_err(|e| PrefsError::IoError(FsError::Platform {
            code: e.raw_os_error().unwrap_or(-1),
            message: e.to_string(),
        }))?;
        fs::rename(&tmp, &pb).map_err(|e| PrefsError::IoError(FsError::Platform {
            code: e.raw_os_error().unwrap_or(-1),
            message: e.to_string(),
        }))?;
        self.dirty = false;
        Ok(())
    }

    /// Returns the backing file path for inspection in tests.
    pub fn path(&self) -> &CanonicalPath {
        &self.path
    }
}

/// Returns OS-specific root folders for common Harmonius data classes.
pub struct PlatformPaths;

impl PlatformPaths {
    /// Preferences root for `app`.
    pub fn preferences(app: &str) -> CanonicalPath {
        #[cfg(target_os = "macos")]
        {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            CanonicalPath::resolve(&format!(
                "{}/Library/Application Support/{}/prefs",
                home, app
            ))
            .unwrap_or_else(|_| CanonicalPath::resolve(".").unwrap())
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = app;
            CanonicalPath::resolve("prefs-root").unwrap()
        }
    }

    /// Player cache root for `app`.
    pub fn player_cache(app: &str) -> CanonicalPath {
        #[cfg(target_os = "macos")]
        {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            CanonicalPath::resolve(&format!("{}/Library/Caches/{}", home, app))
                .unwrap_or_else(|_| CanonicalPath::resolve(".").unwrap())
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = app;
            CanonicalPath::resolve("cache-root").unwrap()
        }
    }

    /// Temporary scratch root for `app`.
    pub fn temp(app: &str) -> CanonicalPath {
        #[cfg(target_os = "macos")]
        {
            let base = std::env::temp_dir();
            CanonicalPath::from_std_path(&base.join(app)).unwrap_or_else(|_| {
                CanonicalPath::resolve(&format!("/tmp/{}", app)).unwrap()
            })
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = app;
            CanonicalPath::resolve("tmp-root").unwrap()
        }
    }
}
