# Genre-Specific Systems Test Cases

Companion test cases for [genre-specific.md](genre-specific.md).

## Unit Tests

### TC-13.20.1.1 Fog Three States

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `set(5,5,faction_0,Visible); get(5,5,faction_0)` | `FogState::Visible` | R-13.20.1 |
| 2 | `get(0,0,faction_0)` (never set) | `FogState::Unexplored` | R-13.20.1 |

### TC-13.20.1.2 Fog 2-Bit Encoding

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 256x256 grid; serialize_full for 1 faction | Payload = 16,384 bytes | R-13.20.1 |

### TC-NFR-13.20.2.1 Fog Delta 90 Percent

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 256x256 grid; 5% cell change; compute delta | Delta >= 90% smaller than full payload | NFR-13.20.2 |

### TC-13.20.2.1 Vision Circle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | VisionSource radius=10 at (50,50) | Cells within 10 tiles set to Visible | R-13.20.2 |

### TC-13.20.2.2 Vision LOS Blocked

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Wall at (55,50); source at (50,50) radius=10 | Cells behind wall remain Unexplored | R-13.20.2 |

### TC-13.20.2.3 Vision Elevation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Base radius=10, elevation_bonus=1, elevation=3 | Effective radius = 13 | R-13.20.2 |

### TC-13.20.3.1 Vision Modifier Stealth

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Entity inside StealthZone volume | Invisible to outside vision sources | R-13.20.3 |

### TC-13.20.4.1 Fog Memory Persist

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Building seen, then shrouded | Ghost building persists in FogMemory | R-13.20.4 |

### TC-13.21.1.1 Grid Reachable Cells

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Origin (5,5), max_ap=3, flat terrain | BFS returns all cells within 3 steps | R-13.21.1 |

### TC-13.21.1.2 Grid Hex Adjacency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hex grid; query neighbors of (3,3) | Correct 6 neighbors for odd row | R-13.21.1 |

### TC-13.21.2.1 Initiative Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Speeds: 10, 20, 15, 5 | Turn order: 20, 15, 10, 5 | R-13.21.2 |

### TC-13.21.2.2 Team Turn Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Team A (3 units), Team B (2 units); TeamBased mode | All of Team A acts, then all of Team B | R-13.21.2 |

### TC-13.21.3.1 AP Movement

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 flat cells (cost=1 each) | Total AP cost = 3 | R-13.21.3 |
| 2 | 3 difficult terrain cells (cost=2 each) | Total AP cost = 6 | R-13.21.3 |

### TC-13.21.3.2 AP Interleave

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move (2 AP), attack (3 AP), move (1 AP); budget=6 | All actions succeed; total AP = 6 | R-13.21.3 |

### TC-13.21.4.1 Cover Directional

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Half cover from north edge; attack from north | CoverValue::Half applied | R-13.21.4 |
| 2 | Same cover; attack from east (flanking) | CoverValue::None applied | R-13.21.4 |

### TC-13.21.4.2 Overwatch Triggers

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enemy moves through overwatch arc | Overwatch shot triggered; triggered=true | R-13.21.4 |

### TC-13.21.5.1 Hit Probability

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 80% base - 10% range - 20% cover | Hit chance = 50% | R-13.21.5 |

### TC-13.21.5.2 Combat Outcomes

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000 rolls at 50% hit chance | Hit count within 95% CI [460, 540] | R-13.21.5 |

### TC-13.26.1.1 Minigame Isolation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Access outer world component from minigame partition | Access fails; isolated | R-13.26.1 |

### TC-13.26.3.1 Minigame Contract

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Complete minigame; apply results | All outputs applied atomically | R-13.26.3 |

### TC-13.26.3.2 Minigame Quit Policy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Quit with QuitPolicy::Refund | Entry cost refunded | R-13.26.3 |
| 2 | Quit with QuitPolicy::Loss | Loss outcome applied | R-13.26.3 |

### TC-13.26.4.1 Timing Windows

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Input at beat +25ms (perfect_ms=30) | Grade = Perfect | R-13.26.4 |
| 2 | Input at beat +50ms (great_ms=60) | Grade = Great | R-13.26.4 |

### TC-13.26.5b.1 Match 3 Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 same pieces in horizontal row | `MatchPattern::ThreeInARow` detected | R-13.26.5b |
| 2 | 3 same pieces in vertical column | `MatchPattern::ThreeInARow` detected | R-13.26.5b |

### TC-13.26.5d.1 Cascade Recursive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Clear match causing gravity fill with new match | Recursive cascade triggered; second match detected | R-13.26.5d |

### TC-13.26.5c.1 Board AI Difficulty

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Easy AI vs random baseline (100 games) | Easy AI win rate < 30% | R-13.26.5c |
| 2 | Hard AI vs random baseline (100 games) | Hard AI win rate > 70% | R-13.26.5c |

### TC-13.26.6.1 Physics Determinism

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Identical physics inputs on 2 runs | Identical output states | R-13.26.6 |

### TC-13.22.1.1 Checkpoint Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Skip checkpoint 2; cross start/finish | Lap NOT incremented | R-13.22.1 |

### TC-13.22.1.2 Lap Counting

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pass all checkpoints; cross start/finish | current_lap increments by 1 | R-13.22.1 |

### TC-13.22.1.3 Split Times

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Current time=45s, best=40s at checkpoint 3 | Split time = +5s | R-13.22.1 |

### TC-13.22.2.1 Race Modes

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Circuit (3 laps); complete 3 laps | Race ends; finish_position assigned | R-13.22.2 |
| 2 | TimeTrial; cross finish | Total time recorded | R-13.22.2 |

### TC-13.22.3b.1 Rubber Banding

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | AI trailing by max_gap_sec | AI speed multiplied by trailing_boost | R-13.22.3b |
| 2 | AI leading by max_gap_sec | AI speed multiplied by leading_penalty | R-13.22.3b |

### TC-13.22.4.1 Drift Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Slip angle = threshold + 5 deg | `DriftState.drifting` = true | R-13.22.4 |
| 2 | Slip angle = threshold - 5 deg | `DriftState.drifting` = false | R-13.22.4 |

### TC-13.22.4.2 Drift Combo

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Drift for 3 seconds continuously | combo_multiplier increases | R-13.22.4 |

### TC-13.22.4.3 Boost Fill

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Drift score accumulated during drift | Boost meter fills proportionally to drift score | R-13.22.4 |

### TC-NFR-13.22.2.1 Ghost Storage 10KB

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Record 1 minute ghost; compress | Compressed size <= 10 KB | NFR-13.22.2 |

### TC-13.22.5.1 Ghost Record Replay

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Record 1 lap; replay ghost | Ghost positions match recorded positions | R-13.22.5 |

### TC-13.27.1.1 Block Registry O(1)

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register 1024 types; lookup by id | Correct BlockTypeDef returned in O(1) | R-13.27.1 |

### TC-13.27.2.1 Block Placement

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Raycast targets face of existing block | New block placed on correct face | R-13.27.2 |

### TC-13.27.3.1 Palette Compression

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Chunk with 8 unique types | 4-bit palette; storage = 2048 bytes | R-13.27.3 |

### TC-13.27.3.2 Uniform Chunk

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | All-air chunk | Stored as single value (minimal bytes) | R-13.27.3 |

### TC-13.27.4.1 Greedy Meshing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 16x16 flat surface of same block | Single quad per face direction | R-13.27.4 |

### TC-13.27.4.2 Interior Culling

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Block fully enclosed by other blocks | Zero mesh faces generated | R-13.27.4 |

### TC-13.27.5.1 Light Propagation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Torch (emission=15) placed in dark area | Light attenuates to 0 at 15 blocks | R-13.27.5 |

### TC-13.27.5.2 Sunlight Flood

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sunlight column above open terrain | Sunlight propagates down at full intensity (level 15) | R-13.27.5 |

### TC-13.27.6a.1 Gravity Block

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sand block with no support below | Falls to nearest solid surface | R-13.27.6a |

### TC-13.27.6b.1 Fluid Flow Levels

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Water source block | Flows with 7 decreasing levels | R-13.27.6b |

### TC-13.27.6c.1 Fluid Interaction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Lava meets water | Cobblestone block generated | R-13.27.6c |

### TC-13.27.7a.1 Signal Attenuation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Signal source; measure at 15 and 16 blocks | Strength > 0 at 15; strength = 0 at 16 | R-13.27.7a |

### TC-13.27.7b.1 Not Gate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Torch on powered block | Output = inverted signal | R-13.27.7b |

### TC-13.27.7c.1 Piston Push

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Power piston with adjacent block | Piston pushes adjacent block one cell | R-13.27.7c |

### TC-13.27.7d.1 Circuit Determinism

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same circuit on 2 clients | Identical output state | R-13.27.7d |

### TC-13.27.7d.2 Circuit Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Exceed redstone evaluation budget | Warning emitted; excess circuits depowered | R-13.27.7d |

### TC-13.27.8a.1 Terrain Seed Determinism

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same seed on 2 platforms | Identical terrain output | R-13.27.8a |

### TC-13.27.8b.1 Biome Composition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Generate plains biome chunk | Top layers = grass/dirt | R-13.27.8b |
| 2 | Generate desert biome chunk | Top layers = sand | R-13.27.8b |

### TC-13.15.1.1 Companion Follow

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Player moves 5m; companion follows | Companion maintains follow distance | R-13.15.1 |

### TC-13.15.1.2 Companion Teleport

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Player moves beyond teleport threshold | Companion teleports to player vicinity | R-13.15.1 |

### TC-13.15.2.1 Pet Needs Decay

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pet needs drain to zero | Pet refuses commands | R-13.15.2 |

### TC-13.15.3b.1 Mount Locomotion

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mount summoned; player rides | Player physics replaced by mount physics | R-13.15.3b |

### TC-13.15.3c.1 Mounted Combat

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Attempt ground-only ability while mounted | Ability blocked; only mounted abilities available | R-13.15.3c |

### TC-13.15.3d.1 Flying Mount Altitude

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Flying mount at altitude limit | Movement clamped; cannot exceed ceiling | R-13.15.3d |

### TC-13.15.4.1 Taming Progress

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Feed creature 3 times | Taming progress advances 3 increments | R-13.15.4 |

### TC-13.15.4.2 Taming Probability

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Player level 5 vs creature level 10 | Success rate reduced by level deficit | R-13.15.4 |

### TC-13.15.5a.1 Life Stages

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Advance time through growth thresholds | Baby -> juvenile -> adult with stat changes | R-13.15.5a |

### TC-13.15.5b.1 Evolution Branch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Feed meat diet during juvenile stage | Evolves to combat wolf specialization | R-13.15.5b |

### TC-13.15.5c.1 Breeding Inheritance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Breed parent A (speed trait) + parent B (strength trait) | Offspring inherits subset of parent traits | R-13.15.5c |

## Integration Tests

### TC-NFR-13.20.1.I1 Fog GPU Texture

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 512x512 grid, 128 vision sources | GPU fog texture gen < 0.5 ms | NFR-13.20.1 |

### TC-NFR-13.20.1.I2 Fog CPU Fallback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 512x512 grid, 128 vision sources; CPU path | CPU fog compute < 2 ms | NFR-13.20.1 |

### TC-NFR-13.21.1.I1 Grid Pathfind Perf

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100x100 grid pathfinding | Completes in < 2 ms | NFR-13.21.1 |

### TC-NFR-13.21.2.I1 Turn Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | End turn; camera focus + UI update | Transition completes in < 500 ms | NFR-13.21.2 |

### TC-NFR-13.26.1.I1 Minigame Create Teardown

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create minigame session; teardown | Create < 500 ms; teardown < 200 ms; zero leak | NFR-13.26.1 |

### TC-NFR-13.26.1.I2 Minigame 100 Cycles

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 create/destroy cycles | Zero net memory allocation | NFR-13.26.1 |

### TC-NFR-13.26.2.I1 Rhythm Input Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Audio beat event to detection pipeline | Audio-to-detection latency < 16 ms | NFR-13.26.2 |

### TC-NFR-13.22.1.I1 Racing Tick Precision

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 identical laps | Lap time variance < 1 ms | NFR-13.22.1 |

### TC-NFR-13.27.1.I1 Chunk Mesh Perf

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mesh 1000 chunks | Average < 2 ms per chunk | NFR-13.27.1 |

### TC-NFR-13.27.1.I2 32k Chunks 60fps

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 32,768 loaded chunks | Sustained 60 fps | NFR-13.27.1 |

### TC-NFR-13.27.1.I3 Block Modify Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Place block; measure input to visual update | Latency < 16 ms | NFR-13.27.1 |

### TC-NFR-13.15.1.I1 Companion 8 Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 8 companions with full AI | Total AI < 4 ms/frame | NFR-13.15.1 |

### TC-NFR-13.15.2.I1 Mount Swap Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Locomotion mode transition | Completes in < 100 ms | NFR-13.15.2 |

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
