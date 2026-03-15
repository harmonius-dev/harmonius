# Reflection & Type System User Stories

## Type Registry

## US-1.3.1 Maintain a Lock-Free Runtime Type Registry

**As an** engine developer (P-26), **I want** a runtime type registry mapping type IDs to
metadata that is immutable after initialization, **so that** all threads can look up type
information concurrently without synchronization overhead.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Type ID to metadata mapping for all registered types | F-1.3.1 | R-1.3.1 |
| Registry immutable after startup, lock-free reads | F-1.3.1 | R-1.3.1 |
| All component, resource, and event types registered | F-1.3.1 | R-1.3.1 |

## US-1.3.2 Verify Type Registry Concurrent Access Safety

**As an** engine tester (P-27), **I want** to verify that concurrent type registry lookups
from multiple threads produce correct results with no data races, **so that** the lock-free
registry design holds under contention.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Concurrent lookups from 16+ threads return correct metadata | F-1.3.1 | R-1.3.1 |
| No data races under sanitizer testing | F-1.3.1 | R-1.3.1 |
| Registry immutability enforced after initialization | F-1.3.1 | R-1.3.1 |

## US-1.3.3 Use Type Registry for Visual Editor Tooling

**As a** designer (P-5), **I want** the visual editor to discover all registered component
types with their field names, types, and metadata, **so that** I can inspect and edit any
component on any entity in the editor.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Editor enumerates all registered types | F-1.3.1 | R-1.3.1 |
| Field names and types displayed per component | F-1.3.2 | R-1.3.2 |
| Metadata (display names, ranges) drives editor UI | F-1.3.6 | R-1.3.6 |

## Type Info

## US-1.3.4 Store Rich Type Descriptors for Dynamic Operations

**As an** engine developer (P-26), **I want** type descriptors storing size, alignment, drop,
clone, default constructor, and field layout, **so that** I can perform safe dynamic operations
on type-erased data for serialization, editor tooling, and network replication.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Size, alignment, drop, clone, default stored per type | F-1.3.2 | R-1.3.2 |
| Field layout with offsets and nested descriptors | F-1.3.2 | R-1.3.2 |
| Safe move, drop, clone of type-erased data | F-1.3.2 | R-1.3.2 |

## Property System

## US-1.3.5 Access Struct Fields by Name and Path

**As a** game developer (P-15), **I want** to read and write struct fields by name using
path-based access like `transform.position.x`, **so that** I can bind animation curves,
build data-driven configuration, and create generic editor inspectors without boilerplate.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Named property access on struct fields and enum variants | F-1.3.3 | R-1.3.3 |
| Path-based access for nested structures | F-1.3.3 | R-1.3.3 |
| Read and write through property API | F-1.3.3 | R-1.3.3 |

## US-1.3.6 Bind Animation Curves to Properties in the Visual Editor

**As a** designer (P-5), **I want** to bind animation curves to any reflected property path
in the visual editor, **so that** I can animate any numeric field without writing code.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Property paths selectable in editor UI | F-1.3.3 | R-1.3.3 |
| Animation curves drive property values at runtime | F-1.3.3 | R-1.3.3 |
| Nested property paths (position.x, color.r) supported | F-1.3.3 | R-1.3.3 |

## US-1.3.7 Reflect Dynamic Collections Through Uniform Interface

**As an** engine developer (P-26), **I want** dynamic-length collections (vectors, maps, sets)
exposed through a uniform reflection interface with insertion, removal, and indexed access,
**so that** generic serialization and editor UI handle collections without per-container
specialization.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Uniform trait interface for arrays, vectors, maps, sets | F-1.3.4 | R-1.3.4 |
| Insertion, removal, iteration, indexed access | F-1.3.4 | R-1.3.4 |
| Editor UI renders collections generically | F-1.3.4 | R-1.3.4 |

## US-1.3.8 Verify Collection Reflection Round-Trip Fidelity

**As an** engine tester (P-27), **I want** to verify that collections serialized and
deserialized through the reflection interface produce identical values, **so that** no data is
lost or corrupted when reflecting vectors, maps, and nested collections.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Vec<T> round-trips through reflection with identical values | F-1.3.4 | R-1.3.4 |
| HashMap<K,V> round-trips preserving all entries | F-1.3.4 | R-1.3.4 |
| Nested collections (Vec<Vec<T>>) round-trip correctly | F-1.3.4 | R-1.3.4 |

## Dynamic Access

## US-1.3.9 Interchange Data via Type-Erased Dynamic Values

**As an** engine developer (P-26), **I want** a `DynamicValue` type that holds any reflected
value and supports diffing and patching, **so that** serialization, network replication, and
the ECS world interchange data through a single format.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| DynamicValue holds any reflected value | F-1.3.5 | R-1.3.5 |
| Diffing produces minimal change set | F-1.3.5 | R-1.3.5 |
| Patches applicable to typed values | F-1.3.5 | R-1.3.5 |

## US-1.3.10 Use Dynamic Values for Network Delta Serialization

**As a** game developer (P-15), **I want** dynamic values constructed from serialized data to
diff against typed values and produce minimal patches, **so that** network replication sends
only changed fields rather than full component state.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Dynamic value constructable from binary data | F-1.3.5 | R-1.3.5 |
| Diff against typed value yields minimal patch | F-1.3.5 | R-1.3.5 |
| Patch application produces correct typed value | F-1.3.5 | R-1.3.5 |

## Attribute / Metadata System

## US-1.3.11 Attach Custom Metadata to Types and Fields

**As a** game developer (P-15), **I want** custom key-value metadata on types and fields
(display names, tooltip text, numeric ranges, serialization hints, replication policies),
**so that** the editor, serializer, and network layer adapt behavior based on attributes.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Key-value metadata attachable at registration time | F-1.3.6 | R-1.3.6 |
| Standard attributes: display name, range, skip, rename | F-1.3.6 | R-1.3.6 |
| Attributes queryable at runtime | F-1.3.6 | R-1.3.6 |

## US-1.3.12 Verify Editor Respects Field Metadata Attributes

**As a** QA engineer (P-19), **I want** to verify that the visual editor respects field
metadata like numeric ranges, display names, and serialization skip hints, **so that**
designers see correct labels, value constraints, and hidden fields.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Range attribute enforces min/max in editor slider | F-1.3.6 | R-1.3.6 |
| Display name shown instead of field identifier | F-1.3.6 | R-1.3.6 |
| Skip attribute hides field from editor | F-1.3.6 | R-1.3.6 |

## US-1.3.13 Resolve Trait Implementations at Runtime by Type ID

**As an** engine developer (P-26), **I want** trait implementations registered against type
IDs for runtime resolution, **so that** generic systems invoke trait methods on type-erased
data while keeping the type system compatible with static dispatch.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Trait implementations registered against type IDs | F-1.3.7 | R-1.3.7 |
| Runtime resolution given only a type ID | F-1.3.7 | R-1.3.7 |
| No virtual dispatch or trait objects used | F-1.3.7 | R-1.3.7 |

## US-1.3.14 Verify Trait Resolution Matches Static Dispatch Behavior

**As an** engine tester (P-27), **I want** to verify that runtime trait resolution produces
the same results as compile-time static dispatch for all registered types, **so that** the
dynamic path is functionally equivalent to the static path.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Runtime Serialize produces same output as static Serialize | F-1.3.7 | R-1.3.7 |
| All registered types resolve their trait impls correctly | F-1.3.7 | R-1.3.7 |
| Missing trait registration reported as error, not silent failure | F-1.3.7 | R-1.3.7 |
