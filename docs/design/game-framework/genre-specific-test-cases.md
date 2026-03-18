# Genre-Specific Systems Test Cases

Companion test cases for [genre-specific.md](genre-specific.md).

## Unit Tests

### TC-13.20.1.1 Fog Three States

| # | Requirement |
|---|-------------|
| 1 | R-13.20.1   |
| 2 | R-13.20.1   |

1. **#1** — `set(5,5,faction_0,Visible); get(5,5,faction_0)`
   - **Expected:** `FogState::Visible`
2. **#2** — `get(0,0,faction_0)` (never set)
   - **Expected:** `FogState::Unexplored`

### TC-13.20.1.2 Fog 2-Bit Encoding

| # | Requirement |
|---|-------------|
| 1 | R-13.20.1   |

1. **#1** — 256x256 grid; serialize_full for 1 faction
   - **Expected:** Payload = 16,384 bytes

### TC-NFR-13.20.2.1 Fog Delta 90 Percent

| # | Requirement |
|---|-------------|
| 1 | NFR-13.20.2 |

1. **#1** — 256x256 grid; 5% cell change; compute delta
   - **Expected:** Delta >= 90% smaller than full payload

### TC-13.20.2.1 Vision Circle

| # | Requirement |
|---|-------------|
| 1 | R-13.20.2   |

1. **#1** — VisionSource radius=10 at (50,50)
   - **Expected:** Cells within 10 tiles set to Visible

### TC-13.20.2.2 Vision LOS Blocked

| # | Requirement |
|---|-------------|
| 1 | R-13.20.2   |

1. **#1** — Wall at (55,50); source at (50,50) radius=10
   - **Expected:** Cells behind wall remain Unexplored

### TC-13.20.2.3 Vision Elevation

| # | Requirement |
|---|-------------|
| 1 | R-13.20.2   |

1. **#1** — Base radius=10, elevation_bonus=1, elevation=3
   - **Expected:** Effective radius = 13

### TC-13.20.3.1 Vision Modifier Stealth

| # | Requirement |
|---|-------------|
| 1 | R-13.20.3   |

1. **#1** — Entity inside StealthZone volume
   - **Expected:** Invisible to outside vision sources

### TC-13.20.4.1 Fog Memory Persist

| # | Requirement |
|---|-------------|
| 1 | R-13.20.4   |

1. **#1** — Building seen, then shrouded
   - **Expected:** Ghost building persists in FogMemory

### TC-13.21.1.1 Grid Reachable Cells

| # | Requirement |
|---|-------------|
| 1 | R-13.21.1   |

1. **#1** — Origin (5,5), max_ap=3, flat terrain
   - **Expected:** BFS returns all cells within 3 steps

### TC-13.21.1.2 Grid Hex Adjacency

| # | Requirement |
|---|-------------|
| 1 | R-13.21.1   |

1. **#1** — Hex grid; query neighbors of (3,3)
   - **Expected:** Correct 6 neighbors for odd row

### TC-13.21.2.1 Initiative Order

| # | Requirement |
|---|-------------|
| 1 | R-13.21.2   |

1. **#1** — Speeds: 10, 20, 15, 5
   - **Expected:** Turn order: 20, 15, 10, 5

### TC-13.21.2.2 Team Turn Order

| # | Requirement |
|---|-------------|
| 1 | R-13.21.2   |

1. **#1** — Team A (3 units), Team B (2 units); TeamBased mode
   - **Expected:** All of Team A acts, then all of Team B

### TC-13.21.3.1 AP Movement

| # | Requirement |
|---|-------------|
| 1 | R-13.21.3   |
| 2 | R-13.21.3   |

1. **#1** — 3 flat cells (cost=1 each)
   - **Expected:** Total AP cost = 3
2. **#2** — 3 difficult terrain cells (cost=2 each)
   - **Expected:** Total AP cost = 6

### TC-13.21.3.2 AP Interleave

| # | Requirement |
|---|-------------|
| 1 | R-13.21.3   |

1. **#1** — Move (2 AP), attack (3 AP), move (1 AP); budget=6
   - **Expected:** All actions succeed; total AP = 6

### TC-13.21.4.1 Cover Directional

| # | Requirement |
|---|-------------|
| 1 | R-13.21.4   |
| 2 | R-13.21.4   |

1. **#1** — Half cover from north edge; attack from north
   - **Expected:** CoverValue::Half applied
2. **#2** — Same cover; attack from east (flanking)
   - **Expected:** CoverValue::None applied

### TC-13.21.4.2 Overwatch Triggers

| # | Requirement |
|---|-------------|
| 1 | R-13.21.4   |

1. **#1** — Enemy moves through overwatch arc
   - **Expected:** Overwatch shot triggered; triggered=true

### TC-13.21.5.1 Hit Probability

| # | Requirement |
|---|-------------|
| 1 | R-13.21.5   |

1. **#1** — 80% base - 10% range - 20% cover
   - **Expected:** Hit chance = 50%

### TC-13.21.5.2 Combat Outcomes

| # | Requirement |
|---|-------------|
| 1 | R-13.21.5   |

1. **#1** — 1000 rolls at 50% hit chance
   - **Expected:** Hit count within 95% CI [460, 540]

### TC-13.26.1.1 Minigame Isolation

| # | Requirement |
|---|-------------|
| 1 | R-13.26.1   |

1. **#1** — Access outer world component from minigame partition
   - **Expected:** Access fails; isolated

### TC-13.26.3.1 Minigame Contract

| # | Requirement |
|---|-------------|
| 1 | R-13.26.3   |

1. **#1** — Complete minigame; apply results
   - **Expected:** All outputs applied atomically

### TC-13.26.3.2 Minigame Quit Policy

| # | Requirement |
|---|-------------|
| 1 | R-13.26.3   |
| 2 | R-13.26.3   |

1. **#1** — Quit with QuitPolicy::Refund
   - **Expected:** Entry cost refunded
2. **#2** — Quit with QuitPolicy::Loss
   - **Expected:** Loss outcome applied

### TC-13.26.4.1 Timing Windows

| # | Requirement |
|---|-------------|
| 1 | R-13.26.4   |
| 2 | R-13.26.4   |

1. **#1** — Input at beat +25ms (perfect_ms=30)
   - **Expected:** Grade = Perfect
2. **#2** — Input at beat +50ms (great_ms=60)
   - **Expected:** Grade = Great

### TC-13.26.5b.1 Match 3 Detection

| # | Requirement |
|---|-------------|
| 1 | R-13.26.5b  |
| 2 | R-13.26.5b  |

1. **#1** — 3 same pieces in horizontal row
   - **Expected:** `MatchPattern::ThreeInARow` detected
2. **#2** — 3 same pieces in vertical column
   - **Expected:** `MatchPattern::ThreeInARow` detected

### TC-13.26.5d.1 Cascade Recursive

| # | Requirement |
|---|-------------|
| 1 | R-13.26.5d  |

1. **#1** — Clear match causing gravity fill with new match
   - **Expected:** Recursive cascade triggered; second match detected

### TC-13.26.5c.1 Board AI Difficulty

| # | Requirement |
|---|-------------|
| 1 | R-13.26.5c  |
| 2 | R-13.26.5c  |

1. **#1** — Easy AI vs random baseline (100 games)
   - **Expected:** Easy AI win rate < 30%
2. **#2** — Hard AI vs random baseline (100 games)
   - **Expected:** Hard AI win rate > 70%

### TC-13.26.6.1 Physics Determinism

| # | Requirement |
|---|-------------|
| 1 | R-13.26.6   |

1. **#1** — Identical physics inputs on 2 runs
   - **Expected:** Identical output states

### TC-13.22.1.1 Checkpoint Order

| # | Requirement |
|---|-------------|
| 1 | R-13.22.1   |

1. **#1** — Skip checkpoint 2; cross start/finish
   - **Expected:** Lap NOT incremented

### TC-13.22.1.2 Lap Counting

| # | Requirement |
|---|-------------|
| 1 | R-13.22.1   |

1. **#1** — Pass all checkpoints; cross start/finish
   - **Expected:** current_lap increments by 1

### TC-13.22.1.3 Split Times

| # | Requirement |
|---|-------------|
| 1 | R-13.22.1   |

1. **#1** — Current time=45s, best=40s at checkpoint 3
   - **Expected:** Split time = +5s

### TC-13.22.2.1 Race Modes

| # | Requirement |
|---|-------------|
| 1 | R-13.22.2   |
| 2 | R-13.22.2   |

1. **#1** — Circuit (3 laps); complete 3 laps
   - **Expected:** Race ends; finish_position assigned
2. **#2** — TimeTrial; cross finish
   - **Expected:** Total time recorded

### TC-13.22.3b.1 Rubber Banding

| # | Requirement |
|---|-------------|
| 1 | R-13.22.3b  |
| 2 | R-13.22.3b  |

1. **#1** — AI trailing by max_gap_sec
   - **Expected:** AI speed multiplied by trailing_boost
2. **#2** — AI leading by max_gap_sec
   - **Expected:** AI speed multiplied by leading_penalty

### TC-13.22.4.1 Drift Detection

| # | Requirement |
|---|-------------|
| 1 | R-13.22.4   |
| 2 | R-13.22.4   |

1. **#1** — Slip angle = threshold + 5 deg
   - **Expected:** `DriftState.drifting` = true
2. **#2** — Slip angle = threshold - 5 deg
   - **Expected:** `DriftState.drifting` = false

### TC-13.22.4.2 Drift Combo

| # | Requirement |
|---|-------------|
| 1 | R-13.22.4   |

1. **#1** — Drift for 3 seconds continuously
   - **Expected:** combo_multiplier increases

### TC-13.22.4.3 Boost Fill

| # | Requirement |
|---|-------------|
| 1 | R-13.22.4   |

1. **#1** — Drift score accumulated during drift
   - **Expected:** Boost meter fills proportionally to drift score

### TC-NFR-13.22.2.1 Ghost Storage 10KB

| # | Requirement |
|---|-------------|
| 1 | NFR-13.22.2 |

1. **#1** — Record 1 minute ghost; compress
   - **Expected:** Compressed size <= 10 KB

### TC-13.22.5.1 Ghost Record Replay

| # | Requirement |
|---|-------------|
| 1 | R-13.22.5   |

1. **#1** — Record 1 lap; replay ghost
   - **Expected:** Ghost positions match recorded positions

### TC-13.27.1.1 Block Registry O(1)

| # | Requirement |
|---|-------------|
| 1 | R-13.27.1   |

1. **#1** — Register 1024 types; lookup by id
   - **Expected:** Correct BlockTypeDef returned in O(1)

### TC-13.27.2.1 Block Placement

| # | Requirement |
|---|-------------|
| 1 | R-13.27.2   |

1. **#1** — Raycast targets face of existing block
   - **Expected:** New block placed on correct face

### TC-13.27.3.1 Palette Compression

| # | Requirement |
|---|-------------|
| 1 | R-13.27.3   |

1. **#1** — Chunk with 8 unique types
   - **Expected:** 4-bit palette; storage = 2048 bytes

### TC-13.27.3.2 Uniform Chunk

| # | Requirement |
|---|-------------|
| 1 | R-13.27.3   |

1. **#1** — All-air chunk
   - **Expected:** Stored as single value (minimal bytes)

### TC-13.27.4.1 Greedy Meshing

| # | Requirement |
|---|-------------|
| 1 | R-13.27.4   |

1. **#1** — 16x16 flat surface of same block
   - **Expected:** Single quad per face direction

### TC-13.27.4.2 Interior Culling

| # | Requirement |
|---|-------------|
| 1 | R-13.27.4   |

1. **#1** — Block fully enclosed by other blocks
   - **Expected:** Zero mesh faces generated

### TC-13.27.5.1 Light Propagation

| # | Requirement |
|---|-------------|
| 1 | R-13.27.5   |

1. **#1** — Torch (emission=15) placed in dark area
   - **Expected:** Light attenuates to 0 at 15 blocks

### TC-13.27.5.2 Sunlight Flood

| # | Requirement |
|---|-------------|
| 1 | R-13.27.5   |

1. **#1** — Sunlight column above open terrain
   - **Expected:** Sunlight propagates down at full intensity (level 15)

### TC-13.27.6a.1 Gravity Block

| # | Requirement |
|---|-------------|
| 1 | R-13.27.6a  |

1. **#1** — Sand block with no support below
   - **Expected:** Falls to nearest solid surface

### TC-13.27.6b.1 Fluid Flow Levels

| # | Requirement |
|---|-------------|
| 1 | R-13.27.6b  |

1. **#1** — Water source block
   - **Expected:** Flows with 7 decreasing levels

### TC-13.27.6c.1 Fluid Interaction

| # | Requirement |
|---|-------------|
| 1 | R-13.27.6c  |

1. **#1** — Lava meets water
   - **Expected:** Cobblestone block generated

### TC-13.27.7a.1 Signal Attenuation

| # | Requirement |
|---|-------------|
| 1 | R-13.27.7a  |

1. **#1** — Signal source; measure at 15 and 16 blocks
   - **Expected:** Strength > 0 at 15; strength = 0 at 16

### TC-13.27.7b.1 Not Gate

| # | Requirement |
|---|-------------|
| 1 | R-13.27.7b  |

1. **#1** — Torch on powered block
   - **Expected:** Output = inverted signal

### TC-13.27.7c.1 Piston Push

| # | Requirement |
|---|-------------|
| 1 | R-13.27.7c  |

1. **#1** — Power piston with adjacent block
   - **Expected:** Piston pushes adjacent block one cell

### TC-13.27.7d.1 Circuit Determinism

| # | Requirement |
|---|-------------|
| 1 | R-13.27.7d  |

1. **#1** — Same circuit on 2 clients
   - **Expected:** Identical output state

### TC-13.27.7d.2 Circuit Budget

| # | Requirement |
|---|-------------|
| 1 | R-13.27.7d  |

1. **#1** — Exceed redstone evaluation budget
   - **Expected:** Warning emitted; excess circuits depowered

### TC-13.27.8a.1 Terrain Seed Determinism

| # | Requirement |
|---|-------------|
| 1 | R-13.27.8a  |

1. **#1** — Same seed on 2 platforms
   - **Expected:** Identical terrain output

### TC-13.27.8b.1 Biome Composition

| # | Requirement |
|---|-------------|
| 1 | R-13.27.8b  |
| 2 | R-13.27.8b  |

1. **#1** — Generate plains biome chunk
   - **Expected:** Top layers = grass/dirt
2. **#2** — Generate desert biome chunk
   - **Expected:** Top layers = sand

### TC-13.15.1.1 Companion Follow

| # | Requirement |
|---|-------------|
| 1 | R-13.15.1   |

1. **#1** — Player moves 5m; companion follows
   - **Expected:** Companion maintains follow distance

### TC-13.15.1.2 Companion Teleport

| # | Requirement |
|---|-------------|
| 1 | R-13.15.1   |

1. **#1** — Player moves beyond teleport threshold
   - **Expected:** Companion teleports to player vicinity

### TC-13.15.2.1 Pet Needs Decay

| # | Requirement |
|---|-------------|
| 1 | R-13.15.2   |

1. **#1** — Pet needs drain to zero
   - **Expected:** Pet refuses commands

### TC-13.15.3b.1 Mount Locomotion

| # | Requirement |
|---|-------------|
| 1 | R-13.15.3b  |

1. **#1** — Mount summoned; player rides
   - **Expected:** Player physics replaced by mount physics

### TC-13.15.3c.1 Mounted Combat

| # | Requirement |
|---|-------------|
| 1 | R-13.15.3c  |

1. **#1** — Attempt ground-only ability while mounted
   - **Expected:** Ability blocked; only mounted abilities available

### TC-13.15.3d.1 Flying Mount Altitude

| # | Requirement |
|---|-------------|
| 1 | R-13.15.3d  |

1. **#1** — Flying mount at altitude limit
   - **Expected:** Movement clamped; cannot exceed ceiling

### TC-13.15.4.1 Taming Progress

| # | Requirement |
|---|-------------|
| 1 | R-13.15.4   |

1. **#1** — Feed creature 3 times
   - **Expected:** Taming progress advances 3 increments

### TC-13.15.4.2 Taming Probability

| # | Requirement |
|---|-------------|
| 1 | R-13.15.4   |

1. **#1** — Player level 5 vs creature level 10
   - **Expected:** Success rate reduced by level deficit

### TC-13.15.5a.1 Life Stages

| # | Requirement |
|---|-------------|
| 1 | R-13.15.5a  |

1. **#1** — Advance time through growth thresholds
   - **Expected:** Baby -> juvenile -> adult with stat changes

### TC-13.15.5b.1 Evolution Branch

| # | Requirement |
|---|-------------|
| 1 | R-13.15.5b  |

1. **#1** — Feed meat diet during juvenile stage
   - **Expected:** Evolves to combat wolf specialization

### TC-13.15.5c.1 Breeding Inheritance

| # | Requirement |
|---|-------------|
| 1 | R-13.15.5c  |

1. **#1** — Breed parent A (speed trait) + parent B (strength trait)
   - **Expected:** Offspring inherits subset of parent traits

## Integration Tests

### TC-NFR-13.20.1.I1 Fog GPU Texture

| # | Requirement |
|---|-------------|
| 1 | NFR-13.20.1 |

1. **#1** — 512x512 grid, 128 vision sources
   - **Expected:** GPU fog texture gen < 0.5 ms

### TC-NFR-13.20.1.I2 Fog CPU Fallback

| # | Requirement |
|---|-------------|
| 1 | NFR-13.20.1 |

1. **#1** — 512x512 grid, 128 vision sources; CPU path
   - **Expected:** CPU fog compute < 2 ms

### TC-NFR-13.21.1.I1 Grid Pathfind Perf

| # | Requirement |
|---|-------------|
| 1 | NFR-13.21.1 |

1. **#1** — 100x100 grid pathfinding
   - **Expected:** Completes in < 2 ms

### TC-NFR-13.21.2.I1 Turn Transition

| # | Requirement |
|---|-------------|
| 1 | NFR-13.21.2 |

1. **#1** — End turn; camera focus + UI update
   - **Expected:** Transition completes in < 500 ms

### TC-NFR-13.26.1.I1 Minigame Create Teardown

| # | Requirement |
|---|-------------|
| 1 | NFR-13.26.1 |

1. **#1** — Create minigame session; teardown
   - **Expected:** Create < 500 ms; teardown < 200 ms; zero leak

### TC-NFR-13.26.1.I2 Minigame 100 Cycles

| # | Requirement |
|---|-------------|
| 1 | NFR-13.26.1 |

1. **#1** — 100 create/destroy cycles
   - **Expected:** Zero net memory allocation

### TC-NFR-13.26.2.I1 Rhythm Input Latency

| # | Requirement |
|---|-------------|
| 1 | NFR-13.26.2 |

1. **#1** — Audio beat event to detection pipeline
   - **Expected:** Audio-to-detection latency < 16 ms

### TC-NFR-13.22.1.I1 Racing Tick Precision

| # | Requirement |
|---|-------------|
| 1 | NFR-13.22.1 |

1. **#1** — 10 identical laps
   - **Expected:** Lap time variance < 1 ms

### TC-NFR-13.27.1.I1 Chunk Mesh Perf

| # | Requirement |
|---|-------------|
| 1 | NFR-13.27.1 |

1. **#1** — Mesh 1000 chunks
   - **Expected:** Average < 2 ms per chunk

### TC-NFR-13.27.1.I2 32k Chunks 60fps

| # | Requirement |
|---|-------------|
| 1 | NFR-13.27.1 |

1. **#1** — 32,768 loaded chunks
   - **Expected:** Sustained 60 fps

### TC-NFR-13.27.1.I3 Block Modify Latency

| # | Requirement |
|---|-------------|
| 1 | NFR-13.27.1 |

1. **#1** — Place block; measure input to visual update
   - **Expected:** Latency < 16 ms

### TC-NFR-13.15.1.I1 Companion 8 Budget

| # | Requirement |
|---|-------------|
| 1 | NFR-13.15.1 |

1. **#1** — 8 companions with full AI
   - **Expected:** Total AI < 4 ms/frame

### TC-NFR-13.15.2.I1 Mount Swap Latency

| # | Requirement |
|---|-------------|
| 1 | NFR-13.15.2 |

1. **#1** — Locomotion mode transition
   - **Expected:** Completes in < 100 ms

## Benchmarks

### TC-NFR-13.20.1.B1 Fog GPU Texture Gen

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 512x512 grid, 128 sources | GPU time | < 0.5 ms | NFR-13.20.1 |

### TC-NFR-13.20.2.B1 Fog Network Payload

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 512x512 grid, 4 factions full sync | Payload size | <= 262,144 bytes | NFR-13.20.2 |

### TC-NFR-13.21.1.B1 Grid Pathfinding

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | A* on 100x100 grid | Wall-clock time | < 2 ms | NFR-13.21.1 |

### TC-NFR-13.21.1.B2 Hit Probability Compute

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single hit probability calculation | Wall-clock time | < 0.1 ms | NFR-13.21.1 |

### TC-NFR-13.26.1.B1 Minigame Session Create

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Create minigame session | Wall-clock time | < 500 ms | NFR-13.26.1 |

### TC-NFR-13.26.2.B1 Rhythm Input Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Audio-to-detection | Latency | < 16 ms | NFR-13.26.2 |

### TC-NFR-13.22.1.B1 Racing Lap Variance

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10 identical laps | Time variance | < 1 ms | NFR-13.22.1 |

### TC-NFR-13.22.2.B1 Ghost Storage

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1 minute ghost recording | Storage size | <= 10 KB | NFR-13.22.2 |

### TC-NFR-13.27.1.B1 Chunk Meshing

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single chunk mesh generation | Wall-clock time | < 2 ms | NFR-13.27.1 |

### TC-NFR-13.27.1.B2 Incremental Re-Mesh

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single block change; re-mesh affected chunk | Wall-clock time | < 1 ms | NFR-13.27.1 |

### TC-NFR-13.27.1.B3 Light Propagation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single block light update | Wall-clock time | < 0.5 ms | NFR-13.27.1 |

### TC-NFR-13.27.1.B4 Block Placement Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Input to visual update | Latency | < 16 ms | NFR-13.27.1 |

### TC-NFR-13.15.1.B1 Companion AI

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 8 companions per frame | Wall-clock time | < 4 ms | NFR-13.15.1 |

### TC-NFR-13.15.2.B1 Mount Locomotion Swap

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Locomotion mode transition | Latency | < 100 ms | NFR-13.15.2 |
