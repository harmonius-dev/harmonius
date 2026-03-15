# User Stories — 10.5 2D Game Support

## US-10.5.1 Render Thousands of Sprites With Minimal Draw Calls

**As a** developer (P-15), **I want** sprites rendered as instanced textured quads batched by
atlas page and blend mode with per-sprite transform, UV rect, tint, and z-order, **so that**
scenes with thousands of sprites draw in a few batched calls.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Per-sprite position, rotation, scale, UV rect, atlas index, tint, z-order | F-10.5.1 | R-10.5.1 |
| Batching by atlas page and blend mode | F-10.5.1 | R-10.5.1 |
| Sorting by z-order within each batch | F-10.5.1 | R-10.5.1 |

## US-10.5.2 Animate Sprites With Configurable Playback and Events

**As a** designer (P-5), **I want** frame-based sprite animation with configurable playback
rate, looping modes, and animation events for footstep sounds and hit detection, **so that**
I can create expressive character animations with precise timing.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Loop, ping-pong, one-shot, reverse playback modes | F-10.5.2 | R-10.5.2 |
| Animation events trigger at specific frames | F-10.5.2 | R-10.5.2 |
| Animation blending and transition graphs | F-10.5.2 | R-10.5.2 |

## US-10.5.3 Verify Sprite Animation Event Timing Accuracy

**As an** engine tester (P-27), **I want** to verify that sprite animation events fire at
exactly the correct frame in all playback modes, **so that** footstep sounds, hit detection,
and other frame-synced events are never early or late.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Events fire at exact frame in loop mode | F-10.5.2 | R-10.5.2 |
| Events fire correctly in ping-pong and reverse modes | F-10.5.2 | R-10.5.2 |
| Transition blends do not skip or duplicate events | F-10.5.2 | R-10.5.2 |

## US-10.5.4 Animate 2D Characters With Skeletal Bone Deformation

**As an** artist (P-8), **I want** bone-based 2D skeletal animation with mesh deformation,
IK constraints, bone-driven sprite swapping, and Spine/DragonBones import, **so that** I can
create smooth character animations beyond what frame-by-frame provides.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Bone hierarchy with per-bone transform keyframes | F-10.5.3 | R-10.5.3 |
| Mesh vertex skinning with blend weights | F-10.5.3 | R-10.5.3 |
| Two-bone IK constraints | F-10.5.3 | R-10.5.3 |
| Spine and DragonBones format import | F-10.5.3 | R-10.5.3 |

## US-10.5.5 Use Runtime Bone Manipulation for Procedural Effects

**As a** developer (P-15), **I want** runtime bone manipulation for procedural effects like
look-at targeting, weapon recoil, and procedural head turning, **so that** 2D characters
respond dynamically to gameplay without pre-authored animation for every case.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Runtime bone manipulation (look-at, recoil) | F-10.5.3 | R-10.5.3 |
| Bone-driven sprite swapping for equipment changes | F-10.5.3 | R-10.5.3 |
| Procedural effects blend with authored animations | F-10.5.3 | R-10.5.3 |

## US-10.5.6 Render Resolution-Independent 2D Shapes From Vector Paths

**As a** developer (P-15), **I want** 2D shapes rendered from vector paths with fill,
stroke, clipping masks, and gradient support at any resolution without pixelation, **so that**
art games, map overlays, and scalable elements work across mobile and desktop.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Fill (solid, gradient, pattern) and stroke (width, cap, join, dash) | F-10.5.4 | R-10.5.4 |
| Bezier curves, arcs, and lines without pixelation | F-10.5.4 | R-10.5.4 |
| Clipping masks for vector paths | F-10.5.4 | R-10.5.4 |

## US-10.5.7 Animate Vector Characters That Stay Crisp at Any Zoom

**As an** artist (P-8), **I want** vector skeletal animation where bones deform vector path
control points instead of raster mesh vertices, **so that** characters maintain crisp edges
at any zoom level while animating smoothly.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Bones deform vector path control points | F-10.5.5 | R-10.5.5 |
| Characters crisp at any zoom level | F-10.5.5 | R-10.5.5 |
| Same IK and animation graph features as raster skeletal | F-10.5.5 | R-10.5.5 |

## US-10.5.8 Build Large Tile-Based Worlds With Auto-Tiling

**As a** designer (P-5), **I want** chunked tilemap rendering with multiple layers, auto-tiling
rules for terrain transitions, animated tiles, and per-tile flip/rotation, **so that** I can
build large tile-based worlds with millions of tiles that stream and cull to the camera.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Chunked tile grids with compute dispatch rendering | F-10.5.6 | R-10.5.6 |
| Multiple tile layers (ground, decoration, collision) | F-10.5.6 | R-10.5.6 |
| Auto-tiling rules for terrain transitions | F-10.5.6 | R-10.5.6 |
| Animated tiles and per-tile flip/rotation | F-10.5.6 | R-10.5.6 |

## US-10.5.9 Implement Tilemap Streaming and Viewport Culling

**As an** engine developer (P-26), **I want** to implement tilemap chunk streaming and
viewport culling, **so that** worlds with millions of tiles load and render only the visible
portion efficiently.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Chunks streamed and culled to camera viewport | F-10.5.6 | R-10.5.6 |
| Mobile limits visible tile layers (3 vs 8 desktop) | F-10.5.6 | R-10.5.6 |
| Smaller tile atlas pages on mobile | F-10.5.6 | R-10.5.6 |

## US-10.5.10 Build Isometric and Hex Grid Worlds

**As a** designer (P-5), **I want** isometric (diamond and staggered) and hexagonal grid
layouts with correct depth sorting, coordinate conversion, and height-stacked tiles, **so
that** I can create isometric RPGs and hex-based strategy games.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Isometric diamond and staggered grid layouts | F-10.5.7 | R-10.5.7 |
| Hexagonal grid layout | F-10.5.7 | R-10.5.7 |
| Correct depth sorting by tile row and column | F-10.5.7 | R-10.5.7 |
| Height-stacked tiles for multi-level maps | F-10.5.7 | R-10.5.7 |

## US-10.5.11 Verify Isometric Depth Sorting and Coordinate Conversion

**As an** engine tester (P-27), **I want** to verify that isometric and hex tilemap depth
sorting and screen-to-tile coordinate conversion are correct for all grid types, **so that**
tiles never z-fight and mouse clicks map to the correct tile.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| No z-fighting between overlapping isometric tiles | F-10.5.7 | R-10.5.7 |
| Screen-to-tile conversion accurate for all grid types | F-10.5.7 | R-10.5.7 |
| Wall occlusion renders correctly in isometric view | F-10.5.7 | R-10.5.7 |

## US-10.5.12 Generate Roguelike Dungeons From Tileset Rules

**As a** developer (P-15), **I want** procedural 2D tilemap generation using WFC, cellular
automata, BSP dungeons, and room-and-corridor algorithms with adjacency constraints,
**so that** I can create explorable roguelike dungeons and infinite side-scrollers.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| WFC generation from tileset adjacency constraints | F-10.5.8 | R-10.5.8 |
| Cellular automata for cave systems | F-10.5.8 | R-10.5.8 |
| BSP and room-and-corridor dungeon generation | F-10.5.8 | R-10.5.8 |
| Deterministic seeding for multiplayer reproducibility | F-10.5.8 | R-10.5.8 |

## US-10.5.13 Configure a 2D Camera With Parallax and Split-Screen

**As a** developer (P-15), **I want** a dedicated 2D camera with position, zoom, rotation,
parallax scrolling, camera smoothing, shake, and split-screen support, **so that** I can
implement smooth camera behavior for any 2D game genre.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Position, zoom, rotation, and viewport bounds | F-10.5.9 | R-10.5.9 |
| Parallax scrolling with multiple background layers | F-10.5.9 | R-10.5.9 |
| Camera smoothing (lerp follow, look-ahead, snap-to-grid) | F-10.5.9 | R-10.5.9 |
| Split-screen for local co-op | F-10.5.9 | R-10.5.9 |

## US-10.5.14 Design Parallax Backgrounds for Depth Illusion

**As an** artist (P-8), **I want** to create multiple parallax background layers that scroll
at different rates for a depth illusion, **so that** side-scrolling and top-down games feel
more immersive with layered backgrounds.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Multiple background layers at configurable scroll rates | F-10.5.9 | R-10.5.9 |
| Layers composite correctly with sprite z-order | F-10.5.9 | R-10.5.9 |
| Camera shake affects all layers proportionally | F-10.5.9 | R-10.5.9 |

## US-10.5.15 Simulate 2D Rigid Body Physics as ECS Components

**As a** developer (P-15), **I want** 2D rigid body physics simulated as ECS components
(RigidBody2D, Velocity2D, Mass2D, Collider2D) with CCD, one-way platforms, and
deterministic simulation, **so that** I can build server-authoritative 2D games with
reliable physics.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| ECS components: RigidBody2D, Velocity2D, Mass2D, Collider2D | F-10.5.10 | R-10.5.10 |
| Continuous collision detection for fast projectiles | F-10.5.10 | R-10.5.10 |
| One-way platforms and conveyor belt surfaces | F-10.5.10 | R-10.5.10 |
| Deterministic for server-authoritative games | F-10.5.10 | R-10.5.10 |

## US-10.5.16 Verify Deterministic 2D Physics Across Platforms

**As an** engine tester (P-27), **I want** to verify bit-identical 2D physics simulation
across all supported platforms, **so that** server-authoritative multiplayer games produce
identical results on client and server regardless of OS.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Bit-identical simulation across Windows, macOS, Linux | F-10.5.10 | R-X.12.2 |
| Same results at different frame rates via fixed timestep | F-10.5.10 | R-10.5.10 |
| CCD produces identical results across platforms | F-10.5.10 | R-10.5.10 |

## US-10.5.17 Use Box, Circle, and Polygon Colliders for 2D Objects

**As a** developer (P-15), **I want** 2D collider shapes (box, circle, capsule, convex
polygon, edge chain, composite) with tilemap collision layers that auto-generate optimized
edge chains, **so that** 2D worlds have efficient collision without manual collider setup.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Box, circle, capsule, convex polygon, edge chain shapes | F-10.5.11 | R-10.5.11 |
| Tilemap collision auto-generates merged edge chains | F-10.5.11 | R-10.5.11 |
| Per-tile collision properties (friction, bounciness, one-way) | F-10.5.11 | R-10.5.11 |

## US-10.5.18 Connect 2D Bodies With Joints for Ropes and Ragdolls

**As a** developer (P-15), **I want** 2D joint types (revolute, prismatic, distance, spring,
rope, weld, wheel, mouse) with motors, limits, and break force, **so that** I can create
ragdolls, swinging platforms, grappling hooks, and destructible chains.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| All joint types connecting RigidBody2D entities | F-10.5.12 | R-10.5.12 |
| Motors, limits, and break force per joint | F-10.5.12 | R-10.5.12 |
| Joints support ragdolls and swinging mechanics | F-10.5.12 | R-10.5.12 |

## US-10.5.19 Cast Rays and Test Overlaps in 2D Space

**As a** developer (P-15), **I want** 2D ray casts, shape casts, and overlap tests against
the shared spatial index, **so that** AI line-of-sight, area-of-effect abilities, platformer
ground detection, and RTS unit selection work efficiently.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Ray casts, shape casts, overlap tests in 2D | F-10.5.13 | R-10.5.13 |
| Results include entity, hit point, normal, distance | F-10.5.13 | R-10.5.13 |
| Batch queries for AI and area-of-effect checks | F-10.5.13 | R-10.5.13 |

## US-10.5.20 Light 2D Scenes With Dynamic Shadows

**As an** artist (P-8), **I want** dynamic 2D point lights, spotlights, and ambient lighting
with shadow casting from sprite edges and tilemap walls, **so that** I can create atmospheric
2D scenes with colored lighting and normal-mapped sprites.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Point lights, spotlights, and global ambient | F-10.5.14 | R-10.5.14 |
| Shadow casting from 2D occluders | F-10.5.14 | R-10.5.14 |
| Normal-mapped sprites for pseudo-3D lighting | F-10.5.14 | R-10.5.14 |
| Emissive sprites that glow independently | F-10.5.14 | R-10.5.14 |

## US-10.5.21 Benchmark 2D Lighting Within Mobile Fill Rate Budget

**As an** engine tester (P-27), **I want** to benchmark 2D dynamic lighting to verify mobile
stays within fill rate budget with 8 lights and half-res light maps, **so that** 2D lighting
does not cause frame drops on mobile devices.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Mobile: max 8 lights, half-res light map at 60fps | F-10.5.14 | R-10.5.14 |
| Desktop: 32 lights at full resolution | F-10.5.14 | R-10.5.14 |
| Light map compositing does not exceed fill rate budget | F-10.5.14 | R-10.5.14 |

## US-10.5.22 Add Particle Effects to 2D Scenes

**As a** designer (P-5), **I want** 2D particle emitters integrated with the sprite pipeline
that render particles as textured quads sorted into the 2D z-order, **so that** I can add
sword slashes, magic effects, and projectile trails to 2D games.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Particles render as textured quads in 2D z-order | F-10.5.15 | R-10.5.15 |
| All 3D particle modules projected onto 2D plane | F-10.5.15 | R-10.5.15 |
| Trail/ribbon particles for slash and projectile effects | F-10.5.15 | R-10.5.15 |
| Mobile caps: 256 per emitter, 2K total on screen | F-10.5.15 | R-10.5.15 |

## US-10.5.23 Configure Virtual Touch Controls for Mobile 2D Games

**As a** designer (P-5), **I want** configurable on-screen virtual joystick, D-pad, action
buttons, and gesture zones that feed into the input action system, **so that** mobile players
have responsive touch controls that adapt to screen size and orientation.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Virtual joystick (fixed or floating) | F-10.5.16 | R-10.5.16 |
| Virtual D-pad and action buttons | F-10.5.16 | R-10.5.16 |
| Layout adapts to screen size and orientation | F-10.5.16 | R-10.5.16 |
| Opacity and position user-configurable at runtime | F-10.5.16 | R-10.5.16 |

## US-10.5.24 Play 2D Games With Touch Gestures

**As a** player (P-23) on a mobile device, **I want** pinch-to-zoom, two-finger pan, swipe,
long-press, and double-tap gestures mapped to 2D game actions, **so that** I can control the
camera, select units, and trigger abilities using natural touch interactions.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Pinch-to-zoom controls 2D camera | F-10.5.17 | R-10.5.17 |
| Two-finger pan scrolls the world | F-10.5.17 | R-10.5.17 |
| Simultaneous gestures (pan + zoom) supported | F-10.5.17 | R-10.5.17 |
| Gesture-to-action mappings rebindable | F-10.5.17 | R-10.5.17 |

## US-10.5.25 Replicate 2D Game State Across Network

**As a** developer (P-15), **I want** 2D state replication optimized for Transform2D delta
compression, sprite animation sync, tilemap chunk replication, and 2D physics state,
**so that** 2D multiplayer games are bandwidth-efficient with hundreds of visible entities.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Transform2D delta compression (position + rotation, no Z) | F-10.5.18 | R-10.5.18 |
| Sprite animation state sync | F-10.5.18 | R-10.5.18 |
| 2D distance-based relevancy filtering | F-10.5.18 | R-10.5.18 |
| Tilemap chunk replication | F-10.5.18 | R-10.5.18 |

## US-10.5.26 Play Competitive 2D Games With Rollback Netcode

**As a** player (P-23), **I want** client-side prediction with rollback netcode for 2D
fighting and action games, **so that** competitive multiplayer feels responsive even with
moderate latency.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Client-side prediction for 2D movement and abilities | F-10.5.19 | R-10.5.19 |
| GGPO-style rollback with state save and re-simulate | F-10.5.19 | R-10.5.19 |
| Configurable input delay and rollback frames | F-10.5.19 | R-10.5.19 |
| Both lockstep and server-authoritative models | F-10.5.19 | R-10.5.19 |

## US-10.5.27 Implement Rollback Netcode for 2D Physics

**As an** engine developer (P-26), **I want** to implement rollback netcode that saves and
re-simulates 2D physics state on misprediction, **so that** competitive 2D games maintain
frame-accurate synchronization with minimal perceived latency.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| 2D physics state save and restore for rollback | F-10.5.19 | R-10.5.19 |
| Re-simulation from confirmed input on misprediction | F-10.5.19 | R-10.5.19 |
| Both lockstep and server-authoritative paths | F-10.5.19 | R-10.5.19 |

## US-10.5.28 Generate Infinite 2D Worlds With Biome Distribution

**As a** developer (P-15), **I want** procedural 2D world generation using noise-based biome
distribution, temperature/moisture maps, and rule-based feature placement with chunk-based
streaming, **so that** open-world 2D RPGs and survival games have explorable infinite worlds.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Noise-based biome distribution on 2D grid | F-10.5.20 | R-10.5.20 |
| Rule-based placement of villages, dungeons, rivers, roads | F-10.5.20 | R-10.5.20 |
| Chunk-based streaming loads/unloads as player explores | F-10.5.20 | R-10.5.20 |
| PCG graph operates in 2D mode | F-10.5.20 | R-10.5.20 |

## US-10.5.29 Generate Traversable 2D Dungeons With Lock-and-Key Logic

**As a** developer (P-15), **I want** specialized 2D dungeon generators using BSP
subdivision, random walk, and room-and-corridor placement with lock-and-key analysis,
**so that** generated dungeons are always traversable with required items found before
locked doors.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| BSP, random walk, room-and-corridor generators | F-10.5.21 | R-10.5.21 |
| Lock-and-key analysis ensures traversability | F-10.5.21 | R-10.5.21 |
| Output includes collision, decoration, and spawn layers | F-10.5.21 | R-10.5.21 |
| Connectivity guarantees prevent unreachable areas | F-10.5.21 | R-10.5.21 |

## US-10.5.30 Composite 3D Characters Into 2D Scenes

**As a** developer (P-15), **I want** 3D-rendered elements composited into the 2D scene
pipeline via RenderLayer3D components inserted into the 2D z-order, **so that** 2.5D games
show 3D characters on 2D backgrounds with correct depth ordering.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| 3D elements rendered to offscreen texture and composited | F-10.5.22 | R-10.5.22 |
| 3D layers interleaved with 2D sprite and parallax layers | F-10.5.22 | R-10.5.22 |
| Mobile limits 2 simultaneous 3D layers at reduced resolution | F-10.5.22 | R-10.5.22 |

## US-10.5.31 Build HD-2D Games With 3D Camera and 2D Physics

**As a** developer (P-15), **I want** a perspective 3D camera viewing a scene where physics
are constrained to a 2D plane (Octopath-style), **so that** I can create HD-2D games with
depth and atmosphere while maintaining simple 2D gameplay logic.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| 3D camera renders scene with depth from angled viewpoint | F-10.5.23 | R-10.5.23 |
| 2D collision and rigid bodies operate on XY or XZ plane | F-10.5.23 | R-10.5.23 |
| 2D sprites layer into 3D scene with depth-correct sorting | F-10.5.23 | R-10.5.23 |
| 3D parallax backgrounds at varying depth rates | F-10.5.23 | R-10.5.23 |

## US-10.5.32 Design Scenes With Mixed 2D and 3D Asset Layers

**As a** designer (P-5), **I want** to arbitrarily layer 2D sprites, 3D meshes, 2D tilemaps,
3D particles, and 2D vector elements with configurable depth and blend modes, **so that**
hybrid scenes have correct occlusion between all render modes.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Each layer has configurable render mode and depth value | F-10.5.24 | R-10.5.24 |
| Depth testing or painter's algorithm per layer | F-10.5.24 | R-10.5.24 |
| 2D and 3D assets coexist with correct occlusion | F-10.5.24 | R-10.5.24 |
| Layer visibility and blend modes configurable per camera | F-10.5.24 | R-10.5.24 |

## US-10.5.33 Verify 2D/3D Layer Occlusion and Depth Sorting

**As an** engine tester (P-27), **I want** to verify that mixed 2D/3D scene layers sort and
occlude correctly across all supported render mode combinations, **so that** no visual
artifacts occur from incorrect depth ordering between layer types.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| 2D sprites occlude behind 3D meshes correctly | F-10.5.24 | R-10.5.24 |
| 3D particles render at correct depth relative to 2D layers | F-10.5.24 | R-10.5.24 |
| Layer blend modes produce correct visual results | F-10.5.24 | R-10.5.24 |
