# User Stories -- 11.6 Effect Graph

## Stories

| ID          | Persona                 |
|-------------|-------------------------|
| US-11.6.1.1 | effects artist (P-12)   |
| US-11.6.1.2 | effects artist (P-12)   |
| US-11.6.1.3 | engine developer (P-26) |
| US-11.6.2.1 | technical artist (P-13) |
| US-11.6.2.2 | technical artist (P-13) |
| US-11.6.2.3 | engine developer (P-26) |
| US-11.6.3.1 | effects artist (P-12)   |
| US-11.6.3.2 | effects artist (P-12)   |
| US-11.6.3.3 | game designer (P-5)     |
| US-11.6.4.1 | effects artist (P-12)   |
| US-11.6.4.2 | effects artist (P-12)   |
| US-11.6.4.3 | engine developer (P-26) |
| US-11.6.5.1 | effects artist (P-12)   |
| US-11.6.5.2 | game designer (P-5)     |
| US-11.6.5.3 | engine developer (P-26) |

1. **US-11.6.1.1** — **As a** effects artist (P-12), **I want** a node-graph editor composing spawn,
   update, and render modules that compile to GPU compute dispatches, **so that** I can create
   particle effects visually without shader code.

2. **US-11.6.1.2** — **As a** effects artist (P-12), **I want** real-time preview with timeline
   scrubbing, looping, and per-emitter performance stats, **so that** I can iterate on timing and
   cost without entering play mode.

3. **US-11.6.1.3** — **As a** engine developer (P-26), **I want** compiled effect graphs capped at
   32 nodes on mobile and 128 on desktop, **so that** effect complexity respects per-platform
   compute budgets.

4. **US-11.6.2.1** — **As a** technical artist (P-13), **I want** custom effect graph nodes authored
   via the Logic Graph system with typed inputs and outputs, **so that** I can add game-specific
   behaviors as reusable library assets.

5. **US-11.6.2.2** — **As a** technical artist (P-13), **I want** custom nodes packaged as library
   assets appearing in the node palette alongside built-in nodes, **so that** the VFX toolkit is
   extensible without engine changes.

6. **US-11.6.2.3** — **As a** engine developer (P-26), **I want** per-particle custom nodes
   restricted to per-emitter execution on mobile, **so that** GPU compute overhead is bounded on
   mobile platforms.

7. **US-11.6.3.1** — **As a** effects artist (P-12), **I want** typed parameter slots (float,
   vector, color, curve, gradient, texture) overridable per-instance in the level editor,
   **so that** one fire template produces campfires and bonfires through parameter variation.

8. **US-11.6.3.2** — **As a** effects artist (P-12), **I want** to keyframe effect parameters in the
   sequencer, **so that** cutscene spell effects build, peak, and fade on precise timing curves.

9. **US-11.6.3.3** — **As a** game designer (P-5), **I want** effect parameters bound to game state
   via reactive data binding, **so that** a health aura automatically changes color as the character
   takes damage.

10. **US-11.6.4.1** — **As a** effects artist (P-12), **I want** VFX spawned from animation notify
    events with context data (position, normal, velocity), **so that** sword swings spawn sparks at
    the exact contact frame.

11. **US-11.6.4.2** — **As a** effects artist (P-12), **I want** VFX spawned from physics collision
    events with surface material context, **so that** boulders hitting sand spawn sand dust while
    stone spawns stone chips.

12. **US-11.6.4.3** — **As a** engine developer (P-26), **I want** mobile to throttle low-priority
    event-driven spawns under budget pressure, **so that** event-driven VFX do not exceed mobile
    particle budgets.

13. **US-11.6.5.1** — **As a** effects artist (P-12), **I want** automatic LOD tiers reducing
    particle count and simulation fidelity for distant effects with a global budget, **so that** VFX
    degrades gracefully without frame drops.

14. **US-11.6.5.2** — **As a** game designer (P-5), **I want** per-effect priority (hero abilities >
    ambient > distant environment) used by the budget manager, **so that** player abilities remain
    visible when budget is exhausted.

15. **US-11.6.5.3** — **As a** engine developer (P-26), **I want** the VFX budget to integrate with
    the dynamic resolution system, **so that** VFX-heavy scenes trigger resolution reduction rather
    than frame drops.
