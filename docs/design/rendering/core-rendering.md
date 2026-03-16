# Core Rendering Design

## Requirements Trace

> **Canonical sources:** Features, requirements, and user
> stories are defined in [features/rendering/](../../features/rendering/),
> [requirements/rendering/](../../requirements/rendering/), and
> [user-stories/rendering/](../../user-stories/rendering/). The table
> below traces design elements to those definitions.

| Feature | Requirement | User Stories | Description |
|---------|-------------|--------------|-------------|
| F-2.3.1 | R-2.3.1 | US-2.3.1.1, US-2.3.1.2, US-2.3.1.3 | Direct lighting: point, spot, directional with unified light buffer |
| F-2.3.2 | R-2.3.2 | US-2.3.2.1, US-2.3.2.2 | GPU meshlet-level frustum culling via compute |
| F-2.3.3 | R-2.3.3 | US-2.3.3.1, US-2.3.3.2 | Meshlet-level backface culling via normal cones |
| F-2.3.4 | R-2.3.4 | US-2.3.4.1, US-2.3.4.2, US-2.3.4.3 | Two-phase HZB occlusion culling |
| F-2.3.5 | R-2.3.5 | US-2.3.5.1, US-2.3.5.2 | Orthographic camera projection |
| F-2.3.6 | R-2.3.6 | US-2.3.6.1, US-2.3.6.2 | Perspective projection with reverse-Z |
| F-2.3.7 | R-2.3.7 | US-2.3.7.1, US-2.3.7.2, US-2.3.7.3 | GPU-driven instancing and batch compaction |
| F-2.3.8 | R-2.3.8 | US-2.3.8.1, US-2.3.8.2 | Render-to-texture with automatic barriers |
| F-2.3.9 | R-2.3.9 | US-2.3.9.1, US-2.3.9.2 | Static and dynamic cubemap rendering |
| F-2.3.10 | R-2.3.10 | US-2.3.10.1, US-2.3.10.2 | Scene capture (planar and cubemap) |
| F-2.3.11 | R-2.3.11 | US-2.3.11.1, US-2.3.11.2, US-2.3.11.3 | Dynamic resolution scaling with GPU feedback |
| F-2.3.12 | R-2.3.12 | US-2.3.12.1, US-2.3.12.2 | Subsurface scattering (screen-space and RT) |
| F-2.3.13 | R-2.3.13 | US-2.3.13.1, US-2.3.13.2 | Alpha mask cutout geometry |

### Non-Functional Requirements

| NFR | Description | Target |
|-----|-------------|--------|
| NFR-2.3.1 | Culling pipeline latency (1M meshlets, 1080p) | < 1.0 ms GPU |
| NFR-2.3.2 | Dynamic resolution convergence | 5 frames, < 5% oscillation |
| NFR-2.3.3 | Instancing draw call reduction (10k instances, 10 mats) | 10 draw calls |

### Cross-Cutting Dependencies

| Dependency | Source | Consumed API |
|------------|--------|--------------|
| GPU backend trait | F-2.1.1 | `GpuBackend` associated types |
| Command buffer | F-2.1.2 | `CommandBuffer::dispatch`, `draw_indexed_indirect` |
| Pipeline state | F-2.1.3 | `PipelineState` creation and binding |
| GPU sub-allocator | F-2.1.7 | `GpuAllocator::alloc_buffer`, `alloc_texture` |
| GPU state tracking | F-2.1.8 | Redundant state filtering |
| Barrier optimization | F-2.1.9 | Automatic barrier batching |
| GPU timing queries | F-2.1.12 | `TimestampQuery` for DRS feedback |
| Render graph | F-2.2.1 | `RenderPass` trait, resource declaration |
| Transient resources | F-2.2.3 | Virtual resource handles |
| Multi-view execution | F-2.2.9 | Per-view parameter overrides |
| Render proxy extraction | F-2.10.1 | Frame snapshot from ECS |
| Render components | F-2.10.2 | SoA proxy layout |
| Change detection | F-2.10.3 | Dirty-flag incremental upload |
| Draw list construction | F-2.10.6 | Sort key generation |
| Meshlet decomposition | F-3.1.1 | Meshlet AABB, normal cone, LOD DAG |
| Two-phase HZB (shared) | F-3.1.2 | HZB build and test shaders |
| Scene transforms | F-1.2.4 | `GlobalTransform` world matrix |
| Shared spatial index | F-1.9.1 | BVH frustum query for coarse cull |
| Thread pool | F-14.3.1 | Scoped parallel extraction |
| PBR materials | F-2.4.3 | Cook-Torrance BRDF evaluation |
| Forward+ tiled | F-2.4.1 | Tiled/clustered light assignment |
| Deferred G-buffer | F-2.4.2 | G-buffer layout and lighting pass |

## Overview

The core rendering subsystem bridges the ECS world
and the GPU. Each frame it extracts visible
entities into a renderer-owned snapshot, executes a
multi-stage GPU culling pipeline, batches survivors
by material, and submits draw commands through the
render graph.

Four design principles govern the system:

1. **ECS is the single source of truth.** All
   render state lives as components. The renderer
   reads via immutable queries and never owns
   persistent scene data outside the extracted
   frame snapshot.
2. **GPU-driven pipeline.** Frustum culling,
   backface culling, occlusion culling, LOD
   selection, and instance compaction all run as
   GPU compute passes. The CPU issues a small,
   fixed number of indirect dispatches per frame.
3. **Dual rendering paths.** Forward+ (tiled
   clustered) and deferred (G-buffer) paths share
   the same culling pipeline, light buffer, and
   material system. Path selection is per-view.
4. **Static dispatch.** The entire pipeline is
   generic over `GpuBackend`. No trait objects, no
   vtables, no dynamic dispatch in the hot path.

## Architecture

### Module Boundaries

```mermaid
graph TD
    subgraph "harmonius_rendering::core"
        EXT[RenderExtractor]
        PROXY[RenderProxy Store]
        CULL[GpuCullingPipeline]
        BATCH[BatchCompactor]
        QUEUE[RenderQueueSorter]
        DRAW[DrawCommandEncoder]
        MAT[MaterialSystem]
        DRS[DynamicResolution]
        PROJ[ProjectionSystem]
        CAP[SceneCapture]
    end

    subgraph "harmonius_rendering::lighting"
        LB[LightBuffer]
        FWD["Forward+ Tiled"]
        DEF[Deferred G-Buffer]
        PBR[PBR Evaluator]
    end

    subgraph "ECS World"
        MC[MeshComponent]
        MATC[MaterialComponent]
        TC[TransformComponent]
        LC[LightComponent]
        CC[CameraComponent]
        VC[VisibilityComponent]
    end

    subgraph "harmonius_gpu"
        GPU[GpuBackend Trait]
        CMD[CommandBuffer]
        PSO[PipelineState]
        MEM[GpuAllocator]
    end

    subgraph "harmonius_rendering::graph"
        RG[RenderGraph]
    end

    MC --> EXT
    MATC --> EXT
    TC --> EXT
    LC --> EXT
    CC --> EXT
    VC --> EXT

    EXT --> PROXY
    PROXY --> CULL
    CULL --> BATCH
    BATCH --> QUEUE
    QUEUE --> DRAW

    MAT --> PSO
    MAT --> DRAW
    LB --> FWD
    LB --> DEF
    PBR --> FWD
    PBR --> DEF

    DRAW --> CMD
    CMD --> GPU
    PSO --> GPU
    MEM --> GPU

    RG --> DRAW
    DRS --> RG
    PROJ --> CULL
    CAP --> RG
```

### Directory Layout

```
harmonius_rendering/
├── core/
│   ├── extract.rs        # RenderExtractor system
│   ├── proxy.rs          # RenderProxy, proxy SoA
│   ├── culling.rs        # GpuCullingPipeline
│   ├── batch.rs          # BatchCompactor
│   ├── queue.rs          # RenderQueueSorter,
│   │                     # SortKey
│   ├── draw.rs           # DrawCommandEncoder
│   ├── projection.rs     # Projection, ViewUniform
│   ├── dynamic_res.rs    # DynamicResolution
│   ├── capture.rs        # SceneCapture,
│   │                     # CubemapCapture
│   ├── depth_prepass.rs  # DepthPrepass render pass
│   ├── hzb.rs            # HzbBuilder,
│   │                     # HzbMipChain
│   └── alpha_cutout.rs   # AlphaCutout pass
├── lighting/
│   ├── light_buffer.rs   # LightBuffer, LightGpu
│   ├── forward_plus.rs   # TiledLightCull,
│   │                     # ForwardPlusPass
│   ├── deferred.rs       # GBufferPass,
│   │                     # DeferredLightPass
│   ├── pbr.rs            # Cook-Torrance BRDF
│   │                     # evaluation
│   └── sss.rs            # SubsurfaceScatter pass
├── material/
│   ├── material.rs       # Material, MaterialId
│   ├── instance.rs       # MaterialInstance
│   ├── permutation.rs    # ShaderPermutation,
│   │                     # PermutationKey
│   └── bindless.rs       # BindlessTable,
│   │                     # descriptor management
│   └── shading_model.rs  # ShadingModel enum
└── shaders/
    ├── culling/
    │   ├── frustum_cull.hlsl
    │   ├── backface_cull.hlsl
    │   ├── hzb_test.hlsl
    │   └── batch_compact.hlsl
    ├── depth/
    │   ├── depth_prepass.hlsl
    │   ├── hzb_build.hlsl
    │   └── alpha_cutout.hlsl
    ├── lighting/
    │   ├── tile_cull.hlsl
    │   ├── forward_plus.hlsl
    │   ├── deferred_gbuffer.hlsl
    │   ├── deferred_light.hlsl
    │   └── sss.hlsl
    └── common/
        ├── pbr_brdf.hlsl
        ├── view_uniforms.hlsl
        └── bindless.hlsl
```

### GPU Culling Pipeline

The culling pipeline is a chain of GPU compute
passes that progressively eliminates invisible
meshlets before any rasterization occurs. Each
stage reads from a global instance buffer and
writes a visibility bitmask or compacted index
list.

```mermaid
flowchart TD
    A[Instance Buffer] --> B[Frustum Cull]
    B -->|pass| C[Backface Cull]
    B -->|fail| R1[Rejected]
    C -->|pass| D[HZB Phase 1]
    C -->|fail| R2[Rejected]
    D -->|pass| E[Depth Prepass]
    D -->|reject| F[Reject Buffer]
    E --> G[Build HZB]
    G --> H[HZB Phase 2]
    F --> H
    H -->|pass| I[Phase 2 Survivors]
    H -->|fail| R3[Rejected]
    E --> J[Batch Compaction]
    I --> J
    J --> K[Indirect Draw Buffer]
    K --> L[Opaque Pass]
    K --> M[Transparent Pass]
```

### Frame Rendering Sequence

```mermaid
sequenceDiagram
    participant ECS as ECS World
    participant EXT as Extractor
    participant UP as GPU Upload
    participant CULL as GPU Culling
    participant BAT as Batch Compactor
    participant RG as Render Graph
    participant GPU as GPU Backend

    ECS->>EXT: extract visible entities
    EXT->>UP: upload instance buffer
    UP->>CULL: dispatch frustum cull
    CULL->>CULL: backface cull
    CULL->>CULL: HZB phase 1
    CULL->>RG: depth prepass
    RG->>CULL: build current HZB
    CULL->>CULL: HZB phase 2
    CULL->>BAT: compact survivors
    BAT->>RG: indirect draw buffer
    RG->>GPU: opaque pass
    RG->>GPU: transparent pass
    RG->>GPU: post-processing
    RG->>GPU: present
```

### Forward+ vs Deferred Data Flow

Both rendering paths share the culling pipeline,
light buffer, material system, and PBR evaluator.
The path divergence happens at the shading stage.

```mermaid
flowchart LR
    subgraph Shared
        CULL[GPU Culling]
        LB[Light Buffer]
        MAT[Material System]
        PBR[PBR BRDF]
    end

    subgraph "Forward+ Path"
        TILE[Tile Light Cull]
        FS[Forward Shade]
    end

    subgraph "Deferred Path"
        GB[G-Buffer Write]
        DL[Deferred Light Pass]
    end

    CULL --> TILE
    CULL --> GB
    LB --> TILE
    LB --> DL
    MAT --> FS
    MAT --> GB
    PBR --> FS
    PBR --> DL
    TILE --> FS
    GB --> DL
```

### Render Component Class Diagram

All render state lives as ECS components. The
extractor copies a minimal SoA snapshot each frame.

```mermaid
classDiagram
    class MeshComponent {
        +mesh_handle: MeshHandle
        +meshlet_count: u32
        +lod_bias: f32
    }

    class MaterialComponent {
        +material_id: MaterialId
        +instance_id: MaterialInstanceId
    }

    class CameraComponent {
        +projection: Projection
        +viewport: Viewport
        +render_path: RenderPath
    }

    class VisibilityComponent {
        +visible: bool
        +render_layers: RenderLayerMask
        +cast_shadows: bool
    }

    class LightComponent {
        +kind: LightKind
        +color: LinearColor
        +intensity: f32
        +range: f32
    }

    class GlobalTransform {
        +matrix: Mat4
    }

    class DynamicResolutionState {
        +scale: f32
        +min_scale: f32
        +max_scale: f32
        +target_ms: f32
    }

    class SceneCaptureComponent {
        +capture_mode: CaptureMode
        +resolution: UVec2
        +target_texture: TextureHandle
    }

    MeshComponent --> MaterialComponent
    MeshComponent --> GlobalTransform
    MeshComponent --> VisibilityComponent
    CameraComponent --> GlobalTransform
    CameraComponent --> DynamicResolutionState
    LightComponent --> GlobalTransform
    SceneCaptureComponent --> CameraComponent
```

### Material System Architecture

```mermaid
graph TD
    subgraph "Material Definition"
        SM[ShadingModel]
        SP[ShaderPermutation]
        PK[PermutationKey]
    end

    subgraph "Material Instance"
        MI[MaterialInstance]
        PP[Parameter Buffer]
        BT[BindlessTable]
    end

    subgraph "GPU Pipeline"
        PSO[PipelineState]
        DS[Descriptor Set]
    end

    SM --> SP
    PK --> SP
    SP --> PSO
    MI --> PP
    MI --> BT
    PP --> DS
    BT --> DS
    PSO --> DS
```

## API Design

### ECS Components

```rust
/// GPU mesh reference. Stored per renderable
/// entity.
#[derive(Clone, Debug, Reflect)]
pub struct MeshComponent {
    /// Handle into the GPU mesh registry.
    pub mesh_handle: MeshHandle,
    /// Number of meshlets in the mesh's LOD 0.
    pub meshlet_count: u32,
    /// Bias applied to LOD selection. Negative
    /// values force higher detail.
    pub lod_bias: f32,
}

/// Material assignment for a renderable entity.
#[derive(Clone, Debug, Reflect)]
pub struct MaterialComponent {
    /// Compiled material (shader + fixed state).
    pub material_id: MaterialId,
    /// Per-instance parameter overrides.
    pub instance_id: MaterialInstanceId,
}

/// Active camera used for view rendering.
#[derive(Clone, Debug, Reflect)]
pub struct CameraComponent {
    pub projection: Projection,
    pub viewport: Viewport,
    pub render_path: RenderPath,
    pub clear_color: LinearColor,
    pub render_order: i32,
}

/// Visibility flags for culling and layer
/// filtering.
#[derive(Clone, Debug, Reflect)]
pub struct VisibilityComponent {
    /// Master visibility toggle.
    pub visible: bool,
    /// Bitmask selecting which render layers
    /// this entity appears in.
    pub render_layers: RenderLayerMask,
    /// Whether this entity casts shadows.
    pub cast_shadows: bool,
    /// Whether this entity is two-sided (skips
    /// backface culling).
    pub two_sided: bool,
}

/// Light source attached to an entity.
#[derive(Clone, Debug, Reflect)]
pub struct LightComponent {
    pub kind: LightKind,
    pub color: LinearColor,
    /// Luminous intensity in candela.
    pub intensity: f32,
    /// Maximum range in world units. Zero means
    /// infinite (directional lights).
    pub range: f32,
    /// IES profile handle. None for uniform
    /// falloff.
    pub ies_profile: Option<IesProfileHandle>,
}

/// Dynamic resolution state per camera.
#[derive(Clone, Debug, Reflect)]
pub struct DynamicResolutionState {
    /// Current render scale [0.0, 1.0].
    pub scale: f32,
    /// Minimum allowed scale.
    pub min_scale: f32,
    /// Maximum allowed scale.
    pub max_scale: f32,
    /// Target GPU frame time in milliseconds.
    pub target_ms: f32,
}

/// Scene capture configuration.
#[derive(Clone, Debug, Reflect)]
pub struct SceneCaptureComponent {
    pub capture_mode: CaptureMode,
    pub resolution: UVec2,
    pub target_texture: TextureHandle,
    /// Update frequency: every N frames.
    pub update_interval: u32,
}
```

### Enums and Value Types

```rust
/// Camera projection mode.
#[derive(Clone, Debug, Reflect)]
pub enum Projection {
    Perspective(PerspectiveProjection),
    Orthographic(OrthographicProjection),
}

#[derive(Clone, Debug, Reflect)]
pub struct PerspectiveProjection {
    /// Vertical field of view in radians.
    pub fov_y: f32,
    /// Near clip plane distance.
    pub near: f32,
    /// Far clip plane. None = infinite far.
    pub far: Option<f32>,
    /// Aspect ratio (width / height).
    pub aspect: f32,
}

#[derive(Clone, Debug, Reflect)]
pub struct OrthographicProjection {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
}

/// Rendering path selection.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Reflect,
)]
pub enum RenderPath {
    /// Tiled/clustered forward lighting.
    ForwardPlus,
    /// G-buffer deferred lighting.
    Deferred,
}

/// Light type discriminant.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Reflect,
)]
pub enum LightKind {
    Directional,
    Point,
    Spot {
        /// Inner cone angle in radians.
        inner_angle: f32,
        /// Outer cone angle in radians.
        outer_angle: f32,
    },
}

/// Scene capture mode.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Reflect,
)]
pub enum CaptureMode {
    /// Single planar capture from camera view.
    Planar,
    /// Six-face cubemap capture.
    Cubemap,
}

/// 32-bit bitmask for render layer filtering.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Reflect,
)]
pub struct RenderLayerMask(pub u32);

impl RenderLayerMask {
    pub const ALL: Self = Self(u32::MAX);
    pub const DEFAULT: Self = Self(1);

    pub fn intersects(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }
}

/// Viewport rectangle in pixels.
#[derive(Clone, Debug, Reflect)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}
```

### Projection System

```rust
/// Per-view uniform data uploaded to the GPU.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod)]
pub struct ViewUniform {
    pub view: Mat4,
    pub projection: Mat4,
    pub view_projection: Mat4,
    pub inv_view: Mat4,
    pub inv_projection: Mat4,
    pub inv_view_projection: Mat4,
    pub camera_position: Vec4,
    pub viewport_size: Vec2,
    pub near_plane: f32,
    pub far_plane: f32,
    /// Frustum planes (6 x Vec4, normal + dist).
    pub frustum_planes: [Vec4; 6],
}

/// Builds projection matrices from camera
/// components.
pub struct ProjectionSystem;

impl ProjectionSystem {
    /// Compute a reverse-Z perspective matrix.
    /// Near maps to depth=1, far maps to depth=0.
    pub fn perspective_reverse_z(
        fov_y: f32,
        aspect: f32,
        near: f32,
        far: Option<f32>,
    ) -> Mat4 {
        // Infinite far plane when far is None.
        // Reverse-Z: swap near/far in the
        // projection to maximize depth precision
        // at distance.
        todo!()
    }

    /// Compute an orthographic projection matrix
    /// with reverse-Z depth mapping.
    pub fn orthographic_reverse_z(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Mat4 {
        todo!()
    }

    /// Extract six frustum planes from a
    /// view-projection matrix.
    pub fn extract_frustum_planes(
        view_projection: &Mat4,
    ) -> [Vec4; 6] {
        todo!()
    }

    /// Build a ViewUniform from camera and
    /// transform components.
    pub fn build_view_uniform(
        camera: &CameraComponent,
        transform: &GlobalTransform,
        drs: Option<&DynamicResolutionState>,
    ) -> ViewUniform {
        todo!()
    }
}
```

### Render Extraction

```rust
/// Opaque handle indexing into the proxy store.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash,
)]
pub struct RenderProxyId(pub u32);

/// Minimal per-instance data for GPU upload.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod)]
pub struct InstanceGpu {
    pub world_matrix: Mat4,
    pub prev_world_matrix: Mat4,
    pub mesh_handle: u32,
    pub material_id: u32,
    pub instance_id: u32,
    pub meshlet_offset: u32,
    pub meshlet_count: u32,
    pub lod_bias: f32,
    pub flags: u32,
    pub _padding: u32,
}

/// Extracted frame snapshot. Owned by the renderer
/// and decoupled from the ECS world.
pub struct RenderProxyStore {
    /// SoA: world matrices.
    pub transforms: Vec<Mat4>,
    /// SoA: previous frame matrices (motion).
    pub prev_transforms: Vec<Mat4>,
    /// SoA: mesh handles.
    pub meshes: Vec<MeshHandle>,
    /// SoA: material + instance IDs.
    pub materials: Vec<(MaterialId, MaterialInstanceId)>,
    /// SoA: visibility flags.
    pub flags: Vec<InstanceFlags>,
    /// Total instance count this frame.
    pub count: u32,
}

/// Flags packed into InstanceGpu::flags.
bitflags::bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct InstanceFlags: u32 {
        const CAST_SHADOWS = 1 << 0;
        const TWO_SIDED    = 1 << 1;
        const ALPHA_CUTOUT = 1 << 2;
        const TRANSPARENT  = 1 << 3;
    }
}

/// Extracts renderable entities from the ECS into
/// the proxy store. Runs as a system in the
/// Extract phase.
pub struct RenderExtractor;

impl RenderExtractor {
    /// Parallel extraction using scoped thread
    /// pool tasks. Reads MeshComponent,
    /// MaterialComponent, GlobalTransform,
    /// VisibilityComponent via immutable queries.
    /// Only extracts entities whose
    /// VisibilityComponent::visible is true.
    /// Uses change detection to incrementally
    /// update only dirty entities.
    pub fn extract<B: GpuBackend>(
        world: &World,
        pool: &ThreadPool,
        store: &mut RenderProxyStore,
    ) {
        todo!()
    }

    /// Upload the proxy store into a GPU instance
    /// buffer. Returns the buffer handle for the
    /// culling pipeline.
    pub fn upload<B: GpuBackend>(
        store: &RenderProxyStore,
        allocator: &GpuAllocator<B>,
        cmd: &mut B::CommandBuffer,
    ) -> B::BufferHandle {
        todo!()
    }
}
```

### GPU Culling Pipeline

```rust
/// Configuration for the GPU culling pipeline.
pub struct CullingConfig {
    /// Maximum meshlet count the buffers can
    /// hold. Determines allocation sizes.
    pub max_meshlets: u32,
    /// HZB mip chain resolution. Typically
    /// matches the depth buffer resolution.
    pub hzb_resolution: UVec2,
    /// Whether to run HZB phase 2 (can be
    /// skipped under budget pressure on mobile).
    pub enable_phase_2: bool,
}

/// Visibility bitmask buffer. One bit per
/// meshlet indicating pass/fail.
pub struct VisibilityBuffer<B: GpuBackend> {
    pub buffer: B::BufferHandle,
    pub meshlet_count: u32,
}

/// The GPU-driven culling pipeline. Each stage
/// is a compute dispatch reading/writing GPU
/// buffers.
pub struct GpuCullingPipeline<B: GpuBackend> {
    frustum_cull_pso: B::PipelineState,
    backface_cull_pso: B::PipelineState,
    hzb_test_pso: B::PipelineState,
    hzb_build_pso: B::PipelineState,
    batch_compact_pso: B::PipelineState,
    config: CullingConfig,
    /// Previous frame's HZB for phase 1.
    prev_hzb: B::TextureHandle,
    /// Current frame's HZB built from depth
    /// prepass.
    current_hzb: B::TextureHandle,
    /// Buffer holding phase-1 rejected meshlet
    /// indices for phase-2 retest.
    reject_buffer: B::BufferHandle,
}

impl<B: GpuBackend> GpuCullingPipeline<B> {
    pub fn new(
        device: &B::Device,
        config: CullingConfig,
    ) -> Self {
        todo!()
    }

    /// Run frustum culling. Tests each meshlet
    /// AABB against the six frustum planes from
    /// ViewUniform. Writes a visibility bitmask.
    pub fn frustum_cull(
        &self,
        cmd: &mut B::CommandBuffer,
        instances: &B::BufferHandle,
        view: &ViewUniform,
    ) -> VisibilityBuffer<B> {
        todo!()
    }

    /// Run backface culling via normal cone test.
    /// Reads the meshlet normal cone data and
    /// camera position. Clears bits in the
    /// visibility buffer for fully back-facing
    /// meshlets. Skips meshlets flagged as
    /// two-sided.
    pub fn backface_cull(
        &self,
        cmd: &mut B::CommandBuffer,
        instances: &B::BufferHandle,
        visibility: &mut VisibilityBuffer<B>,
        camera_pos: Vec3,
    ) {
        todo!()
    }

    /// HZB phase 1: test visible meshlets against
    /// the previous frame's HZB (conservative).
    /// Meshlets that pass are rendered in the
    /// depth prepass. Meshlets that fail are
    /// written to the reject buffer for phase 2.
    pub fn hzb_phase_1(
        &self,
        cmd: &mut B::CommandBuffer,
        instances: &B::BufferHandle,
        visibility: &mut VisibilityBuffer<B>,
    ) {
        todo!()
    }

    /// Build the current frame's HZB from the
    /// depth prepass output. Generates a mip
    /// chain via iterative max-downsample.
    pub fn build_hzb(
        &self,
        cmd: &mut B::CommandBuffer,
        depth_buffer: &B::TextureHandle,
    ) {
        todo!()
    }

    /// HZB phase 2: retest phase-1 rejects
    /// against the newly built HZB. Recovers
    /// newly visible geometry in the same frame.
    pub fn hzb_phase_2(
        &self,
        cmd: &mut B::CommandBuffer,
        visibility: &mut VisibilityBuffer<B>,
    ) {
        todo!()
    }

    /// Swap HZB buffers at end of frame. The
    /// current HZB becomes the previous for the
    /// next frame.
    pub fn end_frame(&mut self) {
        core::mem::swap(
            &mut self.prev_hzb,
            &mut self.current_hzb,
        );
    }
}
```

### Batch Compaction and Instancing

```rust
/// Sort key encoding for render queue ordering.
/// Opaque: pipeline hash | material | depth
///         (front-to-back).
/// Transparent: depth (back-to-front) only.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    PartialOrd, Ord,
)]
pub struct SortKey(pub u64);

impl SortKey {
    /// Build an opaque sort key. Encodes pipeline
    /// in the high bits for minimal state changes,
    /// then material ID, then quantized depth
    /// (front-to-back) for early-Z efficiency.
    pub fn opaque(
        pipeline_hash: u16,
        material_id: u16,
        depth: f32,
    ) -> Self {
        let depth_bits =
            quantize_depth_front_to_back(depth);
        let key = (pipeline_hash as u64) << 48
            | (material_id as u64) << 32
            | depth_bits as u64;
        Self(key)
    }

    /// Build a transparent sort key. Depth is
    /// the only criterion (back-to-front).
    pub fn transparent(depth: f32) -> Self {
        let depth_bits =
            quantize_depth_back_to_front(depth);
        Self(depth_bits as u64)
    }
}

/// Indirect draw command matching
/// D3D12/Vulkan/Metal indirect draw layout.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod)]
pub struct IndirectDrawCommand {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub vertex_offset: i32,
    pub first_instance: u32,
}

/// GPU-side batch compaction. Groups surviving
/// meshlets by material into contiguous indirect
/// draw commands.
pub struct BatchCompactor<B: GpuBackend> {
    compact_pso: B::PipelineState,
    /// Output buffer: one IndirectDrawCommand per
    /// material batch.
    draw_buffer: B::BufferHandle,
    /// Output buffer: draw count (for
    /// multi-draw-indirect-count).
    draw_count_buffer: B::BufferHandle,
    /// Maximum number of material batches.
    max_batches: u32,
}

impl<B: GpuBackend> BatchCompactor<B> {
    pub fn new(
        device: &B::Device,
        max_batches: u32,
    ) -> Self {
        todo!()
    }

    /// Run the compaction compute pass. Reads
    /// the visibility buffer and instance data,
    /// groups by material_id, and writes
    /// contiguous indirect draw commands.
    pub fn compact(
        &self,
        cmd: &mut B::CommandBuffer,
        instances: &B::BufferHandle,
        visibility: &VisibilityBuffer<B>,
    ) -> DrawBatch<B> {
        todo!()
    }
}

/// Result of batch compaction: everything
/// needed to issue indirect draws.
pub struct DrawBatch<B: GpuBackend> {
    /// Indirect draw buffer (one entry per
    /// material batch).
    pub draw_buffer: B::BufferHandle,
    /// Draw count buffer for
    /// execute_indirect_count.
    pub draw_count_buffer: B::BufferHandle,
    /// Maximum batch count (upper bound for
    /// the indirect count).
    pub max_draw_count: u32,
}
```

### Render Queue Sorter

```rust
/// Partitions draw commands into sorted render
/// queues by pass type.
pub struct RenderQueueSorter;

/// Classified render queues for a single view.
pub struct RenderQueues<B: GpuBackend> {
    /// GPU-driven indirect batch for opaque
    /// geometry (front-to-back by sort key).
    pub opaque: DrawBatch<B>,
    /// CPU-sorted transparent draws
    /// (back-to-front, individual draw calls).
    pub transparent: Vec<TransparentDraw>,
    /// Alpha cutout draws (participate in
    /// depth prepass and shadow maps).
    pub alpha_cutout: DrawBatch<B>,
}

/// A single transparent draw command.
/// Transparent objects are not batched.
pub struct TransparentDraw {
    pub sort_key: SortKey,
    pub instance_id: RenderProxyId,
    pub material_id: MaterialId,
    pub instance_params: MaterialInstanceId,
    pub mesh_handle: MeshHandle,
}

impl RenderQueueSorter {
    /// Classify extracted proxies into render
    /// queues. Opaque and alpha-cutout go through
    /// GPU batch compaction. Transparent draws are
    /// sorted CPU-side (back-to-front).
    pub fn sort<B: GpuBackend>(
        store: &RenderProxyStore,
        batch: DrawBatch<B>,
        camera_pos: Vec3,
    ) -> RenderQueues<B> {
        todo!()
    }
}
```

### Material System

```rust
/// Opaque handle to a compiled material.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash,
)]
pub struct MaterialId(pub u32);

/// Opaque handle to a material instance.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash,
)]
pub struct MaterialInstanceId(pub u32);

/// Shading model selection. Determines which
/// BRDF the shader evaluates.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash,
    Reflect,
)]
pub enum ShadingModel {
    /// Standard metallic-roughness PBR.
    DefaultLit,
    /// Subsurface scattering (skin, wax).
    Subsurface,
    /// Clearcoat (car paint, lacquered wood).
    ClearCoat,
    /// Cloth (velvet, cotton, silk).
    Cloth,
    /// Hair (Marschner anisotropic).
    Hair,
    /// Eye (cornea refraction + iris parallax).
    Eye,
    /// Thin translucent (single-pass glass).
    ThinTranslucent,
    /// Two-sided foliage (subsurface transmit).
    Foliage,
    /// Unlit (emissive only, no lighting).
    Unlit,
}

/// Shader feature flags that drive permutation
/// selection.
bitflags::bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct ShaderFeatures: u32 {
        const NORMAL_MAP      = 1 << 0;
        const EMISSIVE        = 1 << 1;
        const ALPHA_CUTOUT    = 1 << 2;
        const VERTEX_COLORS   = 1 << 3;
        const SKINNING        = 1 << 4;
        const MORPH_TARGETS   = 1 << 5;
        const PARALLAX        = 1 << 6;
        const CLEARCOAT       = 1 << 7;
        const ANISOTROPY      = 1 << 8;
        const SUBSURFACE      = 1 << 9;
    }
}

/// Unique key identifying a shader permutation.
/// Used for pipeline state caching.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash,
)]
pub struct PermutationKey {
    pub shading_model: ShadingModel,
    pub features: ShaderFeatures,
    pub render_path: RenderPath,
    pub vertex_format: VertexFormatId,
}

/// A compiled material: shader permutation +
/// fixed render state.
pub struct Material {
    pub id: MaterialId,
    pub shading_model: ShadingModel,
    pub features: ShaderFeatures,
    /// Compiled pipeline states per render path.
    pub pipelines: PipelineSet,
    /// Default parameter values.
    pub default_params: MaterialParams,
}

/// Per-path pipeline states for a material.
pub struct PipelineSet {
    /// Forward+ opaque pipeline.
    pub forward_opaque: Option<PipelineStateId>,
    /// Forward+ transparent pipeline.
    pub forward_transparent: Option<PipelineStateId>,
    /// Deferred G-buffer write pipeline.
    pub deferred_gbuffer: Option<PipelineStateId>,
    /// Depth-only prepass pipeline.
    pub depth_prepass: Option<PipelineStateId>,
    /// Shadow map pipeline.
    pub shadow: Option<PipelineStateId>,
}

/// A material instance: per-instance parameter
/// overrides sharing the parent material's
/// compiled shaders.
pub struct MaterialInstance {
    pub id: MaterialInstanceId,
    pub parent: MaterialId,
    /// Overridden parameters. None values fall
    /// back to the parent's defaults.
    pub params: MaterialParams,
    /// Bindless descriptor index for GPU access.
    pub bindless_index: u32,
}

/// Material parameter storage. Textures are
/// referenced via bindless indices.
pub struct MaterialParams {
    pub albedo_texture: Option<BindlessIndex>,
    pub normal_texture: Option<BindlessIndex>,
    pub metallic_roughness_texture:
        Option<BindlessIndex>,
    pub occlusion_texture: Option<BindlessIndex>,
    pub emissive_texture: Option<BindlessIndex>,
    pub albedo_factor: Vec4,
    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub emissive_factor: Vec3,
    pub alpha_cutoff: f32,
    /// Extended BSDF parameters.
    pub clearcoat_factor: f32,
    pub clearcoat_roughness: f32,
    pub anisotropy: f32,
    pub subsurface_radius: Vec3,
    pub sheen_color: Vec3,
    pub sheen_roughness: f32,
}

/// Manages material definitions, instances, and
/// shader permutation compilation.
pub struct MaterialSystem<B: GpuBackend> {
    materials: Vec<Material>,
    instances: Vec<MaterialInstance>,
    /// Cache: PermutationKey -> PipelineStateId.
    pipeline_cache:
        HashMap<PermutationKey, B::PipelineState>,
    /// Bindless descriptor table for all
    /// material textures.
    bindless_table: BindlessTable<B>,
}

impl<B: GpuBackend> MaterialSystem<B> {
    pub fn new(device: &B::Device) -> Self {
        todo!()
    }

    /// Register a new material. Compiles all
    /// required shader permutations and caches
    /// pipeline states.
    pub fn create_material(
        &mut self,
        device: &B::Device,
        shading_model: ShadingModel,
        features: ShaderFeatures,
        params: MaterialParams,
    ) -> MaterialId {
        todo!()
    }

    /// Create a material instance with parameter
    /// overrides. Shares the parent's compiled
    /// shaders.
    pub fn create_instance(
        &mut self,
        parent: MaterialId,
        params: MaterialParams,
    ) -> MaterialInstanceId {
        todo!()
    }

    /// Update an existing instance's parameters.
    /// Marks the instance dirty for GPU re-upload.
    pub fn update_instance(
        &mut self,
        id: MaterialInstanceId,
        params: MaterialParams,
    ) {
        todo!()
    }

    /// Upload all dirty material instance
    /// parameter buffers to the GPU.
    pub fn upload_dirty(
        &self,
        cmd: &mut B::CommandBuffer,
        allocator: &GpuAllocator<B>,
    ) {
        todo!()
    }

    /// Look up the pipeline state for a given
    /// permutation. Returns a cached PSO or
    /// compiles on demand.
    pub fn get_pipeline(
        &mut self,
        device: &B::Device,
        key: &PermutationKey,
    ) -> &B::PipelineState {
        todo!()
    }
}
```

### Bindless Descriptor Table

```rust
/// Index into the bindless descriptor heap.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash,
)]
pub struct BindlessIndex(pub u32);

/// Manages a global bindless descriptor table
/// for textures and buffers. All materials
/// reference resources by BindlessIndex.
pub struct BindlessTable<B: GpuBackend> {
    /// Platform descriptor heap/argument buffer.
    descriptor_heap: B::DescriptorHeap,
    /// Free list for recycling indices.
    free_list: Vec<BindlessIndex>,
    /// Next index to allocate when free list
    /// is empty.
    next_index: u32,
    /// Maximum descriptor count.
    capacity: u32,
}

impl<B: GpuBackend> BindlessTable<B> {
    pub fn new(
        device: &B::Device,
        capacity: u32,
    ) -> Self {
        todo!()
    }

    /// Allocate a bindless index and write a
    /// texture descriptor.
    pub fn insert_texture(
        &mut self,
        device: &B::Device,
        texture: &B::TextureHandle,
        sampler: &B::SamplerHandle,
    ) -> BindlessIndex {
        todo!()
    }

    /// Release a bindless index for reuse.
    pub fn remove(
        &mut self,
        index: BindlessIndex,
    ) {
        self.free_list.push(index);
    }
}
```

### Light Buffer

```rust
/// GPU-side light data for a single light.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod)]
pub struct LightGpu {
    /// World-space position (point/spot) or
    /// direction (directional).
    pub position_or_direction: Vec4,
    pub color_intensity: Vec4,
    /// x: range, y: inner cone cos,
    /// z: outer cone cos, w: light type.
    pub params: Vec4,
    /// Shadow matrix index (-1 = no shadow).
    pub shadow_matrix_index: i32,
    pub ies_profile_index: i32,
    pub _padding: [u32; 2],
}

/// Unified light buffer consumed by both forward
/// and deferred paths.
pub struct LightBuffer<B: GpuBackend> {
    /// GPU buffer holding LightGpu array.
    pub buffer: B::BufferHandle,
    /// Number of active lights this frame.
    pub light_count: u32,
    /// Maximum light capacity.
    pub capacity: u32,
}

impl<B: GpuBackend> LightBuffer<B> {
    pub fn new(
        device: &B::Device,
        capacity: u32,
    ) -> Self {
        todo!()
    }

    /// Extract lights from ECS and upload to the
    /// GPU buffer. Returns the light count.
    pub fn update(
        &mut self,
        world: &World,
        cmd: &mut B::CommandBuffer,
        allocator: &GpuAllocator<B>,
    ) -> u32 {
        todo!()
    }
}
```

### Depth Prepass

```rust
/// Depth-only prepass render pass. Populates the
/// depth buffer for HZB construction and early-Z
/// optimization in subsequent passes.
pub struct DepthPrepass;

impl DepthPrepass {
    /// Register the depth prepass with the render
    /// graph. Declares a depth-only render target
    /// write.
    pub fn register<B: GpuBackend>(
        graph: &mut RenderGraphBuilder<B>,
        view: &ViewUniform,
        opaque_batch: &DrawBatch<B>,
        alpha_cutout_batch: &DrawBatch<B>,
    ) -> RenderPassId {
        todo!()
    }
}
```

### HZB Builder

```rust
/// Hierarchical Z-buffer builder. Generates a
/// mip chain from the depth buffer via iterative
/// max-downsample compute passes.
pub struct HzbBuilder<B: GpuBackend> {
    downsample_pso: B::PipelineState,
    /// Full mip chain texture.
    pub hzb_texture: B::TextureHandle,
    pub mip_count: u32,
}

impl<B: GpuBackend> HzbBuilder<B> {
    pub fn new(
        device: &B::Device,
        resolution: UVec2,
    ) -> Self {
        todo!()
    }

    /// Build the HZB mip chain from the given
    /// depth buffer. Each mip level takes the
    /// max of a 2x2 region from the previous
    /// level (conservative for reverse-Z where
    /// closer = larger depth values).
    pub fn build(
        &self,
        cmd: &mut B::CommandBuffer,
        depth_buffer: &B::TextureHandle,
    ) {
        todo!()
    }
}
```

### Dynamic Resolution

```rust
/// GPU timing feedback loop that adjusts render
/// resolution to maintain a target frame budget.
pub struct DynamicResolution {
    /// Ring buffer of GPU frame times (ms) for
    /// smoothing.
    history: [f32; 8],
    history_index: usize,
    /// Current scale factor [min, max].
    current_scale: f32,
    /// Exponential moving average of GPU time.
    smoothed_gpu_ms: f32,
}

impl DynamicResolution {
    pub fn new() -> Self {
        todo!()
    }

    /// Feed a new GPU frame time measurement.
    /// Returns the updated render scale for the
    /// next frame. Converges within 5 frames
    /// (NFR-2.3.2). Scale adjusts proportionally
    /// to the ratio of measured vs target time.
    pub fn update(
        &mut self,
        gpu_ms: f32,
        state: &DynamicResolutionState,
    ) -> f32 {
        let target = state.target_ms;
        let ratio = target / gpu_ms.max(0.001);

        // EMA smoothing factor (higher = faster
        // convergence, more noise).
        let alpha = 0.4;
        self.smoothed_gpu_ms = self.smoothed_gpu_ms
            * (1.0 - alpha)
            + gpu_ms * alpha;

        let smoothed_ratio =
            target / self.smoothed_gpu_ms.max(0.001);

        // Scale resolution proportionally.
        // Square root because resolution is 2D
        // but GPU cost scales with pixel count.
        let new_scale = self.current_scale
            * smoothed_ratio.sqrt();

        self.current_scale = new_scale
            .clamp(state.min_scale, state.max_scale);

        self.history[self.history_index] =
            self.current_scale;
        self.history_index =
            (self.history_index + 1) % 8;

        self.current_scale
    }

    pub fn current_scale(&self) -> f32 {
        self.current_scale
    }
}
```

### Scene Capture

```rust
/// Renders the scene from a secondary camera
/// into a texture for use as a material input.
pub struct SceneCapture;

impl SceneCapture {
    /// Register a planar scene capture as a
    /// sub-graph in the render graph. The capture
    /// re-uses the same culling and draw
    /// infrastructure as the main view but with
    /// an independent ViewUniform and render
    /// target.
    pub fn register_planar<B: GpuBackend>(
        graph: &mut RenderGraphBuilder<B>,
        capture: &SceneCaptureComponent,
        camera: &CameraComponent,
        transform: &GlobalTransform,
    ) -> RenderPassId {
        todo!()
    }

    /// Register a cubemap capture. Generates
    /// six views (one per face) sharing the
    /// same capture position. Each face is a
    /// 90-degree FOV perspective view.
    pub fn register_cubemap<B: GpuBackend>(
        graph: &mut RenderGraphBuilder<B>,
        capture: &SceneCaptureComponent,
        transform: &GlobalTransform,
    ) -> [RenderPassId; 6] {
        todo!()
    }
}

/// Dynamic cubemap that re-renders a subset of
/// faces each frame to amortize cost.
pub struct DynamicCubemap<B: GpuBackend> {
    pub cubemap_texture: B::TextureHandle,
    pub face_resolution: u32,
    /// Which face to update this frame (0-5).
    pub current_face: u32,
    /// Number of faces to update per frame.
    pub faces_per_frame: u32,
}

impl<B: GpuBackend> DynamicCubemap<B> {
    pub fn new(
        device: &B::Device,
        resolution: u32,
        faces_per_frame: u32,
    ) -> Self {
        todo!()
    }

    /// Advance the face rotation and return
    /// the face indices to render this frame.
    pub fn next_faces(&mut self) -> &[u32] {
        todo!()
    }
}
```

### Draw Command Encoder

```rust
/// Encodes final draw commands from the sorted
/// render queues into GPU command buffers.
pub struct DrawCommandEncoder;

impl DrawCommandEncoder {
    /// Encode opaque draws via multi-draw-indirect
    /// from the batch compactor output.
    pub fn encode_opaque<B: GpuBackend>(
        cmd: &mut B::CommandBuffer,
        batch: &DrawBatch<B>,
        material_system: &MaterialSystem<B>,
        view_uniform_buffer: &B::BufferHandle,
        light_buffer: &LightBuffer<B>,
    ) {
        todo!()
    }

    /// Encode transparent draws as individual
    /// draw calls in back-to-front order.
    pub fn encode_transparent<B: GpuBackend>(
        cmd: &mut B::CommandBuffer,
        draws: &[TransparentDraw],
        material_system: &MaterialSystem<B>,
        view_uniform_buffer: &B::BufferHandle,
        light_buffer: &LightBuffer<B>,
    ) {
        todo!()
    }

    /// Encode alpha-cutout draws via indirect
    /// batch. Alpha-cutout geometry participates
    /// in the depth prepass and shadow maps.
    pub fn encode_alpha_cutout<B: GpuBackend>(
        cmd: &mut B::CommandBuffer,
        batch: &DrawBatch<B>,
        material_system: &MaterialSystem<B>,
        view_uniform_buffer: &B::BufferHandle,
    ) {
        todo!()
    }
}
```

### PBR BRDF (HLSL Reference)

The Cook-Torrance microfacet BRDF is implemented
in HLSL and shared across all lighting paths.

```hlsl
// pbr_brdf.hlsl — Cook-Torrance BRDF
// GGX/Trowbridge-Reitz NDF + Smith-GGX geometry
// + Schlick Fresnel

float DistributionGGX(
    float NdotH, float roughness
) {
    float a  = roughness * roughness;
    float a2 = a * a;
    float d  = NdotH * NdotH * (a2 - 1.0) + 1.0;
    return a2 / (PI * d * d);
}

float GeometrySmithGGX(
    float NdotV, float NdotL, float roughness
) {
    float r = roughness + 1.0;
    float k = (r * r) / 8.0;
    float gv = NdotV / (NdotV * (1.0 - k) + k);
    float gl = NdotL / (NdotL * (1.0 - k) + k);
    return gv * gl;
}

float3 FresnelSchlick(
    float cosTheta, float3 F0
) {
    float t = 1.0 - cosTheta;
    float t5 = t * t * t * t * t;
    return F0 + (1.0 - F0) * t5;
}

float3 EvaluateCookTorrance(
    float3 N, float3 V, float3 L,
    float3 albedo, float metallic,
    float roughness
) {
    float3 H = normalize(V + L);

    float NdotL = max(dot(N, L), 0.0);
    float NdotV = max(dot(N, V), 0.001);
    float NdotH = max(dot(N, H), 0.0);
    float VdotH = max(dot(V, H), 0.0);

    // Dielectric F0 = 0.04, blend with albedo
    // for metals.
    float3 F0 = lerp(
        float3(0.04, 0.04, 0.04),
        albedo,
        metallic
    );

    float  D = DistributionGGX(NdotH, roughness);
    float  G = GeometrySmithGGX(
        NdotV, NdotL, roughness
    );
    float3 F = FresnelSchlick(VdotH, F0);

    // Specular BRDF.
    float3 numerator = D * G * F;
    float  denominator =
        4.0 * NdotV * NdotL + 0.0001;
    float3 specular = numerator / denominator;

    // Energy conservation: diffuse is reduced
    // by the specular Fresnel reflection.
    float3 kD = (1.0 - F) * (1.0 - metallic);
    float3 diffuse = kD * albedo / PI;

    return (diffuse + specular) * NdotL;
}
```

### Tiled Light Culling (HLSL Reference)

```hlsl
// tile_cull.hlsl — Forward+ tiled light culling
// Assigns lights to screen-space tiles.

#define TILE_SIZE 16
#define MAX_LIGHTS_PER_TILE 256

groupshared uint tile_light_count;
groupshared uint tile_light_indices[
    MAX_LIGHTS_PER_TILE
];
groupshared uint tile_depth_min;
groupshared uint tile_depth_max;

[numthreads(TILE_SIZE, TILE_SIZE, 1)]
void CSTileLightCull(
    uint3 group_id : SV_GroupID,
    uint3 thread_id : SV_GroupThreadID,
    uint flat_id : SV_GroupIndex
) {
    // Step 1: Compute tile depth bounds from
    // the depth buffer.
    if (flat_id == 0) {
        tile_light_count = 0;
        tile_depth_min = 0xFFFFFFFF;
        tile_depth_max = 0;
    }
    GroupMemoryBarrierWithGroupSync();

    uint2 pixel = group_id.xy * TILE_SIZE
        + thread_id.xy;
    float depth = DepthBuffer.Load(
        int3(pixel, 0)
    ).r;
    uint depth_uint = asuint(depth);

    InterlockedMin(tile_depth_min, depth_uint);
    InterlockedMax(tile_depth_max, depth_uint);
    GroupMemoryBarrierWithGroupSync();

    float min_depth = asfloat(tile_depth_min);
    float max_depth = asfloat(tile_depth_max);

    // Step 2: Each thread tests a subset of
    // lights against the tile frustum.
    uint light_count = LightCountBuffer.Load(0);
    for (
        uint i = flat_id;
        i < light_count;
        i += TILE_SIZE * TILE_SIZE
    ) {
        LightGpu light = LightBuffer[i];
        if (LightIntersectsTile(
            light, group_id.xy,
            min_depth, max_depth
        )) {
            uint idx;
            InterlockedAdd(
                tile_light_count, 1, idx
            );
            if (idx < MAX_LIGHTS_PER_TILE) {
                tile_light_indices[idx] = i;
            }
        }
    }
    GroupMemoryBarrierWithGroupSync();

    // Step 3: Write tile light list to output.
    uint tile_index =
        group_id.y * TileCountX + group_id.x;
    uint count = min(
        tile_light_count, MAX_LIGHTS_PER_TILE
    );

    if (flat_id == 0) {
        TileLightCounts[tile_index] = count;
    }

    for (
        uint j = flat_id;
        j < count;
        j += TILE_SIZE * TILE_SIZE
    ) {
        TileLightIndices[
            tile_index * MAX_LIGHTS_PER_TILE + j
        ] = tile_light_indices[j];
    }
}
```

## Data Flow

### Full Frame Pipeline

```rust
// Simplified frame rendering pseudocode.
// Generic over GpuBackend for static dispatch.
fn render_frame<B: GpuBackend>(
    world: &World,
    pool: &ThreadPool,
    renderer: &mut Renderer<B>,
) {
    // 1. Extract ECS -> proxy store.
    RenderExtractor::extract::<B>(
        world, pool, &mut renderer.proxy_store,
    );

    // 2. Upload instance buffer to GPU.
    let instances = RenderExtractor::upload::<B>(
        &renderer.proxy_store,
        &renderer.allocator,
        &mut renderer.cmd,
    );

    // 3. Update light buffer.
    renderer.light_buffer.update(
        world, &mut renderer.cmd,
        &renderer.allocator,
    );

    // 4. For each active view (main camera,
    //    shadow cascades, scene captures):
    for view in &renderer.views {
        let view_uniform =
            ProjectionSystem::build_view_uniform(
                &view.camera,
                &view.transform,
                view.drs.as_ref(),
            );

        // 5. GPU culling pipeline.
        let mut vis = renderer.culling
            .frustum_cull(
                &mut renderer.cmd,
                &instances,
                &view_uniform,
            );
        renderer.culling.backface_cull(
            &mut renderer.cmd,
            &instances,
            &mut vis,
            view_uniform.camera_position.xyz(),
        );
        renderer.culling.hzb_phase_1(
            &mut renderer.cmd,
            &instances,
            &mut vis,
        );

        // 6. Depth prepass.
        // (registered as a render graph pass)

        // 7. Build current-frame HZB.
        renderer.culling.build_hzb(
            &mut renderer.cmd,
            &renderer.depth_buffer,
        );

        // 8. HZB phase 2 retest.
        renderer.culling.hzb_phase_2(
            &mut renderer.cmd,
            &mut vis,
        );

        // 9. Batch compaction.
        let batch = renderer.compactor.compact(
            &mut renderer.cmd,
            &instances,
            &vis,
        );

        // 10. Sort and classify queues.
        let queues = RenderQueueSorter::sort(
            &renderer.proxy_store,
            batch,
            view_uniform.camera_position.xyz(),
        );

        // 11. Encode draw commands.
        DrawCommandEncoder::encode_opaque::<B>(
            &mut renderer.cmd,
            &queues.opaque,
            &renderer.material_system,
            &view.view_buffer,
            &renderer.light_buffer,
        );
        DrawCommandEncoder
            ::encode_transparent::<B>(
            &mut renderer.cmd,
            &queues.transparent,
            &renderer.material_system,
            &view.view_buffer,
            &renderer.light_buffer,
        );
    }

    // 12. Swap HZB for next frame.
    renderer.culling.end_frame();

    // 13. Dynamic resolution feedback.
    if let Some(gpu_ms) = renderer.query_gpu_time()
    {
        renderer.drs.update(
            gpu_ms,
            &renderer.drs_state,
        );
    }

    // 14. Submit and present.
    renderer.submit().await;
    renderer.present().await;
}
```

### Instance Buffer Layout

The instance buffer is a contiguous GPU buffer of
`InstanceGpu` structs, indexed by a flat instance
ID. The culling pipeline reads from this buffer;
the batch compactor writes compacted index lists
referencing back into it.

```
Offset  Field                Size
------  -------------------  ----
0       world_matrix         64 B
64      prev_world_matrix    64 B
128     mesh_handle          4 B
132     material_id          4 B
136     instance_id          4 B
140     meshlet_offset       4 B
144     meshlet_count        4 B
148     lod_bias             4 B
152     flags                4 B
156     _padding             4 B
------                       ------
Total per instance:          160 B
```

### G-Buffer Layout (Deferred Path)

| Target | Format | Contents |
|--------|--------|----------|
| GBuffer0 | RGBA8_UNORM | Albedo (RGB) + Metallic (A) |
| GBuffer1 | RGB10A2_UNORM | World Normal (RGB) + Roughness remapped (A) |
| GBuffer2 | RG16_FLOAT | Motion Vectors (RG) |
| Depth | D32_FLOAT | Reverse-Z depth |

The deferred lighting pass reads all G-buffer
targets plus the light buffer and outputs the lit
result to an HDR render target.

### Multi-View Rendering Flow

Shadow cascades, VR eyes, scene captures, and
split-screen views all share the extracted proxy
store. Each view gets its own ViewUniform, culling
pass, and draw submission but reads from the same
instance buffer.

```mermaid
flowchart TD
    EXT[Extract Proxies] --> IB[Instance Buffer]
    IB --> V1[Main Camera View]
    IB --> V2[Shadow Cascade 0]
    IB --> V3[Shadow Cascade 1]
    IB --> V4[Scene Capture]
    IB --> V5[VR Left Eye]
    IB --> V6[VR Right Eye]
    V1 --> CULL1[Cull + Draw]
    V2 --> CULL2[Cull + Draw]
    V3 --> CULL3[Cull + Draw]
    V4 --> CULL4[Cull + Draw]
    V5 --> CULL5[Cull + Draw]
    V6 --> CULL6[Cull + Draw]
```

## Platform Considerations

### Rendering Path Defaults

| Platform | Default Path | Rationale |
|----------|-------------|-----------|
| Mobile | Forward+ | G-buffer bandwidth prohibitive on tile-based GPUs |
| Switch (handheld) | Forward+ | Bandwidth constrained; 720p native |
| Switch (docked) | Forward+ | Thin G-buffer possible but forward preferred |
| Desktop | Deferred | High light counts; G-buffer bandwidth is acceptable |
| High-end | Deferred | Full G-buffer with 4+ targets |
| VR | Forward+ | Latency-sensitive; single-pass instanced stereo |

### Culling Pipeline Scaling

| Platform | HZB Resolution | Phase 2 | Max Meshlets |
|----------|---------------|---------|-------------|
| Mobile | Quarter | Skippable | 100k |
| Switch (handheld) | Half | Active | 250k |
| Switch (docked) | Half | Active | 500k |
| Desktop | Full | Active | 1M |
| High-end | Full | Active | 4M+ |

### Light Limits Per Tile

| Platform | Max Lights/Tile | Total Active |
|----------|----------------|-------------|
| Mobile | 16 | 64 |
| Switch | 32 | 128 |
| Desktop | 256 | 2048 |
| High-end | Unlimited | Unlimited (stochastic) |

### Indirect Draw Requirements

| Backend | API | Minimum Version |
|---------|-----|----------------|
| Metal | `drawIndexedPrimitives(indirectBuffer:)` | Metal GPU Family 3+ |
| D3D12 | `ExecuteIndirect` | Feature Level 12.0 |
| Vulkan | `vkCmdDrawIndexedIndirect` | Vulkan 1.1 |

### Dynamic Resolution Bounds

| Platform | Min Scale | Max Scale | Target |
|----------|----------|----------|--------|
| Mobile | 50% | 75% | 33 ms (30 fps) |
| Switch (handheld) | 50% | 100% | 33 ms (30 fps) |
| Switch (docked) | 60% | 100% | 16 ms (60 fps) |
| Desktop | 67% | 100% | 16 ms (60 fps) |
| High-end | 50% | 100% | 8-16 ms |

### Depth Buffer Configuration

| Backend | Depth Format | Clear Value (Reverse-Z) |
|---------|-------------|------------------------|
| Metal | `depth32Float` | 0.0 |
| D3D12 | `DXGI_FORMAT_D32_FLOAT` | 0.0 |
| Vulkan | `VK_FORMAT_D32_SFLOAT` | 0.0 |

All backends use reverse-Z with depth cleared to
0.0 (far plane). The near plane maps to depth 1.0.
Depth comparison uses `GREATER` instead of `LESS`.

### Shader Compilation Pipeline

| Stage | Tool | Input | Output |
|-------|------|-------|--------|
| Author | HLSL source | `.hlsl` files | N/A |
| Compile (D3D12) | DXC via cxx.rs | HLSL | DXIL bytecode |
| Compile (Vulkan) | DXC via cxx.rs | HLSL | SPIR-V bytecode |
| Compile (Metal) | Metal Shader Converter via cxx.rs | DXIL | MSL / metallib |

### Proposed Dependencies

| Crate | Purpose | Justification |
|-------|---------|---------------|
| `bitflags` | `InstanceFlags`, `ShaderFeatures`, `RenderLayerMask` | Standard Rust bitflag pattern |
| `bytemuck` | Safe transmute for GPU buffer types (`Pod`, `Zeroable`) | Zero-cost GPU buffer mapping |
| `smallvec` | Inline-allocated light lists, face lists | Avoids heap allocation in hot paths |

## Test Plan

### Unit Tests

| Test | Req | Description |
|------|-----|-------------|
| `test_sort_key_opaque_ordering` | R-2.3.7 | Verify opaque sort keys order by pipeline, then material, then front-to-back depth. |
| `test_sort_key_transparent_ordering` | R-2.3.7, R-2.4.5 | Verify transparent sort keys order back-to-front. |
| `test_perspective_reverse_z_near_far` | R-2.3.6 | Near plane maps to depth 1.0, far maps to 0.0. Infinite far produces valid matrix. |
| `test_orthographic_reverse_z` | R-2.3.5 | Orthographic projection produces linear depth in [0, 1] with reverse mapping. |
| `test_frustum_plane_extraction` | R-2.3.2 | Extracted planes correctly classify known inside/outside points. |
| `test_render_layer_mask_filtering` | R-2.3.7 | Entities with non-intersecting layer masks are excluded from extraction. |
| `test_instance_flags_packing` | R-2.3.13 | `InstanceFlags` round-trips through u32 correctly. Two-sided flag skips backface cull. |
| `test_drs_convergence` | NFR-2.3.2 | DRS converges within 5 frames of a step-change in GPU load. No oscillation > 5%. |
| `test_drs_clamp_bounds` | R-2.3.11 | DRS scale never exceeds max or falls below min. |
| `test_material_permutation_cache` | R-2.4.3 | Same PermutationKey returns the same cached PipelineState. |
| `test_bindless_alloc_free_reuse` | R-2.10.8 | Freed BindlessIndex is recycled on next allocation. |
| `test_batch_compaction_count` | NFR-2.3.3 | 10,000 instances across 10 materials produce exactly 10 indirect draws. |
| `test_view_uniform_struct_size` | R-2.3.6 | `ViewUniform` size matches expected GPU layout (std140/std430). |
| `test_light_gpu_struct_alignment` | R-2.3.1 | `LightGpu` is 16-byte aligned and matches HLSL `cbuffer` layout. |
| `test_two_sided_skips_backface_cull` | R-2.3.3 | Meshlets flagged two-sided bypass normal cone test. |
| `test_alpha_cutout_in_shadow_pass` | R-2.3.13 | Alpha-cutout geometry generates shadow draw commands. |

### Integration Tests

| Test | Req | Description |
|------|-----|-------------|
| `test_forward_deferred_parity` | R-2.3.1 | Same scene rendered via forward+ and deferred produces pixel-identical lighting (within FP tolerance). |
| `test_frustum_cull_gpu_vs_cpu` | R-2.3.2 | GPU frustum cull results match CPU reference for 10,000 meshlets at 1-degree camera sweeps. |
| `test_hzb_two_phase_no_popin` | R-2.3.4 | Fast camera pan revealing occluded geometry shows no single-frame pop-in. Phase 2 recovers geometry same frame. |
| `test_hzb_occlusion_reduction` | R-2.3.4 | Dense urban scene achieves >= 30% draw call reduction from occlusion culling. |
| `test_cubemap_face_seams` | R-2.3.9 | Dynamic cubemap faces produce seamless edges (no visible seam artifacts). |
| `test_scene_capture_same_frame` | R-2.3.10 | Scene capture texture is usable as material input in the same frame it was rendered. |
| `test_drs_under_load` | R-2.3.11 | Resolution decreases within 5 frames when scene exceeds budget, increases when load drops. |
| `test_reverse_z_cross_backend` | R-2.3.6 | Depth buffer clears to 0.0 and comparison uses GREATER on Metal, D3D12, and Vulkan. |
| `test_sss_profile_scatter` | R-2.3.12 | Skin SSS profile produces visible light transmission at shadow terminator. |
| `test_alpha_cutout_shadow_shape` | R-2.3.13 | Shadow cast by alpha-cutout quad matches the alpha mask shape. |

### GPU Benchmarks

| Benchmark | Target | Source |
|-----------|--------|--------|
| Culling pipeline (1M meshlets, 1080p) | < 1.0 ms GPU | NFR-2.3.1 |
| Batch compaction (10k instances) | < 0.1 ms GPU | NFR-2.3.3 |
| HZB build (1080p, full mip chain) | < 0.3 ms GPU | R-2.3.4 |
| Forward+ tile cull (256 lights) | < 0.5 ms GPU | R-2.4.1 |
| Deferred light pass (256 lights) | < 0.5 ms GPU | R-2.4.2 |
| DRS feedback loop overhead | < 0.01 ms CPU | NFR-2.3.2 |
| Instance buffer upload (50k) | < 0.5 ms CPU | R-2.10.3 |
| Material instance upload (1k dirty) | < 0.1 ms CPU | R-2.4.6 |

## Open Questions

1. **Clustered vs tiled forward+.** Tiled forward
   uses 2D screen tiles; clustered forward adds
   depth slices (froxels). Clustered reduces
   per-tile light count for scenes with depth
   variance but increases memory and dispatch
   complexity. Recommended: clustered for desktop,
   tiled for mobile. Needs profiling on target
   hardware.

2. **Meshlet LOD selection integration.** The
   meshlet DAG (F-3.1.1) supports continuous LOD
   selection on the GPU. The culling pipeline
   needs to integrate LOD cut selection before
   frustum culling. Should LOD selection be a
   separate compute pass or merged into the
   frustum cull dispatch?

3. **Transparent object limit.** Transparent
   objects are sorted CPU-side and drawn
   individually. At high transparent object
   counts (>1000) this becomes a CPU bottleneck.
   Order-independent transparency (F-2.4.18) can
   eliminate sorting but has high bandwidth cost.
   Need to determine the transition threshold.

4. **Shader permutation explosion.** With 10
   shading models x 10 feature flags x 2 render
   paths x N vertex formats, the permutation space
   is large. Strategies: uber-shader with runtime
   branching (simple but slower), compile-time
   specialization constants (smaller binaries,
   driver-specific perf), or lazy compilation
   (first-use hitching). Recommended: specialization
   constants on Vulkan/Metal, static permutations
   on D3D12.

5. **HZB temporal stability.** Using the previous
   frame's HZB for phase 1 introduces a one-frame
   lag. Fast camera motion can cause transient
   over-culling. Phase 2 mitigates this but adds
   GPU cost. Should phase 1 use camera-motion
   extrapolation to expand meshlet bounds
   conservatively?

6. **Multi-draw-indirect-count availability.**
   `DrawIndexedIndirectCount` (Vulkan) /
   `ExecuteIndirect` with count buffer (D3D12) /
   `drawIndexedPrimitives(indirectBuffer:
   indirectBufferOffset:)` (Metal) vary in
   availability. Need to determine fallback
   strategy for backends that lack GPU-driven
   draw count.

7. **VR single-pass instanced stereo.** VR views
   can share a single culling pass and use
   viewport instancing to render both eyes in one
   draw call. Requires `VK_KHR_multiview` /
   Metal viewport array / D3D12 view instancing.
   Need to confirm minimum hardware tier.
