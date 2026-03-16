# 2D Game Support Test Cases

Companion test cases for [2d-games.md](2d-games.md).

## Unit Tests

### TC-10.5.1.1 Sprite Batch by Atlas

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10K sprites across 4 atlas pages, 2 blend modes | Batch count = number of (atlas-page, blend-mode) combos | R-10.5.1 |

### TC-10.5.1.2 Sprite Z Order Sort

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sprites with layers [0, 1, 0] and z_orders [0.5, 0.1, 0.2] | Sorted: layer 0 z=0.2, layer 0 z=0.5, layer 1 z=0.1 | R-10.5.1 |

### TC-10.5.2.1 Anim Loop Mode

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 4-frame clip in Loop mode, advance past frame 3 | frame_index wraps to 0 | R-10.5.2 |

### TC-10.5.2.2 Anim PingPong

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 4-frame clip in PingPong, advance past frame 3 | Direction reverses, frame_index = 2 | R-10.5.2 |

### TC-10.5.2.3 Anim OneShot

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 4-frame clip in OneShot, advance past frame 3 | state = PlaybackState::Finished, frame_index = 3 | R-10.5.2 |

### TC-10.5.2.4 Anim Event Fire

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Clip with event at frame 2, advance to frame 2 | Event with matching event_id fired | R-10.5.2 |

### TC-10.5.6.1 Tilemap Ortho Coord

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Orthogonal grid, tile_size=(32,32), screen_pos=(64,96) | `screen_to_tile` returns IVec2(2, 3) | R-10.5.6 |
| 2 | `tile_to_world(IVec2(2, 3))` | Returns Vec2(64, 96) | R-10.5.6 |

### TC-10.5.7.1 Tilemap Iso Diamond

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Isometric diamond grid, known world position | Correct tile coordinate via round-trip conversion | R-10.5.7 |

### TC-10.5.7.2 Tilemap Hex Coord

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hex pointy-top grid, known world position | Correct hex coordinate via round-trip conversion | R-10.5.7 |

### TC-10.5.6.2 Tilemap Auto Tile

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Tile with neighbor_mask = 0b11001010 | Selects matching AutoTileRule UV rect | R-10.5.6 |

### TC-10.5.11.1 Tilemap Collider Merge

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100x100 tilemap with 50% solid tiles | Edge merge reduces colliders by >= 90% vs per-tile boxes | R-10.5.11 |

### TC-10.5.9.1 Camera Deadzone

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Target at Camera2D center within dead_zone rect | Camera position unchanged | R-10.5.9 |
| 2 | Target moves outside dead_zone | Camera follows target | R-10.5.9 |

### TC-10.5.9.2 Camera Pixel Snap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | pixel_perfect=true, ppu=16, position=(3.7, 5.2) | Snaps to (3.6875, 5.1875) [nearest 1/16] | R-10.5.9 |

### TC-10.5.9.3 Parallax Scroll Rate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | ParallaxLayer scroll_rate=0.5, camera moves 100px | Layer moves 50px | R-10.5.9 |

### TC-10.5.10.1 Rigidbody Gravity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Dynamic body, gravity_scale=1.0, dt=1/60 | Velocity.linear.y += gravity * dt | R-10.5.10 |

### TC-10.5.10.2 CCD Tunnel Prevention

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fast projectile (speed=5000) vs thin wall (2px) | CCD detects collision, no tunneling | R-10.5.10 |

### TC-10.5.10.3 One Way Platform

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Body moving upward through one_way collider | Passes through (no collision) | R-10.5.10 |
| 2 | Body moving downward onto one_way collider | Collides (rests on surface) | R-10.5.10 |

### TC-10.5.10.4 Deterministic Sim

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000 physics steps, fixed seed, run twice | Bit-identical results across both runs | R-10.5.10 |

### TC-10.5.12.1 Joint Revolute Limits

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Revolute joint with min=-45deg, max=45deg | Angle stays within [-45, 45] under torque | R-10.5.12 |

### TC-10.5.12.2 Joint Break Force

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Joint with break_force=Some(500.0), apply 600N | Joint breaks, entities separate | R-10.5.12 |

### TC-10.5.13.1 Ray Cast 2D

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ray from (0,0) direction (1,0), box at (5,0) | Hit at point near (4,0), normal=(-1,0) | R-10.5.13 |

### TC-10.5.13.2 Overlap 2D

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Circle r=10 at (0,0), 3 entities within, 2 outside | Returns 3 OverlapHit2D results | R-10.5.13 |

### TC-10.5.14.1 Light Shadow Cast

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Point light + rectangular shadow caster | Shadow map matches occluder silhouette | R-10.5.14 |

### TC-10.5.14.2 Normal Map Response

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Normal-mapped sprite, light at 45 degrees | Shading varies based on normal map direction | R-10.5.14 |

### TC-10.5.14.3 Emissive No Light

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Emissive2D sprite, ambient light = 0 | Sprite visible at emissive intensity | R-10.5.14 |

## Integration Tests

### TC-10.5.6.I1 Tilemap Stream Culling

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Large tilemap, Camera2D viewport covers 4 chunks | Only 4 visible chunks dispatched | R-10.5.6 |

### TC-10.5.1.I1 10K Sprites 60 FPS

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Render 10K sprites on desktop | Frame rate >= 60 fps | R-10.5.1 |

### TC-10.5.10.I1 Physics Collision Events

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two dynamic bodies collide | CollisionStart2D fires with correct entities and contact point | R-10.5.10 |
| 2 | Bodies separate | CollisionEnd2D fires with correct entities | R-10.5.10 |

### TC-10.5.9.I1 Split Screen Render

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two Camera2D viewports, split horizontally | Each renders independently with correct viewport clipping | R-10.5.9 |

### TC-10.5.14.I1 Mobile Light Cap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Spawn 16 Light2D on mobile tier | Only 8 active, light map at half resolution | R-10.5.14 |

### TC-10.5.7.I1 Iso Depth Sorting

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Overlapping isometric tiles at different positions | Correct depth sorting (back-to-front) | R-10.5.7 |

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
