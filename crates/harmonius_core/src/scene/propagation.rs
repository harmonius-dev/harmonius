//! Public entry points for transform propagation.

use super::World;

/// Tick-like token reserved for future ECS integration.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ChangeTick(pub u64);

/// Propagates 3D and 2D transforms for `world`.
pub fn propagate_transforms(world: &mut World) {
    world.run_propagation();
}
