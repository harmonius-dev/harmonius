# R-12.3 Asset Database

| ID        | Derived From                                                   |
|-----------|----------------------------------------------------------------|
| R-12.3.1  | [F-12.3.1](../../features/content-pipeline/asset-database.md)  |
| R-12.3.2  | [F-12.3.2](../../features/content-pipeline/asset-database.md)  |
| R-12.3.3  | [F-12.3.3](../../features/content-pipeline/asset-database.md)  |
| R-12.3.4  | [F-12.3.4](../../features/content-pipeline/asset-database.md)  |
| R-12.3.5  | [F-12.3.5](../../features/content-pipeline/asset-database.md)  |
| R-12.3.6  | [F-12.3.6](../../features/content-pipeline/asset-database.md)  |
| R-12.3.7  | [F-12.3.7](../../features/content-pipeline/asset-database.md)  |
| R-12.3.8  | [F-12.3.8](../../features/content-pipeline/asset-database.md)  |
| R-12.3.9  | [F-12.3.9](../../features/content-pipeline/asset-database.md)  |
| R-12.3.10 | [F-12.3.10](../../features/content-pipeline/asset-database.md) |

1. **R-12.3.1** — The engine **SHALL** store all processed assets in a content-addressable store
   keyed by BLAKE3 cryptographic hash, automatically deduplicating identical assets and enabling
   retrieval of any asset version by its hash.
   - **Rationale:** Content-addressable storage eliminates redundant data in large-scale
     repositories where overlapping content is produced across expansion packs and live-ops updates.
   - **Verification:** Store two identical assets and confirm only one copy exists on disk; retrieve
     an asset by its hash and confirm the content matches the original; store a modified asset and
     confirm both versions are retrievable.
2. **R-12.3.2** — The engine **SHALL** maintain a persistent metadata database mapping asset IDs to
   source paths, content hashes, import settings, dependency lists, tags, thumbnails, and
   platform-specific build outputs, serving as the single source of truth for the content pipeline
   and editor asset browser.
   - **Rationale:** A centralized metadata store ensures consistent asset tracking across the
     import, processing, and editor browsing workflows.
   - **Verification:** Import an asset and confirm all metadata fields (source path, hash, import
     settings, dependencies, tags, thumbnail, build outputs) are present and correct in the
     database; query from the editor asset browser and confirm the metadata matches.
3. **R-12.3.3** — The engine **SHALL** cache processed asset outputs keyed by the hash of source
   content, import settings, and tool version, and skip all import and processing stages on a cache
   hit, returning the previously built output directly.
   - **Rationale:** Hash-based caching reduces full rebuild times from hours to minutes by
     reprocessing only changed assets in large-scale builds.
   - **Verification:** Import an asset, confirm the output is cached; re-import with identical
     source and settings and confirm the processing stages are skipped; change the import settings
     and confirm a cache miss triggers reprocessing.
4. **R-12.3.4** — The engine **SHALL** rebuild only those assets whose source files, import
   settings, or transitive dependencies have changed since the last build, propagating invalidation
   bottom-up through the dependency graph.
   - **Rationale:** Bottom-up invalidation through the dependency graph ensures only affected assets
     are reprocessed, avoiding unnecessary work on unchanged assets.
   - **Verification:** Modify a shared texture and confirm only materials and prefabs that
     transitively depend on it are rebuilt; confirm unchanged assets are not reprocessed.
5. **R-12.3.5** — The engine **SHALL** support full-text and tag-based search across the asset
   database with faceted filtering by type, tag, modification date, file size, and dependency count,
   and support both manual and automatic tagging.
   - **Rationale:** Fast faceted search is essential for locating assets in libraries containing
     millions of entries without relying on memorized naming conventions.
   - **Verification:** Tag assets manually and by automatic rules; perform full-text, tag-based, and
     faceted searches and confirm correct result sets; confirm search returns results within an
     acceptable latency threshold.
6. **R-12.3.6** — The engine **SHALL** asynchronously generate thumbnails during import for meshes
   (orbit camera preview), textures (downscaled preview), materials (sphere preview), and audio
   (waveform visualization), and store them in the metadata database.
   - **Rationale:** Pre-generated thumbnails enable instant visual browsing in the editor asset
     browser without on-demand rendering delays.
   - **Verification:** Import assets of each type (mesh, texture, material, audio) and confirm a
     thumbnail is generated and stored; confirm the editor asset browser displays thumbnails without
     triggering additional rendering.
7. **R-12.3.7** — The engine **SHALL** automatically assign categories, tags, and quality ratings to
   imported assets using an LLM-based classifier that analyzes visual content, naming conventions,
   and metadata, classifying meshes by function, textures by usage, and audio by role.
   - **Rationale:** Automated categorization reduces manual curation overhead for large-scale asset
     libraries with hundreds of thousands of entries.
   - **Verification:** Import a set of unlabeled assets and confirm the classifier assigns correct
     categories and tags with confidence above a defined threshold; compare against a manually
     curated ground-truth set and confirm precision and recall meet acceptance criteria.
8. **R-12.3.8** — The engine **SHALL** support natural language queries against the asset database
   by encoding asset thumbnails, descriptions, tags, and file names into a vector index and
   returning ranked results with confidence scores via similarity search.
   - **Rationale:** Semantic search enables artists and designers to find assets by intent rather
     than exact naming conventions, improving discoverability.
   - **Verification:** Issue natural language queries (e.g., "rusted metal door") and confirm the
     top results are semantically relevant; confirm confidence scores decrease with decreasing
     relevance.
9. **R-12.3.9** — The engine **SHALL** automatically generate asset collections based on usage
   patterns, co-occurrence in scenes, and semantic similarity, and recommend related assets when an
   asset is selected, surfacing underused assets and flagging near-duplicate imports.
   - **Rationale:** Recommendations and smart collections reduce manual curation work and help
     identify redundant or underused assets.
   - **Verification:** Select an asset and confirm the system recommends semantically related
     assets; confirm near-duplicate imports are flagged; confirm underused assets are surfaced in
     collections.
10. **R-12.3.10** — The engine **SHALL** track the full revision history of each asset including
    source file changes, import setting modifications, and processing outputs, enable restoration of
    any previous version by its content hash, and store version history using the structural diffing
    system.
    - **Rationale:** Full version history with hash-based restoration supports live-ops workflows
      where content rollback must be fast and reliable.
    - **Verification:** Import an asset, modify it, and confirm both versions appear in the revision
      history; restore a previous version by hash and confirm the content matches the original;
      confirm structural diffs are stored between consecutive versions.
