# User Stories -- 12.7 Asset Versioning & Collaboration

## US-12.7.1 Load Assets Quickly with Memory-Mapped Binary Format

**As a** game developer (P-15), **I want** all engine assets stored in a universal binary
format with O(1) section access via a table-of-contents header, **so that** asset loading
uses fast mmap-based reads without parsing overhead.

## US-12.7.2 Distribute Assets in Compressed Bundles

**As a** DevOps engineer (P-16), **I want** related assets grouped into compressed bundles
(LZ4 for runtime, Zstd for distribution) with partial decompression and random access,
**so that** bundle creation, streaming, and patching are efficient.

## US-12.7.3 See Visual Diffs of Any Two Asset Versions

**As a** designer (P-5), **I want** structural diffing that shows added, removed, and
modified elements at the semantic level (graph nodes, table rows, vertex count deltas,
texture side-by-side), **so that** I can understand exactly what changed between two
versions of an asset.

## US-12.7.4 Merge Concurrent Edits to the Same Asset

**As a** designer (P-5), **I want** three-way merge that automatically succeeds when changes
are to non-overlapping structural regions, with per-region conflict resolution for
overlapping changes, **so that** my team can work on the same assets concurrently without
losing changes.

## US-12.7.5 Resolve Common Merge Conflicts Automatically

**As a** designer (P-5), **I want** conservative automatic conflict resolution
(last-writer-wins for independent properties, union for additive collections, deterministic
ordering for reordered elements), **so that** routine merges complete without manual
intervention.

## US-12.7.6 Edit Gameplay Data Tables in a Spreadsheet

**As a** designer (P-5), **I want** a spreadsheet-style data table editor with sorting,
filtering, search, inline formula editing, row inheritance, bulk operations, and per-cell
validation, **so that** I can author and iterate on gameplay data efficiently without any
code.

## US-12.7.7 Inspect Any Asset Type Visually

**As a** designer (P-5), **I want** a dedicated visual inspector for every asset type
(3D mesh preview, texture channels/mips, animation timeline, audio waveform, material
sphere), **so that** I never need to view raw files and can always inspect assets in their
natural representation.

## US-12.7.8 Use Git LFS with Custom Merge Drivers

**As a** DevOps engineer (P-16), **I want** binary assets tracked via Git LFS with a custom
merge driver that invokes the engine's structural merge system, optional lock-before-edit,
and status indicators in the asset browser, **so that** version control works smoothly for
large binary assets.

## US-12.7.9 Integrate Version Control in the Asset Browser

**As a** designer (P-5), **I want** the asset browser to show lock owner, modified state,
and merge conflict presence for each asset, **so that** I can see version control status
without switching to a separate Git client.

## US-12.7.10 Use Mobile-Optimized Bundle Sizes for Download-on-Demand

**As a** game developer (P-15), **I want** mobile builds to use smaller bundle granularity
with LZ4 compression for decompression speed, **so that** download-on-demand over metered
connections works with small, fast-loading chunks.

## US-12.7.11 Verify Structural Diff Correctness Across Asset Types

**As an** engine tester (P-27), **I want** automated tests that modify known assets (add
graph nodes, change table rows, alter mesh vertices, adjust textures) and verify structural
diffs report the exact changes, **so that** diff accuracy is validated for every asset type.

## US-12.7.12 Verify Three-Way Merge Produces Correct Results

**As an** engine tester (P-27), **I want** tests that create two divergent edits from a
common ancestor and verify merge produces the expected combined result for non-overlapping
changes and correctly identifies conflicts for overlapping changes, **so that** merge
correctness is validated.

## US-12.7.13 Verify Bundle Partial Decompression

**As an** engine tester (P-27), **I want** tests that extract individual assets from
compressed bundles without decompressing the entire bundle and verify content hash matches,
**so that** partial decompression works correctly for streaming and patching.

## US-12.7.14 Benchmark Bundle Compression and Decompression Speed

**As an** engine tester (P-27), **I want** benchmarks measuring LZ4 and Zstd compression
and decompression throughput for asset bundles, **so that** bundle I/O performance meets
streaming latency and distribution size targets.

## US-12.7.15 Customize Automatic Merge Resolution Strategy per Project

**As a** DevOps engineer (P-16), **I want** configurable merge resolution strategies per
asset type and per project, **so that** teams with different collaboration patterns can
tune automatic merge behavior to their workflow.
