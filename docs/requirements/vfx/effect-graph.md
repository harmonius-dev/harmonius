# R-11.6 -- Effect Graph Requirements

## Authoring

1. **R-11.6.1** — The engine **SHALL** provide a visual node-graph editor composing spawn, update,
   and render modules as typed-edge-connected nodes, compiling to GPU compute dispatches, with
   real-time preview, scrubbing, looping, and performance statistics.
   - **Rationale:** A no-code visual editor lets designers author VFX without programming; GPU
     compilation ensures runtime performance matches preview.
   - **Verification:** Create a graph with spawn, update (gravity), and render (sprite) nodes.
     Compile and verify valid GPU dispatch. Open preview and verify particles render. Scrub timeline
     and verify state update.

2. **R-11.6.2** — The engine **SHALL** allow custom effect graph nodes authored via the Logic Graph
   system with typed inputs and outputs, executing per-particle or per-emitter, appearing as
   reusable library assets in the node palette.
   - **Rationale:** Custom nodes extend the VFX system with game-specific behavior without engine
     code changes.
   - **Verification:** Author a custom node with float input and vector output. Package as library
     asset. Verify it appears in the palette. Connect, compile, and run. Verify expected output
     values.

3. **R-11.6.3** — The engine **SHALL** expose typed parameter slots (float, vector, color, curve,
   gradient, texture) that accept per-instance overrides, bind to game state via reactive data
   binding, and animate through the sequencer.
   - **Rationale:** Externalized parameters let one template serve many contexts through instance
     tuning, data binding, and cinematic animation.
   - **Verification:** Create a graph with a color parameter defaulting to red. Spawn two instances,
     override one to blue. Verify distinct colors. Bind a float to game state, change it, and verify
     update within one frame. Animate via sequencer and verify interpolation.

## Event-Driven Spawning

4. **R-11.6.4** — The engine **SHALL** spawn VFX from ECS observer events (animation notifies,
   physics collisions, destruction events, custom Logic Graph events), parameterizing with position,
   normal, velocity, and surface material.
   - **Rationale:** Event-driven spawning couples VFX to gameplay automatically, ensuring immediate
     context-appropriate feedback.
   - **Verification:** Register an observer for physics collision. Trigger a collision and verify
     VFX at the correct position with surface normal and velocity. Trigger an animation notify and
     verify VFX at bone position.

## LOD and Performance Budget

5. **R-11.6.5** — The engine **SHALL** apply distance- and coverage-based LOD tiers reducing
   particle count and simulation fidelity, enforcing a global VFX budget that scales down
   lower-priority effects when total count, GPU time, or overdraw exceeds limits.
   - **Rationale:** Automatic LOD and budgeting maintain frame rate by gracefully degrading
     lower-priority effects.
   - **Verification:** Spawn at near and far distances and verify far uses fewer particles. Exceed
     the budget and verify lower-priority scaled down while higher-priority retained. Confirm GPU
     time stays within ceiling.
