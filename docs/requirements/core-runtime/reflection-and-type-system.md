# R-1.3 — Reflection & Type System Requirements

## Type Registry

### R-1.3.1 Global Type Registry

The engine **SHALL** maintain a runtime type registry mapping type identifiers to type metadata, immutable
after initialization, enabling lock-free concurrent reads from any thread without synchronization.

- **Derived from:** [F-1.3.1](../../features/core-runtime/reflection-and-type-system.md)
- **Rationale:** Every component, resource, and event type must be registered for serialization, editor
  tooling, and network replication to function.
- **Verification:** Integration test: register 500 types at startup, then concurrently read type
  metadata from 8 threads. Verify no data races (via ThreadSanitizer) and correct metadata retrieval.
  Verify that attempting to register a type after initialization fails with an error.

### R-1.3.1a Type Registry Capacity and Lookup Performance

The type registry **SHALL** support at least 10,000 registered types. Type lookup by ID **SHALL**
complete in O(1). The registry **SHALL** report a diagnostic error if a type is registered with a
duplicate ID, including the conflicting type names.

- **Derived from:** [F-1.3.1](../../features/core-runtime/reflection-and-type-system.md)
- **Rationale:** Large engines with many plugins may register thousands of types; duplicate IDs cause
  silent data corruption.
- **Verification:** Register 10,000 types and verify O(1) lookup time. Attempt to register a
  duplicate type ID and verify the error message names both conflicting types.

## Type Info

### R-1.3.2 Structured Type Descriptors

The engine **SHALL** store type descriptors containing size, alignment, drop function, clone function,
default constructor, field layout, and variant layout, sufficient to perform safe dynamic operations
(move, drop, clone) on type-erased data.

- **Derived from:** [F-1.3.2](../../features/core-runtime/reflection-and-type-system.md)
- **Rationale:** Type descriptors are the foundation for the dynamic property system, serialization, and
  editor reflection without requiring generic type parameters.
- **Verification:** Unit test: register a struct with known layout. Retrieve its descriptor and verify
  size, alignment, and field count match. Construct a type-erased instance via the default constructor,
  clone it, and drop both — verify no memory leaks via a custom allocator tracker.

## Property System

### R-1.3.3 Reflective Property Access

The engine **SHALL** expose struct fields and enum variants as named properties supporting path-based
read/write access (e.g., `transform.position.x`) for nested structures at runtime.

- **Derived from:** [F-1.3.3](../../features/core-runtime/reflection-and-type-system.md)
- **Rationale:** Path-based property access enables generic editor inspectors, animation binding, and
  network delta serialization without per-type boilerplate.
- **Verification:** Unit test: register a nested struct `Outer { inner: Inner { value: f32 } }`. Read
  and write `outer.inner.value` via the path API. Verify the value round-trips correctly. Verify
  accessing a nonexistent path returns an error.

### R-1.3.4 Collection Reflection

The engine **SHALL** extend the property system to handle dynamic-length collections (arrays, vectors,
hash maps, hash sets) with insertion, removal, iteration, and indexed access through a uniform trait
interface.

- **Derived from:** [F-1.3.4](../../features/core-runtime/reflection-and-type-system.md)
- **Rationale:** Generic serialization and editor UI must handle collections without per-container-type
  specialization.
- **Verification:** Unit test: register a `Vec<u32>` and a `HashMap<String, f64>` as reflected
  collections. Insert, remove, iterate, and index-access elements through the reflection API. Verify
  all operations produce correct results matching direct Rust access.

### R-1.3.3a Property Access Performance and Error Handling

Path-based property access **SHALL** complete in under 500 nanoseconds for paths up to 8 segments deep.
Accessing an invalid path **SHALL** return a typed error containing the path string and the point of
failure (e.g., "field 'position' not found on type 'Transform'"), not a panic or undefined behavior.

- **Derived from:** [F-1.3.3](../../features/core-runtime/reflection-and-type-system.md)
- **Rationale:** Property access is used in animation binding and editor inspection hot paths;
  informative errors are critical for debugging visual scripts and data bindings.
- **Verification:** Benchmark: access a 6-segment path 100,000 times and verify under 500 ns per
  access. Access an invalid path and verify the error message contains the failing segment and
  parent type name.

## Dynamic Access

### R-1.3.5 Type-Erased Dynamic Values

The engine **SHALL** provide a `DynamicValue` type that holds any reflected value, supports the full
property API, and can be diffed against typed values and applied as patches.

- **Derived from:** [F-1.3.5](../../features/core-runtime/reflection-and-type-system.md)
- **Rationale:** `DynamicValue` is the interchange format between serialization, network replication,
  and the ECS world, enabling schema migration and patching.
- **Verification:** Unit test: construct a `DynamicValue` from a serialized blob, diff it against a
  typed instance, apply the diff as a patch, and verify the typed instance matches the dynamic value.
  Verify round-trip: typed -> dynamic -> typed produces identical values.

## Attribute / Metadata System

### R-1.3.6 Custom Type and Field Attributes

The engine **SHALL** allow user-defined key-value metadata to be attached to types and individual fields
at registration time, queryable at runtime, including standard attributes for display names, numeric
ranges, serialization hints, and replication policies.

- **Derived from:** [F-1.3.6](../../features/core-runtime/reflection-and-type-system.md)
- **Rationale:** Attributes drive behavior in the editor, serializer, and network layer without
  coupling those systems to specific type definitions.
- **Verification:** Unit test: register a type with custom attributes (`#[range(0.0, 1.0)]`,
  `#[skip_serialize]`, `#[display_name("Health")]`). Query each attribute at runtime and verify
  correct values. Verify that the serializer respects `#[skip_serialize]` by omitting the field.

### R-1.3.7 Trait Object Registration and Dispatch

The engine **SHALL** support registering trait implementations against type IDs, enabling runtime
resolution of trait methods on type-erased data given only a type ID.

- **Derived from:** [F-1.3.7](../../features/core-runtime/reflection-and-type-system.md)
- **Rationale:** Explicit trait registration replaces virtual dispatch with a registry lookup,
  maintaining static dispatch compatibility while enabling runtime polymorphism where needed.
- **Verification:** Unit test: register a `Serialize` trait implementation for type A. Given only type
  A's ID and a type-erased pointer, resolve the trait implementation and invoke serialization. Verify
  correct output. Verify that querying an unregistered trait for a type returns a clear error.
