# 17.3 — Spatial Awareness

## Sense Definitions

| ID       | Feature                       |
|----------|-------------------------------|
| F-17.3.1 | Data-Driven SenseDefinitions |
| F-17.3.2 | 2D Sense Shapes              |

1. **F-17.3.1** — `SenseDefinition` is a named spatial query with shape, range, falloff curve, and
   filter tag set, authored in the editor. Sight, hearing, smell, and custom senses share this one
   type instead of per-sense hardcoded systems.
   - **Deps:** F-1.9.1 (Shared Spatial Index), F-13.1.2 (Gameplay Tags)
2. **F-17.3.2** — 2D sense shapes (circle, cone, rectangle) enable spatial awareness in 2D and 2.5D
   games with Transform2D. Top-down stealth, 2D RTS, and platformers use the same primitive.
   - **Deps:** F-17.3.1

## Awareness State Machine

| ID       | Feature                             |
|----------|-------------------------------------|
| F-17.3.3 | Five-State Awareness State Machine |
| F-17.3.4 | Awareness Transition Events        |

1. **F-17.3.3** — Per-entity awareness state transitions between Unaware, Suspicious, Alert,
   Tracking, and Lost based on sense query results and elapsed time. Detection progression is
   data-driven and engine-level.
   - **Deps:** F-17.3.1, F-1.1.1
2. **F-17.3.4** — State changes emit `AwarenessTransitionEvent` through the ECS event channel. UI,
   audio, and AI systems react without polling.
   - **Deps:** F-17.3.3, F-1.5.1

## Spatial Queries

| ID       | Feature                       |
|----------|-------------------------------|
| F-17.3.5 | High-Throughput Awareness    |
| F-17.3.6 | Shared Selection Queries     |

1. **F-17.3.5** — 100 awareness queries against 1,000 targets each run within 2 ms per frame through
   the shared BVH spatial index. Crowds of AI entities fit in the per-frame budget.
   - **Deps:** F-17.3.1, F-1.9.1
2. **F-17.3.6** — Selection queries (raycast, box select, nearest-N) route through the same spatial
   index, with 50 picks per batch completing in under 0.5 ms. Player picking and AI awareness share
   infrastructure.
   - **Deps:** F-1.9.1

## GPU Scale

| ID       | Feature                       |
|----------|-------------------------------|
| F-17.3.7 | GPU Ultra-Scale Awareness    |

1. **F-17.3.7** — GPU compute awareness evaluates ultra-scale scenarios (1M+ entities) with
   one-frame latency readback. Hybrid CPU/GPU thresholding picks the backend by entity count.
   - **Deps:** F-17.3.1, F-2.2, F-1.9.1

## Visual Indicators

| ID       | Feature                             |
|----------|-------------------------------------|
| F-17.3.8 | Awareness-Driven Indicator Spawning|
| F-17.3.9 | Sense Volume Debug Gizmos          |

1. **F-17.3.8** — 3D visual indicators (detection state icons, threat arrows, stealth meters) spawn
   and despawn in response to `AwarenessTransitionEvent`. Indicator lifecycle is declarative and
   engine-managed.
   - **Deps:** F-17.3.4, F-13.11 (Indicators)
2. **F-17.3.9** — Vision cones, hearing spheres, and awareness state icons render as debug gizmos in
   the editor viewport, filtered by render layer bitmask.
   - **Deps:** F-17.3.1, F-15 (Editor)

## Latency

| ID        | Feature                       |
|-----------|-------------------------------|
| F-17.3.10 | One-Frame Awareness Latency  |

1. **F-17.3.10** — Awareness state changes report within one frame of a triggering stimulus becoming
   observable in the shared spatial index, except for GPU-evaluated ultra-scale paths which use
   one-frame readback latency.
   - **Deps:** F-17.3.1, F-17.3.3
