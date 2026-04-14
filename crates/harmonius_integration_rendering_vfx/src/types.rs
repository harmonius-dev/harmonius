//! Interface types from `docs/design/integration/rendering-vfx.md`.
#![allow(missing_docs)]

use glam::Vec3;

/// Generational handle to a GPU buffer region.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GpuBufferView {
    /// Slot index in `GpuBufferRegistry`.
    pub id: u32,
    /// ABA protection generation.
    pub generation: u32,
}

/// Axis-aligned bounds matching `harmonius_math::Aabb` layout (`#[repr(C)]`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb {
    /// Minimum corner.
    pub min: Vec3,
    /// Maximum corner.
    pub max: Vec3,
}

/// Particle sort key.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SortKey {
    /// Do not sort; opaque particles.
    None,
    /// Back-to-front by camera distance (alpha).
    BackToFront,
    /// Front-to-back (early-z opaque).
    FrontToBack,
    /// Sort by material id (batching).
    Material,
}

/// Transparency blend mode for particles and screen effects.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlendMode {
    /// Opaque surface.
    Opaque,
    /// Standard alpha blend.
    AlphaBlend,
    /// Additive blend.
    Additive,
    /// Premultiplied alpha.
    PremultipliedAlpha,
    /// Multiply blend.
    Multiply,
}

/// Render mode for particle geometry.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RenderMode {
    /// Billboard sprite.
    Sprite,
    /// Ribbon trail strip.
    Ribbon,
    /// Mesh particle.
    Mesh,
}

/// Screen effect kind dispatched as post pass.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScreenEffectKind {
    /// Heat distortion.
    HeatHaze,
    /// Radial shockwave distortion.
    Shockwave,
    /// Damage vignette-style overlay.
    DamageOverlay,
    /// Full-screen flash.
    ScreenFlash,
}

/// LOD tier for an emitter. Hysteresis prevents oscillation at tier boundaries.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LodTier {
    /// Full quality.
    Full,
    /// Reduced cost.
    Reduced,
    /// Impostor representation.
    Impostor,
    /// Fully culled.
    Culled,
}

/// ECS component: per-emitter LOD state.
#[derive(Clone, Debug, PartialEq)]
pub struct EmitterLodComponent {
    /// Distance at which `LodTier::Full` applies when approaching.
    pub full_distance: f32,
    /// Distance at which `LodTier::Reduced` applies.
    pub reduced_distance: f32,
    /// Distance at which `LodTier::Impostor` applies.
    pub impostor_distance: f32,
    /// Distance beyond which the emitter is culled.
    pub cull_distance: f32,
    /// Hysteresis band as a fraction of each tier boundary (e.g. `0.2` = 20%).
    pub hysteresis_pct: f32,
    /// Current resolved tier.
    pub current_tier: LodTier,
}

/// Per-frame global VFX budget caps. Scales spawn rate when exceeded.
#[derive(Clone, Debug, PartialEq)]
pub struct EffectBudget {
    /// Maximum simulated particles.
    pub max_live_particles: u32,
    /// Maximum injected particle lights.
    pub max_particle_lights: u32,
    /// Maximum decals resident in atlas.
    pub max_decals: u32,
    /// Maximum concurrent screen-space effects.
    pub max_screen_effects: u32,
    /// Multiplier applied to spawn rate when over budget (0.0, 1.0].
    pub spawn_rate_scale: f32,
}

/// Particle render pass registration. Transient per-frame descriptor.
#[derive(Clone, Debug, PartialEq)]
pub struct ParticleRenderPassDesc {
    /// Simulated particle GPU buffer.
    pub particle_buffer: GpuBufferView,
    /// Alive index list buffer.
    pub alive_list: GpuBufferView,
    /// Indirect draw arguments buffer.
    pub indirect_args: GpuBufferView,
    /// Geometry mode.
    pub render_mode: RenderMode,
    /// Sorting policy.
    pub sort_key: SortKey,
    /// Blend mode for the draw pass.
    pub blend_mode: BlendMode,
}

/// Froxel injection descriptor from VFX volume sources.
#[derive(Clone, Debug, PartialEq)]
pub struct FroxelInjection {
    /// Base density coefficient.
    pub density: f32,
    /// Scattering color.
    pub scattering: Vec3,
    /// Absorption color.
    pub absorption: Vec3,
    /// World-space bounds for this injection.
    pub world_aabb: Aabb,
}

/// Single deferred decal entry.
#[derive(Clone, Debug, PartialEq)]
pub struct DecalEntry {
    /// Higher values draw on top after stable sort.
    pub priority: u32,
    /// Albedo tint written into the G-buffer.
    pub albedo: Vec3,
    /// World-space surface slope angle in degrees (0 = flat floor).
    pub surface_slope_deg: f32,
}

/// Decal pass descriptor; transient per frame.
#[derive(Clone, Debug, PartialEq)]
pub struct DecalPassDesc {
    /// Decals sorted by ascending priority (lower first), then stable order.
    pub decals: Vec<DecalEntry>,
    /// Atlas indirection buffer view.
    pub atlas_view: GpuBufferView,
}

/// Screen-space effect pass descriptor.
#[derive(Clone, Debug, PartialEq)]
pub struct ScreenEffectPassDesc {
    /// Effect kind.
    pub effect: ScreenEffectKind,
    /// Packed scalar parameters (effect-specific).
    pub params: [f32; 8],
    /// Blend mode for the post pass.
    pub blend_mode: BlendMode,
}

/// Authoring-time VFX graph node kinds used by [`crate::compiler::VfxGraphCompiler`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VfxNodeKind {
    /// GPU particle simulation + draw path.
    ParticleSim,
    /// Deferred decal pass.
    Decal,
    /// Volumetric froxel injection.
    FroxelVolume,
    /// Post-process screen effect.
    ScreenEffect,
}

/// One node in a [`VfxGraph`].
#[derive(Clone, Debug, PartialEq)]
pub struct VfxNode {
    /// Node classification.
    pub kind: VfxNodeKind,
}

/// Directed edge in [`VfxGraph`] (ordering only for future expansion).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VfxEdge {
    /// Source node index.
    pub from: u32,
    /// Destination node index.
    pub to: u32,
}

/// VFX graph — declarative composable effect (CPU mirror; archive format is out of crate scope).
#[derive(Clone, Debug, PartialEq)]
pub struct VfxGraph {
    /// Nodes in authoring order.
    pub nodes: Vec<VfxNode>,
    /// Dependency edges.
    pub edges: Vec<VfxEdge>,
}

/// Emissive particle light candidate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParticleLight {
    /// Light intensity used for brightest selection.
    pub intensity: f32,
    /// Stable ordinal for deterministic tie-breaks.
    pub ordinal: u32,
}

/// GPU vs graphics queue selection after fence timeout recovery.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DispatchQueue {
    /// Hardware async compute queue.
    AsyncCompute,
    /// Graphics queue fallback.
    Graphics,
}

/// Mobile volumetric capability set.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MobileVolumetricCaps {
    /// True when 3D froxel texture path is available.
    pub gpu_froxel_supported: bool,
    /// True when async compute queue exists.
    pub async_compute_supported: bool,
}
