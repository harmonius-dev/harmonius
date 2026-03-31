# User Stories -- 12.3 Asset Database

## Storage

| ID        | Persona                    |
|-----------|----------------------------|
| US-12.3.1 | build engineer (P-16)      |
| US-12.3.2 | technical artist (P-13)    |
| US-12.3.3 | engine developer (P-26)    |

1. **US-12.3.1** — **As a** build engineer (P-16), **I want** all processed assets stored in a
   content-addressable store keyed by BLAKE3 hash, **so that** identical assets are deduplicated
   automatically across builds.
2. **US-12.3.2** — **As a** technical artist (P-13), **I want** a persistent metadata database
   mapping asset IDs to source paths, content hashes, import settings, dependencies, tags, and
   thumbnails, **so that** I have a single source of truth for every asset in the project.
3. **US-12.3.3** — **As an** engine developer (P-26), **I want** the metadata store to track
   platform-specific build outputs per asset, **so that** cross-platform builds can query the
   correct output for each target.

## Caching and Builds

| ID        | Persona                    |
|-----------|----------------------------|
| US-12.3.4 | build engineer (P-16)      |
| US-12.3.5 | build engineer (P-16)      |
| US-12.3.6 | technical artist (P-13)    |

1. **US-12.3.4** — **As a** build engineer (P-16), **I want** processed asset outputs cached by the
   hash of source content, import settings, and tool version, **so that** cache hits skip all
   processing stages and reduce rebuild times.
2. **US-12.3.5** — **As a** build engineer (P-16), **I want** only assets whose source files,
   settings, or transitive dependencies changed to be rebuilt, **so that** incremental builds avoid
   reprocessing unchanged assets.
3. **US-12.3.6** — **As a** technical artist (P-13), **I want** dependency invalidation propagated
   bottom-up through the dependency graph, **so that** changing a shared texture rebuilds only the
   materials that reference it.

## Search, Tagging, and Thumbnails

| ID        | Persona                    |
|-----------|----------------------------|
| US-12.3.7 | environment artist (P-8)   |
| US-12.3.8 | environment artist (P-8)   |
| US-12.3.9 | technical artist (P-13)    |

1. **US-12.3.7** — **As an** environment artist (P-8), **I want** full-text and tag-based search
   with faceted filtering by type, tag, date, size, and dependency count, **so that** I can locate
   assets quickly in large libraries.
2. **US-12.3.8** — **As an** environment artist (P-8), **I want** thumbnails generated automatically
   for meshes, textures, materials, and audio during import, **so that** the asset browser shows
   visual previews instantly.
3. **US-12.3.9** — **As a** technical artist (P-13), **I want** assets tagged both manually and by
   automatic rules based on import type and directory, **so that** categorization scales without
   manual effort.

## AI-Assisted Asset Management

| ID         | Persona                    |
|------------|----------------------------|
| US-12.3.10 | environment artist (P-8)   |
| US-12.3.11 | environment artist (P-8)   |
| US-12.3.12 | technical artist (P-13)    |
| US-12.3.13 | environment artist (P-8)   |

1. **US-12.3.10** — **As an** environment artist (P-8), **I want** an AI classifier to auto-assign
   categories, tags, and quality ratings to imported assets, **so that** I spend less time on manual
   curation.
2. **US-12.3.11** — **As an** environment artist (P-8), **I want** to search the asset database with
   natural language queries like "rusted metal door with broken hinges," **so that** I can find
   assets by intent rather than memorized names.
3. **US-12.3.12** — **As a** technical artist (P-13), **I want** near-duplicate imports flagged
   automatically, **so that** wasted storage from redundant assets is identified early.
4. **US-12.3.13** — **As an** environment artist (P-8), **I want** the system to recommend related
   assets when I select one, **so that** I discover matching trim, decal, and rubble meshes without
   manual searching.

## Versioning

| ID         | Persona                    |
|------------|----------------------------|
| US-12.3.14 | technical artist (P-13)    |
| US-12.3.15 | build engineer (P-16)      |

1. **US-12.3.14** — **As a** technical artist (P-13), **I want** the full revision history of each
   asset tracked including source changes, setting modifications, and outputs, **so that** I can
   inspect how an asset evolved over time.
2. **US-12.3.15** — **As a** build engineer (P-16), **I want** any previous asset version restorable
   by its content hash, **so that** live-ops rollbacks complete quickly when a patch introduces
   regressions.
