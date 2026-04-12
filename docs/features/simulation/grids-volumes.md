# 17.2 — Grids and Volumes

## Uniform Grids

| ID       | Feature                       |
|----------|-------------------------------|
| F-17.2.1 | Generic UniformGrid Primitive|
| F-17.2.2 | Fast Propagation Kernel      |
| F-17.2.3 | Fast Line-of-Sight Queries   |
| F-17.2.4 | 2D Flow Field Computation    |
| F-17.2.5 | Influence Map Propagation    |

1. **F-17.2.1** — `UniformGrid<T>` is a 2D grid of typed cells with fixed cell size and axis-aligned
   bounds. Works with Transform2D in 2D games and Transform in 3D games. Replaces fog of war,
   tactical maps, height fields, scent grids, and influence maps with one type.
   - **Deps:** F-1.1.1, F-1.4.1
2. **F-17.2.2** — PropagationKernel defines cell-to-cell spread per tick for fire, fluid, scent, and
   influence. 256x256 grid propagation completes in under 1 ms per tick on desktop hardware.
   - **Deps:** F-17.2.1, F-14.3.1 (Job System)
3. **F-17.2.3** — Line-of-sight queries traverse up to 128 cells between two positions in under 0.01
   ms per query, enabling many LOS checks per frame for AI and fog reveal.
   - **Deps:** F-17.2.1
4. **F-17.2.4** — Flow fields compute directional vectors across uniform grids via Dijkstra
   propagation from goal positions. RTS unit movement follows the flow field without per-unit A*.
   - **Deps:** F-17.2.1
5. **F-17.2.5** — Influence maps propagate values from source entities across grid cells with decay
   rate per step and obstacle blocking. Drives threat zones and resource attraction.
   - **Deps:** F-17.2.1, F-17.2.2

## Voxel Volumes

| ID       | Feature                       |
|----------|-------------------------------|
| F-17.2.6 | VoxelVolume with Palette Chunks|
| F-17.2.7 | Fast Voxel Raycast            |
| F-17.2.8 | Signed Distance Field         |

1. **F-17.2.6** — `VoxelVolume<T>` stores 3D typed cells in chunks with palette compression for
   blocks. Used only for 3D games; 2D games stay on UniformGrid.
   - **Deps:** F-1.1.1, F-1.4.1
2. **F-17.2.7** — Voxel raycasts traverse up to 512 voxels in under 2 ms on desktop hardware,
   supporting block placement, destruction, and shooting rays.
   - **Deps:** F-17.2.6
3. **F-17.2.8** — Signed distance fields compute from voxel volume surfaces, scoped to dirty
   regions. SDFs feed soft shadows, ambient occlusion, and collision queries.
   - **Deps:** F-17.2.6

## GPU Sync

| ID        | Feature                        |
|-----------|--------------------------------|
| F-17.2.9  | GPU Dirty Region Upload       |
| F-17.2.10 | GPU Compute Grid Propagation  |

1. **F-17.2.9** — Dirty regions of uniform grids upload to the GPU within 1 ms per tick, keeping
   renderer overlays synced within one frame.
   - **Deps:** F-17.2.1, F-2.2 (GPU Abstraction)
2. **F-17.2.10** — Large-scale grids (1024x1024+) offload propagation to GPU compute shaders when
   CPU budget is exceeded. Results match CPU baseline within floating-point tolerance.
   - **Deps:** F-17.2.1, F-2.2

## Hierarchical Grids

| ID        | Feature                        |
|-----------|--------------------------------|
| F-17.2.11 | Multi-Resolution HierarchicalGrid|

1. **F-17.2.11** — HierarchicalGrid provides multi-resolution cells for large worlds requiring
   variable detail. Coarse cells subdivide into finer cells on demand near the camera.
   - **Deps:** F-17.2.1

## Area of Interest

| ID        | Feature                        |
|-----------|--------------------------------|
| F-17.2.12 | Networking Relevancy Grid     |

1. **F-17.2.12** — A dedicated uniform grid of entity sets provides networking area-of- interest
   filtering. The replication system queries nearby cells to determine relevancy per observer
   without global BVH scans.
   - **Deps:** F-17.2.1, F-8.2.1 (State Replication)
