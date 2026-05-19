# 15.11 — Shared Build Cache

## Compiled Asset Caching

| ID | Feature |
| ----------- | ---------------------------------- |
| F-15.11.1 | Centralized Compiled Asset Cache |
| F-15.11.2 | Shader Compilation Cache |
| F-15.11.3 | Logic Graph Compilation Cache |

1. **F-15.11.1** — A web service that stores compiled and processed assets keyed by content hash — a
   combination of source content, build settings, and tool version. When any developer builds an
   asset, the compiled result is uploaded to the shared cache. Other developers download the cached
   result instead of rebuilding locally, eliminating redundant builds across the entire
   organization. One build per unique asset version, regardless of how many developers need it.
   project-scoped API tokens or SSO integration.
   - **Deps:** F-12.5.1 (Asset Database), F-12.6.1 (Content Pipeline)
   - **Platform:** The cache service exposes a REST API over HTTPS. Clients authenticate via
2. **F-15.11.2** — Compiled shader variants (SPIR-V, SPIR-V, SPIR-V) are cached by a composite key
   of shader source hash, target platform, and feature permutation flags. A full shader rebuild that
   takes hours locally completes in minutes by downloading pre-compiled variants from the shared
   cache. The cache is populated by CI builds targeting all supported platforms, ensuring developers
   always have pre-compiled shaders available for their target. on macOS. Each platform's variants
   are cached independently.
   - **Deps:** F-15.11.1, F-15.8.5 (Shader Graphs), F-2.1.1 (GPU Abstraction)
   - **Platform:** Shader output format varies by platform: SPIR-V on Windows, SPIR-V on Linux,
     SPIR-V
3. **F-15.11.3** — Compiled logic graph bytecode and AOT native code are cached by graph content
   hash and target platform. When a graph is unchanged since the last build, the compiled output is
   fetched from the shared cache instead of recompiling. Avoids recompiling thousands of gameplay,
   animation, audio, and tool graphs on every editor launch or branch switch. cache entries are
   platform-agnostic.
   - **Deps:** F-15.11.1, F-15.8.12 (Graph Compilation)
   - **Platform:** AOT native code cache entries are platform-specific (x86-64, ARM64). Bytecode

## Onboarding

| ID | Feature |
| ----------- | --------------------------------------- |
| F-15.11.4 | New Developer Onboarding Acceleration |

1. **F-15.11.4** — A fresh repository clone followed by the first editor launch fetches all compiled
   assets, shaders, and graph bytecode from the shared cache instead of building from source.
   Reduces first-launch time from hours to minutes for large projects. Cache prefetch runs in
   parallel with Git LFS downloads, saturating available bandwidth. A progress dashboard shows
   download status per asset category with estimated time remaining.
   - **Deps:** F-15.11.1, F-15.11.2, F-15.11.3, F-15.10.2 (Git LFS)

## Cache Lifecycle

| ID | Feature |
| ----------- | ------------------------------------------- |
| F-15.11.5 | Cache Invalidation and Garbage Collection |

1. **F-15.11.5** — Cache entries are invalidated when the build tool version changes or the source
   content hash no longer matches any active branch. Garbage collection runs on a configurable
   schedule, removing entries not referenced by any branch head within a retention window (default
   30 days). Storage quotas prevent unbounded growth, with least-recently-used eviction when the
   quota is exceeded. Administrators can trigger manual garbage collection and view invalidation
   logs.
   - **Deps:** F-15.11.1

## Transport and Storage

| ID | Feature |
| ----------- | ----------------------------- |
| F-15.11.6 | Cache Transport and Storage |

1. **F-15.11.6** — The cache service supports multiple storage backends: local filesystem for small
   teams, S3, GCS, or Azure Blob Storage for cloud deployments, and on-premise HTTP servers for
   air-gapped environments. Content is compressed with Zstd and transferred over HTTPS with
   content-addressable deduplication. Parallel downloads use configurable concurrency and bandwidth
   limits to avoid saturating office network links. Linux, uses libcurl. All backends share a
   unified client interface.
   - **Deps:** F-15.11.1
   - **Platform:** On macOS, network transfers use NSURLSession. On Windows, uses WinHTTP. On

## CI/CD Integration

| ID | Feature |
| ----------- | ------------------------ |
| F-15.11.7 | CI/CD Cache Population |

1. **F-15.11.7** — CI build pipelines automatically populate the shared cache as part of their build
   process. Every CI build produces cache entries for all target platforms, ensuring developers
   always have pre-built assets available. Nightly full builds warm the cache for all active
   branches, including feature branches with recent activity. Cache population is idempotent —
   uploading an entry with an existing key is a no-op, avoiding redundant writes. editor, suitable
   for headless build servers.
   - **Deps:** F-15.11.1, F-15.11.2, F-15.11.3
   - **Platform:** CI integration is provided via a standalone CLI tool that runs outside the

## Monitoring

| ID | Feature |
| ----------- | ---------------------------------- |
| F-15.11.8 | Cache Hit Metrics and Monitoring |

1. **F-15.11.8** — A monitoring dashboard displays cache hit rate, miss rate, storage usage,
   download bandwidth, and per-asset build time savings. Alerts fire when the cache hit rate drops
   below a configurable threshold, indicating a tool version change or cache misconfiguration.
   Per-developer and per-team breakdowns identify workflow bottlenecks. Historical trends show cache
   effectiveness over time and correlate hit rate changes with tool version updates. Datadog, or
   other observability platforms.
   - **Deps:** F-15.11.1, F-15.11.5
   - **Platform:** Metrics are exported in OpenTelemetry format for integration with Grafana,
