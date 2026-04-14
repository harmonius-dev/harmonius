//! Minimal graph scheduler for registration and hot reload.

use std::collections::HashMap;

use crate::graph::{
    AssetId, GraphProgram, GraphRegistrationError, GraphSchedulePhase, HotReloadError,
};

/// Owns registered `GraphProgram` assets for one integration harness.
#[derive(Default)]
pub struct GraphScheduler {
    programs: HashMap<AssetId, GraphProgram>,
    staged_access: Option<(AssetId, crate::access::GraphAccessDescriptor)>,
}

impl GraphScheduler {
    /// Empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a program for `target_phase` (IR-2.8.4.3 phase binding).
    pub fn register(
        &mut self,
        target_phase: GraphSchedulePhase,
        program: GraphProgram,
    ) -> Result<(), GraphRegistrationError> {
        if program.phase != target_phase {
            return Err(GraphRegistrationError::PhaseMismatch {
                program_phase: program.phase,
                target_phase,
            });
        }
        self.programs.insert(program.id, program);
        Ok(())
    }

    /// Borrow a program by id.
    pub fn program(&self, id: AssetId) -> Option<&GraphProgram> {
        self.programs.get(&id)
    }

    /// Mutable program for tests.
    pub fn program_mut(&mut self, id: AssetId) -> Option<&mut GraphProgram> {
        self.programs.get_mut(&id)
    }

    /// Stage access metadata replacement (IR-2.8.5.2).
    pub fn stage_access_reload(&mut self, id: AssetId, access: crate::access::GraphAccessDescriptor) {
        self.staged_access = Some((id, access));
    }

    /// Apply staged access reload atomically (phase boundary).
    pub fn advance_phase_boundary(&mut self) {
        if let Some((id, acc)) = self.staged_access.take() {
            if let Some(p) = self.programs.get_mut(&id) {
                p.access = acc;
            }
        }
    }

    /// Hot-reload `.dylib` metadata (IR-2.8.5.N1 / N2).
    pub fn reload_dylib(
        &mut self,
        id: AssetId,
        offered: GraphProgram,
        live_layout_hash: u64,
    ) -> Result<(), HotReloadError> {
        if offered.state_layout_hash != live_layout_hash {
            return Err(HotReloadError::LayoutMismatch {
                expected: live_layout_hash,
                offered: offered.state_layout_hash,
            });
        }
        self.programs.insert(id, offered);
        Ok(())
    }
}
