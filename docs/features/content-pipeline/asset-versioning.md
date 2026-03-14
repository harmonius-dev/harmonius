# 12.7 — Asset Versioning & Collaboration

## Binary Asset Format

### F-12.7.1 Universal Binary Asset Format

All engine assets (meshes, textures, materials, logic graphs, data tables, animations, scenes)
are stored in a single binary format with a common header (magic, version, type ID, content
hash, compression codec) and type-specific payloads. The format is designed for fast mmap-based
loading and O(1) section access via a table-of-contents header. No text-based asset files exist
in production — all authoring goes through visual editors.

- **Requirements:** R-12.7.1
- **Dependencies:** F-1.4.1 (Serialization)
- **Platform notes:** None

### F-12.7.2 Compressed Asset Bundles

Group related assets into compressed bundles (LZ4 for runtime, Zstd for distribution). Bundles
support partial decompression — individual assets can be extracted without decompressing the
entire bundle. Bundle manifests list contained assets with offsets for random access. Bundles
are the unit of streaming, patching, and download-on-demand delivery.

- **Requirements:** R-12.7.2
- **Dependencies:** F-12.7.1, F-12.5.1 (Virtual File System)
- **Platform notes:** None

## Diffing and Merging

### F-12.7.3 Structural Asset Diffing

Diff any two versions of an asset at the structural level — not byte-level. For logic graphs:
show added/removed/modified nodes and connections. For meshes: show vertex count changes and
bounding box deltas. For data tables: show added/removed/changed rows and columns. For
textures: show a visual side-by-side with difference highlighting. Diffs display in the editor
with the same visual representation as the asset editor itself.

- **Requirements:** R-12.7.3
- **Dependencies:** F-12.7.1
- **Platform notes:** None

### F-12.7.4 Three-Way Asset Merge

Merge two divergent versions of an asset against a common ancestor. Automatic merge succeeds
when changes are to non-overlapping structural regions (different nodes in a graph, different
rows in a table, different UV islands in a mesh). Conflicting changes are presented in the
visual diff tool with per-region accept-left/accept-right/manual-edit resolution. Integrates
with git via a custom merge driver.

- **Requirements:** R-12.7.4
- **Dependencies:** F-12.7.3
- **Platform notes:** None

### F-12.7.5 Automatic Merge Conflict Resolution

For common non-ambiguous conflicts, the merge system resolves automatically: last-writer-wins
for independent property changes, union for additive collections (new nodes added by both
sides), and deterministic ordering for reordered elements. Automatic resolution is conservative
— any ambiguity falls through to manual resolution. Resolution strategy is configurable per
asset type and per project.

- **Requirements:** R-12.7.5
- **Dependencies:** F-12.7.4
- **Platform notes:** None

## Visual Editors

### F-12.7.6 Spreadsheet-Style Data Table Editor

Edit gameplay data tables (F-13.7) in a spreadsheet grid with column sorting, filtering,
search, inline formula editing, row inheritance visualization, and bulk operations (copy,
paste, fill-down). Cell validation highlights constraint violations in real-time. Import from
and export to CSV for external tooling. Supports undo/redo with per-cell granularity.

- **Requirements:** R-12.7.6
- **Dependencies:** F-13.7.1 (Table Schema), F-15.1.1 (Editor Framework)
- **Platform notes:** None

### F-12.7.7 Universal Asset Inspector

Every asset type has a dedicated visual inspector in the editor — no raw file editing. Meshes
show a 3D preview with wireframe overlay. Textures show channels, mip levels, and histograms.
Animations show timeline with curve editors. Audio shows waveform and spectrum. Materials show
a lit sphere preview. The inspector is the ONLY way to view and edit assets; there is no
text/code fallback.

- **Requirements:** R-12.7.7
- **Dependencies:** F-15.1.1 (Editor Framework)
- **Platform notes:** None

## Version Control Integration

### F-12.7.8 Git LFS and Custom Merge Driver Integration

Binary assets are tracked via Git LFS with a custom merge driver that invokes the engine's
structural merge system (F-12.7.4) for three-way merges. Lock-before-edit is optional per
asset type for teams that prefer pessimistic locking on large binary assets. Status indicators
in the asset browser show lock owner, modified state, and merge conflict presence.

- **Requirements:** R-12.7.8
- **Dependencies:** F-12.7.4, F-12.3.2 (Asset Metadata)
- **Platform notes:** None
