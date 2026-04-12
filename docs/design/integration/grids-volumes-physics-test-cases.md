# Grids/Volumes ↔ Physics Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-3.10.1.1 | Height grid feeds collider | 64x64 grid, peak at (32,32) | Heightfield collider matches peak | IR-3.10.1 |
| TC-IR-3.10.1.2 | Grid edit updates collider | Raise cell (10,10) by 5.0 | Collider height at (10,10) = +5.0 | IR-3.10.1 |
| TC-IR-3.10.1.3 | Cell-to-world maps correctly | Cell (0,0), cell_size=2.0, origin=(10,10) | World pos (10,10) | IR-3.10.1 |
| TC-IR-3.10.2.1 | Voxel chunk generates collider | 16^3 chunk, half solid | TriangleMesh with valid normals | IR-3.10.2 |
| TC-IR-3.10.2.2 | Voxel place rebuilds collider | Place block at (8,8,8) | Updated mesh includes new block | IR-3.10.2 |
| TC-IR-3.10.2.3 | Voxel remove rebuilds collider | Remove block at (8,8,8) | Updated mesh excludes block | IR-3.10.2 |
| TC-IR-3.10.3.1 | Destruction clears voxels | Sphere blast r=3 at (8,8,8) | Voxels in radius set to air | IR-3.10.3 |
| TC-IR-3.10.3.2 | Destruction triggers remesh | Blast in chunk (0,0,0) | Chunk marked dirty, remeshed | IR-3.10.3 |
| TC-IR-3.10.3.3 | Destruction updates collision | Blast removes floor voxels | Entity falls through gap | IR-3.10.3 |
| TC-IR-3.10.3.4 | Impact outside volume ignored | Blast at (-100,-100,-100) | No voxel changes, no crash | IR-3.10.3 |
| TC-IR-3.10.4.1 | Occupancy tracks body position | Body at world (5.0, 0.0) | Cell (2,0) marked occupied | IR-3.10.4 |
| TC-IR-3.10.4.2 | Occupancy clears on body move | Body moves from (5,0) to (15,0) | Old cell cleared, new cell set | IR-3.10.4 |
| TC-IR-3.10.4.3 | Multiple bodies in one cell | Two bodies in same cell | Cell occupied, both tracked | IR-3.10.4 |
| TC-IR-3.10.5.1 | Propagation blocked by wall | Fire kernel, wall between cells | Fire does not cross wall | IR-3.10.5 |
| TC-IR-3.10.5.2 | Propagation passes open space | Fire kernel, no obstacles | Fire spreads to neighbor | IR-3.10.5 |
| TC-IR-3.10.5.3 | LOS raycast uses shared BVH | Ray from cell A to cell B | Hit result matches wall entity | IR-3.10.5 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-3.10.1.B1 | Heightfield rebuild 257x257 | < 5 ms CPU | IR-3.10.1 |
| TC-IR-3.10.2.B1 | Voxel remesh 16^3 chunk | < 5 ms CPU (NFR-SIM.GV4) | IR-3.10.2 |
| TC-IR-3.10.3.B1 | Destruction blast r=5 | < 1 ms CPU | IR-3.10.3 |
| TC-IR-3.10.4.B1 | Occupancy update 1000 bodies | < 0.5 ms CPU | IR-3.10.4 |
| TC-IR-3.10.5.B1 | Propagation 256x256 with LOS | < 1 ms (NFR-SIM.GV1) | IR-3.10.5 |
