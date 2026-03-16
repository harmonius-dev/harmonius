# User Stories: Shared Build Cache

## F-15.11.1 Centralized Compiled Asset Cache

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.11.1.1 | developer (P-15) | compiled assets uploaded to a shared cache keyed by content hash so other developers download cached results instead of rebuilding | each unique asset version is built at most once across the entire organization |  |  |
| US-15.11.1.2 | DevOps engineer (P-16) | the cache service to expose a REST API over HTTPS with project-scoped API tokens or SSO | access is authenticated and scoped per project |  |  |
| US-15.11.1.3 | server admin (P-22) | the cache service to report health status and storage utilization | I can detect and resolve service issues before they affect developers |  |  |
| US-15.11.1.4 | engine tester (P-27) | to verify that content hashes incorporate source content, build settings, and tool version | cache hits never serve stale or mismatched compiled output |  |  |

## F-15.11.2 Shader Compilation Cache

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.11.2.1 | developer (P-15) | compiled shader variants (SPIR-V, MSL, DXIL) cached by shader source hash, target platform, and feature flags | a full shader rebuild completing in hours locally completes in minutes from cache |  |  |
| US-15.11.2.2 | DevOps engineer (P-16) | CI builds targeting all supported platforms to populate the shader cache | developers always have pre-compiled shaders available for their target |  |  |
| US-15.11.2.3 | tech artist (P-13) | unchanged shader variants fetched from cache when I modify only a subset of the shader graph | partial changes do not trigger a full recompilation |  |  |
| US-15.11.2.4 | engine tester (P-27) | to verify that each platform's shader variants are cached independently | platform-specific compilation does not interfere |  |  |

## F-15.11.3 Logic Graph Compilation Cache

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.11.3.1 | developer (P-15) | compiled logic graph bytecode and AOT native code cached by graph content hash and target platform | unchanged graphs are not recompiled on every editor launch or branch switch |  |  |
| US-15.11.3.2 | designer (P-5) | gameplay, animation, audio, and tool graphs fetched from cache when unchanged | editor startup is fast even for projects with thousands of graphs |  |  |
| US-15.11.3.3 | DevOps engineer (P-16) | AOT native code cache entries stored per platform (x86-64, ARM64) and bytecode entries stored platform-agnostically | the cache structure matches the compilation model |  |  |
| US-15.11.3.4 | engine tester (P-27) | to verify that modifying a graph invalidates its cache entry and triggers recompilation | stale bytecode is never executed |  |  |

## F-15.11.4 New Developer Onboarding Acceleration

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.11.4.1 | developer (P-15) | a fresh clone followed by the first editor launch to fetch all compiled assets, shaders, and graph bytecode from the shared cache | I start working in minutes instead of waiting hours for a full build |  |  |
| US-15.11.4.2 | DevOps engineer (P-16) | cache prefetch to run in parallel with Git LFS downloads saturating available bandwidth with a progress dashboard | onboarding time is minimized |  |  |
| US-15.11.4.3 | designer (P-5) | a progress dashboard showing download status per asset category with estimated time remaining | I know when the editor will be ready |  |  |
| US-15.11.4.4 | engine tester (P-27) | to verify that first-launch cache fetch retrieves all required assets and the editor opens without errors | new developer onboarding works end-to-end |  |  |

## F-15.11.5 Cache Invalidation and Garbage Collection

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.11.5.1 | server admin (P-22) | cache entries invalidated when the build tool version changes and garbage collected on a configurable schedule (default 30-day retention) | stale entries do not consume storage indefinitely |  |  |
| US-15.11.5.2 | DevOps engineer (P-16) | storage quotas with least-recently-used eviction | cache growth is bounded and disk costs are predictable |  |  |
| US-15.11.5.3 | server admin (P-22) | to trigger manual garbage collection and view invalidation logs | I can reclaim storage after large-scale tool version upgrades |  |  |
| US-15.11.5.4 | engine tester (P-27) | to verify that changing the build tool version invalidates all affected cache entries | recompilation occurs when tools change |  |  |

## F-15.11.6 Cache Transport and Storage

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.11.6.1 | DevOps engineer (P-16) | the cache service to support local filesystem, S3, GCS, Azure Blob Storage, and on-premise HTTP servers | I can choose the backend that matches my infrastructure |  |  |
| US-15.11.6.2 | server admin (P-22) | parallel downloads with configurable concurrency and bandwidth limits | cache downloads do not saturate the office network |  |  |
| US-15.11.6.3 | developer (P-15) | content compressed with Zstd and transferred over HTTPS with content-addressable deduplication | downloads are fast and efficient |  |  |
| US-15.11.6.4 | engine tester (P-27) | to verify that network transfers use platform-native HTTP stacks (NSURLSession on macOS, WinHTTP on Windows, libcurl on Linux) | transfers are reliable on each platform |  |  |

## F-15.11.7 CI/CD Cache Population

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.11.7.1 | DevOps engineer (P-16) | CI build pipelines to automatically populate the shared cache for all target platforms as part of the build process | developers always have pre-built assets available |  |  |
| US-15.11.7.2 | server admin (P-22) | nightly full builds to warm the cache for all active branches including feature branches with recent activity | cache hit rates remain high |  |  |
| US-15.11.7.3 | engine developer (P-26) | cache population to be idempotent (uploading an existing key is a no-op) | redundant CI builds do not waste bandwidth or storage |  |  |
| US-15.11.7.4 | engine tester (P-27) | to verify that the standalone CLI cache tool works on headless build servers outside the editor | CI cache population does not require a GUI |  |  |

## F-15.11.8 Cache Hit Metrics and Monitoring

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.11.8.1 | DevOps engineer (P-16) | a monitoring dashboard showing cache hit rate, miss rate, storage usage, download bandwidth, and per-asset build time savings | I can evaluate cache effectiveness |  |  |
| US-15.11.8.2 | server admin (P-22) | alerts when the cache hit rate drops below a configurable threshold | I can detect tool version changes or misconfigurations promptly |  |  |
| US-15.11.8.3 | developer (P-15) | per-developer and per-team breakdowns of cache usage | workflow bottlenecks are identified and addressed |  |  |
| US-15.11.8.4 | engine tester (P-27) | metrics exported in OpenTelemetry format for integration with Grafana, Datadog, or other observability platforms | cache monitoring integrates with existing infrastructure |  |  |
