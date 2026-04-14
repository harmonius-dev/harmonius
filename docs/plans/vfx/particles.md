---
children: []
dependencies: []
design_documents:
  - docs/design/vfx/particles.md
execution_mode: sequential
features:
  - F-1.1.1
  - F-1.1.22
  - F-1.1.30
  - F-1.2.4
  - F-1.9.1
  - F-2.1.1
  - F-2.1.2
  - F-2.1.7
  - F-2.2.1
  - F-2.2.6
  - F-4.7.5
  - F-11.1.1
  - F-11.1.2
  - F-11.1.3
  - F-11.1.4
  - F-11.1.5
  - F-11.1.6
  - F-11.1.7
  - F-11.6.1
  - F-14.3.1
id: PLAN-vfx-particles
name: Particles
parent: null
progress_file: docs/plans/progress/PLAN-vfx-particles.md
requirements:
  - R-11.1.1
  - R-11.1.2
  - R-11.1.3
  - R-11.1.4
  - R-11.1.5
  - R-11.1.6
  - R-11.1.7
status: not_started
test_cases:
  - TC-11.1.1.1
  - TC-11.1.1.2
  - TC-11.1.1.3
  - TC-11.1.1.4
  - TC-11.1.1.5
  - TC-11.1.1.6
  - TC-11.1.1.7
  - TC-11.1.1.8
  - TC-11.1.1.9
  - TC-11.1.2.1
  - TC-11.1.2.2
  - TC-11.1.2.3
  - TC-11.1.2.4
  - TC-11.1.2.5
  - TC-11.1.2.6
  - TC-11.1.2.7
  - TC-11.1.2.8
  - TC-11.1.3.1
  - TC-11.1.3.2
  - TC-11.1.3.3
  - TC-11.1.3.4
  - TC-11.1.3.5
  - TC-11.1.3.6
  - TC-11.1.4.1
  - TC-11.1.4.2
  - TC-11.1.4.3
  - TC-11.1.4.4
  - TC-11.1.4.5
  - TC-11.1.5.1
  - TC-11.1.5.2
  - TC-11.1.5.3
  - TC-11.1.5.4
  - TC-11.1.6.1
  - TC-11.1.6.2
  - TC-11.1.7.1
worktree_branch: plan/vfx-particles
---

# Particles implementation plan

- Plan ID: `PLAN-vfx-particles`
- Progress file: [PLAN-vfx-particles.md](../progress/PLAN-vfx-particles.md)

## Source documents

- Design: [particles.md](../../design/vfx/particles.md)
- Test cases: [particles-test-cases.md](../../design/vfx/particles-test-cases.md)
- Progress: [PLAN-vfx-particles.md](../progress/PLAN-vfx-particles.md)

## Linked specification artifacts

### Features (`docs/features`)

- [perception.md](../../features/ai/perception.md) — covers `F-4.7.5`
- [cloth-hair.md](../../features/animation/cloth-hair.md) — covers `F-4.7.5`
- [content-plugins.md](../../features/content-pipeline/content-plugins.md) — covers `F-1.1.1`
- [entity-component-system.md](../../features/core-runtime/entity-component-system.md) — covers
  `F-1.1.1`, `F-1.1.22`
- [events-and-messaging.md](../../features/core-runtime/events-and-messaging.md) — covers `F-1.1.22`
- [game-loop.md](../../features/core-runtime/game-loop.md) — covers `F-1.2.4`
- [scene-and-transforms.md](../../features/core-runtime/scene-and-transforms.md) — covers
  `F-1.1.22`, `F-1.2.4`
- [spatial-indexing.md](../../features/core-runtime/spatial-indexing.md) — covers `F-1.1.1`,
  `F-1.1.22`
- [attributes-effects.md](../../features/data-systems/attributes-effects.md) — covers `F-1.1.1`
- [containers-slots.md](../../features/data-systems/containers-slots.md) — covers `F-1.1.1`
- [data-tables.md](../../features/data-systems/data-tables.md) — covers `F-1.1.1`
- [camera-system.md](../../features/game-framework/camera-system.md) — covers `F-1.1.1`
- [character-customization.md](../../features/game-framework/character-customization.md) — covers
  `F-1.1.1`
- [game-modes-misc.md](../../features/game-framework/game-modes-misc.md) — covers `F-1.1.1`
- [gameplay-primitives.md](../../features/game-framework/gameplay-primitives.md) — covers `F-1.1.1`
- [inventory.md](../../features/game-framework/inventory.md) — covers `F-1.1.1`
- [save-system.md](../../features/game-framework/save-system.md) — covers `F-1.1.1`
- [scripting.md](../../features/game-framework/scripting.md) — covers `F-1.1.1`
- [selection-system.md](../../features/game-framework/selection-system.md) — covers `F-1.1.1`
- [social.md](../../features/game-framework/social.md) — covers `F-1.1.1`
- [weapon-system.md](../../features/game-framework/weapon-system.md) — covers `F-1.1.1`
- [foliage.md](../../features/geometry/foliage.md) — covers `F-4.7.5`
- [procedural-generation.md](../../features/geometry/procedural-generation.md) — covers `F-1.1.1`,
  `F-2.1.1`
- [session-management.md](../../features/networking/session-management.md) — covers `F-1.1.1`
- [collision-detection.md](../../features/physics/collision-detection.md) — covers `F-1.1.1`
- [constraints-and-joints.md](../../features/physics/constraints-and-joints.md) — covers `F-1.1.1`
- [destruction-and-fracture.md](../../features/physics/destruction-and-fracture.md) — covers
  `F-1.1.1`
- [fluid-simulation.md](../../features/physics/fluid-simulation.md) — covers `F-1.1.1`
- [rigid-body-dynamics.md](../../features/physics/rigid-body-dynamics.md) — covers `F-1.1.1`
- [soft-body-and-cloth.md](../../features/physics/soft-body-and-cloth.md) — covers `F-1.1.1`,
  `F-4.7.5`
- [vehicle-physics.md](../../features/physics/vehicle-physics.md) — covers `F-1.1.1`
- [platform-services.md](../../features/platform/platform-services.md) — covers `F-2.1.1`
- [threading-async.md](../../features/platform/threading-async.md) — covers `F-14.3.1`
- [advanced-rendering.md](../../features/rendering/advanced-rendering.md) — covers `F-2.1.1`
- [core-rendering.md](../../features/rendering/core-rendering.md) — covers `F-2.1.1`, `F-2.2.1`
- [first-person-rendering.md](../../features/rendering/first-person-rendering.md) — covers `F-2.2.1`
- [gpu-abstraction-layer.md](../../features/rendering/gpu-abstraction-layer.md) — covers `F-2.1.1`,
  `F-2.1.2`, `F-2.1.7`
- [render-graph.md](../../features/rendering/render-graph.md) — covers `F-2.1.2`, `F-2.1.7`,
  `F-2.2.1`, `F-2.2.6`
- [scene-rendering-pipeline.md](../../features/rendering/scene-rendering-pipeline.md) — covers
  `F-2.1.7`, `F-2.2.1`
- [event-logs.md](../../features/simulation/event-logs.md) — covers `F-1.1.1`
- [grids-volumes.md](../../features/simulation/grids-volumes.md) — covers `F-1.1.1`, `F-14.3.1`
- [spatial-awareness.md](../../features/simulation/spatial-awareness.md) — covers `F-1.1.1`
- [timelines.md](../../features/simulation/timelines.md) — covers `F-1.1.1`
- [asset-store.md](../../features/tools/asset-store.md) — covers `F-2.1.1`
- [editor-framework.md](../../features/tools/editor-framework.md) — covers `F-1.1.1`
- [logic-graph.md](../../features/tools/logic-graph.md) — covers `F-2.1.1`
- [material-editor.md](../../features/tools/material-editor.md) — covers `F-2.1.1`
- [profiling-tools.md](../../features/tools/profiling-tools.md) — covers `F-2.1.1`
- [remote-editing.md](../../features/tools/remote-editing.md) — covers `F-2.1.1`
- [shared-cache.md](../../features/tools/shared-cache.md) — covers `F-2.1.1`
- [specialized-editors.md](../../features/tools/specialized-editors.md) — covers `F-1.1.1`
- [2d-games.md](../../features/ui/2d-games.md) — covers `F-1.1.1`
- [ui-rendering.md](../../features/ui/ui-rendering.md) — covers `F-2.2.1`
- [destruction.md](../../features/vfx/destruction.md) — covers `F-11.1.1`, `F-11.1.2`, `F-11.1.3`,
  `F-11.1.6`
- [effect-graph.md](../../features/vfx/effect-graph.md) — covers `F-1.1.30`, `F-1.9.1`, `F-11.1.1`,
  `F-11.1.2`, `F-11.6.1`
- [particles.md](../../features/vfx/particles.md) — covers `F-11.1.1`, `F-11.1.2`, `F-11.1.3`,
  `F-11.1.4`, `F-11.1.5`, `F-11.1.6`, `F-11.1.7`
- [weather-environmental.md](../../features/vfx/weather-environmental.md) — covers `F-11.1.1`

### Requirements (`docs/requirements`)

- [particles.md](../../requirements/vfx/particles.md) — covers `R-11.1.1`, `R-11.1.2`, `R-11.1.3`,
  `R-11.1.4`, `R-11.1.5`, `R-11.1.6`, `R-11.1.7`

### User stories (`docs/user-stories`)

- [particles.md](../../user-stories/vfx/particles.md) — covers `US-11.1.1.1`, `US-11.1.1.2`,
  `US-11.1.1.3`, `US-11.1.2.1`, `US-11.1.2.2`, `US-11.1.2.3`, `US-11.1.3.1`, `US-11.1.3.2`...

### Test case sources

- [particles-test-cases.md](../../design/vfx/particles-test-cases.md)

### Gap closure decisions

- No normalization was required for this plan.
- All IDs in this plan are mapped to specification artifacts.

## Traceability coverage

### Features

- `F-1.1.1`
- `F-1.1.22`
- `F-1.1.30`
- `F-1.2.4`
- `F-1.9.1`
- `F-2.1.1`
- `F-2.1.2`
- `F-2.1.7`
- `F-2.2.1`
- `F-2.2.6`
- `F-4.7.5`
- `F-11.1.1`
- `F-11.1.2`
- `F-11.1.3`
- `F-11.1.4`
- `F-11.1.5`
- `F-11.1.6`
- `F-11.1.7`
- `F-11.6.1`
- `F-14.3.1`

### Requirements

- `R-11.1.1`
- `R-11.1.2`
- `R-11.1.3`
- `R-11.1.4`
- `R-11.1.5`
- `R-11.1.6`
- `R-11.1.7`

### User stories

- `US-11.1.1.1`
- `US-11.1.1.2`
- `US-11.1.1.3`
- `US-11.1.2.1`
- `US-11.1.2.2`
- `US-11.1.2.3`
- `US-11.1.3.1`
- `US-11.1.3.2`
- `US-11.1.3.3`
- `US-11.1.4.1`
- `US-11.1.4.2`
- `US-11.1.4.3`
- `US-11.1.5.1`
- `US-11.1.5.2`
- `US-11.1.6.1`
- `US-11.1.6.2`
- `US-11.1.7.1`
- `US-11.1.7.2`

### Test cases

- `TC-11.1.1.1`
- `TC-11.1.1.2`
- `TC-11.1.1.3`
- `TC-11.1.1.4`
- `TC-11.1.1.5`
- `TC-11.1.1.6`
- `TC-11.1.1.7`
- `TC-11.1.1.8`
- `TC-11.1.1.9`
- `TC-11.1.2.1`
- `TC-11.1.2.2`
- `TC-11.1.2.3`
- `TC-11.1.2.4`
- `TC-11.1.2.5`
- `TC-11.1.2.6`
- `TC-11.1.2.7`
- `TC-11.1.2.8`
- `TC-11.1.3.1`
- `TC-11.1.3.2`
- `TC-11.1.3.3`
- `TC-11.1.3.4`
- `TC-11.1.3.5`
- `TC-11.1.3.6`
- `TC-11.1.4.1`
- `TC-11.1.4.2`
- `TC-11.1.4.3`
- `TC-11.1.4.4`
- `TC-11.1.4.5`
- `TC-11.1.5.1`
- `TC-11.1.5.2`
- `TC-11.1.5.3`
- `TC-11.1.5.4`
- `TC-11.1.6.1`
- `TC-11.1.6.2`
- `TC-11.1.7.1`

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

- `TC-11.1.1.1`
- `TC-11.1.1.2`
- `TC-11.1.1.3`
- `TC-11.1.1.4`
- `TC-11.1.1.5`
- `TC-11.1.1.6`
- `TC-11.1.1.7`
- `TC-11.1.1.8`
- `TC-11.1.1.9`
- `TC-11.1.2.1`
- `TC-11.1.2.2`
- `TC-11.1.2.3`
- `TC-11.1.2.4`
- `TC-11.1.2.5`
- `TC-11.1.2.6`
- `TC-11.1.2.7`
- `TC-11.1.2.8`
- `TC-11.1.3.1`
- `TC-11.1.3.2`
- `TC-11.1.3.3`
- `TC-11.1.3.4`
- `TC-11.1.3.5`
- `TC-11.1.3.6`
- `TC-11.1.4.1`
- `TC-11.1.4.2`
- `TC-11.1.4.3`
- `TC-11.1.4.4`
- `TC-11.1.4.5`
- `TC-11.1.5.1`
- `TC-11.1.5.2`
- `TC-11.1.5.3`
- `TC-11.1.5.4`
- `TC-11.1.6.1`
- `TC-11.1.6.2`
- `TC-11.1.7.1`

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
