# Algorithms & Shared Primitives User Stories

## GPU Data Upload

| ID        | Persona                 |
|-----------|-------------------------|
| US-1.10.1 | engine developer (P-26) |

1. **US-1.10.1** — **As an** engine developer (P-26), **I want** to upload UniformGrid cell data to
   a GPU buffer for shader read access, **so that** fog-of-war rendering and density visualization
   run entirely on the GPU without CPU readback.

## Curves and Springs

| ID        | Persona               |
|-----------|-----------------------|
| US-1.10.2 | game developer (P-15) |
| US-1.10.3 | game developer (P-15) |

1. **US-1.10.2** — **As a** game developer (P-15), **I want** a generic Curve type supporting
   linear, Catmull-Rom, Bezier, and step interpolation, **so that** I can reuse the same curve
   evaluation for camera paths, weapon recoil, AI response curves, and animation blending.
2. **US-1.10.3** — **As a** game developer (P-15), **I want** a spring-damper evaluator for smooth
   procedural motion, **so that** I can apply springy behavior to cameras, weapons, cloth, and hair
   with a single tunable API.

## Frame-Budgeted Work

| ID        | Persona                 |
|-----------|-------------------------|
| US-1.10.4 | engine developer (P-26) |

1. **US-1.10.4** — **As an** engine developer (P-26), **I want** a frame-budgeted work queue that
   executes prioritized items within a microsecond budget, **so that** AI evaluation, physics
   activation, and VFX updates respect per-frame time budgets without exceeding frame deadlines.

## Deterministic RNG

| ID        | Persona               |
|-----------|-----------------------|
| US-1.10.5 | game developer (P-15) |
| US-1.10.6 | engine tester (P-27)  |

1. **US-1.10.5** — **As a** game developer (P-15), **I want** a deterministic PRNG with seed-based
   construction and fork() for independent sub-sequences, **so that** AI decisions, loot drops, and
   procedural generation are reproducible across platforms for replay and debugging.
2. **US-1.10.6** — **As an** engine tester (P-27), **I want** to verify that the deterministic PRNG
   produces bit-identical sequences on all platforms for the same seed, **so that** cross-platform
   determinism is guaranteed.

## Condition Expressions

| ID        | Persona               |
|-----------|-----------------------|
| US-1.10.7 | game developer (P-15) |

1. **US-1.10.7** — **As a** game developer (P-15), **I want** a composable boolean expression tree
   (And, Or, Not, Leaf) evaluable against ECS World context, **so that** quest prerequisites, talent
   trees, and combo chain validation use a single reusable primitive.

## Falloff Curves

| ID        | Persona               |
|-----------|-----------------------|
| US-1.10.8 | game developer (P-15) |

1. **US-1.10.8** — **As a** game developer (P-15), **I want** falloff curve primitives (linear,
   quadratic, exponential, custom) evaluable over a max range, **so that** audio rolloff, light
   falloff, and damage falloff share a single tunable attenuation API.

## Platform Tier

| ID        | Persona                 |
|-----------|-------------------------|
| US-1.10.9 | engine developer (P-26) |

1. **US-1.10.9** — **As an** engine developer (P-26), **I want** a PlatformTier enum with per-tier
   quality caps queryable at runtime, **so that** subsystems scale draw distance, particle count,
   and shadow cascades without per-platform branching.

## Compression Codec

| ID         | Persona                 |
|------------|-------------------------|
| US-1.10.10 | engine developer (P-26) |

1. **US-1.10.10** — **As an** engine developer (P-26), **I want** a CompressionCodec abstraction
   supporting LZ4 and Zstd, **so that** asset baking, network deltas, and binary companions share a
   single compression interface.

## Decaying Values

| ID         | Persona               |
|------------|-----------------------|
| US-1.10.11 | game developer (P-15) |

1. **US-1.10.11** — **As a** game developer (P-15), **I want** a decaying value store where entries
   lose strength over time and are removed below a threshold, **so that** AI threat memory and
   reputation systems fade naturally without manual per-tick updates.

## Weighted Lookup

| ID         | Persona               |
|------------|-----------------------|
| US-1.10.12 | game developer (P-15) |

1. **US-1.10.12** — **As a** game developer (P-15), **I want** a weighted lookup table with O(1) key
   access and weighted random selection, **so that** loot tables, spawn pools, and dialogue
   selection use a single efficient data structure.

## Conditional Graphs

| ID         | Persona               |
|------------|-----------------------|
| US-1.10.13 | game developer (P-15) |

1. **US-1.10.13** — **As a** game developer (P-15), **I want** a directed graph with
   condition-guarded edges and reachability queries, **so that** quest graphs, dialogue trees, and
   talent trees evaluate traversability from a shared primitive.
