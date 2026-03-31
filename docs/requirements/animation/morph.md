# R-9.2 -- Morph Target Requirements

## Blend Shapes

1. **R-9.2.1** -- The engine **SHALL** accumulate weighted morph target deltas on the GPU via
   compute shaders with sparse delta storage, applied before skeletal skinning.
   - **Rationale:** GPU-side blend shape accumulation scales to many active targets per mesh without
     CPU overhead; sparse storage minimizes bandwidth.
   - **Verification:** Apply 4 morph targets with known weights. Verify vertex positions match a
     reference within 0.001 units. Measure GPU memory and verify sparse storage reduces footprint
     vs. dense storage.

2. **R-9.2.2** -- The engine **SHALL** activate corrective blend shapes driven by joint angles using
   combination rules.
   - **Rationale:** Corrective shapes fix deformation artifacts at extreme poses without manual
     per-frame adjustment.
   - **Verification:** Bend an elbow past 120 degrees and verify the corrective shape activates.
     Verify no correction at angles below the threshold.

## Facial Animation

1. **R-9.2.3** -- The engine **SHALL** drive facial blend shapes through standardized face action
   units, supporting curve-driven keyframe animation and real-time parameter input.
   - **Rationale:** Standardized action units enable performance capture compatibility and real-time
     lip sync across hundreds of characters.
   - **Verification:** Play a facial clip using ARKit action units and verify correct expressions.
     Drive lip sync parameters in real time and verify mouth shapes correspond.

## Per-Vertex Animation and Streaming

1. **R-9.2.4** -- The engine **SHALL** bake deformations into vertex animation textures sampled in
   the vertex shader with GPU-only playback and zero CPU cost.
   - **Rationale:** VAT enables complex procedural-looking animation for effects like fluid surfaces
     and foliage without per-frame CPU evaluation.
   - **Verification:** Play a VAT animation and verify vertex positions match the source bake.
     Verify CPU frame time shows zero contribution from VAT playback.

2. **R-9.2.5** -- The engine **SHALL** stream morph target delta buffers from disk on demand using
   async I/O with LRU eviction under memory pressure.
   - **Rationale:** Streaming enables MMO-scale character customization without loading all morph
     data into memory simultaneously.
   - **Verification:** Load 100 characters with unique morph targets. Verify only visible targets
     are resident. Verify eviction reclaims memory for off-screen characters.
