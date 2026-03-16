# User Stories — 10.5 2D Game Support

## Sprite Rendering and Animation

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|---|---|---|---|---|---|
| US-10.5.1 | Developer (P-15) | I want sprites rendered as instanced textured quads batched by atlas page and blend mode with per-sprite transform, UV rect, tint, and z-order, so that scenes with thousands of sprites draw in a few batched calls. | Per-sprite position, rotation, scale, UV rect, atlas index, tint, z-order; batching by atlas page and blend mode; sorting by z-order within each batch | F-10.5.1 | R-10.5.1 |
| US-10.5.2 | Designer (P-5) | I want frame-based sprite animation with configurable playback rate, looping modes, and animation events for footstep sounds and hit detection, so that I can create expressive character animations with precise timing. | Loop, ping-pong, one-shot, reverse playback modes; animation events trigger at specific frames; animation blending and transition graphs | F-10.5.2 | R-10.5.2 |
| US-10.5.3 | Engine tester (P-27) | I want to verify that sprite animation events fire at exactly the correct frame in all playback modes, so that footstep sounds, hit detection, and other frame-synced events are never early or late. | Events fire at exact frame in loop mode; events fire correctly in ping-pong and reverse modes; transition blends do not skip or duplicate events | F-10.5.2 | R-10.5.2 |
| US-10.5.4 | Artist (P-8) | I want bone-based 2D skeletal animation with mesh deformation, IK constraints, bone-driven sprite swapping, and Spine/DragonBones import, so that I can create smooth character animations beyond what frame-by-frame provides. | Bone hierarchy with per-bone transform keyframes; mesh vertex skinning with blend weights; two-bone IK constraints; Spine and DragonBones format import | F-10.5.3 | R-10.5.3 |
| US-10.5.5 | Developer (P-15) | I want runtime bone manipulation for procedural effects like look-at targeting, weapon recoil, and procedural head turning, so that 2D characters respond dynamically to gameplay without pre-authored animation for every case. | Runtime bone manipulation (look-at, recoil); bone-driven sprite swapping for equipment changes; procedural effects blend with authored animations | F-10.5.3 | R-10.5.3 |

## Vector Graphics

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|---|---|---|---|---|---|
| US-10.5.6 | Developer (P-15) | I want 2D shapes rendered from vector paths with fill, stroke, clipping masks, and gradient support at any resolution without pixelation, so that art games, map overlays, and scalable elements work across mobile and desktop. | Fill (solid, gradient, pattern) and stroke (width, cap, join, dash); Bezier curves, arcs, and lines without pixelation; clipping masks for vector paths | F-10.5.4 | R-10.5.4 |
| US-10.5.7 | Artist (P-8) | I want vector skeletal animation where bones deform vector path control points instead of raster mesh vertices, so that characters maintain crisp edges at any zoom level while animating smoothly. | Bones deform vector path control points; characters crisp at any zoom level; same IK and animation graph features as raster skeletal | F-10.5.5 | R-10.5.5 |

## Tilemaps

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|---|---|---|---|---|---|
| US-10.5.8 | Designer (P-5) | I want chunked tilemap rendering with multiple layers, auto-tiling rules for terrain transitions, animated tiles, and per-tile flip/rotation, so that I can build large tile-based worlds with millions of tiles that stream and cull to the camera. | Chunked tile grids with compute dispatch rendering; multiple tile layers (ground, decoration, collision); auto-tiling rules for terrain transitions; animated tiles and per-tile flip/rotation | F-10.5.6 | R-10.5.6 |
| US-10.5.9 | Engine developer (P-26) | I want to implement tilemap chunk streaming and viewport culling, so that worlds with millions of tiles load and render only the visible portion efficiently. | Chunks streamed and culled to camera viewport; mobile limits visible tile layers (3 vs 8 desktop); smaller tile atlas pages on mobile | F-10.5.6 | R-10.5.6 |
| US-10.5.10 | Designer (P-5) | I want isometric (diamond and staggered) and hexagonal grid layouts with correct depth sorting, coordinate conversion, and height-stacked tiles, so that I can create isometric RPGs and hex-based strategy games. | Isometric diamond and staggered grid layouts; hexagonal grid layout; correct depth sorting by tile row and column; height-stacked tiles for multi-level maps | F-10.5.7 | R-10.5.7 |
| US-10.5.11 | Engine tester (P-27) | I want to verify that isometric and hex tilemap depth sorting and screen-to-tile coordinate conversion are correct for all grid types, so that tiles never z-fight and mouse clicks map to the correct tile. | No z-fighting between overlapping isometric tiles; screen-to-tile conversion accurate for all grid types; wall occlusion renders correctly in isometric view | F-10.5.7 | R-10.5.7 |
| US-10.5.12 | Developer (P-15) | I want procedural 2D tilemap generation using WFC, cellular automata, BSP dungeons, and room-and-corridor algorithms with adjacency constraints, so that I can create explorable roguelike dungeons and infinite side-scrollers. | WFC generation from tileset adjacency constraints; cellular automata for cave systems; BSP and room-and-corridor dungeon generation; deterministic seeding for multiplayer reproducibility | F-10.5.8 | R-10.5.8 |

## Camera and Scrolling

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|---|---|---|---|---|---|
| US-10.5.13 | Developer (P-15) | I want a dedicated 2D camera with position, zoom, rotation, parallax scrolling, camera smoothing, shake, and split-screen support, so that I can implement smooth camera behavior for any 2D game genre. | Position, zoom, rotation, and viewport bounds; parallax scrolling with multiple background layers; camera smoothing (lerp follow, look-ahead, snap-to-grid); split-screen for local co-op | F-10.5.9 | R-10.5.9 |
| US-10.5.14 | Artist (P-8) | I want to create multiple parallax background layers that scroll at different rates for a depth illusion, so that side-scrolling and top-down games feel more immersive with layered backgrounds. | Multiple background layers at configurable scroll rates; layers composite correctly with sprite z-order; camera shake affects all layers proportionally | F-10.5.9 | R-10.5.9 |

## 2D Physics

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|---|---|---|---|---|---|
| US-10.5.15 | Developer (P-15) | I want 2D rigid body physics simulated as ECS components (RigidBody2D, Velocity2D, Mass2D, Collider2D) with CCD, one-way platforms, and deterministic simulation, so that I can build server-authoritative 2D games with reliable physics. | ECS components: RigidBody2D, Velocity2D, Mass2D, Collider2D; continuous collision detection for fast projectiles; one-way platforms and conveyor belt surfaces; deterministic for server-authoritative games | F-10.5.10 | R-10.5.10 |
| US-10.5.16 | Engine tester (P-27) | I want to verify bit-identical 2D physics simulation across all supported platforms, so that server-authoritative multiplayer games produce identical results on client and server regardless of OS. | Bit-identical simulation across Windows, macOS, Linux; same results at different frame rates via fixed timestep; CCD produces identical results across platforms | F-10.5.10 | R-X.12.2, R-10.5.10 |
| US-10.5.17 | Developer (P-15) | I want 2D collider shapes (box, circle, capsule, convex polygon, edge chain, composite) with tilemap collision layers that auto-generate optimized edge chains, so that 2D worlds have efficient collision without manual collider setup. | Box, circle, capsule, convex polygon, edge chain shapes; tilemap collision auto-generates merged edge chains; per-tile collision properties (friction, bounciness, one-way) | F-10.5.11 | R-10.5.11 |
| US-10.5.18 | Developer (P-15) | I want 2D joint types (revolute, prismatic, distance, spring, rope, weld, wheel, mouse) with motors, limits, and break force, so that I can create ragdolls, swinging platforms, grappling hooks, and destructible chains. | All joint types connecting RigidBody2D entities; motors, limits, and break force per joint; joints support ragdolls and swinging mechanics | F-10.5.12 | R-10.5.12 |
| US-10.5.19 | Developer (P-15) | I want 2D ray casts, shape casts, and overlap tests against the shared spatial index, so that AI line-of-sight, area-of-effect abilities, platformer ground detection, and RTS unit selection work efficiently. | Ray casts, shape casts, overlap tests in 2D; results include entity, hit point, normal, distance; batch queries for AI and area-of-effect checks | F-10.5.13 | R-10.5.13 |

## 2D Lighting and Effects

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|---|---|---|---|---|---|
| US-10.5.20 | Artist (P-8) | I want dynamic 2D point lights, spotlights, and ambient lighting with shadow casting from sprite edges and tilemap walls, so that I can create atmospheric 2D scenes with colored lighting and normal-mapped sprites. | Point lights, spotlights, and global ambient; shadow casting from 2D occluders; normal-mapped sprites for pseudo-3D lighting; emissive sprites that glow independently | F-10.5.14 | R-10.5.14 |
| US-10.5.21 | Engine tester (P-27) | I want to benchmark 2D dynamic lighting to verify mobile stays within fill rate budget with 8 lights and half-res light maps, so that 2D lighting does not cause frame drops on mobile devices. | Mobile: max 8 lights, half-res light map at 60fps; desktop: 32 lights at full resolution; light map compositing does not exceed fill rate budget | F-10.5.14 | R-10.5.14 |
| US-10.5.22 | Designer (P-5) | I want 2D particle emitters integrated with the sprite pipeline that render particles as textured quads sorted into the 2D z-order, so that I can add sword slashes, magic effects, and projectile trails to 2D games. | Particles render as textured quads in 2D z-order; all 3D particle modules projected onto 2D plane; trail/ribbon particles for slash and projectile effects; mobile caps: 256 per emitter, 2K total on screen | F-10.5.15 | R-10.5.15 |

## Mobile and Touch

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|---|---|---|---|---|---|
| US-10.5.23 | Designer (P-5) | I want configurable on-screen virtual joystick, D-pad, action buttons, and gesture zones that feed into the input action system, so that mobile players have responsive touch controls that adapt to screen size and orientation. | Virtual joystick (fixed or floating); virtual D-pad and action buttons; layout adapts to screen size and orientation; opacity and position user-configurable at runtime | F-10.5.16 | R-10.5.16 |
| US-10.5.24 | Player (P-23) | I want pinch-to-zoom, two-finger pan, swipe, long-press, and double-tap gestures mapped to 2D game actions, so that I can control the camera, select units, and trigger abilities using natural touch interactions. | Pinch-to-zoom controls 2D camera; two-finger pan scrolls the world; simultaneous gestures (pan + zoom) supported; gesture-to-action mappings rebindable | F-10.5.17 | R-10.5.17 |

## 2D Networking

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|---|---|---|---|---|---|
| US-10.5.25 | Developer (P-15) | I want 2D state replication optimized for Transform2D delta compression, sprite animation sync, tilemap chunk replication, and 2D physics state, so that 2D multiplayer games are bandwidth-efficient with hundreds of visible entities. | Transform2D delta compression (position + rotation, no Z); sprite animation state sync; 2D distance-based relevancy filtering; tilemap chunk replication | F-10.5.18 | R-10.5.18 |
| US-10.5.26 | Player (P-23) | I want client-side prediction with rollback netcode for 2D fighting and action games, so that competitive multiplayer feels responsive even with moderate latency. | Client-side prediction for 2D movement and abilities; GGPO-style rollback with state save and re-simulate; configurable input delay and rollback frames; both lockstep and server-authoritative models | F-10.5.19 | R-10.5.19 |
| US-10.5.27 | Engine developer (P-26) | I want to implement rollback netcode that saves and re-simulates 2D physics state on misprediction, so that competitive 2D games maintain frame-accurate synchronization with minimal perceived latency. | 2D physics state save and restore for rollback; re-simulation from confirmed input on misprediction; both lockstep and server-authoritative paths | F-10.5.19 | R-10.5.19 |

## 2D Procedural Content

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|---|---|---|---|---|---|
| US-10.5.28 | Developer (P-15) | I want procedural 2D world generation using noise-based biome distribution, temperature/moisture maps, and rule-based feature placement with chunk-based streaming, so that open-world 2D RPGs and survival games have explorable infinite worlds. | Noise-based biome distribution on 2D grid; rule-based placement of villages, dungeons, rivers, roads; chunk-based streaming loads/unloads as player explores; PCG graph operates in 2D mode | F-10.5.20 | R-10.5.20 |
| US-10.5.29 | Developer (P-15) | I want specialized 2D dungeon generators using BSP subdivision, random walk, and room-and-corridor placement with lock-and-key analysis, so that generated dungeons are always traversable with required items found before locked doors. | BSP, random walk, room-and-corridor generators; lock-and-key analysis ensures traversability; output includes collision, decoration, and spawn layers; connectivity guarantees prevent unreachable areas | F-10.5.21 | R-10.5.21 |

## 2.5D and Hybrid Rendering

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|---|---|---|---|---|---|
| US-10.5.30 | Developer (P-15) | I want 3D-rendered elements composited into the 2D scene pipeline via RenderLayer3D components inserted into the 2D z-order, so that 2.5D games show 3D characters on 2D backgrounds with correct depth ordering. | 3D elements rendered to offscreen texture and composited; 3D layers interleaved with 2D sprite and parallax layers; mobile limits 2 simultaneous 3D layers at reduced resolution | F-10.5.22 | R-10.5.22 |
| US-10.5.31 | Developer (P-15) | I want a perspective 3D camera viewing a scene where physics are constrained to a 2D plane (Octopath-style), so that I can create HD-2D games with depth and atmosphere while maintaining simple 2D gameplay logic. | 3D camera renders scene with depth from angled viewpoint; 2D collision and rigid bodies operate on XY or XZ plane; 2D sprites layer into 3D scene with depth-correct sorting; 3D parallax backgrounds at varying depth rates | F-10.5.23 | R-10.5.23 |
| US-10.5.32 | Designer (P-5) | I want to arbitrarily layer 2D sprites, 3D meshes, 2D tilemaps, 3D particles, and 2D vector elements with configurable depth and blend modes, so that hybrid scenes have correct occlusion between all render modes. | Each layer has configurable render mode and depth value; depth testing or painter's algorithm per layer; 2D and 3D assets coexist with correct occlusion; layer visibility and blend modes configurable per camera | F-10.5.24 | R-10.5.24 |
| US-10.5.33 | Engine tester (P-27) | I want to verify that mixed 2D/3D scene layers sort and occlude correctly across all supported render mode combinations, so that no visual artifacts occur from incorrect depth ordering between layer types. | 2D sprites occlude behind 3D meshes correctly; 3D particles render at correct depth relative to 2D layers; layer blend modes produce correct visual results | F-10.5.24 | R-10.5.24 |
