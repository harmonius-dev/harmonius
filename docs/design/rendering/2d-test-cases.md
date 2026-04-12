# 2D Rendering — Test Cases

Companion to [2d.md](2d.md).

Test case IDs use `TC-10.5.Z.N` format. Every row links to a specific R-10.5.Z or F-10.5.Z.

## Unit Tests

| ID            | Name                              | Req        |
|---------------|-----------------------------------|------------|
| TC-10.5.1.1   | `test_sprite_batch_by_atlas`      | R-10.5.1   |
| TC-10.5.1.2   | `test_sprite_batch_by_blend_mode` | R-10.5.1   |
| TC-10.5.1.3   | `test_sprite_z_order_sort`        | R-10.5.1   |
| TC-10.5.1.4   | `test_sprite_layer_priority`      | R-10.5.1   |
| TC-10.5.2.1   | `test_animation_loop_mode`        | R-10.5.2   |
| TC-10.5.2.2   | `test_animation_pingpong_mode`    | R-10.5.2   |
| TC-10.5.2.3   | `test_animation_oneshot_mode`     | R-10.5.2   |
| TC-10.5.2.4   | `test_animation_event_fires`      | R-10.5.2   |
| TC-10.5.6.1   | `test_tilemap_chunk_cull`         | R-10.5.6   |
| TC-10.5.6.2   | `test_tilemap_auto_tile`          | R-10.5.6   |
| TC-10.5.7.1   | `test_iso_diamond_screen_to_tile` | R-10.5.7   |
| TC-10.5.7.2   | `test_hex_pointy_top_neighbors`   | R-10.5.7   |
| TC-10.5.9.1   | `test_camera_parallax_scroll`     | R-10.5.9   |
| TC-10.5.9.2   | `test_camera_shake_decay`         | R-10.5.9   |
| TC-10.5.9.3   | `test_pixel_perfect_snap`         | R-10.5.9a  |
| TC-10.5.10.1  | `test_rigidbody_dynamic_gravity`  | R-10.5.10  |
| TC-10.5.10.2  | `test_one_way_platform_drop`      | R-10.5.10  |
| TC-10.5.10.3  | `test_ccd_no_tunnel`              | R-10.5.10  |
| TC-10.5.11.1  | `test_polygon_collider_overlap`   | R-10.5.11  |
| TC-10.5.11.2  | `test_tilemap_edge_chain_reduce`  | R-10.5.11  |
| TC-10.5.12.1  | `test_joint_revolute_limits`      | R-10.5.12  |
| TC-10.5.13.1  | `test_2d_raycast_hit`             | R-10.5.13  |
| TC-10.5.13.2  | `test_2d_overlap_query`           | R-10.5.13  |
| TC-10.5.14.1  | `test_light2d_radius_falloff`     | R-10.5.14  |
| TC-10.5.14.2  | `test_shadow_caster_occluder`     | R-10.5.14a |
| TC-10.5.15.1  | `test_particle_z_interleave`      | R-10.5.15  |

1. **TC-10.5.1.1** `test_sprite_batch_by_atlas` — Spawn 100 sprites referencing atlas A and 100
   referencing atlas B. Run `SpriteBatchSystem`. Assert exactly two batches are produced.
   - Input: 200 entities `(Sprite { atlas_index, .. }, Transform2D)` with two distinct atlas IDs
   - Expected: `batches.len() == 2`, each batch has `instance_count == 100`, atlas IDs distinct

2. **TC-10.5.1.2** `test_sprite_batch_by_blend_mode` — 50 sprites use `BlendMode::AlphaBlend`, 50
   use `BlendMode::Additive`, all share one atlas. Assert two batches, one per blend mode.
   - Input: 100 sprites, one atlas, two `BlendMode` values evenly distributed
   - Expected: `batches.len() == 2`, batch 0 has `blend == AlphaBlend`, batch 1 has `Additive`

3. **TC-10.5.1.3** `test_sprite_z_order_sort` — Three sprites with `z_order` 0.5, 0.1, 0.9. Assert
   draw order after `SpriteSortSystem` is `[0.1, 0.5, 0.9]` (back to front).
   - Input: three sprites with `SpriteLayer { z_order: 0.5/0.1/0.9, sort_layer: 0 }`
   - Expected: sorted entity order indexes match z_order ascending

4. **TC-10.5.1.4** `test_sprite_layer_priority` — Sprite A has `sort_layer = 5`, sprite B has
   `sort_layer = 1`, B has higher z_order. Assert A draws after B because layer takes precedence.
   - Input: A `(sort_layer=5, z=0.0)`, B `(sort_layer=1, z=0.99)`
   - Expected: sorted order is `[B, A]`

5. **TC-10.5.2.1** `test_animation_loop_mode` — Clip with 4 frames at 10 fps in loop mode. Advance
   0.5 s. Assert `frame_index == 1` and elapsed wraps after 0.4 s.
   - Input: `SpriteAnimation { mode: Loop, playback_rate: 1.0, .. }`, 0.5 s tick
   - Expected: `frame_index == 1`, `elapsed == 0.1`, no `AnimationFinished` event

6. **TC-10.5.2.2** `test_animation_pingpong_mode` — Clip with 4 frames in ping-pong. Advance 8
   frames. Assert sequence is `[0, 1, 2, 3, 2, 1, 0, 1]`.
   - Input: `mode: PingPong`, single 100 ms tick repeated 8 times at 10 fps
   - Expected: recorded frame indices match `[0, 1, 2, 3, 2, 1, 0, 1]`

7. **TC-10.5.2.3** `test_animation_oneshot_mode` — Clip with 3 frames in one-shot. Tick past end.
   Assert `state == Finished`, `frame_index == 2`, one `AnimationFinished` event emitted.
   - Input: `mode: OneShot`, total tick time 0.5 s at 10 fps
   - Expected: `state == PlaybackState::Finished`, `events.len() == 1`

8. **TC-10.5.2.4** `test_animation_event_fires` — Clip has event `"footstep"` at frame 2. Tick from
   frame 0 past frame 2. Assert event fires exactly once with correct name.
   - Input: clip events `[{ frame: 2, name: "footstep" }]`, tick spans frames 0..3
   - Expected: events contain one `AnimationEvent { name: "footstep" }`

9. **TC-10.5.6.1** `test_tilemap_chunk_cull` — Tilemap with 16 chunks, camera viewport overlaps 4 of
   them. Assert only 4 chunks pass `tilemap_cull_system`.
   - Input: 4x4 chunk grid, 32x32 tiles per chunk, viewport covering top-left 2x2 region
   - Expected: `visible_chunks.len() == 4`, all chunks have `chunk_pos.x < 2 && chunk_pos.y < 2`

10. **TC-10.5.6.2** `test_tilemap_auto_tile` — 3x3 grid of tile id `1`; auto-tile rule maps the
    center to id `5` because all 4 cardinal neighbors match. Assert center tile resolves to `5`.
    - Input: 3x3 chunk, all `TileId(1)`, rule set with cardinal-match → `TileId(5)`
    - Expected: `chunk.tiles[center] == TileId(5)` after `apply_auto_tile`

11. **TC-10.5.7.1** `test_iso_diamond_screen_to_tile` — Convert tile `(2, 3)` to screen for a
    `(64, 32)` diamond, then back. Assert round-trip equals `(2, 3)`.
    - Input: `tile_to_screen(IVec2(2, 3), tile_size: Vec2(64, 32))` then inverse
    - Expected: `screen_to_tile(screen, tile_size) == IVec2(2, 3)`

12. **TC-10.5.7.2** `test_hex_pointy_top_neighbors` — `hex_neighbors(IVec2(2, 2))` for pointy-top
    hex. Assert returned set is the 6 axial neighbors.
    - Input: `IVec2(2, 2)`, layout `HexPointyTop`
    - Expected: 6 unique neighbors `[(3,2),(1,2),(2,1),(3,1),(2,3),(3,3)]` (axial)

13. **TC-10.5.9.1** `test_camera_parallax_scroll` — Camera moves 100 px right; layer with parallax
    factor `0.5`. Assert layer offset is 50 px.
    - Input: `Camera2D { position: Vec2(100, 0) }`, `ParallaxLayer { factor: 0.5 }`
    - Expected: `layer_world_offset == Vec2(50.0, 0.0)`

14. **TC-10.5.9.2** `test_camera_shake_decay` — Trigger shake amplitude 10 with 0.5 s duration. Tick
    0.25 s. Assert amplitude is approximately 5 (linear decay).
    - Input: `CameraShake { amplitude: 10.0, duration: 0.5, decay: Linear }`, dt = 0.25
    - Expected: `current_amplitude` within `[4.9, 5.1]`

15. **TC-10.5.9.3** `test_pixel_perfect_snap` — Camera at `(10.4, 7.8)` with `pixel_perfect = true`
    and 1 unit per pixel. Assert position snaps to `(10, 8)`.
    - Input: `Camera2D { position: Vec2(10.4, 7.8), pixel_perfect: true }`
    - Expected: rendered camera position == `Vec2(10.0, 8.0)`

16. **TC-10.5.10.1** `test_rigidbody_dynamic_gravity` — Dynamic body with `gravity_scale = 1.0`, no
    contacts. Step 1 s at 60 Hz. Assert vertical velocity ≈ -9.81.
    - Input: `RigidBody2D { body_type: Dynamic, gravity_scale: 1.0 }`, world gravity `(0, -9.81)`
    - Expected: `velocity.y` within `[-9.85, -9.77]` after 60 steps

17. **TC-10.5.10.2** `test_one_way_platform_drop` — Body above one-way platform falls and lands.
    Toggle drop-through; body falls through. Assert second body crosses platform y.
    - Input: platform at y=0 with `one_way: true`; body falls from y=5
    - Expected: first run lands at y=0; with drop-through flag, second run reaches y < 0

18. **TC-10.5.10.3** `test_ccd_no_tunnel` — Fast bullet (`velocity = (5000, 0)`) and 1-pixel wall.
    Step physics. Assert collision detected (no tunnel).
    - Input: bullet `RigidBody2D { ccd: true }`, wall collider 1 px wide
    - Expected: contact event emitted, bullet position clamped to wall surface

19. **TC-10.5.11.1** `test_polygon_collider_overlap` — Two convex polygon colliders overlap by SAT.
    Assert `overlap_test` returns `true` and contact normal direction is correct.
    - Input: two squares offset by `Vec2(0.5, 0.0)`, both half-extent 1
    - Expected: `overlap == true`, normal ≈ `Vec2(1, 0)`

20. **TC-10.5.11.2** `test_tilemap_edge_chain_reduce` — 100x100 solid tilemap collider generation.
    Assert per-tile collider count would be 10000 but generated edge chains reduce to ≤ 1000.
    - Input: 10000 solid tiles in a single rectangle region
    - Expected: `colliders.len() <= 1000`, one edge chain encloses the rectangle

21. **TC-10.5.12.1** `test_joint_revolute_limits` — Two bodies connected by `Joint2D::Revolute` with
    `AngleLimits { lower: -PI/4, upper: PI/4 }`. Apply torque past upper limit.
    - Input: dynamic A and B with revolute joint, applied torque = 100 N·m for 0.5 s
    - Expected: relative angle clamped to `PI/4 ± 0.001`, no penetration, joint reaction force
      non-zero

22. **TC-10.5.13.1** `test_2d_raycast_hit` — Ray from `(0,0)` direction `(1,0)` length 10. Box at
    `x=5`. Assert hit at `x ≈ 5`, normal `(-1, 0)`.
    - Input: `raycast_2d(origin: Vec2(0,0), dir: Vec2(1,0), max: 10.0)`
    - Expected: `Some(RayHit { point.x ≈ 5.0, normal: Vec2(-1, 0), entity: box_id })`

23. **TC-10.5.13.2** `test_2d_overlap_query` — Circle overlap query returns 3 entities inside,
    excludes 2 outside. Assert returned set matches expected entity IDs.
    - Input: 5 entities at known positions; query `circle_overlap(center, radius=2.0)`
    - Expected: `result == {e1, e2, e3}` (deterministic order)

24. **TC-10.5.14.1** `test_light2d_radius_falloff` — Point light radius 10 with quadratic falloff.
    Sample at distance 0, 5, 10. Assert intensities `[1.0, 0.25, 0.0]` within tolerance.
    - Input: `Light2D { radius: 10.0, falloff: Quadratic }`
    - Expected: sampled intensity vector matches `[1.0, 0.25, 0.0] ± 0.01`

25. **TC-10.5.14.2** `test_shadow_caster_occluder` — Light at `(0,0)`, occluder polygon at `(5,0)`,
    sample point at `(10,0)`. Assert shadow ray hits occluder, sample is in shadow.
    - Input: `ShadowCaster2D` polygon between light and sample
    - Expected: `is_shadowed(sample) == true`

26. **TC-10.5.15.1** `test_particle_z_interleave` — Particle z=0.4, sprite A z=0.3, sprite B z=0.5.
    Run sort. Assert draw order is `[A, particle, B]`.
    - Input: 1 particle and 2 sprites with the listed z values
    - Expected: sorted draw list is `[sprite_A, particle, sprite_B]`

## Integration Tests

| ID           | Name                            | Req       |
|--------------|---------------------------------|-----------|
| TC-10.5.I.1  | `test_10k_sprites_render`       | R-10.5.1  |
| TC-10.5.I.2  | `test_animation_to_event_chain` | R-10.5.2  |
| TC-10.5.I.3  | `test_tilemap_1m_load_cull`     | R-10.5.6  |
| TC-10.5.I.4  | `test_camera_split_screen`      | R-10.5.9  |
| TC-10.5.I.5  | `test_physics_determinism`      | R-10.5.10 |
| TC-10.5.I.6  | `test_lighting_full_scene`      | R-10.5.14 |
| TC-10.5.I.7  | `test_25d_layer_compose`        | R-10.5.22 |
| TC-10.5.1.I1  | `test_us_10_5_1_sprite_workflow`     | US-10.5.1  |
| TC-10.5.2.I1  | `test_us_10_5_2_frame_animation`     | US-10.5.2  |
| TC-10.5.3.I1  | `test_us_10_5_3_skeletal_animation`  | US-10.5.3  |
| TC-10.5.8.I1  | `test_us_10_5_8_orthogonal_tilemap`  | US-10.5.8  |
| TC-10.5.9.I1  | `test_us_10_5_9_iso_hex_tilemap`     | US-10.5.9  |
| TC-10.5.10.I1 | `test_us_10_5_10_camera_parallax`    | US-10.5.10 |
| TC-10.5.11.I1 | `test_us_10_5_11_pixel_perfect`      | US-10.5.11 |
| TC-10.5.13.I1 | `test_us_10_5_13_rigidbody_sim`      | US-10.5.13 |
| TC-10.5.14.I1 | `test_us_10_5_14_collision_events`   | US-10.5.14 |
| TC-10.5.15.I1 | `test_us_10_5_15_collider_shapes`    | US-10.5.15 |
| TC-10.5.16.I1 | `test_us_10_5_16_one_way_platform`   | US-10.5.16 |
| TC-10.5.17.I1 | `test_us_10_5_17_tilemap_collider`   | US-10.5.17 |
| TC-10.5.18.I1 | `test_us_10_5_18_joint_constraint`   | US-10.5.18 |
| TC-10.5.19.I1 | `test_us_10_5_19_spatial_query`      | US-10.5.19 |
| TC-10.5.20.I1 | `test_us_10_5_20_dynamic_lighting`   | US-10.5.20 |
| TC-10.5.21.I1 | `test_us_10_5_21_normal_mapped_2d`   | US-10.5.21 |
| TC-10.5.22.I1 | `test_us_10_5_22_particle_effects`   | US-10.5.22 |

1. **TC-10.5.I.1** `test_10k_sprites_render` — Spawn 10000 sprites across 4 atlas pages with mixed
   blend modes. Run a full frame. Assert batch count equals unique (atlas, blend) pairs.
   - Input: 10000 entities, 4 atlases, 2 blend modes evenly distributed
   - Expected: `batches.len() == 8`, total instance count == 10000, no GPU validation errors

2. **TC-10.5.I.2** `test_animation_to_event_chain` — Sprite plays animation with `attack_hit` event
   at frame 5; event triggers a damage system writing a `Damage` component on a target.
   - Input: animated sprite, target entity, event handler that writes `Damage(10)`
   - Expected: after the frame containing frame 5, target has `Damage` component with value 10

3. **TC-10.5.I.3** `test_tilemap_1m_load_cull` — Load a 1000x1000 tilemap (1M tiles, 1024 chunks).
   Camera covers only 16 chunks. Assert only those 16 are uploaded as GPU compute dispatches.
   - Input: 1M tilemap, camera viewport covering 16 chunks
   - Expected: `dispatched_chunks == 16`, remaining 1008 untouched, frame time within budget

4. **TC-10.5.I.4** `test_camera_split_screen` — Two `Camera2D` entities each with viewport rect
   `(0, 0, 0.5, 1)` and `(0.5, 0, 0.5, 1)`. Render frame. Assert two render passes per frame.
   - Input: two cameras, identical scene, distinct `render_layer` masks
   - Expected: 2 render passes, each writes to its viewport rect, layer mask filtering honored

5. **TC-10.5.I.5** `test_physics_determinism` — Run 1000 timesteps of 100 dynamic bodies twice with
   the same seed and inputs. Assert final positions byte-equal.
   - Input: deterministic seed, same initial RigidBody2D setup
   - Expected: `state_a == state_b`, hash of body positions equal

6. **TC-10.5.I.6** `test_lighting_full_scene` — Scene with 8 point lights, 4 spot lights, 16 shadow
   casters. Render. Assert light map texture written, all sample points correctly lit.
   - Input: scene fixture with above counts
   - Expected: light map non-zero in lit regions, zero in fully shadowed regions

7. **TC-10.5.I.7** `test_25d_layer_compose` — Compose `RenderLayer3D` (3D character) over a 2D
   tilemap background. Assert character pixels are above background but below 2D HUD.
   - Input: `ViewportStack { layers: [tilemap_2d, character_3d, hud_2d] }`
   - Expected: composited frame has correct layer ordering verified at sampled pixels

<!-- THIN: design section lacks detail -->
8. **TC-10.5.1.I1** `test_us_10_5_1_sprite_workflow` — Author and run an end-to-end sprite scene:
   load atlas, spawn sprites, render frame, verify pixels.
   - Input: 50 sprite entities loaded from atlas asset, single-camera scene
   - Expected: framebuffer matches reference image within 1% pixel diff

<!-- THIN: design section lacks detail -->
9. **TC-10.5.2.I1** `test_us_10_5_2_frame_animation` — Drive a frame-based sprite animation across a
   full game frame, verifying clip advancement and rendered output.
   - Input: 8-frame walk cycle clip at 12 fps, single sprite, 1 s simulation
   - Expected: frame index sequence matches expected; final framebuffer matches frame 7

<!-- THIN: design section lacks detail -->
10. **TC-10.5.3.I1** `test_us_10_5_3_skeletal_animation` — Bone-driven 2D skeletal mesh animates a
    character; verify deformed vertex output.
    - Input: 20-bone skeleton, 1 clip with rotation keys, 1 s playback
    - Expected: deformed vertex positions match reference within 0.01 unit

<!-- THIN: design section lacks detail -->
11. **TC-10.5.8.I1** `test_us_10_5_8_orthogonal_tilemap` — Load and render an orthogonal tilemap
    end-to-end; assert culling, dispatch, and pixel output.
    - Input: 100x100 orthogonal tilemap, camera viewport showing 16 chunks
    - Expected: visible chunks rendered, framebuffer matches reference

<!-- THIN: design section lacks detail -->
12. **TC-10.5.9.I1** `test_us_10_5_9_iso_hex_tilemap` — Load and render an isometric and a hex
    tilemap; assert correct projection and tile picking.
    - Input: 20x20 isometric map and 20x20 hex map
    - Expected: pixel at known tile center maps back to that tile via screen_to_tile

<!-- THIN: design section lacks detail -->
13. **TC-10.5.10.I1** `test_us_10_5_10_camera_parallax` — Camera scrolls 1000 px right; 3 parallax
    layers offset by their factors.
    - Input: 3 layers with factors `[0.25, 0.5, 1.0]`, camera moves 1000 px
    - Expected: layer offsets `[250, 500, 1000]` after frame; sampled framebuffer matches

<!-- THIN: design section lacks detail -->
14. **TC-10.5.11.I1** `test_us_10_5_11_pixel_perfect` — Pixel-perfect camera at fractional position
    snaps to integer pixels and renders without subpixel artifacts.
    - Input: camera at `(10.4, 7.8)`, `pixel_perfect = true`, sprite at integer position
    - Expected: rendered camera position `(10.0, 8.0)`, no subpixel jitter across 60 frames

<!-- THIN: design section lacks detail -->
15. **TC-10.5.13.I1** `test_us_10_5_13_rigidbody_sim` — End-to-end rigid body scene: drop 50 bodies
    on static floor; assert all settle without penetration.
    - Input: 50 dynamic bodies, 1 static floor, 5 s simulation at 60 Hz
    - Expected: all bodies at rest with `velocity.length() < 0.01`, no penetration

<!-- THIN: design section lacks detail -->
16. **TC-10.5.14.I1** `test_us_10_5_14_collision_events` — Two bodies collide; ECS observer receives
    `Collision2DEvent` with correct entity pair and contact data.
    - Input: dynamic body moving toward static, observer registered for collision events
    - Expected: 1 collision event emitted with `(a, b)` matching the two entities

<!-- THIN: design section lacks detail -->
17. **TC-10.5.15.I1** `test_us_10_5_15_collider_shapes` — Bodies with circle, box, polygon, and
    capsule colliders all collide correctly with each other.
    - Input: 4 bodies, one of each shape, configured to overlap pairwise
    - Expected: 6 collision events emitted (one per pair), all contact normals valid

<!-- THIN: design section lacks detail -->
18. **TC-10.5.16.I1** `test_us_10_5_16_one_way_platform` — Body lands on one-way platform from above
    and passes through from below.
    - Input: dynamic body, one-way platform, two scenarios (above and below)
    - Expected: lands at platform y from above; passes through from below

<!-- THIN: design section lacks detail -->
19. **TC-10.5.17.I1** `test_us_10_5_17_tilemap_collider` — Tilemap collider auto-generates from
    solid tiles; bodies collide with the generated edge chain.
    - Input: 32x32 tilemap with mixed solid tiles, 1 dynamic body
    - Expected: generated colliders match solid regions; body collides as expected

<!-- THIN: design section lacks detail -->
20. **TC-10.5.18.I1** `test_us_10_5_18_joint_constraint` — Two bodies linked by a revolute joint
    with motor; motor drives them through a full revolution.
    - Input: 2 bodies, `Joint2D::Revolute` with motor speed 2π rad/s, 1 s sim
    - Expected: relative angle changes by 2π ± 0.05; joint anchors stay attached

<!-- THIN: design section lacks detail -->
21. **TC-10.5.19.I1** `test_us_10_5_19_spatial_query` — Mixed raycast and overlap queries against a
    populated 2D world return correct hits.
    - Input: 100 entities, 10 raycasts and 10 circle overlaps with known answers
    - Expected: each query result matches the precomputed expected entity set

<!-- THIN: design section lacks detail -->
22. **TC-10.5.20.I1** `test_us_10_5_20_dynamic_lighting` — Dynamic point and spot lights cast on a
    2D scene with shadow casters; sample lit and shadowed pixels.
    - Input: 4 lights, 8 shadow casters, 16 sample positions
    - Expected: lit samples have non-zero intensity, shadowed samples are zero

<!-- THIN: design section lacks detail -->
23. **TC-10.5.21.I1** `test_us_10_5_21_normal_mapped_2d` — Sprite with normal map responds to a
    moving point light; specular highlight tracks light position.
    - Input: sprite with normal map, point light orbiting at 1 Hz
    - Expected: highlight pixel position correlates with light angle each frame

<!-- THIN: design section lacks detail -->
24. **TC-10.5.22.I1** `test_us_10_5_22_particle_effects` — Particle emitter spawns particles that
    integrate with sprite z sorting and render correctly.
    - Input: 1 emitter at z=0.5, 100 particles, 2 sprites at z=0.3 and z=0.7
    - Expected: particles drawn between sprites in z order, all particles visible

## Benchmarks

| ID           | Benchmark                      | Target    | Req        |
|--------------|--------------------------------|-----------|------------|
| TC-10.5.B.1  | Sprite extract (10k sprites)   | < 1.0 ms  | R-10.5.1a  |
| TC-10.5.B.2  | Sprite sort (10k sprites)      | < 0.5 ms  | R-10.5.1   |
| TC-10.5.B.3  | Tilemap chunk dispatch (1M)    | < 2.0 ms  | R-10.5.6   |
| TC-10.5.B.4  | 2D physics step (500 bodies)   | < 4.0 ms  | R-10.5.10  |
| TC-10.5.B.5  | 2D raycast batch (1k rays)     | < 0.8 ms  | R-10.5.13  |

1. **TC-10.5.B.1** — Extract 10000 visible sprites from ECS into the renderer snapshot. Wall time
   for `SpriteExtractSystem` only. Measured with `criterion`.

2. **TC-10.5.B.2** — Sort 10000 extracted sprite entries by `(sort_layer, z_order)`. Wall time for
   `SpriteSortSystem`. Per-thread arena, no heap allocs.

3. **TC-10.5.B.3** — Dispatch tilemap GPU compute for visible chunks of a 1M-tile world (16 visible
   chunks). Wall time from cull through draw command emission.

4. **TC-10.5.B.4** — Step 500 dynamic 2D rigid bodies with mixed colliders one fixed step at 60 Hz.
   Wall time for `Physics2DStep`. Includes broadphase and narrowphase.

5. **TC-10.5.B.5** — Batch 1000 raycasts against a tilemap-derived edge chain world. Wall time for
   the full batch. Result writeback into per-thread arena.
