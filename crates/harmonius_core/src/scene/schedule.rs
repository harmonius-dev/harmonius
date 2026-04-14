//! Minimal deterministic system schedule for ordering tests.

use super::{propagate_transforms, SceneError, World};

/// Identifiers for built-in systems used in ordering tests (`TC-1.1.28.1`).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SystemId {
    /// Applies deferred hierarchy commands.
    HierarchyApply,
    /// Propagates transforms after hierarchy is stable.
    PropagateTransforms,
}

/// Fixed-function schedule runner (no reflection).
#[derive(Debug, Default)]
pub struct Schedule {
    /// Last execution order for diagnostics.
    pub last_run_order: Vec<SystemId>,
}

impl Schedule {
    /// Runs the provided systems in order for one frame.
    pub fn run_frame(&mut self, world: &mut World, order: &[SystemId]) -> Result<(), SceneError> {
        self.last_run_order.clear();
        for system in order {
            match system {
                SystemId::HierarchyApply => {
                    world.flush_hierarchy_commands()?;
                }
                SystemId::PropagateTransforms => propagate_transforms(world),
            }
            self.last_run_order.push(*system);
        }
        Ok(())
    }
}
