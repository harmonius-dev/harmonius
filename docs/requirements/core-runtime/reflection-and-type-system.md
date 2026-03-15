# R-1.3 — Reflection & Type System Requirements

## Type Registry

### R-1.3.1 Global Type Registry

The engine **SHALL** maintain a runtime type registry mapping type identifiers to type metadata,
immutable after initialization, enabling lock-free concurrent reads from any thread without
synchronization.

- **Derived from:** [F-1.3.1](../../features/core-runtime/reflection-and-type-system.md)
- **Rationale:** Every component, resource, and event type must be registered for serialization,
  editor tooling, and network replication to function.
- **Verification:** Integration test: register 500 types at startup, then concurrently read type
  metadata from 8 threads. Verify no data races (via ThreadSanitizer) and correct metadata
  retrieval. Verify that attempting to register a type after initialization fails with an error.

### R-1.3.1a Type Registry Capacity and Lookup Performance

The type registry **SHALL** support at least 10,000 registered types. Type lookup by ID **SHALL**
complete in O(1). The registry **SHALL** report a diagnostic error if a type is registered with a
duplicate ID, including the conflicting type names.

- **Derived from:** [F-1.3.1](../../features/core-runtime/reflection-and-type-system.md)
- **Rationale:** Large engines with many plugins may register thousands of types; duplicate IDs
  cause silent data corruption.
- **Verification:** Register 10,000 types and verify O(1) lookup time. Attempt to register a
  duplicate type ID and verify the error message names both conflicting types.

## Type Info

### R-1.3.2 Structured Type Descriptors

The engine **SHALL** store type descriptors containing size, alignment, drop function, clone
function, default constructor, field layout, and variant layout, sufficient to perform safe dynamic
operations (move, drop, clone) on type-erased data.

- **Derived from:** [F-1.3.2](../../features/core-runtime/reflection-and-type-system.md)
- **Rationale:** Type descriptors are the foundation for the dynamic property system, serialization,
  and editor reflection without requiring generic type parameters.
- **Verification:** Unit test: register a struct with known layout. Retrieve its descriptor and
  verify size, alignment, and field count match. Construct a type-erased instance via the default
  constructor, clone it, and drop both — verify no memory leaks via a custom allocator tracker.

## Property System

### R-1.3.3 Reflective Property Access

The engine **SHALL** expose struct fields and enum variants as named properties supporting
path-based read/write access (e.g., `transform.position.x`) for nested structures at runtime.

- **Derived from:** [F-1.3.3](../../features/core-runtime/reflection-and-type-system.md)
- **Rationale:** Path-based property access enables generic editor inspectors, animation binding,
  and network delta serialization without per-type boilerplate.
- **Verification:** Unit test: register a nested struct `Outer { inner: Inner { value: f32 } }`.
  Read and write `outer.inner.value` via the path API. Verify the value round-trips correctly.
  Verify accessing a nonexistent path returns an error.

### R-1.3.4 Collection Reflection

The engine **SHALL** extend the property system to handle dynamic-length collections (arrays,
vectors, hash maps, hash sets) with insertion, removal, iteration, and indexed access through a
uniform trait interface.

- **Derived from:** [F-1.3.4](../../features/core-runtime/reflection-and-type-system.md)
- **Rationale:** Generic serialization and editor UI must handle collections without
  per-container-type specialization.
- **Verification:** Unit test: register a `Vec<u32>` and a `HashMap<String, f64>` as reflected
  collections. Insert, remove, iterate, and index-access elements through the reflection API. Verify
  all operations produce correct results matching direct Rust access.

### R-1.3.3a Property Access Performance and Error Handling

Path-based property access **SHALL** complete in under 500 nanoseconds for paths up to 8 segments
deep. Accessing an invalid path **SHALL** return a typed error containing the path string and the
point of failure (e.g., "field 'position' not found on type 'Transform'"), not a panic or undefined
behavior.

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
- **Rationale:** `DynamicValue` is the interchange format between serialization, network
  replication, and the ECS world, enabling schema migration and patching.
- **Verification:** Unit test: construct a `DynamicValue` from a serialized blob, diff it against a
  typed instance, apply the diff as a patch, and verify the typed instance matches the dynamic
  value. Verify round-trip: typed -> dynamic -> typed produces identical values.

## Attribute / Metadata System

### R-1.3.6 Custom Type and Field Attributes

The engine **SHALL** allow user-defined key-value metadata to be attached to types and individual
fields at registration time, queryable at runtime, including standard attributes for display names,
numeric ranges, serialization hints, and replication policies.

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
- **Verification:** Unit test: register a `Serialize` trait implementation for type A. Given only
  type A's ID and a type-erased pointer, resolve the trait implementation and invoke serialization.
  Verify correct output. Verify that querying an unregistered trait for a type returns a clear
  error.

## Reflect Trait

### R-1.3.8 Reflect Trait with Derive Macro

The engine **SHALL** provide a `Reflect` trait modeled after bevy_reflect that exposes runtime
introspection methods: type info access, field get/set by name or index, field iteration, dynamic
clone, and patch application. A derive macro **SHALL** auto-implement `Reflect` for structs and
enums, generating property accessors, type registration, and collection trait implementations from
the type definition.

- **Derived from:** [F-1.3.8](../../features/core-runtime/reflection-and-type-system.md)
- **Rationale:** A unified Reflect trait with derive macro eliminates boilerplate and ensures every
  reflected type supports the full property, serialization, and editor pipeline automatically.
- **Verification:** Unit test: derive `Reflect` on a struct with nested fields, an enum with
  variants, and a tuple struct. Verify that field access by name, index iteration, clone-to-dynamic,
  and patch application all produce correct results. Verify the derive macro registers the type in
  the TypeRegistry automatically.

### R-1.3.8a Reflect Derive Attribute Annotations

The `Reflect` derive macro **SHALL** support attribute annotations: `#[reflect(skip)]` to exclude a
field, `#[reflect(rename = "...")]` to override field names, and `#[reflect(default)]` to provide
fallback values during deserialization. Unsupported attributes **SHALL** produce a compile-time
error with a descriptive message.

- **Derived from:** [F-1.3.8](../../features/core-runtime/reflection-and-type-system.md)
- **Rationale:** Attribute annotations customize reflection behavior per field without requiring
  manual trait implementations.
- **Verification:** Unit test: derive `Reflect` with `skip`, `rename`, and `default` attributes.
  Verify skipped fields are absent from reflection, renamed fields use the new name, and default
  fields receive their default value when missing during patching. Verify an invalid attribute
  produces a compile error.

### R-1.3.9 Reflect Sub-Traits for Type Categories

The engine **SHALL** provide sub-traits of `Reflect` for each structural category: `ReflectStruct`,
`ReflectTupleStruct`, `ReflectEnum`, `ReflectList`, `ReflectMap`, and `ReflectValue`. The `Reflect`
trait **SHALL** expose a method to obtain the appropriate sub-trait reference for category-specific
access.

- **Derived from:** [F-1.3.9](../../features/core-runtime/reflection-and-type-system.md)
- **Rationale:** Sub-traits enable generic code to handle each structural category uniformly without
  matching on concrete types, supporting editor inspectors and serializers.
- **Verification:** Unit test: register a struct, enum, list, and map type. For each, obtain the
  sub-trait reference and invoke category-specific methods (field access for structs, variant access
  for enums, push/pop for lists, insert/get for maps). Verify each returns correct results and that
  requesting the wrong sub-trait returns `None`.

### R-1.3.10 FromReflect Conversion Trait

The engine **SHALL** provide a `FromReflect` trait that constructs a concrete typed value from any
`Reflect` reference or `DynamicValue`, returning `Option` to signal conversion failure. The derive
macro **SHALL** generate `FromReflect` alongside `Reflect`, using registered field defaults for
missing values.

- **Derived from:** [F-1.3.10](../../features/core-runtime/reflection-and-type-system.md)
- **Rationale:** `FromReflect` completes the round-trip between dynamic and static representations,
  enabling deserialization and undo/redo to reconstruct concrete values from dynamic data.
- **Verification:** Unit test: create a `DynamicValue` representing a struct with one field missing.
  Call `FromReflect` and verify the missing field receives its default. Create a `DynamicValue` with
  an incompatible type and verify `FromReflect` returns `None`. Verify round-trip: typed ->
  Reflect::clone -> FromReflect produces an identical value.
