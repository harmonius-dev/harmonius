# R-1.3 — Reflection & Type System Requirements

## Type Registry

1. **R-1.3.1** — The engine **SHALL** maintain a runtime type registry mapping type identifiers to
   metadata, immutable after initialization, enabling lock-free concurrent reads from any thread.
   The registry **SHALL** support at least 10,000 registered types. Type lookup **SHALL** complete
   in O(1). Duplicate type IDs **SHALL** produce a diagnostic error naming both conflicting types.
   - **Rationale:** Every component, resource, and event type must be registered for serialization,
     editor tooling, and network replication.
   - **Verification:** Register 10,000 types; verify O(1) lookup. Concurrently read from 8 threads;
     verify no data races via ThreadSanitizer. Attempt duplicate ID; verify error names both types.

## Type Info

1. **R-1.3.2** — The engine **SHALL** store type descriptors containing size, alignment, drop,
   clone, default constructor, field layout, and variant layout, sufficient for safe dynamic
   operations on type-erased data.
   - **Rationale:** Type descriptors are the foundation for the dynamic property system,
     serialization, and editor reflection.
   - **Verification:** Register a struct; retrieve descriptor; verify size, alignment, field count
     match. Construct via default; clone; drop both; verify no memory leaks.

## Property System

1. **R-1.3.3** — The engine **SHALL** expose struct fields and enum variants as named properties
   supporting path-based read/write access (e.g., transform.position.x) for nested structures at
   runtime. Path access **SHALL** complete in under 500 ns for paths up to 8 segments. Invalid paths
   **SHALL** return a typed error with the failing segment and parent type name.
   - **Rationale:** Path-based access enables generic editor inspectors, animation binding, and
     network delta serialization.
   - **Verification:** Register nested struct; read/write via path; verify round-trip. Access
     invalid path; verify error message. Benchmark 6-segment path at under 500 ns.
2. **R-1.3.4** — The engine **SHALL** extend the property system to dynamic-length collections
   (arrays, vectors, maps, sets) with insertion, removal, iteration, and indexed access through a
   uniform trait interface.
   - **Rationale:** Generic serialization and editor UI must handle collections without
     per-container specialization.
   - **Verification:** Register Vec and HashMap; insert, remove, iterate, index through reflection;
     verify all operations match direct Rust access.

## Dynamic Access

1. **R-1.3.5** — The engine **SHALL** provide a DynamicValue type holding any reflected value,
   supporting the full property API, diffing against typed values, and applying patches.
   - **Rationale:** DynamicValue is the interchange format between serialization, replication, and
     the ECS world.
   - **Verification:** Construct DynamicValue from serialized data; diff against typed instance;
     apply patch; verify match. Round-trip: typed -> dynamic -> typed produces identical values.

## Attribute / Metadata System

1. **R-1.3.6** — The engine **SHALL** allow user-defined key-value metadata on types and fields at
   registration, queryable at runtime, including standard attributes for display names, numeric
   ranges, serialization hints, and replication policies.
   - **Rationale:** Attributes drive editor, serializer, and network layer behavior without coupling
     to specific types.
   - **Verification:** Register type with range, skip, and display_name attributes; query each at
     runtime; verify correct values. Verify serializer respects skip.
2. **R-1.3.7** — The engine **SHALL** support registering trait implementations against type IDs for
   runtime resolution on type-erased data.
   - **Rationale:** Explicit trait registration replaces virtual dispatch with registry lookup while
     maintaining static dispatch compatibility.
   - **Verification:** Register Serialize for type A; resolve by type ID on erased pointer; verify
     correct output. Query unregistered trait; verify clear error.

## Reflect Trait

1. **R-1.3.8** — The engine **SHALL** provide a Reflect trait modeled after bevy_reflect exposing
   runtime introspection: type info, field get/set by name or index, iteration, dynamic clone, and
   patch application. A derive macro **SHALL** auto-implement Reflect for structs and enums,
   generating accessors, type registration, and collection trait implementations.
   - **Rationale:** Unified Reflect with derive macro eliminates boilerplate and ensures every
     reflected type supports the full pipeline.
   - **Verification:** Derive on struct, enum, tuple struct; verify field access, index iteration,
     clone-to-dynamic, and patch all produce correct results. Verify derive macro auto-registers in
     TypeRegistry.
2. **R-1.3.9** — The Reflect derive macro **SHALL** support attributes: reflect(skip) to exclude a
   field, reflect(rename) to override field names, and reflect(default) for fallback values during
   deserialization. Unsupported attributes **SHALL** produce a compile-time error.
   - **Rationale:** Attribute annotations customize reflection per field without manual trait
     implementations.
   - **Verification:** Derive with skip, rename, default; verify skipped fields absent, renamed
     fields use new name, defaults applied when missing. Verify invalid attribute produces compile
     error.
3. **R-1.3.10** — The engine **SHALL** provide sub-traits of Reflect for each structural category:
   ReflectStruct, ReflectTupleStruct, ReflectEnum, ReflectList, ReflectMap, and ReflectValue. The
   Reflect trait **SHALL** expose a method to obtain the appropriate sub-trait reference.
   - **Rationale:** Sub-traits enable generic code to handle each category uniformly.
   - **Verification:** Register struct, enum, list, map; obtain sub-trait; invoke category-specific
     methods; verify correct results. Request wrong sub-trait; verify returns None.
4. **R-1.3.11** — The engine **SHALL** provide a FromReflect trait constructing concrete typed
   values from any Reflect reference or DynamicValue, returning Option to signal conversion failure.
   The derive macro **SHALL** generate FromReflect alongside Reflect, using registered field
   defaults for missing values.
   - **Rationale:** FromReflect completes the round-trip between dynamic and static representations.
   - **Verification:** Create DynamicValue with one missing field; call FromReflect; verify default
     applied. Create incompatible DynamicValue; verify returns None. Round-trip: typed -> clone ->
     FromReflect produces identical value.
