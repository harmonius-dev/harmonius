# User Stories -- 15.11 Shared Build Cache

## Stories

| ID           | Persona                 |
|--------------|-------------------------|
| US-15.11.1.1 | build engineer (P-16)   |
| US-15.11.1.2 | engine developer (P-26) |
| US-15.11.2.1 | technical artist (P-13) |
| US-15.11.2.2 | build engineer (P-16)   |
| US-15.11.3.1 | game designer (P-5)     |
| US-15.11.3.2 | build engineer (P-16)   |
| US-15.11.4.1 | engine developer (P-26) |
| US-15.11.4.2 | build engineer (P-16)   |
| US-15.11.5.1 | build engineer (P-16)   |
| US-15.11.5.2 | engine developer (P-26) |
| US-15.11.6.1 | build engineer (P-16)   |
| US-15.11.6.2 | engine developer (P-26) |
| US-15.11.7.1 | build engineer (P-16)   |
| US-15.11.7.2 | engine developer (P-26) |
| US-15.11.8.1 | build engineer (P-16)   |
| US-15.11.8.2 | engine developer (P-26) |

1. **US-15.11.1.1** — **As a** build engineer (P-16), **I want** a centralized web service caching
   compiled assets keyed by content hash, **so that** no two developers rebuild the same asset.

2. **US-15.11.1.2** — **As a** engine developer (P-26), **I want** cache lookups by source hash,
   build settings, and tool version, **so that** stale entries are never served.

3. **US-15.11.2.1** — **As a** technical artist (P-13), **I want** shader variants cached by
   platform and permutation flags, **so that** a full shader rebuild completes in minutes.

4. **US-15.11.2.2** — **As a** build engineer (P-16), **I want** CI builds to populate the shader
   cache for all platforms, **so that** developers always have pre-compiled shaders.

5. **US-15.11.3.1** — **As a** game designer (P-5), **I want** compiled logic graph bytecode cached
   by content hash, **so that** unchanged graphs load instantly on editor launch.

6. **US-15.11.3.2** — **As a** build engineer (P-16), **I want** cache entries for both bytecode and
   AOT native code, **so that** both runtime modes are accelerated.

7. **US-15.11.4.1** — **As a** engine developer (P-26), **I want** a fresh clone's first launch to
   fetch all compiled assets from the cache, **so that** onboarding time drops from hours to
   minutes.

8. **US-15.11.4.2** — **As a** build engineer (P-16), **I want** cache prefetch to run in parallel
   with Git LFS downloads, **so that** available bandwidth is fully utilized.

9. **US-15.11.5.1** — **As a** build engineer (P-16), **I want** cache invalidation on tool version
   changes and LRU garbage collection with storage quotas, **so that** the cache stays current and
   bounded.

10. **US-15.11.5.2** — **As a** engine developer (P-26), **I want** manual GC triggers and
    invalidation logs, **so that** I can diagnose cache misses.

11. **US-15.11.6.1** — **As a** build engineer (P-16), **I want** multiple storage backends (local,
    S3, GCS, Azure Blob), **so that** I can choose based on team size and budget.

12. **US-15.11.6.2** — **As a** engine developer (P-26), **I want** Zstd compression and
    content-addressable deduplication, **so that** transfer and storage are efficient.

13. **US-15.11.7.1** — **As a** build engineer (P-16), **I want** CI pipelines to auto-populate the
    cache for all target platforms, **so that** developers always have warm caches.

14. **US-15.11.7.2** — **As a** engine developer (P-26), **I want** cache population to be
    idempotent, **so that** redundant CI uploads are no-ops.

15. **US-15.11.8.1** — **As a** build engineer (P-16), **I want** a monitoring dashboard with cache
    hit rate, storage usage, and build time savings, **so that** I can track effectiveness.

16. **US-15.11.8.2** — **As a** engine developer (P-26), **I want** alerts when hit rate drops below
    a threshold, **so that** I can detect tool version mismatches quickly.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-15.11.1 | build engineer (P-16) |
| US-15.11.2 | technical artist (P-13) |
| US-15.11.3 | game designer (P-5) |
| US-15.11.4 | engine developer (P-26) |
| US-15.11.5 | build engineer (P-16) |
| US-15.11.6 | build engineer (P-16) |
| US-15.11.7 | build engineer (P-16) |
| US-15.11.8 | build engineer (P-16) |

1. **US-15.11.1** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.11.1.1 through US-15.11.1.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

2. **US-15.11.2** -- **As a** technical artist (P-13), **I want** the capabilities defined in
   sub-stories US-15.11.2.1 through US-15.11.2.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

3. **US-15.11.3** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-15.11.3.1 through US-15.11.3.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

4. **US-15.11.4** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-15.11.4.1 through US-15.11.4.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

5. **US-15.11.5** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.11.5.1 through US-15.11.5.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

6. **US-15.11.6** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.11.6.1 through US-15.11.6.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

7. **US-15.11.7** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.11.7.1 through US-15.11.7.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

8. **US-15.11.8** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.11.8.1 through US-15.11.8.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.
