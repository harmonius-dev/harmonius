# User Stories Coverage Matrix

Maps every `US-X.Y.Z` to its persona, requirement, design, and test artifacts. Coverage is
partial; see [requirements-matrix.md](requirements-matrix.md) for the matched list of
domains.

## Status legend

| Status   | Meaning                                                                  |
|----------|--------------------------------------------------------------------------|
| Owned    | Persona, requirement, design, and test all linked                        |
| Partial  | One or more artifacts missing                                            |
| Orphan   | Story exists but no requirement / design references it                   |
| Reversed | Story retired; preserved for trace                                       |

## Core Runtime (`US-1.*`)

| Story file                                                                    | Stories | Persona refs   | Requirement                                         | Design                                  | Status |
|-------------------------------------------------------------------------------|--------:|----------------|------------------------------------------------------|-----------------------------------------|--------|
| [entity-component-system.md](../user-stories/core-runtime/entity-component-system.md) | 45 | P-15, P-26, P-27 | requirements/core-runtime/entity-component-system.md | design/core-runtime/ecs.md            | Owned  |
| [serialization.md](../user-stories/core-runtime/serialization.md)             |   22    | P-15, P-26     | requirements/core-runtime/serialization.md          | design/core-runtime/reflection-serialization.md | Owned |
| [async-io.md](../user-stories/core-runtime/async-io.md)                       |   20    | P-15, P-26     | requirements/core-runtime/async-io.md               | design/core-runtime/io.md, memory-async-io.md | Owned |
| [memory-management.md](../user-stories/core-runtime/memory-management.md)     |   18    | P-15, P-26     | requirements/core-runtime/memory-management.md      | design/core-runtime/memory-async-io.md  | Owned |
| [reflection-and-type-system.md](../user-stories/core-runtime/reflection-and-type-system.md) | 18 | P-15, P-26  | requirements/core-runtime/reflection-and-type-system.md | design/core-runtime/reflection-serialization.md | Owned |
| [events-and-messaging.md](../user-stories/core-runtime/events-and-messaging.md) | 15    | P-15, P-26     | requirements/core-runtime/events-and-messaging.md   | design/core-runtime/events-plugins.md   | Owned |
| [spatial-indexing.md](../user-stories/core-runtime/spatial-indexing.md)       |   15    | P-15, P-26     | requirements/core-runtime/spatial-indexing.md       | design/core-runtime/spatial-index.md    | Owned |
| [plugin-system.md](../user-stories/core-runtime/plugin-system.md)             |   13    | P-15, P-26     | requirements/core-runtime/plugin-system.md          | design/core-runtime/events-plugins.md   | Owned |
| [scene-and-transforms.md](../user-stories/core-runtime/scene-and-transforms.md) | 12    | P-15, P-26     | requirements/core-runtime/scene-and-transforms.md   | design/core-runtime/scene-transforms.md | Owned |
| [game-loop.md](../user-stories/core-runtime/game-loop.md)                     |   10    | P-15, P-26     | requirements/core-runtime/game-loop.md              | design/core-runtime/game-loop.md        | Owned |
| [algorithms.md](../user-stories/core-runtime/algorithms.md)                   |   10    | P-15, P-26     | requirements/core-runtime/algorithms.md             | design/core-runtime/algorithms.md, primitives.md | Owned |

## Data Systems (`US-16.*`)

| Story file                                                                  | Stories | Requirement                                          | Design                                    | Status |
|-----------------------------------------------------------------------------|--------:|------------------------------------------------------|-------------------------------------------|--------|
| [directed-graphs.md](../user-stories/data-systems/directed-graphs.md)       |   14    | requirements/data-systems/directed-graphs.md        | design/data-systems/directed-graphs.md     | Owned  |
| [data-tables.md](../user-stories/data-systems/data-tables.md)               |   12    | requirements/data-systems/data-tables.md            | design/data-systems/data-tables.md         | Owned  |
| [attributes-effects.md](../user-stories/data-systems/attributes-effects.md) |    9    | requirements/data-systems/attributes-effects.md     | design/data-systems/attributes-effects.md  | Partial — BL-0029 |
| [containers-slots.md](../user-stories/data-systems/containers-slots.md)     |   10    | requirements/data-systems/containers-slots.md       | design/data-systems/containers-slots.md    | Partial — BL-0029, BL-0045 |

## Simulation (`US-17.*`)

| Story file                                                              | Stories | Requirement                                          | Design                              | Status |
|-------------------------------------------------------------------------|--------:|------------------------------------------------------|-------------------------------------|--------|
| [grids-volumes.md](../user-stories/simulation/grids-volumes.md)         |   13    | requirements/simulation/grids-volumes.md            | design/simulation/grids-volumes.md   | Owned  |
| [spatial-awareness.md](../user-stories/simulation/spatial-awareness.md) |   13    | requirements/simulation/spatial-awareness.md        | design/simulation/spatial-awareness.md | Owned |
| [timelines.md](../user-stories/simulation/timelines.md)                 |    8    | requirements/simulation/timelines.md                | design/simulation/timelines.md       | Partial — BL-0045 |
| [event-logs.md](../user-stories/simulation/event-logs.md)               |    8    | requirements/simulation/event-logs.md               | design/simulation/event-logs.md      | Partial — BL-0045 |

## Networking (`US-8.*`)

| Story file                                                                   | Stories | Status                                       |
|------------------------------------------------------------------------------|--------:|----------------------------------------------|
| [communication.md](../user-stories/networking/communication.md)              |   29    | Owned                                         |
| [non-functional.md](../user-stories/networking/non-functional.md) (replaced) |   24    | Owned (deslop pass; see BL-0044 for matching reqs) |
| [session-management.md](../user-stories/networking/session-management.md)    |   18    | Owned                                         |
| [transport-layer.md](../user-stories/networking/transport-layer.md)          |   16    | Owned                                         |
| [mmo-infrastructure.md](../user-stories/networking/mmo-infrastructure.md)    |   15    | Owned                                         |
| [anti-cheat.md](../user-stories/networking/anti-cheat.md)                    |   12    | Partial — BL-0038                            |
| [prediction-and-rollback.md](../user-stories/networking/prediction-and-rollback.md) | 12 | Owned                                       |
| [replay-system.md](../user-stories/networking/replay-system.md)              |   12    | Owned                                         |
| [state-replication.md](../user-stories/networking/state-replication.md)      |   11    | Partial — BL-0023                            |
| [remote-procedure-calls.md](../user-stories/networking/remote-procedure-calls.md) | 10 | Owned                                       |

## AI (`US-7.*`)

| Story file                                                          | Stories | Status                                    |
|---------------------------------------------------------------------|--------:|-------------------------------------------|
| [behavior-trees.md](../user-stories/ai/behavior-trees.md)           |   34    | Owned                                     |
| [non-functional.md](../user-stories/ai/non-functional.md) (new)     |   30    | Owned (deslop pass; replaces misplaced)   |
| [navigation.md](../user-stories/ai/navigation.md)                   |   30    | Partial — BL-0027                         |
| [perception.md](../user-stories/ai/perception.md)                   |   26    | Owned                                     |
| [crowd-simulation.md](../user-stories/ai/crowd-simulation.md)       |   24    | Owned                                     |
| [goap.md](../user-stories/ai/goap.md)                               |   23    | Owned                                     |
| [steering-avoidance.md](../user-stories/ai/steering-avoidance.md)   |   22    | Partial — BL-0041                         |
| [tactical-combat.md](../user-stories/ai/tactical-combat.md)         |   22    | Owned                                     |
| [utility-ai.md](../user-stories/ai/utility-ai.md)                   |   19    | Owned                                     |

## Other domains (Summary only)

Same approach as the requirements matrix: 13 domains carry "Summary only" rows. Detailed
mapping deferred to 2026-Q3 OKR-3.

| Domain               | Story files | Notes                                                  |
|----------------------|------------:|--------------------------------------------------------|
| Rendering            |     13      |                                                        |
| Geometry             |      6      |                                                        |
| Physics              |      8      |                                                        |
| Audio                |      5      | Decompose tracked via BL-0050                          |
| Input                |      5      | Decompose tracked via BL-0051                          |
| Animation            |      7      |                                                        |
| UI & 2D              |      6      | Decompose tracked via BL-0052                          |
| VFX                  |      6      |                                                        |
| Content Pipeline     |      7      |                                                        |
| Game Framework       |     27      | Genre stories per ADR-0011 composition model           |
| Platform             |      7      | Includes new telemetry, crash-reporting                |
| Tools & Editor       |     24      |                                                        |

## Coverage roll-up

| Domain          | Owned | Partial | Orphan | Reversed | Total |
|-----------------|------:|--------:|-------:|---------:|------:|
| Core Runtime    |    11 |       0 |      0 |        0 |    11 |
| Data Systems    |     2 |       2 |      0 |        0 |     4 |
| Simulation      |     2 |       2 |      0 |        0 |     4 |
| Networking      |     8 |       2 |      0 |        0 |    10 |
| AI              |     7 |       2 |      0 |        0 |     9 |
| **Total (indexed)** | **30** | **8** | **0** | **0** |    **38** |

Indexed coverage: 30 / 38 = **79%** Owned across the four matrixed domains. The 8 Partial
rows have backlog issues attached.
