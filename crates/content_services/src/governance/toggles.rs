//! Feature toggle state backing generative AI and assistant gates.

use super::error::GovernanceError;
use super::policy::PolicyDocument;

/// Persisted AI governance flags from enterprise policy documents.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeatureToggleState {
    /// When false, generative pipelines must not run.
    pub generative_ai_enabled: bool,
    /// When false, assistant command submission is rejected at the gate.
    pub assistance_enabled: bool,
    /// Monotonic policy generation counter (stub for later signature wiring).
    pub policy_version: u64,
}

/// Runtime view of feature toggles used by editor systems.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeatureToggles {
    state: FeatureToggleState,
}

impl FeatureToggles {
    /// Wraps explicit toggle state (tests construct this directly).
    pub fn new(state: FeatureToggleState) -> Self {
        Self { state }
    }

    /// Returns whether generative AI is permitted (TC-15.7.3.1).
    pub fn is_generative_enabled(&self) -> bool {
        self.state.generative_ai_enabled
    }

    /// Returns whether the assistant may accept commands (TC-15.7.4.1).
    pub fn is_assistance_enabled(&self) -> bool {
        self.state.assistance_enabled
    }

    /// Verifies `doc` and merges flags when the version advances monotonically.
    pub fn apply_policy(
        &mut self,
        doc: &PolicyDocument,
        secret: &[u8; 8],
    ) -> Result<(), GovernanceError> {
        if doc.version < self.state.policy_version {
            return Err(GovernanceError::StalePolicy);
        }
        doc.verify(secret)?;
        let gen = doc.payload.first().copied().unwrap_or(0) != 0;
        let assist = doc.payload.get(1).copied().unwrap_or(0) != 0;
        self.state.generative_ai_enabled = gen;
        self.state.assistance_enabled = assist;
        self.state.policy_version = doc.version;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::policy::PolicyDocument;
    use super::*;

    const SECRET: &[u8; 8] = b"unit-sek";

    /// TC-15.7.5.1 — tampered policy bytes fail signature verification.
    #[test]
    fn tc_15_7_5_1_policy_signature_verify() {
        let mut doc = PolicyDocument::sign(SECRET, 4, true, true);
        doc.payload[0] ^= 0x01;
        let mut toggles = FeatureToggles::new(FeatureToggleState {
            generative_ai_enabled: false,
            assistance_enabled: false,
            policy_version: 0,
        });
        let result = toggles.apply_policy(&doc, SECRET);
        assert_eq!(result, Err(GovernanceError::InvalidSignature));
    }

    /// TC-15.7.5.2 — older policy versions are rejected after a newer apply.
    #[test]
    fn tc_15_7_5_2_policy_version_monotonic() {
        let mut toggles = FeatureToggles::new(FeatureToggleState {
            generative_ai_enabled: false,
            assistance_enabled: false,
            policy_version: 0,
        });
        let v5 = PolicyDocument::sign(SECRET, 5, true, true);
        toggles.apply_policy(&v5, SECRET).unwrap();
        let v3 = PolicyDocument::sign(SECRET, 3, false, false);
        let result = toggles.apply_policy(&v3, SECRET);
        assert_eq!(result, Err(GovernanceError::StalePolicy));
    }

    /// TC-15.7.3.1 — generative AI toggle gate.
    #[test]
    fn tc_15_7_3_1_generative_toggle_gate() {
        let toggles = FeatureToggles::new(FeatureToggleState {
            generative_ai_enabled: false,
            assistance_enabled: true,
            policy_version: 1,
        });
        assert!(!toggles.is_generative_enabled());
    }
}
