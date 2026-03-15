# 1.4 — Serialization

## Binary Serialization

### F-1.4.1 Compact Binary Serialization Format

Serialize and deserialize component data, resources, and scenes into a compact binary format
optimized for read/write throughput and minimal allocation. The format uses fixed-size headers, type
ID tags, and tightly packed payloads. Binary serialization is the primary format for asset baking,
save files, and network replication payloads where human readability is not required.

- **Requirements:** R-1.4.1
- **Dependencies:** F-1.3.2 (Structured Type Descriptors), F-1.3.5 (Dynamic Values)
- **Platform notes:** Binary layout uses little-endian byte order on all platforms for consistency.

### F-1.4.2 Zero-Copy Deserialization for Read-Only Data

Support zero-copy deserialization of immutable binary blobs by memory-mapping asset files and
interpreting pointer-offset tables in place. This eliminates deserialization cost for large static
datasets such as terrain heightmaps, navigation meshes, and lookup tables, reducing level load times
at MMO scale.

- **Requirements:** R-1.4.2
- **Dependencies:** F-1.4.1, F-1.7.1 (Arena Allocators)
- **Platform notes:** Requires platform-native memory mapping (mmap on POSIX, MapViewOfFile on
  Windows).

## Text Serialization

### F-1.4.3 Human-Readable Text Serialization

Serialize and deserialize data in a human-readable text format (RON, JSON, or TOML) for
configuration files, debug inspection, and hand-authored content. Text serialization is driven
entirely by the reflection system so that adding new types requires no custom serialization code.
Round-trip fidelity must be guaranteed: deserializing a serialized value must produce an identical
value.

- **Requirements:** R-1.4.3
- **Dependencies:** F-1.3.3 (Property System), F-1.3.4 (Collection Reflection)
- **Platform notes:** None

## Schema Versioning

### F-1.4.4 Schema Versioning with Semantic Version Tags

Tag every serialized type with a semantic version. The version is embedded in serialized headers and
checked during deserialization. Version mismatches trigger migration rather than hard failure,
ensuring forward compatibility when the game is updated while players hold save files or when server
and client run different builds.

- **Requirements:** R-1.4.4
- **Dependencies:** F-1.4.1, F-1.3.6 (Custom Attributes)
- **Platform notes:** None

## Migration

### F-1.4.5 Data Migration Pipeline

Provide a migration registry where version-to-version transform functions are registered for each
type. During deserialization, if the stored version differs from the current version, the pipeline
chains migrations to bring data up to date. Migrations operate on dynamic values so that removed
fields are still accessible during transformation.

- **Requirements:** R-1.4.5
- **Dependencies:** F-1.4.4, F-1.3.5 (Dynamic Values)
- **Platform notes:** None

## Asset Serialization

### F-1.4.6 Asset-Aware Serialization with Handle Resolution

Serialize and deserialize asset handles as stable asset IDs (UUID or path-based) rather than raw
pointers or indices. During deserialization, asset references are resolved through the asset system,
which triggers async loading if the referenced asset is not yet resident. This enables scenes and
prefabs to reference shared assets without duplicating data.

- **Requirements:** R-1.4.6
- **Dependencies:** F-1.4.1, F-1.4.3
- **Platform notes:** None

## Scene Serialization

### F-1.4.7 Full Scene Serialization and Deserialization

Serialize an entire ECS world — entities, components, hierarchy, and resources — into a portable
scene format. Deserialization reconstructs the world state faithfully, including entity ID remapping
to avoid collisions when merging scenes. This powers save/load, level streaming, and
server-to-server world migration.

- **Requirements:** R-1.4.7
- **Dependencies:** F-1.4.1, F-1.4.3, F-1.2.1, F-1.1.34
- **Platform notes:** Mobile: streaming deserialization with 1 MB staging buffer, max scene size 64
  MB. Switch: 2 MB staging buffer, max 256 MB. Desktop: configurable staging buffer, no scene size
  cap. Serialize in chunks to avoid memory spikes on constrained platforms.

## Mixed-Format Serialization

### F-1.4.8 Mixed-Format Serialization with Binary Companions

Support a serialization mode where a human-readable text file (RON or TOML) contains structured data
alongside references to binary companion files that store bulk data. The text file uses a
`$binary("path", offset, len)` directive to reference byte ranges within a companion `.bin` file
stored alongside it. This enables scenes, prefabs, and assets to keep metadata human-readable and
diff-friendly while storing large payloads (meshes, textures, audio clips, navigation meshes) in
efficient binary form. The serializer writes both the text and binary files atomically. The
deserializer resolves binary references transparently, presenting a unified data view to the caller.

- **Requirements:** R-1.4.8
- **Dependencies:** F-1.4.1, F-1.4.3, F-1.3.6
- **Platform notes:** None

### F-1.4.9 Binary Companion File Format

Define a binary companion file format (`.bin`) designed to sit alongside a textual asset file. The
format uses a header containing a magic number, format version, and a table of contents (TOC)
listing named blobs with their offset, length, compression, and content hash. Blobs are stored
contiguously after the TOC. The format supports optional per-blob LZ4 compression and
content-addressable deduplication via hash comparison. Multiple text files may reference the same
companion binary to share bulk data. The format is append-friendly for incremental asset builds.

- **Requirements:** R-1.4.9
- **Dependencies:** F-1.4.8, F-1.4.2
- **Platform notes:** Binary layout uses little-endian byte order on all platforms, consistent with
  F-1.4.1.

### F-1.4.10 Reflection-Driven Binary Reference Attributes

Extend the reflection attribute system with a `#[binary]` field attribute that instructs the
mixed-format serializer to store the annotated field's data in the binary companion file rather than
inline in the text file. The attribute accepts optional parameters for compression
(`#[binary(compress = "lz4")]`) and alignment (`#[binary(align = 16)]`). During text serialization,
fields marked `#[binary]` emit a `$binary` reference directive. During deserialization, the
reference is resolved from the companion file transparently.

- **Requirements:** R-1.4.10
- **Dependencies:** F-1.4.8, F-1.3.6, F-1.3.8
- **Platform notes:** None
