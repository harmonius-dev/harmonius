//! Feature toggle state backing generative AI and assistant gates.

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
}

#[cfg(test)]
mod tests {
    use super::*;

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
