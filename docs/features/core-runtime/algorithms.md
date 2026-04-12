# 1.10 — Algorithms & Shared Primitives

## GPU Data Upload

| ID       | Feature                        |
|----------|--------------------------------|
| F-1.10.1 | UniformGrid GPU Buffer Upload |

1. **F-1.10.1** — Upload UniformGrid cell data to a GPU buffer for shader-side read access. Enables
   fog-of-war rendering and density visualization without CPU readback. The upload writes a
   contiguous cell array to a storage buffer accessible from compute and fragment shaders.
   - **Deps:** F-1.7.6 (Memory Budgets)
   - **Platform:** Mobile: max 128x128 grid per upload. Desktop: max 1024x1024. GPU buffer format
     matches platform shader storage buffer requirements.

## Graph Compilation

| ID       | Feature                               |
|----------|---------------------------------------|
| F-1.10.2 | Visual Node Graph to Shader Compiler |

2. **F-1.10.2** — Compile visual node graphs into platform shader bytecode (DXIL, SPIR-V, MSL) via a
   GraphCompiler. The compiler traverses the node graph, generates intermediate HLSL, and invokes
   CLI shader tools (dxc, metal-shaderconverter) to produce final bytecode. Per-node error
   diagnostics report type mismatches, missing connections, and invalid configurations.
   - **Deps:** F-1.10.4 (Condition Expressions)

## Deterministic RNG

| ID       | Feature                            |
|----------|------------------------------------|
| F-1.10.3 | Deterministic PRNG with Forking   |

3. **F-1.10.3** — Provide a deterministic pseudo-random number generator with seed-based
   construction and fork() for independent sub-sequences. The PRNG produces identical output on all
   platforms for the same seed. Used by AI decisions, loot tables, VFX particle emission, and
   procedural generation.
   - **Platform:** All platforms use the same algorithm (no platform-specific RNG). Output is
     bit-identical across architectures.

## Condition Expressions

| ID       | Feature                              |
|----------|--------------------------------------|
| F-1.10.4 | Composable Boolean Expression Tree  |

4. **F-1.10.4** — Provide a composable boolean expression tree with And, Or, Not, and Leaf nodes.
   Expressions evaluate against an ECS World context to determine truth values. Supports at least
   256 leaf conditions per expression. Used for quest prerequisites, talent trees, combo chain
   validation, and unlock gates.
   - **Deps:** F-1.1.17 (Composable Archetype Queries)

## Frame-Budgeted Work

| ID       | Feature                           |
|----------|-----------------------------------|
| F-1.10.5 | Per-Frame Time-Budgeted Work Queue|

5. **F-1.10.5** — Provide a per-frame time-budgeted work queue that executes prioritized work items
   within a configurable microsecond budget. Items that do not complete within the budget are
   deferred to subsequent frames. Priority ordering ensures critical work (AI re-evaluation, physics
   activation) runs before optional work (VFX quality updates).
   - **Platform:** Mobile: default 1 ms budget. Desktop: default 2 ms budget. Configurable per
     platform tier.

## Falloff Curves

| ID       | Feature                            |
|----------|------------------------------------|
| F-1.10.6 | Distance-Based Falloff Curves     |

6. **F-1.10.6** — Provide falloff curve primitives with linear, quadratic, exponential, and custom
   variants. Each curve evaluates over [0, max_range] for distance-based attenuation. Used by audio
   rolloff, light falloff, damage falloff, and VFX intensity scaling. Custom curves accept
   user-defined keypoints with interpolation.
   - **Deps:** F-1.10.3 (Deterministic RNG for noise variants)

## Platform Tier

| ID       | Feature                            |
|----------|------------------------------------|
| F-1.10.7 | PlatformTier Quality Cap Enum     |

7. **F-1.10.7** — Provide a PlatformTier enumeration (Mobile, Switch, Desktop, HighEnd) with
   per-tier quality caps queryable at runtime. Caps include max draw distance, max particle count,
   max shadow cascades, and max concurrent audio sources. Subsystems query caps for scalability
   decisions without per-platform branching.
   - **Platform:** Tier detection is automatic based on hardware capability queries at startup.

## Compression Codec

| ID       | Feature                             |
|----------|-------------------------------------|
| F-1.10.8 | LZ4 and Zstd Compression Codec     |

8. **F-1.10.8** — Provide a CompressionCodec abstraction with LZ4 and Zstd variants. Compress and
   decompress operations work on byte slices. LZ4 is used for real-time compression (network deltas,
   command buffers). Zstd is used for asset baking and binary companion files where higher ratios
   justify slower compression.
   - **Deps:** F-1.7.1 (Arena Allocators for scratch buffers)

## Decaying Values

| ID       | Feature                            |
|----------|------------------------------------|
| F-1.10.9 | Time-Decaying Value Store          |

9. **F-1.10.9** — Provide a decaying value store where entries lose strength over time at
   configurable decay rates. Entries below a configurable threshold are automatically removed during
   tick updates. Used for AI memory (threat awareness fading over time), hate tables, and reputation
   decay.
   - **Deps:** F-1.10.3 (Deterministic RNG for jitter)

## Weighted Lookup

| ID        | Feature                            |
|-----------|------------------------------------|
| F-1.10.10 | Weighted Random Lookup Table      |

10. **F-1.10.10** — Provide a weighted lookup table with O(1) key lookup and weighted random
    selection using a DeterministicRng. Selection uses the alias method for O(1) weighted sampling
    after O(n) table construction. Used for loot tables, spawn pools, dialogue selection, and random
    encounter generation.
    - **Deps:** F-1.10.3 (Deterministic RNG)

## Conditional Graphs

| ID        | Feature                            |
|-----------|------------------------------------|
| F-1.10.11 | Condition-Guarded Directed Graph  |

11. **F-1.10.11** — Provide a directed graph with condition-guarded edges. Each edge carries a
    ConditionExpr evaluated against a ConditionContext to determine traversability. Reachability
    queries return all nodes reachable from a start node given the current context. Used for quest
    state graphs, dialogue trees, talent trees, and recipe chains.
    - **Deps:** F-1.10.4 (Condition Expressions)
