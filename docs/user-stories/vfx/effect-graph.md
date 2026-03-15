# User Stories -- 11.6 Effect Graph

## US-11.6.1.1 Author VFX Systems in a Visual Node-Graph Editor

**As a** effects artist (P-12), **I want** a node-graph editor where I compose spawn, update, and
render modules as connected nodes that compile to GPU compute dispatches, **so that** I can create
particle effects entirely visually without writing shader code.

## US-11.6.1.2 Preview Effects With Scrubbing and Performance Stats

**As a** effects artist (P-12), **I want** real-time effect preview in the editor with timeline
scrubbing, looping, and per-emitter performance statistics (particle count, GPU time), **so that** I
can iterate on timing and cost without entering play mode.

## US-11.6.1.3 Validate Effect Graph Node Count Limits Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile caps compiled effect graphs at 32
nodes and desktop at 128, **so that** effect complexity respects per-platform compute budgets.

## US-11.6.2.1 Extend the Effect Graph With Custom Logic Nodes

**As a** technical artist (P-13), **I want** to author custom effect graph nodes using the logic
graph system with typed inputs and outputs, **so that** I can add game-specific behaviors (sample
health for particle color, custom attractor forces) as reusable library assets.

## US-11.6.2.2 Validate Custom Nodes Restricted to Per-Emitter on Mobile

**As an** engine tester (P-27), **I want** to verify that per-particle custom nodes are restricted
to per-emitter execution on mobile, **so that** custom node GPU compute overhead is bounded on
mobile platforms.

## US-11.6.3.1 Expose Effect Parameters for Per-Instance Overrides

**As a** effects artist (P-12), **I want** typed parameter slots (float, vector, color, curve,
gradient, texture) on effect graphs that designers override per-instance in the level editor, **so
that** a single fire effect template produces small campfires and large bonfires through parameter
variation.

## US-11.6.3.2 Bind Effect Parameters to Game State for Reactive VFX

**As a** game developer (P-15), **I want** effect parameters bound to game state via reactive data
binding, **so that** a health-based aura effect automatically changes color and intensity as the
character takes damage without manual event wiring.

## US-11.6.3.3 Animate Effect Parameters via the Cinematic Sequencer

**As a** effects artist (P-12), **I want** to keyframe effect parameters in the sequencer for
cinematic VFX choreography, **so that** cutscene spell effects build intensity, peak, and fade on
precisely timed curves.

## US-11.6.4.1 Spawn Sparks From Sword Slash Animation Events

**As a** effects artist (P-12), **I want** VFX spawned in response to animation notify events with
context data (position, normal, velocity), **so that** sword swings spawn sparks at the exact
contact frame without hardcoded timing.

## US-11.6.4.2 Spawn Dust on Physics Collision Events

**As a** effects artist (P-12), **I want** VFX spawned from physics collision events with surface
material context, **so that** a boulder hitting sand spawns sand dust while hitting stone spawns
stone chips automatically.

## US-11.6.4.3 Validate Event-Driven Spawn Throttling on Mobile

**As an** engine tester (P-27), **I want** to verify that mobile throttles low-priority event-
driven spawns (ambient dust, distant impacts) under budget pressure, **so that** event-driven VFX do
not exceed mobile particle budgets.

## US-11.6.5.1 Scale VFX Quality by Distance and Performance Budget

**As a** effects artist (P-12), **I want** automatic LOD tiers that reduce particle counts and
simulation fidelity for distant effects, with a global budget tracking GPU compute time and
overdraw, **so that** VFX quality degrades gracefully without frame drops.

## US-11.6.5.2 Prioritize Hero Ability Effects Over Ambient Particles

**As a** game designer (P-5), **I want** per-effect priority assignment (hero abilities > ambient
> distant environment) used by the global budget manager, **so that** player ability effects
remain visible even when the VFX budget is exhausted.

## US-11.6.5.3 Validate VFX Budget Integration With Dynamic Resolution

**As an** engine tester (P-27), **I want** to verify that the VFX performance budget integrates with
the dynamic resolution system to maintain frame rate, **so that** VFX-heavy scenes trigger
resolution reduction rather than frame drops.
