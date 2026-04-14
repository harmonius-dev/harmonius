//! Assistant command interpreter gate aligned with feature toggles.

use super::toggles::FeatureToggleState;

/// Opaque editor user identifier (stub).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UserId(pub u64);

/// Multi-modal context assembled for an LLM request (stub for toggle tests).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultiModalContext {
    /// Placeholder token budget field for future expansion.
    pub token_budget: u32,
}

/// Handle returned when a command is accepted for asynchronous completion (stub).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CommandHandle(pub u64);

/// Assistant-side failures before a network hop is attempted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AssistantError {
    /// Assistant disabled via policy or user toggle (TC-15.7.4.1).
    Disabled,
}

/// Synchronous gate in front of QUIC enqueue (TC-15.7.4.1).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandInterpreter {
    toggles: FeatureToggleState,
    next_handle: u64,
}

impl CommandInterpreter {
    /// Builds an interpreter with the given persisted toggle snapshot.
    pub fn new(toggles: FeatureToggleState) -> Self {
        Self {
            toggles,
            next_handle: 1,
        }
    }

    /// Enforces `assistance_enabled` before issuing a command handle.
    pub fn process_command(
        &mut self,
        _context: MultiModalContext,
        _user: UserId,
    ) -> Result<CommandHandle, AssistantError> {
        if !self.toggles.assistance_enabled {
            return Err(AssistantError::Disabled);
        }
        let handle = CommandHandle(self.next_handle);
        self.next_handle = self.next_handle.saturating_add(1);
        Ok(handle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-15.7.4.1 — assistance toggle gate on `process_command`.
    #[test]
    fn tc_15_7_4_1_assistance_toggle_gate() {
        let mut interpreter = CommandInterpreter::new(FeatureToggleState {
            generative_ai_enabled: true,
            assistance_enabled: false,
            policy_version: 1,
        });
        let result =
            interpreter.process_command(MultiModalContext { token_budget: 1024 }, UserId(42));
        assert_eq!(result, Err(AssistantError::Disabled));
    }
}
