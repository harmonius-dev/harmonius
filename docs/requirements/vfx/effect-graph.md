# R-11.6 — Effect Graph Requirements

## R-11.6.1 Node-Based Effect Graph Editor

The engine **SHALL** provide a visual node-graph editor that composes spawn, update, and render
modules as typed-edge-connected nodes, compiles the graph to GPU compute dispatches, and
displays real-time preview with scrubbing, looping, and performance statistics.

- **Derived from:** [F-11.6.1](../../features/vfx/effect-graph.md)
- **Rationale:** A no-code visual editor lets designers author VFX systems without programming,
  while GPU compilation ensures runtime performance matches preview.
- **Verification:** Integration test — create an effect graph with a spawn node (burst emitter),
  an update node (gravity force), and a render node (sprite); compile the graph; assert a valid
  GPU compute dispatch is produced; open the preview and assert particles render in the
  viewport; scrub the timeline and assert particle state updates to the scrubbed time.

## R-11.6.2 Custom Effect Graph Nodes

The engine **SHALL** allow custom effect graph nodes authored via the logic graph system, with
typed inputs and outputs, executing per-particle or per-emitter, and appearing in the node
palette as reusable library assets alongside built-in nodes.

- **Derived from:** [F-11.6.2](../../features/vfx/effect-graph.md)
- **Rationale:** Custom nodes let designers extend the VFX system with game-specific behavior
  (attractors, game-state sampling) without engine code changes.
- **Verification:** Integration test — author a custom node with a float input and vector
  output via the logic graph system; package it as a library asset; open the effect graph
  editor and assert the custom node appears in the node palette; connect it to a particle
  update chain; compile and run the effect; assert the custom node executes per-particle and
  produces the expected output values.

## R-11.6.3 Effect Graph Parameter System

The engine **SHALL** expose typed parameter slots (float, vector, color, curve, gradient,
texture) on effect graphs that accept per-instance overrides, bind to game state via reactive
data binding, and animate through the sequencer.

- **Derived from:** [F-11.6.3](../../features/vfx/effect-graph.md)
- **Rationale:** Externalized parameters enable a single effect template to serve many gameplay
  contexts through instance-level tuning, data binding, and cinematic animation.
- **Verification:** Integration test — create an effect graph with a color parameter defaulting
  to red; spawn two instances, overriding one to blue; assert each instance renders with its
  respective color; bind a float parameter to a game-state value; change the value and assert
  the effect updates within one frame; animate a parameter via the sequencer and assert it
  interpolates across keyframes.

## R-11.6.4 Event-Driven Effect Spawning

The engine **SHALL** spawn VFX in response to ECS observer events (animation notifies, physics
collisions, destruction events, and custom logic graph events), parameterizing the spawned
effect with event context data including position, normal, velocity, and surface material.

- **Derived from:** [F-11.6.4](../../features/vfx/effect-graph.md)
- **Rationale:** Event-driven spawning couples VFX to gameplay actions automatically, ensuring
  visual feedback is immediate and context-appropriate without manual scripting.
- **Verification:** Integration test — register an observer for a physics collision event; trigger
  a collision; assert a VFX instance spawns at the collision position with the correct surface
  normal and contact velocity; trigger an animation notify event; assert the corresponding VFX
  spawns at the notified bone position; verify context data (position, normal, velocity,
  material) is passed to the effect parameters.

## R-11.6.5 VFX LOD and Performance Budget

The engine **SHALL** apply distance-based and screen-coverage-based LOD tiers that reduce
particle count and simulation fidelity for distant effects, and enforce a global VFX performance
budget that scales down lower-priority effects when total particle count, GPU compute time, or
overdraw exceeds configured limits.

- **Derived from:** [F-11.6.5](../../features/vfx/effect-graph.md)
- **Rationale:** Automatic LOD and budgeting maintain frame rate targets during particle-heavy
  scenes by gracefully degrading lower-priority effects before impacting hero visuals.
- **Verification:** Integration test — spawn an effect at near and far distances; assert the far
  instance uses fewer particles than the near instance; spawn effects until total particle count
  exceeds the global budget; assert lower-priority effects are scaled down while higher-priority
  effects retain full fidelity; verify GPU compute time stays within the configured budget
  ceiling.
