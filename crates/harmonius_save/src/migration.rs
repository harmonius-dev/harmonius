//! Ordered schema migrations without `HashMap` on hot paths (R-13.3.2).

use crate::error::MigrationError;
use crate::types::SchemaVersion;

/// One codegen-backed migration edge between two schema versions for a component type.
#[derive(Clone, Copy)]
pub struct MigrationStep {
    pub type_hash: u64,
    pub from: SchemaVersion,
    pub to: SchemaVersion,
    pub func: fn(&[u8]) -> Result<Vec<u8>, MigrationError>,
}

impl std::fmt::Debug for MigrationStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MigrationStep")
            .field("type_hash", &self.type_hash)
            .field("from", &self.from)
            .field("to", &self.to)
            .field("func", &"<fn>")
            .finish()
    }
}

/// Registry of migration steps sorted for binary search (TC-13.3.2.*).
#[derive(Debug, Default)]
pub struct MigrationRegistry {
    steps: Vec<MigrationStep>,
}

fn step_key(s: &MigrationStep) -> (u64, SchemaVersion, SchemaVersion) {
    (s.type_hash, s.from, s.to)
}

impl MigrationRegistry {
    /// Empty registry.
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    /// Insert a migration step, keeping `steps` sorted by `(type_hash, from, to)`.
    pub fn register(&mut self, step: MigrationStep) -> Result<(), MigrationError> {
        let key = step_key(&step);
        if self.steps.binary_search_by_key(&key, step_key).is_ok() {
            return Err(MigrationError::InvalidOrder {
                expected: step.from,
                got: step.to,
            });
        }
        let pos = self.steps.binary_search_by_key(&key, step_key).unwrap_err();
        self.steps.insert(pos, step);
        Ok(())
    }

    /// True when `saved` differs from `current` for any plausible migration work.
    pub fn needs_migration(&self, saved: SchemaVersion, current: SchemaVersion) -> bool {
        let _ = self;
        saved != current
    }

    /// Ordered chain of steps from `saved` to `current` for `type_hash`.
    pub fn migration_chain(
        &self,
        type_hash: u64,
        saved: SchemaVersion,
        current: SchemaVersion,
    ) -> Vec<MigrationStep> {
        if saved == current {
            return Vec::new();
        }
        let mut chain = Vec::new();
        let mut cursor = saved;
        while cursor != current {
            let candidates: Vec<&MigrationStep> = self
                .steps
                .iter()
                .filter(|s| s.type_hash == type_hash && s.from == cursor)
                .collect();
            let Some(step) = candidates
                .into_iter()
                .min_by_key(|s| (s.to.major, s.to.minor, s.to.patch))
            else {
                break;
            };
            chain.push(*step);
            cursor = step.to;
        }
        chain
    }

    /// Apply migrations from `saved` to `current`. The input slice is never mutated (TC-13.3.2.2).
    pub fn migrate(
        &self,
        data: &[u8],
        type_hash: u64,
        saved: SchemaVersion,
        current: SchemaVersion,
    ) -> Result<Vec<u8>, MigrationError> {
        if saved == current {
            return Ok(data.to_vec());
        }
        let chain = self.migration_chain(type_hash, saved, current);
        if chain.is_empty() {
            return Err(MigrationError::NoPath {
                type_hash,
                from: saved,
                to: current,
            });
        }
        if chain.first().expect("non-empty").from != saved {
            return Err(MigrationError::NoPath {
                type_hash,
                from: saved,
                to: current,
            });
        }
        if chain.last().expect("non-empty").to != current {
            return Err(MigrationError::NoPath {
                type_hash,
                from: saved,
                to: current,
            });
        }
        for w in chain.windows(2) {
            if w[0].to != w[1].from {
                return Err(MigrationError::InvalidOrder {
                    expected: w[0].to,
                    got: w[1].from,
                });
            }
        }
        let mut working = data.to_vec();
        for step in &chain {
            working = match (step.func)(&working) {
                Ok(next) => next,
                Err(e) => {
                    return Err(MigrationError::StepFailed {
                        type_hash,
                        step_from: step.from,
                        step_to: step.to,
                        detail: e.to_string(),
                    });
                }
            };
        }
        Ok(working)
    }
}

/// Helper for tests: identity step.
pub fn identity_step(type_hash: u64, from: SchemaVersion, to: SchemaVersion) -> MigrationStep {
    MigrationStep {
        type_hash,
        from,
        to,
        func: |b| Ok(b.to_vec()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(m: u16, n: u16, p: u16) -> SchemaVersion {
        SchemaVersion {
            major: m,
            minor: n,
            patch: p,
        }
    }

    /// TC-13.3.2.1 Migration v1 to v3
    #[test]
    fn tc_13_3_2_1_chain_v1_to_v3() {
        let mut reg = MigrationRegistry::new();
        let th = 0xC0FFEE_u64;
        reg.register(identity_step(th, v(1, 0, 0), v(2, 0, 0)))
            .unwrap();
        reg.register(identity_step(th, v(2, 0, 0), v(3, 0, 0)))
            .unwrap();
        let chain = reg.migration_chain(th, v(1, 0, 0), v(3, 0, 0));
        assert_eq!(chain.len(), 2);
        assert_eq!(chain[0].from, v(1, 0, 0));
        assert_eq!(chain[0].to, v(2, 0, 0));
        assert_eq!(chain[1].from, v(2, 0, 0));
        assert_eq!(chain[1].to, v(3, 0, 0));
        let out = reg
            .migrate(b"payload", th, v(1, 0, 0), v(3, 0, 0))
            .expect("migrate");
        assert_eq!(out, b"payload");
    }

    /// TC-13.3.2.2 Migration failure preserves the input buffer.
    #[test]
    fn tc_13_3_2_2_step_failed_returns_error() {
        fn ok_v1_v2(_: &[u8]) -> Result<Vec<u8>, MigrationError> {
            Ok(vec![1])
        }

        fn fail_v2_v3(_: &[u8]) -> Result<Vec<u8>, MigrationError> {
            Err(MigrationError::StepFailed {
                type_hash: 1,
                step_from: v(2, 0, 0),
                step_to: v(3, 0, 0),
                detail: "boom".into(),
            })
        }

        let mut reg = MigrationRegistry::new();
        let th = 1_u64;
        reg.register(MigrationStep {
            type_hash: th,
            from: v(1, 0, 0),
            to: v(2, 0, 0),
            func: ok_v1_v2,
        })
        .unwrap();
        reg.register(MigrationStep {
            type_hash: th,
            from: v(2, 0, 0),
            to: v(3, 0, 0),
            func: fail_v2_v3,
        })
        .unwrap();
        let input: &[u8] = b"orig";
        let res = reg.migrate(input, th, v(1, 0, 0), v(3, 0, 0));
        assert!(res.is_err());
        assert_eq!(input, b"orig");
    }

    /// TC-13.3.2.3 Migration no path
    #[test]
    fn tc_13_3_2_3_no_path() {
        let reg = MigrationRegistry::new();
        let err = reg
            .migrate(b"x", 9, v(1, 0, 0), v(2, 0, 0))
            .expect_err("expected err");
        assert!(matches!(err, MigrationError::NoPath { .. }));
    }

    /// TC-13.3.2.5 Data loss warnings are representable while migrations complete in upper layers.
    #[test]
    fn tc_13_3_2_5_data_loss_warning_variant() {
        let w = MigrationError::DataLossWarning {
            type_hash: 7,
            field: "hp",
        };
        assert!(format!("{w}").contains("data loss"));
    }

    /// TC-13.3.2.7 Split component migration via custom func.
    #[test]
    fn tc_13_3_2_7_split_component() {
        let mut reg = MigrationRegistry::new();
        let old_hash = 100_u64;
        reg.register(MigrationStep {
            type_hash: old_hash,
            from: v(1, 0, 0),
            to: v(2, 0, 0),
            func: |_| Ok(vec![10, 20]),
        })
        .unwrap();
        let out = reg
            .migrate(b"legacy", old_hash, v(1, 0, 0), v(2, 0, 0))
            .expect("migrate");
        assert_eq!(out, vec![10, 20]);
    }
}
