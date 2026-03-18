# R-12.7 Asset Versioning & Collaboration

| ID       | Derived From                                                    |
|----------|-----------------------------------------------------------------|
| R-12.7.1 | [F-12.7.1](../../features/content-pipeline/asset-versioning.md) |
| R-12.7.2 | [F-12.7.2](../../features/content-pipeline/asset-versioning.md) |
| R-12.7.3 | [F-12.7.3](../../features/content-pipeline/asset-versioning.md) |
| R-12.7.4 | [F-12.7.4](../../features/content-pipeline/asset-versioning.md) |
| R-12.7.5 | [F-12.7.5](../../features/content-pipeline/asset-versioning.md) |
| R-12.7.6 | [F-12.7.6](../../features/content-pipeline/asset-versioning.md) |
| R-12.7.7 | [F-12.7.7](../../features/content-pipeline/asset-versioning.md) |
| R-12.7.8 | [F-12.7.8](../../features/content-pipeline/asset-versioning.md) |

1. **R-12.7.1** — The engine **SHALL** store all assets (meshes, textures, materials, logic graphs,
   data tables, animations, scenes) in a single binary format with a common header (magic, version,
   type ID, content hash, compression codec) and type-specific payloads, supporting fast mmap-based
   loading and O(1) section access via a table-of-contents header, with no text-based asset files in
   production.
   - **Rationale:** A unified binary format with mmap loading and O(1) section lookup eliminates
     parsing overhead and enables instant random access to any asset section.
   - **Verification:** Write and read back each asset type; confirm the header contains correct
     magic, version, type ID, and content hash; mmap a multi-section asset and confirm O(1) access
     to an arbitrary section without reading preceding sections.
2. **R-12.7.2** — The engine **SHALL** group related assets into compressed bundles using LZ4 for
   runtime and Zstd for distribution, support partial decompression of individual assets without
   decompressing the entire bundle, and include bundle manifests listing contained assets with
   offsets for random access.
   - **Rationale:** Compressed bundles with partial decompression reduce storage and bandwidth while
     avoiding the latency penalty of full-bundle decompression for single-asset access.
   - **Verification:** Create a bundle with 10 assets; extract the 5th asset without decompressing
     the other 9; confirm the manifest lists all 10 assets with correct offsets; confirm LZ4 is used
     at runtime and Zstd at distribution time.
3. **R-12.7.3** — The engine **SHALL** diff any two asset versions at the structural level, showing
   added, removed, and modified elements specific to each asset type (nodes/connections for logic
   graphs, vertex count/bounding box for meshes, rows/columns for data tables, visual side-by-side
   with difference highlighting for textures), displayed in the editor using the same visual
   representation as the asset editor.
   - **Rationale:** Structural diffs let artists and designers understand what changed in an asset
     at the semantic level rather than inspecting raw binary differences.
   - **Verification:** Create two versions of a logic graph with an added node; diff them and
     confirm the added node is highlighted; diff two mesh versions and confirm vertex count and
     bounding box deltas are shown; diff two textures and confirm a visual side-by-side is rendered.
4. **R-12.7.4** — The engine **SHALL** merge two divergent asset versions against a common ancestor,
   automatically succeed when changes are to non-overlapping structural regions, present conflicting
   changes in the visual diff tool with per-region accept-left/accept-right/manual-edit resolution,
   and integrate with Git via a custom merge driver.
   - **Rationale:** Structural three-way merge with a Git merge driver enables concurrent artist
     edits on the same asset without forcing pessimistic file locking.
   - **Verification:** Branch an asset, make non-overlapping changes on each branch, merge, and
     confirm automatic success; make overlapping changes, merge, and confirm the conflict is
     presented in the visual diff tool with resolution options; confirm the Git merge driver invokes
     the engine merge system.
5. **R-12.7.5** — The engine **SHALL** automatically resolve non-ambiguous merge conflicts using
   last-writer-wins for independent property changes, union for additive collections, and
   deterministic ordering for reordered elements, with any ambiguity falling through to manual
   resolution and resolution strategy configurable per asset type and per project.
   - **Rationale:** Conservative automatic resolution for common non-ambiguous cases reduces manual
     merge workload while preserving safety through fallback to manual resolution.
   - **Verification:** Create a merge with independent property changes on both sides and confirm
     automatic resolution; create a merge with ambiguous changes and confirm fallback to manual
     resolution; change the resolution strategy per asset type and confirm the new strategy applies.
6. **R-12.7.6** — The engine **SHALL** provide a spreadsheet-style editor for gameplay data tables
   with column sorting, filtering, search, inline formula editing, row inheritance visualization,
   bulk operations (copy, paste, fill-down), real-time cell validation with constraint violation
   highlighting, CSV import/export, and per-cell undo/redo.
   - **Rationale:** A spreadsheet interface matches designers' mental model for tabular data and
     provides the bulk-editing capabilities needed for large gameplay data sets.
   - **Verification:** Open a data table with 100 rows; sort, filter, and search; edit a cell and
     confirm undo/redo restores the previous value; enter an invalid value and confirm a constraint
     violation is highlighted; export to CSV and reimport, confirming round-trip fidelity.
7. **R-12.7.7** — The engine **SHALL** provide a dedicated visual inspector for every asset type (3D
   preview with wireframe for meshes, channel/mip/histogram view for textures, timeline with curve
   editor for animations, waveform and spectrum for audio, lit sphere for materials) as the sole
   interface for viewing and editing assets, with no text or code fallback.
   - **Rationale:** Visual-only asset inspection enforces the no-code principle and ensures all
     asset types are presented in their native visual form.
   - **Verification:** Open one asset of each type (mesh, texture, animation, audio, material) and
     confirm the correct visual inspector is displayed; confirm no raw text or code view is
     available for any asset type.
8. **R-12.7.8** — The engine **SHALL** track binary assets via Git LFS with a custom merge driver
   that invokes the engine's structural merge system (R-12.7.4) for three-way merges, support
   optional lock-before-edit per asset type, and display lock owner, modified state, and merge
   conflict presence in the asset browser.
   - **Rationale:** Git LFS with a structural merge driver combines standard version control tooling
     with engine-aware binary merging, giving teams the choice between optimistic and pessimistic
     locking workflows.
   - **Verification:** Commit a binary asset via Git LFS; merge two branches with conflicting
     changes and confirm the custom merge driver invokes structural merge; enable lock-before-edit
     on an asset type and confirm the asset browser shows lock owner and modified state.
