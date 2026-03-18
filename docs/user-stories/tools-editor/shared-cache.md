# User Stories: Shared Build Cache

## F-15.11.1 Centralized Compiled Asset Cache

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.11.1.1 | developer (P-15)       |          |              |
| US-15.11.1.2 | DevOps engineer (P-16) |          |              |
| US-15.11.1.3 | server admin (P-22)    |          |              |
| US-15.11.1.4 | engine tester (P-27)   |          |              |

1. **US-15.11.1.1** — compiled assets uploaded to a shared cache keyed by content hash so other
   developers download cached results instead of rebuilding
   - **Acceptance:** each unique asset version is built at most once across the entire organization
2. **US-15.11.1.2** — the cache service to expose a REST API over HTTPS with project-scoped API
   tokens or SSO
   - **Acceptance:** access is authenticated and scoped per project
3. **US-15.11.1.3** — the cache service to report health status and storage utilization
   - **Acceptance:** I can detect and resolve service issues before they affect developers
4. **US-15.11.1.4** — to verify that content hashes incorporate source content, build settings, and
   tool version
   - **Acceptance:** cache hits never serve stale or mismatched compiled output

## F-15.11.2 Shader Compilation Cache

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.11.2.1 | developer (P-15)       |          |              |
| US-15.11.2.2 | DevOps engineer (P-16) |          |              |
| US-15.11.2.3 | tech artist (P-13)     |          |              |
| US-15.11.2.4 | engine tester (P-27)   |          |              |

1. **US-15.11.2.1** — compiled shader variants (SPIR-V, MSL, DXIL) cached by shader source hash,
   target platform, and feature flags
   - **Acceptance:** a full shader rebuild completing in hours locally completes in minutes from
     cache
2. **US-15.11.2.2** — CI builds targeting all supported platforms to populate the shader cache
   - **Acceptance:** developers always have pre-compiled shaders available for their target
3. **US-15.11.2.3** — unchanged shader variants fetched from cache when I modify only a subset of
   the shader graph
   - **Acceptance:** partial changes do not trigger a full recompilation
4. **US-15.11.2.4** — to verify that each platform's shader variants are cached independently
   - **Acceptance:** platform-specific compilation does not interfere

## F-15.11.3 Logic Graph Compilation Cache

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.11.3.1 | developer (P-15)       |          |              |
| US-15.11.3.2 | designer (P-5)         |          |              |
| US-15.11.3.3 | DevOps engineer (P-16) |          |              |
| US-15.11.3.4 | engine tester (P-27)   |          |              |

1. **US-15.11.3.1** — compiled logic graph bytecode and AOT native code cached by graph content hash
   and target platform
   - **Acceptance:** unchanged graphs are not recompiled on every editor launch or branch switch
2. **US-15.11.3.2** — gameplay, animation, audio, and tool graphs fetched from cache when unchanged
   - **Acceptance:** editor startup is fast even for projects with thousands of graphs
3. **US-15.11.3.3** — AOT native code cache entries stored per platform (x86-64, ARM64) and bytecode
   entries stored platform-agnostically
   - **Acceptance:** the cache structure matches the compilation model
4. **US-15.11.3.4** — to verify that modifying a graph invalidates its cache entry and triggers
   recompilation
   - **Acceptance:** stale bytecode is never executed

## F-15.11.4 New Developer Onboarding Acceleration

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.11.4.1 | developer (P-15)       |          |              |
| US-15.11.4.2 | DevOps engineer (P-16) |          |              |
| US-15.11.4.3 | designer (P-5)         |          |              |
| US-15.11.4.4 | engine tester (P-27)   |          |              |

1. **US-15.11.4.1** — a fresh clone followed by the first editor launch to fetch all compiled
   assets, shaders, and graph bytecode from the shared cache
   - **Acceptance:** I start working in minutes instead of waiting hours for a full build
2. **US-15.11.4.2** — cache prefetch to run in parallel with Git LFS downloads saturating available
   bandwidth with a progress dashboard
   - **Acceptance:** onboarding time is minimized
3. **US-15.11.4.3** — a progress dashboard showing download status per asset category with estimated
   time remaining
   - **Acceptance:** I know when the editor will be ready
4. **US-15.11.4.4** — to verify that first-launch cache fetch retrieves all required assets and the
   editor opens without errors
   - **Acceptance:** new developer onboarding works end-to-end

## F-15.11.5 Cache Invalidation and Garbage Collection

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.11.5.1 | server admin (P-22)    |          |              |
| US-15.11.5.2 | DevOps engineer (P-16) |          |              |
| US-15.11.5.3 | server admin (P-22)    |          |              |
| US-15.11.5.4 | engine tester (P-27)   |          |              |

1. **US-15.11.5.1** — cache entries invalidated when the build tool version changes and garbage
   collected on a configurable schedule (default 30-day retention)
   - **Acceptance:** stale entries do not consume storage indefinitely
2. **US-15.11.5.2** — storage quotas with least-recently-used eviction
   - **Acceptance:** cache growth is bounded and disk costs are predictable
3. **US-15.11.5.3** — to trigger manual garbage collection and view invalidation logs
   - **Acceptance:** I can reclaim storage after large-scale tool version upgrades
4. **US-15.11.5.4** — to verify that changing the build tool version invalidates all affected cache
   entries
   - **Acceptance:** recompilation occurs when tools change

## F-15.11.6 Cache Transport and Storage

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.11.6.1 | DevOps engineer (P-16) |          |              |
| US-15.11.6.2 | server admin (P-22)    |          |              |
| US-15.11.6.3 | developer (P-15)       |          |              |
| US-15.11.6.4 | engine tester (P-27)   |          |              |

1. **US-15.11.6.1** — the cache service to support local filesystem, S3, GCS, Azure Blob Storage,
   and on-premise HTTP servers
   - **Acceptance:** I can choose the backend that matches my infrastructure
2. **US-15.11.6.2** — parallel downloads with configurable concurrency and bandwidth limits
   - **Acceptance:** cache downloads do not saturate the office network
3. **US-15.11.6.3** — content compressed with Zstd and transferred over HTTPS with
   content-addressable deduplication
   - **Acceptance:** downloads are fast and efficient
4. **US-15.11.6.4** — to verify that network transfers use platform-native HTTP stacks (NSURLSession
   on macOS, WinHTTP on Windows, libcurl on Linux)
   - **Acceptance:** transfers are reliable on each platform

## F-15.11.7 CI/CD Cache Population

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.11.7.1 | DevOps engineer (P-16)  |          |              |
| US-15.11.7.2 | server admin (P-22)     |          |              |
| US-15.11.7.3 | engine developer (P-26) |          |              |
| US-15.11.7.4 | engine tester (P-27)    |          |              |

1. **US-15.11.7.1** — CI build pipelines to automatically populate the shared cache for all target
   platforms as part of the build process
   - **Acceptance:** developers always have pre-built assets available
2. **US-15.11.7.2** — nightly full builds to warm the cache for all active branches including
   feature branches with recent activity
   - **Acceptance:** cache hit rates remain high
3. **US-15.11.7.3** — cache population to be idempotent (uploading an existing key is a no-op)
   - **Acceptance:** redundant CI builds do not waste bandwidth or storage
4. **US-15.11.7.4** — to verify that the standalone CLI cache tool works on headless build servers
   outside the editor
   - **Acceptance:** CI cache population does not require a GUI

## F-15.11.8 Cache Hit Metrics and Monitoring

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.11.8.1 | DevOps engineer (P-16) |          |              |
| US-15.11.8.2 | server admin (P-22)    |          |              |
| US-15.11.8.3 | developer (P-15)       |          |              |
| US-15.11.8.4 | engine tester (P-27)   |          |              |

1. **US-15.11.8.1** — a monitoring dashboard showing cache hit rate, miss rate, storage usage,
   download bandwidth, and per-asset build time savings
   - **Acceptance:** I can evaluate cache effectiveness
2. **US-15.11.8.2** — alerts when the cache hit rate drops below a configurable threshold
   - **Acceptance:** I can detect tool version changes or misconfigurations promptly
3. **US-15.11.8.3** — per-developer and per-team breakdowns of cache usage
   - **Acceptance:** workflow bottlenecks are identified and addressed
4. **US-15.11.8.4** — metrics exported in OpenTelemetry format for integration with Grafana,
   Datadog, or other observability platforms
   - **Acceptance:** cache monitoring integrates with existing infrastructure
