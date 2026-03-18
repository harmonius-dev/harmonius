# Procedural Generation Test Cases

Companion test cases for [procedural-generation.md](procedural-generation.md).

## Unit Tests

### TC-3.6.1.1 Graph Validate Cycle

| # | Requirement |
|---|-------------|
| 1 | R-3.6.1     |
| 2 | R-3.6.1     |

1. **#1** — Graph with node A -> B -> A (cycle)
   - **Expected:** Error::CycleDetected
2. **#2** — Graph with A -> B -> C (no cycle)
   - **Expected:** Validation passes

### TC-3.6.1.2 Graph Validate Type Mismatch

| # | Requirement |
|---|-------------|
| 1 | R-3.6.1     |
| 2 | R-3.6.1     |

1. **#1** — Connect PointSet output to Scalar input
   - **Expected:** Error::TypeMismatch
2. **#2** — Connect PointSet output to PointSet input
   - **Expected:** Connection accepted

### TC-3.6.8.1 Graph Subgraph IO

| # | Requirement |
|---|-------------|
| 1 | R-3.6.8     |

1. **#1** — Subgraph with 1 PointSet input and 1 PointSet output
   - **Expected:** Output identical to equivalent inlined nodes

### TC-3.6.2.1 Poisson Disk Min Distance

| # | Requirement |
|---|-------------|
| 1 | R-3.6.2     |

1. **#1** — 500K points, flat surface, min_distance=2.0m
   - **Expected:** All pairwise distances >= 2.0m

### TC-3.6.3.1 Point Filter Height Range

| # | Requirement |
|---|-------------|
| 1 | R-3.6.3     |

1. **#1** — 1000 points at y=[0..100]; filter range=[20, 60]
   - **Expected:** All surviving points have y in [20, 60]

### TC-3.6.7.1 Point Set Union

| # | Requirement |
|---|-------------|
| 1 | R-3.6.7     |

1. **#1** — Set A (100 points) union Set B (200 points), disjoint
   - **Expected:** Result count = 300

### TC-3.6.7.2 Point Set Difference

| # | Requirement |
|---|-------------|
| 1 | R-3.6.7     |

1. **#1** — Set A (500 points); Set B covers 100 of A's points
   - **Expected:** Difference has 400 points; no point within B's bounds

### TC-3.6.5.1 Seed Determinism

| # | Requirement |
|---|-------------|
| 1 | R-3.6.5     |
| 2 | R-3.6.5     |

1. **#1** — Same seed, same graph, evaluate twice
   - **Expected:** Bit-identical output both times
2. **#2** — Different seeds, same graph
   - **Expected:** Different output

### TC-3.6.5.2 Seed Cross-Thread

| # | Requirement |
|---|-------------|
| 1 | R-3.6.5     |

1. **#1** — Same seed, 1 thread vs 4 threads
   - **Expected:** Identical output regardless of thread count

### TC-3.6.6.1 Attribute Insert and Get

| # | Requirement |
|---|-------------|
| 1 | R-3.6.6     |

1. **#1** — Insert "biome_id" u32 column with values [1, 2, 3]
   - **Expected:** get("biome_id") returns [1, 2, 3]

### TC-3.6.6.2 Attribute Partition

| # | Requirement |
|---|-------------|
| 1 | R-3.6.6     |

1. **#1** — 100 points with biome_id in {1, 2}; partition by biome_id
   - **Expected:** Two subsets; counts sum to 100

### TC-3.6.33.1 Noise Perlin Range

| # | Requirement |
|---|-------------|
| 1 | R-3.6.33    |

1. **#1** — 10K random sample coordinates, Perlin noise
   - **Expected:** All outputs in [-1.0, 1.0]

### TC-3.6.33.2 Noise Simplex Range

| # | Requirement |
|---|-------------|
| 1 | R-3.6.33    |

1. **#1** — 10K random sample coordinates, Simplex noise
   - **Expected:** All outputs in [-1.0, 1.0]

### TC-3.6.33.3 Noise Worley Positive

| # | Requirement |
|---|-------------|
| 1 | R-3.6.33    |

1. **#1** — 10K random sample coordinates, Worley noise
   - **Expected:** All outputs >= 0.0

### TC-3.6.33.4 Noise CPU GPU Identical

| # | Requirement |
|---|-------------|
| 1 | R-3.6.33    |

1. **#1** — 1K sample points, CPU noise vs GPU noise
   - **Expected:** Bit-identical results

### TC-3.6.33.5 Noise Deterministic

| # | Requirement |
|---|-------------|
| 1 | R-3.6.33    |

1. **#1** — Same seed + same coords, evaluate twice
   - **Expected:** Identical noise value both times

### TC-3.6.20.1 WFC 2D All Collapsed

| # | Requirement |
|---|-------------|
| 1 | R-3.6.20    |

1. **#1** — 20x20 grid, tile set with 5 tile types
   - **Expected:** All 400 cells collapsed; all adjacency constraints satisfied

### TC-3.6.20.2 WFC 2D Deterministic

| # | Requirement |
|---|-------------|
| 1 | R-3.6.20    |

1. **#1** — Same seed, same tileset, 20x20 grid, evaluate twice
   - **Expected:** Identical WFC output both times

### TC-3.6.22.1 WFC 2D Pin Cell

| # | Requirement |
|---|-------------|
| 1 | R-3.6.22    |

1. **#1** — Pin cell (5,5) to tile_id=3; solve 20x20 grid
   - **Expected:** Cell (5,5) = tile_id=3 after solving

### TC-3.6.21.1 WFC 3D Boundary Share

| # | Requirement |
|---|-------------|
| 1 | R-3.6.21    |

1. **#1** — Two adjacent 8x8x8 chunks
   - **Expected:** Shared boundary constraints satisfied; no contradiction

### TC-3.6.18.1 Shape Grammar Floor Count

| # | Requirement |
|---|-------------|
| 1 | R-3.6.18    |
| 2 | R-3.6.18    |

1. **#1** — Grammar with floor_count=5
   - **Expected:** Exactly 5 horizontal divisions generated
2. **#2** — Grammar with floor_count=1
   - **Expected:** Exactly 1 horizontal division

### TC-3.6.23.1 Socket Type Mismatch

| # | Requirement |
|---|-------------|
| 1 | R-3.6.23    |
| 2 | R-3.6.23    |

1. **#1** — Connect "door_wide" socket to "window_small" socket
   - **Expected:** Error::SocketTypeMismatch
2. **#2** — Connect "door_wide" socket to "door_wide" socket
   - **Expected:** Connection accepted

### TC-3.6.23.2 Socket Transform Resolve

| # | Requirement |
|---|-------------|
| 1 | R-3.6.23    |

1. **#1** — Piece A at origin, socket at (5,0,0); connect piece B
   - **Expected:** Piece B world-space transform starts at (5,0,0)

### TC-3.6.30.1 CSP Min Distance

| # | Requirement |
|---|-------------|
| 1 | R-3.6.30    |

1. **#1** — 10 buildings, min_distance=50m constraint
   - **Expected:** All pairwise distances >= 50m

### TC-3.6.30.2 CSP Unsatisfiable

| # | Requirement |
|---|-------------|
| 1 | R-3.6.30    |

1. **#1** — 100 buildings in 10x10m area with min_distance=50m
   - **Expected:** Error::Unsatisfiable (no infinite loop)

### TC-3.6.9.1 Stamp Non-Destructive

| # | Requirement |
|---|-------------|
| 1 | R-3.6.9     |

1. **#1** — Apply stamps A, B, C; remove stamp B
   - **Expected:** Result equals A+C (B cleanly removed)

### TC-3.6.9.2 Stamp Reorder

| # | Requirement |
|---|-------------|
| 1 | R-3.6.9     |

1. **#1** — Apply stamps A, B; reorder to B, A
   - **Expected:** Different but valid result (order matters)

### TC-3.6.11.1 Biome Whittaker

| # | Requirement |
|---|-------------|
| 1 | R-3.6.11    |
| 2 | R-3.6.11    |

1. **#1** — Equatorial lowland (temp=high, precip=high)
   - **Expected:** Tropical biome assigned
2. **#2** — Polar highland (temp=low, precip=low)
   - **Expected:** Tundra biome assigned

### TC-3.6.16.1 Spline SDF Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-3.6.16    |

1. **#1** — 50 random sample points near spline
   - **Expected:** SDF texture values match brute-force polynomial eval within 0.5m

### TC-3.6.12.1 Vegetation Slope Filter

| # | Requirement |
|---|-------------|
| 1 | R-3.6.12    |
| 2 | R-3.6.12    |

1. **#1** — Rule: slope < 30 deg; terrain with 35-degree face
   - **Expected:** Zero vegetation instances on 35-degree face
2. **#2** — Rule: slope < 30 deg; terrain with 20-degree face
   - **Expected:** Vegetation instances present on 20-degree face

### TC-3.6.63.1 Cosmic Key Precision

| # | Requirement |
|---|-------------|
| 1 | R-3.6.63    |

1. **#1** — 128-bit key at 10 billion light-year distance
   - **Expected:** Sub-meter positional precision

### TC-3.6.63.2 Cosmic Octree Sparse

| # | Requirement |
|---|-------------|
| 1 | R-3.6.63    |

1. **#1** — Empty octree region (no objects)
   - **Expected:** Memory usage < 1 KB

### TC-3.6.49.1 Star Spectral Distribution

| # | Requirement |
|---|-------------|
| 1 | R-3.6.49    |

1. **#1** — Generate 100 stars
   - **Expected:** Spectral class distribution follows initial mass function

### TC-3.6.54.1 Planet Classification

| # | Requirement |
|---|-------------|
| 1 | R-3.6.54    |
| 2 | R-3.6.54    |

1. **#1** — Inner planet (inside frost line)
   - **Expected:** Classified as rocky/terrestrial
2. **#2** — Outer planet (beyond frost line)
   - **Expected:** Classified as gas giant

## Integration Tests

### TC-3.6.1.I1 Graph End-to-End

| # | Requirement |
|---|-------------|
| 1 | R-3.6.1     |

1. **#1** — Graph: point gen (N=500) -> height filter -> spawn
   - **Expected:** Entity count matches filter output count

### TC-3.6.1.I2 Graph Editor Runtime Parity

| # | Requirement |
|---|-------------|
| 1 | R-3.6.1     |

1. **#1** — Same graph in editor and runtime context
   - **Expected:** Identical output from both contexts

### TC-3.6.31.1 Chunk Generate and Revisit

| # | Requirement |
|---|-------------|
| 1 | R-3.6.31    |

1. **#1** — Generate chunk at (0,0); evict; revisit
   - **Expected:** Bit-identical output on revisit

### TC-3.6.31.2 Chunk Activation Radius

| # | Requirement |
|---|-------------|
| 1 | R-3.6.31    |

1. **#1** — Move camera through 20 chunk boundaries
   - **Expected:** All chunks generate before becoming visible

### TC-3.6.31.3 Chunk Memory Eviction

| # | Requirement |
|---|-------------|
| 1 | R-3.6.31    |

1. **#1** — Generate 100 chunks with budget for 50
   - **Expected:** Total memory stays within budget; 50 evicted

### TC-3.6.32.I1 GPU Heightmap vs CPU

| # | Requirement |
|---|-------------|
| 1 | R-3.6.32    |

1. **#1** — GPU-generated 512x512 heightmap vs CPU reference
   - **Expected:** Values match within float tolerance (1e-5)

### TC-3.6.32.I2 GPU Single Frame

| # | Requirement |
|---|-------------|
| 1 | R-3.6.32    |

1. **#1** — 512x512 heightmap GPU generation
   - **Expected:** Completes within one frame (< 16.6 ms)

### TC-3.6.12.I1 Vegetation 100K Instances

| # | Requirement |
|---|-------------|
| 1 | R-3.6.12    |

1. **#1** — 100K+ vegetation instances rendered
   - **Expected:** Frame rate stays above target threshold

### TC-3.6.15.I1 Road Network Connected

| # | Requirement |
|---|-------------|
| 1 | R-3.6.15    |

1. **#1** — 3 population centers
   - **Expected:** All connected via primary road segments

### TC-3.6.15.I2 Road Slope Limit

| # | Requirement |
|---|-------------|
| 1 | R-3.6.15    |

1. **#1** — Generated road network, max_slope=15 deg
   - **Expected:** No road segment exceeds 15 degrees

### TC-3.6.17.I1 Intersection Geometry

| # | Requirement |
|---|-------------|
| 1 | R-3.6.17    |

1. **#1** — Two crossing splines
   - **Expected:** Crossroads junction generated with continuous terrain

### TC-3.6.19.I1 Modular Building 3-Story

| # | Requirement |
|---|-------------|
| 1 | R-3.6.19    |

1. **#1** — 3-story L-shaped building spec
   - **Expected:** All sockets match; no overlapping geometry

### TC-3.6.26.I1 Hierarchical Composition

| # | Requirement |
|---|-------------|
| 1 | R-3.6.26    |

1. **#1** — Walls -> room -> floor -> building hierarchy
   - **Expected:** All transforms correct at each level

### TC-3.6.35.I1 Settlement Scaling

| # | Requirement |
|---|-------------|
| 1 | R-3.6.35    |

1. **#1** — Population 10K settlement
   - **Expected:** Road network, building lots, distinct zoning present

### TC-3.6.36.I1 Faction Territories

| # | Requirement |
|---|-------------|
| 1 | R-3.6.36    |

1. **#1** — 5 factions generated
   - **Expected:** Each controls >= 1 settlement; buffer zones between rivals

### TC-3.6.38.I1 Ecosystem Lotka-Volterra

| # | Requirement |
|---|-------------|
| 1 | R-3.6.38    |

1. **#1** — 100 prey + 20 predators, 100 simulation ticks
   - **Expected:** Populations oscillate (predator-prey dynamics)

### TC-3.6.39.I1 History Ruins

| # | Requirement |
|---|-------------|
| 1 | R-3.6.39    |

1. **#1** — 10 historical epochs simulated
   - **Expected:** At least one fallen faction with ruined settlements

### TC-3.6.44.I1 Planet Biome Diversity

| # | Requirement |
|---|-------------|
| 1 | R-3.6.44    |

1. **#1** — Full planet generation
   - **Expected:** >= 16 distinct biome types present

### TC-3.6.45.I1 Planet River Downhill

| # | Requirement |
|---|-------------|
| 1 | R-3.6.45    |

1. **#1** — Generated planet rivers
   - **Expected:** All river segments flow strictly downhill

### TC-3.6.46.I1 Planet Landform Count

| # | Requirement |
|---|-------------|
| 1 | R-3.6.46    |

1. **#1** — Generated planet surface
   - **Expected:** >= 30 distinct landform types in correct geological contexts

### TC-3.6.47.I1 GIS Import Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-3.6.47    |

1. **#1** — 100km x 100km Earth data import
   - **Expected:** RMS elevation error < 5m

### TC-3.6.48.I1 Planet Presets Distinct

| # | Requirement |
|---|-------------|
| 1 | R-3.6.48    |

1. **#1** — All 7 planet presets generated
   - **Expected:** Each produces visually distinct terrain/biome distribution

### TC-3.6.50.I1 Accretion Frost Line

| # | Requirement |
|---|-------------|
| 1 | R-3.6.50    |

1. **#1** — 50 generated star systems
   - **Expected:** Gas giants form beyond frost line in all systems

### TC-3.6.60.I1 Universe Cosmic to Surface

| # | Requirement |
|---|-------------|
| 1 | R-3.6.60    |

1. **#1** — Zoom from cosmic web to planet surface
   - **Expected:** Consistent LOD at each tier; no gaps or artifacts

### TC-3.6.60.I2 Universe Deterministic

| # | Requirement |
|---|-------------|
| 1 | R-3.6.60    |

1. **#1** — Same seed, regenerate universe
   - **Expected:** Identical output

### TC-3.6.64.I1 Universe Memory Budget

| # | Requirement |
|---|-------------|
| 1 | R-3.6.64    |

1. **#1** — Explore 100 star systems
   - **Expected:** Memory stays within configured budget

### TC-3.6.64.I2 LOD Resolve Time

| # | Requirement |
|---|-------------|
| 1 | R-3.6.64    |

1. **#1** — Each LOD tier (cosmic to surface)
   - **Expected:** Resolves within 5 seconds per tier

## Benchmarks

### TC-3.6.1.B1 Graph Evaluation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100-node graph, desktop | Wall time | < 5 ms | R-3.6.1 |

### TC-3.6.2.B1 Poisson Disk

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500K points, flat surface | Wall time | < 50 ms | R-3.6.2 |

### TC-3.6.33.B1 Noise Fill CPU

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 512x512 grid, CPU Perlin noise | Wall time | < 10 ms | R-3.6.33 |

### TC-3.6.32.B1 Noise Fill GPU

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 512x512 grid, GPU compute noise | GPU time | < 2 ms | R-3.6.32 |

### TC-3.6.20.B1 WFC 2D

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 128x128 grid, 2D WFC | Wall time | < 100 ms | R-3.6.20 |

### TC-3.6.21.B1 WFC 3D

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 64x64x32 volume, 3D WFC | Wall time | < 500 ms | R-3.6.21 |

### TC-3.6.31.B1 Chunk Generation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full chunk generation, desktop | Wall time | < 8 ms | R-3.6.31 |

### TC-3.6.5.B1 Seed Derivation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single xxHash seed derivation | Wall time | < 10 ns | R-3.6.5 |

### TC-3.6.12.B1 Vegetation Scatter GPU

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100K instances, GPU scatter | GPU time | < 1 ms | R-3.6.12 |

### TC-3.6.16.B1 SDF Texture

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 256x256 spline SDF texture | Wall time | < 2 ms | R-3.6.16 |

### TC-3.6.18.B1 Shape Grammar

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 5-floor building, shape grammar evaluation | Wall time | < 1 ms | R-3.6.18 |

### TC-3.6.30.B1 CSP Solver

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100 variables, constraint satisfaction | Wall time | < 50 ms | R-3.6.30 |

### TC-3.6.42.B1 Planet Tectonic Simulation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 8-plate tectonic simulation | Wall time | < 30 s | R-3.6.42 |

### TC-3.6.43.B1 Climate Simulation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full planet climate simulation | Wall time | < 60 s | R-3.6.43 |

### TC-3.6.63.B1 Galaxy Storage

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100B statistical stars, sparse storage | Storage size | < 100 GB | R-3.6.63 |
