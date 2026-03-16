# R-15.11 -- Shared Build Cache Requirements

## Compiled Asset Caching

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.11.1 | The engine **SHALL** provide a shared cache service storing compiled assets keyed by content hash (source content, build settings, tool version), with REST API over HTTPS and token authentication, so that identical builds are never repeated across the team. [F-15.11.1](../../features/tools-editor/shared-cache.md) duplicate work. and verify identical output without rebuild. |  | Redundant builds waste developer time; content-addressed caching eliminates | Unit test: build an asset, upload to cache, fetch from cache on another machine, |
| R-15.11.2 | Compiled shader variants **SHALL** be cached per platform keyed by shader source hash and feature permutation flags, with CI builds populating the cache for all platforms. [F-15.11.2](../../features/tools-editor/shared-cache.md) |  | Full shader rebuilds take hours; caching reduces this to download time. | Unit test: verify cached shader output matches CI-compiled output byte-for-byte. |
| R-15.11.3 | Compiled graph bytecode and AOT native code **SHALL** be cached by content hash and target platform so that unchanged graphs are not recompiled on every launch. [F-15.11.3](../../features/tools-editor/shared-cache.md) |  | Thousands of graphs must not recompile on every editor launch or branch switch. | Unit test: verify an unchanged graph fetches from cache instead of recompiling. |

## Onboarding

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.11.4 | First editor launch **SHALL** fetch all compiled assets from the shared cache in parallel with Git LFS downloads, with a progress dashboard showing download status and ETAs, reducing first-launch time from hours to under 10 minutes for projects that take 1+ hours to build from source. [F-15.11.4](../../features/tools-editor/shared-cache.md) hours from source. |  | Multi-hour first builds block new developer productivity. | Benchmark: verify first-launch time is under 10 minutes for a project taking 1+ |

## Cache Lifecycle

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.11.5 | Cache entries **SHALL** be invalidated on tool version change, with configurable garbage collection schedules, LRU eviction when storage quota is exceeded, and manual garbage collection with invalidation logs. [F-15.11.5](../../features/tools-editor/shared-cache.md) |  | Stale cache entries cause correctness issues; unbounded growth wastes storage. | Unit test: change the tool version and verify all old entries are invalidated. |

## Transport and Storage

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.11.6 | The cache **SHALL** support multiple storage backends (S3, GCS, Azure, local, HTTP), Zstd compression for transfers, configurable download concurrency and bandwidth limits, and platform-native HTTP clients (NSURLSession, WinHTTP, libcurl). [F-15.11.6](../../features/tools-editor/shared-cache.md) network saturation. |  | Infrastructure diversity requires pluggable backends; bandwidth limits prevent | Unit test: verify bandwidth limit is respected during downloads. |

## CI/CD Population

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.11.7 | CI builds **SHALL** auto-populate the cache for all platforms with idempotent uploads that skip existing entries, nightly builds warming cache for active branches, and a standalone CLI for headless build servers. [F-15.11.7](../../features/tools-editor/shared-cache.md) |  | CI-populated caches ensure developers always have pre-built assets available. | Unit test: upload the same entry twice and verify the second upload is a no-op. |

## Monitoring

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.11.8 | The cache **SHALL** provide a monitoring dashboard with hit rate and storage usage, configurable alerts when hit rate drops below threshold, per-developer and per-team usage breakdowns, and metrics exported in OpenTelemetry format. [F-15.11.8](../../features/tools-editor/shared-cache.md) |  | Cache effectiveness monitoring catches misconfigurations and tracks ROI. | Unit test: drop hit rate below threshold and verify the alert fires. |

---

## US-15.11.1 Centralized Compiled Asset Cache

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.11.1.1 | developer | I want compiled assets cached by content hash so that identical builds are never repeated across the team. |  |  |
| US-15.11.1.2 | developer | I want to download cached results instead of rebuilding so that my build times are reduced to cache fetch times. |  |  |
| US-15.11.1.3 | DevOps | I want the cache keyed by source content, build settings, and tool version so that cache entries are invalidated correctly on toolchain changes. |  |  |
| US-15.11.1.4 | DevOps | I want REST API over HTTPS with token authentication so that cache access is secure and auditable. |  |  |
| US-15.11.1.5 | engine tester | I want to verify a cache hit returns the identical compiled asset without rebuild so that cache correctness is regression-tested. |  |  |

## US-15.11.2 Shader Compilation Cache

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.11.2.1 | developer | I want shader variants cached per platform so that I download pre-compiled shaders instead of rebuilding. |  |  |
| US-15.11.2.2 | developer | I want cache keyed by shader source hash and feature permutation flags so that variant-level caching is precise. |  |  |
| US-15.11.2.3 | DevOps | I want CI builds to populate shader cache for all platforms so that developers always have pre-compiled shaders available. |  |  |
| US-15.11.2.4 | engine tester | I want to verify cached shader output matches CI-compiled output byte-for-byte so that shader cache integrity is regression-tested. |  |  |

## US-15.11.3 Logic Graph Compilation Cache

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.11.3.1 | developer | I want compiled graph bytecode cached by content hash so that unchanged graphs are not recompiled on every launch. |  |  |
| US-15.11.3.2 | developer | I want AOT native code cached per target platform so that platform-specific compiled graphs are reused. |  |  |
| US-15.11.3.3 | engine tester | I want to verify unchanged graphs fetch from cache instead of recompiling so that graph cache hit behavior is regression-tested. |  |  |

## US-15.11.4 New Developer Onboarding Acceleration

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.11.4.1 | developer | I want first editor launch to fetch all compiled assets from cache so that my first build takes minutes instead of hours. |  |  |
| US-15.11.4.2 | developer | I want cache prefetch running in parallel with Git LFS downloads so that both operations saturate available bandwidth. |  |  |
| US-15.11.4.3 | developer | I want a progress dashboard showing download status so that I can monitor onboarding progress with ETAs. |  |  |
| US-15.11.4.4 | engine tester | I want to verify first-launch time is under 10 minutes for a project taking 1+ hours to build from source so that onboarding acceleration is regression-tested. |  |  |

## US-15.11.5 Cache Invalidation and Garbage Collection

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.11.5.1 | DevOps | I want cache entries invalidated on tool version change so that stale compiled assets are never served. |  |  |
| US-15.11.5.2 | DevOps | I want configurable garbage collection schedules so that orphaned entries are cleaned up automatically. |  |  |
| US-15.11.5.3 | DevOps | I want LRU eviction when storage quota is exceeded so that the cache does not grow unboundedly. |  |  |
| US-15.11.5.4 | server admin | I want manual garbage collection and invalidation logs so that I can audit and trigger cleanup when needed. |  |  |
| US-15.11.5.5 | engine tester | I want to verify tool version change invalidates all old entries so that invalidation correctness is regression-tested. |  |  |

## US-15.11.6 Cache Transport and Storage

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.11.6.1 | DevOps | I want multiple storage backends (S3, GCS, Azure, local, HTTP) so that the cache fits our infrastructure. |  |  |
| US-15.11.6.2 | DevOps | I want Zstd compression for cache transfers so that network bandwidth is used efficiently. |  |  |
| US-15.11.6.3 | DevOps | I want configurable download concurrency and bandwidth limits so that cache traffic does not saturate the office network. |  |  |
| US-15.11.6.4 | engine dev | I want platform-native HTTP clients (NSURLSession, WinHTTP, libcurl) so that transfers use the most performant stack per platform. |  |  |
| US-15.11.6.5 | engine tester | I want to verify bandwidth limit is respected during downloads so that throttling is regression-tested. |  |  |

## US-15.11.7 CI/CD Cache Population

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.11.7.1 | DevOps | I want CI builds to auto-populate the cache for all platforms so that developers have pre-built assets without building themselves. |  |  |
| US-15.11.7.2 | DevOps | I want idempotent uploads that skip existing entries so that duplicate uploads do not waste bandwidth. |  |  |
| US-15.11.7.3 | DevOps | I want nightly builds warming cache for all active branches so that feature branch developers benefit from the cache. |  |  |
| US-15.11.7.4 | DevOps | I want a standalone CLI for headless build servers so that cache population works outside the editor. |  |  |
| US-15.11.7.5 | engine tester | I want to verify duplicate upload is a no-op so that idempotent upload behavior is regression-tested. |  |  |

## US-15.11.8 Cache Hit Metrics and Monitoring

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.11.8.1 | DevOps | I want a monitoring dashboard with hit rate and storage usage so that I can track cache effectiveness. |  |  |
| US-15.11.8.2 | DevOps | I want configurable alerts when hit rate drops below threshold so that cache misconfigurations are detected quickly. |  |  |
| US-15.11.8.3 | DevOps | I want per-developer and per-team usage breakdowns so that I can identify workflow bottlenecks. |  |  |
| US-15.11.8.4 | DevOps | I want metrics exported in OpenTelemetry format so that I can integrate with Grafana, Datadog, or other observability tools. |  |  |
| US-15.11.8.5 | engine tester | I want to verify the alert fires when hit rate drops below threshold so that alerting is regression-tested. |  |  |
