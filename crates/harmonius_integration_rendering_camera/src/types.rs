//! Shared value types for the rendering ↔ camera integration surface.

use glam::{Mat4, Quat, Vec3, Vec4};

/// Stable ECS entity identifier referenced by blend entries and volumes.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Entity(pub u32);

/// Pixel rectangle for a render target region.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Viewport {
    /// Left origin in pixels.
    pub x: u32,
    /// Top origin in pixels.
    pub y: u32,
    /// Width in pixels (after DRS scaling).
    pub width: u32,
    /// Height in pixels (after DRS scaling).
    pub height: u32,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 1280,
            height: 720,
        }
    }
}

/// GPU-facing uniform block for a single view.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewUniform {
    /// World-to-view matrix.
    pub view: Mat4,
    /// Clip-space projection matrix.
    pub projection: Mat4,
    /// Combined projection * view.
    pub view_projection: Mat4,
    /// Homogeneous camera world position.
    pub camera_position: Vec4,
}

/// Per-view data consumed by the render graph.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderView {
    /// Bitmask copied from the emitting camera.
    pub visibility_bits: u32,
    /// World-to-view matrix.
    pub view_matrix: Mat4,
    /// Clip projection matrix.
    pub projection: Mat4,
    /// Target viewport in pixels.
    pub viewport: Viewport,
}

/// Worker-thread snapshot message sent to the render thread.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderViewFromCamera {
    /// Stable tie-break index copied from [`CameraOutput::stable_index`].
    pub stable_index: u32,
    /// World-to-view matrix.
    pub view_matrix: Mat4,
    /// Clip projection matrix.
    pub projection: Mat4,
    /// Combined projection * view.
    pub view_projection: Mat4,
    /// Camera world position.
    pub camera_position: Vec3,
    /// Near clip distance in world units.
    pub near_clip: f32,
    /// Far clip distance in world units.
    pub far_clip: f32,
    /// Render-layer bitmask after substitution rules.
    pub render_layers: u32,
    /// Signed draw order; lower renders first.
    pub render_order: i32,
    /// Pixel viewport for this view.
    pub viewport: Viewport,
}

/// Projection mode carried on [`CameraOutput`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Projection {
    /// Perspective with vertical FOV (radians), reverse-Z clip.
    Perspective {
        /// Vertical field of view in radians.
        fov_y_radians: f32,
        /// Width divided by height.
        aspect: f32,
        /// Positive distance to the near plane.
        near: f32,
        /// Positive distance to the far plane (clipping only).
        far: f32,
    },
    /// Symmetric orthographic projection.
    Orthographic {
        /// Positive half-extent on Y in world units.
        half_height: f32,
        /// Width divided by height.
        aspect: f32,
        /// Near clip plane (may be negative).
        near: f32,
        /// Far clip plane (may be negative).
        far: f32,
    },
}

/// Final camera parameters after brain evaluation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CameraOutput {
    /// Owning camera brain entity (used for diagnostics keys).
    pub brain: Entity,
    /// Stable insertion index for deterministic tie-breaks.
    pub stable_index: u32,
    /// When false, this brain is skipped by the extractor.
    pub active: bool,
    /// Camera world position.
    pub position: Vec3,
    /// Camera world orientation.
    pub rotation: Quat,
    /// Lens projection.
    pub projection: Projection,
    /// Raw render-layer bitmask (zero triggers substitution).
    pub render_layers: u32,
    /// Draw order for multi-view sorting.
    pub render_order: i32,
    /// Full-resolution viewport before DRS scaling.
    pub viewport: Viewport,
}

/// Blend entry referencing a post-process volume entity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PostProcessBlend {
    /// Volume entity referenced by this stack entry.
    pub volume: Entity,
    /// Blend weight in `[0, 1]` (clamped on write in resolver).
    pub weight: f32,
    /// Priority carried on the volume; higher wins ties.
    pub priority: i16,
}

/// Resolved post-process parameters for a camera.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PostProcessParams {
    /// Linear exposure multiplier.
    pub exposure: f32,
    /// Linear contrast multiplier.
    pub contrast: f32,
    /// Linear saturation multiplier.
    pub saturation: f32,
    /// Vignette strength in `[0, 1]`.
    pub vignette_intensity: f32,
    /// Bloom threshold in linear units.
    pub bloom_threshold: f32,
    /// Bloom intensity multiplier.
    pub bloom_intensity: f32,
}

impl Default for PostProcessParams {
    fn default() -> Self {
        Self {
            exposure: 1.0,
            contrast: 1.0,
            saturation: 1.0,
            vignette_intensity: 0.0,
            bloom_threshold: 1.0,
            bloom_intensity: 0.0,
        }
    }
}

/// Ordered post-process stack evaluated on workers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PostProcessStack {
    /// Entries requested for this camera after dead-entity filtering.
    pub entries: Vec<PostProcessBlend>,
    /// Blended parameters for GPU consumption.
    pub resolved: PostProcessParams,
}

/// DRS feedback message from render to workers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DynamicResolutionState {
    /// Current scale in `[min_scale, 1.0]`.
    pub scale: f32,
    /// Lower clamp for scale.
    pub min_scale: f32,
    /// Smoothed frame time in milliseconds.
    pub frame_ms: f32,
}

/// Axis-aligned volume used for post-process containment tests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PostProcessVolume {
    /// Owning entity id.
    pub entity: Entity,
    /// Minimum corner (inclusive on each axis).
    pub min: Vec3,
    /// Maximum corner (exclusive on each axis).
    pub max: Vec3,
    /// Volume priority used when resolving overlaps.
    pub priority: i16,
    /// Parameters authored for this volume.
    pub params: PostProcessParams,
}
