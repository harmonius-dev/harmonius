# AI Behavior ↔ Grids/Volumes Integration Test Cases

All tests are CI-runnable. Integration tests use real `UniformGrid`, `VoxelVolume`,
`HierarchicalGrid`, and real MPSC channels -- no mocks, no stubs. Benchmarks run under `cargo bench`
and gate on performance requirements from the grids and AI designs.

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.3.1.1 | BT samples 2D influence | Agent at (10,10), val=0.8 | BB threat=0.8 | IR-2.3.1 |
| TC-IR-2.3.1.2 | BT samples 3D influence | Voxel (4,2,6), val=0.6 | BB threat=0.6 | IR-2.3.1 |
| TC-IR-2.3.1.3 | BT samples hierarchical LOD | LOD=2, val=0.4 | BB threat=0.4 | IR-2.3.1 |
| TC-IR-2.3.2.1 | 2D flow reads direction | Cell Vec2(1,0) | Move east | IR-2.3.2 |
| TC-IR-2.3.2.2 | 3D flow reads direction | Voxel Vec3(0,1,0) | Move up | IR-2.3.2 |
| TC-IR-2.3.3.1 | BT checks visible cell | FogState::Visible | Condition = true | IR-2.3.3 |
| TC-IR-2.3.3.2 | BT checks unexplored | FogState::Unexplored | Condition = false | IR-2.3.3 |
| TC-IR-2.3.4.1 | Utility scores from grid | Cell val=0.5, linear | Score = 0.5 | IR-2.3.4 |
| TC-IR-2.3.4.2 | Utility scores 3D cell | Voxel val=0.3, linear | Score = 0.3 | IR-2.3.4 |
| TC-IR-2.3.5.1 | GOAP reads safe zone | Influence > 0.7 | in_safe_zone=true | IR-2.3.5 |
| TC-IR-2.3.5.2 | GOAP reads danger zone | Influence < 0.3 | in_safe_zone=false | IR-2.3.5 |
| TC-IR-2.3.6.1 | AI enqueues additive write | Combat at (5,5), +0.5 | Msg in channel | IR-2.3.6 |
| TC-IR-2.3.6.2 | Drain applies additive | 3 writes of 0.1 | Cell += 0.3 | IR-2.3.6 |
| TC-IR-2.3.6.3 | Drain applies Overwrite | seq=3, 2, 1; vals a,b,c | Cell = c (seq=1) | IR-2.3.6 |
| TC-IR-2.3.6.4 | Drain applies Max | writes 0.1, 0.8, 0.4 | Cell = 0.8 | IR-2.3.6 |
| TC-IR-2.3.6.5 | Mixed-mode order | Add 0.2; Max 0.5; OW 0.3 | Cell = 0.3 | IR-2.3.6 |
| TC-IR-2.3.6.6 | Write survives to next tick | Write frame N | Read frame N+1 | IR-2.3.6 |
| TC-IR-2.3.6.7 | Propagation consumes write | Write + 1 propagation | Neighbors spread | IR-2.3.6 |

## Negative / Error-path Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.3.E1 | Sample outside 2D grid | Agent (-5,-5) | BB threat=default, Unknown | IR-2.3.1 |
| TC-IR-2.3.E2 | Sample outside 3D volume | Voxel (-1,0,0) | Default, Unknown flag | IR-2.3.1 |
| TC-IR-2.3.E3 | Flow field unreachable | valid=false | Fallback seek active | IR-2.3.2 |
| TC-IR-2.3.E4 | Fog with no coverage | No fog data | Treat as Unexplored | IR-2.3.3 |
| TC-IR-2.3.E5 | MPSC channel full | 4097 writes in 1 frame | 1 Dropped, warn logged | IR-2.3.6 |
| TC-IR-2.3.E6 | Stale grid entity | Wrong generation | DroppedStaleEntity | IR-2.3.6 |
| TC-IR-2.3.E7 | Missing position key | BB lacks position_key | DroppedMissingKey | IR-2.3.6 |
| TC-IR-2.3.E8 | Read skipped propagation | Prop budget exhausted | Front buffer stale | IR-2.3.1 |
| TC-IR-2.3.E9 | OOB write target | Cell outside grid | Drop, warn logged | IR-2.3.6 |

## Determinism Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.3.D1 | Reordered producers | Shuffle 100 writes | Same cell values | IR-2.3.6 |
| TC-IR-2.3.D2 | Repeated run same seed | 2 runs, same inputs | Bit-identical cells | IR-2.3.6 |
| TC-IR-2.3.D3 | Parallel Phase 4 write | 8 threads, same cell | Commutative fold ok | IR-2.3.6 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-2.3.1.B1 | 1000 2D influence samples | < 0.1 ms | IR-2.3.1 |
| TC-IR-2.3.1.B2 | 1000 3D voxel samples | < 0.15 ms | IR-2.3.1 |
| TC-IR-2.3.1.B3 | 1000 hierarchical LOD samples | < 0.15 ms | IR-2.3.1 |
| TC-IR-2.3.2.B1 | 1000 2D flow field reads | < 0.1 ms | IR-2.3.2 |
| TC-IR-2.3.2.B2 | 1000 3D flow field reads | < 0.15 ms | IR-2.3.2 |
| TC-IR-2.3.3.B1 | 500 fog state checks | < 0.05 ms | IR-2.3.3 |
| TC-IR-2.3.4.B1 | 500 utility scorings + curve | < 0.2 ms | IR-2.3.4 |
| TC-IR-2.3.4.B2 | 500 3D utility scorings | < 0.25 ms | IR-2.3.4 |
| TC-IR-2.3.5.B1 | 500 GOAP WorldState refreshes | < 0.25 ms | IR-2.3.5 |
| TC-IR-2.3.5.B2 | 500 GOAP 3D refreshes | < 0.3 ms | IR-2.3.5 |
| TC-IR-2.3.6.B1 | 256x256 drain + propagation | < 1 ms | IR-2.3.6 |
| TC-IR-2.3.6.B2 | 64x64x64 voxel drain + prop | < 2 ms | IR-2.3.6 |
| TC-IR-2.3.6.B3 | 4096 MPSC enqueues (8 threads) | < 0.3 ms | IR-2.3.6 |
