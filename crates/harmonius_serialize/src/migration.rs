//! Schema versions and [`MigrationRegistry`].

use std::collections::{BTreeMap, HashMap};

use crate::error::MigrationError;

/// Semantic version tag embedded in serialized headers.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SchemaVersion {
    /// Major version.
    pub major: u16,
    /// Minor version.
    pub minor: u16,
    /// Patch version.
    pub patch: u16,
}

impl SchemaVersion {
    /// Builds a new schema version literal.
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

/// Minimal value tree used by migrations (RF-4).
#[derive(Clone, Debug, PartialEq)]
pub enum MigrationValue {
    /// Boolean.
    Bool(bool),
    /// Signed integer.
    Int(i64),
    /// Floating point.
    Float(f64),
    /// UTF-8 string.
    String(String),
    /// Ordered list.
    Vec(Vec<MigrationValue>),
    /// Sorted string keys for deterministic merges.
    Map(BTreeMap<String, MigrationValue>),
    /// Opaque entity id placeholder.
    Entity(u64),
    /// Raw bytes.
    Bytes(Vec<u8>),
    /// JSON-null analogue.
    Null,
}

/// One schema-bump migration step on the flat field map.
pub type MigrationFn = fn(&mut BTreeMap<String, MigrationValue>) -> Result<(), MigrationError>;

/// Registry of linear migration edges and per-type current versions.
#[derive(Debug, Default)]
pub struct MigrationRegistry {
    /// `(type_name, from_version) -> (to_version, migration)`.
    edges: HashMap<(String, SchemaVersion), (SchemaVersion, MigrationFn)>,
    current_versions: HashMap<String, SchemaVersion>,
}

impl MigrationRegistry {
    /// Empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register one forward edge `from -> to`.
    pub fn register(
        &mut self,
        type_name: &str,
        from: SchemaVersion,
        to: SchemaVersion,
        migrate: MigrationFn,
    ) {
        self.edges
            .insert((type_name.to_string(), from), (to, migrate));
    }

    /// Declare the newest supported schema for `type_name`.
    pub fn set_current_version(&mut self, type_name: &str, version: SchemaVersion) {
        self.current_versions.insert(type_name.to_string(), version);
    }

    /// Returns the configured current version, if any.
    pub fn current_version(&self, type_name: &str) -> Option<SchemaVersion> {
        self.current_versions.get(type_name).copied()
    }

    /// Resolves an ordered list of migration functions from `from` to the
    /// registered current version.
    pub fn get_chain(
        &self,
        type_name: &str,
        from: SchemaVersion,
    ) -> Result<Vec<MigrationFn>, MigrationError> {
        let target = self
            .current_versions
            .get(type_name)
            .copied()
            .ok_or_else(|| MigrationError::UnknownType(type_name.to_string()))?;
        if from == target {
            return Ok(Vec::new());
        }
        let mut out = Vec::new();
        let mut cur = from;
        while cur != target {
            let (next, step) = self
                .edges
                .get(&(type_name.to_string(), cur))
                .copied()
                .ok_or(MigrationError::MissingStep {
                    type_name: type_name.to_string(),
                    from: cur,
                    to: target,
                })?;
            out.push(step);
            cur = next;
        }
        Ok(out)
    }

    /// Applies [`Self::get_chain`] in order to `value`.
    pub fn migrate(
        &self,
        type_name: &str,
        from: SchemaVersion,
        value: &mut BTreeMap<String, MigrationValue>,
    ) -> Result<(), MigrationError> {
        for step in self.get_chain(type_name, from)? {
            step(value)?;
        }
        Ok(())
    }
}
