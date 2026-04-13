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

1. **TC-2.7.2.1** `test_cine_lens_vfov_50mm_35mm_sensor` -- 50 mm lens, 35 mm sensor.
   - Expected: vFOV ~= 39.6 degrees.
2. **TC-2.7.2.2** `test_cine_lens_vfov_24mm_35mm_sensor` -- 24 mm lens, 35 mm sensor.
   - Expected: vFOV ~= 73.7 degrees.
3. **TC-2.7.2.3** `test_cine_lens_ev100_sunny_16` -- f/16, 1/100s, ISO 100.
   - Expected: EV100 ~= 8.0 (+/- 0.1).
4. **TC-2.7.2.4** `test_cine_lens_ev100_indoor` -- f/2.8, 1/60s, ISO 400.
   - Expected: EV100 ~= 3.2.
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

## Integration Tests

| ID         | Name                                        | Req     |
|------------|---------------------------------------------|---------|
| TC-2.7.3.1 | `test_pip_two_viewports_composite`          | R-2.7.3 |
| TC-2.7.3.2 | `test_pip_four_viewports_composite`         | R-2.7.3 |
| TC-2.7.3.3 | `test_pip_priority_order_respected`         | R-2.7.3 |
| TC-2.7.4.2 | `test_sequencer_cutscene_end_to_end`        | R-2.7.4 |
| TC-2.7.1.5 | `test_spring_arm_with_real_physics_scene`   | R-2.7.1 |
| TC-2.7.6.4 | `test_dof_pass_renders_without_error`       | R-2.7.6 |
| TC-2.7.7.5 | `test_auto_exposure_adapts_over_sequence`   | R-2.7.7 |

1. **TC-2.7.3.1** `test_pip_two_viewports_composite` -- Two cameras, one full, one small. Assert
   both render; pixel samples match expectations.
2. **TC-2.7.3.2** `test_pip_four_viewports_composite` -- Four cameras. Assert all composed.
3. **TC-2.7.3.3** `test_pip_priority_order_respected` -- Higher priority overlays lower.
4. **TC-2.7.4.2** `test_sequencer_cutscene_end_to_end` -- Timeline-driven three-camera cutscene.
   Frames at each keyframe match reference.
5. **TC-2.7.1.5** `test_spring_arm_with_real_physics_scene` -- Camera orbits a character in a scene
   with walls. No clipping, smooth recovery.
6. **TC-2.7.6.4** `test_dof_pass_renders_without_error` -- DoF enabled on a scene, one frame, assert
   no device errors.
7. **TC-2.7.7.5** `test_auto_exposure_adapts_over_sequence` -- Scene bright->dark->bright.
   Auto-exposure adapts smoothly.

## Benchmarks

| ID           | Name                                  | Target            |
|--------------|---------------------------------------|-------------------|
| TC-2.7.3.B1  | `bench_four_viewport_pip_frame`       | < 8 ms at 1080p   |
| TC-2.7.1.B1  | `bench_spring_arm_256_cameras`        | < 0.2 ms/frame    |
| TC-2.7.6.B1  | `bench_dof_pass_1080p`                | < 1.5 ms          |
| TC-2.7.7.B1  | `bench_histogram_exposure_1080p`      | < 0.3 ms          |
| TC-2.7.5.B1  | `bench_distortion_pass_1080p`         | < 0.2 ms          |

1. **TC-2.7.3.B1** -- Four cameras compositing onto one swapchain. < 8 ms/frame at 1080p.
2. **TC-2.7.1.B1** -- 256 spring-arm cameras updating each frame. < 0.2 ms.
3. **TC-2.7.6.B1** -- Full DoF pass at 1080p. < 1.5 ms.
4. **TC-2.7.7.B1** -- Histogram-based auto-exposure at 1080p. < 0.3 ms.
5. **TC-2.7.5.B1** -- Lens distortion + CA pass at 1080p. < 0.2 ms.
