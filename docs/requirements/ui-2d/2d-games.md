# R-10.5 — 2D Game Support Requirements

## Sprite Rendering and Animation

| ID       | Derived From                                 |
|----------|----------------------------------------------|
| R-10.5.1 | [F-10.5.1](../../features/ui-2d/2d-games.md) |
| R-10.5.2 | [F-10.5.2](../../features/ui-2d/2d-games.md) |
| R-10.5.3 | [F-10.5.3](../../features/ui-2d/2d-games.md) |

1. **R-10.5.1** — The engine **SHALL** render 2D sprites as instanced textured quads with per-sprite
   position, rotation, scale, UV rect, atlas index, tint color, and z-order, batching sprites by
   atlas page and blend mode to minimize draw calls.
   - **Rationale:** Efficient sprite batching is essential for rendering thousands of sprites per
     frame without excessive draw call overhead.
   - **Verification:** Render 10,000 sprites across 4 atlas pages and verify batch count equals the
     number of unique atlas-page/blend-mode combinations, with correct z-order sorting within each
     batch.
2. **R-10.5.2** — The engine **SHALL** support frame-based sprite animation with configurable
   playback rate, looping modes (loop, ping-pong, one-shot, reverse), and animation events that
   trigger callbacks at specified frames.
   - **Rationale:** Frame-based animation is the standard technique for 2D games and requires
     precise timing control and event hooks for gameplay integration.
   - **Verification:** Play animations in each looping mode and verify correct frame sequencing,
     playback rate accuracy within 1 ms, and event callback invocation at the designated frames.
3. **R-10.5.3** — The engine **SHALL** support bone-based 2D animation with mesh deformation, blend
   weights, IK constraints, bone-driven sprite swapping, and runtime bone manipulation, with import
   support for Spine and DragonBones formats.
   - **Rationale:** Skeletal animation provides smoother character motion than frame-by-frame
     sprites and enables procedural effects like look-at and recoil.
   - **Verification:** Import a Spine skeleton, animate it with IK constraints and bone
     manipulation, and verify mesh deformation matches the reference output within 1 pixel tolerance
     at 1080p.

## Vector Graphics

| ID       | Derived From                                 |
|----------|----------------------------------------------|
| R-10.5.4 | [F-10.5.4](../../features/ui-2d/2d-games.md) |
| R-10.5.5 | [F-10.5.5](../../features/ui-2d/2d-games.md) |

1. **R-10.5.4** — The engine **SHALL** render 2D shapes from vector paths (Bezier curves, arcs,
   lines) with fill, stroke, and clipping mask support, producing resolution-independent output
   without pixelation at any scale. The vector rendering engine is shared with the UI vector
   graphics system (R-10.4.3).
   - **Rationale:** Vector rendering enables resolution-independent graphics that scale cleanly
     across mobile and desktop displays without requiring multiple raster asset resolutions.
   - **Verification:** Render vector shapes at 1x, 2x, and 4x zoom and verify edges remain sharp
     with no pixelation artifacts, and that fill/stroke/clip operations produce correct visual
     output.
2. **R-10.5.5** — The engine **SHALL** support skeletal animation of vector paths where bones deform
   vector control points, maintaining crisp edges at any zoom level while providing the same bone
   hierarchy, IK, and animation graph features as raster skeletal animation.
   - **Rationale:** Combining vector rendering with skeletal animation enables stylized 2D games
     with zoom mechanics to maintain visual quality at all scales.
   - **Verification:** Animate a vector character skeleton at 1x and 8x zoom, and verify edges
     remain alias-free and bone deformations match the raster skeletal animation feature set.

## Tilemaps

| ID       | Derived From                                 |
|----------|----------------------------------------------|
| R-10.5.6 | [F-10.5.6](../../features/ui-2d/2d-games.md) |
| R-10.5.7 | [F-10.5.7](../../features/ui-2d/2d-games.md) |
| R-10.5.8 | [F-10.5.8](../../features/ui-2d/2d-games.md) |

1. **R-10.5.6** — The engine **SHALL** render chunked tile grids mapped to atlas UVs via compute
   dispatch, supporting multiple tile layers, auto-tiling rules, animated tiles, per-tile
   flip/rotation flags, and viewport culling for worlds with millions of tiles.
   - **Rationale:** Tilemap rendering is fundamental to 2D world construction and must scale to
     large worlds through chunking and culling.
   - **Verification:** Load a tilemap with 1 million tiles across 4 layers, verify only
     viewport-visible chunks are dispatched, auto-tiling transitions render correctly, and animated
     tiles update each frame.
2. **R-10.5.7** — The engine **SHALL** support isometric (diamond and staggered) and hexagonal grid
   layouts with correct coordinate conversion, depth sorting by row and column, height-stacked
   tiles, and isometric wall occlusion.
   - **Rationale:** Isometric and hex grids are standard layouts for strategy and RPG games and
     require specialized coordinate math and depth sorting.
   - **Verification:** Render isometric diamond, isometric staggered, and hex grids, verify
     screen-to-tile coordinate conversion round-trips correctly, and validate depth sorting with
     overlapping stacked tiles.
3. **R-10.5.8** — The engine **SHALL** generate 2D tilemaps procedurally using WFC, noise-based
   terrain, cellular automata, BSP dungeon generation, and room-and-corridor algorithms, with
   deterministic seeding for reproducible output.
   - **Rationale:** Procedural generation enables replayable content for roguelikes, infinite
     scrollers, and procedural overworlds without manual level authoring.
   - **Verification:** Generate tilemaps with each algorithm using a fixed seed, verify identical
     output across runs, and validate adjacency constraints are satisfied in WFC output.

## Camera and Scrolling

| ID       | Derived From                                 |
|----------|----------------------------------------------|
| R-10.5.9 | [F-10.5.9](../../features/ui-2d/2d-games.md) |

1. **R-10.5.9** — The engine **SHALL** provide a 2D camera with position, zoom, rotation, viewport
   bounds, parallax scrolling across multiple layers, smoothing modes (lerp follow, look-ahead,
   snap-to-grid), camera shake, and split-screen support for local co-op.
   - **Rationale:** A dedicated 2D camera is required for consistent scrolling, framing, and
     viewport management across all 2D game types.
   - **Verification:** Configure a 2D camera with parallax layers and smoothing, verify layers
     scroll at specified rates, camera shake decays correctly, and split-screen renders to separate
     viewports.

## 2D Physics

| ID        | Derived From                                  |
|-----------|-----------------------------------------------|
| R-10.5.10 | [F-10.5.10](../../features/ui-2d/2d-games.md) |
| R-10.5.11 | [F-10.5.11](../../features/ui-2d/2d-games.md) |
| R-10.5.12 | [F-10.5.12](../../features/ui-2d/2d-games.md) |
| R-10.5.13 | [F-10.5.13](../../features/ui-2d/2d-games.md) |

1. **R-10.5.10** — The engine **SHALL** implement 2D rigid body simulation as ECS components and
   systems, supporting continuous collision detection, one-way platforms, conveyor belt surface
   velocity, and deterministic simulation for server-authoritative games.
   - **Rationale:** ECS-based 2D physics is required for platformers, top-down games, and any 2D
     game needing physical interaction, with determinism enabling networked play.
   - **Verification:** Simulate 100 rigid bodies for 1000 fixed timesteps with a fixed seed and
     verify identical final state across runs, CCD prevents tunneling at high velocities, and
     one-way platforms allow upward passage only.
2. **R-10.5.11** — The engine **SHALL** support 2D collider shapes (box, circle, capsule, convex
   polygon, edge chain, composite) and auto-generate optimized edge chain colliders from tilemap
   collision flags with per-tile collision properties.
   - **Rationale:** Optimized tilemap colliders reduce collider count from thousands of individual
     tiles to merged edge segments, improving broadphase performance.
   - **Verification:** Generate colliders from a 100x100 tilemap and verify edge merging reduces
     collider count by at least 90% compared to per-tile colliders, with correct collision response
     on all shape types.
3. **R-10.5.12** — The engine **SHALL** provide 2D joint types (revolute, prismatic, distance,
   spring, rope, weld, wheel, mouse) connecting rigid body entities, with support for motors,
   limits, and break force thresholds.
   - **Rationale:** Joints enable complex mechanical interactions such as ragdolls, swinging
     platforms, grappling hooks, and vehicle suspension in 2D games.
   - **Verification:** Create each joint type between two rigid bodies, verify motor-driven motion
     respects configured limits, and confirm joints break when applied force exceeds the break force
     threshold.
4. **R-10.5.13** — The engine **SHALL** support ray casts, shape casts, and overlap tests in 2D
   space using the shared spatial index, returning entity, hit point, normal, and distance, with
   batch query support.
   - **Rationale:** Spatial queries are essential for AI line-of-sight, area-of-effect, projectile
     trajectory checks, and selection mechanics in 2D games.
   - **Verification:** Perform ray cast, shape cast, and overlap queries against known 2D geometry
     and verify hit results match expected entity, point, normal, and distance within floating-point
     tolerance.

## 2D Lighting and Effects

| ID        | Derived From                                  |
|-----------|-----------------------------------------------|
| R-10.5.14 | [F-10.5.14](../../features/ui-2d/2d-games.md) |
| R-10.5.15 | [F-10.5.15](../../features/ui-2d/2d-games.md) |

1. **R-10.5.14** — The engine **SHALL** render dynamic 2D lighting with point lights, spotlights,
   ambient light, and shadow casting from 2D occluders into a compositable light map, supporting
   colored lights, falloff curves, normal-mapped sprites, and emissive sprites.
   - **Rationale:** Dynamic lighting adds depth and atmosphere to 2D scenes and is a visual baseline
     expectation for modern 2D games.
   - **Verification:** Place point and spot lights near occluding sprites, verify shadows are cast
     correctly, normal-mapped sprites respond to light direction, and emissive sprites remain
     visible without external light.
2. **R-10.5.15** — The engine **SHALL** integrate 2D particle emitters with the sprite rendering
   pipeline, rendering particles as textured quads sorted into the 2D z-order, supporting all 3D
   particle modules projected onto the 2D plane and trail/ribbon particles.
   - **Rationale:** Particle effects are critical for visual feedback (explosions, magic, trails)
     and must integrate with 2D z-ordering to render correctly among sprites.
   - **Verification:** Emit 2D particles with velocity, gravity, and color-over-life modules, verify
     correct z-order interleaving with sprites, and confirm trail particles render continuous
     ribbons.

## Mobile and Touch

| ID        | Derived From                                  |
|-----------|-----------------------------------------------|
| R-10.5.16 | [F-10.5.16](../../features/ui-2d/2d-games.md) |
| R-10.5.17 | [F-10.5.17](../../features/ui-2d/2d-games.md) |

1. **R-10.5.16** — The engine **SHALL** provide configurable on-screen touch controls (virtual
   joystick, D-pad, action buttons, gesture zones) as ECS entities whose output feeds into the input
   action system identically to physical gamepad input, with layout adaptation to screen size and
   orientation.
   - **Rationale:** On-screen controls are required for mobile platforms where physical gamepads are
     unavailable, and must integrate seamlessly with the input action system.
   - **Verification:** Render virtual joystick and buttons on mobile, verify touch input produces
     action events identical to gamepad input, and confirm layout adapts correctly to portrait and
     landscape orientations.
2. **R-10.5.17** — The engine **SHALL** map touch gestures (pinch-to-zoom, two-finger pan, swipe,
   long-press, double-tap) to 2D game actions through the input action system with rebindable
   mappings, supporting simultaneous gestures and priority resolution.
   - **Rationale:** Touch gesture integration enables intuitive mobile controls for 2D camera
     manipulation, object interaction, and unit selection.
   - **Verification:** Perform each gesture type and verify the mapped action fires, confirm
     simultaneous pinch+pan works without conflict, and validate that gesture-to-action bindings are
     rebindable at runtime.

## 2D Networking

| ID        | Derived From                                  |
|-----------|-----------------------------------------------|
| R-10.5.18 | [F-10.5.18](../../features/ui-2d/2d-games.md) |
| R-10.5.19 | [F-10.5.19](../../features/ui-2d/2d-games.md) |

1. **R-10.5.18** — The engine **SHALL** replicate 2D game state (Transform2D with delta compression,
   sprite animation state, tilemap chunks, 2D physics state) using the networking stack with
   2D-distance-based relevancy filtering for area-of-interest culling.
   - **Rationale:** 2D-optimized replication reduces bandwidth by omitting the Z axis and using
     tighter relevancy bounds, enabling 2D MMOs with hundreds of visible entities.
   - **Verification:** Replicate 200 entities between server and client, verify Transform2D delta
     compression reduces bandwidth versus uncompressed, and confirm entities outside 2D relevancy
     radius are culled.
2. **R-10.5.19** — The engine **SHALL** support client-side prediction with server reconciliation
   and rollback netcode (GGPO-style) for 2D games, with configurable input delay and rollback
   frames, supporting both lockstep and server-authoritative models.
   - **Rationale:** Rollback netcode is essential for competitive 2D fighting and action games to
     provide responsive input while maintaining consistency across clients.
   - **Verification:** Simulate a 2-player 2D session with artificial latency, inject a
     misprediction, verify rollback re-simulates from confirmed input and corrects client state
     within the configured rollback frame window.

## 2D Procedural Content

| ID        | Derived From                                  |
|-----------|-----------------------------------------------|
| R-10.5.20 | [F-10.5.20](../../features/ui-2d/2d-games.md) |
| R-10.5.21 | [F-10.5.21](../../features/ui-2d/2d-games.md) |

1. **R-10.5.20** — The engine **SHALL** generate infinite or bounded 2D worlds using noise-based
   biome distribution, temperature/moisture maps, and rule-based feature placement, with chunk-based
   streaming that loads and unloads 2D world regions as the player explores.
   - **Rationale:** Procedural world generation enables open-world 2D RPGs and survival games with
     vast explorable spaces that do not require manual authoring.
   - **Verification:** Generate a world with multiple biomes and features, verify chunk streaming
     loads only nearby regions, and confirm deterministic seeding produces identical worlds across
     runs.
2. **R-10.5.21** — The engine **SHALL** generate 2D room-based levels using BSP subdivision, random
   walk carving, room-and-corridor placement, and cellular automata, outputting tilemap data with
   collision, decoration, and spawn-point layers, with lock-and-key traversability guarantees.
   - **Rationale:** Dungeon generation with traversability guarantees ensures procedurally generated
     levels are always completable by the player.
   - **Verification:** Generate 100 dungeons with lock-and-key constraints and verify 100% are
     traversable from start to exit, with required keys reachable before their corresponding locked
     doors.

## 2.5D and Hybrid Rendering

| ID        | Derived From                                  |
|-----------|-----------------------------------------------|
| R-10.5.22 | [F-10.5.22](../../features/ui-2d/2d-games.md) |
| R-10.5.23 | [F-10.5.23](../../features/ui-2d/2d-games.md) |
| R-10.5.24 | [F-10.5.24](../../features/ui-2d/2d-games.md) |

1. **R-10.5.22** — The engine **SHALL** composite 3D-rendered elements into the 2D scene pipeline
   via a `RenderLayer3D` component that renders 3D content into an offscreen texture inserted into
   the 2D z-order, with 3D objects respecting 2D movement, collision, and physics constraints.
   - **Rationale:** 2.5D composition enables side-scrollers and top-down games to display 3D
     characters with lighting and shadows on 2D backgrounds.
   - **Verification:** Render a 3D character on a 2D sprite background, verify correct z-order
     interleaving with 2D layers, and confirm the 3D object collides using 2D physics shapes.
2. **R-10.5.23** — The engine **SHALL** support a rendering mode where a perspective or orthographic
   3D camera renders a full 3D scene while gameplay physics are constrained to a 2D plane, with 2D
   sprite assets layered into the 3D scene with depth-correct sorting and 3D parallax backgrounds.
   - **Rationale:** The HD-2D rendering model (Octopath Traveler style) combines 3D visual depth
     with 2D gameplay simplicity, requiring tight integration between 3D rendering and 2D physics.
   - **Verification:** Render a 3D scene with 2D physics on the XZ plane, verify 2D collisions
     operate correctly, 2D sprites sort by depth in the 3D view, and parallax backgrounds scroll at
     depth-based rates.
3. **R-10.5.24** — The engine **SHALL** support arbitrary layering of 2D and 3D assets within a
   single scene, where each layer has a render mode and depth value, with the compositor resolving
   visibility via depth testing or painter's algorithm and per-layer configurable blend modes.
   - **Rationale:** Flexible layer composition enables mixed-media scenes where UI, sprites, 3D
     meshes, particles, and vector graphics coexist with correct occlusion.
   - **Verification:** Compose a scene with 2D sprite, 3D mesh, tilemap, particle, and vector layers
     at interleaved depths, and verify correct occlusion and blend mode application across all layer
     pairs.
