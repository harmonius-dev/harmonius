# R-1.4 — Serialization Requirements

## Binary Serialization

1. **R-1.4.1** — The engine **SHALL** serialize and deserialize component data, resources, and
   scenes into a compact binary format using fixed-size headers, type ID tags, and tightly packed
   payloads, with little-endian byte order on all platforms. Throughput **SHALL** exceed 500 MB/s
   for bulk component data.
   - **Rationale:** Binary serialization is the primary format for asset baking, save files, and
     network replication.
   - **Verification:** Serialize struct with mixed types to binary; deserialize; verify field
     equality. Benchmark 100,000 components; verify 500 MB/s throughput.
2. **R-1.4.2** — Deserialization of malformed or truncated data **SHALL** return a typed error
   describing the failure point (byte offset, expected type, actual data) rather than panicking or
   producing undefined behavior.
   - **Rationale:** Network payloads and save files may be corrupted; robust error handling prevents
     crashes.
   - **Verification:** Feed truncated binary to deserializer; verify descriptive error with failure
     byte offset.
3. **R-1.4.3** — The engine **SHALL** support zero-copy deserialization of immutable binary blobs
   via memory-mapping, interpreting pointer-offset tables in place without copying into separate
   allocations.
   - **Rationale:** Zero-copy eliminates load-time overhead for large static datasets.
   - **Verification:** Memory-map 256 MB asset; access via zero-copy; verify no allocation beyond
     mmap. Verify field access under 1 us.

## Text Serialization

1. **R-1.4.4** — The engine **SHALL** serialize and deserialize data in human-readable text format
   driven entirely by the reflection system, guaranteeing round-trip fidelity.
   - **Rationale:** Text formats are essential for configuration, debugging, and hand-authored
     content.
   - **Verification:** Serialize complex struct (nested, enums, collections) to text; deserialize;
     verify bit-exact equality. Test edge cases: empty collections, optional fields, Unicode
     strings.

## Schema Versioning

1. **R-1.4.5** — The engine **SHALL** embed a semantic version tag in every serialized type header
   and trigger data migration (not hard failure) when the stored version differs from the current
   runtime version.
   - **Rationale:** Version mismatches between save files and updated builds must be handled
     gracefully.
   - **Verification:** Serialize at v1.0.0; update type to v1.1.0 with new field; deserialize old
     data; verify migration triggered and new field gets default value.

## Migration

1. **R-1.4.6** — The engine **SHALL** provide a migration registry chaining version-to-version
   transforms during deserialization, operating on dynamic values to access removed fields. Chains
   of at least 50 version steps **SHALL** be supported. Missing migration steps **SHALL** produce a
   diagnostic error identifying source version, target version, and the missing step.
   - **Rationale:** Chained migrations ensure forward compatibility across multiple release
     versions.
   - **Verification:** Register v1->v2 (rename), v2->v3 (add/remove field); serialize at v1;
     deserialize at v3; verify chain executes correctly. Register 50-step chain; verify end-to-end.
     Remove one step; verify error identifies gap.

## Asset Serialization

1. **R-1.4.7** — The engine **SHALL** serialize asset handles as stable asset IDs (UUID or path)
   rather than raw pointers, resolving references through the asset system during deserialization
   and triggering async loading for non-resident assets. Missing assets **SHALL** produce a clear
   error rather than crashing.
   - **Rationale:** Stable IDs prevent dangling references and enable shared asset referencing
     without duplication.
   - **Verification:** Serialize component with asset handle; deserialize in fresh world; verify ID
     preserved and load request sent. Load scene with missing asset; verify error reported.

## Scene Serialization

1. **R-1.4.8** — The engine **SHALL** serialize an entire ECS world (entities, components,
   hierarchy, resources) into a portable scene format, with entity ID remapping during
   deserialization to avoid collisions when merging scenes. Streaming deserialization with
   configurable staging buffers **SHALL** be supported.
   - **Rationale:** Full scene serialization powers save/load, level streaming, and server
     migration.
   - **Verification:** Populate world with 1,000 entities including hierarchy and resources;
     serialize; deserialize into new world; verify all state matches. Deserialize same scene twice;
     verify no ID collisions. Verify mobile: 1 MB staging buffer, max 64 MB scene.

## Mixed-Format Serialization

1. **R-1.4.9** — The engine **SHALL** support a mixed serialization mode where a human-readable text
   file references bulk data in a binary companion .bin file via $binary directives. The serializer
   **SHALL** write both files atomically. The deserializer **SHALL** resolve references
   transparently.
   - **Rationale:** Keeping metadata in text and bulk data in binary provides readable diffs with
     efficient storage.
   - **Verification:** Serialize scene with mesh (bulk) and transform (small); verify text contains
     transform inline and $binary ref for mesh. Verify companion contains vertex data. Deserialize;
     verify match.
2. **R-1.4.10** — Atomic writes **SHALL** use temporary files and rename, ensuring a crash never
   leaves partial text referencing missing or incomplete binary companion.
   - **Rationale:** Partial writes during crash would corrupt asset files.
   - **Verification:** Simulate write interrupted after text but before binary; verify neither file
     at final path. Verify successful write leaves both consistent.
3. **R-1.4.11** — The engine **SHALL** define a binary companion format (.bin) with header (magic,
   version, TOC) listing named blobs with offset, length, compression, and content hash. The format
   **SHALL** support per-blob LZ4 compression and content-addressable deduplication. Appending new
   blobs **SHALL NOT** require rewriting existing data.
   - **Rationale:** Structured binary container enables random access, deduplication, and
     incremental builds.
   - **Verification:** Write 3 named blobs (1 compressed, 2 not); read by name; verify integrity via
     hash. Write duplicate; verify deduplication. Append new blob; verify existing unmodified.
     Verify TOC correctness.
4. **R-1.4.12** — The engine **SHALL** support a #[binary] field attribute instructing the
   mixed-format serializer to store the field in the binary companion, with optional compress and
   align parameters. Text serialization **SHALL** emit $binary reference; deserialization **SHALL**
   resolve transparently.
   - **Rationale:** Per-field annotation gives type authors control over text vs. binary storage.
   - **Verification:** Define struct with #[binary(compress = "lz4")] Vec<u8> and regular String.
     Serialize in mixed mode; verify text contains string inline and $binary ref for bytes. Verify
     companion contains LZ4 data. Deserialize; verify both fields match original.
