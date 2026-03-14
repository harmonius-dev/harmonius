# 1.3 — Reflection & Type System

## Type Registry

### F-1.3.1 Global Type Registry

Maintain a runtime registry that maps type identifiers to type metadata. Registration occurs at startup via
inventory collection or explicit calls. The registry is immutable after initialization, enabling lock-free
concurrent reads. Every component, resource, and event type used in the ECS must be registered to support
serialization, editor tooling, and network replication.

- **Requirements:** R-1.3.1
- **Dependencies:** None
- **Platform notes:** None

## Type Info

### F-1.3.2 Structured Type Descriptors

Store rich type information including size, alignment, drop function, clone function, default constructor, field
layout, and variant layout. Type descriptors are sufficient to perform safe dynamic operations (move, drop, clone)
on type-erased data. This is the foundation for the dynamic property system, serialization, and editor reflection.

- **Requirements:** R-1.3.2
- **Dependencies:** F-1.3.1
- **Platform notes:** None

## Property System

### F-1.3.3 Reflective Property Access

Expose struct fields and enum variants as named properties that can be read and written dynamically at runtime.
Properties support path-based access (e.g., `transform.position.x`) for nested structures. This enables generic
editor inspectors, animation binding, data-driven configuration, and network delta serialization without
hand-written boilerplate for each type.

- **Requirements:** R-1.3.3
- **Dependencies:** F-1.3.2
- **Platform notes:** None

### F-1.3.4 Collection Reflection

Extend the property system to handle dynamic-length collections: arrays, vectors, hash maps, and hash sets.
Collection reflection provides insertion, removal, iteration, and indexed access through a uniform trait interface.
This allows generic serialization and editor UI to handle collections without specialization per container type.

- **Requirements:** R-1.3.4
- **Dependencies:** F-1.3.3
- **Platform notes:** None

## Dynamic Access

### F-1.3.5 Type-Erased Dynamic Values

Provide a `DynamicValue` type that can hold any reflected value and supports the full property API. Dynamic values
can be constructed from serialized data, diffed against typed values, and applied as patches. This is the
interchange format between the serialization layer, the network replication layer, and the ECS world.

- **Requirements:** R-1.3.5
- **Dependencies:** F-1.3.3, F-1.3.4
- **Platform notes:** None

## Attribute / Metadata System

### F-1.3.6 Custom Type and Field Attributes

Allow user-defined key-value metadata to be attached to types and individual fields at registration time. Standard
attributes include display names, tooltip text, numeric ranges, serialization hints (skip, rename, flatten), and
replication policies. Attributes are queryable at runtime and drive behavior in the editor, serializer, and network
layer.

- **Requirements:** R-1.3.6
- **Dependencies:** F-1.3.2
- **Platform notes:** None

### F-1.3.7 Trait Object Registration and Dispatch

Register trait implementations against type IDs so that generic systems can invoke trait methods on type-erased
data. For example, a `Serialize` or `Inspect` trait can be resolved at runtime given only a type ID. This replaces
virtual dispatch with an explicit registry, keeping the type system compatible with static dispatch while enabling
runtime polymorphism where needed.

- **Requirements:** R-1.3.7
- **Dependencies:** F-1.3.1
- **Platform notes:** None
