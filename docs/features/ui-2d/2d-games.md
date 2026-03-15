# 10.5 — 2D Game Support

## Sprite Rendering

### F-10.5.1 Sprite Rendering and Sprite Sheets

Render 2D sprites as instanced textured quads with per-sprite position, rotation, scale, UV
rect, atlas index, tint color, and z-order. Sprite sheets pack multiple frames into atlas
textures with metadata defining frame boundaries, pivot points, and trim rects. The sprite
renderer batches sprites by atlas page and blend mode, sorting by z-order within each batch,
to minimize draw calls for scenes with thousands of sprites.

- **Requirements:** R-10.5.1
- **Dependencies:** F-10.4.1
- **Platform notes:** None

### F-10.5.2 Frame-Based Sprite Animation

Frame-based sprite animation with configurable playback rate, looping modes (loop, ping-pong,
one-shot, reverse), and animation events (trigger callbacks at specific frames for footstep
sounds, hit detection). Supports animation blending and transition graphs for smooth state
changes. Animation clips reference frame ranges within a sprite sheet atlas.

- **Requirements:** R-10.5.2
- **Dependencies:** F-10.5.1
- **Platform notes:** None

### F-10.5.3 2D Skeletal Animation

Bone-based 2D animation with mesh deformation for characters requiring smoother motion than
frame-by-frame provides. Skeletons define bone hierarchies with per-bone transform keyframes.
Mesh vertices are skinned to bones with blend weights. Supports IK constraints (two-bone),
bone-driven sprite swapping (change weapon sprite based on equipment), and runtime bone
manipulation for procedural effects (look-at, recoil). Compatible with Spine and DragonBones
import formats.

- **Requirements:** R-10.5.3
- **Dependencies:** F-10.5.1, F-9.1.1 (Skeletal Animation)
- **Platform notes:** None

## Vector Graphics

### F-10.5.4 Vector-Based 2D Rendering

Render 2D shapes from vector paths (Bezier curves, arcs, lines) at any resolution without
pixelation. Paths support fill (solid, gradient, pattern), stroke (width, cap, join, dash),
and clipping masks. Vector shapes are tessellated to GPU triangles at render time or cached
when static. Used for resolution-independent UI, clean-line art games, map overlays, and
scalable game elements that work across mobile and desktop resolutions.

- **Requirements:** R-10.5.4
- **Dependencies:** F-10.4.3 (Vector Graphics)
- **Platform notes:** None

### F-10.5.5 Vector Skeletal Animation

Combine vector-based rendering with skeletal animation — bones deform vector path control
points rather than rasterized mesh vertices. Characters and objects maintain crisp edges at
any zoom level while animating smoothly. Supports the same bone hierarchy, IK, and animation
graph features as raster skeletal animation (F-10.5.3). Ideal for stylized 2D games with
zoom mechanics (strategy, puzzle, narrative).

- **Requirements:** R-10.5.5
- **Dependencies:** F-10.5.4, F-10.5.3
- **Platform notes:** None

## Tilemaps

### F-10.5.6 Tilemap Rendering

Render large tile-based worlds using chunked tile grids where each chunk is a fixed-size block
of tiles mapped to atlas UVs via a compute dispatch. Supports multiple tile layers (ground,
decoration, collision overlay), auto-tiling rules for terrain transitions, animated tiles, and
per-tile flip/rotation flags. Tilemap chunks are streamed and culled to the camera viewport,
enabling worlds with millions of tiles.

- **Requirements:** R-10.5.6
- **Dependencies:** F-10.5.1
- **Platform notes:** Mobile limits visible tile layers (3 vs 8 on desktop) and uses smaller
  tile atlas pages to reduce GPU memory pressure.

### F-10.5.7 Isometric and Hex Tilemap Support

Extend the tilemap system with isometric (diamond and staggered) and hexagonal grid layouts.
Provide coordinate conversion between screen space and tile space for each grid type, correct
depth sorting by tile row and column, and height-stacked tiles for multi-level isometric maps.
Supports isometric wall occlusion and z-fighting resolution for overlapping tile layers.

- **Requirements:** R-10.5.7
- **Dependencies:** F-10.5.6
- **Platform notes:** None

### F-10.5.8 Procedural 2D Tilemap Generation

Generate 2D tilemaps procedurally using WFC (F-3.6.20), noise-based terrain generation,
cellular automata (cave systems), BSP dungeon generation, and room-and-corridor algorithms.
Generation rules are authored as tileset assets with adjacency constraints. Runtime generation
produces explorable roguelike dungeons, infinite side-scrollers, and procedural overworlds.
Deterministic seeding (F-3.6.5) ensures reproducible worlds for multiplayer.

- **Requirements:** R-10.5.8
- **Dependencies:** F-10.5.6, F-3.6.20 (2D WFC), F-3.6.5 (Seeding)
- **Platform notes:** None

## Camera and Scrolling

### F-10.5.9 2D Camera System

Dedicated 2D camera with position, zoom, rotation, and viewport bounds (dead zones, soft
edges, camera shake). Parallax scrolling composites multiple background layers at different
scroll rates for depth illusion. Camera smoothing: lerp follow, look-ahead based on player
velocity, snap-to-grid, screen-edge push zones. Split-screen mode for local co-op — multiple
2D cameras render to separate viewports within the same window.

- **Requirements:** R-10.5.9
- **Dependencies:** F-10.5.1
- **Platform notes:** None

## 2D Physics

### F-10.5.10 2D Rigid Body Physics

Full 2D rigid body simulation as ECS components: `RigidBody2D`, `Velocity2D`, `Mass2D`,
`Collider2D`. Integration, collision detection (broadphase + narrowphase), and constraint
solving run as ECS systems in the physics schedule. Supports continuous collision detection
for fast projectiles, one-way platforms (pass through from below), and conveyor belt surface
velocity. Deterministic for server-authoritative 2D games.

- **Requirements:** R-10.5.10
- **Dependencies:** F-1.1.1 (ECS), F-4.1.1 (Integration)
- **Platform notes:** Mobile budgets fewer active rigid bodies (200 vs 500+ on desktop) to
  maintain 60fps on lower-core-count devices.

### F-10.5.11 2D Collision Shapes and Tilemap Colliders

Collider shapes for 2D: box, circle, capsule, convex polygon, edge chain, and composite
shapes. Tilemap collision layers auto-generate optimized edge chain colliders from tile
collision flags — merged edges reduce collider count from thousands of tiles to dozens of
edge segments. Per-tile collision properties (friction, bounciness, one-way) are supported.

- **Requirements:** R-10.5.11
- **Dependencies:** F-10.5.10, F-10.5.6
- **Platform notes:** None

### F-10.5.12 2D Joints and Constraints

2D joint types: revolute (hinge), prismatic (slider), distance, spring, rope, weld, wheel,
and mouse (drag). Joints connect `RigidBody2D` entities and support motors, limits, and break
force. Used for ragdolls, swinging platforms, grappling hooks, destructible chains, and
vehicle suspension in 2D games.

- **Requirements:** R-10.5.12
- **Dependencies:** F-10.5.10
- **Platform notes:** None

### F-10.5.13 2D Spatial Queries

Ray casts, shape casts, and overlap tests in 2D space. Queries operate on the shared spatial
index (F-1.9.1) filtered to 2D colliders. Results include entity, hit point, normal, and
distance. Batch queries for AI line-of-sight, area-of-effect, and projectile trajectory
checks. Essential for top-down shooters, platformer ground detection, and 2D RTS selection.

- **Requirements:** R-10.5.13
- **Dependencies:** F-10.5.10, F-1.9.4 (Unified Spatial Query)
- **Platform notes:** None

## 2D Lighting and Effects

### F-10.5.14 2D Dynamic Lighting

Dynamic 2D lighting with point lights, spotlights, and global ambient. Shadow casting from
2D occluders (sprite edges, tilemap walls) using shadow map or ray-marched approaches. Light
rendered into a compositable light map texture. Supports colored lights, falloff curves,
normal-mapped sprites for pseudo-3D lighting, and emissive sprites that glow independently.

- **Requirements:** R-10.5.14
- **Dependencies:** F-10.5.1
- **Platform notes:** Mobile caps dynamic 2D lights (8 vs 32 on desktop) and uses
  lower-resolution light maps (half res) to stay within fill rate budget.

### F-10.5.15 2D Particle Effects

2D particle emitters integrated with the sprite rendering pipeline. Particles render as
textured quads sorted into the 2D z-order. Supports all 3D particle modules (velocity,
gravity, color/size over life, collision) projected onto the 2D plane. Trail/ribbon particles
for sword slashes, magic effects, and projectile trails. Performance-optimized for mobile.

- **Requirements:** R-10.5.15
- **Dependencies:** F-10.5.1, F-11.1.1 (Particle System)
- **Platform notes:** Mobile caps active particles per emitter (256 vs 1024 on desktop) and
  total on-screen particles (2K vs 10K). Trail particles use fewer segments on mobile.

## Mobile and Touch

### F-10.5.16 On-Screen Virtual Controls

Configurable on-screen touch controls: virtual joystick (fixed or floating), virtual D-pad,
action buttons, and gesture zones. Controls are ECS entities with customizable sprites, dead
zones, and input mapping. Virtual joystick output feeds into the input action system (F-6.2)
identically to a physical gamepad. Layout adapts to screen size and orientation. Opacity and
position are user-configurable at runtime.

- **Requirements:** R-10.5.16
- **Dependencies:** F-6.1.4 (Touch Input), F-6.2.1 (Input Actions)
- **Platform notes:** iOS and Android touch input. Haptic feedback on button press where
  supported (F-6.4.1).

### F-10.5.17 Mobile Gesture Integration for 2D Games

Map touch gestures to 2D game actions: pinch-to-zoom controls the 2D camera, two-finger pan
scrolls the world, swipe flings objects or triggers abilities, long-press selects units, and
double-tap performs context actions. Gesture recognition (F-6.3) integrates with the input
action system so gesture-to-action mappings are rebindable. Supports simultaneous gestures
(pan + zoom) and gesture priority resolution.

- **Requirements:** R-10.5.17
- **Dependencies:** F-6.3.1 (Gestures), F-10.5.9 (2D Camera)
- **Platform notes:** None

## 2D Networking

### F-10.5.18 2D State Replication

Replicate 2D game state using the same networking stack as 3D (F-8.2) but optimized for 2D
data: `Transform2D` delta compression (position + rotation, no Z), sprite animation state
sync, tilemap chunk replication, and 2D physics state (velocity, angular velocity). Bandwidth-
efficient for 2D MMOs with hundreds of visible entities. Relevancy filtering uses 2D distance
(no vertical axis) for tighter area-of-interest culling.

- **Requirements:** R-10.5.18
- **Dependencies:** F-8.2.1 (State Replication), F-8.2.2 (Relevancy)
- **Platform notes:** None

### F-10.5.19 2D Client Prediction and Rollback

Client-side prediction for 2D movement and abilities with server reconciliation. Rollback
netcode (GGPO-style) for competitive 2D fighting and action games — save 2D physics state,
re-simulate from confirmed input on misprediction. Input delay and rollback frames are
configurable. Supports both lockstep (fighting games, RTS) and server-authoritative (MMO,
co-op) models.

- **Requirements:** R-10.5.19
- **Dependencies:** F-10.5.18, F-8.4.1 (Prediction), F-10.5.10 (2D Physics)
- **Platform notes:** None

## 2D Procedural Content

### F-10.5.20 Procedural 2D World Generation

Generate infinite or bounded 2D worlds using noise-based biome distribution, temperature/
moisture maps projected onto a 2D grid, and rule-based feature placement (villages, dungeons,
rivers, roads). The PCG graph system (F-3.6.1) operates in 2D mode with point generation on
a plane rather than a surface. Chunk-based streaming (F-3.6.31) loads/unloads 2D world regions
as the player explores. Used for open-world 2D RPGs, survival games, and 2D MMOs.

- **Requirements:** R-10.5.20
- **Dependencies:** F-3.6.1 (PCG Graph), F-3.6.5 (Seeding), F-10.5.6 (Tilemaps)
- **Platform notes:** None

### F-10.5.21 2D Room and Dungeon Generation

Specialized procedural generators for 2D room-based levels: BSP tree subdivision, random
walk carving, room-and-corridor placement with connectivity guarantees, and cellular automata
for organic cave shapes. Generators output tilemap data (F-10.5.6) with collision, decoration,
and spawn-point layers. Lock-and-key analysis ensures generated dungeons are traversable with
required items found before locked doors.

- **Requirements:** R-10.5.21
- **Dependencies:** F-10.5.8, F-3.6.30 (Constraint Solver)
- **Platform notes:** None

## 2.5D and Hybrid Rendering

### F-10.5.22 2.5D Layer Composition

Composite 3D-rendered elements into a 2D scene pipeline. A `RenderLayer3D` component renders
3D characters, props, or effects into an offscreen texture that is inserted into the 2D
z-order as a sprite-like layer. 3D objects respect 2D game constraints (movement on a 2D
plane, 2D collision, 2D physics) while displaying full 3D models with lighting and shadows.
Multiple 3D layers can be interleaved with 2D sprite and parallax layers for depth effects.
Used for side-scrollers and top-down games with 3D characters on 2D backgrounds.

- **Requirements:** R-10.5.22
- **Dependencies:** F-10.5.1, F-10.5.9 (2D Camera), F-2.10.1 (Render Proxy)
- **Platform notes:** Mobile limits simultaneous 3D layers (2 vs 4+ on desktop) and renders
  them at reduced resolution to control GPU fill rate.

### F-10.5.23 Perspective 3D with 2D Physics (Octopath-Style)

A perspective or orthographic 3D camera renders a full 3D scene while gameplay physics are
constrained to a 2D plane. 2D collision shapes (F-10.5.11) and 2D rigid bodies (F-10.5.10)
operate on the XY or XZ plane; the 3D renderer shows the scene from an angled viewpoint with
depth. 2D sprite assets (billboards, particle cards, painted backdrops) layer into the 3D
scene with depth-correct sorting. 3D parallax backgrounds scroll at varying rates based on
depth. This is the HD-2D rendering model (Octopath Traveler, Triangle Strategy).

- **Requirements:** R-10.5.23
- **Dependencies:** F-10.5.10 (2D Physics), F-10.5.14 (2D Lighting), F-2.10.4 (View Setup)
- **Platform notes:** None

### F-10.5.24 Dynamic 2D/3D Asset Layering

Arbitrarily layer 2D and 3D assets within a single scene. Each layer has a render mode (2D
sprite, 3D mesh, 2D tilemap, 3D particle, 2D vector) and a depth value. The compositor
resolves visibility between layers using depth testing or painter's algorithm as configured.
2D UI elements, 3D characters, 2D foreground foliage, and 3D environmental effects coexist
in a single frame with correct occlusion. Layer visibility and blend modes are configurable
per layer and per camera.

- **Requirements:** R-10.5.24
- **Dependencies:** F-10.5.1, F-10.5.4, F-2.10.1 (Render Proxy)
- **Platform notes:** None
