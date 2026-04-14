//! Lazy migration on first access (TC-13.3.2.8).

use crate::error::MigrationError;
use crate::migration::MigrationRegistry;
use crate::types::SchemaVersion;

/// Holds unmigrated bytes until first access triggers work.
#[derive(Clone, Debug)]
pub struct LazyEntityBytes {
    type_hash: u64,
    saved_version: SchemaVersion,
    current_version: SchemaVersion,
    raw: Vec<u8>,
    migrated: Option<Vec<u8>>,
}

impl LazyEntityBytes {
    /// Construct lazily-migrated storage.
    pub fn new(
        type_hash: u64,
        saved_version: SchemaVersion,
        current_version: SchemaVersion,
        raw: Vec<u8>,
    ) -> Self {
        Self {
            type_hash,
            saved_version,
            current_version,
            raw,
            migrated: None,
        }
    }

    /// Returns migrated bytes, running the migration chain at most once.
    pub fn materialize(&mut self, registry: &MigrationRegistry) -> Result<&[u8], MigrationError> {
        if self.migrated.is_none() {
            let out = registry.migrate(
                &self.raw,
                self.type_hash,
                self.saved_version,
                self.current_version,
            )?;
            self.migrated = Some(out);
        }
        Ok(self.migrated.as_ref().expect("just set"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::migration::identity_step;

    /// TC-13.3.2.8 Only touched entities pay migration cost.
    #[test]
    fn tc_13_3_2_8_lazy_migration_on_access() {
        let mut reg = MigrationRegistry::new();
        let th = 5u64;
        let v1 = SchemaVersion {
            major: 1,
            minor: 0,
            patch: 0,
        };
        let v2 = SchemaVersion {
            major: 2,
            minor: 0,
            patch: 0,
        };
        reg.register(identity_step(th, v1, v2)).unwrap();
        let mut entities: Vec<LazyEntityBytes> = (0..10_000)
            .map(|_| LazyEntityBytes::new(th, v1, v2, vec![1]))
            .collect();
        for e in entities.iter_mut().take(5) {
            e.materialize(&reg).unwrap();
        }
        assert_eq!(entities[0].migrated.is_some(), true);
        assert_eq!(entities[5000].migrated.is_none(), true);
    }
}
