# Rendering ↔ Physics Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-3.4.1.1 | Sphere collider draws wireframe | Sphere r=1.0 at origin | Circle lines at correct radius | IR-3.4.1 |
| TC-IR-3.4.1.2 | Box collider draws wireframe | Box half-extents (1,2,3) | 12 edge lines at correct size | IR-3.4.1 |
| TC-IR-3.4.1.3 | Capsule collider draws wireframe | Capsule r=0.5, h=2.0 | Hemisphere caps + cylinder lines | IR-3.4.1 |
| TC-IR-3.4.1.4 | Color matches body type | Static=blue, dynamic=green, kinematic=yellow | Correct colors per type | IR-3.4.1 |
| TC-IR-3.4.2.1 | Contact normals draw arrows | Two boxes touching, normal=(0,1,0) | Arrow pointing up at contact | IR-3.4.2 |
| TC-IR-3.4.2.2 | Penetration scales arrow | Penetration 0.1 vs 0.5 | Longer arrow for deeper pen | IR-3.4.2 |
| TC-IR-3.4.3.1 | BVH leaf nodes draw green | 10 entities in BVH | 10 green AABB wireframes | IR-3.4.3 |
| TC-IR-3.4.3.2 | BVH depth filter works | max_depth=2, tree depth=5 | Only top 2 levels drawn | IR-3.4.3 |
| TC-IR-3.4.4.1 | Raycast hit draws green line | Ray hits box at t=5.0 | Green line origin to hit | IR-3.4.4 |
| TC-IR-3.4.4.2 | Raycast miss draws red line | Ray misses all objects | Red line origin to max dist | IR-3.4.4 |
| TC-IR-3.4.5.1 | Debug stripped in shipping | Build without debug_draw feature | Zero debug draw calls | IR-3.4.5 |
| TC-IR-3.4.6.1 | Interpolation smooths motion | Physics 30 Hz, render 60 Hz | Object moves smoothly | IR-3.4.6 |
| TC-IR-3.4.6.2 | Alpha clamped to [0,1] | Accumulator edge case alpha=1.5 | Clamped to 1.0, no overshoot | IR-3.4.6 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-3.4.1.B1 | 2000 collider wireframes | < 1 ms GPU | IR-3.4.1 |
| TC-IR-3.4.2.B1 | 5000 contact point arrows | < 0.5 ms GPU | IR-3.4.2 |
| TC-IR-3.4.3.B1 | BVH viz 50K nodes depth=3 | < 1 ms GPU | IR-3.4.3 |
| TC-IR-3.4.5.B1 | Shipping build overhead | 0 ms (stripped) | IR-3.4.5 |
| TC-IR-3.4.6.B1 | Interpolation 10K bodies | < 0.1 ms CPU | IR-3.4.6 |
