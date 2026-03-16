# Reflection and Serialization Test Cases

Companion test cases for [reflection-serialization.md](reflection-serialization.md).

## Unit Tests

### TC-1.3.1.1 Registry 10K Types

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register 10,000 types | All registered without error | R-1.3.1a |
| 2 | Lookup each by TypeId | O(1) per lookup, all found | R-1.3.1a |
| 3 | Lookup each by name | O(1) per lookup, all found | R-1.3.1a |

### TC-1.3.1.2 Registry Duplicate ID Error

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register type A with TypeId X | Success | R-1.3.1a |
| 2 | Register type B with same TypeId X | Error naming both A and B | R-1.3.1a |

### TC-1.3.1.3 Registry Freeze Rejects Insert

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Call `freeze()` on registry | Frozen | R-1.3.1 |
| 2 | Call `register::<T>()` | Panics | R-1.3.1 |

### TC-1.3.2.1 Type Descriptor Layout

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register struct with fields (u32, f64, bool) | Descriptor created | R-1.3.2 |
| 2 | Query size, alignment, field count, offsets | Match `std::mem::size_of`, `align_of`, and `offset_of` | R-1.3.2 |

### TC-1.3.2.2 Dynamic Move Drop Clone

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Construct type-erased instance via `default_fn` | Instance created | R-1.3.2 |
| 2 | Clone instance | Two independent copies | R-1.3.2 |
| 3 | Drop both | Custom allocator reports zero leaks | R-1.3.2 |

### TC-1.3.3.1 Property Path Nested

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write 42 to `outer.inner.value` via path API | Write succeeds | R-1.3.3 |
| 2 | Read `outer.inner.value` | Returns 42 | R-1.3.3 |

### TC-1.3.3.2 Property Path Invalid

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Access `outer.nonexistent.field` | Error containing "nonexistent" and parent type | R-1.3.3a |

### TC-1.3.3.3 Property Path Performance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Access 6-segment path 100,000 times | All accesses succeed | R-1.3.3a |
| 2 | Measure per-access time | < 500 ns | R-1.3.3a |

### TC-1.3.4.1 Collection Vec Reflect

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert 3 elements into Vec via reflection | Vec length = 3 | R-1.3.4 |
| 2 | Remove element at index 1 | Vec length = 2, correct values | R-1.3.4 |
| 3 | Iterate via reflection | All remaining elements visited | R-1.3.4 |
| 4 | Index access `[0]` | Returns first element | R-1.3.4 |

### TC-1.3.4.2 Collection Map Reflect

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert ("a", 1), ("b", 2) via reflection | Map size = 2 | R-1.3.4 |
| 2 | Get "a" | Returns 1 | R-1.3.4 |
| 3 | Remove "b" | Map size = 1 | R-1.3.4 |

### TC-1.3.5.1 DynamicValue Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Typed struct -> `DynamicValue` | DynamicValue created | R-1.3.5 |
| 2 | `FromReflect` back to typed | Identical to original | R-1.3.5 |

### TC-1.3.5.2 Diff Minimal Patch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10-field struct, change field 3 only | Diff produces patch with 1 op | R-1.3.5 |

### TC-1.3.5.3 Patch Apply Correct

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply SetField(field_3, 99) to struct | `struct.field_3 == 99` | R-1.3.5 |
| 2 | Other fields unchanged | All 9 other fields identical | R-1.3.5 |

### TC-1.3.6.1 Attributes Range Skip Rename

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register type with `#[reflect(min=0, max=100)]` | Attribute queryable at runtime | R-1.3.6 |
| 2 | Query skip attribute on skipped field | `skip = true` | R-1.3.6 |
| 3 | Query rename attribute | Returns overridden name | R-1.3.6 |

### TC-1.3.7.1 Trait Registration and Resolve

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register Serialize trait impl for type A | Registration succeeds | R-1.3.7 |
| 2 | Resolve by TypeId, invoke serialize | Produces correct output | R-1.3.7 |

### TC-1.3.7.2 Trait Missing Error

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Query unregistered trait for type B | Error message with trait and type names | R-1.3.7 |

### TC-1.3.8.1 Derive Struct Reflect

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `#[derive(Reflect)]` on struct with 3 fields | Reflect impl generated | R-1.3.8 |
| 2 | Access field by name "x" | Returns correct value | R-1.3.8 |
| 3 | Access field by index 0 | Returns same value as by name | R-1.3.8 |

### TC-1.3.8.2 Derive Enum Reflect

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `#[derive(Reflect)]` on enum with 3 variants | Reflect impl generated | R-1.3.8 |
| 2 | Query current variant name | Returns correct variant | R-1.3.8 |
| 3 | Access variant field | Returns correct data | R-1.3.8 |

### TC-1.3.8.3 Derive Skip Attribute

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `#[reflect(skip)]` on field "secret" | Field absent from reflection | R-1.3.8a |

### TC-1.3.8.4 Derive Rename Attribute

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `#[reflect(rename = "display_name")]` on field "x" | Field accessed as "display_name" | R-1.3.8a |

### TC-1.3.8.5 Derive Default Attribute

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Patch struct with missing field (has default) | Missing field receives default value | R-1.3.8a |

### TC-1.3.9.1 Sub-trait Struct

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Downcast struct's Reflect to ReflectStruct | Succeeds | R-1.3.9 |
| 2 | Downcast struct's Reflect to ReflectEnum | Returns None | R-1.3.9 |

### TC-1.3.9.2 Sub-trait Enum

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Downcast enum's Reflect to ReflectEnum | Succeeds | R-1.3.9 |
| 2 | Downcast enum's Reflect to ReflectStruct | Returns None | R-1.3.9 |

### TC-1.3.9.3 Sub-trait List

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Downcast Vec to ReflectList | Succeeds | R-1.3.9 |
| 2 | Call push, pop, len | All operations work correctly | R-1.3.9 |

### TC-1.3.9.4 Sub-trait Map

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Downcast HashMap to ReflectMap | Succeeds | R-1.3.9 |
| 2 | Call insert, get, remove | All operations work correctly | R-1.3.9 |

### TC-1.3.10.1 FromReflect Missing Field

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | DynamicValue with 2 of 3 fields | FromReflect fills third with default | R-1.3.10 |

### TC-1.3.10.2 FromReflect Incompatible

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | DynamicValue with wrong type for field | `FromReflect` returns None | R-1.3.10 |

### TC-1.4.1.1 Binary Serialize Mixed Types

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Serialize struct with u32, f64, String, nested struct | Bytes produced | R-1.4.1 |
| 2 | Deserialize | Value equals original | R-1.4.1 |

### TC-1.4.1.2 Binary Malformed Error

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Feed truncated binary data | Error with byte offset of failure | R-1.4.1a |

### TC-1.4.1.3 Binary Throughput

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Serialize/deserialize 100K components | Throughput >= 500 MB/s | R-1.4.1a |

### TC-1.4.3.1 Text Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Serialize complex struct to RON | Valid RON string | R-1.4.3 |
| 2 | Deserialize RON back | Bit-exact equality with original | R-1.4.3 |

### TC-1.4.3.2 Text Edge Cases

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Empty Vec, None Option, Unicode string | Serialize succeeds | R-1.4.3 |
| 2 | Deserialize | Values match originals | R-1.4.3 |

### TC-1.4.4.1 Schema Version Migration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Serialize at schema v1.0.0 | Binary with v1.0.0 tag | R-1.4.4 |
| 2 | Deserialize at schema v1.1.0 | Migration triggered, data correct | R-1.4.4 |

### TC-1.4.5.1 Migration Chain 3 Steps

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Data at v1, migrations v1->v2->v3 registered | Chain exists | R-1.4.5 |
| 2 | Deserialize at v3 | Each step applied in order, final data correct | R-1.4.5 |

### TC-1.4.5.2 Migration Chain 50 Steps

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 50-step migration chain v1 through v50 | All registered | R-1.4.5a |
| 2 | Deserialize v1 data at v50 | End-to-end success | R-1.4.5a |

### TC-1.4.5.3 Migration Missing Step

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Remove step v2->v3 from chain | Gap exists | R-1.4.5a |
| 2 | Attempt migration v1->v3 | Error identifies missing v2->v3 | R-1.4.5a |

### TC-1.4.9.1 Companion Write Read

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write 3 blobs (1 LZ4 compressed, 2 raw) | Companion file created | R-1.4.9 |
| 2 | Read each blob back | Hash matches original for all 3 | R-1.4.9 |

### TC-1.4.9.2 Companion Dedup

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write blob A, write identical blob A again | No additional storage consumed | R-1.4.9 |

### TC-1.4.9.3 Companion Append

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create companion with 2 blobs | 2 blobs readable | R-1.4.9a |
| 2 | Append third blob | All 3 blobs readable | R-1.4.9a |

### TC-1.4.10.1 Binary Attribute

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Field with `#[binary(compress = "lz4")]` | Field stored in companion (compressed) | R-1.4.10 |
| 2 | String field without attribute | Field stored inline in text | R-1.4.10 |

### TC-1.4.8.1 Atomic Write Interrupted

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Simulate crash after text write but before companion rename | No partial files on disk | R-1.4.8a |

## Integration Tests

### TC-1.3.1.I1 Registry Concurrent Reads

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register 500 types, freeze | Registry frozen | R-1.3.1 |
| 2 | 16 threads read concurrently | ThreadSanitizer clean, all reads correct | R-1.3.1 |

### TC-1.4.7.I1 Scene Roundtrip 1K Entities

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1,000 entities with hierarchy | All spawned | R-1.4.7 |
| 2 | Serialize, deserialize | Full state matches original | R-1.4.7 |

### TC-1.4.7.I2 Scene Merge No Collisions

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Deserialize same scene twice into one world | Two copies exist | R-1.4.7 |
| 2 | Verify entity IDs | All remapped, no collisions | R-1.4.7 |

### TC-1.4.8.I1 Mixed Format Scene

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Scene with transforms inline (text) and mesh vertices in binary companion | Loaded correctly | R-1.4.8 |

### TC-1.4.6.I1 Asset Handle Resolution

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Serialize asset handle | Handle persisted | R-1.4.6 |
| 2 | Deserialize in fresh world | Load request issued for asset | R-1.4.6 |

### TC-1.4.7.I2 Streaming Deser Mobile

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 64 MB scene with 1 MB staging buffer | Deserialize completes | R-1.4.7 |
| 2 | Monitor memory | No spike beyond staging buffer | R-1.4.7 |

### TC-1.4.2.I1 Zero Copy 256MB

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Memory-map 256 MB asset | Mapped successfully | R-1.4.2 |
| 2 | Access fields | Zero allocation beyond mmap | R-1.4.2 |

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
