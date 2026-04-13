# R-15.11 -- Shared Build Cache Requirements

## Requirements

1. **R-15.11.1** — The engine **SHALL** provide a centralized cache service storing compiled assets
   keyed by content hash, build settings, and tool version, exposed via REST over HTTPS.
   - **Rationale:** Shared caching eliminates redundant builds across all team members.
   - **Verification:** Build an asset, verify it is uploaded, clear local cache, rebuild, and verify
     the cache hit.

2. **R-15.11.2** — The engine **SHALL** cache shader variants by source hash, platform, and
   permutation flags, populated by CI for all supported platforms.
   - **Rationale:** Shader compilation is the slowest build step; caching reduces it from hours to
     minutes.
   - **Verification:** Compile shaders, verify cache entries exist for all target platforms.

3. **R-15.11.3** — The engine **SHALL** cache compiled logic graph bytecode and AOT native code by
   graph content hash and target platform.
   - **Rationale:** Graph recompilation on every launch wastes time for unchanged graphs.
   - **Verification:** Modify a graph, rebuild, verify only the modified graph recompiles.

4. **R-15.11.4** — The engine **SHALL** prefetch all compiled assets from the cache on first launch
   after clone, running in parallel with Git LFS downloads.
   - **Rationale:** Fast onboarding is critical for team productivity.
   - **Verification:** Clone a project, launch the editor, and verify all assets are fetched from
     cache without local compilation.

5. **R-15.11.5** — The engine **SHALL** invalidate cache entries on tool version changes and perform
   LRU garbage collection within configurable storage quotas.
   - **Rationale:** Stale cache entries cause build errors; unbounded growth wastes storage.
   - **Verification:** Change a tool version and verify previously cached entries are invalidated.

6. **R-15.11.6** — The engine **SHALL** support local filesystem, S3, GCS, and Azure Blob storage
   backends with Zstd compression and content-addressable deduplication.
   - **Rationale:** Flexible backends serve teams from solo developers to large studios.
   - **Verification:** Configure S3 backend, upload and download a cached asset, and verify
     integrity.

7. **R-15.11.7** — The engine **SHALL** support idempotent CI cache population for all target
   platforms with nightly full builds warming all active branches.
   - **Rationale:** CI-populated caches ensure developers always have pre-built assets.
   - **Verification:** Run a CI build, verify cache entries are created; run again and verify no
     redundant uploads.

8. **R-15.11.8** — The engine **SHALL** export cache metrics in OpenTelemetry format with a
   monitoring dashboard and configurable hit-rate alerts.
   - **Rationale:** Monitoring ensures cache effectiveness and detects configuration issues.
   - **Verification:** Verify metrics appear in a Grafana dashboard and a hit-rate alert fires when
     artificially lowered.
