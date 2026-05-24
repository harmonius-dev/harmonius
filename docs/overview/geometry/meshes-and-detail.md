# Meshes and Detail

Mesh hierarchies, level-of-detail systems, and geometric optimization.

## What it covers

- Static meshes: immobile geometry.
- Dynamic meshes: changing topology (cloth, destruction).
- Skeletal meshes: bone-driven geometry for characters.
- Mesh LOD chains: multiple detail levels per mesh.
- Screen coverage LOD: selecting LOD based on screen-space size.
- Streaming LOD: progressive mesh loading.
- Vertex data layouts: position, normal, tangent, UV, color attributes.
- Bounding volumes: AABB, OBB, sphere for culling and queries.
- Billboarding: flat 2D sprites replacing distant meshes.
- Imposters: pre-rendered 2D images of complex geometry.

## Concepts

### Mesh Types and Data Layout

Static meshes store immutable vertex and index data. Dynamic meshes rebuild topology per frame (cloth
tearing, destruction deformation). Skeletal meshes reference bone hierarchy; the GPU updates
vertex positions via bone weights. Vertex data includes position (XYZ), normal (surface direction),
tangent (texture direction), UV coordinates (texture lookup), and color. Vertex compression reduces
memory: quantizing positions to 16-bit, normals to oct-encoded 16-bit.

### Level of Detail Chains

Each mesh defines LOD levels 0 (highest detail), 1, 2 (lowest detail). Each LOD uses fewer
vertices and triangles. The renderer selects LOD based on screen-space size: large objects use LOD
0, distant objects use LOD 2. Blending between LODs hides popping. Streaming progressively loads
LOD 0 while rendering LOD 1, eliminating frame hitches during mesh loading.

### Bounding Volumes

Bounding volumes (AABB, sphere, OBB) enable culling: if a bounding volume is off-screen, skip
rendering. Frustum culling checks if bounding volume intersects camera frustum. Occlusion culling
skips meshes occluded behind other geometry. Raycasts and proximity queries test bounding volumes
first (cheap test), then detailed mesh geometry (expensive test).

### Billboarding and Imposters

Billboarding renders a 2D quad (two triangles) facing the camera instead of complex 3D geometry. At
distance, billboards replace detailed meshes, dramatically reducing geometry. Imposters pre-render
complex geometry into a 2D image; distant rendering uses the imposters. Imposters must rotate to
face the camera, updating per-frame.

## How it fits

- See [terrain-and-foliage.md](./terrain-and-foliage.md) for terrain LOD and vegetation.
- See [water-and-sky.md](./water-and-sky.md) for procedural geometry.
- See [procedural-generation.md](./procedural-generation.md) for runtime mesh generation.
