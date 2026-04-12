# R-10.5 — 2D Game Support Requirements

## Sprite Rendering

1. **R-10.5.1** — The engine **SHALL** render 2D sprites as instanced textured quads batched by
   atlas page and blend mode with per-sprite transform, UV rect, tint, and z-order sorting.
   - **Rationale:** Efficient batching is essential for thousands of sprites per frame.
   - **Verification:** Render 10000 sprites across 4 atlas pages. Assert batch count equals unique
     page/blend combinations with correct z-order.

2. **R-10.5.2** — The engine **SHALL** support frame-based sprite animation with configurable
   playback rate, looping modes (loop, ping-pong, one-shot, reverse), and animation events at
   specified frames.
   - **Rationale:** Frame animation is standard for 2D games and requires precise timing and event
     hooks.
   - **Verification:** Play animations in each mode. Verify frame sequencing and event invocation at
     designated frames within 1 ms accuracy.

3. **R-10.5.3** — The engine **SHALL** support bone-based 2D animation with mesh deformation, blend
   weights, IK, bone-driven sprite swap, runtime bone manipulation, and Spine/DragonBones import.
   - **Rationale:** Skeletal animation provides smoother motion and enables procedural effects.
   - **Verification:** Import Spine skeleton, animate with IK and bone manipulation. Assert
     deformation matches reference within 1 pixel at 1080p.

## Vector Graphics

4. **R-10.5.4** — The engine **SHALL** render 2D shapes from vector paths with fill, stroke, and
   clipping, producing resolution-independent output at any scale. The renderer is shared with the
   UI vector system (R-10.4.3).
   - **Rationale:** Vector rendering scales cleanly across device resolutions without multiple
     raster assets.
   - **Verification:** Render at 1x, 2x, 4x zoom. Assert sharp edges with no pixelation.

5. **R-10.5.5** — The engine **SHALL** support skeletal animation of vector paths where bones deform
   control points, maintaining crisp edges at any zoom.
   - **Rationale:** Stylized 2D games with zoom mechanics need visual quality at all scales.
   - **Verification:** Animate vector skeleton at 1x and 8x zoom. Assert alias-free edges.

## Tilemaps

6. **R-10.5.6** — The engine **SHALL** render chunked tile grids with multiple layers, auto-tiling,
   animated tiles, per-tile flip/rotation, and viewport culling for worlds with millions of tiles.
   - **Rationale:** Tilemap rendering must scale to large worlds through chunking and culling.
   - **Verification:** Load 1M tiles across 4 layers. Assert only visible chunks dispatched. Verify
     auto-tiling transitions.

7. **R-10.5.7** — The engine **SHALL** support isometric (diamond and staggered) and hexagonal grids
   with coordinate conversion, depth sorting, and height-stacked tiles.
   - **Rationale:** Isometric and hex grids are standard for strategy and RPG games.
   - **Verification:** Render each grid type. Verify screen-to-tile round-trip and depth sorting
     with stacked tiles.

8. **R-10.5.8** — The engine **SHALL** generate 2D tilemaps procedurally using WFC, noise, cellular
   automata, BSP, and room-and-corridor algorithms with deterministic seeding.
   - **Rationale:** Procedural generation enables replayable content without manual authoring.
   - **Verification:** Generate with fixed seed. Assert identical output across runs. Validate WFC
     adjacency constraints.

## Camera

9. **R-10.5.9** — The engine **SHALL** provide a 2D camera with position, zoom, rotation, parallax
   layers, smoothing, shake, and split-screen.
   - **Rationale:** A dedicated 2D camera is required for consistent scrolling and framing.
   - **Verification:** Configure camera with parallax and smoothing. Verify scroll rates, shake
     decay, and split-screen viewports.

## 2D Physics

10. **R-10.5.10** — The engine **SHALL** implement 2D rigid body physics as ECS components with CCD,
    one-way platforms, conveyor surfaces, and deterministic simulation.
    - **Rationale:** ECS-based 2D physics enables platformers and server-authoritative games.
    - **Verification:** Simulate 100 bodies for 1000 timesteps. Assert identical state across runs.
      Assert CCD prevents tunneling. Assert one-way platforms.

11. **R-10.5.11** — The engine **SHALL** support 2D collider shapes (box, circle, capsule, polygon,
    edge chain, composite) with tilemap auto-generated edge chains and per-tile properties.
    - **Rationale:** Optimized tilemap colliders reduce count by 90%+ versus per-tile colliders.
    - **Verification:** Generate colliders from 100x100 tilemap. Assert 90%+ reduction. Verify all
      shape types.

12. **R-10.5.12** — The engine **SHALL** provide 2D joints (revolute, prismatic, distance, spring,
    rope, weld, wheel, mouse) with motors, limits, and break force.
    - **Rationale:** Joints enable ragdolls, grappling hooks, and vehicle suspension.
    - **Verification:** Create each joint type. Verify motor limits and break force thresholds.

13. **R-10.5.13** — The engine **SHALL** support 2D ray casts, shape casts, and overlap tests using
    the shared spatial index with batch query support.
    - **Rationale:** Spatial queries are essential for AI, area-of-effect, and selection in 2D.
    - **Verification:** Perform queries against known geometry. Assert results match expected
      entity, point, normal, and distance.

## Lighting and Effects

14. **R-10.5.14** — The engine **SHALL** render dynamic 2D lighting with point lights, spotlights,
    ambient, and shadow casting from 2D occluders into a compositable light map.
    - **Rationale:** Dynamic lighting adds depth and is a baseline expectation for modern 2D games.
    - **Verification:** Place lights near occluders. Verify shadows, normal-mapped response, and
      emissive sprites.

15. **R-10.5.15** — The engine **SHALL** integrate 2D particle emitters with the sprite pipeline,
    sorted into 2D z-order, with trail/ribbon particles.
    - **Rationale:** Particles are critical for visual feedback and must integrate with 2D
      z-ordering.
    - **Verification:** Emit particles with velocity and gravity. Verify z-order interleaving with
      sprites and trail continuity.

## Mobile and Touch

16. **R-10.5.16** — The engine **SHALL** provide on-screen touch controls (virtual joystick, D-pad,
    buttons, gesture zones) as ECS entities feeding into the input action system (F-6.2.1), with
    layout adaptation.
    - **Rationale:** On-screen controls are required for mobile where physical gamepads are
      unavailable.
    - **Verification:** Render controls on mobile. Assert touch produces action events identical to
      gamepad. Assert layout adapts to orientation.

17. **R-10.5.17** — The engine **SHALL** map touch gestures to 2D game actions through the input
    action system (F-6.3.1) with rebindable mappings and simultaneous gesture support.
    - **Rationale:** Touch gestures enable intuitive mobile camera and interaction controls.
    - **Verification:** Perform each gesture. Assert mapped action fires. Assert simultaneous
      pinch+pan. Assert rebindable at runtime.

## Networking

18. **R-10.5.18** — The engine **SHALL** replicate 2D game state (Transform2D delta compression,
    sprite animation, tilemap chunks, 2D physics) with 2D-distance relevancy filtering.
    - **Rationale:** 2D-optimized replication reduces bandwidth by omitting the Z axis.
    - **Verification:** Replicate 200 entities. Assert delta compression reduces bandwidth. Assert
      entities outside relevancy culled.

19. **R-10.5.19** — The engine **SHALL** support client-side prediction with rollback netcode
    (GGPO-style) for 2D games with configurable input delay and rollback frames.
    - **Rationale:** Rollback netcode is essential for competitive 2D fighting and action games.
    - **Verification:** Simulate 2-player with latency. Inject misprediction. Assert rollback
      corrects state within configured frame window.

## Procedural Content

20. **R-10.5.20** — The engine **SHALL** generate infinite or bounded 2D worlds using noise biome
    distribution, temperature/moisture maps, and rule-based placement with chunk streaming.
    - **Rationale:** Procedural worlds enable open-world 2D RPGs without manual authoring.
    - **Verification:** Generate world. Assert chunk streaming loads only nearby regions. Assert
      deterministic seeding produces identical worlds.

21. **R-10.5.21** — The engine **SHALL** generate 2D room-based levels using BSP, random walk, and
    room-and-corridor algorithms with lock-and-key traversability guarantees.
    - **Rationale:** Traversability guarantees ensure dungeons are always completable.
    - **Verification:** Generate 100 dungeons. Assert 100% traversable. Assert keys before locked
      doors.

## 2.5D and Hybrid

22. **R-10.5.22** — The engine **SHALL** composite 3D elements into the 2D pipeline via
    `RenderLayer3D`, with 3D objects respecting 2D physics constraints.
    - **Rationale:** 2.5D enables side-scrollers with 3D characters on 2D backgrounds.
    - **Verification:** Render 3D character on 2D background. Assert correct z-order and 2D
      collision.

23. **R-10.5.23** — The engine **SHALL** support a 3D camera with gameplay physics constrained to a
    2D plane, with 2D sprites layered into the 3D scene with depth-correct sorting.
    - **Rationale:** HD-2D rendering combines 3D depth with 2D gameplay simplicity.
    - **Verification:** Render 3D scene with 2D physics on XZ plane. Verify 2D collisions and sprite
      sorting.

24. **R-10.5.24** — The engine **SHALL** support arbitrary layering of 2D and 3D assets with
    per-layer render mode, depth, and blend mode, resolving visibility via depth testing or
    painter's algorithm.
    - **Rationale:** Flexible composition enables mixed-media scenes with correct occlusion.
    - **Verification:** Compose sprite, mesh, tilemap, particle, and vector layers at interleaved
      depths. Assert correct occlusion and blending.

## Pixel-Perfect and Interpolation

25. **R-10.5.9a** — The engine **SHALL** provide a pixel-perfect camera mode that snaps Camera2D to
    integer pixel boundaries and applies integer upscaling on high-DPI displays.
    - **Rationale:** Pixel art games require crisp rendering without sub-pixel blurring on Retina
      and high-DPI screens.
    - **Verification:** Enable pixel-perfect mode on a 2x DPI display. Assert all sprites align to
      integer pixel boundaries. Assert no sub-pixel blurring.

26. **R-10.5.10a** — The engine **SHALL** interpolate Transform2D between previous and current
    physics positions for rendering when the physics fixed-step rate differs from the frame rate.
    - **Rationale:** Without interpolation, objects stutter when physics and render rates diverge.
    - **Verification:** Run physics at 50 Hz and rendering at 60 Hz. Assert smooth visual motion
      with no stutter or jitter.

## GPU Buffer Strategy

27. **R-10.5.1a** — The engine **SHALL** use ring-buffered GPU instance buffers for sprite, tilemap,
    and particle data to avoid CPU/GPU synchronization stalls.
    - **Rationale:** Ring buffering prevents the CPU from waiting on GPU consumption of the previous
      frame's data.
    - **Verification:** Profile a 10000-sprite scene with 3 frames in flight. Assert zero CPU/GPU
      sync stalls per frame.

28. **R-10.5.1b** — The engine **SHALL** allocate 2D hot-path scratch data (sprite lists, sort
    buffers, draw commands, physics manifolds) from per-thread arenas reset at frame boundaries.
    - **Rationale:** Arena allocation avoids per-frame heap allocation overhead in
      performance-critical 2D paths.
    - **Verification:** Profile a 10000-sprite frame. Assert zero heap allocations from hot-path
      systems after initial warm-up.

## Platform GPU Upload

29. **R-10.5.6a** — The engine **SHALL** upload tilemap chunk GPU buffers via DirectStorage
    (Windows) and Metal I/O (Apple) for reduced CPU overhead on chunk streaming.
    - **Rationale:** Platform-native GPU upload paths bypass CPU copies for faster tilemap chunk
      loading.
    - **Verification:** Stream 16 tilemap chunks. Assert GPU upload uses DirectStorage on Windows
      and Metal I/O on Apple. Assert no CPU staging copies.

## Viewport Composition

30. **R-10.5.22a** — The engine **SHALL** composite 2D and 3D render views via a configurable
    ViewportStack with back-to-front layer ordering.
    - **Rationale:** Games mixing 2D and 3D need deterministic composition order (3D scene, 2D
      world, 2D HUD, 2D debug overlay).
    - **Verification:** Configure a 4-layer ViewportStack. Assert layers composite in declared
      order. Assert HUD renders above world content.

## Shadow Caster

31. **R-10.5.14a** — The engine **SHALL** provide a ShadowCaster2D component with configurable
    occluder vertices and self-shadow control, independent of the Light2D component.
    - **Rationale:** Separating shadow casters from lights enables occluders on non-light-emitting
      geometry and per-entity self-shadow toggles.
    - **Verification:** Add ShadowCaster2D to an entity without Light2D. Assert shadows cast from
      that entity. Toggle self-shadow flag and assert correct behavior.
