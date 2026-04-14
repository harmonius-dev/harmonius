//! Migration registry for save payload bytes (IR-5.10.3).

use crate::error::LoadError;
use crate::types::SchemaVersion;

/// One migration step applied to opaque payload bytes.
#[derive(Clone, Debug)]
pub struct MigrationStep {
    /// Type hash this step applies to (integration design field).
    pub type_hash: u64,
    /// Source schema version.
    pub from: SchemaVersion,
    /// Target schema version after this step.
    pub to: SchemaVersion,
    /// Applies the migration to `input`.
    pub apply: fn(&[u8]) -> Result<Vec<u8>, &'static str>,
}

/// Registry of ordered migration steps.
#[derive(Clone, Debug, Default)]
pub struct MigrationRegistry {
    steps: Vec<MigrationStep>,
}

impl MigrationRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    /// Registers a migration step (append-only).
    pub fn register(&mut self, step: MigrationStep) {
        self.steps.push(step);
    }

    /// Returns whether a migration chain should run before deserialization.
    pub fn needs_migration(&self, from: SchemaVersion, to: SchemaVersion) -> bool {
        from != to
    }

    /// Applies all registered steps in registration order.
    pub fn migrate_all(&self, mut data: Vec<u8>) -> Result<Vec<u8>, LoadError> {
        for (idx, step) in self.steps.iter().enumerate() {
            data = (step.apply)(&data).map_err(|detail| LoadError::MigrationFailed {
                step_index: u32::try_from(idx).unwrap_or(u32::MAX),
                type_hash: step.type_hash,
                detail: detail.into(),
            })?;
        }
        Ok(data)
    }
}
