# 12.3 — Asset Database

## Storage

| ID       | Feature                     | Requirements |
|----------|-----------------------------|--------------|
| F-12.3.1 | Content-Addressable Storage | R-12.3.1     |
| F-12.3.2 | Asset Metadata Store        | R-12.3.2     |

1. **F-12.3.1** — All processed assets are stored in a content-addressable store keyed by the
   cryptographic hash (BLAKE3) of their contents. Identical assets are deduplicated automatically,
   and any asset version can be retrieved by its hash. This is critical for MMO-scale repositories
   where thousands of artists produce overlapping content across expansion packs and live-ops
   updates.
2. **F-12.3.2** — A persistent metadata database mapping asset IDs to source paths, content hashes,
   import settings, dependency lists, tags, thumbnails, and platform-specific build outputs. The
   metadata store is the single source of truth for the entire content pipeline and editor asset
   browser.
   - **Deps:** F-12.3.1

## Caching and Builds

| ID       | Feature                   | Requirements |
|----------|---------------------------|--------------|
| F-12.3.3 | Hash-Based Import Caching | R-12.3.3     |
| F-12.3.4 | Incremental Builds        | R-12.3.4     |

1. **F-12.3.3** — Cache processed asset outputs keyed by the hash of (source content + import
   settings + tool version). A cache hit skips all import and processing stages, returning the
   previously built output directly. For MMO-scale builds with hundreds of thousands of assets,
   caching reduces full rebuild times from hours to minutes by reprocessing only changed assets.
   - **Deps:** F-12.3.1, F-12.2.8
2. **F-12.3.4** — Rebuild only the assets whose source files, import settings, or transitive
   dependencies have changed since the last build. The dependency graph is walked bottom-up to
   propagate invalidation: changing a shared texture reprocesses only the materials and prefabs that
   reference it, not the entire asset library.
   - **Deps:** F-12.3.3, F-12.2.8

## Search, Tagging, and Thumbnails

| ID       | Feature                    | Requirements |
|----------|----------------------------|--------------|
| F-12.3.5 | Asset Search and Tagging   | R-12.3.5     |
| F-12.3.6 | Asset Thumbnail Generation | R-12.3.6     |

1. **F-12.3.5** — Full-text and tag-based search across the asset database. Assets can be tagged
   manually or automatically (e.g., by import type, directory, resolution tier). Faceted search by
   type, tag, modification date, file size, and dependency count enables artists and designers to
   locate assets quickly in libraries containing millions of entries.
   - **Deps:** F-12.3.2
2. **F-12.3.6** — Automatic thumbnail rendering for meshes (orbit camera preview), textures
   (downscaled preview), materials (sphere preview), and audio (waveform visualization). Thumbnails
   are generated asynchronously during import and stored in the metadata database for instant
   display in the editor asset browser.
   - **Deps:** F-12.3.2

## AI-Assisted Asset Management

| ID       | Feature                               | Requirements |
|----------|---------------------------------------|--------------|
| F-12.3.7 | AI-Driven Auto-Categorization         | R-12.3.7     |
| F-12.3.8 | LLM-Based Semantic Asset Search       | R-12.3.8     |
| F-12.3.9 | Smart Collections and Recommendations | R-12.3.9     |

1. **F-12.3.7** — An LLM-based classifier that automatically assigns categories, tags, and quality
   ratings to imported assets based on visual content, naming conventions, and metadata. Meshes are
   classified by function (prop, weapon, architecture, vegetation), textures by usage (diffuse,
   normal, mask), and audio by role (ambient, SFX, music). Reduces manual curation overhead for
   MMO-scale asset libraries with hundreds of thousands of entries across expansion packs.
   - **Deps:** F-12.3.2, F-12.3.6
2. **F-12.3.8** — Natural language queries against the asset database — e.g., "rusted metal door
   with broken hinges" or "ambient forest sounds at night." An embedding model encodes asset
   thumbnails, descriptions, tags, and file names into a vector index. Similarity search returns
   ranked results with confidence scores. Enables artists and designers to find assets by intent
   rather than memorized naming conventions.
   - **Deps:** F-12.3.5, F-12.3.7
3. **F-12.3.9** — Automatically generated asset collections based on usage patterns, co-occurrence
   in scenes, and semantic similarity. When an artist selects a stone wall texture, the system
   recommends matching trim, decal, and rubble meshes. Recommendations update as the project
   evolves, surfacing underused assets and flagging near-duplicate imports that waste storage.
   - **Deps:** F-12.3.8

## Versioning

| ID        | Feature          | Requirements |
|-----------|------------------|--------------|
| F-12.3.10 | Asset Versioning | R-12.3.10    |

1. **F-12.3.10** — Track the full revision history of each asset, including source file changes,
   import setting modifications, and processing outputs. Any previous version can be restored by its
   content hash. Versioning supports live-ops workflows where content must be rolled back quickly if
   a patch introduces visual regressions or asset corruption. Version history is stored using the
   structural diffing system defined in F-12.7.3.
   - **Deps:** F-12.3.1, F-12.3.2, F-12.7.3 (Structural Asset Diffing)
