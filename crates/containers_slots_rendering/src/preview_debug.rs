//! Snap preview and editor debug payload types (IR-5.8.3, IR-5.8.5).

use crate::visual_override::{AssetHandle, Material, Mesh};
use crate::GlobalTransform;

/// Static string label used for editor debug overlays.
pub type StaticStr = &'static str;

/// 8-bit RGBA color for debug gizmos.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rgba8 {
    /// Red channel.
    pub r: u8,
    /// Green channel.
    pub g: u8,
    /// Blue channel.
    pub b: u8,
    /// Alpha channel.
    pub a: u8,
}

/// Snap point preview data for drag operations (IR-5.8.5).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SnapPreview {
    /// Ghost mesh drawn at the snap target.
    pub ghost_mesh: AssetHandle<Mesh>,
    /// Ghost material for translucent preview drawing.
    pub ghost_material: AssetHandle<Material>,
    /// World transform of the preview instance.
    pub world_transform: GlobalTransform,
    /// Activation radius matching `SnapPoint.snap_radius` in containers-slots.
    pub snap_radius: f32,
}

/// Editor-only debug gizmo payload per socket (IR-5.8.3).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DebugSocketGizmo {
    /// Human-readable socket name for overlay text.
    pub socket_name: StaticStr,
    /// World transform for gizmo placement.
    pub world_transform: GlobalTransform,
    /// Gizmo color in linear or sRGB space per caller convention.
    pub color: Rgba8,
}

/// Runtime toggle for socket debug visualization (IR-5.8.3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DebugConfig {
    /// When true, `DebugSocketGizmo` instances may be extracted for rendering.
    pub sockets_enabled: bool,
}
