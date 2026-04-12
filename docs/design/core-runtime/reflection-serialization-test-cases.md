# Reflection and Serialization Test Cases

Companion test cases for [reflection-serialization.md](reflection-serialization.md).

## Unit Tests

### TC-1.3.1.1 Registry 10K Types

| # | Requirement |
|---|-------------|
| 1 | R-1.3.1a    |
| 2 | R-1.3.1a    |
| 3 | R-1.3.1a    |

1. **#1** — Register 10,000 types
   - **Expected:** All registered without error
2. **#2** — Lookup each by TypeId
   - **Expected:** O(1) per lookup, all found
3. **#3** — Lookup each by name
   - **Expected:** O(1) per lookup, all found

### TC-1.3.1.2 Registry Duplicate ID Error

| # | Requirement |
|---|-------------|
| 1 | R-1.3.1a    |
| 2 | R-1.3.1a    |

1. **#1** — Register type A with TypeId X
   - **Expected:** Success
2. **#2** — Register type B with same TypeId X
   - **Expected:** Error naming both A and B

### TC-1.3.1.3 Registry Freeze Rejects Insert

| # | Requirement |
|---|-------------|
| 1 | R-1.3.1     |
| 2 | R-1.3.1     |

1. **#1** — Call `freeze()` on registry
   - **Expected:** Frozen
2. **#2** — Call `register::<T>()`
   - **Expected:** Panics

### TC-1.3.2.1 Type Descriptor Layout

| # | Requirement |
|---|-------------|
| 1 | R-1.3.2     |
| 2 | R-1.3.2     |

1. **#1** — Register struct with fields (u32, f64, bool)
   - **Expected:** Descriptor created
2. **#2** — Query size, alignment, field count, offsets
   - **Expected:** Match `std::mem::size_of`, `align_of`, and `offset_of`

### TC-1.3.2.2 Dynamic Move Drop Clone

| # | Requirement |
|---|-------------|
| 1 | R-1.3.2     |
| 2 | R-1.3.2     |
| 3 | R-1.3.2     |

1. **#1** — Construct type-erased instance via `default_fn`
   - **Expected:** Instance created
2. **#2** — Clone instance
   - **Expected:** Two independent copies
3. **#3** — Drop both
   - **Expected:** Custom allocator reports zero leaks

### TC-1.3.3.1 Property Path Nested

| # | Requirement |
|---|-------------|
| 1 | R-1.3.3     |
| 2 | R-1.3.3     |

1. **#1** — Write 42 to `outer.inner.value` via path API
   - **Expected:** Write succeeds
2. **#2** — Read `outer.inner.value`
   - **Expected:** Returns 42

### TC-1.3.3.2 Property Path Invalid

| # | Requirement |
|---|-------------|
| 1 | R-1.3.3a    |

1. **#1** — Access `outer.nonexistent.field`
   - **Expected:** Error containing "nonexistent" and parent type

### TC-1.3.3.3 Property Path Performance

| # | Requirement |
|---|-------------|
| 1 | R-1.3.3a    |
| 2 | R-1.3.3a    |

1. **#1** — Access 6-segment path 100,000 times
   - **Expected:** All accesses succeed
2. **#2** — Measure per-access time
   - **Expected:** < 500 ns

### TC-1.3.4.1 Collection Vec Reflect

| # | Requirement |
|---|-------------|
| 1 | R-1.3.4     |
| 2 | R-1.3.4     |
| 3 | R-1.3.4     |
| 4 | R-1.3.4     |

1. **#1** — Insert 3 elements into Vec via reflection
   - **Expected:** Vec length = 3
2. **#2** — Remove element at index 1
   - **Expected:** Vec length = 2, correct values
3. **#3** — Iterate via reflection
   - **Expected:** All remaining elements visited
4. **#4** — Index access `[0]`
   - **Expected:** Returns first element

### TC-1.3.4.2 Collection Map Reflect

| # | Requirement |
|---|-------------|
| 1 | R-1.3.4     |
| 2 | R-1.3.4     |
| 3 | R-1.3.4     |

1. **#1** — Insert ("a", 1), ("b", 2) via reflection
   - **Expected:** Map size = 2
2. **#2** — Get "a"
   - **Expected:** Returns 1
3. **#3** — Remove "b"
   - **Expected:** Map size = 1

### TC-1.3.5.1 DynamicValue Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-1.3.5     |
| 2 | R-1.3.5     |

1. **#1** — Typed struct -> `DynamicValue`
   - **Expected:** DynamicValue created
2. **#2** — `FromReflect` back to typed
   - **Expected:** Identical to original

### TC-1.3.5.2 Diff Minimal Patch

| # | Requirement |
|---|-------------|
| 1 | R-1.3.5     |

1. **#1** — 10-field struct, change field 3 only
   - **Expected:** Diff produces patch with 1 op

### TC-1.3.5.3 Patch Apply Correct

| # | Requirement |
|---|-------------|
| 1 | R-1.3.5     |
| 2 | R-1.3.5     |

1. **#1** — Apply SetField(field_3, 99) to struct
   - **Expected:** `struct.field_3 == 99`
2. **#2** — Other fields unchanged
   - **Expected:** All 9 other fields identical

### TC-1.3.6.1 Attributes Range Skip Rename

| # | Requirement |
|---|-------------|
| 1 | R-1.3.6     |
| 2 | R-1.3.6     |
| 3 | R-1.3.6     |

1. **#1** — Register type with `#[reflect(min=0, max=100)]`
   - **Expected:** Attribute queryable at runtime
2. **#2** — Query skip attribute on skipped field
   - **Expected:** `skip = true`
3. **#3** — Query rename attribute
   - **Expected:** Returns overridden name

### TC-1.3.7.1 Trait Registration and Resolve

| # | Requirement |
|---|-------------|
| 1 | R-1.3.7     |
| 2 | R-1.3.7     |

1. **#1** — Register Serialize trait impl for type A
   - **Expected:** Registration succeeds
2. **#2** — Resolve by TypeId, invoke serialize
   - **Expected:** Produces correct output

### TC-1.3.7.2 Trait Missing Error

| # | Requirement |
|---|-------------|
| 1 | R-1.3.7     |

1. **#1** — Query unregistered trait for type B
   - **Expected:** Error message with trait and type names

### TC-1.3.8.1 Derive Struct Reflect

| # | Requirement |
|---|-------------|
| 1 | R-1.3.8     |
| 2 | R-1.3.8     |
| 3 | R-1.3.8     |

1. **#1** — `#[derive(Reflect)]` on struct with 3 fields
   - **Expected:** Reflect impl generated
2. **#2** — Access field by name "x"
   - **Expected:** Returns correct value
3. **#3** — Access field by index 0
   - **Expected:** Returns same value as by name

### TC-1.3.8.2 Derive Enum Reflect

| # | Requirement |
|---|-------------|
| 1 | R-1.3.8     |
| 2 | R-1.3.8     |
| 3 | R-1.3.8     |

1. **#1** — `#[derive(Reflect)]` on enum with 3 variants
   - **Expected:** Reflect impl generated
2. **#2** — Query current variant name
   - **Expected:** Returns correct variant
3. **#3** — Access variant field
   - **Expected:** Returns correct data

### TC-1.3.8.3 Derive Skip Attribute

| # | Requirement |
|---|-------------|
| 1 | R-1.3.8a    |

1. **#1** — `#[reflect(skip)]` on field "secret"
   - **Expected:** Field absent from reflection

### TC-1.3.8.4 Derive Rename Attribute

| # | Requirement |
|---|-------------|
| 1 | R-1.3.8a    |

1. **#1** — `#[reflect(rename = "display_name")]` on field "x"
   - **Expected:** Field accessed as "display_name"

### TC-1.3.8.5 Derive Default Attribute

| # | Requirement |
|---|-------------|
| 1 | R-1.3.8a    |

1. **#1** — Patch struct with missing field (has default)
   - **Expected:** Missing field receives default value

### TC-1.3.9.1 Sub-trait Struct

| # | Requirement |
|---|-------------|
| 1 | R-1.3.9     |
| 2 | R-1.3.9     |

1. **#1** — Downcast struct's Reflect to ReflectStruct
   - **Expected:** Succeeds
2. **#2** — Downcast struct's Reflect to ReflectEnum
   - **Expected:** Returns None

### TC-1.3.9.2 Sub-trait Enum

| # | Requirement |
|---|-------------|
| 1 | R-1.3.9     |
| 2 | R-1.3.9     |

1. **#1** — Downcast enum's Reflect to ReflectEnum
   - **Expected:** Succeeds
2. **#2** — Downcast enum's Reflect to ReflectStruct
   - **Expected:** Returns None

### TC-1.3.9.3 Sub-trait List

| # | Requirement |
|---|-------------|
| 1 | R-1.3.9     |
| 2 | R-1.3.9     |

1. **#1** — Downcast Vec to ReflectList
   - **Expected:** Succeeds
2. **#2** — Call push, pop, len
   - **Expected:** All operations work correctly

### TC-1.3.9.4 Sub-trait Map

| # | Requirement |
|---|-------------|
| 1 | R-1.3.9     |
| 2 | R-1.3.9     |

1. **#1** — Downcast HashMap to ReflectMap
   - **Expected:** Succeeds
2. **#2** — Call insert, get, remove
   - **Expected:** All operations work correctly

### TC-1.3.10.1 FromReflect Missing Field

| # | Requirement |
|---|-------------|
| 1 | R-1.3.10    |

1. **#1** — DynamicValue with 2 of 3 fields
   - **Expected:** FromReflect fills third with default

### TC-1.3.10.2 FromReflect Incompatible

| # | Requirement |
|---|-------------|
| 1 | R-1.3.10    |

1. **#1** — DynamicValue with wrong type for field
   - **Expected:** `FromReflect` returns None

### TC-1.4.1.1 Binary Serialize Mixed Types

| # | Requirement |
|---|-------------|
| 1 | R-1.4.1     |
| 2 | R-1.4.1     |

1. **#1** — Serialize struct with u32, f64, String, nested struct
   - **Expected:** Bytes produced
2. **#2** — Deserialize
   - **Expected:** Value equals original

### TC-1.4.1.2 Binary Malformed Error

| # | Requirement |
|---|-------------|
| 1 | R-1.4.1a    |

1. **#1** — Feed truncated binary data
   - **Expected:** Error with byte offset of failure

### TC-1.4.1.3 Binary Throughput

| # | Requirement |
|---|-------------|
| 1 | R-1.4.1a    |

1. **#1** — Serialize/deserialize 100K components
   - **Expected:** Throughput >= 500 MB/s

### TC-1.4.3.1 Text Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-1.4.3     |
| 2 | R-1.4.3     |

1. **#1** — Serialize complex struct to RON
   - **Expected:** Valid RON string
2. **#2** — Deserialize RON back
   - **Expected:** Bit-exact equality with original

### TC-1.4.3.2 Text Edge Cases

| # | Requirement |
|---|-------------|
| 1 | R-1.4.3     |
| 2 | R-1.4.3     |

1. **#1** — Empty Vec, None Option, Unicode string
   - **Expected:** Serialize succeeds
2. **#2** — Deserialize
   - **Expected:** Values match originals

### TC-1.4.4.1 Schema Version Migration

| # | Requirement |
|---|-------------|
| 1 | R-1.4.4     |
| 2 | R-1.4.4     |

1. **#1** — Serialize at schema v1.0.0
   - **Expected:** Binary with v1.0.0 tag
2. **#2** — Deserialize at schema v1.1.0
   - **Expected:** Migration triggered, data correct

### TC-1.4.5.1 Migration Chain 3 Steps

| # | Requirement |
|---|-------------|
| 1 | R-1.4.5     |
| 2 | R-1.4.5     |

1. **#1** — Data at v1, migrations v1->v2->v3 registered
   - **Expected:** Chain exists
2. **#2** — Deserialize at v3
   - **Expected:** Each step applied in order, final data correct

### TC-1.4.5.2 Migration Chain 50 Steps

| # | Requirement |
|---|-------------|
| 1 | R-1.4.5a    |
| 2 | R-1.4.5a    |

1. **#1** — 50-step migration chain v1 through v50
   - **Expected:** All registered
2. **#2** — Deserialize v1 data at v50
   - **Expected:** End-to-end success

### TC-1.4.5.3 Migration Missing Step

| # | Requirement |
|---|-------------|
| 1 | R-1.4.5a    |
| 2 | R-1.4.5a    |

1. **#1** — Remove step v2->v3 from chain
   - **Expected:** Gap exists
2. **#2** — Attempt migration v1->v3
   - **Expected:** Error identifies missing v2->v3

### TC-1.4.9.1 Companion Write Read

| # | Requirement |
|---|-------------|
| 1 | R-1.4.9     |
| 2 | R-1.4.9     |

1. **#1** — Write 3 blobs (1 LZ4 compressed, 2 raw)
   - **Expected:** Companion file created
2. **#2** — Read each blob back
   - **Expected:** Hash matches original for all 3

### TC-1.4.9.2 Companion Dedup

| # | Requirement |
|---|-------------|
| 1 | R-1.4.9     |

1. **#1** — Write blob A, write identical blob A again
   - **Expected:** No additional storage consumed

### TC-1.4.9.3 Companion Append

| # | Requirement |
|---|-------------|
| 1 | R-1.4.9a    |
| 2 | R-1.4.9a    |

1. **#1** — Create companion with 2 blobs
   - **Expected:** 2 blobs readable
2. **#2** — Append third blob
   - **Expected:** All 3 blobs readable

### TC-1.4.10.1 Binary Attribute

| # | Requirement |
|---|-------------|
| 1 | R-1.4.10    |
| 2 | R-1.4.10    |

1. **#1** — Field with `#[binary(compress = "lz4")]`
   - **Expected:** Field stored in companion (compressed)
2. **#2** — String field without attribute
   - **Expected:** Field stored inline in text

### TC-1.4.8.1 Atomic Write Interrupted

| # | Requirement |
|---|-------------|
| 1 | R-1.4.8a    |

1. **#1** — Simulate crash after text write but before companion rename
   - **Expected:** No partial files on disk

## Integration Tests

### TC-1.3.1.I1 Registry Concurrent Reads

| # | Requirement |
|---|-------------|
| 1 | R-1.3.1     |
| 2 | R-1.3.1     |

1. **#1** — Register 500 types, freeze
   - **Expected:** Registry frozen
2. **#2** — 16 threads read concurrently
   - **Expected:** ThreadSanitizer clean, all reads correct

### TC-1.4.7.I1 Scene Roundtrip 1K Entities

| # | Requirement |
|---|-------------|
| 1 | R-1.4.7     |
| 2 | R-1.4.7     |

1. **#1** — 1,000 entities with hierarchy
   - **Expected:** All spawned
2. **#2** — Serialize, deserialize
   - **Expected:** Full state matches original

### TC-1.4.7.I2 Scene Merge No Collisions

| # | Requirement |
|---|-------------|
| 1 | R-1.4.7     |
| 2 | R-1.4.7     |

1. **#1** — Deserialize same scene twice into one world
   - **Expected:** Two copies exist
2. **#2** — Verify entity IDs
   - **Expected:** All remapped, no collisions

### TC-1.4.8.I1 Mixed Format Scene

| # | Requirement |
|---|-------------|
| 1 | R-1.4.8     |

1. **#1** — Scene with transforms inline (text) and mesh vertices in binary companion
   - **Expected:** Loaded correctly

### TC-1.4.6.I1 Asset Handle Resolution

| # | Requirement |
|---|-------------|
| 1 | R-1.4.6     |
| 2 | R-1.4.6     |

1. **#1** — Serialize asset handle
   - **Expected:** Handle persisted
2. **#2** — Deserialize in fresh world
   - **Expected:** Load request issued for asset

### TC-1.4.7.I2 Streaming Deser Mobile

| # | Requirement |
|---|-------------|
| 1 | R-1.4.7     |
| 2 | R-1.4.7     |

1. **#1** — 64 MB scene with 1 MB staging buffer
   - **Expected:** Deserialize completes
2. **#2** — Monitor memory
   - **Expected:** No spike beyond staging buffer

### TC-1.4.2.I1 Zero Copy 256MB

| # | Requirement |
|---|-------------|
| 1 | R-1.4.2     |
| 2 | R-1.4.2     |

1. **#1** — Memory-map 256 MB asset
   - **Expected:** Mapped successfully
2. **#2** — Access fields
   - **Expected:** Zero allocation beyond mmap

### TC-1.3.3.I1 Engine Developer Queries Type Descriptors

| # | Requirement |
|---|-------------|
| 1 | US-1.3.3    |
| 2 | US-1.3.3    |

1. **#1** — Engine developer calls `type_info::<Transform>()` and reads size, alignment, field list
   - **Expected:** Returns descriptor with correct `size`, `align`, and field name/offset/type
     tuples
2. **#2** — Use descriptor to walk fields via property path for every registered component
   - **Expected:** Every field accessible without panics across 1K types

### TC-1.3.6.I1 Engine Developer Exposes Dynamic Collections Via Reflection

| # | Requirement |
|---|-------------|
| 1 | US-1.3.6    |
| 2 | US-1.3.6    |

1. **#1** — Engine developer calls `push`/`pop`/`len`/`get(i)` on a reflected `Vec<f32>`
   - **Expected:** All operations mutate the underlying vector through the reflection trait
2. **#2** — Reflect a `HashMap<String, i32>` and iterate entries
   - **Expected:** Iterator yields all key-value pairs, insertion/removal work through reflection

### TC-1.3.10.I1 Game Developer Attaches Metadata To Reflected Types

| # | Requirement |
|---|-------------|
| 1 | US-1.3.10   |
| 2 | US-1.3.10   |

1. **#1** — Game developer attaches `("category", "gameplay")` metadata to `Health` component via
   attribute macro
   - **Expected:** `type_info::<Health>().metadata().get("category")` returns `"gameplay"`
2. **#2** — Attach 5 metadata keys to a field and query them
   - **Expected:** All 5 keys returned, keys lexicographically retrievable

<!-- THIN: design section lacks detail -->
### TC-1.3.19.I1 Editor Reads Reflection For Inspector Panel

| # | Requirement |
|---|-------------|
| 1 | US-1.3.19   |
| 2 | US-1.3.19   |

1. **#1** — Editor selects entity and walks `type_info` for every attached component
   - **Expected:** Inspector panel renders field widgets for each field of each component
2. **#2** — Edit a field in inspector, commit value
   - **Expected:** Reflection write applies mutation, next read returns edited value

## Benchmarks

### TC-1.4.1.B1 Binary Serialize Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Serialize 100K components to binary | Throughput | >= 500 MB/s | R-1.4.1a |

### TC-1.4.1.B2 Binary Deserialize Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Deserialize 100K components from binary | Throughput | >= 500 MB/s | R-1.4.1a |

### TC-1.3.3.B1 Property Path Access

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 6-segment property path access | Latency | < 500 ns | R-1.3.3a |

### TC-1.3.1.B1 TypeRegistry Lookup

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Lookup by TypeId | Latency | < 50 ns | R-1.3.1a |

### TC-1.3.5.B1 Diff Performance

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Diff 10-field struct, 1 change | Latency | < 1 us | R-1.3.5 |

### TC-1.3.5.B2 Patch Apply Performance

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Apply 1 SetField op | Latency | < 500 ns | R-1.3.5 |

### TC-1.4.2.B1 Zero-Copy Field Access

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Access field from memory-mapped asset | Latency | < 1 us | R-1.4.2 |

### TC-1.4.3.B1 RON Serialize

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Serialize 100 components to RON | Time | < 10 ms | R-1.4.3 |

### TC-1.4.5.B1 Migration Chain

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10-step migration chain | Time | < 100 us | R-1.4.5 |
