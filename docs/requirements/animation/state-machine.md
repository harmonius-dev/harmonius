# R-9.4 -- Animation State Machine Requirements

| ID       | Derived From                                          |
|----------|-------------------------------------------------------|
| R-9.4.1  | [F-9.4.1](../../features/animation/state-machine.md)  |
| R-9.4.2  | [F-9.4.2](../../features/animation/state-machine.md)  |
| R-9.4.3  | [F-9.4.3](../../features/animation/state-machine.md)  |
| R-9.4.4  | [F-9.4.4](../../features/animation/state-machine.md)  |
| R-9.4.5  | [F-9.4.5](../../features/animation/state-machine.md)  |
| R-9.4.6  | [F-9.4.6](../../features/animation/state-machine.md)  |
| R-9.4.7  | [F-9.4.7](../../features/animation/state-machine.md)  |
| R-9.4.8  | [F-9.4.8](../../features/animation/state-machine.md)  |
| R-9.4.9  | [F-9.4.9](../../features/animation/state-machine.md)  |
| R-9.4.10 | [F-9.4.10](../../features/animation/state-machine.md) |

1. **R-9.4.1** — The engine **SHALL** evaluate a CPU-side declarative state graph each frame,
   producing blend descriptors (clip references, weights, playback times) uploaded to the GPU, with
   per-character graph instances sharing graph definitions to minimize memory overhead.
   - **Rationale:** A declarative state graph separates animation logic from evaluation, enabling
     shared definitions across thousands of characters with minimal per-instance memory.
   - **Verification:** Create a state graph with 10 states and instantiate it for 1000 characters.
     Verify each instance produces correct blend descriptors by comparing against a single-instance
     reference. Measure per-instance memory overhead and verify it is below 1 KB (excluding clip
     data). Confirm blend descriptors are uploaded to the GPU each frame via GPU capture inspection.
2. **R-9.4.2** — The engine **SHALL** transition between states with configurable blend duration,
   blend curve type (linear, ease-in/out, cubic), per-bone blend profiles, and sync marker alignment
   to prevent foot sliding during locomotion transitions.
   - **Rationale:** Sync marker alignment ensures cyclic animations (walk/run) maintain foot contact
     timing during transitions, and per-bone profiles allow body parts to transition at different
     rates.
   - **Verification:** Transition from walk to run with sync markers on foot contacts. Verify foot
     contact frames align within 1 frame between source and destination clips. Apply a per-bone
     blend profile with upper-body transitioning in 0.2 seconds and lower-body in 0.5 seconds, and
     verify each body part completes its transition at the configured time within 1 frame tolerance.
3. **R-9.4.3** — The engine **SHALL** support encapsulation of state groups into reusable sub-state
   machines with defined entry and exit points, composable hierarchically and shareable across
   character archetypes.
   - **Rationale:** Sub-state machines manage complexity by encapsulating related states (e.g.,
     combat) into reusable modules shared across character types.
   - **Verification:** Create a "combat" sub-state machine with 5 states and nest it inside two
     different top-level graphs. Verify transitions into and out of the sub-state machine use the
     defined entry and exit points. Modify the sub-state machine definition and verify both
     top-level graphs reflect the change without duplication.
4. **R-9.4.4** — The engine **SHALL** run multiple state machine instances in parallel on separate
   layers with per-bone masks and blend modes (override, additive), with dynamically adjustable
   layer weights.
   - **Rationale:** Parallel layers enable independent state machines for body regions (upper/lower
     body, face) and additive overlays (hit reactions) without cross-contamination.
   - **Verification:** Run an upper-body combat state machine and a lower-body locomotion state
     machine on separate layers with a spine bone mask boundary. Verify upper-body bones follow the
     combat layer and lower-body bones follow locomotion. Set the combat layer weight to 0.0 and
     verify the character reverts entirely to the locomotion pose within 1 frame.
5. **R-9.4.5** — The engine **SHALL** expose named parameters (booleans, floats, integers, triggers)
   that drive state transitions via boolean expressions, with trigger parameters auto-resetting
   after consumption to ensure one-shot events fire exactly once.
   - **Rationale:** Named parameters decouple gameplay logic from animation graph internals, and
     auto-resetting triggers prevent duplicate event firing.
   - **Verification:** Set a float parameter `speed` to 2.0 and verify the transition rule
     `speed > 1.0` evaluates to true, triggering the expected state change. Set a trigger parameter
     `jump` and verify it fires exactly once and auto-resets to false on the next frame. Set `jump`
     again on the same frame and verify it does not fire a second transition.
6. **R-9.4.6** — The engine **SHALL** keep all animation clips within a sync group
   phase-synchronized by advancing them in normalized time, maintaining sync marker alignment
   regardless of individual playback rates.
   - **Rationale:** Sync groups ensure locomotion blends (walk-to-run) maintain consistent foot
     contact timing, eliminating foot sliding when blending between speeds.
   - **Verification:** Place walk (1.0 s) and run (0.6 s) clips in a sync group with foot-contact
     sync markers. Blend at 50% weight and verify foot-contact markers align within 1 frame
     throughout a full cycle. Verify the walk clip plays at a faster normalized rate and the run
     clip at a slower normalized rate to maintain phase alignment.
7. **R-9.4.7** — The engine **SHALL** play one-shot or looping animation sequences that temporarily
   override state machine output on specific bone groups, supporting branching sections, notify
   events, and configurable blend-in/blend-out curves.
   - **Rationale:** Montages enable gameplay-triggered animations (emotes, abilities, interactions)
     that override the state machine without disrupting its internal state.
   - **Verification:** Play a montage on the upper body while locomotion runs on the lower body.
     Verify upper-body bones follow the montage and lower-body bones follow the state machine.
     Trigger a branch at section 2 of a 3-section montage and verify playback jumps to section 2.
     Verify blend-out completes in the configured duration and the state machine resumes seamlessly.
8. **R-9.4.8** — The engine **SHALL** interpolate between animation clips based on one or two
   continuous parameters using triangulated sample points and barycentric weight evaluation for the
   active triangle in 2D blend spaces.
   - **Rationale:** Blend spaces provide continuous parameterized animation (e.g., speed, direction)
     without discrete state transitions, producing smooth locomotion at any speed or angle.
   - **Verification:** Create a 2D blend space with 9 sample points (3x3 grid of speed x direction).
     Set parameters to the center of a triangle and verify the resulting pose is the barycentric
     interpolation of the 3 triangle vertices within 0.001 units per joint. Sweep the speed
     parameter from 0 to 1 and verify no discontinuities in the resulting pose by sampling at 100
     intermediate values.
9. **R-9.4.9** — The engine **SHALL** apply additive aim offset layers parameterized by pitch and
   yaw as 2D blend spaces, producing per-bone additive rotations on top of any locomotion state with
   per-bone masking and IK integration for weapon alignment.
   - **Rationale:** Aim offsets decouple aiming direction from locomotion state, enabling any
     locomotion pose to aim in any direction without combinatorial clip authoring.
   - **Verification:** Set aim offset pitch to 45 degrees up and yaw to 30 degrees right. Verify the
     weapon direction vector points within 2 degrees of the target direction. Verify lower-body
     bones are unaffected by the aim offset (position difference below 0.001 units from the base
     pose). Verify IK adjusts the weapon end-effector to align with the aim target within 0.05
     units.
10. **R-9.4.10** — The engine **SHALL** allow behavior trees and GOAP planners to trigger animation
    state transitions and query animation state (current state, remaining clip time, root motion
    displacement, montage notify status) through the logic graph system.
    - **Rationale:** AI-animation integration enables behavior-driven animation control and allows
      AI decision-making to synchronize with animation timing for coordinated gameplay responses.
    - **Verification:** Configure a behavior tree node to set the `attack` trigger parameter. Verify
      the animation state machine transitions to the attack state within 1 frame. Query remaining
      clip time from the AI and verify it returns the correct value within 0.01 seconds. Query root
      motion displacement and verify it matches the actual character displacement within 0.01 units.
