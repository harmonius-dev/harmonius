# Database and Versioning

Tracking assets, deduplication, and collaborative editing.

## What it covers

- Content-addressable storage keyed by BLAKE3 hash with automatic deduplication.
- Persistent metadata database mapping asset IDs to source paths, hashes, dependencies, and
  thumbnails.
- Import cache based on source + settings + tool version for incremental builds.
- Full-text and faceted search (type, tag, modified date).
- LLM-based asset classification and semantic vector search.
- Asset revision history with hash-based restoration.
- Structural diffs showing asset-type-specific changes (graph nodes, mesh stats, table rows).
- Three-way merge with automatic resolution for non-overlapping changes and visual conflict tools.
- Git integration with custom merge driver and asset-specific locking.

## Concepts

### Content Addressing

Every asset is stored by its BLAKE3 content hash. Two assets with identical content share a single
storage entry. Importing the same asset twice produces the same hash and deduplicates automatically.
Version retrieval is possible by hash, enabling reproducible builds.

### Metadata and Indexing

A persistent database maps human-readable asset IDs to hashes, source paths, dependencies, and build
outputs. Secondary indices (full-text, faceted) enable fast search. Thumbnails are generated
asynchronously and cached in the database for quick browsing.

### Collaborative Editing

Assets are treated like source code: stored in Git, diffable, mergeable, and lockable. Structural
diffs show semantic changes (node count, mesh triangle count) instead of binary diffs. Three-way
merge resolves non-overlapping changes automatically; overlapping changes surface a visual conflict
tool. Locking prevents simultaneous edit by multiple authors.

## How it fits

- See [import-and-processing](./import-and-processing.md) for where assets are built.
- See [streaming-and-hot-reload](./streaming-and-hot-reload.md) for asset delivery to the running
  game.
- Integrates with [authoring-runtime](../core-runtime/authoring-runtime.md) for graphs and plugin
  content.
