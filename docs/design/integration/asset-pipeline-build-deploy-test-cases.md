# Asset Pipeline ↔ Build/Deploy Integration Test Cases

## Integration Tests

| ID | Name | Req |
|----|------|-----|
| TC-IR-5.1.1.1 | Cook all assets for Windows | IR-5.1.1 |
| TC-IR-5.1.1.2 | Cook all assets for macOS | IR-5.1.1 |
| TC-IR-5.1.2.1 | PlatformProfile selects BC7 on Windows | IR-5.1.2 |
| TC-IR-5.1.2.2 | PlatformProfile selects ASTC on iOS | IR-5.1.2 |
| TC-IR-5.1.3.1 | Incremental build skips unchanged | IR-5.1.3 |
| TC-IR-5.1.3.2 | Cache invalidation forces rebuild | IR-5.1.3 |
| TC-IR-5.1.4.1 | BundleBuilder produces valid pak | IR-5.1.4 |
| TC-IR-5.1.4.2 | Bundle respects size limits | IR-5.1.4 |
| TC-IR-5.1.5.1 | Shader variants compiled per platform | IR-5.1.5 |
| TC-IR-5.1.5.2 | Shader cache hit avoids recompile | IR-5.1.5 |
| TC-IR-5.1.6.1 | Delta patch uses BLAKE3 diff | IR-5.1.6 |
| TC-IR-5.1.6.2 | Identical content zero-byte patch | IR-5.1.6 |
| TC-IR-5.1.6.3 | Corrupted v1 bundle fails gracefully | IR-5.1.6 |
| TC-IR-5.1.6.N1 | Mismatched Merkle root rejected | IR-5.1.6 |
| TC-IR-5.1.7.1 | Shared CAS cache hit on CI | IR-5.1.7 |
| TC-IR-5.1.7.2 | Cache miss falls back to cook | IR-5.1.7 |
| TC-IR-5.1.7.3 | Concurrent CI writes same key | IR-5.1.7 |
| TC-IR-5.1.7.4 | Cache eviction under pressure | IR-5.1.7 |
| TC-IR-5.1.7.N1 | Malformed CAS key rejected | IR-5.1.7 |

### Details

1. **TC-IR-5.1.1.1** — Input: 100 mixed assets, Windows profile. Expected: `CookedManifest` with
   BC7-compressed textures and SPIR-V shader variants, all entries stored in the CAS with valid
   `blake3` keys.
2. **TC-IR-5.1.1.2** — Input: same 100 assets, macOS profile. Expected: `CookedManifest` with ASTC
   textures and `.SPIR-V module` shader variants produced via the `naga` -> `naga`
   subprocess pipeline.
3. **TC-IR-5.1.2.1** — Input: one texture asset, Windows target. Expected: BC7-compressed output
   stored in the CAS under a stable content-addressed key.
4. **TC-IR-5.1.2.2** — Input: one texture asset, iOS target. Expected: ASTC-compressed output stored
   in the CAS.
5. **TC-IR-5.1.3.1** — Input: 100 assets, 1 changed. Expected: exactly 1 asset re-processed;
   remaining 99 served from the incremental cache.
6. **TC-IR-5.1.3.2** — Input: invalidate 5 assets in the cache. Expected: exactly 5 re-processed on
   the next cook.
7. **TC-IR-5.1.4.1** — Input: `CookedManifest` with 50 entries. Expected: `BundleSet` with valid
   per-bundle BLAKE3 hashes.
8. **TC-IR-5.1.4.2** — Input: 1 GB of cooked assets, 256 MB bundle size limit. Expected: multiple
   bundles produced, each under the limit, with consistent ordering.
9. **TC-IR-5.1.5.1** — Input: 1 shader graph, 3 target platforms. Expected: 3 distinct bytecode
   artifacts (SPIR-V, SPIR-V, SPIR-V module) stored under separate CAS keys.
10. **TC-IR-5.1.5.2** — Input: same shader compiled in a second build. Expected: cache hit, zero
    `naga` subprocess invocations.
11. **TC-IR-5.1.6.1** — Input: v1 and v2 bundles that share most content. Expected: patch bundle
    strictly smaller than v2, verified by byte-size comparison.
12. **TC-IR-5.1.6.2** — Input: v1 and v2 with identical content. Expected: patch size is exactly 0
    bytes.
13. **TC-IR-5.1.6.3** — Input: corrupted v1, valid v2. Expected: error result surfaced to the
    caller; no crash, no panic, no partial write.
14. **TC-IR-5.1.6.N1** (negative) — Input: v2 with a tampered `blake3_root`. Expected: delta patcher
    refuses the input and returns a `MerkleRootMismatch` error.
15. **TC-IR-5.1.7.1** — Input: pre-warmed cache, clean checkout. Expected: zero processing, every
    asset served as a cache hit.
16. **TC-IR-5.1.7.2** — Input: empty cache, 10 assets. Expected: all 10 cooked and stored in the
    CAS.
17. **TC-IR-5.1.7.3** — Input: 2 CI agents writing the same CAS key concurrently. Expected: exactly
    1 CAS entry, no corruption, no partial writes (fake CAS emulates platform-native I/O).
18. **TC-IR-5.1.7.4** — Input: cache at capacity, new entry. Expected: the LRU entry is evicted and
    the new entry is stored.
19. **TC-IR-5.1.7.N1** (negative) — Input: CAS key with wrong length (not 32 bytes). Expected: error
    returned, no write attempted, no partial state left in the CAS.

### Execution

All integration tests run via `cargo test -p harmonius-integration-tests` with no external services.
CI hosts invoke the same command; there is no GitHub Actions dependency and no external credentials
are required. Fakes emulate platform I/O behavior (io_uring, GCD `dispatch_io`, IOCP) so the tests
run on any host that can build the workspace. Shader-pipeline tests invoke bundled `naga`
in-process; no external shader compiler binaries are required.

## Benchmarks

| ID | Name | Target | Req |
|----|------|--------|-----|
| TC-IR-5.1.1.B1 | Full cook 1000 assets | < 60 s (8-core) | IR-5.1.1 |
| TC-IR-5.1.3.B1 | Incremental cook 1 of 1000 | < 2 s | IR-5.1.3 |
| TC-IR-5.1.5.B1 | 100 shader variants parallel | < 30 s | IR-5.1.5 |
| TC-IR-5.1.6.B1 | Delta patch 1 GB, 1% changed | < 5 s, patch < 20 MB | IR-5.1.6 |
| TC-IR-5.1.6.B2 | BLAKE3 Merkle root 100k entries | < 50 ms (8-core) | IR-5.1.6 |
| TC-IR-5.1.7.B1 | Cache lookup latency (L1) | < 1 ms | IR-5.1.7 |
| TC-IR-5.1.7.B2 | CAS store 10k entries mmap | < 2 s (8-core) | IR-5.1.7 |

Benchmarks run via `cargo bench -p harmonius-integration-benches`. They execute on developer
workstations and CI hosts without external services. Benchmark inputs are generated in-process; no
network, no cloud storage, no GitHub Actions.
