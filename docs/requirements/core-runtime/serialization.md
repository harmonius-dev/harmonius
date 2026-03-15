# R-1.4 — Serialization Requirements

## Binary Serialization

### R-1.4.1 Compact Binary Serialization Format

The engine **SHALL** serialize and deserialize component data, resources, and scenes into a compact
binary format using fixed-size headers, type ID tags, and tightly packed payloads, with little-endian
byte order on all platforms.

- **Derived from:** [F-1.4.1](../../features/core-runtime/serialization.md)
- **Rationale:** Binary serialization is the primary format for asset baking, save files, and network
  replication where throughput matters more than readability.
- **Verification:** Unit test: serialize a struct with mixed field types (integers, floats, strings,
  nested structs) to binary. Deserialize and verify field-by-field equality. Benchmark: serialize
  and deserialize 100,000 component instances and verify throughput exceeds 500 MB/s.

### R-1.4.1a Binary Serialization Throughput and Error Handling

Binary serialization **SHALL** achieve at least 500 MB/s throughput for bulk component data.
Deserialization of malformed or truncated data **SHALL** return a typed error describing the failure
point (byte offset, expected type, actual data) rather than panicking or producing undefined behavior.

- **Derived from:** [F-1.4.1](../../features/core-runtime/serialization.md)
- **Rationale:** Network payloads and save files may be corrupted; robust error handling prevents
  crashes from untrusted data.
- **Verification:** Benchmark: serialize and deserialize 100,000 components and verify throughput.
  Feed truncated binary data to the deserializer and verify a descriptive error is returned with the
  failure byte offset.

### R-1.4.2 Zero-Copy Deserialization for Read-Only Data

The engine **SHALL** support zero-copy deserialization of immutable binary blobs via memory-mapping,
interpreting pointer-offset tables in place without copying data into separate allocations.

- **Derived from:** [F-1.4.2](../../features/core-runtime/serialization.md)
- **Rationale:** Zero-copy deserialization eliminates load-time overhead for large static datasets like
  terrain heightmaps and navigation meshes.
- **Verification:** Benchmark: memory-map a 256 MB asset file and access fields via zero-copy
  deserialization. Verify no allocation occurs beyond the mmap itself. Verify access latency is
  under 1 microsecond for individual field reads.

## Text Serialization

### R-1.4.3 Human-Readable Text Serialization

The engine **SHALL** serialize and deserialize data in a human-readable text format driven entirely by
the reflection system, guaranteeing round-trip fidelity where deserializing a serialized value produces
an identical value.

- **Derived from:** [F-1.4.3](../../features/core-runtime/serialization.md)
- **Rationale:** Text formats are essential for configuration files, debug inspection, and hand-authored
  content that developers need to read and edit.
- **Verification:** Unit test: serialize a complex struct (nested structs, enums, collections) to text
  format. Deserialize it and verify bit-exact equality with the original. Test with edge cases:
  empty collections, optional fields, Unicode strings.

## Schema Versioning

### R-1.4.4 Schema Versioning with Semantic Version Tags

The engine **SHALL** embed a semantic version tag in every serialized type header and trigger data
migration (not hard failure) when the stored version differs from the current runtime version.

- **Derived from:** [F-1.4.4](../../features/core-runtime/serialization.md)
- **Rationale:** Version mismatches between save files and updated game builds must be handled gracefully
  to avoid data loss.
- **Verification:** Unit test: serialize a type at version 1.0.0, update the type to version 1.1.0 with
  a new field. Deserialize the old data and verify migration is triggered rather than a deserialization
  error. Verify the new field receives its default value.

## Migration

### R-1.4.5 Data Migration Pipeline

The engine **SHALL** provide a migration registry where version-to-version transform functions are
registered, chaining migrations during deserialization to bring data from any stored version to the
current version, operating on dynamic values to access removed fields.

- **Derived from:** [F-1.4.5](../../features/core-runtime/serialization.md)
- **Rationale:** Chained migrations ensure forward compatibility across multiple release versions
  without requiring manual conversion scripts.
- **Verification:** Unit test: register migrations v1->v2 (rename field), v2->v3 (add field, remove
  field). Serialize at v1, deserialize at v3. Verify the chain executes both migrations in order and
  the final result has the correct fields and values.

## Asset Serialization

### R-1.4.6 Asset-Aware Serialization with Handle Resolution

The engine **SHALL** serialize asset handles as stable asset IDs (UUID or path) rather than raw pointers,
resolving references through the asset system during deserialization and triggering async loading for
non-resident assets.

- **Derived from:** [F-1.4.6](../../features/core-runtime/serialization.md)
- **Rationale:** Stable asset IDs prevent dangling references and enable scenes to reference shared
  assets without data duplication.
- **Verification:** Integration test: serialize a component containing an asset handle. Deserialize it
  in a fresh world. Verify the asset ID is preserved and the asset system receives a load request for
  the referenced asset. Verify that loading a scene with missing asset references reports an error
  rather than crashing.

### R-1.4.5a Migration Chain Depth and Error Handling

The migration pipeline **SHALL** support chains of at least 50 version steps. If a migration function
fails or no migration path exists between the stored and current versions, the engine **SHALL** return
a diagnostic error identifying the source version, target version, and the missing migration step.

- **Derived from:** [F-1.4.5](../../features/core-runtime/serialization.md)
- **Rationale:** Long-lived games accumulate many schema versions; clear migration errors prevent
  silent data corruption from broken upgrade paths.
- **Verification:** Register a 50-step migration chain and verify end-to-end migration succeeds.
  Remove one step from the chain and verify the error identifies the gap.

## Scene Serialization

### R-1.4.7 Full Scene Serialization and Deserialization

The engine **SHALL** serialize an entire ECS world — entities, components, hierarchy, and resources —
into a portable scene format, with entity ID remapping during deserialization to avoid collisions when
merging scenes.

- **Derived from:** [F-1.4.7](../../features/core-runtime/serialization.md)
- **Rationale:** Full scene serialization powers save/load, level streaming, and server-to-server world
  migration.
- **Verification:** Integration test: populate a world with 1,000 entities including hierarchy,
  relationships, and resources. Serialize and deserialize into a new world. Verify entity count,
  hierarchy structure, component values, and resource values all match. Deserialize the same scene
  twice into one world and verify entity IDs are remapped without collisions.
