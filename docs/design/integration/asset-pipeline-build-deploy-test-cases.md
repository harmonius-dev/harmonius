# Asset Pipeline ↔ Build/Deploy Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-5.1.1.1 | Cook all assets for Windows | 100 mixed assets, Windows profile | CookedManifest with BC7 textures, DXIL shaders | IR-5.1.1 |
| TC-IR-5.1.1.2 | Cook all assets for macOS | Same 100 assets, macOS profile | CookedManifest with ASTC textures, metallib shaders | IR-5.1.1 |
| TC-IR-5.1.2.1 | PlatformProfile selects BC7 on Windows | Texture asset, Windows target | BC7-compressed output in CAS | IR-5.1.2 |
| TC-IR-5.1.2.2 | PlatformProfile selects ASTC on iOS | Texture asset, iOS target | ASTC-compressed output in CAS | IR-5.1.2 |
| TC-IR-5.1.3.1 | Incremental build skips unchanged | 100 assets, 1 changed | Only 1 asset re-processed | IR-5.1.3 |
| TC-IR-5.1.3.2 | Cache invalidation forces rebuild | Invalidate 5 assets | Exactly 5 re-processed | IR-5.1.3 |
| TC-IR-5.1.4.1 | BundleBuilder produces valid pak | CookedManifest with 50 entries | BundleSet with BLAKE3 hashes | IR-5.1.4 |
| TC-IR-5.1.4.2 | Bundle respects size limits | 1 GB of cooked assets, 256 MB limit | Multiple bundles, each under limit | IR-5.1.4 |
| TC-IR-5.1.5.1 | Shader variants compiled per platform | 1 shader graph, 3 platforms | 3 distinct bytecode artifacts | IR-5.1.5 |
| TC-IR-5.1.5.2 | Shader cache hit avoids recompile | Same shader, second build | Cache hit, zero dxc invocations | IR-5.1.5 |
| TC-IR-5.1.6.1 | Delta patch uses BLAKE3 diff | v1 and v2 bundles | Patch bundle smaller than v2 | IR-5.1.6 |
| TC-IR-5.1.7.1 | Shared CAS cache hit on CI | Pre-warmed cache, clean checkout | Zero processing, all cache hits | IR-5.1.7 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-5.1.1.B1 | Full cook 1000 assets | < 60 s (8-core) | IR-5.1.1 |
| TC-IR-5.1.3.B1 | Incremental cook 1 changed of 1000 | < 2 s | IR-5.1.3 |
| TC-IR-5.1.5.B1 | 100 shader variants parallel compile | < 30 s | IR-5.1.5 |
| TC-IR-5.1.7.B1 | Cache lookup latency (local L1) | < 1 ms | IR-5.1.7 |
