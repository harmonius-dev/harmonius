# Rendering Core Design

Scene rendering, draw calls, culling, shadows, and lighting -- the core rendering pipeline that
bridges ECS and GPU.

## Requirements Trace

> **Canonical sources:** Features, requirements, and user stories are defined in
> [features/](../../features/), [requirements/](../../requirements/), and
> [user-stories/](../../user-stories/).

### Scene Rendering (2.3)

| Feature  | Requirement | User Stories |
|----------|-------------|--------------|
| F-2.3.1  | R-2.3.1     | US-2.3.1.1, US-2.3.1.2, US-2.3.1.3 |
| F-2.3.2  | R-2.3.2     | US-2.3.2.1, US-2.3.2.2 |
| F-2.3.3  | R-2.3.3     | US-2.3.3.1, US-2.3.3.2 |
| F-2.3.4  | R-2.3.4     | US-2.3.4.1, US-2.3.4.2, US-2.3.4.3 |
| F-2.3.5  | R-2.3.5     | US-2.3.5.1, US-2.3.5.2 |
| F-2.3.6  | R-2.3.6     | US-2.3.6.1, US-2.3.6.2 |
| F-2.3.7  | R-2.3.7     | US-2.3.7.1, US-2.3.7.2, US-2.3.7.3 |
| F-2.3.8  | R-2.3.8     | US-2.3.8.1, US-2.3.8.2 |
| F-2.3.9  | R-2.3.9     | US-2.3.9.1, US-2.3.9.2 |
| F-2.3.10 | R-2.3.10    | US-2.3.10.1, US-2.3.10.2 |
| F-2.3.11 | R-2.3.11    | US-2.3.11.1, US-2.3.11.2, US-2.3.11.3 |
| F-2.3.12 | R-2.3.12    | US-2.3.12.1, US-2.3.12.2 |
| F-2.3.13 | R-2.3.13    | US-2.3.13.1, US-2.3.13.2 |

1. **F-2.3.1** -- Direct lighting: point, spot, directional
2. **F-2.3.2** -- GPU meshlet frustum culling via compute
3. **F-2.3.3** -- Meshlet backface culling via normal cones
4. **F-2.3.4** -- Two-phase HZB occlusion culling
5. **F-2.3.5** -- Orthographic camera projection
6. **F-2.3.6** -- Perspective projection with reverse-Z
7. **F-2.3.7** -- GPU-driven instancing and batch compaction
8. **F-2.3.8** -- Render-to-texture with automatic barriers
9. **F-2.3.9** -- Static and dynamic cubemap rendering
10. **F-2.3.10** -- Scene capture (planar and cubemap)
11. **F-2.3.11** -- Dynamic resolution scaling with GPU feedback
12. **F-2.3.12** -- Subsurface scattering (screen-space and RT)
13. **F-2.3.13** -- Alpha mask cutout geometry

### Render Pipeline (2.10)

| Feature  | Requirement | User Stories |
|----------|-------------|--------------|
| F-2.10.1 | R-2.10.1    | US-2.10.1.1, US-2.10.1.2 |
| F-2.10.2 | R-2.10.2    | US-2.10.2.1, US-2.10.2.2 |
| F-2.10.3 | R-2.10.3    | US-2.10.3.1, US-2.10.3.2, US-2.10.3.3 |
| F-2.10.4 | R-2.10.4    | US-2.10.4.1, US-2.10.4.2 |
| F-2.10.5 | R-2.10.5    | US-2.10.5.1, US-2.10.5.2 |
| F-2.10.6 | R-2.10.6    | US-2.10.6.1, US-2.10.6.2 |
| F-2.10.7 | R-2.10.7    | US-2.10.7.1, US-2.10.7.2 |
| F-2.10.8 | R-2.10.8    | US-2.10.8.1, US-2.10.8.2 |
| F-2.10.9 | R-2.10.9    | US-2.10.9.1, US-2.10.9.2 |
| F-2.10.10 | R-2.10.10  | US-2.10.10.1, US-2.10.10.2 |

1. **F-2.10.1** -- Render proxy extraction via immutable queries
2. **F-2.10.2** -- SoA proxy components for GPU upload
3. **F-2.10.3** -- Dirty-flag incremental updates, O(changed)
4. **F-2.10.4** -- View/camera registration with projection
5. **F-2.10.5** -- Multi-view rendering from single snapshot
6. **F-2.10.6** -- Per-view draw lists with sort keys
7. **F-2.10.7** -- GPU compute batch compaction
8. **F-2.10.8** -- Bindless material parameter binding
9. **F-2.10.9** -- Debug draw API, compile-time gated
10. **F-2.10.10** -- Buffer visualization modes

### Lighting System (2.4)

| Feature  | Requirement | User Stories |
|----------|-------------|--------------|
| F-2.4.1  | R-2.4.1, NFR-2.4.1 | US-2.4.1.1, US-2.4.1.2 |
| F-2.4.2  | R-2.4.2     | US-2.4.2.1, US-2.4.2.2 |
| F-2.4.10 | R-2.4.10    | US-2.4.10.1, US-2.4.10.2 |
| F-2.4.11 | R-2.4.11    | US-2.4.11.1, US-2.4.11.2 |
| F-2.4.12 | R-2.4.12    | US-2.4.12.1, US-2.4.12.2 |
| F-2.4.13 | R-2.4.13    | US-2.4.13.1, US-2.4.13.2 |
| F-2.4.14 | R-2.4.14    | US-2.4.14.1, US-2.4.14.2 |
| F-2.4.15 | R-2.4.15    | US-2.4.15.1, US-2.4.15.2 |
| F-2.4.16 | R-2.4.16    | US-2.4.16.1, US-2.4.16.2 |
| F-2.4.17 | R-2.4.17    | US-2.4.17.1, US-2.4.17.2 |
| F-2.4.18 | R-2.4.18    | US-2.4.18.1, US-2.4.18.2 |
| F-2.4.19 | R-2.4.19    | US-2.4.19.1, US-2.4.19.2 |
| F-2.4.20 | R-2.4.20    | US-2.4.20.1, US-2.4.20.2 |
| F-2.4.21 | R-2.4.21    | US-2.4.21.1, US-2.4.21.2 |
| F-2.4.22 | R-2.4.22    | US-2.4.22.1, US-2.4.22.2 |
| F-2.4.23 | R-2.4.23    | US-2.4.23.1, US-2.4.23.2 |

1. **F-2.4.1** -- Tiled/clustered forward+ light culling
2. **F-2.4.2** -- Deferred lighting via G-buffer
3. **F-2.4.10** -- Stochastic many-light sampling
4. **F-2.4.11** -- Cascaded shadow maps
5. **F-2.4.12** -- Soft shadows: PCF, PCSS, RT
6. **F-2.4.13** -- Ambient occlusion: SSAO, GTAO, RT AO
7. **F-2.4.14** -- Virtual shadow maps
8. **F-2.4.15** -- Contact shadows via ray march
9. **F-2.4.16** -- Distance field shadows
10. **F-2.4.17** -- Capsule shadows for skeletal meshes
11. **F-2.4.18** -- Moment-based OIT
12. **F-2.4.19** -- Volumetric shadow maps
13. **F-2.4.20** -- Area lights (rect/sphere) via LTC
14. **F-2.4.21** -- Sky light / IBL split-sum
15. **F-2.4.22** -- IES photometric light profiles
16. **F-2.4.23** -- Light functions (gobo/cookie)

### Non-Functional Requirements

| NFR | Target |
|-----|--------|
| NFR-2.3.1 | Culling < 1.0 ms GPU (1M meshlets, 1080p) |
| NFR-2.3.2 | DRS converges in 5 frames, < 5% oscillation |
| NFR-2.3.3 | 10 draw calls (10k instances, 10 materials) |
| NFR-2.10.1 | Extract < 2.0 ms full / < 0.5 ms dirty |
| NFR-2.10.2 | 500K proxies/ms/core draw list throughput |
| NFR-2.10.3 | Zero debug viz overhead in shipping |
| NFR-2.4.1 | 500+ dynamic lights, sub-linear scaling |
| NFR-2.4.2 | Shadow atlas <= 256 MB VRAM |
| NFR-2.4.3 | BRDF eval < 0.1 ms / 1M fragments |

## Overview

The core rendering subsystem bridges ECS and GPU via an **extract-prepare-render** pipeline that
decouples simulation from rendering.

Four principles:

1. **ECS is truth.** All render state as components. Renderer reads via immutable queries, owns no
   persistent scene data.
2. **Decoupled snapshot.** Extraction copies only changed data. Simulation advances once extraction
   completes.
3. **GPU-driven pipeline.** Frustum, backface, occlusion culling, LOD, and instancing all run as GPU
   compute passes.
4. **Dual rendering paths.** Forward+ and deferred share culling, light buffer, and material system.
   Selection is per-view.

## Architecture

### Module Boundaries

```mermaid
graph TD
    subgraph core["rendering::core"]
        EXT[RenderExtractor]
        PROXY[ProxyStore]
        CULL[GpuCullingPipeline]
        BATCH[BatchCompactor]
        QUEUE[RenderQueueSorter]
        DRAW[DrawCommandEncoder]
        DRS[DynamicResolution]
    end

    subgraph lighting["rendering::lighting"]
        LB[LightBuffer]
        FWD["Forward+ Tiled"]
        DEF[Deferred G-Buffer]
        SM[ShadowMapping]
        AO[AmbientOcclusion]
    end

    subgraph ecs["ECS World"]
        MC[MeshComponent]
        LC[LightComponent]
        CC[CameraComponent]
    end

    MC --> EXT
    LC --> LB
    CC --> EXT
    EXT --> PROXY
    PROXY --> CULL
    CULL --> BATCH
    BATCH --> QUEUE
    QUEUE --> DRAW
    LB --> FWD
    LB --> DEF
    SM --> DEF
    AO --> DEF
```

### Extract-Prepare-Render Pipeline

```mermaid
sequenceDiagram
    participant SIM as Simulation
    participant EX as Extract
    participant PREP as Prepare
    participant REN as Render
    participant GPU as GPU

    SIM->>EX: Immutable ECS queries
    EX->>EX: Copy changed to proxy store
    EX->>PREP: ProxyStore handoff
    PREP->>PREP: View setup + frustum cull
    PREP->>PREP: Build and sort draw lists
    PREP->>PREP: GPU batch compaction
    PREP->>REN: Compiled draw lists
    REN->>REN: Parallel command encoding
    REN->>GPU: Submit command buffers
```

### GPU Culling Pipeline

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
    E --> J[Batch Compaction]
    I --> J
    J --> K[Indirect Draw Buffer]
```

### Light Culling Pipeline

```mermaid
flowchart TD
    A[ECS Light Query] --> B[Extract Active Lights]
    B --> C[Frustum Cull via Spatial Index]
    C --> D[Upload GpuLightBuffer]
    D --> E{Rendering Path}
    E -->|Forward+| F[Cluster Assignment]
    E -->|Deferred| G[G-Buffer Accumulation]
    F --> H[Per-Cluster Light Lists]
    H --> I[Forward Fragment Shader]
    G --> J[Fullscreen Compute]
    I --> K[Lit Output]
    J --> K
```

### Core Class Diagram

```mermaid
classDiagram
    class MeshComponent {
        +mesh_handle MeshHandle
        +meshlet_count u32
        +lod_bias f32
    }

    class MaterialComponent {
        +material_id MaterialId
        +instance_id MaterialInstanceId
    }

    class CameraComponent {
        +projection Projection
        +render_path RenderPath
        +clear_color LinearColor
        +render_order i32
    }

    class VisibilityComponent {
        +visible bool
        +render_layers RenderLayerMask
        +cast_shadows bool
        +two_sided bool
    }

    class DynamicResolutionState {
        +scale f32
        +min_scale f32
        +max_scale f32
        +target_ms f32
    }

    class SceneCaptureComponent {
        +capture_mode CaptureMode
        +resolution UVec2
        +target_texture TextureHandle
        +update_interval u32
    }

    class Projection {
        Perspective
        Orthographic
    }

    class RenderPath {
        ForwardPlus
        Deferred
    }

    class CaptureMode {
        Planar
        Cubemap
    }

    class ProxyStore {
        -transforms Vec~Mat4~
        -prev_transforms Vec~Mat4~
        -mesh_ids Vec~MeshId~
        -material_ids Vec~MaterialId~
        -bounds Vec~Aabb~
        -flags Vec~ProxyFlags~
        -entity_map HashMap
        +insert(entity, ...) ProxyIndex
        +update_transform(index, transform)
        +remove(entity) Option~ProxyIndex~
        +len() u32
    }

    class ProxyFlags {
        +ACTIVE u32
        +DIRTY_TRANSFORM u32
        +DIRTY_MESH u32
        +CAST_SHADOW u32
    }

    class RenderView {
        +view_id ViewId
        +view_matrix Mat4
        +projection Mat4
        +viewport Viewport
        +platform_tier PlatformTier
        +view_type ViewType
        +visibility_bits BitVec
    }

    class ViewType {
        MainCamera
        ShadowCascade
        ReflectionProbe
        SplitScreen
        VrEye
    }

    class SortKey {
        +bits u64
        +opaque(phase, pipe, mat, depth) Self
        +transparent(phase, pipe, mat, depth) Self
    }

    class RenderPhase {
        Opaque
        AlphaTest
        Transparent
        UI
        Debug
    }

    class DrawCommand {
        +proxy_index ProxyIndex
        +mesh_id MeshId
        +material_id MaterialId
        +instance_data_offset u32
        +lod_level u8
    }

    class DrawList {
        +phase RenderPhase
        +push(cmd, key)
        +sort()
        +len() usize
    }

    class ViewUniform {
        +view Mat4
        +projection Mat4
        +view_projection Mat4
        +camera_position Vec4
        +viewport_size Vec2
        +frustum_planes Vec4[6]
    }

    class RenderWorld {
        +proxies ProxyStore
        +views Vec~RenderView~
        +draw_lists DrawListSet
        +instance_buffer InstanceBuffer
        +frame_index u64
    }

    MeshComponent --> MaterialComponent
    MeshComponent --> VisibilityComponent
    CameraComponent --> DynamicResolutionState
    ProxyStore *-- ProxyFlags
    RenderView --> ViewType
    DrawList --> DrawCommand
    DrawCommand --> SortKey
    SortKey --> RenderPhase
    RenderWorld *-- ProxyStore
    RenderWorld *-- RenderView
    RenderWorld *-- DrawList
```

### Lighting Class Diagram

```mermaid
classDiagram
    class DirectionalLight {
        +color LinearColor
        +intensity f32
        +cast_shadows bool
        +shadow_config CsmConfig
        +light_function Option~LightFunctionHandle~
    }

    class PointLight {
        +color LinearColor
        +intensity f32
        +radius f32
        +cast_shadows bool
        +shadow_config PointShadowConfig
        +ies_profile Option~IesProfileHandle~
    }

    class SpotLight {
        +color LinearColor
        +intensity f32
        +range f32
        +inner_angle f32
        +outer_angle f32
        +cast_shadows bool
        +shadow_config SpotShadowConfig
    }

    class AreaLight {
        +color LinearColor
        +intensity f32
        +shape AreaLightShape
        +two_sided bool
        +range f32
    }

    class AreaLightShape {
        Rect
        Disc
        Sphere
    }

    class ShadowFilter {
        Hard
        Pcf
        Pcss
        Vsm
        RayTraced
    }

    class ShadowAtlas {
        -tiles Vec~ShadowAtlasTile~
        -texture GpuTexture
        +allocate(resolution) Option~TileId~
        +release(tile_id)
        +evict_lru(budget)
        +texture_view() GpuTextureView
    }

    class ClusterGrid {
        -tile_count_x u32
        -tile_count_y u32
        -slice_count u32
        +dispatch_assignment(lights, view)
        +bind_for_shading() ClusterBindGroup
    }

    class GpuLightBuffer {
        -buffer GpuBuffer
        -count u32
        +upload(lights)
        +bind() GpuBufferView
    }

    class AoTier {
        Off
        Ssao
        Gtao
        RayTraced
    }

    class AoConfig {
        +tier AoTier
        +sample_count u8
        +radius f32
        +intensity f32
    }

    class SkyLight {
        +cubemap Option~CubemapHandle~
        +diffuse_intensity f32
        +specular_intensity f32
    }

    class IesProfileHandle {
        +0 u32
    }

    class LightFunctionHandle {
        +0 u32
    }

    class LightProbe {
        +sh_coefficients LinearColor[9]
        +radius f32
        +priority u8
    }

    class ReflectionProbe {
        +cubemap CubemapHandle
        +influence_aabb Aabb
        +blend_aabb Aabb
        +refresh ProbeRefreshMode
    }

    class ProbeRefreshMode {
        Baked
        OnLoad
        Periodic
        OnChange
    }

    class MaterialId {
        +0 u32
    }

    class ShadingModel {
        DefaultLit
        Subsurface
        ClearCoat
        Cloth
        Hair
        Eye
        ThinTranslucent
        Foliage
        Unlit
    }

    class PermutationKey {
        +shading_model ShadingModel
        +features ShaderFeatures
        +render_path RenderPath
    }

    DirectionalLight --> ShadowFilter
    PointLight --> ShadowFilter
    SpotLight --> ShadowFilter
    AreaLight --> AreaLightShape
    ClusterGrid --> GpuLightBuffer
    ShadowAtlas --> DirectionalLight
    ReflectionProbe --> ProbeRefreshMode
```

### Sort Key Bit Layout

| Field | Bits | Width | Purpose |
|-------|------|-------|---------|
| Translucency | 63 | 1 | 0=opaque, 1=transparent |
| Phase | 56-62 | 7 | Render phase ordinal |
| Pipeline | 40-55 | 16 | PSO hash |
| Material | 24-39 | 16 | Material ID |
| Depth | 0-23 | 24 | Quantized depth |

## Data Flow

### Per-Frame Lighting Pipeline

1. **Light extraction** (CPU) -- Query components, frustum cull, pack `GpuLight`, sort by screen
   contribution
2. **GPU light buffer upload** -- SSBO write
3. **Shadow scheduling** -- Allocate atlas tiles per light
4. **Shadow rendering** (GPU) -- CSM, cubemap, spot passes
5. **Cluster assignment** (GPU compute) -- Build 3D cluster grid
6. **AO pass** (GPU compute) -- SSAO/GTAO/RT AO by tier
7. **Lighting evaluation** (GPU) -- Forward+ or deferred path

### Shadow Configuration Per Platform

| Platform | CSM | Resolution | Atlas | VSM |
|----------|-----|-----------|-------|-----|
| Mobile | 1-2 | 512 | 2048 | No |
| Switch | 2-3 | 1024 | 4096 | No |
| Desktop | 3-4 | 2048 | 8192 | Yes |
| High-end | 4 | 4096 | 16384 | Yes |

### AO Configuration Per Platform

| Platform | Mode | Samples | Resolution |
|----------|------|---------|-----------|
| Mobile | SSAO | 4 | Quarter |
| Switch | SSAO/GTAO | 8 | Half |
| Desktop | GTAO | 16 | Full |
| High-end | RT AO | N/A | Full |

## Platform Considerations

### Light Feature Availability

| Feature | Mobile | Switch | Desktop | High-end |
|---------|--------|--------|---------|----------|
| Forward+ | Yes (8) | Yes (16) | Yes (24) | Yes (24) |
| Deferred | No | Thin G-buf | Full | Full |
| Area lights | Point fallback | Rect only | Full LTC | Unlimited |
| IES profiles | 1D 64 | 1D 128 | 2D 256 | 2D 256+ |
| Light functions | Baked | Scalar | Full color | Full color |
| Stochastic | Disabled | Disabled | 1-2 spp | 4+ spp |
| VSM | Disabled | Disabled | 8k | 16k |
| Contact shadows | No | Dir only | All | All |
| MBOIT | Disabled | Disabled | Half-res | Full-res |

## Test Plan

See companion file [rendering-core-test-cases.md](rendering-core-test-cases.md).

### Unit Tests (Scene Rendering)

| Test | Req |
|------|-----|
| `test_proxy_insert_remove` | R-2.10.1 |
| `test_proxy_dirty_incremental` | R-2.10.3 |
| `test_frustum_cull_aabb` | R-2.3.2 |
| `test_backface_cull_normal_cone` | R-2.3.3 |
| `test_hzb_phase1_phase2` | R-2.3.4 |
| `test_reverse_z_perspective` | R-2.3.6 |
| `test_orthographic_projection` | R-2.3.5 |
| `test_sort_key_opaque_ordering` | R-2.10.6 |
| `test_sort_key_transparent_bft` | R-2.10.6 |
| `test_batch_compaction_groups` | R-2.3.7 |
| `test_drs_convergence` | R-2.3.11, NFR-2.3.2 |
| `test_cubemap_capture` | R-2.3.9 |
| `test_alpha_cutout_depth` | R-2.3.13 |
| `test_debug_draw_stripped` | NFR-2.10.3 |

### Unit Tests (Lighting)

| Test | Req |
|------|-----|
| `test_cluster_grid_dimensions` | R-2.4.1 |
| `test_cluster_assignment_500` | NFR-2.4.1 |
| `test_shadow_atlas_alloc_release` | R-2.4.11 |
| `test_shadow_atlas_lru` | NFR-2.4.2 |
| `test_csm_split_computation` | R-2.4.11 |
| `test_csm_temporal_stabilization` | R-2.4.11 |
| `test_pcf_filter_4_tap` | R-2.4.12 |
| `test_pcss_penumbra_scales` | R-2.4.12 |
| `test_ssao_half_res_output` | R-2.4.13 |
| `test_gtao_bent_normals` | R-2.4.13 |
| `test_ies_parse_valid` | R-2.4.22 |
| `test_area_light_ltc_energy` | R-2.4.20 |
| `test_sky_light_irradiance` | R-2.4.21 |
| `test_gpu_light_pack_alignment` | R-2.4.1 |
| `test_vsm_page_alloc_evict` | R-2.4.14 |
| `test_contact_shadow_step_count` | R-2.4.15 |
| `test_stochastic_convergence` | R-2.4.10 |

### Integration Tests

| Test | Req |
|------|-----|
| `test_extract_100k_entities` | NFR-2.10.1 |
| `test_draw_list_500k_proxies` | NFR-2.10.2 |
| `test_forward_plus_500_lights` | NFR-2.4.1 |
| `test_deferred_matches_forward` | R-2.4.2 |
| `test_shadow_atlas_20_lights` | NFR-2.4.2 |
| `test_tier_fallback_mobile` | All |
| `test_cross_backend_shadow` | R-2.4.11 |

### Benchmarks

| Benchmark | Target |
|-----------|--------|
| Culling (1M meshlets, 1080p) | < 1.0 ms GPU |
| Extraction (100K full) | < 2.0 ms |
| Extraction (1% dirty) | < 0.5 ms |
| Draw list throughput | 500K proxies/ms/core |
| Cluster assignment (500 lights) | < 0.5 ms |
| BRDF eval (1M fragments) | < 0.1 ms |
| CSM 4-cascade (2048 each) | < 2.0 ms |
| GTAO full-res 1080p | < 1.0 ms |

## Open Questions

1. **Cluster tile size** -- 64 px standard vs 32 px for fewer overflows. Needs target hardware
   profiling.
2. **Dual-paraboloid vs cubemap on mobile** -- 2 faces vs 6 with warping artifacts. Quality/cost
   tradeoff evaluation.
3. **VSM cache eviction** -- LRU may thrash on rotation. Priority-weighted by screen coverage under
   consideration.
4. **Shadow atlas fragmentation** -- Consider buddy allocator for power-of-two tile sizes in long
   sessions.
5. **V-buffer alternative** -- Once mesh shaders are ubiquitous, migrating to visibility buffer
   would unify lighting paths.
