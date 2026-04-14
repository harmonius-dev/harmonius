//! ECS-style command buffer for UI prompt spawning (IR-4.9.5).

use crate::ids::Entity;

/// User choice written by UI input (capacity 16 channel in design).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChoiceMadeEvent {
    pub branch_id: u32,
}

/// Queued spawn for a `ChoicePromptEntity` (consumed at frame boundary).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChoicePromptSpawn {
    pub entity: Entity,
}

/// Command buffer drained after simulation sub-steps.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CommandBuffer {
    pending_prompts: Vec<ChoicePromptSpawn>,
    applied_prompts: Vec<Entity>,
}

impl CommandBuffer {
    /// Queues a prompt entity for visibility after apply.
    pub fn spawn_prompt(&mut self, entity: Entity) {
        self.pending_prompts.push(ChoicePromptSpawn { entity });
    }

    /// Drains queued spawns into the post-apply visible set.
    pub fn apply(&mut self) {
        for p in std::mem::take(&mut self.pending_prompts) {
            self.applied_prompts.push(p.entity);
        }
    }

    /// Entities visible to UI queries after `apply`.
    pub fn visible_prompts(&self) -> &[Entity] {
        &self.applied_prompts
    }

    /// Clears visible prompts (test harness reset).
    pub fn clear_visible(&mut self) {
        self.applied_prompts.clear();
    }
}
