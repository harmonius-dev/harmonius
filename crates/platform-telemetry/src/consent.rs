//! First-run consent state (R-14.5.1).

use serde::{Deserialize, Serialize};

use crate::types::AnonId;

/// Persisted opt-in flags and anonymous identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsentState {
    /// Engine-only telemetry scope.
    pub engine_scope: bool,
    /// Gameplay telemetry scope.
    pub game_scope: bool,
    /// Random install identifier.
    pub anonymous_id: AnonId,
    /// Whether the first-run prompt has been completed.
    pub first_run_ack: bool,
}

impl ConsentState {
    /// Default-off scopes for a fresh install.
    pub fn fresh() -> Self {
        Self {
            engine_scope: false,
            game_scope: false,
            anonymous_id: AnonId::random(),
            first_run_ack: false,
        }
    }

    /// True until any consent mutation marks the prompt complete.
    pub fn is_first_run(&self) -> bool {
        !self.first_run_ack
    }

    /// Whether recording is allowed for a scope.
    pub fn consent_for(&self, scope: crate::types::Scope) -> bool {
        match scope {
            crate::types::Scope::Engine => self.engine_scope,
            crate::types::Scope::GameLogic => self.game_scope,
        }
    }
}

/// Serialize consent to disk for persistence tests.
pub fn save_consent(path: &std::path::Path, state: &ConsentState) -> std::io::Result<()> {
    let data = serde_json::to_vec_pretty(state)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
    std::fs::write(path, data)
}

/// Load consent from disk.
pub fn load_consent(path: &std::path::Path) -> std::io::Result<ConsentState> {
    let data = std::fs::read(path)?;
    serde_json::from_slice(&data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Scope;
    use tempfile::tempdir;

    #[test]
    fn test_first_run_defaults_both_scopes_off() {
        let s = ConsentState::fresh();
        assert!(!s.engine_scope);
        assert!(!s.game_scope);
    }

    #[test]
    fn test_first_run_prompt_once_per_install() {
        let mut s = ConsentState::fresh();
        assert!(s.is_first_run());
        s.engine_scope = true;
        s.first_run_ack = true;
        assert!(!s.is_first_run());
    }

    #[test]
    fn test_consent_persists_across_restart() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("consent.json");
        let mut s = ConsentState::fresh();
        s.engine_scope = true;
        s.game_scope = false;
        s.first_run_ack = true;
        save_consent(&path, &s).unwrap();
        let loaded = load_consent(&path).unwrap();
        assert!(loaded.engine_scope);
        assert!(!loaded.game_scope);
        assert_eq!(loaded.anonymous_id, s.anonymous_id);
        assert!(loaded.first_run_ack);
        assert!(loaded.consent_for(Scope::Engine));
        assert!(!loaded.consent_for(Scope::GameLogic));
    }
}
