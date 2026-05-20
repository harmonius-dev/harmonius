# Requirements Coverage Matrix

Maps every `R-X.Y.Z` to its feature, user-story, design, and test artifacts. Coverage status is
per-row.

The matrix is partial: Core Runtime, Data Systems, Simulation, and Networking are matrixed in full
per [docs/coverage/index.md](index.md). Other domains carry "Summary only" entries and are tracked
by [BL-0044](https://github.com/cjhowe-us/harmonius/issues?q=BL-0044) and
[BL-0048](https://github.com/cjhowe-us/harmonius/issues?q=BL-0048).

## Status legend

| Status   | Meaning                                                |
|----------|--------------------------------------------------------|
| Owned    | All four artifacts present and linked                  |
| Partial  | One or more artifacts missing                          |
| Orphan   | Requirement exists but no design / test references it  |
| Reversed | Requirement was retired; preserved for trace           |

## Core Runtime (`R-1.*`)

| File                                                                        | R-IDs | Feature                | Stories                | Design                  | Tests | Status |
|-----------------------------------------------------------------------------|-------|------------------------|------------------------|-------------------------|-------|--------|
| [entity-component-system.md](../requirements/core-runtime/entity-component-system.md) | R-1.1.1..39 | features/core-runtime/entity-component-system.md | user-stories/core-runtime/entity-component-system.md | design/core-runtime/ecs.md | ecs-test-cases.md | Owned |
| [scene-and-transforms.md](../requirements/core-runtime/scene-and-transforms.md)       | R-1.2.1..8  | features/core-runtime/scene-and-transforms.md    | user-stories/core-runtime/scene-and-transforms.md    | design/core-runtime/scene-transforms.md | scene-transforms-test-cases.md | Owned |
| [reflection-and-type-system.md](../requirements/core-runtime/reflection-and-type-system.md) | R-1.3.1..11 | features/core-runtime/reflection-and-type-system.md | user-stories/core-runtime/reflection-and-type-system.md | design/core-runtime/reflection-serialization.md | reflection-serialization-test-cases.md | Owned |
| [serialization.md](../requirements/core-runtime/serialization.md)           | R-1.4.1..12 | features/core-runtime/serialization.md           | user-stories/core-runtime/serialization.md           | design/core-runtime/reflection-serialization.md | reflection-serialization-test-cases.md | Owned |
| [events-and-messaging.md](../requirements/core-runtime/events-and-messaging.md)       | R-1.5.1..9  | features/core-runtime/events-and-messaging.md    | user-stories/core-runtime/events-and-messaging.md    | design/core-runtime/events-plugins.md   | events-plugins-test-cases.md | Owned |
| [plugin-system.md](../requirements/core-runtime/plugin-system.md)           | R-1.6.1..8  | features/core-runtime/plugin-system.md           | user-stories/core-runtime/plugin-system.md           | design/core-runtime/events-plugins.md   | events-plugins-test-cases.md | Owned |
| [memory-management.md](../requirements/core-runtime/memory-management.md)   | R-1.7.1..10 | features/core-runtime/memory-management.md       | user-stories/core-runtime/memory-management.md       | design/core-runtime/memory-async-io.md  | memory-async-io-test-cases.md | Owned |
| [async-io.md](../requirements/core-runtime/async-io.md)                     | R-1.8.1..14 | features/core-runtime/async-io.md                | user-stories/core-runtime/async-io.md                | design/core-runtime/io.md plus memory-async-io.md | io-test-cases.md plus memory-async-io-test-cases.md | Owned |
| [spatial-indexing.md](../requirements/core-runtime/spatial-indexing.md)     | R-1.9.1..11 | features/core-runtime/spatial-indexing.md        | user-stories/core-runtime/spatial-indexing.md        | design/core-runtime/spatial-index.md    | spatial-index-test-cases.md | Owned |
| [algorithms.md](../requirements/core-runtime/algorithms.md)                 | R-1.10.1..11| features/core-runtime/algorithms.md              | user-stories/core-runtime/algorithms.md              | design/core-runtime/algorithms.md plus primitives.md | algorithms-test-cases.md plus primitives-test-cases.md | Owned |
| [game-loop.md](../requirements/core-runtime/game-loop.md)                   | R-1.11.1..10| features/core-runtime/game-loop.md               | user-stories/core-runtime/game-loop.md               | design/core-runtime/game-loop.md        | game-loop-test-cases.md | Owned |

Foundation primitives (`error.md`, `io.md`, `ids.md`, `hot-reload-protocol.md`, `graph-runtime.md`,
`change-detection.md`, `console-variables.md`, `primitives.md`) currently live as design docs only;
their R-IDs feed off the existing requirement files (`async-io.md`, `algorithms.md`, etc.). A future
audit will split these into dedicated requirement files if the requirement count justifies it.

## Data Systems (`R-16.*`)

| File                                                                          | R-IDs | Stories                | Design                            | Tests                            | Status |
|-------------------------------------------------------------------------------|-------|------------------------|-----------------------------------|----------------------------------|--------|
| [directed-graphs.md](../requirements/data-systems/directed-graphs.md)         | R-16.1.* | user-stories/data-systems/directed-graphs.md | design/data-systems/directed-graphs.md     | directed-graphs-test-cases.md   | Owned  |
| [data-tables.md](../requirements/data-systems/data-tables.md)                 | R-16.2.* | user-stories/data-systems/data-tables.md     | design/data-systems/data-tables.md         | data-tables-test-cases.md       | Owned  |
| [attributes-effects.md](../requirements/data-systems/attributes-effects.md)   | R-16.3.* | user-stories/data-systems/attributes-effects.md | design/data-systems/attributes-effects.md | attributes-effects-test-cases.md| Owned  |
| [containers-slots.md](../requirements/data-systems/containers-slots.md)       | R-16.4.* | user-stories/data-systems/containers-slots.md   | design/data-systems/containers-slots.md   | containers-slots-test-cases.md  | Owned  |

The `composition.md` design has no dedicated requirement file; it documents authoring patterns over
the four primitives. Tracked under
[BL-0028](https://github.com/cjhowe-us/harmonius/issues?q=BL-0028) and
[BL-0029](https://github.com/cjhowe-us/harmonius/issues?q=BL-0029).

## Simulation (`R-17.*`)

| File                                                                | R-IDs | Stories                | Design                             | Tests                          | Status |
|---------------------------------------------------------------------|-------|------------------------|------------------------------------|--------------------------------|--------|
| [grids-volumes.md](../requirements/simulation/grids-volumes.md)     | R-17.1.* | user-stories/simulation/grids-volumes.md     | design/simulation/grids-volumes.md         | grids-volumes-test-cases.md   | Owned  |
| [spatial-awareness.md](../requirements/simulation/spatial-awareness.md) | R-17.2.* | user-stories/simulation/spatial-awareness.md | design/simulation/spatial-awareness.md     | spatial-awareness-test-cases.md| Owned  |
| [timelines.md](../requirements/simulation/timelines.md)             | R-17.3.* | user-stories/simulation/timelines.md         | design/simulation/timelines.md             | timelines-test-cases.md       | Owned  |
| [event-logs.md](../requirements/simulation/event-logs.md)           | R-17.4.* | user-stories/simulation/event-logs.md         | design/simulation/event-logs.md            | event-logs-test-cases.md      | Owned  |

## Networking (`R-8.*`)

| File                                                                              | R-IDs | Stories                | Design                                     | Tests                                  | Status  |
|-----------------------------------------------------------------------------------|-------|------------------------|--------------------------------------------|----------------------------------------|---------|
| [transport-layer.md](../requirements/networking/transport-layer.md)               | R-8.1.* | user-stories/networking/transport-layer.md | design/networking/network-transport.md     | network-transport-test-cases.md       | Owned   |
| [state-replication.md](../requirements/networking/state-replication.md)           | R-8.2.* | user-stories/networking/state-replication.md | design/networking/network-services.md      | network-services-test-cases.md        | Partial — see BL-0023 |
| [prediction-and-rollback.md](../requirements/networking/prediction-and-rollback.md) | R-8.3.* | user-stories/networking/prediction-and-rollback.md | design/networking/network-services.md   | network-services-test-cases.md        | Partial |
| [remote-procedure-calls.md](../requirements/networking/remote-procedure-calls.md) | R-8.4.* | user-stories/networking/remote-procedure-calls.md | design/networking/network-services.md  | network-services-test-cases.md        | Owned   |
| [session-management.md](../requirements/networking/session-management.md)         | R-8.5.* | user-stories/networking/session-management.md | design/networking/network-infrastructure.md | network-infrastructure-test-cases.md | Owned   |
| [replay-system.md](../requirements/networking/replay-system.md)                   | R-8.6.* | user-stories/networking/replay-system.md      | design/networking/network-services.md       | network-services-test-cases.md        | Owned   |
| [communication.md](../requirements/networking/communication.md)                   | R-8.6a.*| user-stories/networking/communication.md      | design/networking/network-services.md       | network-services-test-cases.md        | Owned   |
| [mmo-infrastructure.md](../requirements/networking/mmo-infrastructure.md)         | R-8.7.* | user-stories/networking/mmo-infrastructure.md | design/networking/network-infrastructure.md | network-infrastructure-test-cases.md | Owned   |
| [anti-cheat.md](../requirements/networking/anti-cheat.md)                         | R-8.8.* | user-stories/networking/anti-cheat.md         | design/networking/network-infrastructure.md | network-infrastructure-test-cases.md | Partial — see BL-0038 |
| `non-functional.md` (planned, see BL-0044)                                          | R-8.NFR.* | user-stories/networking/non-functional.md    | (multi-doc)                                  | (multi-doc)                            | Partial |

Networking-NFR requirements file is tracked as a follow-up under
[BL-0044](https://github.com/cjhowe-us/harmonius/issues?q=BL-0044) — currently the user-story file
exists (deslop pass) but a matching `requirements/networking/non-functional.md` has not yet been
authored.

## AI (`R-7.*`)

| File                                                                  | Status                          |
|-----------------------------------------------------------------------|---------------------------------|
| [behavior-trees.md](../requirements/ai/behavior-trees.md)             | Owned                           |
| [utility-ai.md](../requirements/ai/utility-ai.md)                     | Owned                           |
| [goap.md](../requirements/ai/goap.md)                                 | Owned                           |
| [navigation.md](../requirements/ai/navigation.md)                     | Partial — see BL-0027           |
| [steering-avoidance.md](../requirements/ai/steering-avoidance.md)     | Partial — see BL-0041           |
| [crowd-simulation.md](../requirements/ai/crowd-simulation.md)         | Owned                           |
| [tactical-combat.md](../requirements/ai/tactical-combat.md)           | Owned                           |
| [perception.md](../requirements/ai/perception.md)                     | Owned                           |
| [non-functional.md](../requirements/ai/non-functional.md) (new)       | Owned (deslop pass)             |

## Other domains (Summary only)

The following domains are not yet matrixed in full. They carry one summary row per file here.
Detailed mapping is tracked under [BL-0044](https://github.com/cjhowe-us/harmonius/issues?q=BL-0044)
and the [2026-Q3 OKR O-3](../okrs/2026-q3.md).

| Domain               | Files | Notes                                                |
|----------------------|------:|------------------------------------------------------|
| Rendering            |    14 | gpu-runtime.md added; remaining files Owned          |
| Geometry             |     6 | Summary only; tracked in 2026-Q3 OKR-3               |
| Physics              |     8 | Cloth/fracture/fluid open via BL-0018, BL-0019       |
| Audio                |     5 | Decompose tracked via BL-0050                        |
| Input                |     5 | Decompose tracked via BL-0051                        |
| Animation            |     7 | Compression open via BL-0020                          |
| UI & 2D              |     6 | Decompose tracked via BL-0052                        |
| VFX                  |     6 | Summary only                                         |
| Content Pipeline     |     7 | Summary only                                         |
| Game Framework       |    27 | Summary only; many compositions per ADR-0011         |
| Platform             |     7 | New telemetry, crash-reporting, console-integration  |
| Tools & Editor       |    24 | New undo-redo, selection-model, scene-versioning, plugin-marketplace |

## Coverage roll-up

| Domain          | Owned | Partial | Orphan | Reversed | Total |
|-----------------|------:|--------:|-------:|---------:|------:|
| Core Runtime    |    11 |       0 |      0 |        0 |    11 |
| Data Systems    |     4 |       0 |      0 |        0 |     4 |
| Simulation      |     4 |       0 |      0 |        0 |     4 |
| Networking      |     6 |       4 |      0 |        0 |    10 |
| AI              |     7 |       2 |      0 |        0 |     9 |
| **Total (indexed)** | **32** | **6** | **0** | **0** |    **38** |

Indexed coverage: 32 / 38 = **84%** Owned across the four matrixed domains. Remaining 6 Partial rows
have backlog issues attached.
