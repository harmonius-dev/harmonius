# R-15.11 -- Shared Build Cache User Stories

## US-15.11.1 Centralized Compiled Asset Cache

### US-15.11.1.1
As a **developer (P-15)**, I want compiled assets cached by content hash so that identical builds
are never repeated across the team.

### US-15.11.1.2
As a **developer (P-15)**, I want to download cached results instead of rebuilding so that my build
times are reduced to cache fetch times.

### US-15.11.1.3
As a **DevOps (P-16)**, I want the cache keyed by source content, build settings, and tool version
so that cache entries are invalidated correctly on toolchain changes.

### US-15.11.1.4
As a **DevOps (P-16)**, I want REST API over HTTPS with token authentication so that cache access is
secure and auditable.

### US-15.11.1.5
As an **engine tester (P-27)**, I want to verify a cache hit returns the identical compiled asset
without rebuild so that cache correctness is regression-tested.

---

## US-15.11.2 Shader Compilation Cache

### US-15.11.2.1
As a **developer (P-15)**, I want shader variants cached per platform so that I download
pre-compiled shaders instead of rebuilding.

### US-15.11.2.2
As a **developer (P-15)**, I want cache keyed by shader source hash and feature permutation flags so
that variant-level caching is precise.

### US-15.11.2.3
As a **DevOps (P-16)**, I want CI builds to populate shader cache for all platforms so that
developers always have pre-compiled shaders available.

### US-15.11.2.4
As an **engine tester (P-27)**, I want to verify cached shader output matches CI-compiled output
byte-for-byte so that shader cache integrity is regression-tested.

---

## US-15.11.3 Logic Graph Compilation Cache

### US-15.11.3.1
As a **developer (P-15)**, I want compiled graph bytecode cached by content hash so that unchanged
graphs are not recompiled on every launch.

### US-15.11.3.2
As a **developer (P-15)**, I want AOT native code cached per target platform so that
platform-specific compiled graphs are reused.

### US-15.11.3.3
As an **engine tester (P-27)**, I want to verify unchanged graphs fetch from cache instead of
recompiling so that graph cache hit behavior is regression-tested.

---

## US-15.11.4 New Developer Onboarding Acceleration

### US-15.11.4.1
As a **developer (P-15)**, I want first editor launch to fetch all compiled assets from cache so
that my first build takes minutes instead of hours.

### US-15.11.4.2
As a **developer (P-15)**, I want cache prefetch running in parallel with Git LFS downloads so that
both operations saturate available bandwidth.

### US-15.11.4.3
As a **developer (P-15)**, I want a progress dashboard showing download status so that I can monitor
onboarding progress with ETAs.

### US-15.11.4.4
As an **engine tester (P-27)**, I want to verify first-launch time is under 10 minutes for a project
taking 1+ hours to build from source so that onboarding acceleration is regression-tested.

---

## US-15.11.5 Cache Invalidation and Garbage Collection

### US-15.11.5.1
As a **DevOps (P-16)**, I want cache entries invalidated on tool version change so that stale
compiled assets are never served.

### US-15.11.5.2
As a **DevOps (P-16)**, I want configurable garbage collection schedules so that orphaned entries
are cleaned up automatically.

### US-15.11.5.3
As a **DevOps (P-16)**, I want LRU eviction when storage quota is exceeded so that the cache does
not grow unboundedly.

### US-15.11.5.4
As a **server admin (P-22)**, I want manual garbage collection and invalidation logs so that I can
audit and trigger cleanup when needed.

### US-15.11.5.5
As an **engine tester (P-27)**, I want to verify tool version change invalidates all old entries so
that invalidation correctness is regression-tested.

---

## US-15.11.6 Cache Transport and Storage

### US-15.11.6.1
As a **DevOps (P-16)**, I want multiple storage backends (S3, GCS, Azure, local, HTTP) so that the
cache fits our infrastructure.

### US-15.11.6.2
As a **DevOps (P-16)**, I want Zstd compression for cache transfers so that network bandwidth is
used efficiently.

### US-15.11.6.3
As a **DevOps (P-16)**, I want configurable download concurrency and bandwidth limits so that cache
traffic does not saturate the office network.

### US-15.11.6.4
As an **engine dev (P-26)**, I want platform-native HTTP clients (NSURLSession, WinHTTP, libcurl) so
that transfers use the most performant stack per platform.

### US-15.11.6.5
As an **engine tester (P-27)**, I want to verify bandwidth limit is respected during downloads so
that throttling is regression-tested.

---

## US-15.11.7 CI/CD Cache Population

### US-15.11.7.1
As a **DevOps (P-16)**, I want CI builds to auto-populate the cache for all platforms so that
developers have pre-built assets without building themselves.

### US-15.11.7.2
As a **DevOps (P-16)**, I want idempotent uploads that skip existing entries so that duplicate
uploads do not waste bandwidth.

### US-15.11.7.3
As a **DevOps (P-16)**, I want nightly builds warming cache for all active branches so that feature
branch developers benefit from the cache.

### US-15.11.7.4
As a **DevOps (P-16)**, I want a standalone CLI for headless build servers so that cache population
works outside the editor.

### US-15.11.7.5
As an **engine tester (P-27)**, I want to verify duplicate upload is a no-op so that idempotent
upload behavior is regression-tested.

---

## US-15.11.8 Cache Hit Metrics and Monitoring

### US-15.11.8.1
As a **DevOps (P-16)**, I want a monitoring dashboard with hit rate and storage usage so that I can
track cache effectiveness.

### US-15.11.8.2
As a **DevOps (P-16)**, I want configurable alerts when hit rate drops below threshold so that cache
misconfigurations are detected quickly.

### US-15.11.8.3
As a **DevOps (P-16)**, I want per-developer and per-team usage breakdowns so that I can identify
workflow bottlenecks.

### US-15.11.8.4
As a **DevOps (P-16)**, I want metrics exported in OpenTelemetry format so that I can integrate with
Grafana, Datadog, or other observability tools.

### US-15.11.8.5
As an **engine tester (P-27)**, I want to verify the alert fires when hit rate drops below threshold
so that alerting is regression-tested.
