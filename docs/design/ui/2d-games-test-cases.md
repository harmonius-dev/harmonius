# 2D Game Support Test Cases

Companion test cases for [2d-games.md](2d-games.md).

## Unit Tests

### TC-10.5.1.1 Sprite Batch by Atlas

| # | Requirement |
|---|-------------|
| 1 | R-10.5.1    |

1. **#1** — 10K sprites across 4 atlas pages, 2 blend modes
   - **Expected:** Batch count = number of (atlas-page, blend-mode) combos

### TC-10.5.1.2 Sprite Z Order Sort

| # | Requirement |
|---|-------------|
| 1 | R-10.5.1    |

1. **#1** — Sprites with layers [0, 1, 0] and z_orders [0.5, 0.1, 0.2]
   - **Expected:** Sorted: layer 0 z=0.2, layer 0 z=0.5, layer 1 z=0.1

### TC-10.5.2.1 Anim Loop Mode

| # | Requirement |
|---|-------------|
| 1 | R-10.5.2    |

1. **#1** — 4-frame clip in Loop mode, advance past frame 3
   - **Expected:** frame_index wraps to 0

### TC-10.5.2.2 Anim PingPong

| # | Requirement |
|---|-------------|
| 1 | R-10.5.2    |

1. **#1** — 4-frame clip in PingPong, advance past frame 3
   - **Expected:** Direction reverses, frame_index = 2

### TC-10.5.2.3 Anim OneShot

| # | Requirement |
|---|-------------|
| 1 | R-10.5.2    |

1. **#1** — 4-frame clip in OneShot, advance past frame 3
   - **Expected:** state = PlaybackState::Finished, frame_index = 3

### TC-10.5.2.4 Anim Event Fire

| # | Requirement |
|---|-------------|
| 1 | R-10.5.2    |

1. **#1** — Clip with event at frame 2, advance to frame 2
   - **Expected:** Event with matching event_id fired

### TC-10.5.6.1 Tilemap Ortho Coord

| # | Requirement |
|---|-------------|
| 1 | R-10.5.6    |
| 2 | R-10.5.6    |

1. **#1** — Orthogonal grid, tile_size=(32,32), screen_pos=(64,96)
   - **Expected:** `screen_to_tile` returns IVec2(2, 3)
2. **#2** — `tile_to_world(IVec2(2, 3))`
   - **Expected:** Returns Vec2(64, 96)

### TC-10.5.7.1 Tilemap Iso Diamond

| # | Requirement |
|---|-------------|
| 1 | R-10.5.7    |

1. **#1** — Isometric diamond grid, known world position
   - **Expected:** Correct tile coordinate via round-trip conversion

### TC-10.5.7.2 Tilemap Hex Coord

| # | Requirement |
|---|-------------|
| 1 | R-10.5.7    |

1. **#1** — Hex pointy-top grid, known world position
   - **Expected:** Correct hex coordinate via round-trip conversion

### TC-10.5.6.2 Tilemap Auto Tile

| # | Requirement |
|---|-------------|
| 1 | R-10.5.6    |

1. **#1** — Tile with neighbor_mask = 0b11001010
   - **Expected:** Selects matching AutoTileRule UV rect

### TC-10.5.11.1 Tilemap Collider Merge

| # | Requirement |
|---|-------------|
| 1 | R-10.5.11   |

1. **#1** — 100x100 tilemap with 50% solid tiles
   - **Expected:** Edge merge reduces colliders by >= 90% vs per-tile boxes

### TC-10.5.9.1 Camera Deadzone

| # | Requirement |
|---|-------------|
| 1 | R-10.5.9    |
| 2 | R-10.5.9    |

1. **#1** — Target at Camera2D center within dead_zone rect
   - **Expected:** Camera position unchanged
2. **#2** — Target moves outside dead_zone
   - **Expected:** Camera follows target

### TC-10.5.9.2 Camera Pixel Snap

| # | Requirement |
|---|-------------|
| 1 | R-10.5.9    |

1. **#1** — pixel_perfect=true, ppu=16, position=(3.7, 5.2)
   - **Expected:** Snaps to (3.6875, 5.1875) [nearest 1/16]

### TC-10.5.9.3 Parallax Scroll Rate

| # | Requirement |
|---|-------------|
| 1 | R-10.5.9    |

1. **#1** — ParallaxLayer scroll_rate=0.5, camera moves 100px
   - **Expected:** Layer moves 50px

### TC-10.5.10.1 Rigidbody Gravity

| # | Requirement |
|---|-------------|
| 1 | R-10.5.10   |

1. **#1** — Dynamic body, gravity_scale=1.0, dt=1/60
   - **Expected:** Velocity.linear.y += gravity * dt

### TC-10.5.10.2 CCD Tunnel Prevention

| # | Requirement |
|---|-------------|
| 1 | R-10.5.10   |

1. **#1** — Fast projectile (speed=5000) vs thin wall (2px)
   - **Expected:** CCD detects collision, no tunneling

### TC-10.5.10.3 One Way Platform

| # | Requirement |
|---|-------------|
| 1 | R-10.5.10   |
| 2 | R-10.5.10   |

1. **#1** — Body moving upward through one_way collider
   - **Expected:** Passes through (no collision)
2. **#2** — Body moving downward onto one_way collider
   - **Expected:** Collides (rests on surface)

### TC-10.5.10.4 Deterministic Sim

| # | Requirement |
|---|-------------|
| 1 | R-10.5.10   |

1. **#1** — 1000 physics steps, fixed seed, run twice
   - **Expected:** Bit-identical results across both runs

### TC-10.5.12.1 Joint Revolute Limits

| # | Requirement |
|---|-------------|
| 1 | R-10.5.12   |

1. **#1** — Revolute joint with min=-45deg, max=45deg
   - **Expected:** Angle stays within [-45, 45] under torque

### TC-10.5.12.2 Joint Break Force

| # | Requirement |
|---|-------------|
| 1 | R-10.5.12   |

1. **#1** — Joint with break_force=Some(500.0), apply 600N
   - **Expected:** Joint breaks, entities separate

### TC-10.5.13.1 Ray Cast 2D

| # | Requirement |
|---|-------------|
| 1 | R-10.5.13   |

1. **#1** — Ray from (0,0) direction (1,0), box at (5,0)
   - **Expected:** Hit at point near (4,0), normal=(-1,0)

### TC-10.5.13.2 Overlap 2D

| # | Requirement |
|---|-------------|
| 1 | R-10.5.13   |

1. **#1** — Circle r=10 at (0,0), 3 entities within, 2 outside
   - **Expected:** Returns 3 OverlapHit2D results

### TC-10.5.14.1 Light Shadow Cast

| # | Requirement |
|---|-------------|
| 1 | R-10.5.14   |

1. **#1** — Point light + rectangular shadow caster
   - **Expected:** Shadow map matches occluder silhouette

### TC-10.5.14.2 Normal Map Response

| # | Requirement |
|---|-------------|
| 1 | R-10.5.14   |

1. **#1** — Normal-mapped sprite, light at 45 degrees
   - **Expected:** Shading varies based on normal map direction

### TC-10.5.14.3 Emissive No Light

| # | Requirement |
|---|-------------|
| 1 | R-10.5.14   |

1. **#1** — Emissive2D sprite, ambient light = 0
   - **Expected:** Sprite visible at emissive intensity

## Integration Tests

### TC-10.5.6.I1 Tilemap Stream Culling

| # | Requirement |
|---|-------------|
| 1 | R-10.5.6    |

1. **#1** — Large tilemap, Camera2D viewport covers 4 chunks
   - **Expected:** Only 4 visible chunks dispatched

### TC-10.5.1.I1 10K Sprites 60 FPS

| # | Requirement |
|---|-------------|
| 1 | R-10.5.1    |

1. **#1** — Render 10K sprites on desktop
   - **Expected:** Frame rate >= 60 fps

### TC-10.5.10.I1 Physics Collision Events

| # | Requirement |
|---|-------------|
| 1 | R-10.5.10   |
| 2 | R-10.5.10   |

1. **#1** — Two dynamic bodies collide
   - **Expected:** CollisionStart2D fires with correct entities and contact point
2. **#2** — Bodies separate
   - **Expected:** CollisionEnd2D fires with correct entities

### TC-10.5.9.I1 Split Screen Render

| # | Requirement |
|---|-------------|
| 1 | R-10.5.9    |

1. **#1** — Two Camera2D viewports, split horizontally
   - **Expected:** Each renders independently with correct viewport clipping

### TC-10.5.14.I1 Mobile Light Cap

| # | Requirement |
|---|-------------|
| 1 | R-10.5.14   |

1. **#1** — Spawn 16 Light2D on mobile tier
   - **Expected:** Only 8 active, light map at half resolution

### TC-10.5.7.I1 Iso Depth Sorting

| # | Requirement |
|---|-------------|
| 1 | R-10.5.7    |

1. **#1** — Overlapping isometric tiles at different positions
   - **Expected:** Correct depth sorting (back-to-front)

## Benchmarks

### TC-10.5.1.B1 Sprite Batch Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Render 10K sprites | Frame time | < 2 ms | R-10.5.1 |

### TC-10.5.6.B1 Tilemap Chunk Dispatch

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1M tiles across visible chunks | Dispatch time | < 4 ms | R-10.5.6 |

### TC-10.5.10.B1 Physics Step 200 Bodies

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 200 dynamic rigid bodies on mobile | Step time | < 2 ms | R-10.5.10 |

### TC-10.5.13.B1 Ray Cast 1000 Rays

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1000 ray casts against spatial index | Total time | < 0.5 ms | R-10.5.13 |

### TC-10.5.14.B1 Light Map Composite

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 8 Light2D with shadow casters on mobile | Composite time | < 1 ms | R-10.5.14 |
