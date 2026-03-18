# User Stories -- 11.6 Effect Graph

## Authoring

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-11.6.1.1 | Effects artist (P-12)   | F-11.6.1 | R-11.6.1     |
| US-11.6.1.2 | Effects artist (P-12)   | F-11.6.1 | R-11.6.1     |
| US-11.6.1.3 | Engine tester (P-27)    | F-11.6.1 | R-11.6.1     |
| US-11.6.2.1 | Technical artist (P-13) | F-11.6.2 | R-11.6.2     |
| US-11.6.2.2 | Engine tester (P-27)    | F-11.6.2 | R-11.6.2     |

1. **US-11.6.1.1** — I want a node-graph editor where I compose spawn, update, and render modules as
   connected nodes that compile to GPU compute dispatches, so that I can create particle effects
   entirely visually without writing shader code.
   - **Acceptance:** Valid GPU compute dispatch produced from compiled graph; particles render in
     editor viewport
2. **US-11.6.1.2** — I want real-time effect preview in the editor with timeline scrubbing, looping,
   and per-emitter performance statistics (particle count, GPU time), so that I can iterate on
   timing and cost without entering play mode.
   - **Acceptance:** Preview renders in real time; scrubbing updates particle state to scrubbed
     time; performance stats visible per emitter
3. **US-11.6.1.3** — I want to verify that mobile caps compiled effect graphs at 32 nodes and
   desktop at 128, so that effect complexity respects per-platform compute budgets.
   - **Acceptance:** Mobile caps at 32 nodes; desktop caps at 128 nodes; compilation rejects graphs
     exceeding limits
4. **US-11.6.2.1** — I want to author custom effect graph nodes using the logic graph system with
   typed inputs and outputs, so that I can add game-specific behaviors (sample health for particle
   color, custom attractor forces) as reusable library assets.
   - **Acceptance:** Custom nodes appear in node palette; execute per-particle or per-emitter;
     produce expected output values
5. **US-11.6.2.2** — I want to verify that per-particle custom nodes are restricted to per-emitter
   execution on mobile, so that custom node GPU compute overhead is bounded on mobile platforms.
   - **Acceptance:** Per-particle custom nodes restricted to per-emitter on mobile; desktop allows
     per-particle execution

## Parameters and Data Binding

| ID          | Persona               | Features | Requirements |
|-------------|-----------------------|----------|--------------|
| US-11.6.3.1 | Effects artist (P-12) | F-11.6.3 | R-11.6.3     |
| US-11.6.3.2 | Developer (P-15)      | F-11.6.3 | R-11.6.3     |
| US-11.6.3.3 | Effects artist (P-12) | F-11.6.3 | R-11.6.3     |

1. **US-11.6.3.1** — I want typed parameter slots (float, vector, color, curve, gradient, texture)
   on effect graphs that designers override per-instance in the level editor, so that a single fire
   effect template produces small campfires and large bonfires through parameter variation.
   - **Acceptance:** Per-instance overrides produce distinct visual results; parameter defaults
     apply when not overridden
2. **US-11.6.3.2** — I want effect parameters bound to game state via reactive data binding, so that
   a health-based aura effect automatically changes color and intensity as the character takes
   damage without manual event wiring.
   - **Acceptance:** Bound parameters update within one frame of game state change; no manual event
     wiring required
3. **US-11.6.3.3** — I want to keyframe effect parameters in the sequencer for cinematic VFX
   choreography, so that cutscene spell effects build intensity, peak, and fade on precisely timed
   curves.
   - **Acceptance:** Parameters interpolate across keyframes in sequencer; timing matches authored
     curves

## Event-Driven Spawning

| ID          | Persona               | Features | Requirements |
|-------------|-----------------------|----------|--------------|
| US-11.6.4.1 | Effects artist (P-12) | F-11.6.4 | R-11.6.4     |
| US-11.6.4.2 | Effects artist (P-12) | F-11.6.4 | R-11.6.4     |
| US-11.6.4.3 | Engine tester (P-27)  | F-11.6.4 | R-11.6.4     |

1. **US-11.6.4.1** — I want VFX spawned in response to animation notify events with context data
   (position, normal, velocity), so that sword swings spawn sparks at the exact contact frame
   without hardcoded timing.
   - **Acceptance:** VFX spawns at correct bone position on animation notify; context data
     (position, normal, velocity) passed to effect
2. **US-11.6.4.2** — I want VFX spawned from physics collision events with surface material context,
   so that a boulder hitting sand spawns sand dust while hitting stone spawns stone chips
   automatically.
   - **Acceptance:** VFX spawns at collision position with correct surface normal; material context
     selects appropriate effect
3. **US-11.6.4.3** — I want to verify that mobile throttles low-priority event-driven spawns
   (ambient dust, distant impacts) under budget pressure, so that event-driven VFX do not exceed
   mobile particle budgets.
   - **Acceptance:** Low-priority spawns throttled on mobile under budget pressure; high-priority
     spawns retained

## LOD and Performance Budget

| ID          | Persona               | Features | Requirements |
|-------------|-----------------------|----------|--------------|
| US-11.6.5.1 | Effects artist (P-12) | F-11.6.5 | R-11.6.5     |
| US-11.6.5.2 | Designer (P-5)        | F-11.6.5 | R-11.6.5     |
| US-11.6.5.3 | Engine tester (P-27)  | F-11.6.5 | R-11.6.5     |

1. **US-11.6.5.1** — I want automatic LOD tiers that reduce particle counts and simulation fidelity
   for distant effects, with a global budget tracking GPU compute time and overdraw, so that VFX
   quality degrades gracefully without frame drops.
   - **Acceptance:** Far instances use fewer particles than near; budget enforced without frame
     drops
2. **US-11.6.5.2** — I want per-effect priority assignment (hero abilities > ambient > distant
   environment) used by the global budget manager, so that player ability effects remain visible
   even when the VFX budget is exhausted.
   - **Acceptance:** Higher-priority effects retain full fidelity; lower-priority effects scaled
     down first
3. **US-11.6.5.3** — I want to verify that the VFX performance budget integrates with the dynamic
   resolution system to maintain frame rate, so that VFX-heavy scenes trigger resolution reduction
   rather than frame drops.
   - **Acceptance:** VFX budget integrates with dynamic resolution; frame rate maintained in
     VFX-heavy scenes
