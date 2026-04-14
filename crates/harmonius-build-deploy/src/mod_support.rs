//! Mod SDK, policy, sandbox, publishing, and workshop sync (R-15.16.*).

/// Allowed asset kinds for a mod SDK (TC-15.16.1.1).
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ModSdkConstraints {
    /// Maximum texture count.
    pub max_textures: u32,
}

/// Validates mod manifest against SDK caps (TC-15.16.1.1).
#[must_use]
pub(crate) fn mod_sdk_allows_textures(constraints: &ModSdkConstraints, textures: u32) -> bool {
    textures <= constraints.max_textures
}

/// Declares mod policy flags (TC-15.16.2.1).
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ModPolicy {
    /// Disallows network IO when true.
    pub disallow_network: bool,
}

/// Validates policy against requested capability (TC-15.16.2.1).
#[must_use]
pub(crate) fn mod_policy_allows_network(policy: &ModPolicy, wants_network: bool) -> bool {
    !(policy.disallow_network && wants_network)
}

/// Sandbox memory budget (TC-15.16.4.1).
#[must_use]
pub(crate) fn mod_memory_budget_ok(used: u64, budget: u64) -> bool {
    used <= budget
}

/// Marks mod active state after budget violation (TC-15.16.4.2).
#[must_use]
pub(crate) fn mod_active_after_violation(violated: bool) -> bool {
    !violated
}

/// Publishes a mod package id (TC-15.16.3.1).
#[must_use]
pub(crate) fn publish_mod_package(name: &str, version: &str) -> String {
    format!("{name}@{version}")
}

/// Workshop sync cursor (TC-15.16.5.1).
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorkshopSync {
    /// Last applied remote revision.
    pub revision: u64,
}

impl WorkshopSync {
    /// Applies a remote revision monotonically.
    pub(crate) fn apply_remote(&mut self, revision: u64) -> bool {
        if revision > self.revision {
            self.revision = revision;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ModPolicy, ModSdkConstraints, WorkshopSync, mod_active_after_violation, mod_memory_budget_ok,
        mod_policy_allows_network, mod_sdk_allows_textures, publish_mod_package,
    };

    /// TC-15.16.1.1 — Mod SDK constraints (R-15.16.1).
    #[test]
    fn tc_15_16_1_1_mod_sdk_constraints() {
        let c = ModSdkConstraints { max_textures: 4 };
        assert!(mod_sdk_allows_textures(&c, 4));
        assert!(!mod_sdk_allows_textures(&c, 5));
    }

    /// TC-15.16.2.1 — Mod policy validation (R-15.16.2).
    #[test]
    fn tc_15_16_2_1_mod_policy() {
        let p = ModPolicy {
            disallow_network: true,
        };
        assert!(!mod_policy_allows_network(&p, true));
    }

    /// TC-15.16.4.1 — Mod sandbox budget check (R-15.16.4).
    #[test]
    fn tc_15_16_4_1_budget_check() {
        assert!(mod_memory_budget_ok(10, 16));
        assert!(!mod_memory_budget_ok(20, 16));
    }

    /// TC-15.16.4.2 — Mod deactivation on exceed (R-15.16.4).
    #[test]
    fn tc_15_16_4_2_deactivate() {
        assert!(!mod_active_after_violation(true));
        assert!(mod_active_after_violation(false));
    }

    /// TC-15.16.3.1 — Mod package publish (R-15.16.3).
    #[test]
    fn tc_15_16_3_1_publish() {
        assert_eq!(publish_mod_package("demo", "1.0.0"), "demo@1.0.0");
    }

    /// TC-15.16.5.1 — Mod workshop sync (R-15.16.5).
    #[test]
    fn tc_15_16_5_1_workshop_sync() {
        let mut w = WorkshopSync { revision: 1 };
        assert!(w.apply_remote(2));
        assert!(!w.apply_remote(2));
    }
}
