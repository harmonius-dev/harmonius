# integration_physics_geometry

Executable contracts for
[`docs/design/integration/physics-geometry.md`](../../docs/design/integration/physics-geometry.md).

## Companion test-case coverage

Rows map companion doc
[`physics-geometry-test-cases.md`](../../docs/design/integration/physics-geometry-test-cases.md) IDs
to this crate. Full engine, bake, and CI scenarios stay future work.

| TC-ID | Status in this crate |
|-------|----------------------|
| TC-IR-3.8.1.1 | Covered (`collider_shape` unit test) |
| TC-IR-3.8.1.2–1.3 | Not covered (needs bake / dynamics) |
| TC-IR-3.8.2.1 | Partial (peak tile via unit test) |
| TC-IR-3.8.2.2 | Not covered (needs character controller) |
| TC-IR-3.8.2.3 | Covered (`heightfield` unit test) |
| TC-IR-3.8.3.* | Not covered (needs LOD / camera harness) |
| TC-IR-3.8.4.1–4.2 | Partial (bit layout unit tests) |
| TC-IR-3.8.4.3 | Covered (`heightfield` unit test) |
| TC-IR-3.8.5.* | Not covered (needs voxel mesher) |
| TC-IR-3.8.6.* | Not covered (needs broadphase pair test) |
| TC-IR-3.8.E1–E4, E6–E7 | Not covered (runtime / logging harness) |
| TC-IR-3.8.E5 | Covered (`heightfield` unit test) |
| Benchmarks | Not in this crate |
