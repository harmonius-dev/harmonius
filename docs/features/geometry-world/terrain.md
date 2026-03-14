# 3.2 — Terrain

## Heightfield Terrain

### F-3.2.1 Heightfield Terrain with Tile-Based Streaming

The terrain system uses a tile-based heightfield as its core representation. Each tile stores a 16-bit or 32-bit
height grid at configurable resolution, plus metadata for material layers, holes, and physics. Tiles stream in and
out based on camera proximity via platform-native async I/O. A residency manager tracks loaded, loading, and
evicted tiles with placeholder low-LOD fallbacks, ensuring seamless transitions during MMO zone traversal.

- **Requirements:** R-3.2.1
- **Dependencies:** None
- **Platform notes:** Uses platform-native async I/O (IOCP on Windows, GCD on macOS, io_uring on Linux)

## Virtual Texturing

### F-3.2.2 Virtual Texture Clipmap

Terrain surface materials use a virtual texture clipmap centered on the camera. The clipmap maintains nested
resolution rings so near-camera terrain has full material detail while distant terrain uses coarser pages. A GPU
feedback pass identifies which virtual texture pages are visible, and missing pages are transcoded and uploaded
asynchronously. This bounds VRAM usage regardless of world size.

- **Requirements:** R-3.2.2
- **Dependencies:** F-3.2.1
- **Platform notes:** Requires sparse/virtual texture support (VK_EXT_sparse_residency, D3D12 tiled resources)

## Terrain LOD

### F-3.2.3 CDLOD / Geometry Clipmap LOD

Terrain meshes use a clipmap-based LOD scheme (CDLOD) where nested rectangular rings of decreasing vertex density
surround the camera. Morphing zones at ring boundaries interpolate vertex heights to prevent visible seams.
Screen-space error drives ring sizing so that vertex density matches pixel resolution at every distance,
maintaining consistent visual quality across draw distances of tens of kilometers.

- **Requirements:** R-3.2.3
- **Dependencies:** F-3.2.1
- **Platform notes:** None

## Terrain Holes and Materials

### F-3.2.4 Terrain Hole Masking

Per-tile bitmask layers mark terrain regions as holes, allowing caves, tunnels, building entrances, and dungeon
transitions to punch through the heightfield. Hole masks are stored as 1-bit-per-vertex grids per tile. The
terrain mesh shader discards triangles that overlap hole regions, and the collision system mirrors these masks so
physics and navigation remain consistent.

- **Requirements:** R-3.2.4
- **Dependencies:** F-3.2.1
- **Platform notes:** None

### F-3.2.5 Splatmap Material Blending

Multi-layer terrain materials composited per pixel using weight splatmaps. Each material layer provides full PBR
textures (albedo, normal, roughness, height). Blending uses height-weighted transitions for natural results at
layer boundaries (e.g., rock emerging through soil). Up to 16 layers per tile are supported, with a runtime
indirection table selecting the active layer set.

- **Requirements:** R-3.2.5
- **Dependencies:** F-3.2.1, F-3.2.2
- **Platform notes:** None

## Terrain Collision

### F-3.2.6 Terrain Physics Collision

A dedicated terrain collision representation derived from the heightfield that supports efficient ray and shape
queries. The collision mesh mirrors the streaming tile grid and updates as tiles load. Collision LOD matches the
physics simulation's needs rather than the visual LOD, ensuring stable character movement and vehicle physics on
sloped or rough terrain across the entire MMO world.

- **Requirements:** R-3.2.6
- **Dependencies:** F-3.2.1
- **Platform notes:** None

## Large World Coordinates

### F-3.2.7 Large World Coordinate Support

All world-space positions use 64-bit floating-point coordinates with camera-relative rendering. Before GPU
submission, positions are rebased to a camera-origin coordinate frame using 32-bit floats, eliminating jitter at
large distances from the world origin. The coordinate system supports worlds spanning thousands of kilometers,
which is essential for seamless MMO continental maps and ocean traversal.

- **Requirements:** R-3.2.7
- **Dependencies:** None
- **Platform notes:** None
