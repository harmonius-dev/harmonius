# 1.4 — Serialization

## Binary Serialization

### F-1.4.1 Compact Binary Serialization Format

Serialize and deserialize component data, resources, and scenes into a compact binary format optimized for
read/write throughput and minimal allocation. The format uses fixed-size headers, type ID tags, and tightly packed
payloads. Binary serialization is the primary format for asset baking, save files, and network replication payloads
where human readability is not required.

- **Requirements:** R-1.4.1
- **Dependencies:** F-1.3.2 (Structured Type Descriptors), F-1.3.5 (Dynamic Values)
- **Platform notes:** Binary layout uses little-endian byte order on all platforms for consistency.

### F-1.4.2 Zero-Copy Deserialization for Read-Only Data

Support zero-copy deserialization of immutable binary blobs by memory-mapping asset files and interpreting
pointer-offset tables in place. This eliminates deserialization cost for large static datasets such as terrain
heightmaps, navigation meshes, and lookup tables, reducing level load times at MMO scale.

- **Requirements:** R-1.4.2
- **Dependencies:** F-1.4.1, F-1.7.1 (Arena Allocators)
- **Platform notes:** Requires platform-native memory mapping (mmap on POSIX, MapViewOfFile on Windows).

## Text Serialization

### F-1.4.3 Human-Readable Text Serialization

Serialize and deserialize data in a human-readable text format (RON, JSON, or TOML) for configuration files,
debug inspection, and hand-authored content. Text serialization is driven entirely by the reflection system so that
adding new types requires no custom serialization code. Round-trip fidelity must be guaranteed: deserializing a
serialized value must produce an identical value.

- **Requirements:** R-1.4.3
- **Dependencies:** F-1.3.3 (Property System), F-1.3.4 (Collection Reflection)
- **Platform notes:** None

## Schema Versioning

### F-1.4.4 Schema Versioning with Semantic Version Tags

Tag every serialized type with a semantic version. The version is embedded in serialized headers and checked during
deserialization. Version mismatches trigger migration rather than hard failure, ensuring forward compatibility when
the game is updated while players hold save files or when server and client run different builds.

- **Requirements:** R-1.4.4
- **Dependencies:** F-1.4.1, F-1.3.6 (Custom Attributes)
- **Platform notes:** None

## Migration

### F-1.4.5 Data Migration Pipeline

Provide a migration registry where version-to-version transform functions are registered for each type. During
deserialization, if the stored version differs from the current version, the pipeline chains migrations to bring
data up to date. Migrations operate on dynamic values so that removed fields are still accessible during
transformation.

- **Requirements:** R-1.4.5
- **Dependencies:** F-1.4.4, F-1.3.5 (Dynamic Values)
- **Platform notes:** None

## Asset Serialization

### F-1.4.6 Asset-Aware Serialization with Handle Resolution

Serialize and deserialize asset handles as stable asset IDs (UUID or path-based) rather than raw pointers or
indices. During deserialization, asset references are resolved through the asset system, which triggers async
loading if the referenced asset is not yet resident. This enables scenes and prefabs to reference shared assets
without duplicating data.

- **Requirements:** R-1.4.6
- **Dependencies:** F-1.4.1, F-1.4.3
- **Platform notes:** None

## Scene Serialization

### F-1.4.7 Full Scene Serialization and Deserialization

Serialize an entire ECS world — entities, components, hierarchy, and resources — into a portable scene format.
Deserialization reconstructs the world state faithfully, including entity ID remapping to avoid collisions when
merging scenes. This powers save/load, level streaming, and server-to-server world migration in MMO deployments.

- **Requirements:** R-1.4.7
- **Dependencies:** F-1.4.1, F-1.4.3, F-1.2.1 (Scene Hierarchy), F-1.1.10 (Multiple Worlds)
- **Platform notes:** None
