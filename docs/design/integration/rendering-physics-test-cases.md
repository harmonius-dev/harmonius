# Rendering ↔ Physics Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-3.4.1.1 | Sphere wireframe | Sphere r=1.0 | Circle lines at radius | IR-3.4.1 |
| TC-IR-3.4.1.2 | Box wireframe | Half-extents (1,2,3) | 12 edge lines | IR-3.4.1 |
| TC-IR-3.4.1.3 | Capsule wireframe | r=0.5, h=2.0 | Hemispheres + cylinder | IR-3.4.1 |
| TC-IR-3.4.1.4 | Convex hull wireframe | 8-vertex hull | N edge lines match hull | IR-3.4.1 |
| TC-IR-3.4.1.5 | Triangle mesh wireframe | 100-tri mesh | 300 edge lines drawn | IR-3.4.1 |
| TC-IR-3.4.1.6 | Heightfield wireframe | 16x16 heightfield | Grid lines per cell | IR-3.4.1 |
| TC-IR-3.4.1.7 | Color per body type | Static, dynamic, kinematic | Correct colors per type | IR-3.4.1 |
| TC-IR-3.4.2.1 | Contact normal arrow | Two boxes, normal=(0,1,0) | Arrow up at contact | IR-3.4.2 |
| TC-IR-3.4.2.2 | Penetration scales arrow | Pen 0.1 vs 0.5 | Longer arrow deeper | IR-3.4.2 |
| TC-IR-3.4.2.3 | Contact point dots | 4-point manifold | 4 dots at positions | IR-3.4.2 |
| TC-IR-3.4.3.1 | BVH leaf green | 10 entities | 10 green AABBs | IR-3.4.3 |
| TC-IR-3.4.3.2 | BVH depth filter | max_depth=2, tree=5 | Top 2 levels drawn | IR-3.4.3 |
| TC-IR-3.4.3.3 | BVH internal yellow | 3-level tree | Internal nodes yellow | IR-3.4.3 |
| TC-IR-3.4.4.1 | Raycast hit green | Ray hits box at t=5.0 | Green line to hit | IR-3.4.4 |
| TC-IR-3.4.4.2 | Raycast miss red | Ray misses all | Red line to max dist | IR-3.4.4 |
| TC-IR-3.4.4.3 | ShapeCast swept volume | Capsule sweep 0..5 | Start + end + tangents | IR-3.4.4 |
| TC-IR-3.4.5.1 | Runtime toggle off | All flags false at runtime | Zero lines emitted | IR-3.4.5 |
| TC-IR-3.4.5.2 | Runtime toggle on | Flags toggled true live | Lines appear same frame | IR-3.4.5 |
| TC-IR-3.4.5.3 | Shipping build has config | Release profile | Config struct exists | IR-3.4.5 |
| TC-IR-3.4.6.1 | Interp smooths motion | 30 Hz phys, 60 Hz render | Object moves smoothly | IR-3.4.6 |
| TC-IR-3.4.6.2 | Alpha clamped | Accumulator alpha=1.5 | Res value = 1.0 | IR-3.4.6 |
| TC-IR-3.4.6.3 | Alpha frame-global | 1000 interp entities | Single Res, not per entity | IR-3.4.6 |

## Negative Tests (Failure Modes)

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-3.4.N1 | Line budget overflow | Colliders > max_debug_lines | Excess dropped, counter++ | IR-3.4.1 |
| TC-IR-3.4.N2 | Stale contacts cleared | Contacts frame N only | Frame N+1 buffer empty | IR-3.4.2 |
| TC-IR-3.4.N3 | BVH depth clamp | bvh_max_depth=3, tree=10 | No nodes below depth 3 | IR-3.4.3 |
| TC-IR-3.4.N4 | Alpha > 1 clamp | accumulator > fixed_dt | Res alpha == 1.0 | IR-3.4.6 |
| TC-IR-3.4.N5 | MPSC channel full | Producer faster than render | Oldest dropped, newest kept | IR-3.4.5 |
| TC-IR-3.4.N6 | Empty BVH walk | Zero entities in BVH | Zero AABB lines | IR-3.4.3 |
| TC-IR-3.4.N7 | Raycast zero length | origin == max_dist | No NaN, zero-length line | IR-3.4.4 |
| TC-IR-3.4.N8 | ShapeCast no hit | Sweep misses everything | Swept outline only, red | IR-3.4.4 |

All negative tests are CI-runnable: they use the software physics solver and a headless debug buffer
inspector. None require a GPU or display.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-3.4.1.B1 | 2000 collider wireframes | < 1 ms GPU | IR-3.4.1 |
| TC-IR-3.4.1.B2 | 500 triangle mesh colliders | < 2 ms GPU | IR-3.4.1 |
| TC-IR-3.4.2.B1 | 5000 contact arrows + dots | < 0.5 ms GPU | IR-3.4.2 |
| TC-IR-3.4.3.B1 | BVH viz 50K nodes depth=3 | < 1 ms GPU | IR-3.4.3 |
| TC-IR-3.4.4.B1 | 200 raycast + shapecast viz | < 0.3 ms GPU | IR-3.4.4 |
| TC-IR-3.4.5.B1 | Runtime toggle overhead off | < 50 ns CPU | IR-3.4.5 |
| TC-IR-3.4.5.B2 | Runtime toggle on, empty scene | < 1 us CPU | IR-3.4.5 |
| TC-IR-3.4.6.B1 | Interpolation 10K bodies | < 0.1 ms CPU | IR-3.4.6 |
| TC-IR-3.4.6.B2 | Alpha compute per frame | < 10 ns CPU | IR-3.4.6 |
