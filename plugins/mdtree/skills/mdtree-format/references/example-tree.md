# Example mdtree: Features

A concrete example of a complete mdtree hierarchy.

## Directory Structure

```text
features/
├── README.md
├── 1-rendering/
│   ├── 1.1-core-rendering.md
│   ├── 1.2-lighting-and-materials.md
│   └── 1.3-shadows-and-effects.md
├── 2-advanced-rendering/
│   ├── 2.1-ray-tracing.md
│   └── 2.2-environment.md
├── 3-geometry/
│   ├── 3.1-meshlet-pipeline.md
│   └── 3.2-worlds-and-terrain.md
├── 4-animation/
│   └── 4.1-animation.md
├── 5-ui-and-2d/
│   └── 5.1-ui-and-2d.md
└── 6-tooling-and-io/
    ├── 6.1-shader-and-assets.md
    └── 6.2-io-and-streaming.md
```

## Root README

```markdown
# Features

Hierarchical feature specification. Each feature has a unique identifier
(e.g., `F-2.1.3` is the third feature in section 2.1). Features reference
requirements from [docs/requirements/](../requirements/) by ID.

## Sections

### 1 Rendering
- [1.1 Core Rendering](1-rendering/1.1-core-rendering.md) — culling,
  projection, instancing
- [1.2 Lighting and Materials](1-rendering/1.2-lighting-and-materials.md) —
  forward+, deferred, PBR
- [1.3 Shadows and Effects](1-rendering/1.3-shadows-and-effects.md) —
  shadow maps, AO, SSS

### 2 Advanced Rendering
- [2.1 Ray Tracing](2-advanced-rendering/2.1-ray-tracing.md) — acceleration
  structures, RT reflections
- [2.2 Environment](2-advanced-rendering/2.2-environment.md) — sky,
  volumetrics, clouds, fog
```

## Document File

```markdown
# 1.1 Core Rendering

## F-1.1.1 Direct Lighting

Point, spot, and directional light evaluation with physically-based attenuation.

- **Requirements:** [R-1.1.2](../../requirements/1-architecture/1.1-core-constraints.md)
  GPU-driven rendering

## F-1.1.2 GPU Frustum Culling

Meshlet-level frustum culling on the GPU via a compute pass.

- **Requirements:** [R-1.1.2](../../requirements/1-architecture/1.1-core-constraints.md)
  GPU-driven, [R-1.1.3](../../requirements/1-architecture/1.1-core-constraints.md)
  mesh shaders
```

## Example With R- Prefix (Requirements)

```markdown
# 1.1 Core Constraints

## R-1.1.1 Safe User-Facing API

The user-facing API SHALL be 100% safe Rust.

## R-1.1.2 GPU-Driven Rendering

All rendering SHALL be GPU-driven.
```

## Cross-Reference Pattern

Features referencing requirements from a sibling tree:

```text
docs/
├── features/           ← source tree (has prefix F-)
│   └── 1-rendering/
│       └── 1.1-core-rendering.md
└── requirements/       ← target tree (prefix R-)
    └── 1-architecture/
        └── 1.1-core-constraints.md
```

Link format from features to requirements:

```markdown
- **Requirements:** [R-{req-id}](../../requirements/{section-folder}/{file}.md)
  {description}
```
