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
