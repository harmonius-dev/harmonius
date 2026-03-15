# User Stories: Shared Build Cache

## F-15.11.1 Centralized Compiled Asset Cache

## US-15.11.1.1 Developer Downloads Cached Assets
**As a** developer (P-15), **I want** compiled assets uploaded to a shared cache keyed by content
hash so other developers download cached results instead of rebuilding, **so that** each unique
asset version is built at most once across the entire organization.

## US-15.11.1.2 DevOps Manages Cache Service
**As a** DevOps engineer (P-16), **I want** the cache service to expose a REST API over HTTPS with
project-scoped API tokens or SSO, **so that** access is authenticated and scoped per project.

## US-15.11.1.3 Server Admin Monitors Cache Health
**As a** server admin (P-22), **I want** the cache service to report health status and storage
utilization, **so that** I can detect and resolve service issues before they affect developers.

## US-15.11.1.4 Engine Tester Validates Content Hashing
**As an** engine tester (P-27), **I want** to verify that content hashes incorporate source content,
build settings, and tool version, **so that** cache hits never serve stale or mismatched compiled
output.

## F-15.11.2 Shader Compilation Cache

## US-15.11.2.1 Developer Skips Local Shader Build
**As a** developer (P-15), **I want** compiled shader variants (SPIR-V, MSL, DXIL) cached by shader
source hash, target platform, and feature flags, **so that** a full shader rebuild completing in
hours locally completes in minutes from cache.

## US-15.11.2.2 DevOps Populates All Platform Variants
**As a** DevOps engineer (P-16), **I want** CI builds targeting all supported platforms to populate
the shader cache, **so that** developers always have pre-compiled shaders available for their
target.

## US-15.11.2.3 Tech Artist Iterates Faster on Shaders
**As a** tech artist (P-13), **I want** unchanged shader variants fetched from cache when I modify
only a subset of the shader graph, **so that** partial changes do not trigger a full recompilation.

## US-15.11.2.4 Engine Tester Validates Platform Independence
**As an** engine tester (P-27), **I want** to verify that each platform's shader variants are cached
independently, **so that** platform-specific compilation does not interfere.

## F-15.11.3 Logic Graph Compilation Cache

## US-15.11.3.1 Developer Fetches Cached Graph Bytecode
**As a** developer (P-15), **I want** compiled logic graph bytecode and AOT native code cached by
graph content hash and target platform, **so that** unchanged graphs are not recompiled on every
editor launch or branch switch.

## US-15.11.3.2 Designer Skips Graph Recompilation
**As a** designer (P-5), **I want** gameplay, animation, audio, and tool graphs fetched from cache
when unchanged, **so that** editor startup is fast even for projects with thousands of graphs.

## US-15.11.3.3 DevOps Caches Per-Platform AOT Code
**As a** DevOps engineer (P-16), **I want** AOT native code cache entries stored per platform
(x86-64, ARM64) and bytecode entries stored platform-agnostically, **so that** the cache structure
matches the compilation model.

## US-15.11.3.4 Engine Tester Validates Cache Invalidation
**As an** engine tester (P-27), **I want** to verify that modifying a graph invalidates its cache
entry and triggers recompilation, **so that** stale bytecode is never executed.

## F-15.11.4 New Developer Onboarding Acceleration

## US-15.11.4.1 Developer Starts Working in Minutes
**As a** developer (P-15), **I want** a fresh clone followed by the first editor launch to fetch all
compiled assets, shaders, and graph bytecode from the shared cache, **so that** I start working in
minutes instead of waiting hours for a full build.

## US-15.11.4.2 DevOps Monitors Onboarding Time
**As a** DevOps engineer (P-16), **I want** cache prefetch to run in parallel with Git LFS downloads
saturating available bandwidth with a progress dashboard, **so that** onboarding time is minimized.

## US-15.11.4.3 Designer Sees Download Progress
**As a** designer (P-5), **I want** a progress dashboard showing download status per asset category
with estimated time remaining, **so that** I know when the editor will be ready.

## US-15.11.4.4 Engine Tester Validates First-Launch Cache
**As an** engine tester (P-27), **I want** to verify that first-launch cache fetch retrieves all
required assets and the editor opens without errors, **so that** new developer onboarding works
end-to-end.

## F-15.11.5 Cache Invalidation and Garbage Collection

## US-15.11.5.1 Server Admin Configures Retention
**As a** server admin (P-22), **I want** cache entries invalidated when the build tool version
changes and garbage collected on a configurable schedule (default 30-day retention), **so that**
stale entries do not consume storage indefinitely.

## US-15.11.5.2 DevOps Sets Storage Quotas
**As a** DevOps engineer (P-16), **I want** storage quotas with least-recently-used eviction, **so
that** cache growth is bounded and disk costs are predictable.

## US-15.11.5.3 Server Admin Triggers Manual GC
**As a** server admin (P-22), **I want** to trigger manual garbage collection and view invalidation
logs, **so that** I can reclaim storage after large-scale tool version upgrades.

## US-15.11.5.4 Engine Tester Validates Invalidation Logic
**As an** engine tester (P-27), **I want** to verify that changing the build tool version
invalidates all affected cache entries, **so that** recompilation occurs when tools change.

## F-15.11.6 Cache Transport and Storage

## US-15.11.6.1 DevOps Chooses Storage Backend
**As a** DevOps engineer (P-16), **I want** the cache service to support local filesystem, S3, GCS,
Azure Blob Storage, and on-premise HTTP servers, **so that** I can choose the backend that matches
my infrastructure.

## US-15.11.6.2 Server Admin Configures Bandwidth Limits
**As a** server admin (P-22), **I want** parallel downloads with configurable concurrency and
bandwidth limits, **so that** cache downloads do not saturate the office network.

## US-15.11.6.3 Developer Gets Compressed Transfers
**As a** developer (P-15), **I want** content compressed with Zstd and transferred over HTTPS with
content-addressable deduplication, **so that** downloads are fast and efficient.

## US-15.11.6.4 Engine Tester Validates Platform Clients
**As an** engine tester (P-27), **I want** to verify that network transfers use platform-native HTTP
stacks (NSURLSession on macOS, WinHTTP on Windows, libcurl on Linux), **so that** transfers are
reliable on each platform.

## F-15.11.7 CI/CD Cache Population

## US-15.11.7.1 DevOps Populates Cache from CI
**As a** DevOps engineer (P-16), **I want** CI build pipelines to automatically populate the shared
cache for all target platforms as part of the build process, **so that** developers always have
pre-built assets available.

## US-15.11.7.2 Server Admin Schedules Nightly Warm-Up
**As a** server admin (P-22), **I want** nightly full builds to warm the cache for all active
branches including feature branches with recent activity, **so that** cache hit rates remain high.

## US-15.11.7.3 Engine Dev Ensures Idempotent Upload
**As an** engine developer (P-26), **I want** cache population to be idempotent (uploading an
existing key is a no-op), **so that** redundant CI builds do not waste bandwidth or storage.

## US-15.11.7.4 Engine Tester Validates CLI Tool
**As an** engine tester (P-27), **I want** to verify that the standalone CLI cache tool works on
headless build servers outside the editor, **so that** CI cache population does not require a GUI.

## F-15.11.8 Cache Hit Metrics and Monitoring

## US-15.11.8.1 DevOps Views Hit Rate Dashboard
**As a** DevOps engineer (P-16), **I want** a monitoring dashboard showing cache hit rate, miss
rate, storage usage, download bandwidth, and per-asset build time savings, **so that** I can
evaluate cache effectiveness.

## US-15.11.8.2 Server Admin Configures Alerts
**As a** server admin (P-22), **I want** alerts when the cache hit rate drops below a configurable
threshold, **so that** I can detect tool version changes or misconfigurations promptly.

## US-15.11.8.3 Developer Sees Per-Team Breakdown
**As a** developer (P-15), **I want** per-developer and per-team breakdowns of cache usage, **so
that** workflow bottlenecks are identified and addressed.

## US-15.11.8.4 Engine Tester Validates Metrics Export
**As an** engine tester (P-27), **I want** metrics exported in OpenTelemetry format for integration
with Grafana, Datadog, or other observability platforms, **so that** cache monitoring integrates
with existing infrastructure.
