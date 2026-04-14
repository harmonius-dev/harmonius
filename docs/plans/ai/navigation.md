---
children: []
dependencies: []
design_documents:
  - docs/design/ai/navigation.md
execution_mode: sequential
features:
  - F-1.1.22
  - F-1.5.1
  - F-1.9.1
  - F-4.6.3
  - F-7.1.1
  - F-7.1.2
  - F-7.1.3
  - F-7.1.4
  - F-7.1.5
  - F-7.1.6
  - F-7.1.7
  - F-7.1.8
  - F-7.1.9
  - F-7.1.10
  - F-7.1.11
  - F-7.1.12
  - F-7.1.13
  - F-7.1.14
  - F-7.1.15
  - F-14.3.1
  - F-14.3.3
  - F-15.1.4
id: PLAN-ai-navigation
name: Navigation
parent: null
progress_file: docs/plans/progress/PLAN-ai-navigation.md
requirements:
  - R-7.1.1
  - R-7.1.2
  - R-7.1.3
  - R-7.1.4
  - R-7.1.5
  - R-7.1.6
  - R-7.1.7
  - R-7.1.8
  - R-7.1.9
  - R-7.1.10
  - R-7.1.11
  - R-7.1.12
  - R-7.1.13
  - R-7.1.14
  - R-7.1.15
status: not_started
test_cases:
  - TC-7.1.1.1
  - TC-7.1.1.2
  - TC-7.1.1.3
  - TC-7.1.2.1
  - TC-7.1.2.2
  - TC-7.1.3.1
  - TC-7.1.3.2
  - TC-7.1.3.3
  - TC-7.1.4.1
  - TC-7.1.4.2
  - TC-7.1.5.1
  - TC-7.1.5.2
  - TC-7.1.6.1
  - TC-7.1.6.2
  - TC-7.1.7.1
  - TC-7.1.7.2
  - TC-7.1.7.3
  - TC-7.1.8.1
  - TC-7.1.8.2
  - TC-7.1.9.1
  - TC-7.1.9.2
  - TC-7.1.10.1
  - TC-7.1.10.2
  - TC-7.1.11.1
  - TC-7.1.11.2
  - TC-7.1.12.1
  - TC-7.1.12.2
  - TC-7.1.13.1
  - TC-7.1.13.2
  - TC-7.1.13.3
  - TC-7.1.14.1
  - TC-7.1.14.2
  - TC-7.1.14.3
  - TC-7.1.15.1
worktree_branch: plan/ai-navigation
---

# Navigation implementation plan

- Plan ID: `PLAN-ai-navigation`
- Progress file: [PLAN-ai-navigation.md](../progress/PLAN-ai-navigation.md)

## Source documents

- Design: [navigation.md](../../design/ai/navigation.md)
- Test cases: [navigation-test-cases.md](../../design/ai/navigation-test-cases.md)
- Progress: [PLAN-ai-navigation.md](../progress/PLAN-ai-navigation.md)

## Linked specification artifacts

### Features (`docs/features`)

- [crowd-simulation.md](../../features/ai/crowd-simulation.md) — covers `F-7.1.1`, `F-7.1.2`
- [navigation.md](../../features/ai/navigation.md) — covers `F-1.5.1`, `F-14.3.3`, `F-15.1.4`,
  `F-4.6.3`, `F-7.1.1`, `F-7.1.10`, `F-7.1.11`, `F-7.1.12`...
- [perception.md](../../features/ai/perception.md) — covers `F-7.1.1`
- [steering-avoidance.md](../../features/ai/steering-avoidance.md) — covers `F-7.1.3`
- [tactical-combat.md](../../features/ai/tactical-combat.md) — covers `F-7.1.1`
- [skeletal.md](../../features/animation/skeletal.md) — covers `F-1.9.1`
- [entity-component-system.md](../../features/core-runtime/entity-component-system.md) — covers
  `F-1.1.22`
- [events-and-messaging.md](../../features/core-runtime/events-and-messaging.md) — covers `F-1.1.22`
- [scene-and-transforms.md](../../features/core-runtime/scene-and-transforms.md) — covers
  `F-1.1.22`, `F-1.9.1`
- [spatial-indexing.md](../../features/core-runtime/spatial-indexing.md) — covers `F-1.1.22`,
  `F-1.9.1`
- [terrain.md](../../features/geometry/terrain.md) — covers `F-1.9.1`
- [collision-detection.md](../../features/physics/collision-detection.md) — covers `F-1.9.1`
- [destruction-and-fracture.md](../../features/physics/destruction-and-fracture.md) — covers
  `F-1.9.1`
- [spatial-queries.md](../../features/physics/spatial-queries.md) — covers `F-1.9.1`
- [threading-async.md](../../features/platform/threading-async.md) — covers `F-14.3.1`
- [event-logs.md](../../features/simulation/event-logs.md) — covers `F-1.9.1`
- [grids-volumes.md](../../features/simulation/grids-volumes.md) — covers `F-14.3.1`
- [spatial-awareness.md](../../features/simulation/spatial-awareness.md) — covers `F-1.9.1`
- [2d-games.md](../../features/ui/2d-games.md) — covers `F-1.9.1`
- [effect-graph.md](../../features/vfx/effect-graph.md) — covers `F-1.9.1`

### Requirements (`docs/requirements`)

- [navigation.md](../../requirements/ai/navigation.md) — covers `R-7.1.1`, `R-7.1.10`, `R-7.1.11`,
  `R-7.1.12`, `R-7.1.13`, `R-7.1.14`, `R-7.1.15`, `R-7.1.2`...

### User stories (`docs/user-stories`)

- [navigation.md](../../user-stories/ai/navigation.md) — covers `US-7.1.1`, `US-7.1.1.1`,
  `US-7.1.1.12`, `US-7.1.10`, `US-7.1.10.1`, `US-7.1.10.12`, `US-7.1.11`, `US-7.1.11.1`...

### Test case sources

- [navigation-test-cases.md](../../design/ai/navigation-test-cases.md)

### Gap closure decisions

- Normalized `US-7.1.1.12` to `US-7.1.1` using user-stories parent-ID mapping.
- Normalized `US-7.1.10.12` to `US-7.1.10` using user-stories parent-ID mapping.
- Normalized `US-7.1.11.12` to `US-7.1.11` using user-stories parent-ID mapping.
- Normalized `US-7.1.12.12` to `US-7.1.12` using user-stories parent-ID mapping.
- Normalized `US-7.1.13.12` to `US-7.1.13` using user-stories parent-ID mapping.
- Normalized `US-7.1.14.12` to `US-7.1.14` using user-stories parent-ID mapping.
- Normalized `US-7.1.14.3` to `US-7.1.14` using user-stories parent-ID mapping.
- Normalized `US-7.1.15.12` to `US-7.1.15` using user-stories parent-ID mapping.
- Normalized `US-7.1.2.12` to `US-7.1.2` using user-stories parent-ID mapping.
- Normalized `US-7.1.3.12` to `US-7.1.3` using user-stories parent-ID mapping.
- Normalized `US-7.1.3.4` to `US-7.1.3` using user-stories parent-ID mapping.
- Normalized `US-7.1.4.12` to `US-7.1.4` using user-stories parent-ID mapping.
- Normalized `US-7.1.5.11` to `US-7.1.5` using user-stories parent-ID mapping.
- Normalized `US-7.1.5.12` to `US-7.1.5` using user-stories parent-ID mapping.
- Normalized `US-7.1.6.12` to `US-7.1.6` using user-stories parent-ID mapping.
- Normalized `US-7.1.6.7` to `US-7.1.6` using user-stories parent-ID mapping.
- Normalized `US-7.1.7.12` to `US-7.1.7` using user-stories parent-ID mapping.
- Normalized `US-7.1.7.4` to `US-7.1.7` using user-stories parent-ID mapping.
- Normalized `US-7.1.7.5` to `US-7.1.7` using user-stories parent-ID mapping.
- Normalized `US-7.1.8.12` to `US-7.1.8` using user-stories parent-ID mapping.
- All IDs in this plan are mapped to specification artifacts.

## Traceability coverage

### Features

- `F-1.1.22`
- `F-1.5.1`
- `F-1.9.1`
- `F-4.6.3`
- `F-7.1.1`
- `F-7.1.2`
- `F-7.1.3`
- `F-7.1.4`
- `F-7.1.5`
- `F-7.1.6`
- `F-7.1.7`
- `F-7.1.8`
- `F-7.1.9`
- `F-7.1.10`
- `F-7.1.11`
- `F-7.1.12`
- `F-7.1.13`
- `F-7.1.14`
- `F-7.1.15`
- `F-14.3.1`
- `F-14.3.3`
- `F-15.1.4`

### Requirements

- `R-7.1.1`
- `R-7.1.2`
- `R-7.1.3`
- `R-7.1.4`
- `R-7.1.5`
- `R-7.1.6`
- `R-7.1.7`
- `R-7.1.8`
- `R-7.1.9`
- `R-7.1.10`
- `R-7.1.11`
- `R-7.1.12`
- `R-7.1.13`
- `R-7.1.14`
- `R-7.1.15`

### User stories

- `US-7.1.1`
- `US-7.1.1.1`
- `US-7.1.1.12`
- `US-7.1.2`
- `US-7.1.2.1`
- `US-7.1.2.12`
- `US-7.1.3`
- `US-7.1.3.1`
- `US-7.1.3.4`
- `US-7.1.3.12`
- `US-7.1.4`
- `US-7.1.4.1`
- `US-7.1.4.12`
- `US-7.1.5`
- `US-7.1.5.1`
- `US-7.1.5.11`
- `US-7.1.5.12`
- `US-7.1.6`
- `US-7.1.6.1`
- `US-7.1.6.7`
- `US-7.1.6.12`
- `US-7.1.7`
- `US-7.1.7.1`
- `US-7.1.7.4`
- `US-7.1.7.5`
- `US-7.1.7.12`
- `US-7.1.8`
- `US-7.1.8.1`
- `US-7.1.8.12`
- `US-7.1.9`
- `US-7.1.9.1`
- `US-7.1.9.3`
- `US-7.1.9.12`
- `US-7.1.10`
- `US-7.1.10.1`
- `US-7.1.10.12`
- `US-7.1.11`
- `US-7.1.11.1`
- `US-7.1.11.12`
- `US-7.1.12`
- `US-7.1.12.1`
- `US-7.1.12.12`
- `US-7.1.13`
- `US-7.1.13.1`
- `US-7.1.13.12`
- `US-7.1.14`
- `US-7.1.14.1`
- `US-7.1.14.3`
- `US-7.1.14.12`
- `US-7.1.15`
- `US-7.1.15.1`
- `US-7.1.15.12`

### Test cases

- `TC-7.1.1.1`
- `TC-7.1.1.2`
- `TC-7.1.1.3`
- `TC-7.1.2.1`
- `TC-7.1.2.2`
- `TC-7.1.3.1`
- `TC-7.1.3.2`
- `TC-7.1.3.3`
- `TC-7.1.4.1`
- `TC-7.1.4.2`
- `TC-7.1.5.1`
- `TC-7.1.5.2`
- `TC-7.1.6.1`
- `TC-7.1.6.2`
- `TC-7.1.7.1`
- `TC-7.1.7.2`
- `TC-7.1.7.3`
- `TC-7.1.8.1`
- `TC-7.1.8.2`
- `TC-7.1.9.1`
- `TC-7.1.9.2`
- `TC-7.1.10.1`
- `TC-7.1.10.2`
- `TC-7.1.11.1`
- `TC-7.1.11.2`
- `TC-7.1.12.1`
- `TC-7.1.12.2`
- `TC-7.1.13.1`
- `TC-7.1.13.2`
- `TC-7.1.13.3`
- `TC-7.1.14.1`
- `TC-7.1.14.2`
- `TC-7.1.14.3`
- `TC-7.1.15.1`

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

- `TC-7.1.1.1`
- `TC-7.1.1.2`
- `TC-7.1.1.3`
- `TC-7.1.2.1`
- `TC-7.1.2.2`
- `TC-7.1.3.1`
- `TC-7.1.3.2`
- `TC-7.1.3.3`
- `TC-7.1.4.1`
- `TC-7.1.4.2`
- `TC-7.1.5.1`
- `TC-7.1.5.2`
- `TC-7.1.6.1`
- `TC-7.1.6.2`
- `TC-7.1.7.1`
- `TC-7.1.7.2`
- `TC-7.1.7.3`
- `TC-7.1.8.1`
- `TC-7.1.8.2`
- `TC-7.1.9.1`
- `TC-7.1.9.2`
- `TC-7.1.10.1`
- `TC-7.1.10.2`
- `TC-7.1.11.1`
- `TC-7.1.11.2`
- `TC-7.1.12.1`
- `TC-7.1.12.2`
- `TC-7.1.13.1`
- `TC-7.1.13.2`
- `TC-7.1.13.3`
- `TC-7.1.14.1`
- `TC-7.1.14.2`
- `TC-7.1.14.3`
- `TC-7.1.15.1`

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

- `US-7.1.1.12` resolved via parent `US-7.1.1` in
  [navigation.md](../../user-stories/ai/navigation.md).
- `US-7.1.10.12` resolved via parent `US-7.1.10` in
  [navigation.md](../../user-stories/ai/navigation.md).
- `US-7.1.11.12` resolved via parent `US-7.1.11` in
  [navigation.md](../../user-stories/ai/navigation.md).
- `US-7.1.12.12` resolved via parent `US-7.1.12` in
  [navigation.md](../../user-stories/ai/navigation.md).
- `US-7.1.13.12` resolved via parent `US-7.1.13` in
  [navigation.md](../../user-stories/ai/navigation.md).
- `US-7.1.14.12` resolved via parent `US-7.1.14` in
  [navigation.md](../../user-stories/ai/navigation.md).
- `US-7.1.14.3` resolved via parent `US-7.1.14` in
  [navigation.md](../../user-stories/ai/navigation.md).
- `US-7.1.15.12` resolved via parent `US-7.1.15` in
  [navigation.md](../../user-stories/ai/navigation.md).
- `US-7.1.2.12` resolved via parent `US-7.1.2` in
  [navigation.md](../../user-stories/ai/navigation.md).
- `US-7.1.3.12` resolved via parent `US-7.1.3` in
  [navigation.md](../../user-stories/ai/navigation.md).
- `US-7.1.3.4` resolved via parent `US-7.1.3` in
  [navigation.md](../../user-stories/ai/navigation.md).
- `US-7.1.4.12` resolved via parent `US-7.1.4` in
  [navigation.md](../../user-stories/ai/navigation.md).
- `US-7.1.5.11` resolved via parent `US-7.1.5` in
  [navigation.md](../../user-stories/ai/navigation.md).
- `US-7.1.5.12` resolved via parent `US-7.1.5` in
  [navigation.md](../../user-stories/ai/navigation.md).
- `US-7.1.6.12` resolved via parent `US-7.1.6` in
  [navigation.md](../../user-stories/ai/navigation.md).
- `US-7.1.6.7` resolved via parent `US-7.1.6` in
  [navigation.md](../../user-stories/ai/navigation.md).
- `US-7.1.7.12` resolved via parent `US-7.1.7` in
  [navigation.md](../../user-stories/ai/navigation.md).
- `US-7.1.7.4` resolved via parent `US-7.1.7` in
  [navigation.md](../../user-stories/ai/navigation.md).
- `US-7.1.7.5` resolved via parent `US-7.1.7` in
  [navigation.md](../../user-stories/ai/navigation.md).
- `US-7.1.8.12` resolved via parent `US-7.1.8` in
  [navigation.md](../../user-stories/ai/navigation.md).
- `US-7.1.9.12` resolved via parent `US-7.1.9` in
  [navigation.md](../../user-stories/ai/navigation.md).
- `US-7.1.9.3` resolved via parent `US-7.1.9` in
  [navigation.md](../../user-stories/ai/navigation.md).

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
