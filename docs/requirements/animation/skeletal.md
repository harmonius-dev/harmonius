# R-9.1 -- Skeletal Animation Requirements

| ID       | Derived From                                     |
|----------|--------------------------------------------------|
| R-9.1.1  | [F-9.1.1](../../features/animation/skeletal.md)  |
| R-9.1.2  | [F-9.1.2](../../features/animation/skeletal.md)  |
| R-9.1.3  | [F-9.1.3](../../features/animation/skeletal.md)  |
| R-9.1.4  | [F-9.1.4](../../features/animation/skeletal.md)  |
| R-9.1.5  | [F-9.1.5](../../features/animation/skeletal.md)  |
| R-9.1.6  | [F-9.1.6](../../features/animation/skeletal.md)  |
| R-9.1.7  | [F-9.1.7](../../features/animation/skeletal.md)  |
| R-9.1.8  | [F-9.1.8](../../features/animation/skeletal.md)  |
| R-9.1.9  | [F-9.1.9](../../features/animation/skeletal.md)  |
| R-9.1.10 | [F-9.1.10](../../features/animation/skeletal.md) |

1. **R-9.1.1** — The engine **SHALL** transform vertices by weighted bone matrices on the GPU using
   compute shaders, supporting both linear blend skinning and dual-quaternion skinning with bone
   palettes uploaded as world-space matrices each frame.
   - **Rationale:** GPU compute skinning offloads per-vertex deformation from the CPU and enables
     skinning thousands of characters without CPU bottlenecks.
   - **Verification:** Skin a mesh with a twist joint (e.g., forearm rotation of 180 degrees) using
     both linear blend and dual-quaternion modes. Verify dual-quaternion mode produces no
     candy-wrapping artifacts by comparing vertex positions against a reference mesh. Confirm all
     bone matrices are read from a single GPU buffer via GPU capture inspection.
2. **R-9.1.2** — The engine **SHALL** evaluate animation curves entirely on the GPU using Hermite
   interpolation, sampling per-joint translation, rotation, and scale tracks at arbitrary time
   offsets in a single compute dispatch.
   - **Rationale:** GPU-side keyframe evaluation eliminates CPU-GPU data transfer for pose
     computation and scales to thousands of animated entities per frame.
   - **Verification:** Play a 60-frame animation clip at 30 FPS and sample at frame 15.5. Compare
     the interpolated joint transforms against a CPU reference Hermite evaluator and verify
     positional error is below 0.001 units per joint. Confirm looped, clamped, and ping-pong modes
     produce correct wrap behavior at clip boundaries.
3. **R-9.1.3** — The engine **SHALL** blend up to 8 simultaneous animation clips per skeleton using
   linear interpolation or cubic Hermite curves, producing smooth transitions between poses from
   CPU-computed blend descriptors.
   - **Rationale:** Multi-clip blending enables smooth state transitions and layered animation
     without requiring a unique authored clip for every combination.
   - **Verification:** Blend 8 clips with equal weights and verify the resulting pose matches a CPU
     reference weighted average within 0.001 units per joint. Verify cubic blend curves produce
     smooth second-derivative-continuous transitions by sampling at 10 intermediate blend weights
     and confirming no discontinuities.
4. **R-9.1.4** — The engine **SHALL** apply animations on separate layers with per-bone masks and
   support additive layers that encode pose deltas relative to a reference pose, enabling overlay
   animations without per-combination clip authoring.
   - **Rationale:** Layered animation with bone masks separates upper-body and lower-body animation,
     and additive layers enable composable reactions without combinatorial clip explosion.
   - **Verification:** Apply an upper-body combat animation layer with a bone mask while a
     lower-body locomotion layer plays. Verify masked bones follow the combat layer exclusively and
     unmasked bones follow locomotion exclusively. Apply an additive breathing layer and verify the
     resulting pose equals the base pose plus the additive delta within 0.001 units per joint.
5. **R-9.1.5** — The engine **SHALL** batch-evaluate animation for at least 1000 skeleton instances
   in a single compute dispatch using an arena buffer, grouping instances that share the same
   animation clip for optimal GPU occupancy.
   - **Rationale:** Instanced evaluation eliminates per-instance CPU overhead, enabling MMO-scale
     crowds and armies without linear CPU cost scaling.
   - **Verification:** Spawn 1000 skeleton instances sharing 10 animation clips. Verify all
     instances produce correct poses by comparing a random sample of 50 instances against
     single-instance evaluation. Measure that GPU dispatch count is 1 (single dispatch) and total
     frame time for animation evaluation is under 2 ms on reference hardware.
6. **R-9.1.6** — The engine **SHALL** extract translation and rotation deltas from the root bone of
   animation clips and apply them to the character's world transform via the physics system,
   separating authored locomotion from gameplay movement.
   - **Rationale:** Root motion ensures animation-driven movement matches visual displacement
     exactly, preventing foot sliding during authored traversal sequences.
   - **Verification:** Play a dodge-roll animation with root motion enabled. Measure the character's
     world-space displacement and verify it matches the root bone delta within 0.01 units. Confirm
     the physics capsule moves with the root motion and that collision detection remains active
     throughout the traversal.
7. **R-9.1.7** — The engine **SHALL** compress animation clip data using variable-rate quantization
   per track, achieving at least 10:1 compression ratio on typical humanoid clips while maintaining
   visual fidelity below a configurable error threshold.
   - **Rationale:** High compression ratios are critical for streaming thousands of unique
     animations in an open-world game without exceeding memory budgets.
   - **Verification:** Compress a humanoid animation clip and verify the compression ratio is at
     least 10:1. Play the compressed clip alongside the uncompressed original and verify per-joint
     positional error is below 0.5 mm. Verify rotation tracks use smallest-three quaternion encoding
     by inspecting the compressed data format.
8. **R-9.1.8** — The engine **SHALL** retarget animation clips from a source skeleton to a target
   skeleton with different bone counts and proportions, using a shared canonical pose and per-bone
   retargeting modes (direct copy, scaled translation, rotation only).
   - **Rationale:** Retargeting enables animation reuse across character archetypes and species,
     reducing content authoring costs for diverse character rosters.
   - **Verification:** Retarget a walk cycle from a standard humanoid to a skeleton with 20% longer
     legs and 10% shorter arms. Verify foot contact timing is preserved, leg stride scales
     correctly, and finger animations copy directly. Retarget human mocap to a quadruped skeleton
     using manual bone remapping and verify the result produces no NaN transforms or bone
     explosions.
9. **R-9.1.9** — The engine **SHALL** fire named events embedded at specific frames within animation
   clips as typed event components through the ECS observer system, supporting branching events,
   window events, and montage-scoped events.
   - **Rationale:** Animation events synchronize gameplay logic (sound, VFX, hit detection) with
     visual playback without polling or manual time checks.
   - **Verification:** Embed a footstep event at frame 10 and a hit-window event spanning frames
     20-25 in a clip. Play the clip and verify the footstep event fires exactly once when playback
     crosses frame 10. Verify the hit-window event is active for exactly 6 frames. Verify a
     montage-scoped event does not fire during non-montage playback.
10. **R-9.1.10** — The engine **SHALL** reduce animation processing cost for distant or occluded
    entities using at least 4 LOD tiers, transitioning smoothly over configurable ranges using the
    shared spatial index distance, and maintaining a per-entity LOD bias for hero characters.
    - **Rationale:** Animation LOD prevents distant characters from consuming the same compute
      budget as nearby ones, critical for maintaining frame rate with hundreds of visible
      characters.
    - **Verification:** Place 200 animated characters at distances from 5 m to 500 m. Verify
      characters within 10 m use full skeletal evaluation, characters at 50 m use a reduced bone
      set, characters at 200 m use half bone count at lower update rate, and characters at 500 m use
      VAT playback. Set a hero character LOD bias and verify it uses full evaluation at 200 m.
      Verify no visible popping during LOD transitions by recording a flythrough and inspecting
      frame-by-frame.
