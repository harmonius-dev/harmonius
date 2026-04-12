# R-9.4 -- Animation State Machine Requirements

## State Graph and Transitions

1. **R-9.4.1** -- The engine **SHALL** provide a CPU-side declarative state graph where each node
   represents a pose source, with shared graph definitions across character instances.
   - **Rationale:** A declarative graph enables visual authoring and memory-efficient instancing for
     MMO-scale character counts.
   - **Verification:** Create a 10-state graph. Verify 100 character instances share the graph
     definition. Verify correct pose output for each state.

2. **R-9.4.2** -- The engine **SHALL** support transitions with configurable blend duration, blend
   curve, per-bone blend profiles, and sync marker alignment.
   - **Rationale:** Polished transitions require per-bone control and footfall sync to avoid sliding
     and popping.
   - **Verification:** Transition between walk and run with sync markers. Verify footfalls align
     during the blend. Verify per-bone profiles apply independently.

3. **R-9.4.3** -- The engine **SHALL** support sub-state machines with defined entry and exit
   points, composable hierarchically and shareable across archetypes.
   - **Rationale:** Modular sub-graphs enable reusable animation behaviors without duplicating
     states across character types.
   - **Verification:** Create a locomotion sub-state machine and share it between two character
     types. Verify both produce correct animation. Verify nesting 3 levels deep works.

## Layers, Variables, and Sync

1. **R-9.4.4** -- The engine **SHALL** run multiple state machine instances in parallel on separate
   layers with per-bone masks, override and additive blend modes, and dynamically adjustable
   weights.
   - **Rationale:** Parallel layers enable independent upper/lower body, facial, and additive
     overlay animation.
   - **Verification:** Run upper-body combat and lower-body locomotion on separate layers. Verify
     masked bones follow the correct layer. Fade a layer weight to zero and verify it has no effect.

2. **R-9.4.5** -- The engine **SHALL** expose named parameters (booleans, floats, integers,
   triggers) that gameplay code sets to drive transitions, with triggers auto-resetting after
   consumption.
   - **Rationale:** Parameterized transitions decouple gameplay logic from animation graph
     authoring.
   - **Verification:** Set a boolean to true and verify the transition fires. Set a trigger and
     verify it fires once then auto-resets.

3. **R-9.4.6** -- The engine **SHALL** support sync groups that keep multiple clips
   phase-synchronized regardless of playback rates.
   - **Rationale:** Sync groups ensure foot contact timing is consistent when blending between walk
     and run cycles.
   - **Verification:** Sync a walk and run clip. Verify foot contact markers align at all blend
     weights.

## Montages, Blend Spaces, and Integration

1. **R-9.4.7** -- The engine **SHALL** support animation montages that temporarily override state
   machine output on specific bone groups, with branching sections, notify events, and
   blend-in/blend-out curves.
   - **Rationale:** Montages enable one-shot gameplay animations without disrupting the base state
     machine flow.
   - **Verification:** Play a montage on the upper body. Verify it overrides the state machine on
     masked bones. Verify blend-out returns control smoothly.

2. **R-9.4.8** -- The engine **SHALL** provide 1D and 2D blend spaces that interpolate between clips
   based on continuous variables using pre-computed triangulation.
   - **Rationale:** Blend spaces produce smooth animation parameterized by speed and direction
     without per-combination authoring.
   - **Verification:** Create a 2D blend space with 9 sample points. Verify barycentric
     interpolation produces correct poses at intermediate values.

3. **R-9.4.9** -- The engine **SHALL** support additive aim offset layers parameterized by pitch and
   yaw with per-bone masking and IK integration (F-9.3.1).
   - **Rationale:** Aim offsets enable weapon aiming on top of locomotion without authored per-angle
     clips.
   - **Verification:** Set aim pitch and yaw. Verify upper-body bones rotate additively. Verify IK
     integration aligns the weapon with the aim point.

4. **R-9.4.10** -- The engine **SHALL** integrate with behavior trees (F-7.3.1) and GOAP planners
   (F-7.5.1) through the Logic Graph system (F-15.8.4), enabling AI agents to drive animation from
   blackboard variables with query APIs for state, clip time, root motion, and montage status.
   - **Rationale:** AI-driven animation requires bidirectional communication between decision
     systems and the animation state machine.
   - **Verification:** Set a blackboard variable from a behavior tree and verify the animation state
     transitions. Query remaining clip time and verify the value matches expected playback position.

## Transition Modes and Pose Sources

1. **R-9.4.11** -- The engine **SHALL** support an inertialization transition blend mode in the
   state machine that samples only the target pose and decays the source offset over the transition
   duration, alongside Crossfade and Frozen modes.
   - **Rationale:** Inertialization costs roughly half of crossfade evaluation and produces smoother
     transitions for motion matching and state graph transitions.
   - **Verification:** Transition from walk to run via inertialization. Assert only the target pose
     is sampled each frame. Assert the offset decays to zero over the configured duration.
2. **R-9.4.12** -- The engine **SHALL** support a Multiply layer blend mode in the animation layer
   stack, scaling bone transforms by layer factors for scaling animations and effects.
   - **Rationale:** Multiply blending produces size pulsing, muscle flex, and magnification effects
     that override and additive layers cannot express.
   - **Verification:** Apply a Multiply layer scaling a bone by 1.5. Assert the final pose has the
     bone scaled relative to the base. Assert disabling the layer returns the base scale.
3. **R-9.4.13** -- The engine **SHALL** expose a `SpriteSheet` pose source in the state machine with
   texture atlas reference, frame list, frames-per-second, and playback mode, allowing 2D
   sprite-sheet animation to reuse all state machine transitions, conditions, layers, and sync
   groups.
   - **Rationale:** 2D games should reuse the state machine infrastructure rather than maintaining a
     separate sprite animation system.
   - **Verification:** Drive a 2D character with a sprite-sheet pose source in a state graph. Assert
     transitions, conditions, and sync groups fire exactly as they do for skeletal animation.
4. **R-9.4.14** -- The engine **SHALL** expose a `MotionMatching` pose source in the state machine
   referencing a pose database and search schema, with inertialization transitions into and out of
   matching states.
   - **Rationale:** State graphs must be able to enter and exit motion matching regions without
     losing transition blend fidelity.
   - **Verification:** Create a state containing a motion matching node. Transition into it via
     inertialization. Assert the pose database drives subsequent frames. Transition out and assert
     inertialization decays the motion-matched offset.

## Montages, Parameters, and Codegen

5. **R-9.4.15** -- The engine **SHALL** resolve overlapping montages on the same bone groups via a
   `u8` priority field on `MontageDef`, with last-played-wins tiebreaking when priorities are equal.
   - **Rationale:** Multiple gameplay systems may request montages on the same bone groups;
     deterministic priority resolution avoids nondeterministic visual artifacts.
   - **Verification:** Play two montages targeting the same bone group with different priorities.
     Assert the higher-priority montage wins. Play two with equal priority and assert the most
     recently started wins.
6. **R-9.4.16** -- The engine **SHALL** provide an `AnimationParams` ECS component written by
   gameplay and AI systems (speed, direction, grounded, crouching, jumping, falling, aim pitch, aim
   yaw, triggers) and read by the state machine for transition evaluation, decoupling input sources
   from animation.
   - **Rationale:** A single parameter component lets player input and AI logic drive the same state
     machine without duplicating state graphs.
   - **Verification:** Set speed from player input. Assert the locomotion state machine transitions.
     Set speed from AI logic on a second entity. Assert the same state machine transitions
     identically.
7. **R-9.4.17** -- The engine **SHALL** provide an `AnimationQuery` read-only ECS component exposing
   current state, elapsed time, remaining time, transitioning flag, active montage, and root motion
   delta for AI and gameplay inspection.
   - **Rationale:** AI and gameplay systems must observe animation state to sequence actions without
     polling the state machine internals.
   - **Verification:** Run a montage. Assert `AnimationQuery.active_montage` reports the montage
     name. Assert `remaining_time` matches the authored duration minus elapsed playback.
8. **R-9.4.18** -- The engine **SHALL** compile editor-authored state graphs to static Rust
   evaluation functions via the middleman codegen pipeline, producing inlined match-based dispatch
   with no runtime graph traversal.
   - **Rationale:** Static codegen eliminates graph interpretation overhead and enables the compiler
     to inline transition conditions.
   - **Verification:** Author a state graph in the editor. Build the project. Assert the generated
     Rust contains a match expression over state variants. Assert no runtime graph data structure is
     traversed during evaluation.

## Non-Functional Requirements

1. **R-9.4.NF1** -- The engine **SHALL** evaluate 1000 state graph instances within 1 ms CPU time.
   - **Rationale:** State machine evaluation must scale to MMO-scale character counts.
   - **Verification:** Evaluate 1000 graph instances. Assert total CPU time stays within 1 ms.

2. **R-9.4.NF2** -- The engine **SHALL** use under 1 KB of per-instance memory for state graph
   instances.
   - **Rationale:** Lightweight instances enable thousands of animated entities without memory
     pressure.
   - **Verification:** Measure per-instance memory. Assert it stays under 1 KB.

3. **R-9.4.NF3** -- The engine **SHALL** maintain foot sliding below 1 cm during sync-group
   transitions.
   - **Rationale:** Visible foot skating breaks locomotion quality during state transitions.
   - **Verification:** Transition between walk and run with sync markers. Measure foot position
     delta. Assert sliding stays below 1 cm.
