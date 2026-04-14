//! Snapshot and worker-local types for the editor rendering integration.

use rkyv::{Archive, Deserialize, Serialize};
use smallvec::SmallVec;

/// Opaque view identity (generational).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Archive, Serialize, Deserialize)]
pub struct ViewId {
    /// Stable index within the view registry.
    pub index: u32,
    /// Generation incremented when an index is recycled.
    pub generation: u32,
}

/// Generational entity identifier used by the editor integration.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Archive, Serialize, Deserialize)]
pub struct Entity {
    /// Stable index within the entity storage.
    pub index: u32,
    /// Generation incremented when an index is recycled.
    pub generation: u32,
}

/// Generational handle to a render target resource.
///
/// This is the concrete form of the design’s `Handle<RenderTarget>`: it carries
/// only indices and is safe to snapshot across threads.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Archive, Serialize, Deserialize)]
pub struct RenderTargetHandle {
    /// Arena index for the render target.
    pub index: u32,
    /// Generation incremented when an index is recycled.
    pub generation: u32,
}

/// Marker type used only in type signatures for documentation parity with the
/// integration design (`Handle<RenderTarget>`).
#[derive(Debug)]
pub enum RenderTarget {}

/// Linear RGBA color in linear light space.
#[derive(Copy, Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
pub struct LinearColor {
    /// Red channel.
    pub r: f32,
    /// Green channel.
    pub g: f32,
    /// Blue channel.
    pub b: f32,
    /// Alpha channel.
    pub a: f32,
}

/// Three-component vector used by gizmo shapes in snapshots.
#[derive(Copy, Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
pub struct Vec3 {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
    /// Z component.
    pub z: f32,
}

/// Column-major 4×4 matrix (`cols[j][i]` is row *i*, column *j*).
#[derive(Copy, Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
pub struct Mat4 {
    /// Matrix columns.
    pub cols: [[f32; 4]; 4],
}

/// Render layer bitmask. Bit *N* toggles layer *N* for a view or draw command.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Archive, Serialize, Deserialize)]
pub struct RenderLayers {
    /// Bitmask of active layers.
    pub mask: u32,
}

impl RenderLayers {
    /// Default gameplay draw layers.
    pub const GAMEPLAY: Self = Self { mask: 0x0000_00FF };
    /// Editor gizmo overlay layer.
    pub const EDITOR_GIZMOS: Self = Self { mask: 0x0000_0100 };
    /// Editor infinite grid layer.
    pub const EDITOR_GRID: Self = Self { mask: 0x0000_0200 };
    /// Editor selection outline layer.
    pub const EDITOR_OUTLINE: Self = Self { mask: 0x0000_0400 };
}

/// Render path chosen per viewport.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Archive, Serialize, Deserialize)]
pub enum RenderPath {
    /// Forward shading path.
    Forward,
    /// Deferred shading path.
    Deferred,
    /// Wireframe visualization path.
    Wireframe,
}

/// Debug buffer visualization modes.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Archive, Serialize, Deserialize)]
pub enum BufferVisMode {
    /// Albedo channel visualization.
    Albedo,
    /// World-space normals as false color.
    WorldNormals,
    /// Roughness channel visualization.
    Roughness,
    /// Metallic channel visualization.
    Metallic,
    /// Ambient occlusion visualization.
    AmbientOcclusion,
    /// Wireframe overlay mode.
    Wireframe,
    /// Overdraw heatmap mode.
    Overdraw,
    /// Meshlet identifier false color.
    MeshletId,
    /// Discrete LOD level visualization.
    LodLevel,
    /// UV checkerboard visualization.
    UvChecker,
}

/// Extensible gizmo type identifier issued by the middleman `.dylib` registry.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Archive, Serialize, Deserialize)]
pub struct GizmoTypeId {
    /// Stable index within the gizmo type registry.
    pub index: u32,
}

/// Primitive shapes used by editor gizmo draw commands.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
pub enum GizmoShape {
    /// Axis-aligned arrow primitive.
    Arrow {
        /// Arrow axis direction.
        axis: Vec3,
        /// Arrow length in world units.
        length: f32,
    },
    /// Ring primitive around an axis.
    Ring {
        /// Ring axis direction.
        axis: Vec3,
        /// Ring radius in world units.
        radius: f32,
    },
    /// Axis-aligned cube primitive.
    Cube {
        /// Half extents along each axis.
        half_extents: Vec3,
    },
    /// Sphere primitive.
    Sphere {
        /// Sphere radius in world units.
        radius: f32,
    },
    /// Line primitive between two points.
    Line {
        /// Line start in world space.
        start: Vec3,
        /// Line end in world space.
        end: Vec3,
    },
    /// Custom gizmo contributed by a plugin.
    Custom(GizmoTypeId),
}

/// Single gizmo draw command queued on the worker thread.
#[derive(Copy, Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
pub struct GizmoDrawCommand {
    /// Primitive to draw.
    pub shape: GizmoShape,
    /// Instance transform mapping primitive space to world space.
    pub transform: Mat4,
    /// Color and opacity.
    pub color: LinearColor,
    /// Whether depth testing is enabled for this command.
    pub depth_test: bool,
}

/// Immutable camera snapshot produced during Phase 7.
#[derive(Copy, Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
pub struct CameraSnapshot {
    /// World-to-camera view matrix.
    pub view: Mat4,
    /// Camera projection matrix.
    pub projection: Mat4,
    /// Near clip plane distance.
    pub near: f32,
    /// Far clip plane distance.
    pub far: f32,
}

/// Render view produced by the rendering core for a single viewport.
#[derive(Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
pub struct RenderView {
    /// View identity for correlation with editor panels.
    pub view_id: ViewId,
    /// Sampled camera parameters.
    pub camera: CameraSnapshot,
    /// Render target handle for the viewport swapchain or offscreen target.
    pub target: RenderTargetHandle,
}

/// Scene proxy entry snapshotted from worker to render thread.
#[derive(Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
pub struct RenderProxy {
    /// Owning entity for correlation with ECS selections.
    pub entity: Entity,
}

/// Scene proxies snapshotted from worker to render thread.
#[derive(Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
pub struct ProxyStore {
    /// All proxies visible this frame.
    pub proxies: Vec<RenderProxy>,
}

/// Selection outline data snapshotted into the frame.
#[derive(Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
pub struct SelectionOutlineData {
    /// Selected entities contributing to stencil or color-ID outline passes.
    pub selected_entities: SmallVec<[Entity; 256]>,
    /// Outline RGBA color in linear space.
    pub outline_color: LinearColor,
    /// Outline width in pixels.
    pub outline_width: f32,
}

/// Editor-specific render view contributed into the frame snapshot.
#[derive(Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
pub struct EditorRenderView {
    /// Base render view (camera, target, identity).
    pub view: RenderView,
    /// Layer mask for this editor view.
    pub render_layers: RenderLayers,
    /// Gizmo commands for the overlay pass.
    pub gizmo_commands: SmallVec<[GizmoDrawCommand; 64]>,
    /// Optional selection outline parameters.
    pub selection_outline: Option<SelectionOutlineData>,
    /// Optional buffer visualization mode for debug views.
    pub buffer_vis: Option<BufferVisMode>,
}

/// Per open editor panel configuration snapshotted at Phase 7.
#[derive(Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
pub struct EditorViewport {
    /// Stable view identity for this panel.
    pub view_id: ViewId,
    /// Camera entity reference; snapshots sample this entity each frame.
    pub camera_entity: Entity,
    /// Selected shading path for the viewport.
    pub render_path: RenderPath,
    /// Optional debug visualization mode for the viewport.
    pub debug_mode: Option<BufferVisMode>,
    /// Layer mask controlling which scene layers draw into this view.
    pub render_layers: RenderLayers,
    /// Whether the infinite editor grid should draw.
    pub show_grid: bool,
    /// Whether editor gizmos should draw.
    pub show_gizmos: bool,
}

/// Worker-thread-owned selection state (not serialized across the render boundary).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectionSet {
    /// Selected entities for this frame.
    pub items: SmallVec<[Entity; 8]>,
}

/// Per-view draw command list wrapper used while building gizmo commands on the worker.
#[derive(Clone, Debug, PartialEq)]
pub struct DrawList {
    /// Commands collected for the current frame.
    pub commands: SmallVec<[GizmoDrawCommand; 64]>,
}

/// Static registry loaded from the middleman `.dylib` at startup (not snapshotted).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EditorOverlayRegistry {
    /// Registered buffer visualization modes.
    pub buffer_vis_modes: Vec<BufferVisMode>,
    /// Registered custom gizmo type identifiers.
    pub gizmo_types: Vec<GizmoTypeId>,
}

/// Render graph pass identifier for stable ordering.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RenderPhase {
    /// Opaque geometry pass.
    Opaque,
    /// Transparent geometry pass.
    Transparent,
    /// Stencil writes for selected entities.
    SelectionStencil,
    /// Sobel outline compute / post-process.
    SelectionOutline,
    /// Gizmo overlay pass after scene color is resolved.
    GizmoOverlay,
    /// Debug overlay pass (normals, UVs, etc.).
    DebugOverlay,
    /// Infinite editor grid pass.
    EditorGrid,
}

/// Immutable render frame handed to the render thread after Phase 7 extract.
#[derive(Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
pub struct RenderFrame {
    /// All editor views for this frame.
    pub editor_views: SmallVec<[EditorRenderView; 4]>,
    /// Scene proxies shared across all views.
    pub proxy_store: ProxyStore,
    /// Monotonic frame counter.
    pub frame_index: u64,
}
