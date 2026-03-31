# Reflection & Type System User Stories

## Type Registry

| ID       | Persona                 |
|----------|-------------------------|
| US-1.3.1 | engine developer (P-26) |
| US-1.3.2 | engine tester (P-27)    |

1. **US-1.3.1** — **As an** engine developer (P-26), **I want** a runtime type registry mapping type
   IDs to metadata that is immutable after initialization, **so that** all threads can look up type
   information concurrently without synchronization overhead.
2. **US-1.3.2** — **As an** engine tester (P-27), **I want** to verify that concurrent type registry
   lookups from multiple threads produce correct results with no data races, **so that** the
   lock-free registry design holds under contention.

## Type Info

| ID       | Persona                 |
|----------|-------------------------|
| US-1.3.3 | engine developer (P-26) |

1. **US-1.3.3** — **As an** engine developer (P-26), **I want** type descriptors storing size,
   alignment, drop, clone, default constructor, and field layout, **so that** I can perform safe
   dynamic operations on type-erased data for serialization and editor tooling.

## Property System

| ID       | Persona                 |
|----------|-------------------------|
| US-1.3.4 | game developer (P-15)   |
| US-1.3.5 | technical artist (P-13) |
| US-1.3.6 | engine developer (P-26) |
| US-1.3.7 | engine tester (P-27)    |

1. **US-1.3.4** — **As a** game developer (P-15), **I want** to read and write struct fields by name
   using path-based access like transform.position.x, **so that** I can bind animation curves and
   build data-driven configuration without boilerplate.
2. **US-1.3.5** — **As a** technical artist (P-13), **I want** to bind animation curves to any
   reflected property path in the editor, **so that** I can animate any numeric field without
   writing code.
3. **US-1.3.6** — **As an** engine developer (P-26), **I want** dynamic-length collections exposed
   through a uniform reflection interface with insertion, removal, and indexed access, **so that**
   generic serialization and editor UI handle collections without per-container specialization.
4. **US-1.3.7** — **As an** engine tester (P-27), **I want** to verify that collections serialized
   and deserialized through the reflection interface produce identical values, **so that** no data
   is lost when reflecting vectors, maps, and nested collections.

## Dynamic Access

| ID       | Persona                 |
|----------|-------------------------|
| US-1.3.8 | engine developer (P-26) |
| US-1.3.9 | game developer (P-15)   |

1. **US-1.3.8** — **As an** engine developer (P-26), **I want** a DynamicValue type that holds any
   reflected value and supports diffing and patching, **so that** serialization, network
   replication, and the ECS world interchange data through a single format.
2. **US-1.3.9** — **As a** game developer (P-15), **I want** dynamic values to diff against typed
   values and produce minimal patches, **so that** network replication sends only changed fields
   rather than full component state.

## Attribute / Metadata System

| ID        | Persona                 |
|-----------|-------------------------|
| US-1.3.10 | game developer (P-15)   |
| US-1.3.11 | engine developer (P-26) |
| US-1.3.12 | engine tester (P-27)    |

1. **US-1.3.10** — **As a** game developer (P-15), **I want** custom key-value metadata on types and
   fields including display names, numeric ranges, serialization hints, and replication policies,
   **so that** the editor and serializer adapt behavior based on attributes.
2. **US-1.3.11** — **As an** engine developer (P-26), **I want** trait implementations registered
   against type IDs for runtime resolution, **so that** generic systems invoke trait methods on
   type-erased data while keeping the type system compatible with static dispatch.
3. **US-1.3.12** — **As an** engine tester (P-27), **I want** to verify that runtime trait
   resolution produces the same results as compile-time static dispatch for all registered types,
   **so that** the dynamic path is functionally equivalent.

## Reflect Trait

| ID        | Persona                 |
|-----------|-------------------------|
| US-1.3.13 | game developer (P-15)   |
| US-1.3.14 | game developer (P-15)   |
| US-1.3.15 | game developer (P-15)   |
| US-1.3.16 | engine developer (P-26) |
| US-1.3.17 | engine developer (P-26) |
| US-1.3.18 | engine tester (P-27)    |

1. **US-1.3.13** — **As a** game developer (P-15), **I want** to derive Reflect on my component
   structs and have all property accessors, type registration, and collection trait implementations
   generated automatically, **so that** my types work with the editor, serializer, and network
   replication without boilerplate.
2. **US-1.3.14** — **As a** game developer (P-15), **I want** to derive Reflect on enum types so
   that each variant and its fields are accessible through the reflection API, **so that** state
   machines, event types, and config enums work with the editor and serializer.
3. **US-1.3.15** — **As a** game developer (P-15), **I want** to annotate fields with reflect(skip),
   reflect(rename), and reflect(default) to control reflection behavior per field, **so that**
   internal fields stay hidden and serialization uses clean names.
4. **US-1.3.16** — **As an** engine developer (P-26), **I want** to downcast a Reflect reference to
   the appropriate sub-trait (ReflectStruct, ReflectEnum, ReflectList, ReflectMap, ReflectValue),
   **so that** generic inspectors handle each category with the right access methods.
5. **US-1.3.17** — **As an** engine developer (P-26), **I want** to call FromReflect to reconstruct
   a concrete typed value from a DynamicValue or Reflect reference, **so that** deserialization and
   undo/redo produce real typed instances from dynamic intermediaries.
6. **US-1.3.18** — **As an** engine tester (P-27), **I want** to verify that converting a typed
   value to DynamicValue via Reflect::clone and back via FromReflect produces an identical value,
   **so that** the dynamic representation is lossless.
