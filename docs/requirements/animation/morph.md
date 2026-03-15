# R-9.2 -- Morph Target Requirements

## R-9.2.1 GPU Blend Shape Accumulation

The engine **SHALL** accumulate weighted morph target deltas (position and normal offsets) on the
GPU via compute shaders before skeletal skinning, supporting an arbitrary number of active targets
per mesh with sparse delta storage.

- **Derived from:** [F-9.2.1](../../features/animation/morph.md)
- **Rationale:** GPU-side blend shape accumulation keeps the deformation pipeline entirely on the
  GPU, avoiding CPU-GPU transfers for morph target weights.
- **Verification:** Apply 16 morph targets simultaneously to a face mesh with varying weights.
  Compare the deformed vertex positions against a CPU reference accumulator and verify positional
  error is below 0.001 units. Verify sparse delta storage skips vertices with zero deltas by
  confirming memory usage scales with active delta count, not total vertex count.

## R-9.2.2 Corrective Blend Shapes

The engine **SHALL** automatically activate corrective morph targets driven by joint angle
combination rules, applying difference-from-expected deltas to fix deformation artifacts at
extreme poses.

- **Derived from:** [F-9.2.2](../../features/animation/morph.md)
- **Rationale:** Corrective shapes fix skinning artifacts (e.g., collapsed elbow volumes) at
  extreme joint angles without requiring manual per-frame artist intervention.
- **Verification:** Configure a corrective shape to activate when elbow bend exceeds 120 degrees.
  Animate the elbow from 0 to 180 degrees and verify the corrective shape weight is 0.0 below
  120 degrees and 1.0 at 180 degrees. Compare the deformed mesh volume at 150 degrees against a
  reference mesh and verify the corrective shape restores at least 90% of the expected volume.

## R-9.2.3 Facial Animation System

The engine **SHALL** drive facial blend shapes through a standardized set of face action units
compatible with performance capture data, supporting both curve-driven keyframe animation and
real-time parameter input for lip sync and expression blending.

- **Derived from:** [F-9.2.3](../../features/animation/morph.md)
- **Rationale:** Standardized action units enable performance capture data to drive any conforming
  face rig, and real-time parameter input supports dynamic lip sync and expression responses.
- **Verification:** Load a performance capture sequence targeting the standard action unit set and
  verify all action units map to the correct blend shapes on the face mesh. Drive lip sync from a
  real-time audio input and verify viseme transitions occur within 1 frame of the audio event.
  Render 100 NPCs with unique expression blends and verify all display distinct facial poses
  simultaneously.

## R-9.2.4 Per-Vertex Animation Textures

The engine **SHALL** play back per-vertex animation textures (VATs) in the vertex shader with zero
CPU cost, where each animation frame is stored as a texel row sampled at the current playback
time.

- **Derived from:** [F-9.2.4](../../features/animation/morph.md)
- **Rationale:** VATs enable complex deformation playback (fluids, tentacles, crowd LOD) with no
  CPU overhead, ideal for decorative and distant animations.
- **Verification:** Bake a 120-frame tentacle animation into a VAT. Play it back and verify the
  deformed mesh matches the source animation within 0.5 mm per vertex. Confirm zero CPU animation
  cost by profiling CPU time with 100 VAT-animated meshes and verifying no increase over a static
  mesh baseline.

## R-9.2.5 Morph Target Streaming

The engine **SHALL** stream morph target delta buffers from disk on demand using platform-native
async I/O, loading only targets needed for currently visible characters and evicting unused targets
under memory pressure with an LRU policy.

- **Derived from:** [F-9.2.5](../../features/animation/morph.md)
- **Rationale:** On-demand streaming with LRU eviction keeps GPU memory bounded regardless of how
  many unique morph sets exist across the character population.
- **Verification:** Spawn 500 characters with unique facial morph sets exceeding the GPU memory
  budget. Verify only morph targets for visible characters are resident in GPU memory. Move the
  camera away from a group of characters and verify their morph targets are evicted within 2
  seconds. Move the camera back and verify morph targets stream in before the characters become
  fully visible (no visible pop-in on morph detail).
