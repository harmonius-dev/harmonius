# Spatial Awareness

What agents perceive: sight, hearing, smell, and custom senses.

## What it covers

- Data-driven sense definitions: shape (circle, cone, rectangle), range, falloff, and tag
  filters.
- Five-state awareness FSM: Unaware, Suspicious, Alert, Tracking, Lost.
- State transitions emit ECS events triggering audio, UI, and visual indicators.
- Unified query API: 100 observers × 1,000 targets in <2 ms via shared spatial index.
- GPU compute path for 1M+ entities with one-frame latency.
- Raycast and box picks for precise targeting.
- Editor gizmos showing sense shapes and state transitions.
- One-frame latency from stimulus to state change (except GPU readback).

## Concepts

### Sense Definitions

A sense is defined by shape (2D circle/cone/rectangle or 3D sphere/cone), range, falloff curve, and
tag filters. All senses evaluate against the shared spatial index; no sense is hardcoded. Custom
senses (echolocation, tremor-sense) are authored as data.

### Awareness State Machine

Agents transition through five awareness states based on sense results and elapsed time. Unaware →
Suspicious (weak stimulus), Suspicious → Alert (strong stimulus), Alert → Tracking (target
acquired), Tracking → Lost (target left sense range, timeout). Each transition emits an ECS event so
the engine can spawn warnings, play audio, or update tactics.

### Performance

Spatial-index queries, grid-based propagation, and raycasts together keep perception <2 ms for 100
observers querying 1,000 targets. GPU compute scales to 1M entities if CPU budget is exceeded.

## How it fits

- See [grids-and-volumes](./grids-and-volumes.md) for grid-based sense data.
- See [timelines](./timelines.md) for awareness-driven cutscenes and cinematics.
- Integrates with [ai](../ai/perception.md) for agent decision-making based on awareness.
- Integrates with [game-framework](../game-framework/gameplay-features.md) for perception-based
  stealth mechanics.
