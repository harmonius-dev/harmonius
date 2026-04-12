# Spatial Awareness User Stories

## Sense Definitions

| ID        | Persona             |
|-----------|---------------------|
| US-17.3.1 | game designer (P-5) |
| US-17.3.2 | game designer (P-5) |

1. **US-17.3.1** — **As a** game designer (P-5), **I want** data-driven SenseDefinitions with shape,
   range, falloff, and filter tags, **so that** I can configure sight, hearing, and smell without
   engine code changes.
2. **US-17.3.2** — **As a** game designer (P-5), **I want** 2D sense shapes (circle, cone,
   rectangle) for top-down stealth games, **so that** enemy detection cones work correctly in 2D
   without paying 3D volume cost.

## Awareness State Machine

| ID        | Persona               |
|-----------|-----------------------|
| US-17.3.3 | game designer (P-5)   |
| US-17.3.4 | game developer (P-15) |

1. **US-17.3.3** — **As a** game designer (P-5), **I want** a five-state awareness state machine
   (Unaware, Suspicious, Alert, Tracking, Lost) authored per entity, **so that** detection
   progression is engine-level instead of custom per-game.
2. **US-17.3.4** — **As a** game developer (P-15), **I want** AwarenessTransitionEvent emitted on
   every state change, **so that** UI, audio, and AI systems can react to detection events without
   polling.

## Spatial Queries

| ID        | Persona                 |
|-----------|-------------------------|
| US-17.3.5 | engine developer (P-26) |
| US-17.3.6 | gamer (P-23)            |

1. **US-17.3.5** — **As an** engine developer (P-26), **I want** 100 awareness queries against 1,000
   targets each to run within 2 ms per frame through the shared BVH spatial index, **so that**
   crowds of AI fit in the per-frame budget.
2. **US-17.3.6** — **As a** gamer (P-23), **I want** raycast, box select, and nearest-N picking to
   feel instant even in scenes with thousands of entities, **so that** selection during battles is
   never laggy.

## GPU Scale

| ID        | Persona               |
|-----------|-----------------------|
| US-17.3.7 | game developer (P-15) |

1. **US-17.3.7** — **As a** game developer (P-15), **I want** spatial awareness to scale to millions
   of entities via GPU compute, **so that** massive RTS battles and simulation games maintain frame
   rate.

## Visual Indicators

| ID        | Persona      |
|-----------|--------------|
| US-17.3.8 | gamer (P-23) |
| US-17.3.9 | QA lead (P-20)|

1. **US-17.3.8** — **As a** gamer (P-23), **I want** detection state icons, threat direction arrows,
   and stealth meters to appear on enemies automatically, **so that** I can tell at a glance whether
   I have been seen.
2. **US-17.3.9** — **As a** QA lead (P-20), **I want** vision cones and hearing spheres rendered as
   debug gizmos in the editor, filtered by render layer, **so that** I can verify AI perception
   shapes without cluttering the viewport.

## Latency

| ID         | Persona               |
|------------|-----------------------|
| US-17.3.10 | game developer (P-15) |

1. **US-17.3.10** — **As a** game developer (P-15), **I want** awareness state changes to report
   within one frame of stimulus observability, **so that** enemy reaction feels responsive instead
   of delayed.
