---
children: []
dependencies: []
design_documents:
  - docs/design/geometry/world-geometry.md
execution_mode: sequential
features:
  - F-1.1.1
  - F-1.1.34
  - F-1.1.35
  - F-1.2.4
  - F-1.7.6
  - F-1.9.1
  - F-2.1.1
  - F-2.2.1
  - F-2.10.1
  - F-3.1.1
  - F-3.1.2
  - F-3.1.3
  - F-3.1.4
  - F-3.1.5
  - F-3.1.6
  - F-3.1.7
  - F-3.2.1
  - F-3.2.2
  - F-3.2.3
  - F-3.2.4
  - F-3.2.5
  - F-3.2.6
  - F-3.2.7
  - F-3.2.8
  - F-3.2.9
  - F-3.2.10
  - F-3.2.11
  - F-3.2.12
  - F-3.2.13
  - F-3.2.14
  - F-3.3.1
  - F-3.3.2
  - F-3.3.3
  - F-3.3.4
  - F-3.3.5
  - F-3.3.6
  - F-3.3.7
  - F-3.4.1
  - F-3.4.2
  - F-3.4.3
  - F-3.4.4
  - F-3.4.5
  - F-3.4.6
  - F-3.4.7
  - F-3.5.1
  - F-3.5.2
  - F-3.5.3
  - F-3.5.4
  - F-3.5.5
  - F-3.5.6
  - F-3.5.7
  - F-3.6.12
  - F-4.7.5
  - F-4.8.5
  - F-12.2.3
  - F-12.5.2
  - F-14.3.1
id: PLAN-geometry-world-geometry
name: World Geometry
parent: null
progress_file: docs/plans/progress/PLAN-geometry-world-geometry.md
requirements:
  - R-3.1.1
  - R-3.1.2
  - R-3.1.3
  - R-3.1.4
  - R-3.1.5
  - R-3.1.6
  - R-3.1.7
  - R-3.2.1
  - R-3.2.2
  - R-3.2.3
  - R-3.2.4
  - R-3.2.5
  - R-3.2.6
  - R-3.2.7
  - R-3.2.8
  - R-3.2.9
  - R-3.2.10
  - R-3.2.11
  - R-3.2.12
  - R-3.2.13
  - R-3.2.14
  - R-3.3.1
  - R-3.3.2
  - R-3.3.3
  - R-3.3.4
  - R-3.3.5
  - R-3.3.6
  - R-3.3.7
  - R-3.4.1
  - R-3.4.2
  - R-3.4.3
  - R-3.4.4
  - R-3.4.5
  - R-3.4.6
  - R-3.4.7
  - R-3.5.1
  - R-3.5.2
  - R-3.5.3
  - R-3.5.4
  - R-3.5.5
  - R-3.5.6
  - R-3.5.7
status: not_started
test_cases:
  - TC-3.1.1.1
  - TC-3.1.1.2
  - TC-3.1.1.3
  - TC-3.1.1.4
  - TC-3.1.1.5
  - TC-3.1.1.6
  - TC-3.1.1.7
  - TC-3.1.5.1
  - TC-3.1.5.2
  - TC-3.1.5.3
  - TC-3.1.6.1
  - TC-3.1.6.2
  - TC-3.1.7.1
  - TC-3.1.7.2
  - TC-3.2.1.1
  - TC-3.2.1.2
  - TC-3.2.1.3
  - TC-3.2.1.4
  - TC-3.2.1.5
  - TC-3.2.1.6
  - TC-3.2.2.1
  - TC-3.2.2.2
  - TC-3.2.3.1
  - TC-3.2.3.2
  - TC-3.2.3.3
  - TC-3.2.4.1
  - TC-3.2.4.2
  - TC-3.2.5.1
  - TC-3.2.5.2
  - TC-3.2.6.1
  - TC-3.2.6.2
  - TC-3.2.6.3
  - TC-3.2.7.1
  - TC-3.2.7.2
  - TC-3.2.8.1
  - TC-3.2.9.1
  - TC-3.2.9.2
  - TC-3.2.10.1
  - TC-3.2.11.1
  - TC-3.2.11.2
  - TC-3.2.11.3
  - TC-3.2.12.1
  - TC-3.2.12.2
  - TC-3.2.12.3
  - TC-3.2.12.4
  - TC-3.2.13.1
  - TC-3.2.13.2
  - TC-3.2.13.3
  - TC-3.2.13.4
  - TC-3.2.13.5
  - TC-3.2.13.6
  - TC-3.2.13.7
  - TC-3.2.13.8
  - TC-3.2.14.1
  - TC-3.2.14.2
  - TC-3.2.14.3
  - TC-3.2.14.4
  - TC-3.3.2.1
  - TC-3.3.2.2
  - TC-3.3.2.3
  - TC-3.3.3.1
  - TC-3.3.3.2
  - TC-3.3.4.1
  - TC-3.3.4.2
  - TC-3.3.5.1
  - TC-3.3.6.1
  - TC-3.3.7.1
  - TC-3.4.1.1
  - TC-3.4.1.2
  - TC-3.4.1.3
  - TC-3.4.2.1
  - TC-3.4.2.2
  - TC-3.4.3.1
  - TC-3.4.3.2
  - TC-3.4.4.1
  - TC-3.4.5.1
  - TC-3.4.6.1
  - TC-3.4.7.1
  - TC-3.4.7.2
  - TC-3.5.1.1
  - TC-3.5.1.2
  - TC-3.5.2.1
  - TC-3.5.2.2
  - TC-3.5.2.3
  - TC-3.5.3.1
  - TC-3.5.3.2
  - TC-3.5.4.1
  - TC-3.5.4.2
  - TC-3.5.5.1
  - TC-3.5.5.2
  - TC-3.5.6.1
  - TC-3.5.6.2
  - TC-3.5.6.3
  - TC-3.5.7.1
worktree_branch: plan/geometry-world-geometry
---

# World Geometry implementation plan

- Plan ID: `PLAN-geometry-world-geometry`
- Progress file: [PLAN-geometry-world-geometry.md](../progress/PLAN-geometry-world-geometry.md)

## Source documents

- Design: [world-geometry.md](../../design/geometry/world-geometry.md)
- Test cases: [world-geometry-test-cases.md](../../design/geometry/world-geometry-test-cases.md)
- Progress: [PLAN-geometry-world-geometry.md](../progress/PLAN-geometry-world-geometry.md)

## Linked specification artifacts

### Features (`docs/features`)

- [asset-import.md](../../features/content-pipeline/asset-import.md) — covers `F-12.5.2`
- [asset-processing.md](../../features/content-pipeline/asset-processing.md) — covers `F-12.2.3`
- [streaming-io.md](../../features/content-pipeline/streaming-io.md) — covers `F-12.5.2`
- [game-loop.md](../../features/core-runtime/game-loop.md) — covers `F-1.2.4`
- [scene-and-transforms.md](../../features/core-runtime/scene-and-transforms.md) — covers `F-1.2.4`
- [building-survival.md](../../features/game-framework/building-survival.md) — covers `F-2.10.1`
- [fog-of-war.md](../../features/game-framework/fog-of-war.md) — covers `F-2.10.1`
- [selection-system.md](../../features/game-framework/selection-system.md) — covers `F-2.10.1`
- [turn-based.md](../../features/game-framework/turn-based.md) — covers `F-2.10.1`
- [weapon-system.md](../../features/game-framework/weapon-system.md) — covers `F-2.10.1`
- [foliage.md](../../features/geometry/foliage.md) — covers `F-3.1.1`, `F-3.1.3`, `F-3.2.1`,
  `F-3.3.1`, `F-3.3.2`, `F-3.3.3`, `F-3.3.4`, `F-3.3.5`...
- [meshlet-pipeline.md](../../features/geometry/meshlet-pipeline.md) — covers `F-3.1.1`, `F-3.1.2`,
  `F-3.1.3`, `F-3.1.4`, `F-3.1.5`, `F-3.1.6`, `F-3.1.7`
- [procedural-generation.md](../../features/geometry/procedural-generation.md) — covers `F-1.1.1`,
  `F-2.1.1`, `F-3.2.1`, `F-3.2.3`, `F-3.2.5`, `F-3.2.7`, `F-3.3.1`, `F-3.3.2`...
- [sky-atmosphere.md](../../features/geometry/sky-atmosphere.md) — covers `F-3.5.1`, `F-3.5.2`,
  `F-3.5.3`, `F-3.5.4`, `F-3.5.5`, `F-3.5.6`, `F-3.5.7`
- [terrain.md](../../features/geometry/terrain.md) — covers `F-1.1.34`, `F-1.1.35`, `F-1.7.6`,
  `F-1.9.1`, `F-3.1.1`, `F-3.2.1`, `F-3.2.10`, `F-3.2.11`...
- [water.md](../../features/geometry/water.md) — covers `F-3.2.1`, `F-3.4.1`, `F-3.4.2`, `F-3.4.3`,
  `F-3.4.4`, `F-3.4.5`, `F-3.4.6`, `F-3.4.7`...
- [collision-detection.md](../../features/physics/collision-detection.md) — covers `F-12.2.3`
- [threading-async.md](../../features/platform/threading-async.md) — covers `F-14.3.1`
- [core-rendering.md](../../features/rendering/core-rendering.md) — covers `F-2.2.1`
- [first-person-rendering.md](../../features/rendering/first-person-rendering.md) — covers
  `F-2.10.1`, `F-2.2.1`
- [render-graph.md](../../features/rendering/render-graph.md) — covers `F-2.2.1`
- [scene-rendering-pipeline.md](../../features/rendering/scene-rendering-pipeline.md) — covers
  `F-2.10.1`, `F-2.2.1`
- [stylized-effects.md](../../features/rendering/stylized-effects.md) — covers `F-2.10.1`
- [grids-volumes.md](../../features/simulation/grids-volumes.md) — covers `F-14.3.1`
- [2d-games.md](../../features/ui/2d-games.md) — covers `F-2.10.1`
- [ui-rendering.md](../../features/ui/ui-rendering.md) — covers `F-2.2.1`

### Requirements (`docs/requirements`)

- [foliage.md](../../requirements/geometry/foliage.md) — covers `R-3.3.1`, `R-3.3.2`, `R-3.3.3`,
  `R-3.3.4`, `R-3.3.5`, `R-3.3.6`, `R-3.3.7`
- [meshlet-pipeline.md](../../requirements/geometry/meshlet-pipeline.md) — covers `R-3.1.1`,
  `R-3.1.2`, `R-3.1.3`, `R-3.1.4`, `R-3.1.5`, `R-3.1.6`, `R-3.1.7`
- [sky-atmosphere.md](../../requirements/geometry/sky-atmosphere.md) — covers `R-3.5.1`, `R-3.5.2`,
  `R-3.5.3`, `R-3.5.4`, `R-3.5.5`, `R-3.5.6`, `R-3.5.7`
- [terrain.md](../../requirements/geometry/terrain.md) — covers `R-3.2.1`, `R-3.2.10`, `R-3.2.11`,
  `R-3.2.12`, `R-3.2.13`, `R-3.2.14`, `R-3.2.2`, `R-3.2.3`...
- [water.md](../../requirements/geometry/water.md) — covers `R-3.4.1`, `R-3.4.2`, `R-3.4.3`,
  `R-3.4.4`, `R-3.4.5`, `R-3.4.6`, `R-3.4.7`

### User stories (`docs/user-stories`)

- [foliage.md](../../user-stories/geometry/foliage.md) — covers `US-3.3.1`, `US-3.3.1.1`,
  `US-3.3.1.2`, `US-3.3.1.3`, `US-3.3.1.4`, `US-3.3.1.5`, `US-3.3.2`, `US-3.3.2.1`...
- [meshlet-pipeline.md](../../user-stories/geometry/meshlet-pipeline.md) — covers `US-3.1.1`,
  `US-3.1.1.1`, `US-3.1.1.2`, `US-3.1.1.4`, `US-3.1.2`, `US-3.1.2.1`, `US-3.1.2.2`, `US-3.1.2.3`...
- [sky-atmosphere.md](../../user-stories/geometry/sky-atmosphere.md) — covers `US-3.5.1`,
  `US-3.5.1.1`, `US-3.5.1.2`, `US-3.5.1.3`, `US-3.5.2`, `US-3.5.2.1`, `US-3.5.2.2`, `US-3.5.2.3`...
- [terrain.md](../../user-stories/geometry/terrain.md) — covers `US-3.2.1`, `US-3.2.1.1`,
  `US-3.2.1.3`, `US-3.2.1.4`, `US-3.2.10`, `US-3.2.10.1`, `US-3.2.10.2`, `US-3.2.11`...
- [water.md](../../user-stories/geometry/water.md) — covers `US-3.4.1`, `US-3.4.1.1`, `US-3.4.1.2`,
  `US-3.4.1.3`, `US-3.4.1.4`, `US-3.4.2`, `US-3.4.2.1`, `US-3.4.2.2`...

### Test case sources

- [world-geometry-test-cases.md](../../design/geometry/world-geometry-test-cases.md)

### Gap closure decisions

- No normalization was required for this plan.
- All IDs in this plan are mapped to specification artifacts.

## Traceability coverage

### Features

- `F-1.1.1`
- `F-1.1.34`
- `F-1.1.35`
- `F-1.2.4`
- `F-1.7.6`
- `F-1.9.1`
- `F-2.1.1`
- `F-2.2.1`
- `F-2.10.1`
- `F-3.1.1`
- `F-3.1.2`
- `F-3.1.3`
- `F-3.1.4`
- `F-3.1.5`
- `F-3.1.6`
- `F-3.1.7`
- `F-3.2.1`
- `F-3.2.2`
- `F-3.2.3`
- `F-3.2.4`
- `F-3.2.5`
- `F-3.2.6`
- `F-3.2.7`
- `F-3.2.8`
- `F-3.2.9`
- `F-3.2.10`
- `F-3.2.11`
- `F-3.2.12`
- `F-3.2.13`
- `F-3.2.14`
- `F-3.3.1`
- `F-3.3.2`
- `F-3.3.3`
- `F-3.3.4`
- `F-3.3.5`
- `F-3.3.6`
- `F-3.3.7`
- `F-3.4.1`
- `F-3.4.2`
- `F-3.4.3`
- `F-3.4.4`
- `F-3.4.5`
- `F-3.4.6`
- `F-3.4.7`
- `F-3.5.1`
- `F-3.5.2`
- `F-3.5.3`
- `F-3.5.4`
- `F-3.5.5`
- `F-3.5.6`
- `F-3.5.7`
- `F-3.6.12`
- `F-4.7.5`
- `F-4.8.5`
- `F-12.2.3`
- `F-12.5.2`
- `F-14.3.1`

### Requirements

- `R-3.1.1`
- `R-3.1.2`
- `R-3.1.3`
- `R-3.1.4`
- `R-3.1.5`
- `R-3.1.6`
- `R-3.1.7`
- `R-3.2.1`
- `R-3.2.2`
- `R-3.2.3`
- `R-3.2.4`
- `R-3.2.5`
- `R-3.2.6`
- `R-3.2.7`
- `R-3.2.8`
- `R-3.2.9`
- `R-3.2.10`
- `R-3.2.11`
- `R-3.2.12`
- `R-3.2.13`
- `R-3.2.14`
- `R-3.3.1`
- `R-3.3.2`
- `R-3.3.3`
- `R-3.3.4`
- `R-3.3.5`
- `R-3.3.6`
- `R-3.3.7`
- `R-3.4.1`
- `R-3.4.2`
- `R-3.4.3`
- `R-3.4.4`
- `R-3.4.5`
- `R-3.4.6`
- `R-3.4.7`
- `R-3.5.1`
- `R-3.5.2`
- `R-3.5.3`
- `R-3.5.4`
- `R-3.5.5`
- `R-3.5.6`
- `R-3.5.7`

### User stories

- `US-3.1.1`
- `US-3.1.1.1`
- `US-3.1.1.2`
- `US-3.1.1.4`
- `US-3.1.2`
- `US-3.1.2.1`
- `US-3.1.2.2`
- `US-3.1.2.3`
- `US-3.1.3`
- `US-3.1.3.1`
- `US-3.1.3.2`
- `US-3.1.3.3`
- `US-3.1.4`
- `US-3.1.4.1`
- `US-3.1.4.2`
- `US-3.1.4.3`
- `US-3.1.5`
- `US-3.1.5.1`
- `US-3.1.5.3`
- `US-3.1.5.4`
- `US-3.1.6`
- `US-3.1.6.1`
- `US-3.1.6.2`
- `US-3.1.6.3`
- `US-3.1.6.4`
- `US-3.1.7`
- `US-3.1.7.1`
- `US-3.1.7.2`
- `US-3.1.7.3`
- `US-3.2.1`
- `US-3.2.1.1`
- `US-3.2.1.3`
- `US-3.2.1.4`
- `US-3.2.2`
- `US-3.2.2.1`
- `US-3.2.2.2`
- `US-3.2.2.3`
- `US-3.2.3`
- `US-3.2.3.1`
- `US-3.2.3.2`
- `US-3.2.3.3`
- `US-3.2.4`
- `US-3.2.4.1`
- `US-3.2.4.2`
- `US-3.2.5`
- `US-3.2.5.1`
- `US-3.2.5.2`
- `US-3.2.6`
- `US-3.2.6.1`
- `US-3.2.6.2`
- `US-3.2.7`
- `US-3.2.7.1`
- `US-3.2.7.2`
- `US-3.2.8.1`
- `US-3.2.8.2`
- `US-3.2.8.3`
- `US-3.2.9`
- `US-3.2.9.1`
- `US-3.2.9.2`
- `US-3.2.10`
- `US-3.2.10.1`
- `US-3.2.10.2`
- `US-3.2.11`
- `US-3.2.11.1`
- `US-3.2.11.2`
- `US-3.2.12`
- `US-3.2.12.1`
- `US-3.2.12.2`
- `US-3.2.13`
- `US-3.2.13.1`
- `US-3.2.14`
- `US-3.2.14.1`
- `US-3.3.1`
- `US-3.3.1.1`
- `US-3.3.1.2`
- `US-3.3.1.3`
- `US-3.3.1.4`
- `US-3.3.1.5`
- `US-3.3.2`
- `US-3.3.2.1`
- `US-3.3.2.2`
- `US-3.3.2.3`
- `US-3.3.2.4`
- `US-3.3.3`
- `US-3.3.3.1`
- `US-3.3.3.2`
- `US-3.3.3.3`
- `US-3.3.3.4`
- `US-3.3.4`
- `US-3.3.4.1`
- `US-3.3.4.2`
- `US-3.3.4.3`
- `US-3.3.4.4`
- `US-3.3.5`
- `US-3.3.5.1`
- `US-3.3.5.2`
- `US-3.3.5.3`
- `US-3.3.5.4`
- `US-3.3.6`
- `US-3.3.6.1`
- `US-3.3.6.2`
- `US-3.3.6.3`
- `US-3.3.6.4`
- `US-3.3.7`
- `US-3.3.7.1`
- `US-3.3.7.2`
- `US-3.3.7.3`
- `US-3.3.7.4`
- `US-3.4.1`
- `US-3.4.1.1`
- `US-3.4.1.2`
- `US-3.4.1.3`
- `US-3.4.1.4`
- `US-3.4.2`
- `US-3.4.2.1`
- `US-3.4.2.2`
- `US-3.4.2.3`
- `US-3.4.3`
- `US-3.4.3.1`
- `US-3.4.3.2`
- `US-3.4.3.3`
- `US-3.4.4`
- `US-3.4.4.1`
- `US-3.4.4.2`
- `US-3.4.4.3`
- `US-3.4.5`
- `US-3.4.5.1`
- `US-3.4.5.2`
- `US-3.4.5.3`
- `US-3.4.6`
- `US-3.4.6.1`
- `US-3.4.6.2`
- `US-3.4.6.3`
- `US-3.4.7`
- `US-3.4.7.1`
- `US-3.4.7.2`
- `US-3.4.7.3`
- `US-3.5.1`
- `US-3.5.1.1`
- `US-3.5.1.2`
- `US-3.5.1.3`
- `US-3.5.2`
- `US-3.5.2.1`
- `US-3.5.2.2`
- `US-3.5.2.3`
- `US-3.5.2.4`
- `US-3.5.3`
- `US-3.5.3.1`
- `US-3.5.3.2`
- `US-3.5.3.3`
- `US-3.5.3.4`
- `US-3.5.4`
- `US-3.5.4.1`
- `US-3.5.4.2`
- `US-3.5.5`
- `US-3.5.5.1`
- `US-3.5.5.2`
- `US-3.5.5.3`
- `US-3.5.6`
- `US-3.5.6.1`
- `US-3.5.6.2`
- `US-3.5.6.3`
- `US-3.5.7`
- `US-3.5.7.1`
- `US-3.5.7.2`
- `US-3.5.7.3`

### Test cases

- `TC-3.1.1.1`
- `TC-3.1.1.2`
- `TC-3.1.1.3`
- `TC-3.1.1.4`
- `TC-3.1.1.5`
- `TC-3.1.1.6`
- `TC-3.1.1.7`
- `TC-3.1.5.1`
- `TC-3.1.5.2`
- `TC-3.1.5.3`
- `TC-3.1.6.1`
- `TC-3.1.6.2`
- `TC-3.1.7.1`
- `TC-3.1.7.2`
- `TC-3.2.1.1`
- `TC-3.2.1.2`
- `TC-3.2.1.3`
- `TC-3.2.1.4`
- `TC-3.2.1.5`
- `TC-3.2.1.6`
- `TC-3.2.2.1`
- `TC-3.2.2.2`
- `TC-3.2.3.1`
- `TC-3.2.3.2`
- `TC-3.2.3.3`
- `TC-3.2.4.1`
- `TC-3.2.4.2`
- `TC-3.2.5.1`
- `TC-3.2.5.2`
- `TC-3.2.6.1`
- `TC-3.2.6.2`
- `TC-3.2.6.3`
- `TC-3.2.7.1`
- `TC-3.2.7.2`
- `TC-3.2.8.1`
- `TC-3.2.9.1`
- `TC-3.2.9.2`
- `TC-3.2.10.1`
- `TC-3.2.11.1`
- `TC-3.2.11.2`
- `TC-3.2.11.3`
- `TC-3.2.12.1`
- `TC-3.2.12.2`
- `TC-3.2.12.3`
- `TC-3.2.12.4`
- `TC-3.2.13.1`
- `TC-3.2.13.2`
- `TC-3.2.13.3`
- `TC-3.2.13.4`
- `TC-3.2.13.5`
- `TC-3.2.13.6`
- `TC-3.2.13.7`
- `TC-3.2.13.8`
- `TC-3.2.14.1`
- `TC-3.2.14.2`
- `TC-3.2.14.3`
- `TC-3.2.14.4`
- `TC-3.3.2.1`
- `TC-3.3.2.2`
- `TC-3.3.2.3`
- `TC-3.3.3.1`
- `TC-3.3.3.2`
- `TC-3.3.4.1`
- `TC-3.3.4.2`
- `TC-3.3.5.1`
- `TC-3.3.6.1`
- `TC-3.3.7.1`
- `TC-3.4.1.1`
- `TC-3.4.1.2`
- `TC-3.4.1.3`
- `TC-3.4.2.1`
- `TC-3.4.2.2`
- `TC-3.4.3.1`
- `TC-3.4.3.2`
- `TC-3.4.4.1`
- `TC-3.4.5.1`
- `TC-3.4.6.1`
- `TC-3.4.7.1`
- `TC-3.4.7.2`
- `TC-3.5.1.1`
- `TC-3.5.1.2`
- `TC-3.5.2.1`
- `TC-3.5.2.2`
- `TC-3.5.2.3`
- `TC-3.5.3.1`
- `TC-3.5.3.2`
- `TC-3.5.4.1`
- `TC-3.5.4.2`
- `TC-3.5.5.1`
- `TC-3.5.5.2`
- `TC-3.5.6.1`
- `TC-3.5.6.2`
- `TC-3.5.6.3`
- `TC-3.5.7.1`

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

- `TC-3.1.1.1`
- `TC-3.1.1.2`
- `TC-3.1.1.3`
- `TC-3.1.1.4`
- `TC-3.1.1.5`
- `TC-3.1.1.6`
- `TC-3.1.1.7`
- `TC-3.1.5.1`
- `TC-3.1.5.2`
- `TC-3.1.5.3`
- `TC-3.1.6.1`
- `TC-3.1.6.2`
- `TC-3.1.7.1`
- `TC-3.1.7.2`
- `TC-3.2.1.1`
- `TC-3.2.1.2`
- `TC-3.2.1.3`
- `TC-3.2.1.4`
- `TC-3.2.1.5`
- `TC-3.2.1.6`
- `TC-3.2.2.1`
- `TC-3.2.2.2`
- `TC-3.2.3.1`
- `TC-3.2.3.2`
- `TC-3.2.3.3`
- `TC-3.2.4.1`
- `TC-3.2.4.2`
- `TC-3.2.5.1`
- `TC-3.2.5.2`
- `TC-3.2.6.1`
- `TC-3.2.6.2`
- `TC-3.2.6.3`
- `TC-3.2.7.1`
- `TC-3.2.7.2`
- `TC-3.2.8.1`
- `TC-3.2.9.1`
- `TC-3.2.9.2`
- `TC-3.2.10.1`
- `TC-3.2.11.1`
- `TC-3.2.11.2`
- `TC-3.2.11.3`
- `TC-3.2.12.1`
- `TC-3.2.12.2`
- `TC-3.2.12.3`
- `TC-3.2.12.4`
- `TC-3.2.13.1`
- `TC-3.2.13.2`
- `TC-3.2.13.3`
- `TC-3.2.13.4`
- `TC-3.2.13.5`
- `TC-3.2.13.6`
- `TC-3.2.13.7`
- `TC-3.2.13.8`
- `TC-3.2.14.1`
- `TC-3.2.14.2`
- `TC-3.2.14.3`
- `TC-3.2.14.4`
- `TC-3.3.2.1`
- `TC-3.3.2.2`
- `TC-3.3.2.3`
- `TC-3.3.3.1`
- `TC-3.3.3.2`
- `TC-3.3.4.1`
- `TC-3.3.4.2`
- `TC-3.3.5.1`
- `TC-3.3.6.1`
- `TC-3.3.7.1`
- `TC-3.4.1.1`
- `TC-3.4.1.2`
- `TC-3.4.1.3`
- `TC-3.4.2.1`
- `TC-3.4.2.2`
- `TC-3.4.3.1`
- `TC-3.4.3.2`
- `TC-3.4.4.1`
- `TC-3.4.5.1`
- `TC-3.4.6.1`
- `TC-3.4.7.1`
- `TC-3.4.7.2`
- `TC-3.5.1.1`
- `TC-3.5.1.2`
- `TC-3.5.2.1`
- `TC-3.5.2.2`
- `TC-3.5.2.3`
- `TC-3.5.3.1`
- `TC-3.5.3.2`
- `TC-3.5.4.1`
- `TC-3.5.4.2`
- `TC-3.5.5.1`
- `TC-3.5.5.2`
- `TC-3.5.6.1`
- `TC-3.5.6.2`
- `TC-3.5.6.3`
- `TC-3.5.7.1`

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
