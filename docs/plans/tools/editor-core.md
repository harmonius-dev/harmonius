---
children: []
dependencies: []
design_documents:
  - docs/design/tools/editor-core.md
execution_mode: sequential
features:
  - F-1.2.1
  - F-1.2.4
  - F-1.3.1
  - F-1.3.10
  - F-1.4.1
  - F-1.4.3
  - F-1.5.1
  - F-1.5.7
  - F-1.6.1
  - F-1.6.2
  - F-1.6.3
  - F-1.6.4
  - F-1.6.5
  - F-1.6.6
  - F-1.6.7
  - F-1.9.1
  - F-2.3.8
  - F-10.1.1
  - F-10.2.1
  - F-10.2.2
  - F-10.2.3
  - F-10.2.4
  - F-10.2.5
  - F-10.2.6
  - F-10.2.7
  - F-10.2.8
  - F-14.1.1
  - F-14.3.1
  - F-15.1.1
  - F-15.1.2
  - F-15.1.3
  - F-15.1.4
  - F-15.1.5
  - F-15.1.6
  - F-15.1.7
  - F-15.1.8
  - F-15.1.9
  - F-15.12.3
  - F-15.16.1
  - F-15.16.2
  - F-15.16.3
  - F-15.16.4
  - F-15.16.5
id: PLAN-tools-editor-core
name: Editor Core
parent: null
progress_file: docs/plans/progress/PLAN-tools-editor-core.md
requirements:
  - R-1.6.1
  - R-1.6.2
  - R-1.6.3
  - R-1.6.4
  - R-1.6.5
  - R-1.6.6
  - R-1.6.7
  - R-15.1.1
  - R-15.1.2
  - R-15.1.3
  - R-15.1.4
  - R-15.1.5
  - R-15.1.6
  - R-15.1.7
  - R-15.1.8
  - R-15.1.9
  - R-15.16.1
  - R-15.16.2
  - R-15.16.3
  - R-15.16.4
  - R-15.16.5
status: not_started
test_cases:
  - TC-1.6.1.1
  - TC-1.6.2.1
  - TC-1.6.3.1
  - TC-1.6.4.1
  - TC-1.6.5.1
  - TC-1.6.5.2
  - TC-1.6.6.1
  - TC-1.6.7.1
  - TC-1.6.7.2
  - TC-15.1.1.1
  - TC-15.1.1.2
  - TC-15.1.1.3
  - TC-15.1.1.4
  - TC-15.1.1.5
  - TC-15.1.2.1
  - TC-15.1.2.2
  - TC-15.1.2.3
  - TC-15.1.3.1
  - TC-15.1.3.2
  - TC-15.1.3.3
  - TC-15.1.3.4
  - TC-15.1.3.5
  - TC-15.1.4.1
  - TC-15.1.4.2
  - TC-15.1.4.3
  - TC-15.1.4.4
  - TC-15.1.5.1
  - TC-15.1.5.2
  - TC-15.1.5.3
  - TC-15.1.5.4
  - TC-15.1.6.1
  - TC-15.1.7.1
  - TC-15.1.7.2
  - TC-15.1.9.1
  - TC-15.16.1.1
  - TC-15.16.1.2
  - TC-15.16.2.1
  - TC-15.16.2.2
  - TC-15.16.3.1
  - TC-15.16.4.1
  - TC-15.16.4.2
  - TC-15.16.5.1
worktree_branch: plan/tools-editor-core
---

# Editor Core implementation plan

- Plan ID: `PLAN-tools-editor-core`
- Progress file: [PLAN-tools-editor-core.md](../progress/PLAN-tools-editor-core.md)

## Source documents

- Design: [editor-core.md](../../design/tools/editor-core.md)
- Test cases: [editor-core-test-cases.md](../../design/tools/editor-core-test-cases.md)
- Progress: [PLAN-tools-editor-core.md](../progress/PLAN-tools-editor-core.md)

## Linked specification artifacts

### Features (`docs/features`)

- [skeletal.md](../../features/animation/skeletal.md) — covers `F-1.9.1`
- [asset-versioning.md](../../features/content-pipeline/asset-versioning.md) — covers `F-1.4.1`
- [events-and-messaging.md](../../features/core-runtime/events-and-messaging.md) — covers `F-1.5.7`
- [game-loop.md](../../features/core-runtime/game-loop.md) — covers `F-1.2.4`
- [plugin-system.md](../../features/core-runtime/plugin-system.md) — covers `F-1.6.2`, `F-1.6.3`,
  `F-1.6.4`, `F-1.6.5`, `F-1.6.6`, `F-1.6.7`
- [reflection-and-type-system.md](../../features/core-runtime/reflection-and-type-system.md) —
  covers `F-1.3.10`
- [scene-and-transforms.md](../../features/core-runtime/scene-and-transforms.md) — covers `F-1.2.1`,
  `F-1.2.4`, `F-1.9.1`
- [serialization.md](../../features/core-runtime/serialization.md) — covers `F-1.2.1`, `F-1.4.1`,
  `F-1.4.3`
- [spatial-indexing.md](../../features/core-runtime/spatial-indexing.md) — covers `F-1.2.1`,
  `F-1.9.1`
- [data-tables.md](../../features/data-systems/data-tables.md) — covers `F-1.4.1`
- [character-customization.md](../../features/game-framework/character-customization.md) — covers
  `F-1.4.1`
- [cinematics.md](../../features/game-framework/cinematics.md) — covers `F-10.1.1`
- [game-modes-misc.md](../../features/game-framework/game-modes-misc.md) — covers `F-10.1.1`
- [gameplay-databases.md](../../features/game-framework/gameplay-databases.md) — covers `F-1.4.1`
- [inventory.md](../../features/game-framework/inventory.md) — covers `F-1.4.1`
- [minigames.md](../../features/game-framework/minigames.md) — covers `F-10.1.1`
- [npc-simulation.md](../../features/game-framework/npc-simulation.md) — covers `F-10.1.1`
- [quest-dialogue.md](../../features/game-framework/quest-dialogue.md) — covers `F-1.4.1`,
  `F-10.1.1`
- [save-system.md](../../features/game-framework/save-system.md) — covers `F-1.4.1`
- [selection-system.md](../../features/game-framework/selection-system.md) — covers `F-10.1.1`
- [traversal-interaction.md](../../features/game-framework/traversal-interaction.md) — covers
  `F-1.4.1`
- [weapon-system.md](../../features/game-framework/weapon-system.md) — covers `F-10.1.1`
- [terrain.md](../../features/geometry/terrain.md) — covers `F-1.9.1`
- [input-actions-and-mapping.md](../../features/input/input-actions-and-mapping.md) — covers
  `F-1.4.1`
- [collision-detection.md](../../features/physics/collision-detection.md) — covers `F-1.9.1`
- [destruction-and-fracture.md](../../features/physics/destruction-and-fracture.md) — covers
  `F-1.9.1`
- [spatial-queries.md](../../features/physics/spatial-queries.md) — covers `F-1.9.1`
- [threading-async.md](../../features/platform/threading-async.md) — covers `F-14.3.1`
- [event-logs.md](../../features/simulation/event-logs.md) — covers `F-1.4.1`, `F-1.9.1`
- [grids-volumes.md](../../features/simulation/grids-volumes.md) — covers `F-1.4.1`, `F-14.3.1`
- [spatial-awareness.md](../../features/simulation/spatial-awareness.md) — covers `F-1.9.1`
- [timelines.md](../../features/simulation/timelines.md) — covers `F-1.4.1`
- [ai-assistant.md](../../features/tools/ai-assistant.md) — covers `F-14.1.1`, `F-15.1.1`,
  `F-15.1.2`, `F-15.1.3`, `F-15.1.5`, `F-15.1.7`, `F-15.1.8`
- [ai-cloud-backend.md](../../features/tools/ai-cloud-backend.md) — covers `F-15.1.3`
- [animation-editor.md](../../features/tools/animation-editor.md) — covers `F-15.1.2`, `F-15.1.3`,
  `F-15.1.4`
- [asset-store.md](../../features/tools/asset-store.md) — covers `F-15.1.1`, `F-15.1.3`
- [cloud-build.md](../../features/tools/cloud-build.md) — covers `F-15.1.1`
- [deployment.md](../../features/tools/deployment.md) — covers `F-15.1.1`
- [documentation.md](../../features/tools/documentation.md) — covers `F-1.3.1`, `F-15.1.1`
- [editor-framework.md](../../features/tools/editor-framework.md) — covers `F-14.1.1`, `F-15.1.1`,
  `F-15.1.2`, `F-15.1.3`, `F-15.1.4`, `F-15.1.5`, `F-15.1.6`, `F-15.1.7`...
- [editor-plugins.md](../../features/tools/editor-plugins.md) — covers `F-1.6.1`, `F-15.1.3`,
  `F-15.1.7`, `F-15.1.8`
- [level-editor.md](../../features/tools/level-editor.md) — covers `F-15.1.3`, `F-15.1.4`,
  `F-15.1.5`
- [localization-editor.md](../../features/tools/localization-editor.md) — covers `F-15.1.1`
- [logic-graph.md](../../features/tools/logic-graph.md) — covers `F-1.3.1`, `F-1.5.1`, `F-15.1.1`,
  `F-15.1.3`, `F-15.1.8`
- [material-editor.md](../../features/tools/material-editor.md) — covers `F-15.1.1`
- [mod-support.md](../../features/tools/mod-support.md) — covers `F-15.1.1`, `F-15.16.1`,
  `F-15.16.2`, `F-15.16.3`, `F-15.16.4`, `F-15.16.5`
- [remote-editing.md](../../features/tools/remote-editing.md) — covers `F-15.1.1`, `F-15.1.2`,
  `F-15.1.3`, `F-15.12.3`
- [server-infrastructure.md](../../features/tools/server-infrastructure.md) — covers `F-15.12.3`
- [specialized-editors.md](../../features/tools/specialized-editors.md) — covers `F-15.1.3`,
  `F-15.1.4`, `F-15.12.3`
- [version-control.md](../../features/tools/version-control.md) — covers `F-15.1.1`, `F-15.1.8`,
  `F-15.12.3`
- [vr-editor.md](../../features/tools/vr-editor.md) — covers `F-14.1.1`, `F-15.1.1`, `F-15.1.3`,
  `F-15.1.5`, `F-15.1.9`, `F-15.12.3`
- [world-building.md](../../features/tools/world-building.md) — covers `F-15.1.3`, `F-15.1.5`
- [2d-games.md](../../features/ui/2d-games.md) — covers `F-1.9.1`
- [accessibility.md](../../features/ui/accessibility.md) — covers `F-10.1.1`, `F-10.2.1`
- [common-widgets.md](../../features/ui/common-widgets.md) — covers `F-10.1.1`, `F-10.2.1`,
  `F-10.2.2`, `F-10.2.3`, `F-10.2.4`, `F-10.2.5`, `F-10.2.6`, `F-10.2.7`...
- [hud-and-game-ui.md](../../features/ui/hud-and-game-ui.md) — covers `F-10.1.1`, `F-10.2.1`,
  `F-10.2.2`, `F-10.2.5`, `F-10.2.6`, `F-10.2.7`
- [widget-framework.md](../../features/ui/widget-framework.md) — covers `F-10.1.1`
- [effect-graph.md](../../features/vfx/effect-graph.md) — covers `F-1.9.1`

### Requirements (`docs/requirements`)

- [plugin-system.md](../../requirements/core-runtime/plugin-system.md) — covers `R-1.6.1`,
  `R-1.6.2`, `R-1.6.3`, `R-1.6.4`, `R-1.6.5`, `R-1.6.6`, `R-1.6.7`
- [editor-framework.md](../../requirements/tools/editor-framework.md) — covers `R-15.1.1`,
  `R-15.1.2`, `R-15.1.3`, `R-15.1.4`, `R-15.1.5`, `R-15.1.6`, `R-15.1.7`, `R-15.1.8`...
- [mod-support.md](../../requirements/tools/mod-support.md) — covers `R-15.16.1`, `R-15.16.2`,
  `R-15.16.3`, `R-15.16.4`, `R-15.16.5`

### User stories (`docs/user-stories`)

- [plugin-system.md](../../user-stories/core-runtime/plugin-system.md) — covers `US-1.6.1`,
  `US-1.6.1.1`, `US-1.6.1.6`, `US-1.6.2`, `US-1.6.2.1`, `US-1.6.2.5`, `US-1.6.3`, `US-1.6.3.1`...
- [editor-framework.md](../../user-stories/tools/editor-framework.md) — covers `US-15.1.1`,
  `US-15.1.1.1`, `US-15.1.1.13`, `US-15.1.2`, `US-15.1.2.1`, `US-15.1.2.8`, `US-15.1.3.1`,
  `US-15.1.3.6`...
- [mod-support.md](../../user-stories/tools/mod-support.md) — covers `US-15.16.1`, `US-15.16.1.1`,
  `US-15.16.1.4`, `US-15.16.2`, `US-15.16.2.1`, `US-15.16.2.5`, `US-15.16.3`, `US-15.16.3.1`...

### Test case sources

- [editor-core-test-cases.md](../../design/tools/editor-core-test-cases.md)

### Gap closure decisions

- Normalized `US-1.6.1.1` to `US-1.6.1` using user-stories parent-ID mapping.
- Normalized `US-1.6.1.6` to `US-1.6.1` using user-stories parent-ID mapping.
- Normalized `US-1.6.2.1` to `US-1.6.2` using user-stories parent-ID mapping.
- Normalized `US-1.6.2.5` to `US-1.6.2` using user-stories parent-ID mapping.
- Normalized `US-1.6.3.1` to `US-1.6.3` using user-stories parent-ID mapping.
- Normalized `US-1.6.3.4` to `US-1.6.3` using user-stories parent-ID mapping.
- Normalized `US-1.6.4.1` to `US-1.6.4` using user-stories parent-ID mapping.
- Normalized `US-1.6.4.3` to `US-1.6.4` using user-stories parent-ID mapping.
- Normalized `US-1.6.5.1` to `US-1.6.5` using user-stories parent-ID mapping.
- Normalized `US-1.6.5.4` to `US-1.6.5` using user-stories parent-ID mapping.
- Normalized `US-1.6.6.1` to `US-1.6.6` using user-stories parent-ID mapping.
- Normalized `US-1.6.6.3` to `US-1.6.6` using user-stories parent-ID mapping.
- Normalized `US-1.6.7.1` to `US-1.6.7` using user-stories parent-ID mapping.
- Normalized `US-1.6.7.3` to `US-1.6.7` using user-stories parent-ID mapping.
- Normalized `US-15.1.1.13` to `US-15.1.1` using user-stories parent-ID mapping.
- Normalized `US-15.1.2.8` to `US-15.1.2` using user-stories parent-ID mapping.
- Normalized `US-15.1.3.6` to `US-15.1.3` using user-stories parent-ID mapping.
- Normalized `US-15.1.3.9` to `US-15.1.3` using user-stories parent-ID mapping.
- Normalized `US-15.1.4.10` to `US-15.1.4` using user-stories parent-ID mapping.
- Normalized `US-15.1.5.8` to `US-15.1.5` using user-stories parent-ID mapping.
- All IDs in this plan are mapped to specification artifacts.

## Traceability coverage

### Features

- `F-1.2.1`
- `F-1.2.4`
- `F-1.3.1`
- `F-1.3.10`
- `F-1.4.1`
- `F-1.4.3`
- `F-1.5.1`
- `F-1.5.7`
- `F-1.6.1`
- `F-1.6.2`
- `F-1.6.3`
- `F-1.6.4`
- `F-1.6.5`
- `F-1.6.6`
- `F-1.6.7`
- `F-1.9.1`
- `F-2.3.8`
- `F-10.1.1`
- `F-10.2.1`
- `F-10.2.2`
- `F-10.2.3`
- `F-10.2.4`
- `F-10.2.5`
- `F-10.2.6`
- `F-10.2.7`
- `F-10.2.8`
- `F-14.1.1`
- `F-14.3.1`
- `F-15.1.1`
- `F-15.1.2`
- `F-15.1.3`
- `F-15.1.4`
- `F-15.1.5`
- `F-15.1.6`
- `F-15.1.7`
- `F-15.1.8`
- `F-15.1.9`
- `F-15.12.3`
- `F-15.16.1`
- `F-15.16.2`
- `F-15.16.3`
- `F-15.16.4`
- `F-15.16.5`

### Requirements

- `R-1.6.1`
- `R-1.6.2`
- `R-1.6.3`
- `R-1.6.4`
- `R-1.6.5`
- `R-1.6.6`
- `R-1.6.7`
- `R-15.1.1`
- `R-15.1.2`
- `R-15.1.3`
- `R-15.1.4`
- `R-15.1.5`
- `R-15.1.6`
- `R-15.1.7`
- `R-15.1.8`
- `R-15.1.9`
- `R-15.16.1`
- `R-15.16.2`
- `R-15.16.3`
- `R-15.16.4`
- `R-15.16.5`

### User stories

- `US-1.6.1`
- `US-1.6.1.1`
- `US-1.6.1.6`
- `US-1.6.2`
- `US-1.6.2.1`
- `US-1.6.2.5`
- `US-1.6.3`
- `US-1.6.3.1`
- `US-1.6.3.4`
- `US-1.6.4`
- `US-1.6.4.1`
- `US-1.6.4.3`
- `US-1.6.5`
- `US-1.6.5.1`
- `US-1.6.5.4`
- `US-1.6.6`
- `US-1.6.6.1`
- `US-1.6.6.3`
- `US-1.6.7`
- `US-1.6.7.1`
- `US-1.6.7.3`
- `US-15.1.1`
- `US-15.1.1.1`
- `US-15.1.1.13`
- `US-15.1.2`
- `US-15.1.2.1`
- `US-15.1.2.8`
- `US-15.1.3.1`
- `US-15.1.3.6`
- `US-15.1.3.9`
- `US-15.1.4`
- `US-15.1.4.1`
- `US-15.1.4.10`
- `US-15.1.5`
- `US-15.1.5.1`
- `US-15.1.5.8`
- `US-15.1.6`
- `US-15.1.6.1`
- `US-15.1.6.6`
- `US-15.1.7`
- `US-15.1.7.1`
- `US-15.1.7.8`
- `US-15.1.8`
- `US-15.1.8.1`
- `US-15.1.8.9`
- `US-15.1.9`
- `US-15.1.9.1`
- `US-15.1.9.8`
- `US-15.16.1`
- `US-15.16.1.1`
- `US-15.16.1.4`
- `US-15.16.2`
- `US-15.16.2.1`
- `US-15.16.2.5`
- `US-15.16.3`
- `US-15.16.3.1`
- `US-15.16.3.3`
- `US-15.16.4`
- `US-15.16.4.1`
- `US-15.16.4.4`
- `US-15.16.5.1`
- `US-15.16.5.2`
- `US-15.16.5.3`

### Test cases

- `TC-1.6.1.1`
- `TC-1.6.2.1`
- `TC-1.6.3.1`
- `TC-1.6.4.1`
- `TC-1.6.5.1`
- `TC-1.6.5.2`
- `TC-1.6.6.1`
- `TC-1.6.7.1`
- `TC-1.6.7.2`
- `TC-15.1.1.1`
- `TC-15.1.1.2`
- `TC-15.1.1.3`
- `TC-15.1.1.4`
- `TC-15.1.1.5`
- `TC-15.1.2.1`
- `TC-15.1.2.2`
- `TC-15.1.2.3`
- `TC-15.1.3.1`
- `TC-15.1.3.2`
- `TC-15.1.3.3`
- `TC-15.1.3.4`
- `TC-15.1.3.5`
- `TC-15.1.4.1`
- `TC-15.1.4.2`
- `TC-15.1.4.3`
- `TC-15.1.4.4`
- `TC-15.1.5.1`
- `TC-15.1.5.2`
- `TC-15.1.5.3`
- `TC-15.1.5.4`
- `TC-15.1.6.1`
- `TC-15.1.7.1`
- `TC-15.1.7.2`
- `TC-15.1.9.1`
- `TC-15.16.1.1`
- `TC-15.16.1.2`
- `TC-15.16.2.1`
- `TC-15.16.2.2`
- `TC-15.16.3.1`
- `TC-15.16.4.1`
- `TC-15.16.4.2`
- `TC-15.16.5.1`

## Step-by-step implementation workflow

1. Confirm scope boundaries and dependencies from `docs/plans/index.md`.
2. Build trace matrix from every linked `R-*`, `US-*`, and `TC-*` item.
3. Add failing tests for one behavior slice at a time (red).
4. Implement the smallest deterministic change to pass those tests (green).
5. Refactor internal structure while preserving behavior and passing tests.
6. Integrate with adjacent subsystems through explicit interfaces and events.
7. Validate constraints, performance budgets, and fallback behavior.
8. Collect evidence artifacts and update progress checklist and event log.

## Algorithm-level plan

- Data transforms use pure functions to preserve determinism and replayability.
- Search or selection paths follow design-defined bounded algorithms.
- Scheduling follows explicit phase ordering to preserve dependency correctness.
- Fallback paths degrade gracefully with telemetry instead of hard failure.
- Integration points are validated at ECS boundaries and serialized interfaces.

## TDD-first sequencing

### Red

- Create failing tests for each uncovered behavior in `TC-*`, `R-*`, and `US-*` scope.
- Keep fixtures immutable and deterministic; do not use mock frameworks.
- Verify failures indicate missing behavior, not test harness defects.

### Green

- Implement minimal code to satisfy the current failing slice.
- Keep side effects at explicit boundaries (IO seams, command buffers).
- Re-run focused suites after each slice.

### Refactor

- Simplify structure and remove duplication without changing externally visible behavior.
- Re-run full test suite and lint checks before advancing status.

## Complete test plan

- Unit coverage for each requirement-level behavior and edge case.
- Integration coverage for subsystem boundaries and data contracts.
- Benchmark coverage for documented performance targets where present.
- Regression coverage for previously delivered behaviors in this area.

### Test inventory

- `TC-1.6.1.1`
- `TC-1.6.2.1`
- `TC-1.6.3.1`
- `TC-1.6.4.1`
- `TC-1.6.5.1`
- `TC-1.6.5.2`
- `TC-1.6.6.1`
- `TC-1.6.7.1`
- `TC-1.6.7.2`
- `TC-15.1.1.1`
- `TC-15.1.1.2`
- `TC-15.1.1.3`
- `TC-15.1.1.4`
- `TC-15.1.1.5`
- `TC-15.1.2.1`
- `TC-15.1.2.2`
- `TC-15.1.2.3`
- `TC-15.1.3.1`
- `TC-15.1.3.2`
- `TC-15.1.3.3`
- `TC-15.1.3.4`
- `TC-15.1.3.5`
- `TC-15.1.4.1`
- `TC-15.1.4.2`
- `TC-15.1.4.3`
- `TC-15.1.4.4`
- `TC-15.1.5.1`
- `TC-15.1.5.2`
- `TC-15.1.5.3`
- `TC-15.1.5.4`
- `TC-15.1.6.1`
- `TC-15.1.7.1`
- `TC-15.1.7.2`
- `TC-15.1.9.1`
- `TC-15.16.1.1`
- `TC-15.16.1.2`
- `TC-15.16.2.1`
- `TC-15.16.2.2`
- `TC-15.16.3.1`
- `TC-15.16.4.1`
- `TC-15.16.4.2`
- `TC-15.16.5.1`

## Integration and constraint validation

- Validate ECS composition and no hidden mutable global state.
- Validate engine threading and IO constraints from `docs/design/constraints.md`.
- Validate deterministic ordering for equal inputs and fixed seeds.
- Validate cross-subsystem compatibility at documented interfaces.

## Assumptions and fallback handling

- If design prose conflicts with examples, treat normative requirement trace as canonical.
- If companion test-case docs are missing, derive exhaustive tests from `R-*` and `US-*`.
- If dependency behavior is unavailable, gate features with explicit safe fallbacks.
- Log assumptions and fallback decisions in the progress event log.

## Manual validation procedures

1. Execute primary and failure-path scenarios for each linked user story.
2. Capture screenshots for state transitions and visible acceptance points.
3. Capture short videos for temporal behaviors and recovery flows.
4. Record expected vs observed outcomes and link artifacts in progress evidence.
5. Treat validation as incomplete if evidence or acceptance criteria are missing.

## Open questions resolution for implementation readiness

- Input schema ambiguity: resolved by using design type signatures as authoritative.
- Behavior tie-breakers: resolved by deterministic ordering (stable sort / explicit priority).
- Missing artifact granularity: resolved by derived tests tied to nearest mapped IDs.
- Runtime failure modes: resolved with fail-safe fallback and telemetry emission.
- Manual validation controls: resolved with scripted scenario list + evidence capture checklist.

## Gap closure and open question resolution

### Coverage gap closure

- `US-1.6.1.1` resolved via parent `US-1.6.1` in
  [plugin-system.md](../../user-stories/core-runtime/plugin-system.md).
- `US-1.6.1.6` resolved via parent `US-1.6.1` in
  [plugin-system.md](../../user-stories/core-runtime/plugin-system.md).
- `US-1.6.2.1` resolved via parent `US-1.6.2` in
  [plugin-system.md](../../user-stories/core-runtime/plugin-system.md).
- `US-1.6.2.5` resolved via parent `US-1.6.2` in
  [plugin-system.md](../../user-stories/core-runtime/plugin-system.md).
- `US-1.6.3.1` resolved via parent `US-1.6.3` in
  [plugin-system.md](../../user-stories/core-runtime/plugin-system.md).
- `US-1.6.3.4` resolved via parent `US-1.6.3` in
  [plugin-system.md](../../user-stories/core-runtime/plugin-system.md).
- `US-1.6.4.1` resolved via parent `US-1.6.4` in
  [plugin-system.md](../../user-stories/core-runtime/plugin-system.md).
- `US-1.6.4.3` resolved via parent `US-1.6.4` in
  [plugin-system.md](../../user-stories/core-runtime/plugin-system.md).
- `US-1.6.5.1` resolved via parent `US-1.6.5` in
  [plugin-system.md](../../user-stories/core-runtime/plugin-system.md).
- `US-1.6.5.4` resolved via parent `US-1.6.5` in
  [plugin-system.md](../../user-stories/core-runtime/plugin-system.md).
- `US-1.6.6.1` resolved via parent `US-1.6.6` in
  [plugin-system.md](../../user-stories/core-runtime/plugin-system.md).
- `US-1.6.6.3` resolved via parent `US-1.6.6` in
  [plugin-system.md](../../user-stories/core-runtime/plugin-system.md).
- `US-1.6.7.1` resolved via parent `US-1.6.7` in
  [plugin-system.md](../../user-stories/core-runtime/plugin-system.md).
- `US-1.6.7.3` resolved via parent `US-1.6.7` in
  [plugin-system.md](../../user-stories/core-runtime/plugin-system.md).
- `US-15.1.1.13` resolved via parent `US-15.1.1` in
  [editor-framework.md](../../user-stories/tools/editor-framework.md).
- `US-15.1.2.8` resolved via parent `US-15.1.2` in
  [editor-framework.md](../../user-stories/tools/editor-framework.md).
- `US-15.1.3.6` resolved via parent `US-15.1.3` in
  [editor-framework.md](../../user-stories/tools/editor-framework.md).
- `US-15.1.3.9` resolved via parent `US-15.1.3` in
  [editor-framework.md](../../user-stories/tools/editor-framework.md).
- `US-15.1.4.10` resolved via parent `US-15.1.4` in
  [editor-framework.md](../../user-stories/tools/editor-framework.md).
- `US-15.1.5.8` resolved via parent `US-15.1.5` in
  [editor-framework.md](../../user-stories/tools/editor-framework.md).
- `US-15.1.6.6` resolved via parent `US-15.1.6` in
  [editor-framework.md](../../user-stories/tools/editor-framework.md).
- `US-15.1.7.8` resolved via parent `US-15.1.7` in
  [editor-framework.md](../../user-stories/tools/editor-framework.md).
- `US-15.1.8.9` resolved via parent `US-15.1.8` in
  [editor-framework.md](../../user-stories/tools/editor-framework.md).
- `US-15.1.9.8` resolved via parent `US-15.1.9` in
  [editor-framework.md](../../user-stories/tools/editor-framework.md).
- `US-15.16.1.4` resolved via parent `US-15.16.1` in
  [mod-support.md](../../user-stories/tools/mod-support.md).
- `US-15.16.2.5` resolved via parent `US-15.16.2` in
  [mod-support.md](../../user-stories/tools/mod-support.md).
- `US-15.16.3.3` resolved via parent `US-15.16.3` in
  [mod-support.md](../../user-stories/tools/mod-support.md).
- `US-15.16.4.4` resolved via parent `US-15.16.4` in
  [mod-support.md](../../user-stories/tools/mod-support.md).
- `US-15.16.5.3` resolved via parent `US-15.16.5` in
  [mod-support.md](../../user-stories/tools/mod-support.md).

### Remaining open questions

- None. All prior previously unmapped IDs are resolved by deterministic parent-chain mapping or by
  nearest canonical spec lineage in the same subsystem.

### TDD implementation defaults

- Default to pure-function first implementations (`Input -> Output`) before side effects.
- For each previously unmapped dependency at runtime, gate the path behind deterministic fallback.
- Define test-first acceptance with explicit fixture IDs tied to `TC-*` rows.
- Capture one screenshot per state transition and one short video per temporal behavior.
- Promote plan status only after red/green/refactor, integration, and evidence checks pass.

## Completion criteria

- All linked `R-*`, `US-*`, and `TC-*` items have passing evidence.
- All integration and constraints checks pass without previously unmapped blockers.
- Progress status is `code_complete` only after red, green, and refactor completion.
