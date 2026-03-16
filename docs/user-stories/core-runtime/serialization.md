# Serialization User Stories

## Binary Serialization

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-1.4.1 | engine developer (P-26) | a compact binary serialization format with fixed-size headers, type ID tags, and tightly packed payloads, so that asset baking, save files, and network replication payloads are optimized for throughput | Fixed-size headers and type ID tags<br>Tightly packed payloads<br>Little-endian byte order on all platforms | F-1.4.1 | R-1.4.1 |
| US-1.4.2 | game developer (P-15) | binary serialization that produces compact payloads for network replication, so that component state is transmitted with minimal bandwidth overhead between server and client | Compact encoding reduces bandwidth usage<br>Serialization throughput sufficient for 60Hz replication<br>Deterministic output for identical input data | F-1.4.1 | R-1.4.1 |
| US-1.4.3 | engine tester (P-27) | to benchmark binary serialization and deserialization throughput for typical component sets, so that I can verify the format meets performance targets for 60Hz network replication and asset loading | Serialization throughput measured in MB/s<br>Deserialization throughput measured in MB/s<br>Meets target for 60Hz replication of typical entity sets | F-1.4.1 | R-1.4.1 |
| US-1.4.4 | engine developer (P-26) | zero-copy deserialization of immutable binary blobs by memory-mapping asset files and interpreting pointer-offset tables in place, so that large static datasets load with zero deserialization cost | Memory-mapped asset files interpreted in place<br>Pointer-offset tables for data access<br>Zero allocation during deserialization | F-1.4.2 | R-1.4.2 |
| US-1.4.5 | engine tester (P-27) | to verify that zero-copy deserialization of memory-mapped files produces data identical to standard deserialization, so that the zero-copy path is functionally equivalent to the allocation-based path | Memory-mapped data matches standard deserialization output<br>Platform-native mmap (POSIX) / MapViewOfFile (Windows)<br>No corruption on concurrent read access | F-1.4.2 | R-1.4.2 |

## Text Serialization

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-1.4.6 | game developer (P-15) | reflection-driven text serialization in human-readable formats (RON, JSON, TOML) with guaranteed round-trip fidelity, so that I can hand-edit configuration files and inspect serialized data during debugging | Driven entirely by reflection, no custom code per type<br>Round-trip fidelity: deserialize(serialize(v)) == v<br>Supports RON, JSON, and TOML formats | F-1.4.3 | R-1.4.3 |
| US-1.4.7 | designer (P-5) | to inspect serialized asset data in a human-readable format within the visual editor, so that I can understand and debug asset content without external tools | Asset data viewable as human-readable text<br>All types render through reflection without custom code<br>Nested structures displayed with indentation | F-1.4.3 | R-1.4.3 |

## Schema Versioning

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-1.4.8 | game developer (P-15) | serialized types tagged with semantic versions that trigger migration on mismatch, so that save files from older game versions are upgraded gracefully rather than causing hard failures | Semantic version embedded in serialized headers<br>Version checked during deserialization<br>Mismatch triggers migration, not failure | F-1.4.4 | R-1.4.4 |
| US-1.4.9 | player (P-23) | my save files from previous game versions to load correctly after an update, so that I never lose progress due to a game patch | Save files from any prior version loadable<br>All saved data preserved or migrated correctly<br>No data loss during version migration | F-1.4.4, F-1.4.5, F-1.4.5 | R-1.4.4, R-1.4.5, R-1.4.5 |

## Migration

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-1.4.10 | game developer (P-15) | a migration registry that chains version-to-version transforms during deserialization, so that data from any historical version can be brought up to date without manual conversion scripts | Migration functions registered per type per version pair<br>Migrations chain automatically (v1 -> v2 -> v3)<br>Migrations operate on dynamic values (removed fields accessible) | F-1.4.5 | R-1.4.5 |
| US-1.4.11 | engine tester (P-27) | to verify that migration chains from any historical version to current produce correct data, so that no version path produces corrupted or incomplete results | v1 -> current produces correct data<br>Every registered version pair tested<br>Removed fields handled correctly during migration | F-1.4.5 | R-1.4.5 |

## Asset Serialization

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-1.4.12 | game developer (P-15) | asset handles serialized as stable asset IDs that resolve through the asset system on deserialization, so that scenes and prefabs reference shared assets without duplicating data or creating dangling pointers | Asset handles stored as UUID or path-based IDs<br>References resolved through asset system on load<br>Async loading triggered for non-resident assets | F-1.4.6 | R-1.4.6 |
| US-1.4.13 | engine tester (P-27) | to verify that all asset references in serialized scenes resolve correctly on deserialization and that missing assets are reported, so that no dangling references silently break scene loading | All referenced assets resolve on load<br>Missing assets produce clear error message<br>Async-loaded references available before first use | F-1.4.6 | R-1.4.6 |

## Scene Serialization

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-1.4.14 | game developer (P-15) | to serialize and deserialize entire ECS worlds including entities, components, hierarchy, and resources with entity ID remapping, so that save/load, level streaming, and server migration work with full state fidelity | Full world state serialized (entities, components, hierarchy)<br>Entity ID remapping on deserialization<br>Resources included in scene format | F-1.4.7 | R-1.4.7 |
| US-1.4.15 | player (P-23) | my game progress saved and loaded faithfully, so that I can resume exactly where I left off without losing any state, inventory, or quest progress | All gameplay state preserved across save/load<br>Entity hierarchies reconstructed correctly<br>No state loss for any component type | F-1.4.7 | R-1.4.7 |
| US-1.4.16 | engine developer (P-26) | to implement streaming scene deserialization with configurable staging buffers to avoid memory spikes, so that scene loading works within mobile's memory constraints (1 MB staging buffer, 64 MB max scene) | Streaming deserialization with 1 MB staging buffer on mobile<br>No memory spikes during scene load<br>Chunked serialization for large scenes | F-1.4.7 | R-1.4.7 |
| US-1.4.17 | engine tester (P-27) | to verify that serializing and deserializing a complex ECS world produces identical entity state, hierarchy, and resources, so that save/load and migration preserve complete world fidelity | Entity count identical before and after round-trip<br>All component values match after round-trip<br>Hierarchy relationships preserved exactly<br>Resources restored to correct values | F-1.4.7 | R-1.4.7 |

## Mixed-Format Serialization

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-1.4.18 | game developer (P-15) | to serialize a scene so that transforms and component metadata are stored in a human-readable RON file while mesh vertex data is stored in a binary companion `.bin` file, so that I can diff and review scene metadata in version control while keeping bulk data efficient | Text file contains transforms and metadata inline<br>Mesh data stored in binary companion via `$binary` ref<br>Both files written atomically | F-1.4.8 | R-1.4.8, R-1.4.8a |
| US-1.4.19 | game developer (P-15) | to deserialize a mixed-format scene file and have binary references resolved automatically, so that my loading code sees a unified data view without handling text vs. binary separately | Binary references resolved during deserialization<br>Caller receives unified data view<br>Missing companion file produces clear error | F-1.4.8 | R-1.4.8 |
| US-1.4.20 | game developer (P-15) | scene metadata stored in human-readable text so that version control diffs show meaningful changes to transforms, component configs, and entity hierarchy, so that code reviews catch unintended scene modifications | Text scene files produce readable VCS diffs<br>Bulk data changes isolated to binary companion<br>Metadata changes visible without binary diff tools | F-1.4.8, F-1.4.8, F-1.4.9 | R-1.4.8, R-1.4.8, R-1.4.9 |
| US-1.4.21 | game developer (P-15) | to mark specific fields with `#[binary]` so the mixed-format serializer stores them in the binary companion file, so that large data (vertex buffers, heightmaps) stays in binary while small fields remain readable | `#[binary]` fields stored in companion file<br>Non-binary fields remain inline in text file<br>`#[binary(compress = "lz4")]` compresses the blob | F-1.4.10 | R-1.4.10 |
| US-1.4.22 | engine developer (P-26) | multiple text asset files to reference the same binary companion file, so that shared bulk data (e.g., a common terrain heightmap) is stored once and deduplicated via content hashing | Multiple text files reference one companion binary<br>Content-addressable deduplication via hash<br>No duplicate blob storage for identical content | F-1.4.9 | R-1.4.9 |
| US-1.4.23 | engine developer (P-26) | to append new blobs to an existing binary companion file without rewriting the entire file, so that incremental asset builds are fast and proportional to changed content | New blobs appended without rewriting existing data<br>TOC updated to include new entries<br>Existing blobs remain readable after append | F-1.4.9 | R-1.4.9a |
| US-1.4.24 | engine tester (P-27) | to verify that serializing data in mixed format and deserializing it produces identical values for both the text-inline and binary-companion portions, so that the mixed format is lossless | Text-inline fields match after round-trip<br>Binary-companion fields match after round-trip<br>Compressed blobs decompress correctly | F-1.4.8, F-1.4.8, F-1.4.9, F-1.4.9 | R-1.4.8, R-1.4.8, R-1.4.9, R-1.4.9 |
| US-1.4.25 | engine tester (P-27) | to verify that a simulated crash during mixed-format write does not leave partial files at the final paths, so that asset integrity is maintained even under failure | Interrupted write leaves no partial files<br>Successful write leaves both files consistent<br>Temporary files cleaned up on failure | F-1.4.8 | R-1.4.8a |
