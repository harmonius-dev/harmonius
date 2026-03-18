# R-15.11 -- Shared Build Cache Requirements

## Compiled Asset Caching

| ID        | Derived From |
|-----------|--------------|
| R-15.11.1 |              |
| R-15.11.2 |              |
| R-15.11.3 |              |

1. **R-15.11.1** — The engine **SHALL** provide a shared cache service storing compiled assets keyed
   by content hash (source content, build settings, tool version), with REST API over HTTPS and
   token authentication, so that identical builds are never repeated across the team.
   [F-15.11.1](../../features/tools-editor/shared-cache.md) duplicate work. and verify identical
   output without rebuild.
   - **Rationale:** Redundant builds waste developer time; content-addressed caching eliminates
   - **Verification:** Unit test: build an asset, upload to cache, fetch from cache on another
     machine,
2. **R-15.11.2** — Compiled shader variants **SHALL** be cached per platform keyed by shader source
   hash and feature permutation flags, with CI builds populating the cache for all platforms.
   [F-15.11.2](../../features/tools-editor/shared-cache.md)
   - **Rationale:** Full shader rebuilds take hours; caching reduces this to download time.
   - **Verification:** Unit test: verify cached shader output matches CI-compiled output
     byte-for-byte.
3. **R-15.11.3** — Compiled graph bytecode and AOT native code **SHALL** be cached by content hash
   and target platform so that unchanged graphs are not recompiled on every launch.
   [F-15.11.3](../../features/tools-editor/shared-cache.md)
   - **Rationale:** Thousands of graphs must not recompile on every editor launch or branch switch.
   - **Verification:** Unit test: verify an unchanged graph fetches from cache instead of
     recompiling.

## Onboarding

| ID        | Derived From |
|-----------|--------------|
| R-15.11.4 |              |

1. **R-15.11.4** — First editor launch **SHALL** fetch all compiled assets from the shared cache in
   parallel with Git LFS downloads, with a progress dashboard showing download status and ETAs,
   reducing first-launch time from hours to under 10 minutes for projects that take 1+ hours to
   build from source. [F-15.11.4](../../features/tools-editor/shared-cache.md) hours from source.
   - **Rationale:** Multi-hour first builds block new developer productivity.
   - **Verification:** Benchmark: verify first-launch time is under 10 minutes for a project taking
     1+

## Cache Lifecycle

| ID        | Derived From |
|-----------|--------------|
| R-15.11.5 |              |

1. **R-15.11.5** — Cache entries **SHALL** be invalidated on tool version change, with configurable
   garbage collection schedules, LRU eviction when storage quota is exceeded, and manual garbage
   collection with invalidation logs. [F-15.11.5](../../features/tools-editor/shared-cache.md)
   - **Rationale:** Stale cache entries cause correctness issues; unbounded growth wastes storage.
   - **Verification:** Unit test: change the tool version and verify all old entries are
     invalidated.

## Transport and Storage

| ID        | Derived From |
|-----------|--------------|
| R-15.11.6 |              |

1. **R-15.11.6** — The cache **SHALL** support multiple storage backends (S3, GCS, Azure, local,
   HTTP), Zstd compression for transfers, configurable download concurrency and bandwidth limits,
   and platform-native HTTP clients (NSURLSession, WinHTTP, libcurl).
   [F-15.11.6](../../features/tools-editor/shared-cache.md) network saturation.
   - **Rationale:** Infrastructure diversity requires pluggable backends; bandwidth limits prevent
   - **Verification:** Unit test: verify bandwidth limit is respected during downloads.

## CI/CD Population

| ID        | Derived From |
|-----------|--------------|
| R-15.11.7 |              |

1. **R-15.11.7** — CI builds **SHALL** auto-populate the cache for all platforms with idempotent
   uploads that skip existing entries, nightly builds warming cache for active branches, and a
   standalone CLI for headless build servers.
   [F-15.11.7](../../features/tools-editor/shared-cache.md)
   - **Rationale:** CI-populated caches ensure developers always have pre-built assets available.
   - **Verification:** Unit test: upload the same entry twice and verify the second upload is a
     no-op.

## Monitoring

| ID        | Derived From |
|-----------|--------------|
| R-15.11.8 |              |

1. **R-15.11.8** — The cache **SHALL** provide a monitoring dashboard with hit rate and storage
   usage, configurable alerts when hit rate drops below threshold, per-developer and per-team usage
   breakdowns, and metrics exported in OpenTelemetry format.
   [F-15.11.8](../../features/tools-editor/shared-cache.md)
   - **Rationale:** Cache effectiveness monitoring catches misconfigurations and tracks ROI.
   - **Verification:** Unit test: drop hit rate below threshold and verify the alert fires.

---

## US-15.11.1 Centralized Compiled Asset Cache

| ID           | Persona       | Features | Requirements |
|--------------|---------------|----------|--------------|
| US-15.11.1.1 | developer     |          |              |
| US-15.11.1.2 | developer     |          |              |
| US-15.11.1.3 | DevOps        |          |              |
| US-15.11.1.4 | DevOps        |          |              |
| US-15.11.1.5 | engine tester |          |              |

1. **US-15.11.1.1** — I want compiled assets cached by content hash so that identical builds are
   never repeated across the team.
2. **US-15.11.1.2** — I want to download cached results instead of rebuilding so that my build times
   are reduced to cache fetch times.
3. **US-15.11.1.3** — I want the cache keyed by source content, build settings, and tool version so
   that cache entries are invalidated correctly on toolchain changes.
4. **US-15.11.1.4** — I want REST API over HTTPS with token authentication so that cache access is
   secure and auditable.
5. **US-15.11.1.5** — I want to verify a cache hit returns the identical compiled asset without
   rebuild so that cache correctness is regression-tested.

## US-15.11.2 Shader Compilation Cache

| ID           | Persona       | Features | Requirements |
|--------------|---------------|----------|--------------|
| US-15.11.2.1 | developer     |          |              |
| US-15.11.2.2 | developer     |          |              |
| US-15.11.2.3 | DevOps        |          |              |
| US-15.11.2.4 | engine tester |          |              |

1. **US-15.11.2.1** — I want shader variants cached per platform so that I download pre-compiled
   shaders instead of rebuilding.
2. **US-15.11.2.2** — I want cache keyed by shader source hash and feature permutation flags so that
   variant-level caching is precise.
3. **US-15.11.2.3** — I want CI builds to populate shader cache for all platforms so that developers
   always have pre-compiled shaders available.
4. **US-15.11.2.4** — I want to verify cached shader output matches CI-compiled output byte-for-byte
   so that shader cache integrity is regression-tested.

## US-15.11.3 Logic Graph Compilation Cache

| ID           | Persona       | Features | Requirements |
|--------------|---------------|----------|--------------|
| US-15.11.3.1 | developer     |          |              |
| US-15.11.3.2 | developer     |          |              |
| US-15.11.3.3 | engine tester |          |              |

1. **US-15.11.3.1** — I want compiled graph bytecode cached by content hash so that unchanged graphs
   are not recompiled on every launch.
2. **US-15.11.3.2** — I want AOT native code cached per target platform so that platform-specific
   compiled graphs are reused.
3. **US-15.11.3.3** — I want to verify unchanged graphs fetch from cache instead of recompiling so
   that graph cache hit behavior is regression-tested.

## US-15.11.4 New Developer Onboarding Acceleration

| ID           | Persona       | Features | Requirements |
|--------------|---------------|----------|--------------|
| US-15.11.4.1 | developer     |          |              |
| US-15.11.4.2 | developer     |          |              |
| US-15.11.4.3 | developer     |          |              |
| US-15.11.4.4 | engine tester |          |              |

1. **US-15.11.4.1** — I want first editor launch to fetch all compiled assets from cache so that my
   first build takes minutes instead of hours.
2. **US-15.11.4.2** — I want cache prefetch running in parallel with Git LFS downloads so that both
   operations saturate available bandwidth.
3. **US-15.11.4.3** — I want a progress dashboard showing download status so that I can monitor
   onboarding progress with ETAs.
4. **US-15.11.4.4** — I want to verify first-launch time is under 10 minutes for a project taking 1+
   hours to build from source so that onboarding acceleration is regression-tested.

## US-15.11.5 Cache Invalidation and Garbage Collection

| ID           | Persona       | Features | Requirements |
|--------------|---------------|----------|--------------|
| US-15.11.5.1 | DevOps        |          |              |
| US-15.11.5.2 | DevOps        |          |              |
| US-15.11.5.3 | DevOps        |          |              |
| US-15.11.5.4 | server admin  |          |              |
| US-15.11.5.5 | engine tester |          |              |

1. **US-15.11.5.1** — I want cache entries invalidated on tool version change so that stale compiled
   assets are never served.
2. **US-15.11.5.2** — I want configurable garbage collection schedules so that orphaned entries are
   cleaned up automatically.
3. **US-15.11.5.3** — I want LRU eviction when storage quota is exceeded so that the cache does not
   grow unboundedly.
4. **US-15.11.5.4** — I want manual garbage collection and invalidation logs so that I can audit and
   trigger cleanup when needed.
5. **US-15.11.5.5** — I want to verify tool version change invalidates all old entries so that
   invalidation correctness is regression-tested.

## US-15.11.6 Cache Transport and Storage

| ID           | Persona       | Features | Requirements |
|--------------|---------------|----------|--------------|
| US-15.11.6.1 | DevOps        |          |              |
| US-15.11.6.2 | DevOps        |          |              |
| US-15.11.6.3 | DevOps        |          |              |
| US-15.11.6.4 | engine dev    |          |              |
| US-15.11.6.5 | engine tester |          |              |

1. **US-15.11.6.1** — I want multiple storage backends (S3, GCS, Azure, local, HTTP) so that the
   cache fits our infrastructure.
2. **US-15.11.6.2** — I want Zstd compression for cache transfers so that network bandwidth is used
   efficiently.
3. **US-15.11.6.3** — I want configurable download concurrency and bandwidth limits so that cache
   traffic does not saturate the office network.
4. **US-15.11.6.4** — I want platform-native HTTP clients (NSURLSession, WinHTTP, libcurl) so that
   transfers use the most performant stack per platform.
5. **US-15.11.6.5** — I want to verify bandwidth limit is respected during downloads so that
   throttling is regression-tested.

## US-15.11.7 CI/CD Cache Population

| ID           | Persona       | Features | Requirements |
|--------------|---------------|----------|--------------|
| US-15.11.7.1 | DevOps        |          |              |
| US-15.11.7.2 | DevOps        |          |              |
| US-15.11.7.3 | DevOps        |          |              |
| US-15.11.7.4 | DevOps        |          |              |
| US-15.11.7.5 | engine tester |          |              |

1. **US-15.11.7.1** — I want CI builds to auto-populate the cache for all platforms so that
   developers have pre-built assets without building themselves.
2. **US-15.11.7.2** — I want idempotent uploads that skip existing entries so that duplicate uploads
   do not waste bandwidth.
3. **US-15.11.7.3** — I want nightly builds warming cache for all active branches so that feature
   branch developers benefit from the cache.
4. **US-15.11.7.4** — I want a standalone CLI for headless build servers so that cache population
   works outside the editor.
5. **US-15.11.7.5** — I want to verify duplicate upload is a no-op so that idempotent upload
   behavior is regression-tested.

## US-15.11.8 Cache Hit Metrics and Monitoring

| ID           | Persona       | Features | Requirements |
|--------------|---------------|----------|--------------|
| US-15.11.8.1 | DevOps        |          |              |
| US-15.11.8.2 | DevOps        |          |              |
| US-15.11.8.3 | DevOps        |          |              |
| US-15.11.8.4 | DevOps        |          |              |
| US-15.11.8.5 | engine tester |          |              |

1. **US-15.11.8.1** — I want a monitoring dashboard with hit rate and storage usage so that I can
   track cache effectiveness.
2. **US-15.11.8.2** — I want configurable alerts when hit rate drops below threshold so that cache
   misconfigurations are detected quickly.
3. **US-15.11.8.3** — I want per-developer and per-team usage breakdowns so that I can identify
   workflow bottlenecks.
4. **US-15.11.8.4** — I want metrics exported in OpenTelemetry format so that I can integrate with
   Grafana, Datadog, or other observability tools.
5. **US-15.11.8.5** — I want to verify the alert fires when hit rate drops below threshold so that
   alerting is regression-tested.
