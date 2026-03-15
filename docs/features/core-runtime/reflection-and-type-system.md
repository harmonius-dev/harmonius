# 1.3 — Reflection & Type System

## Type Registry

### F-1.3.1 Global Type Registry

Maintain a runtime registry that maps type identifiers to type metadata. Registration occurs at
startup via inventory collection or explicit calls. The registry is immutable after initialization,
enabling lock-free concurrent reads. Every component, resource, and event type used in the ECS must
be registered to support serialization, editor tooling, and network replication.

- **Requirements:** R-1.3.1
- **Dependencies:** None
- **Platform notes:** None

## Type Info

### F-1.3.2 Structured Type Descriptors

Store rich type information including size, alignment, drop function, clone function, default
constructor, field layout, and variant layout. Type descriptors are sufficient to perform safe
dynamic operations (move, drop, clone) on type-erased data. This is the foundation for the dynamic
property system, serialization, and editor reflection.

- **Requirements:** R-1.3.2
- **Dependencies:** F-1.3.1
- **Platform notes:** None

## Property System

### F-1.3.3 Reflective Property Access

Expose struct fields and enum variants as named properties that can be read and written dynamically
at runtime. Properties support path-based access (e.g., `transform.position.x`) for nested
structures. This enables generic editor inspectors, animation binding, data-driven configuration,
and network delta serialization without hand-written boilerplate for each type.

- **Requirements:** R-1.3.3
- **Dependencies:** F-1.3.2
- **Platform notes:** None

### F-1.3.4 Collection Reflection

Extend the property system to handle dynamic-length collections: arrays, vectors, hash maps, and
hash sets. Collection reflection provides insertion, removal, iteration, and indexed access through
a uniform trait interface. This allows generic serialization and editor UI to handle collections
without specialization per container type.

- **Requirements:** R-1.3.4
- **Dependencies:** F-1.3.3
- **Platform notes:** None

## Dynamic Access

### F-1.3.5 Type-Erased Dynamic Values

Provide a `DynamicValue` type that can hold any reflected value and supports the full property API.
Dynamic values can be constructed from serialized data, diffed against typed values, and applied as
patches. This is the interchange format between the serialization layer, the network replication
layer, and the ECS world.

- **Requirements:** R-1.3.5
- **Dependencies:** F-1.3.3, F-1.3.4
- **Platform notes:** None

## Attribute / Metadata System

### F-1.3.6 Custom Type and Field Attributes

Allow user-defined key-value metadata to be attached to types and individual fields at registration
time. Standard attributes include display names, tooltip text, numeric ranges, serialization hints
(skip, rename, flatten), and replication policies. Attributes are queryable at runtime and drive
behavior in the editor, serializer, and network layer.

- **Requirements:** R-1.3.6
- **Dependencies:** F-1.3.2
- **Platform notes:** None

### F-1.3.7 Trait Object Registration and Dispatch

Register trait implementations against type IDs so that generic systems can invoke trait methods on
type-erased data. For example, a `Serialize` or `Inspect` trait can be resolved at runtime given
only a type ID. This replaces virtual dispatch with an explicit registry, keeping the type system
compatible with static dispatch while enabling runtime polymorphism where needed.

- **Requirements:** R-1.3.7
- **Dependencies:** F-1.3.1
- **Platform notes:** None

## Reflect Trait

### F-1.3.8 Reflect Trait with Derive Macro

Provide a `Reflect` trait modeled after bevy_reflect that serves as the unified entry point for
runtime type introspection. The trait exposes methods for accessing type info, getting and setting
field values by name or index, iterating fields, cloning into a `DynamicValue`, and applying patches
from dynamic data. A derive macro auto-implements `Reflect` for structs and enums, generating all
property accessors, type registration code, and collection trait implementations from the type
definition. The derive macro supports attribute annotations for customizing reflection behavior
(skip, rename, default).

- **Requirements:** R-1.3.8
- **Dependencies:** F-1.3.1, F-1.3.2, F-1.3.3
- **Platform notes:** None

### F-1.3.9 Reflect Sub-Traits for Type Categories

Define sub-traits of `Reflect` for each structural category: `ReflectStruct` for named-field
structs, `ReflectTupleStruct` for tuple structs, `ReflectEnum` for enums, `ReflectList` for ordered
collections, `ReflectMap` for key-value collections, and `ReflectValue` for opaque leaf types. Each
sub-trait provides category-specific access methods. The `Reflect` trait exposes a method to
downcast to the appropriate sub-trait, enabling generic code to handle each category uniformly
without matching on concrete types.

- **Requirements:** R-1.3.9
- **Dependencies:** F-1.3.8, F-1.3.4
- **Platform notes:** None

### F-1.3.10 FromReflect Conversion Trait

Provide a `FromReflect` trait that constructs a concrete typed value from a `DynamicValue` or any
`Reflect` reference. This is the inverse of the `Reflect::clone` operation and enables
round-tripping between dynamic and static representations. The derive macro generates `FromReflect`
alongside `Reflect`, using field defaults for missing values. `FromReflect` returns `Option` to
signal conversion failure for incompatible dynamic data.

- **Requirements:** R-1.3.10
- **Dependencies:** F-1.3.8, F-1.3.5
- **Platform notes:** None
