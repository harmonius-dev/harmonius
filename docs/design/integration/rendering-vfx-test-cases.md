# Rendering ↔ VFX Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-3.7.1.1 | Particle sim in render graph | 1000-particle emitter | Compute pass dispatches correctly | IR-3.7.1 |
| TC-IR-3.7.1.2 | Barrier between sim and draw | Sim writes, draw reads same buffer | No data race, valid pixels | IR-3.7.1 |
| TC-IR-3.7.2.1 | Sprite particles draw indirect | 500 billboard particles | DrawIndirect with correct count | IR-3.7.2 |
| TC-IR-3.7.2.2 | Ribbon particles draw indirect | Ribbon trail emitter | Connected strip geometry | IR-3.7.2 |
| TC-IR-3.7.2.3 | Alpha sort by camera distance | 100 transparent particles | Back-to-front draw order | IR-3.7.2 |
| TC-IR-3.7.3.1 | Fog volume injects to froxel | Fog volume AABB in view | Froxel cells have density > 0 | IR-3.7.3 |
| TC-IR-3.7.3.2 | Weather particles add scattering | Rain at intensity=1.0 | Froxel scattering increased | IR-3.7.3 |
| TC-IR-3.7.4.1 | Decal modifies G-buffer albedo | Blood decal on wall | Albedo channel changed at decal | IR-3.7.4 |
| TC-IR-3.7.4.2 | Decal priority layering | Two decals, priority 1 and 2 | Higher priority on top | IR-3.7.4 |
| TC-IR-3.7.4.3 | Decal angle threshold clips | Decal on 80-deg slope, threshold=60 | Decal fades on steep surface | IR-3.7.4 |
| TC-IR-3.7.5.1 | Particle lights in cluster grid | 10 emissive particles | 10 point lights in GpuLightBuffer | IR-3.7.5 |
| TC-IR-3.7.5.2 | Particle lights capped at budget | 1000 particle lights, budget=100 | Only 100 brightest injected | IR-3.7.5 |
| TC-IR-3.7.6.1 | Heat haze distortion renders | Haze source in view | Screen-space distortion visible | IR-3.7.6 |
| TC-IR-3.7.6.2 | Shockwave expands correctly | Shockwave at origin, t=0.5 | Ring distortion at half radius | IR-3.7.6 |
| TC-IR-3.7.7.1 | LOD reduces at distance | Emitter at 200m, full_dist=50m | LodTier::Reduced active | IR-3.7.7 |
| TC-IR-3.7.7.2 | LOD culls at max distance | Emitter at 500m, cull_dist=400m | LodTier::Culled, no render | IR-3.7.7 |
| TC-IR-3.7.7.3 | LOD hysteresis prevents pop | Emitter oscillates at boundary | No rapid tier switching | IR-3.7.7 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-3.7.1.B1 | Particle sim 100K | < 1 ms GPU | IR-3.7.1 |
| TC-IR-3.7.2.B1 | Particle draw 100K sprites | < 2 ms GPU | IR-3.7.2 |
| TC-IR-3.7.2.B2 | GPU radix sort 100K | < 0.5 ms GPU | IR-3.7.2 |
| TC-IR-3.7.3.B1 | Froxel injection 10 volumes | < 0.5 ms GPU | IR-3.7.3 |
| TC-IR-3.7.4.B1 | 100 decals on G-buffer | < 0.5 ms GPU | IR-3.7.4 |
| TC-IR-3.7.5.B1 | 100 particle lights inject | < 0.1 ms CPU | IR-3.7.5 |
