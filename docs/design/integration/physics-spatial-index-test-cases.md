# Physics ↔ Spatial Index Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-3.9.1.1 | Broadphase finds overlapping pair | Two boxes overlapping AABBs | Pair returned from BVH query | IR-3.9.1 |
| TC-IR-3.9.1.2 | Broadphase excludes distant pair | Two boxes 100m apart | No pair returned | IR-3.9.1 |
| TC-IR-3.9.1.3 | Broadphase feeds narrowphase | Overlapping pair found | ContactManifold generated | IR-3.9.1 |
| TC-IR-3.9.2.1 | Single BVH serves all queries | Physics + AI + audio query | All use same BvhIndex resource | IR-3.9.2 |
| TC-IR-3.9.3.1 | Layer filter passes matching | A: member=0x01 mask=0x02, B: member=0x02 mask=0x01 | Pair returned | IR-3.9.3 |
| TC-IR-3.9.3.2 | Layer filter rejects mismatch | A: member=0x01 mask=0x01, B: member=0x02 mask=0x02 | No pair returned | IR-3.9.3 |
| TC-IR-3.9.3.3 | Trigger volume uses layers | Trigger layer=0x04, player=0x04 | TriggerEnter event fired | IR-3.9.3 |
| TC-IR-3.9.4.1 | Batch ray cast parallel | 100 rays from same origin | All results returned, parallel | IR-3.9.4 |
| TC-IR-3.9.4.2 | Batch shape cast for CCD | 50 swept spheres | TOI results for each | IR-3.9.4 |
| TC-IR-3.9.4.3 | Batch scales with threads | 1000 queries, 4 vs 8 threads | Near-linear speedup | IR-3.9.4 |
| TC-IR-3.9.5.1 | BVH refits after physics move | 100 dynamic bodies moved | BVH AABBs match new positions | IR-3.9.5 |
| TC-IR-3.9.5.2 | Static body skips refit | 50 static bodies unchanged | Zero refit operations for static | IR-3.9.5 |
| TC-IR-3.9.5.3 | Spawned entity inserted | New entity spawned mid-frame | Entity in BVH next query | IR-3.9.5 |
| TC-IR-3.9.5.4 | Despawned entity removed | Entity despawned | No stale entry in BVH | IR-3.9.5 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-3.9.1.B1 | Broadphase 50K AABBs | < 1 ms (R-4.2.NF1) | IR-3.9.1 |
| TC-IR-3.9.4.B1 | Ray cast 1M entities | < 10 us per ray (R-1.9.4a) | IR-3.9.4 |
| TC-IR-3.9.4.B2 | Frustum cull 1M entities | < 500 us (R-1.9.4a) | IR-3.9.4 |
| TC-IR-3.9.5.B1 | Refit 1000 moved entities | < 0.5 ms | IR-3.9.5 |
| TC-IR-3.9.5.B2 | BVH memory per entity | <= 64 bytes (R-1.9.1a) | IR-3.9.5 |
