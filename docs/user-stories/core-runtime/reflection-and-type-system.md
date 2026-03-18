# Reflection & Type System User Stories

## Type Registry

| ID       | Persona                 | Features                  | Requirements              |
|----------|-------------------------|---------------------------|---------------------------|
| US-1.3.1 | engine developer (P-26) | F-1.3.1                   | R-1.3.1                   |
| US-1.3.2 | engine tester (P-27)    | F-1.3.1                   | R-1.3.1                   |
| US-1.3.3 | designer (P-5)          | F-1.3.1, F-1.3.2, F-1.3.6 | R-1.3.1, R-1.3.2, R-1.3.6 |

1. **US-1.3.1** — a runtime type registry mapping type IDs to metadata that is immutable after
   initialization, so that all threads can look up type information concurrently without
   synchronization overhead
   - **Acceptance:** Type ID to metadata mapping for all registered types<br>Registry immutable
     after startup, lock-free reads<br>All component, resource, and event types registered
2. **US-1.3.2** — to verify that concurrent type registry lookups from multiple threads produce
   correct results with no data races, so that the lock-free registry design holds under contention
   - **Acceptance:** Concurrent lookups from 16+ threads return correct metadata<br>No data races
     under sanitizer testing<br>Registry immutability enforced after initialization
3. **US-1.3.3** — the visual editor to discover all registered component types with their field
   names, types, and metadata, so that I can inspect and edit any component on any entity in the
   editor
   - **Acceptance:** Editor enumerates all registered types<br>Field names and types displayed per
     component<br>Metadata (display names, ranges) drives editor UI

## Type Info

| ID       | Persona                 | Features | Requirements |
|----------|-------------------------|----------|--------------|
| US-1.3.4 | engine developer (P-26) | F-1.3.2  | R-1.3.2      |

1. **US-1.3.4** — type descriptors storing size, alignment, drop, clone, default constructor, and
   field layout, so that I can perform safe dynamic operations on type-erased data for
   serialization, editor tooling, and network replication
   - **Acceptance:** Size, alignment, drop, clone, default stored per type<br>Field layout with
     offsets and nested descriptors<br>Safe move, drop, clone of type-erased data

## Property System

| ID       | Persona                 | Features | Requirements |
|----------|-------------------------|----------|--------------|
| US-1.3.5 | game developer (P-15)   | F-1.3.3  | R-1.3.3      |
| US-1.3.6 | designer (P-5)          | F-1.3.3  | R-1.3.3      |
| US-1.3.7 | engine developer (P-26) | F-1.3.4  | R-1.3.4      |
| US-1.3.8 | engine tester (P-27)    | F-1.3.4  | R-1.3.4      |

1. **US-1.3.5** — to read and write struct fields by name using path-based access like
   `transform.position.x`, so that I can bind animation curves, build data-driven configuration, and
   create generic editor inspectors without boilerplate
   - **Acceptance:** Named property access on struct fields and enum variants<br>Path-based access
     for nested structures<br>Read and write through property API
2. **US-1.3.6** — to bind animation curves to any reflected property path in the visual editor, so
   that I can animate any numeric field without writing code
   - **Acceptance:** Property paths selectable in editor UI<br>Animation curves drive property
     values at runtime<br>Nested property paths (position.x, color.r) supported
3. **US-1.3.7** — dynamic-length collections (vectors, maps, sets) exposed through a uniform
   reflection interface with insertion, removal, and indexed access, so that generic serialization
   and editor UI handle collections without per-container specialization
   - **Acceptance:** Uniform trait interface for arrays, vectors, maps, sets<br>Insertion, removal,
     iteration, indexed access<br>Editor UI renders collections generically
4. **US-1.3.8** — to verify that collections serialized and deserialized through the reflection
   interface produce identical values, so that no data is lost or corrupted when reflecting vectors,
   maps, and nested collections
   - **Acceptance:** Vec<T> round-trips through reflection with identical values<br>HashMap<K,V>
     round-trips preserving all entries<br>Nested collections (Vec<Vec<T>>) round-trip correctly

## Dynamic Access

| ID        | Persona                 | Features | Requirements |
|-----------|-------------------------|----------|--------------|
| US-1.3.9  | engine developer (P-26) | F-1.3.5  | R-1.3.5      |
| US-1.3.10 | game developer (P-15)   | F-1.3.5  | R-1.3.5      |

1. **US-1.3.9** — a `DynamicValue` type that holds any reflected value and supports diffing and
   patching, so that serialization, network replication, and the ECS world interchange data through
   a single format
   - **Acceptance:** DynamicValue holds any reflected value<br>Diffing produces minimal change
     set<br>Patches applicable to typed values
2. **US-1.3.10** — dynamic values constructed from serialized data to diff against typed values and
   produce minimal patches, so that network replication sends only changed fields rather than full
   component state
   - **Acceptance:** Dynamic value constructable from binary data<br>Diff against typed value yields
     minimal patch<br>Patch application produces correct typed value

## Attribute / Metadata System

| ID        | Persona                 | Features | Requirements |
|-----------|-------------------------|----------|--------------|
| US-1.3.11 | game developer (P-15)   | F-1.3.6  | R-1.3.6      |
| US-1.3.12 | QA engineer (P-19)      | F-1.3.6  | R-1.3.6      |
| US-1.3.13 | engine developer (P-26) | F-1.3.7  | R-1.3.7      |
| US-1.3.14 | engine tester (P-27)    | F-1.3.7  | R-1.3.7      |

1. **US-1.3.11** — custom key-value metadata on types and fields (display names, tooltip text,
   numeric ranges, serialization hints, replication policies), so that the editor, serializer, and
   network layer adapt behavior based on attributes
   - **Acceptance:** Key-value metadata attachable at registration time<br>Standard attributes:
     display name, range, skip, rename<br>Attributes queryable at runtime
2. **US-1.3.12** — to verify that the visual editor respects field metadata like numeric ranges,
   display names, and serialization skip hints, so that designers see correct labels, value
   constraints, and hidden fields
   - **Acceptance:** Range attribute enforces min/max in editor slider<br>Display name shown instead
     of field identifier<br>Skip attribute hides field from editor
3. **US-1.3.13** — trait implementations registered against type IDs for runtime resolution, so that
   generic systems invoke trait methods on type-erased data while keeping the type system compatible
   with static dispatch
   - **Acceptance:** Trait implementations registered against type IDs<br>Runtime resolution given
     only a type ID<br>No virtual dispatch or trait objects used
4. **US-1.3.14** — to verify that runtime trait resolution produces the same results as compile-time
   static dispatch for all registered types, so that the dynamic path is functionally equivalent to
   the static path
   - **Acceptance:** Runtime Serialize produces same output as static Serialize<br>All registered
     types resolve their trait impls correctly<br>Missing trait registration reported as error, not
     silent failure

## Reflect Trait

| ID | Persona | Features | Requirements |
|-----|---------|----------|--------------|
| US-1.3.15 | game developer (P-15) | F-1.3.8 | R-1.3.8 |
| US-1.3.16 | game developer (P-15) | F-1.3.8, F-1.3.8, F-1.3.9 | R-1.3.8, R-1.3.8, R-1.3.9 |
| US-1.3.17 | game developer (P-15) | F-1.3.8 | R-1.3.8a |
| US-1.3.18 | engine developer (P-26) | F-1.3.9 | R-1.3.9 |
| US-1.3.19 | designer (P-5) | F-1.3.9 | R-1.3.9 |
| US-1.3.20 | engine developer (P-26) | F-1.3.10 | R-1.3.10 |
| US-1.3.21 | engine tester (P-27) | F-1.3.10, F-1.3.10, F-1.3.4, F-1.3.10, F-1.3.8 | R-1.3.10 |
| US-1.3.22 | engine tester (P-27) | F-1.3.8 | R-1.3.8a |

1. **US-1.3.15** — to derive `Reflect` on my component structs and have all property accessors, type
   registration, and collection trait implementations generated automatically, so that my types work
   with the editor, serializer, and network replication without boilerplate
   - **Acceptance:** Derive macro generates Reflect impl for struct<br>Fields accessible by name and
     index<br>Type auto-registered in TypeRegistry
2. **US-1.3.16** — to derive `Reflect` on enum types so that each variant and its fields are
   accessible through the reflection API, so that state machines, event types, and config enums work
   with the editor and serializer
   - **Acceptance:** Derive macro generates Reflect impl for enum<br>Variant names and fields
     enumerable at runtime<br>Active variant readable and writable
3. **US-1.3.17** — to annotate fields with `#[reflect(skip)]`, `#[reflect(rename)]`, and
   `#[reflect(default)]` to control reflection behavior per field, so that internal fields stay
   hidden and serialization uses clean names
   - **Acceptance:** `#[reflect(skip)]` excludes field from
     reflection<br>`#[reflect(rename = "...")]` overrides field name<br>`#[reflect(default)]`
     provides fallback on missing data
4. **US-1.3.18** — to downcast a `Reflect` reference to the appropriate sub-trait (`ReflectStruct`,
   `ReflectEnum`, `ReflectList`, `ReflectMap`, `ReflectValue`), so that generic editor inspectors
   and serializers handle each category with the right access methods
   - **Acceptance:** Structs downcast to ReflectStruct<br>Enums downcast to ReflectEnum<br>Lists and
     maps downcast to ReflectList/ReflectMap
5. **US-1.3.19** — the visual editor to display an appropriate inspector widget for each reflected
   type category (struct fields, enum dropdown, list add/remove, map key-value pairs), so that I can
   edit any component without specialized UI code
   - **Acceptance:** Struct fields rendered as labeled inputs<br>Enum variants shown as dropdown
     selection<br>Lists rendered with add/remove buttons
6. **US-1.3.20** — to call `FromReflect` to reconstruct a concrete typed value from a `DynamicValue`
   or `Reflect` reference, so that deserialization and undo/redo produce real typed instances from
   dynamic intermediaries
   - **Acceptance:** FromReflect reconstructs concrete value from dynamic<br>Missing fields filled
     with registered defaults<br>Incompatible dynamic data returns None
7. **US-1.3.21** — to verify that converting a typed value to a `DynamicValue` via `Reflect::clone`
   and back via `FromReflect` produces an identical value, so that the dynamic representation is
   lossless
   - **Acceptance:** typed -> clone -> FromReflect == original<br>All field types preserved through
     round-trip<br>Nested structs and collections round-trip correctly
8. **US-1.3.22** — to verify that using an unsupported attribute on the `Reflect` derive macro
   produces a clear compile-time error, so that developers get immediate feedback on attribute
   mistakes
   - **Acceptance:** Invalid attribute triggers compile error<br>Error message names the invalid
     attribute<br>Valid attributes compile without warnings
