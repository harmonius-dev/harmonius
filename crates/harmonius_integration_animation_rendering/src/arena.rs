//! Arena overflow demotion policy for instanced skinning.

use crate::types::GpuArenaBuffer;

/// Editor-tunable secondary capacity inputs for arena overflow (design arena table).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArenaOverflowBudgets {
    /// Slots allowed at `AnimationLodTier::ReducedBones` after primary arena is full.
    pub reduced_bones_capacity: u32,
}

/// Counts how instances are accepted or demoted when an arena is at capacity.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ArenaOverflowOutcome {
    /// Instances kept at their requested tier within arena capacity.
    pub accepted_at_requested_tier: u32,
    /// Instances demoted to `AnimationLodTier::ReducedBones` when the arena is full.
    pub demoted_to_reduced_bones: u32,
    /// Instances demoted to `AnimationLodTier::Vat` when reduced-bone budget is exhausted.
    pub demoted_to_vat: u32,
}

/// Applies arena capacity to a batch of instances at `AnimationLodTier::Full`.
///
/// Excess counts first move to `AnimationLodTier::ReducedBones`, then to `AnimationLodTier::Vat`
/// when the secondary budget is exhausted (test contract).
#[must_use]
pub fn allocate_with_arena_overflow_policy(
    arena: &GpuArenaBuffer,
    requested_instances: u32,
    budgets: ArenaOverflowBudgets,
) -> ArenaOverflowOutcome {
    let mut remaining = requested_instances;

    let primary_room = arena.capacity.saturating_sub(arena.used);
    let take_primary = remaining.min(primary_room);
    let accepted = take_primary;
    remaining -= take_primary;

    if remaining == 0 {
        return ArenaOverflowOutcome {
            accepted_at_requested_tier: accepted,
            demoted_to_reduced_bones: 0,
            demoted_to_vat: 0,
        };
    }

    let take_reduced = remaining.min(budgets.reduced_bones_capacity);
    let reduced = take_reduced;
    remaining -= take_reduced;

    if remaining == 0 {
        return ArenaOverflowOutcome {
            accepted_at_requested_tier: accepted,
            demoted_to_reduced_bones: reduced,
            demoted_to_vat: 0,
        };
    }

    let vat = remaining;

    ArenaOverflowOutcome {
        accepted_at_requested_tier: accepted,
        demoted_to_reduced_bones: reduced,
        demoted_to_vat: vat,
    }
}
