# R-1.10 — Algorithms & Shared Primitives Requirements

## GPU Data Upload

1. **R-1.10.1** — The engine **SHALL** support uploading UniformGrid cell data to a GPU buffer for
   shader read access, enabling fog-of-war rendering and density visualization without CPU readback.
   - **Rationale:** Shader-side grid access enables real-time fog-of-war and density heatmaps
     without per-frame CPU-GPU round trips.
   - **Verification:** Upload a 256x256 grid to GPU; verify shader reads match CPU-side values.
     Verify no CPU readback occurs during visualization.

## Graph Compilation

2. **R-1.10.2** — The engine **SHALL** compile visual node graphs into platform shader bytecode
   (SPIR-V, SPIR-V, SPIR-V) via a GraphCompiler, producing CompiledShader output with per-node error
   diagnostics.
   - **Rationale:** Visual node graphs are the primary authoring surface; compiling to native
     bytecode eliminates interpreter overhead.
   - **Verification:** Compile a 20-node material graph to each target; verify valid bytecode
     output. Insert a type mismatch; verify per-node error diagnostic.

## Deterministic RNG

3. **R-1.10.3** — The engine **SHALL** provide a deterministic PRNG with seed-based construction and
   fork() for independent sub-sequences, producing identical output across all platforms for the
   same seed.
   - **Rationale:** Deterministic RNG is required for reproducible AI decisions, loot tables,
     procedural generation, and network-synchronized VFX.
   - **Verification:** Seed with value N on Windows, macOS, and Linux; generate 10,000 values;
     verify bit-identical sequences. Fork twice; verify forked sequences are independent and
     deterministic.

## Condition Expressions

4. **R-1.10.4** — The engine **SHALL** provide a composable boolean expression tree (And, Or, Not,
   Leaf) evaluable against an ECS World context, supporting at least 256 leaf conditions per
   expression.
   - **Rationale:** Condition expressions power quest prerequisites, talent trees, combo chains, and
     unlock gates without hardcoded branching logic.
   - **Verification:** Build a 256-leaf expression with nested And/Or/Not; evaluate against a World
     with known state; verify correct boolean result. Modify one leaf's state; verify result
     changes.

## Frame-Budgeted Work

5. **R-1.10.5** — The engine **SHALL** provide a per-frame time-budgeted work queue that executes
   prioritized work items within a microsecond budget, deferring remaining items to subsequent
   frames.
   - **Rationale:** AI evaluation, physics activation, and VFX updates must respect per-frame time
     budgets to avoid exceeding frame deadlines.
   - **Verification:** Enqueue 1,000 items with a 2 ms budget; verify items execute until budget
     exhausted and remaining defer to next frame. Verify priority ordering is respected.

## Falloff Curves

6. **R-1.10.6** — The engine **SHALL** provide falloff curve primitives (linear, quadratic,
   exponential, custom) evaluable in [0, max_range] for distance-based attenuation in audio,
   lighting, and effects.
   - **Rationale:** Distance-based attenuation is shared across audio, lighting, and VFX; a common
     primitive avoids duplicated curve evaluation code.
   - **Verification:** Evaluate each variant at 0, max_range/2, and max_range; verify expected
     values within f32 epsilon. Verify custom curve with user keypoints interpolates correctly.

## Platform Tier

7. **R-1.10.7** — The engine **SHALL** provide a PlatformTier enumeration with platform-specific
   quality caps (max draw distance, max particle count) queryable at runtime for scalability
   decisions.
   - **Rationale:** A unified tier abstraction enables subsystems to scale quality without
     per-platform branching.
   - **Verification:** Query PlatformTier::Mobile caps; verify draw distance and particle count
     below Desktop caps. Verify all tiers return non-zero values for every cap.

## Compression Codec

8. **R-1.10.8** — The engine **SHALL** provide a CompressionCodec abstraction supporting LZ4 and
   Zstd compression with compress/decompress operations on byte slices.
   - **Rationale:** Asset baking, network delta compression, and binary companion files need a
     shared compression interface.
   - **Verification:** Compress 1 MB of data with LZ4 and Zstd; decompress; verify byte-identical
     output. Verify LZ4 completes in under 2 ms for 1 MB.

## Decaying Values

9. **R-1.10.9** — The engine **SHALL** provide a decaying value store where entries lose strength
   over time at configurable rates, automatically removing entries below a threshold.
   - **Rationale:** AI memory, threat tables, and reputation systems require time-decaying values
     without manual tick-by-tick updates.
   - **Verification:** Insert 100 entries; advance time by 10 seconds; verify entries decayed by
     expected factor. Verify entries below threshold are removed automatically.

## Weighted Lookup

10. **R-1.10.10** — The engine **SHALL** provide a weighted lookup table supporting O(1) key lookup
    and weighted random selection via a DeterministicRng.
    - **Rationale:** Loot tables, spawn pools, and dialogue selection need weighted random selection
      from a keyed table.
    - **Verification:** Insert 100 weighted entries; perform 100,000 weighted selections; verify
      distribution matches weights within 5% chi-squared tolerance. Verify O(1) key lookup.

## Conditional Graphs

11. **R-1.10.11** — The engine **SHALL** provide a directed graph with condition-guarded edges,
    evaluating edge conditions against a ConditionContext to determine reachable nodes from any
    starting node.
    - **Rationale:** Quest graphs, dialogue trees, and talent trees need condition-guarded traversal
      without hardcoded graph logic.
    - **Verification:** Build a 50-node graph with condition-guarded edges; evaluate reachability
      from root with known context; verify only expected nodes are reachable. Change one condition;
      verify reachable set changes.
