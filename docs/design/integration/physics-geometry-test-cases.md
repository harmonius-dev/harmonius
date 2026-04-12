# Physics ↔ World Geometry Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-3.8.1.1 | Triangle mesh collider from asset | Cube mesh asset | ColliderShape::TriangleMesh with 12 tris | IR-3.8.1 |
| TC-IR-3.8.1.2 | Simplified collision mesh | 10K-tri visual mesh | Collision mesh < 1K tris | IR-3.8.1 |
| TC-IR-3.8.1.3 | Dynamic body rests on tri-mesh | Sphere drops onto mesh floor | Sphere stops, contacts generated | IR-3.8.1 |
| TC-IR-3.8.2.1 | Heightfield collider from tile | 257x257 tile, peak at center | Collider height matches at peak | IR-3.8.2 |
| TC-IR-3.8.2.2 | Character walks on heightfield | Character controller on terrain | Ground state detected, smooth walk | IR-3.8.2 |
| TC-IR-3.8.3.1 | Collision ignores visual LOD | Object at LOD 3 visually | Collision uses full-res shape | IR-3.8.3 |
| TC-IR-3.8.3.2 | Physics resolution constant | Camera far from terrain | Heightfield collision unchanged | IR-3.8.3 |
| TC-IR-3.8.4.1 | Hole mask allows fall-through | Tile with hole at (5,5) | Entity falls through hole cell | IR-3.8.4 |
| TC-IR-3.8.4.2 | Hole mask blocks adjacent | Tile hole at (5,5), entity at (6,5) | Entity collides normally | IR-3.8.4 |
| TC-IR-3.8.5.1 | Voxel chunk generates collider | 16^3 chunk, half solid | Triangle mesh from marching cubes | IR-3.8.5 |
| TC-IR-3.8.5.2 | Voxel edit rebuilds collision | Remove voxel at (4,4,4) | Updated collision mesh next frame | IR-3.8.5 |
| TC-IR-3.8.5.3 | Entity collides with voxel mesh | Sphere rolls over voxel terrain | Contacts generated at surface | IR-3.8.5 |
| TC-IR-3.8.6.1 | Collision layer filters terrain | Projectile layer=0x04, terrain mask=0x01 | No collision with terrain | IR-3.8.6 |
| TC-IR-3.8.6.2 | Character collides with terrain | Character layer=0x01, terrain mask=0x01 | Normal collision response | IR-3.8.6 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-3.8.1.B1 | Tri-mesh narrowphase 10K pairs | < 2 ms (R-4.2.NF2) | IR-3.8.1 |
| TC-IR-3.8.2.B1 | Heightfield build 257x257 | < 5 ms CPU | IR-3.8.2 |
| TC-IR-3.8.5.B1 | Voxel remesh 16^3 chunk | < 5 ms CPU | IR-3.8.5 |
| TC-IR-3.8.2.B2 | 200 characters on terrain | < 4 ms total physics | IR-3.8.2 |
