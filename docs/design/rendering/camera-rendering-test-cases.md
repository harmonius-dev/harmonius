# Camera Rendering -- Test Cases

Companion to [camera-rendering.md](camera-rendering.md).

Test case IDs use `TC-2.7.Z.N` format.

## Unit Tests

| ID         | Name                                     | Req     |
|------------|------------------------------------------|---------|
| TC-2.7.2.1 | `test_cine_lens_vfov_50mm_35mm_sensor`   | R-2.7.2 |
| TC-2.7.2.2 | `test_cine_lens_vfov_24mm_35mm_sensor`   | R-2.7.2 |
| TC-2.7.2.3 | `test_cine_lens_ev100_sunny_16`          | R-2.7.2 |
| TC-2.7.2.4 | `test_cine_lens_ev100_indoor`            | R-2.7.2 |
| TC-2.7.1.1 | `test_spring_arm_no_contact_full_length` | R-2.7.1 |
| TC-2.7.1.2 | `test_spring_arm_contact_retract_rate`   | R-2.7.1 |
| TC-2.7.1.3 | `test_spring_arm_restore_smooth`         | R-2.7.1 |
| TC-2.7.1.4 | `test_spring_arm_hits_use_layer_mask`    | R-2.7.1 |
| TC-2.7.6.1 | `test_dof_coc_at_focus`                  | R-2.7.6 |
| TC-2.7.6.2 | `test_dof_coc_behind_focus`              | R-2.7.6 |
| TC-2.7.6.3 | `test_dof_coc_in_front_of_focus`         | R-2.7.6 |
| TC-2.7.7.1 | `test_exposure_manual_constant`          | R-2.7.7 |
| TC-2.7.7.2 | `test_exposure_auto_histogram_midgray`   | R-2.7.7 |
| TC-2.7.7.3 | `test_exposure_clamped_to_range`         | R-2.7.7 |
| TC-2.7.7.4 | `test_exposure_adaptation_rate`          | R-2.7.7 |
| TC-2.7.5.1 | `test_distortion_zero_is_identity`       | R-2.7.5 |
| TC-2.7.5.2 | `test_distortion_barrel_outside_center`  | R-2.7.5 |
| TC-2.7.5.3 | `test_chromatic_aberration_rgb_offset`   | R-2.7.5 |
| TC-2.7.8.1 | `test_layer_mask_excludes_entity`        | R-2.7.8 |
| TC-2.7.8.2 | `test_layer_mask_intersection_logic`     | R-2.7.8 |
| TC-2.7.4.1 | `test_sequencer_blend_ab`                | R-2.7.4 |
| TC-2.7.4.2 | `test_sequencer_cutscene_end_to_end`     | R-2.7.4 |

1. **TC-2.7.2.1** `test_cine_lens_vfov_50mm_35mm_sensor` -- 50 mm lens, 35 mm sensor width.
   - Expected: vFOV ~= 38.58 degrees (`2 * atan(sensor / (2 * focal))` in degrees).
2. **TC-2.7.2.2** `test_cine_lens_vfov_24mm_35mm_sensor` -- 24 mm lens, 35 mm sensor width.
   - Expected: vFOV ~= 72.20 degrees (same formula).
3. **TC-2.7.2.3** `test_cine_lens_ev100_sunny_16` -- f/16, 1/100s, ISO 100.
   - Expected: EV100 ~= 14.64 (`log2(N^2 / t * (100 / ISO))`).
4. **TC-2.7.2.4** `test_cine_lens_ev100_indoor` -- f/2.8, 1/60s, ISO 400.
   - Expected: EV100 ~= 6.88 (same formula).
5. **TC-2.7.1.1** `test_spring_arm_no_contact_full_length` -- No physics contact.
   - Expected: `current_length` converges to `desired_length`.
6. **TC-2.7.1.2** `test_spring_arm_contact_retract_rate` -- Sweep reports hit at 2 m, desired is 3
   m.
   - Expected: `current_length` retracts toward 2 m at `retract_rate`.
7. **TC-2.7.1.3** `test_spring_arm_restore_smooth` -- After obstruction clears, restore.
   - Expected: no overshoot, monotonic approach.
8. **TC-2.7.1.4** `test_spring_arm_hits_use_layer_mask` -- Sphere cast with layer mask.
9. **TC-2.7.6.1** `test_dof_coc_at_focus` -- CoC at `focus_distance_m`.
   - Expected: 0.
10. **TC-2.7.6.2** `test_dof_coc_behind_focus` -- Sample behind focus increases CoC monotonically.
11. **TC-2.7.6.3** `test_dof_coc_in_front_of_focus` -- Same for in front.
12. **TC-2.7.7.1** `test_exposure_manual_constant` -- Manual mode produces constant EV.
13. **TC-2.7.7.2** `test_exposure_auto_histogram_midgray` -- 18% gray histogram converges to target
    EV.
14. **TC-2.7.7.3** `test_exposure_clamped_to_range` -- Output never leaves [min_ev, max_ev].
15. **TC-2.7.7.4** `test_exposure_adaptation_rate` -- Over 1 second at 60 fps, EV changes by
    `adaptation_speed`.
16. **TC-2.7.5.1** `test_distortion_zero_is_identity` -- `k1 = k2 = 0` produces identity.
17. **TC-2.7.5.2** `test_distortion_barrel_outside_center` -- Positive k1 pushes outer pixels
    outward.
18. **TC-2.7.5.3** `test_chromatic_aberration_rgb_offset` -- R and B channels offset from G.
19. **TC-2.7.8.1** `test_layer_mask_excludes_entity` -- Entity with mask bit 1 not rendered by
    camera with mask bit 2.
20. **TC-2.7.8.2** `test_layer_mask_intersection_logic` -- Non-zero intersection renders.
21. **TC-2.7.4.1** `test_sequencer_blend_ab` -- Two-camera blend at t=0.5.
    - Expected: camera pose midway between A and B.
22. **TC-2.7.4.2** `test_sequencer_cutscene_end_to_end` -- Multi-slot `CameraSequencerState` returns
    expected blended positions for staged `(index_a, index_b, blend_t)` samples.

## Integration Tests

| ID         | Name                                           | Req     |
|------------|------------------------------------------------|---------|
| TC-2.7.3.1 | `test_pip_two_viewports_composite`             | R-2.7.3 |
| TC-2.7.3.2 | `test_pip_four_viewports_composite`            | R-2.7.3 |
| TC-2.7.3.3 | `test_pip_priority_order_respected`            | R-2.7.3 |
| TC-2.7.1.5 | `test_spring_arm_with_scripted_obstruction_world` | R-2.7.1 |
| TC-2.7.6.4 | `test_dof_pass_settings_validate_for_pass`     | R-2.7.6 |
| TC-2.7.7.5 | `test_auto_exposure_adapts_over_sequence`      | R-2.7.7 |

1. **TC-2.7.3.1** `test_pip_two_viewports_composite` -- Assert deterministic `composite_order` for
   two PiP cameras (ordering slice of R-2.7.3).
2. **TC-2.7.3.2** `test_pip_four_viewports_composite` -- Same for four cameras.
3. **TC-2.7.3.3** `test_pip_priority_order_respected` -- Higher `priority` is composited after lower
   priority (drawn on top).
4. **TC-2.7.1.5** `test_spring_arm_with_scripted_obstruction_world` -- Scripted `SpringArmWorld`
   toggles obstruction; boom stays off the pivot and returns near desired length.
5. **TC-2.7.6.4** `test_dof_pass_settings_validate_for_pass`
   - `DofSettings::validate_for_pass` accepts finite positive inputs; GPU smoke stays deferred.
6. **TC-2.7.7.5** `test_auto_exposure_adapts_over_sequence`
   - Histogram swings bright to dark to bright; EV ordering stays consistent with the swings.

### Deferred GPU coverage (same requirement IDs, render harness)

Pixel compositing, swap-chain frames, and timeline frame grabs for PiP / DoF / sequencer remain
tracked here but require GPU-backed tests once the headless render fixture exists.

## Benchmarks

| ID           | Name                                  | Target            |
|--------------|---------------------------------------|-------------------|
| TC-2.7.3.B1  | `bench_four_viewport_pip_frame`       | < 8 ms at 1080p   |
| TC-2.7.1.B1  | `bench_spring_arm_256_cameras`        | < 0.2 ms/frame    |
| TC-2.7.6.B1  | `bench_dof_pass_1080p`                | < 1.5 ms          |
| TC-2.7.7.B1  | `bench_histogram_exposure_1080p`      | < 0.3 ms          |
| TC-2.7.5.B1  | `bench_distortion_pass_1080p`         | < 0.2 ms          |

Criterion benches for these IDs are **not** in the `rendering_camera` crate yet; keep the targets as
the performance contract and add benches when the subsystem shares a stable frame harness with CI.

1. **TC-2.7.3.B1** -- Four cameras compositing onto one swapchain. < 8 ms/frame at 1080p.
2. **TC-2.7.1.B1** -- 256 spring-arm cameras updating each frame. < 0.2 ms.
3. **TC-2.7.6.B1** -- Full DoF pass at 1080p. < 1.5 ms.
4. **TC-2.7.7.B1** -- Histogram-based auto-exposure at 1080p. < 0.3 ms.
5. **TC-2.7.5.B1** -- Lens distortion + CA pass at 1080p. < 0.2 ms.
