# Procedural Generation Test Cases

Companion test cases for [procedural-generation.md](procedural-generation.md).

## Unit Tests

### TC-3.6.1.1 Graph Validate Cycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph with node A -> B -> A (cycle) | Error::CycleDetected | R-3.6.1 |
| 2 | Graph with A -> B -> C (no cycle) | Validation passes | R-3.6.1 |

### TC-3.6.1.2 Graph Validate Type Mismatch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Connect PointSet output to Scalar input | Error::TypeMismatch | R-3.6.1 |
| 2 | Connect PointSet output to PointSet input | Connection accepted | R-3.6.1 |

### TC-3.6.8.1 Graph Subgraph IO

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Subgraph with 1 PointSet input and 1 PointSet output | Output identical to equivalent inlined nodes | R-3.6.8 |

### TC-3.6.2.1 Poisson Disk Min Distance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 500K points, flat surface, min_distance=2.0m | All pairwise distances >= 2.0m | R-3.6.2 |

### TC-3.6.3.1 Point Filter Height Range

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000 points at y=[0..100]; filter range=[20, 60] | All surviving points have y in [20, 60] | R-3.6.3 |

### TC-3.6.7.1 Point Set Union

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set A (100 points) union Set B (200 points), disjoint | Result count = 300 | R-3.6.7 |

### TC-3.6.7.2 Point Set Difference

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set A (500 points); Set B covers 100 of A's points | Difference has 400 points; no point within B's bounds | R-3.6.7 |

### TC-3.6.5.1 Seed Determinism

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same seed, same graph, evaluate twice | Bit-identical output both times | R-3.6.5 |
| 2 | Different seeds, same graph | Different output | R-3.6.5 |

### TC-3.6.5.2 Seed Cross-Thread

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same seed, 1 thread vs 4 threads | Identical output regardless of thread count | R-3.6.5 |

### TC-3.6.6.1 Attribute Insert and Get

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert "biome_id" u32 column with values [1, 2, 3] | get("biome_id") returns [1, 2, 3] | R-3.6.6 |

### TC-3.6.6.2 Attribute Partition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 points with biome_id in {1, 2}; partition by biome_id | Two subsets; counts sum to 100 | R-3.6.6 |

### TC-3.6.33.1 Noise Perlin Range

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10K random sample coordinates, Perlin noise | All outputs in [-1.0, 1.0] | R-3.6.33 |

### TC-3.6.33.2 Noise Simplex Range

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10K random sample coordinates, Simplex noise | All outputs in [-1.0, 1.0] | R-3.6.33 |

### TC-3.6.33.3 Noise Worley Positive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10K random sample coordinates, Worley noise | All outputs >= 0.0 | R-3.6.33 |

### TC-3.6.33.4 Noise CPU GPU Identical

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1K sample points, CPU noise vs GPU noise | Bit-identical results | R-3.6.33 |

### TC-3.6.33.5 Noise Deterministic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same seed + same coords, evaluate twice | Identical noise value both times | R-3.6.33 |

### TC-3.6.20.1 WFC 2D All Collapsed

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 20x20 grid, tile set with 5 tile types | All 400 cells collapsed; all adjacency constraints satisfied | R-3.6.20 |

### TC-3.6.20.2 WFC 2D Deterministic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same seed, same tileset, 20x20 grid, evaluate twice | Identical WFC output both times | R-3.6.20 |

### TC-3.6.22.1 WFC 2D Pin Cell

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pin cell (5,5) to tile_id=3; solve 20x20 grid | Cell (5,5) = tile_id=3 after solving | R-3.6.22 |

### TC-3.6.21.1 WFC 3D Boundary Share

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two adjacent 8x8x8 chunks | Shared boundary constraints satisfied; no contradiction | R-3.6.21 |

### TC-3.6.18.1 Shape Grammar Floor Count

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Grammar with floor_count=5 | Exactly 5 horizontal divisions generated | R-3.6.18 |
| 2 | Grammar with floor_count=1 | Exactly 1 horizontal division | R-3.6.18 |

### TC-3.6.23.1 Socket Type Mismatch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Connect "door_wide" socket to "window_small" socket | Error::SocketTypeMismatch | R-3.6.23 |
| 2 | Connect "door_wide" socket to "door_wide" socket | Connection accepted | R-3.6.23 |

### TC-3.6.23.2 Socket Transform Resolve

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Piece A at origin, socket at (5,0,0); connect piece B | Piece B world-space transform starts at (5,0,0) | R-3.6.23 |

### TC-3.6.30.1 CSP Min Distance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 buildings, min_distance=50m constraint | All pairwise distances >= 50m | R-3.6.30 |

### TC-3.6.30.2 CSP Unsatisfiable

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 buildings in 10x10m area with min_distance=50m | Error::Unsatisfiable (no infinite loop) | R-3.6.30 |

### TC-3.6.9.1 Stamp Non-Destructive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply stamps A, B, C; remove stamp B | Result equals A+C (B cleanly removed) | R-3.6.9 |

### TC-3.6.9.2 Stamp Reorder

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply stamps A, B; reorder to B, A | Different but valid result (order matters) | R-3.6.9 |

### TC-3.6.11.1 Biome Whittaker

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Equatorial lowland (temp=high, precip=high) | Tropical biome assigned | R-3.6.11 |
| 2 | Polar highland (temp=low, precip=low) | Tundra biome assigned | R-3.6.11 |

### TC-3.6.16.1 Spline SDF Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 50 random sample points near spline | SDF texture values match brute-force polynomial eval within 0.5m | R-3.6.16 |

### TC-3.6.12.1 Vegetation Slope Filter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Rule: slope < 30 deg; terrain with 35-degree face | Zero vegetation instances on 35-degree face | R-3.6.12 |
| 2 | Rule: slope < 30 deg; terrain with 20-degree face | Vegetation instances present on 20-degree face | R-3.6.12 |

### TC-3.6.63.1 Cosmic Key Precision

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 128-bit key at 10 billion light-year distance | Sub-meter positional precision | R-3.6.63 |

### TC-3.6.63.2 Cosmic Octree Sparse

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Empty octree region (no objects) | Memory usage < 1 KB | R-3.6.63 |

### TC-3.6.49.1 Star Spectral Distribution

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Generate 100 stars | Spectral class distribution follows initial mass function | R-3.6.49 |

### TC-3.6.54.1 Planet Classification

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Inner planet (inside frost line) | Classified as rocky/terrestrial | R-3.6.54 |
| 2 | Outer planet (beyond frost line) | Classified as gas giant | R-3.6.54 |

## Integration Tests

### TC-3.6.1.I1 Graph End-to-End

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph: point gen (N=500) -> height filter -> spawn | Entity count matches filter output count | R-3.6.1 |

### TC-3.6.1.I2 Graph Editor Runtime Parity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same graph in editor and runtime context | Identical output from both contexts | R-3.6.1 |

### TC-3.6.31.1 Chunk Generate and Revisit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Generate chunk at (0,0); evict; revisit | Bit-identical output on revisit | R-3.6.31 |

### TC-3.6.31.2 Chunk Activation Radius

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move camera through 20 chunk boundaries | All chunks generate before becoming visible | R-3.6.31 |

### TC-3.6.31.3 Chunk Memory Eviction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Generate 100 chunks with budget for 50 | Total memory stays within budget; 50 evicted | R-3.6.31 |

### TC-3.6.32.I1 GPU Heightmap vs CPU

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | GPU-generated 512x512 heightmap vs CPU reference | Values match within float tolerance (1e-5) | R-3.6.32 |

### TC-3.6.32.I2 GPU Single Frame

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 512x512 heightmap GPU generation | Completes within one frame (< 16.6 ms) | R-3.6.32 |

### TC-3.6.12.I1 Vegetation 100K Instances

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100K+ vegetation instances rendered | Frame rate stays above target threshold | R-3.6.12 |

### TC-3.6.15.I1 Road Network Connected

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 population centers | All connected via primary road segments | R-3.6.15 |

### TC-3.6.15.I2 Road Slope Limit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Generated road network, max_slope=15 deg | No road segment exceeds 15 degrees | R-3.6.15 |

### TC-3.6.17.I1 Intersection Geometry

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two crossing splines | Crossroads junction generated with continuous terrain | R-3.6.17 |

### TC-3.6.19.I1 Modular Building 3-Story

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3-story L-shaped building spec | All sockets match; no overlapping geometry | R-3.6.19 |

### TC-3.6.26.I1 Hierarchical Composition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Walls -> room -> floor -> building hierarchy | All transforms correct at each level | R-3.6.26 |

### TC-3.6.35.I1 Settlement Scaling

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Population 10K settlement | Road network, building lots, distinct zoning present | R-3.6.35 |

### TC-3.6.36.I1 Faction Territories

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5 factions generated | Each controls >= 1 settlement; buffer zones between rivals | R-3.6.36 |

### TC-3.6.38.I1 Ecosystem Lotka-Volterra

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 prey + 20 predators, 100 simulation ticks | Populations oscillate (predator-prey dynamics) | R-3.6.38 |

### TC-3.6.39.I1 History Ruins

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 historical epochs simulated | At least one fallen faction with ruined settlements | R-3.6.39 |

### TC-3.6.44.I1 Planet Biome Diversity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Full planet generation | >= 16 distinct biome types present | R-3.6.44 |

### TC-3.6.45.I1 Planet River Downhill

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Generated planet rivers | All river segments flow strictly downhill | R-3.6.45 |

### TC-3.6.46.I1 Planet Landform Count

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Generated planet surface | >= 30 distinct landform types in correct geological contexts | R-3.6.46 |

### TC-3.6.47.I1 GIS Import Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100km x 100km Earth data import | RMS elevation error < 5m | R-3.6.47 |

### TC-3.6.48.I1 Planet Presets Distinct

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | All 7 planet presets generated | Each produces visually distinct terrain/biome distribution | R-3.6.48 |

### TC-3.6.50.I1 Accretion Frost Line

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 50 generated star systems | Gas giants form beyond frost line in all systems | R-3.6.50 |

### TC-3.6.60.I1 Universe Cosmic to Surface

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Zoom from cosmic web to planet surface | Consistent LOD at each tier; no gaps or artifacts | R-3.6.60 |

### TC-3.6.60.I2 Universe Deterministic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same seed, regenerate universe | Identical output | R-3.6.60 |

### TC-3.6.64.I1 Universe Memory Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Explore 100 star systems | Memory stays within configured budget | R-3.6.64 |

### TC-3.6.64.I2 LOD Resolve Time

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Each LOD tier (cosmic to surface) | Resolves within 5 seconds per tier | R-3.6.64 |

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
