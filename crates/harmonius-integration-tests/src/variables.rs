//! Entity-variable codegen stand-ins (IR-2.8.6).

use crate::context::ExecutionContext;

/// `Health` component backing `"health"` variable reads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Health(pub i32);

/// `Armor` component backing `"armor"` variable reads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Armor(pub i32);

/// `Speed` component backing `"speed"` variable writes.
#[derive(Clone, Debug, PartialEq)]
pub struct Speed(pub f32);

/// Transform slice of `"pose"` mapping.
#[derive(Clone, Debug, PartialEq)]
pub struct Transform(pub f32, pub f32);

/// Animation slice of `"pose"` mapping.
#[derive(Clone, Debug, PartialEq)]
pub struct AnimState(pub i32);

/// Read `"health"` via typed world access (codegen stand-in).
pub fn read_health_var(ctx: &ExecutionContext<'_>) -> Option<i32> {
    ctx.read::<Health>().map(|h| h.0)
}

/// Read `"armor"` via typed world access.
pub fn read_armor_var(ctx: &ExecutionContext<'_>) -> Option<i32> {
    ctx.read::<Armor>().map(|a| a.0)
}

/// Read multi-component `"pose"` variable.
pub fn read_pose_var(ctx: &ExecutionContext<'_>) -> Option<(f32, f32, i32)> {
    let t = ctx.read::<Transform>()?;
    let a = ctx.read::<AnimState>()?;
    Some((t.0, t.1, a.0))
}

/// Write `"speed"` variable (deferred component write).
pub fn write_speed_var(ctx: &mut ExecutionContext<'_>, v: f32) {
    ctx.write(ctx.entity, Speed(v));
}

/// Write `"pose"` atomically if both backing components exist.
///
/// Returns `false` when either component is missing (IR-2.8.6.N2).
pub fn write_pose_var(ctx: &mut ExecutionContext<'_>, x: f32, y: f32, anim: i32) -> bool {
    if ctx.read::<Transform>().is_none() || ctx.read::<AnimState>().is_none() {
        return false;
    }
    ctx.write(ctx.entity, Transform(x, y));
    ctx.write(ctx.entity, AnimState(anim));
    true
}
