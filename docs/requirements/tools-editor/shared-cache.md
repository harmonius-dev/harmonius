# R-15.11 -- Shared Build Cache Requirements

## R-15.11.1 Centralized Compiled Asset Cache

The engine **SHALL** provide a web service that stores compiled assets keyed by content hash (source
content, build settings, tool version) and serves cached results to other developers, ensuring each
unique asset version is built at most once across the organization.

- **Derived from:** [F-15.11.1](../../features/tools-editor/shared-cache.md)
- **Rationale:** Eliminating redundant builds across developers saves hours of daily compute time
  and ensures all team members work with identical compiled assets for a given source version.
- **Verification:** Build an asset on developer A's machine and verify it is uploaded to the cache.
  Trigger the same build on developer B's machine and verify the cached result is downloaded
  instead of rebuilt. Modify the source and verify the new build produces a distinct cache entry.
  Verify authentication via project-scoped API tokens and SSO integration.

## R-15.11.2 Shader Compilation Cache

The engine **SHALL** cache compiled shader variants (SPIR-V, MSL, DXIL) keyed by shader source
hash, target platform, and feature permutation flags, enabling developers to download pre-compiled
variants from the shared cache instead of rebuilding locally.

- **Derived from:** [F-15.11.2](../../features/tools-editor/shared-cache.md)
- **Rationale:** Shader compilation across all permutations is one of the most time-consuming
  build steps; caching eliminates redundant compilation and accelerates iteration.
- **Verification:** Compile shaders for all target platforms via CI and verify cache entries are
  created per platform (DXIL, SPIR-V, MSL). Fetch a shader variant on a developer machine and
  verify it matches the CI-compiled output byte-for-byte. Modify the shader source and verify
  the old cache entry is not served for the new hash.

## R-15.11.3 Logic Graph Compilation Cache

The engine **SHALL** cache compiled logic graph bytecode and AOT native code keyed by graph content
hash and target platform, serving cached outputs instead of recompiling unchanged graphs.

- **Derived from:** [F-15.11.3](../../features/tools-editor/shared-cache.md)
- **Rationale:** Large projects contain thousands of logic graphs whose recompilation on every
  editor launch or branch switch wastes significant developer time.
- **Verification:** Compile a logic graph and verify the bytecode and AOT native code are uploaded
  to the cache. Launch the editor on a clean workspace and verify the cached output is
  downloaded. Verify platform-specific AOT entries (x86-64, ARM64) are cached independently.
  Verify platform-agnostic bytecode entries are shared across platforms.

## R-15.11.4 New Developer Onboarding Acceleration

The engine **SHALL** fetch all compiled assets, shaders, and graph bytecode from the shared cache on
first editor launch after a fresh clone, running cache prefetch in parallel with Git LFS downloads,
with a progress dashboard showing per-category status and estimated time remaining.

- **Derived from:** [F-15.11.4](../../features/tools-editor/shared-cache.md)
- **Rationale:** Reducing first-launch time from hours to minutes removes a major friction point
  in developer onboarding and branch switching.
- **Verification:** Perform a fresh repository clone and launch the editor. Verify compiled assets
  are fetched from the cache instead of built from source. Verify cache prefetch runs in parallel
  with LFS downloads. Measure first-launch time and verify it is under 10 minutes for a project
  that takes over 1 hour to build from source. Verify the progress dashboard displays accurate
  per-category download status.

## R-15.11.5 Cache Invalidation and Garbage Collection

The engine **SHALL** invalidate cache entries when the build tool version changes or the source
content hash is no longer referenced by any active branch, with configurable garbage collection
schedules, a retention window (default 30 days), and LRU eviction when storage quotas are exceeded.

- **Derived from:** [F-15.11.5](../../features/tools-editor/shared-cache.md)
- **Rationale:** Automatic invalidation and garbage collection prevent unbounded storage growth
  and ensure developers never receive stale compiled assets.
- **Verification:** Change the build tool version and verify all cache entries compiled with the
  old version are invalidated. Delete all branches referencing a source hash and run garbage
  collection after the retention window. Verify the orphaned entry is removed. Exceed the storage
  quota and verify the least-recently-used entry is evicted. Verify administrators can trigger
  manual garbage collection and view invalidation logs.

## R-15.11.6 Cache Transport and Storage

The engine **SHALL** support multiple storage backends (local filesystem, S3, GCS, Azure Blob
Storage, on-premise HTTP) with Zstd-compressed content-addressable transfers over HTTPS and
configurable parallel download concurrency and bandwidth limits.

- **Derived from:** [F-15.11.6](../../features/tools-editor/shared-cache.md)
- **Rationale:** Flexible backend support accommodates teams ranging from small local setups to
  large cloud deployments and air-gapped environments.
- **Verification:** Configure the cache service with S3 and local filesystem backends. Upload and
  download an asset via each backend and verify content integrity. Verify content is Zstd-
  compressed in transit. Set a bandwidth limit and verify download throughput does not exceed the
  configured limit. Verify platform-native HTTP clients are used (NSURLSession, WinHTTP,
  libcurl).

## R-15.11.7 CI/CD Cache Population

The engine **SHALL** automatically populate the shared cache from CI build pipelines for all target
platforms, with idempotent uploads that skip entries with existing keys, and nightly full builds
that warm the cache for all active branches including recent feature branches.

- **Derived from:** [F-15.11.7](../../features/tools-editor/shared-cache.md)
- **Rationale:** CI-driven cache population ensures developers always have pre-built assets
  available without relying on other developers to build first.
- **Verification:** Run a CI build and verify cache entries are created for all target platforms.
  Run the same CI build again and verify no redundant uploads occur (idempotent). Verify the
  standalone CLI tool operates correctly outside the editor on a headless build server. Verify
  nightly builds produce entries for all active feature branches.

## Non-Functional Requirements

### R-15.11.NF1 Cache Service Availability and Performance

The shared build cache service **SHALL** maintain 99.9% availability (less than 8.8 hours downtime
per year). Cache lookups (hit or miss) **SHALL** complete within 200ms at the 99th percentile.
Cache uploads **SHALL** complete within 5 seconds for assets up to 100 MB over a 1 Gbps connection.
The service **SHALL** support at least 100 concurrent developer connections without degradation.
When the cache service is unavailable, the editor **SHALL** fall back to local builds without
blocking or error dialogs.

- **Derived from:** F-15.11.1 through F-15.11.8 (all shared cache features).
- **Rationale:** The shared cache is on the critical path of every developer's build workflow.
  Slow lookups or downtime negate the time savings the cache provides.
- **Verification:** Load test the cache service with 100 concurrent clients performing lookups and
  uploads. Assert 99th percentile lookup latency is under 200ms. Upload a 100 MB asset over 1 Gbps
  and assert completion within 5 seconds. Simulate cache service downtime and verify the editor
  builds locally without errors. Measure uptime over a 30-day period and assert 99.9% availability.

## R-15.11.8 Cache Hit Metrics and Monitoring

The engine **SHALL** export cache hit rate, miss rate, storage usage, download bandwidth, and per-
asset build time savings to a monitoring dashboard, with configurable alerts when the hit rate drops
below a threshold, and per-developer and per-team breakdowns with historical trends.

- **Derived from:** [F-15.11.8](../../features/tools-editor/shared-cache.md)
- **Rationale:** Monitoring cache effectiveness identifies workflow bottlenecks, tool version
  regressions, and misconfigurations that degrade build times.
- **Verification:** Build assets with and without cache hits and verify the dashboard reports
  accurate hit and miss rates. Configure a hit rate alert threshold, drop the hit rate below it,
  and verify the alert fires. Verify per-developer breakdowns show distinct usage patterns.
  Verify metrics are exported in OpenTelemetry format.
