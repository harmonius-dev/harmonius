# R-1.4 — Serialization Requirements

## Binary Serialization

| ID       | Derived From                                            |
|----------|---------------------------------------------------------|
| R-1.4.1  | [F-1.4.1](../../features/core-runtime/serialization.md) |
| R-1.4.1a | [F-1.4.1](../../features/core-runtime/serialization.md) |
| R-1.4.2  | [F-1.4.2](../../features/core-runtime/serialization.md) |

1. **R-1.4.1** — The engine **SHALL** serialize and deserialize component data, resources, and
   scenes into a compact binary format using fixed-size headers, type ID tags, and tightly packed
   payloads, with little-endian byte order on all platforms.
   - **Rationale:** Binary serialization is the primary format for asset baking, save files, and
     network replication where throughput matters more than readability.
   - **Verification:** Unit test: serialize a struct with mixed field types (integers, floats,
     strings, nested structs) to binary. Deserialize and verify field-by-field equality. Benchmark:
     serialize and deserialize 100,000 component instances and verify throughput exceeds 500 MB/s.
2. **R-1.4.1a** — Binary serialization **SHALL** achieve at least 500 MB/s throughput for bulk
   component data. Deserialization of malformed or truncated data **SHALL** return a typed error
   describing the failure point (byte offset, expected type, actual data) rather than panicking or
   producing undefined behavior.
   - **Rationale:** Network payloads and save files may be corrupted; robust error handling prevents
     crashes from untrusted data.
   - **Verification:** Benchmark: serialize and deserialize 100,000 components and verify
     throughput. Feed truncated binary data to the deserializer and verify a descriptive error is
     returned with the failure byte offset.
3. **R-1.4.2** — The engine **SHALL** support zero-copy deserialization of immutable binary blobs
   via memory-mapping, interpreting pointer-offset tables in place without copying data into
   separate allocations.
   - **Rationale:** Zero-copy deserialization eliminates load-time overhead for large static
     datasets like terrain heightmaps and navigation meshes.
   - **Verification:** Benchmark: memory-map a 256 MB asset file and access fields via zero-copy
     deserialization. Verify no allocation occurs beyond the mmap itself. Verify access latency is
     under 1 microsecond for individual field reads.

## Text Serialization

| ID      | Derived From                                            |
|---------|---------------------------------------------------------|
| R-1.4.3 | [F-1.4.3](../../features/core-runtime/serialization.md) |

1. **R-1.4.3** — The engine **SHALL** serialize and deserialize data in a human-readable text format
   driven entirely by the reflection system, guaranteeing round-trip fidelity where deserializing a
   serialized value produces an identical value.
   - **Rationale:** Text formats are essential for configuration files, debug inspection, and
     hand-authored content that developers need to read and edit.
   - **Verification:** Unit test: serialize a complex struct (nested structs, enums, collections) to
     text format. Deserialize it and verify bit-exact equality with the original. Test with edge
     cases: empty collections, optional fields, Unicode strings.

## Schema Versioning

| ID      | Derived From                                            |
|---------|---------------------------------------------------------|
| R-1.4.4 | [F-1.4.4](../../features/core-runtime/serialization.md) |

1. **R-1.4.4** — The engine **SHALL** embed a semantic version tag in every serialized type header
   and trigger data migration (not hard failure) when the stored version differs from the current
   runtime version.
   - **Rationale:** Version mismatches between save files and updated game builds must be handled
     gracefully to avoid data loss.
   - **Verification:** Unit test: serialize a type at version 1.0.0, update the type to version
     1.1.0 with a new field. Deserialize the old data and verify migration is triggered rather than
     a deserialization error. Verify the new field receives its default value.

## Migration

| ID      | Derived From                                            |
|---------|---------------------------------------------------------|
| R-1.4.5 | [F-1.4.5](../../features/core-runtime/serialization.md) |

1. **R-1.4.5** — The engine **SHALL** provide a migration registry where version-to-version
   transform functions are registered, chaining migrations during deserialization to bring data from
   any stored version to the current version, operating on dynamic values to access removed fields.
   - **Rationale:** Chained migrations ensure forward compatibility across multiple release versions
     without requiring manual conversion scripts.
   - **Verification:** Unit test: register migrations v1->v2 (rename field), v2->v3 (add field,
     remove field). Serialize at v1, deserialize at v3. Verify the chain executes both migrations in
     order and the final result has the correct fields and values.

## Asset Serialization

| ID       | Derived From                                            |
|----------|---------------------------------------------------------|
| R-1.4.6  | [F-1.4.6](../../features/core-runtime/serialization.md) |
| R-1.4.5a | [F-1.4.5](../../features/core-runtime/serialization.md) |

1. **R-1.4.6** — The engine **SHALL** serialize asset handles as stable asset IDs (UUID or path)
   rather than raw pointers, resolving references through the asset system during deserialization
   and triggering async loading for non-resident assets.
   - **Rationale:** Stable asset IDs prevent dangling references and enable scenes to reference
     shared assets without data duplication.
   - **Verification:** Integration test: serialize a component containing an asset handle.
     Deserialize it in a fresh world. Verify the asset ID is preserved and the asset system receives
     a load request for the referenced asset. Verify that loading a scene with missing asset
     references reports an error rather than crashing.
2. **R-1.4.5a** — The migration pipeline **SHALL** support chains of at least 50 version steps. If a
   migration function fails or no migration path exists between the stored and current versions, the
   engine **SHALL** return a diagnostic error identifying the source version, target version, and
   the missing migration step.
   - **Rationale:** Long-lived games accumulate many schema versions; clear migration errors prevent
     silent data corruption from broken upgrade paths.
   - **Verification:** Register a 50-step migration chain and verify end-to-end migration succeeds.
     Remove one step from the chain and verify the error identifies the gap.

## Scene Serialization

| ID      | Derived From                                            |
|---------|---------------------------------------------------------|
| R-1.4.7 | [F-1.4.7](../../features/core-runtime/serialization.md) |

1. **R-1.4.7** — The engine **SHALL** serialize an entire ECS world — entities, components,
   hierarchy, and resources — into a portable scene format, with entity ID remapping during
   deserialization to avoid collisions when merging scenes.
   - **Rationale:** Full scene serialization powers save/load, level streaming, and server-to-server
     world migration.
   - **Verification:** Integration test: populate a world with 1,000 entities including hierarchy,
     relationships, and resources. Serialize and deserialize into a new world. Verify entity count,
     hierarchy structure, component values, and resource values all match. Deserialize the same
     scene twice into one world and verify entity IDs are remapped without collisions.

## Mixed-Format Serialization

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-1.4.8  | [F-1.4.8](../../features/core-runtime/serialization.md)  |
| R-1.4.8a | [F-1.4.8](../../features/core-runtime/serialization.md)  |
| R-1.4.9  | [F-1.4.9](../../features/core-runtime/serialization.md)  |
| R-1.4.9a | [F-1.4.9](../../features/core-runtime/serialization.md)  |
| R-1.4.10 | [F-1.4.10](../../features/core-runtime/serialization.md) |

1. **R-1.4.8** — The engine **SHALL** support a mixed serialization mode where a human-readable text
   file (RON or TOML) references bulk data stored in a binary companion `.bin` file via
   `$binary("path", offset, len)` directives. The serializer **SHALL** write both files atomically.
   The deserializer **SHALL** resolve binary references transparently, presenting a unified data
   view to the caller.
   - **Rationale:** Keeping metadata in text and bulk data in binary gives the best of both worlds:
     human-readable, diff-friendly structure with efficient binary storage for large payloads.
   - **Verification:** Integration test: serialize a scene containing a mesh component (vertices as
     bulk data) and a transform component (small structured data). Verify the text file contains the
     transform inline and a `$binary` reference for the mesh vertices. Verify the binary companion
     contains the vertex data at the referenced offset and length. Deserialize and verify the
     reconstructed scene matches the original.
2. **R-1.4.8a** — When writing mixed-format files, the engine **SHALL** write to temporary files and
   rename atomically, ensuring that a crash during serialization never leaves a partially-written
   text file referencing a missing or incomplete binary companion.
   - **Rationale:** Partial writes during crash would corrupt asset files; atomic rename ensures
     consistency.
   - **Verification:** Unit test: simulate a write interrupted after the text file but before the
     binary companion. Verify neither file exists at the final path. Verify a successful write
     leaves both files in a consistent state.
3. **R-1.4.9** — The engine **SHALL** define a binary companion file format (`.bin`) with a header
   containing a magic number, format version, and a table of contents listing named blobs with
   offset, length, compression method, and content hash. Blobs **SHALL** be stored contiguously
   after the TOC. The format **SHALL** support optional per-blob LZ4 compression and
   content-addressable deduplication via hash comparison.
   - **Rationale:** A structured binary container with TOC enables efficient random access to
     individual blobs, deduplication across assets, and incremental builds.
   - **Verification:** Unit test: write three named blobs (one compressed, two uncompressed) to a
     companion file. Read each by name and verify data integrity via content hash. Write a duplicate
     blob and verify deduplication (no additional storage consumed). Verify the TOC lists all
     entries with correct offsets and lengths.
4. **R-1.4.9a** — The binary companion format **SHALL** support appending new blobs without
   rewriting existing data. The TOC **SHALL** be updatable in place or relocatable to the end of the
   file to enable append operations during incremental asset builds.
   - **Rationale:** Incremental asset builds must add new blobs without reprocessing existing ones,
     keeping build times proportional to changed content.
   - **Verification:** Unit test: create a companion file with two blobs. Append a third blob.
     Verify all three blobs are readable and the first two were not rewritten (check file
     modification via byte comparison). Verify the TOC includes all three entries.
5. **R-1.4.10** — The engine **SHALL** support a `#[binary]` field attribute that instructs the
   mixed-format serializer to store the field's data in the binary companion rather than inline in
   the text file. The attribute **SHALL** accept optional parameters: `compress` (compression
   algorithm, default none) and `align` (byte alignment, default 1). During text serialization,
   `#[binary]` fields **SHALL** emit a `$binary` reference. During deserialization, references
   **SHALL** be resolved from the companion transparently.
   - **Rationale:** Per-field binary annotation gives type authors control over which data goes to
     binary companions, keeping small structured data readable in text while offloading large
     buffers.
   - **Verification:** Unit test: define a struct with a `#[binary(compress = "lz4")]` `Vec<u8>`
     field and a regular `String` field. Serialize in mixed mode. Verify the text file contains the
     string inline and a `$binary` reference for the byte vector. Verify the companion file contains
     the LZ4-compressed bytes. Deserialize and verify both fields match the original.
