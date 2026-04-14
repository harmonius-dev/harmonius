//! Per-instance entity template property overrides (`PLAN-tools-level-world`).
//!
//! Covered test IDs: `TC-15.2.3.1`, `TC-15.2.3.2` in
//! `docs/design/tools/level-world-test-cases.md`.

use std::collections::BTreeMap;

/// Active overrides for string-keyed numeric properties (stand-in for typed ECS paths).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PropertyOverrideMap {
    inner: BTreeMap<String, i32>,
}

/// `apply_override_to_source` was called with no override at `path`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApplyToSourceError {
    /// No entry existed for the requested property path.
    NoActiveOverride,
}

impl PropertyOverrideMap {
    /// Records `value` at `path`, replacing any prior entry.
    pub fn set(&mut self, path: impl Into<String>, value: i32) {
        self.inner.insert(path.into(), value);
    }

    /// Drops an override so the instance tracks the template source again.
    pub fn revert(&mut self, path: &str) {
        self.inner.remove(path);
    }

    /// Returns whether `path` currently overrides the template.
    #[must_use]
    pub fn has_override(&self, path: &str) -> bool {
        self.inner.contains_key(path)
    }
}

/// Resolves the effective `i32` for `path` (`TC-15.2.3.1`).
#[must_use]
pub fn effective_i32(source: i32, path: &str, overrides: &PropertyOverrideMap) -> i32 {
    overrides.inner.get(path).copied().unwrap_or(source)
}

/// Promotes the instance override at `path` onto `template_source` and removes it (`TC-15.2.3.2`).
pub fn apply_override_to_source(
    template_source: &mut i32,
    path: &str,
    instance: &mut PropertyOverrideMap,
) -> Result<(), ApplyToSourceError> {
    let v = instance
        .inner
        .remove(path)
        .ok_or(ApplyToSourceError::NoActiveOverride)?;
    *template_source = v;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const HP: &str = "hp";

    /// TC-15.2.3.1 — set + revert round-trip (`level-world-test-cases.md`).
    #[test]
    fn tc_15_2_3_1_override_set_then_revert_tracks_source() {
        let source_a = 10_i32;
        let mut inst = PropertyOverrideMap::default();
        assert!(!inst.has_override(HP));
        assert_eq!(effective_i32(source_a, HP, &inst), source_a);

        inst.set(HP, 20);
        assert!(inst.has_override(HP));
        assert_eq!(effective_i32(source_a, HP, &inst), 20);

        inst.revert(HP);
        assert!(!inst.has_override(HP));
        assert_eq!(effective_i32(source_a, HP, &inst), source_a);
    }

    /// TC-15.2.3.2 — apply override to template source (`level-world-test-cases.md`).
    #[test]
    fn tc_15_2_3_2_apply_to_source_updates_template_and_non_overridden_instances() {
        let mut template_hp = 100_i32;
        let inst_a = PropertyOverrideMap::default();
        let mut inst_b = PropertyOverrideMap::default();
        inst_b.set(HP, 150);

        assert_eq!(effective_i32(template_hp, HP, &inst_a), 100);
        assert_eq!(effective_i32(template_hp, HP, &inst_b), 150);

        apply_override_to_source(&mut template_hp, HP, &mut inst_b).expect("override present");
        assert_eq!(template_hp, 150);
        assert!(!inst_b.has_override(HP));
        assert_eq!(effective_i32(template_hp, HP, &inst_a), 150);
        assert_eq!(effective_i32(template_hp, HP, &inst_b), 150);
    }

    #[test]
    fn apply_to_source_errors_without_override() {
        let mut template_hp = 100_i32;
        let mut inst = PropertyOverrideMap::default();
        assert_eq!(
            apply_override_to_source(&mut template_hp, HP, &mut inst),
            Err(ApplyToSourceError::NoActiveOverride)
        );
    }
}
