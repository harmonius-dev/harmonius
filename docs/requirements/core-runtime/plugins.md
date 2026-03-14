# API and Extensibility

## User API

### R-5.1.1 Declarative Scene Description

The user-facing API SHALL provide a declarative scene description where users specify entities,
materials, transforms, and render settings. The engine SHALL derive the full execution plan from this
description without user-issued draw calls.

### R-5.1.2 Runtime Quality Configuration

All visual quality settings (shadow resolution, AO quality, RT sample counts, LOD bias,
post-processing toggles) SHALL be adjustable at runtime via a structured quality settings interface
without requiring an application restart. Parameter-level changes (resolution scale, pass activation,
budget thresholds) SHALL take effect within one frame without graph recompilation. Topology-level
changes (variant selection, lighting model, AA mode) MAY trigger graph recompilation but SHALL NOT
require a restart or manual pipeline reconstruction.

### R-5.1.3 Typed Error Reporting

All fallible GPU and IO operations SHALL return typed, recoverable errors as C++ `std::expected`
types. GPU validation errors, resource creation failures, and shader compilation errors SHALL be
surfaced with diagnostic context. No GPU error SHALL cause an unrecoverable abort in the user-facing
API.

### R-5.1.4 Generational Resource Handles

GPU resource references SHALL use generational index handles. Accessing a resource through a stale
handle (after the resource has been freed) SHALL return a typed error, never undefined behavior or
silent data corruption.

## Extensibility

All extension points use C++20 concepts and static dispatch. User-defined types satisfy concept
contracts and are registered at compile time -- no virtual methods, no vtables, no dynamic dispatch.
This is consistent with R-1.1.5 and the GPU backend's concept-based architecture.

### R-5.2.1 Shader Node Registry

Third-party shader node types SHALL be registrable via a typed node registry. Each registration
SHALL declare a slot signature (input/output types) and an IR lowering function. Registered nodes
SHALL be usable in shader graphs without modifying the core compiler. Node types SHALL satisfy a
`ShaderNode` concept defining the required interface.

### R-5.2.2 Custom Render Graph Passes

Users SHALL be able to declare custom render graph passes specifying resource inputs, outputs, and
queue requirements. Custom passes SHALL participate in the graph compiler's barrier insertion,
resource aliasing, and pass scheduling on equal footing with built-in passes.

### R-5.2.3 Material Model Extensions

Extended material layers (clearcoat, subsurface, sheen, anisotropy) SHALL be implementable as
composable extensions over the base PBR model. Extensions SHALL integrate via the shader graph
system without modifying core lighting shaders.

### R-5.2.4 Custom Animation Evaluators

User-defined animation evaluator types SHALL satisfy an `AnimationEvaluator` concept and be
registrable at compile time for evaluation within the animation state machine. Custom evaluators
SHALL receive parameter state and output per-joint transforms or arbitrary float channels.
