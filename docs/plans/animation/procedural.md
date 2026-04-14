---
children: []
dependencies: []
design_documents:
  - docs/design/animation/procedural.md
execution_mode: sequential
features:
  - F-1.1.22
  - F-1.1.32
  - F-1.3.1
  - F-1.9.1
  - F-1.9.4
  - F-1.9.5
  - F-4.1.1
  - F-4.1.8
  - F-4.3.1
  - F-4.3.5
  - F-4.7.1
  - F-4.7.5
  - F-6.2.1
  - F-9.1.1
  - F-9.1.3
  - F-9.1.6
  - F-9.3.1
  - F-9.3.2
  - F-9.3.3
  - F-9.3.4
  - F-9.3.5
  - F-9.3.6
  - F-9.3.7
  - F-9.3.8
  - F-9.3.9
  - F-9.3.10
  - F-9.3.11
  - F-9.5.1
  - F-9.5.2
  - F-9.5.3
  - F-9.5.4
  - F-9.5.5
  - F-9.5.6
  - F-9.6.1
  - F-9.6.2
  - F-9.6.3
  - F-9.6.4
  - F-13.2.1
  - F-13.16.3
  - F-14.3.1
id: PLAN-animation-procedural
name: Procedural
parent: null
progress_file: docs/plans/progress/PLAN-animation-procedural.md
requirements:
  - R-9.3.1
  - R-9.3.2
  - R-9.3.3
  - R-9.3.4
  - R-9.3.5
  - R-9.3.6
  - R-9.3.7
  - R-9.3.8
  - R-9.3.9
  - R-9.3.10
  - R-9.3.11
  - R-9.5.1
  - R-9.5.2
  - R-9.5.3
  - R-9.5.4
  - R-9.5.5
  - R-9.5.6
  - R-9.6.1
  - R-9.6.2
  - R-9.6.3
  - R-9.6.4
status: not_started
test_cases:
  - TC-9.3.1.1
  - TC-9.3.1.2
  - TC-9.3.1.3
  - TC-9.3.1.4
  - TC-9.3.2.1
  - TC-9.3.2.2
  - TC-9.3.2.3
  - TC-9.3.3.1
  - TC-9.3.3.2
  - TC-9.3.3.3
  - TC-9.3.4.1
  - TC-9.3.4.2
  - TC-9.3.4.3
  - TC-9.3.4.4
  - TC-9.3.5.1
  - TC-9.3.5.2
  - TC-9.3.5.3
  - TC-9.3.5.4
  - TC-9.3.6.1
  - TC-9.3.7.1
  - TC-9.3.7.2
  - TC-9.3.7.3
  - TC-9.3.8.1
  - TC-9.3.8.2
  - TC-9.3.8.3
  - TC-9.3.8.4
  - TC-9.3.9.1
  - TC-9.3.9.2
  - TC-9.3.9.3
  - TC-9.3.10.1
  - TC-9.3.10.2
  - TC-9.3.10.3
  - TC-9.3.10.4
  - TC-9.3.11.1
  - TC-9.3.11.2
  - TC-9.3.11.3
  - TC-9.5.1.1
  - TC-9.5.2.1
  - TC-9.5.3.1
  - TC-9.5.4.1
  - TC-9.5.5.1
  - TC-9.5.6.1
  - TC-9.6.1.1
  - TC-9.6.2.1
  - TC-9.6.3.1
  - TC-9.6.4.1
worktree_branch: plan/animation-procedural
---

# Procedural implementation plan

- Plan ID: `PLAN-animation-procedural`
- Progress file: [PLAN-animation-procedural.md](../progress/PLAN-animation-procedural.md)

## Source documents

- Design: [procedural.md](../../design/animation/procedural.md)
- Test cases: [procedural-test-cases.md](../../design/animation/procedural-test-cases.md)
- Progress: [PLAN-animation-procedural.md](../progress/PLAN-animation-procedural.md)

## Linked specification artifacts

### Features (`docs/features`)

- [cloth-hair.md](../../features/animation/cloth-hair.md) — covers `F-4.7.1`, `F-4.7.5`, `F-9.1.1`,
  `F-9.5.1`, `F-9.5.2`, `F-9.5.3`, `F-9.5.4`, `F-9.5.5`...
- [first-person.md](../../features/animation/first-person.md) — covers `F-13.16.3`, `F-13.2.1`,
  `F-4.1.8`, `F-6.2.1`, `F-9.6.1`, `F-9.6.2`, `F-9.6.3`, `F-9.6.4`
- [morph.md](../../features/animation/morph.md) — covers `F-9.1.1`
- [motion-matching.md](../../features/animation/motion-matching.md) — covers `F-4.1.8`, `F-6.2.1`,
  `F-9.1.1`, `F-9.3.1`, `F-9.3.5`
- [procedural.md](../../features/animation/procedural.md) — covers `F-1.1.32`, `F-1.9.4`, `F-4.1.1`,
  `F-4.3.1`, `F-4.3.5`, `F-9.1.1`, `F-9.1.3`, `F-9.1.6`...
- [skeletal.md](../../features/animation/skeletal.md) — covers `F-1.9.1`, `F-4.3.5`, `F-9.1.1`,
  `F-9.1.3`, `F-9.1.6`
- [state-machine.md](../../features/animation/state-machine.md) — covers `F-9.1.3`, `F-9.1.6`,
  `F-9.3.1`, `F-9.3.6`
- [entity-component-system.md](../../features/core-runtime/entity-component-system.md) — covers
  `F-1.1.22`, `F-1.3.1`
- [events-and-messaging.md](../../features/core-runtime/events-and-messaging.md) — covers
  `F-1.1.22`, `F-1.3.1`
- [plugin-system.md](../../features/core-runtime/plugin-system.md) — covers `F-1.3.1`
- [reflection-and-type-system.md](../../features/core-runtime/reflection-and-type-system.md) —
  covers `F-1.3.1`
- [scene-and-transforms.md](../../features/core-runtime/scene-and-transforms.md) — covers `F-1.1.22`
- [spatial-indexing.md](../../features/core-runtime/spatial-indexing.md) — covers `F-1.1.22`,
  `F-1.9.5`
- [gameplay-databases.md](../../features/game-framework/gameplay-databases.md) — covers `F-1.3.1`
- [gameplay-primitives.md](../../features/game-framework/gameplay-primitives.md) — covers `F-1.3.1`
- [save-system.md](../../features/game-framework/save-system.md) — covers `F-1.3.1`
- [threading-async.md](../../features/platform/threading-async.md) — covers `F-14.3.1`
- [grids-volumes.md](../../features/simulation/grids-volumes.md) — covers `F-14.3.1`
- [documentation.md](../../features/tools/documentation.md) — covers `F-1.3.1`
- [logic-graph.md](../../features/tools/logic-graph.md) — covers `F-1.3.1`

### Requirements (`docs/requirements`)

- [cloth-hair.md](../../requirements/animation/cloth-hair.md) — covers `R-9.5.1`, `R-9.5.2`,
  `R-9.5.3`, `R-9.5.4`, `R-9.5.5`, `R-9.5.6`
- [first-person.md](../../requirements/animation/first-person.md) — covers `R-9.6.1`, `R-9.6.2`,
  `R-9.6.3`, `R-9.6.4`
- [procedural.md](../../requirements/animation/procedural.md) — covers `R-9.3.1`, `R-9.3.10`,
  `R-9.3.11`, `R-9.3.2`, `R-9.3.3`, `R-9.3.4`, `R-9.3.5`, `R-9.3.6`...

### User stories (`docs/user-stories`)

- [cloth-hair.md](../../user-stories/animation/cloth-hair.md) — covers `US-9.5.1.1`, `US-9.5.1.2`,
  `US-9.5.1.3`, `US-9.5.2.1`, `US-9.5.2.2`, `US-9.5.2.3`, `US-9.5.3.1`, `US-9.5.3.2`...
- [first-person.md](../../user-stories/animation/first-person.md) — covers `US-9.6.1.1`,
  `US-9.6.1.2`, `US-9.6.1.3`, `US-9.6.2.1`, `US-9.6.2.2`, `US-9.6.2.3`, `US-9.6.3.1`,
  `US-9.6.3.3`...
- [procedural.md](../../user-stories/animation/procedural.md) — covers `US-9.3.1.1`, `US-9.3.1.2`,
  `US-9.3.1.3`, `US-9.3.10.1`, `US-9.3.10.3`, `US-9.3.11.1`, `US-9.3.11.3`, `US-9.3.2.1`...

### Test case sources

- [procedural-test-cases.md](../../design/animation/procedural-test-cases.md)

### Gap closure decisions

- No normalization was required for this plan.
- All IDs in this plan are mapped to specification artifacts.

## Traceability coverage

### Features

- `F-1.1.22`
- `F-1.1.32`
- `F-1.3.1`
- `F-1.9.1`
- `F-1.9.4`
- `F-1.9.5`
- `F-4.1.1`
- `F-4.1.8`
- `F-4.3.1`
- `F-4.3.5`
- `F-4.7.1`
- `F-4.7.5`
- `F-6.2.1`
- `F-9.1.1`
- `F-9.1.3`
- `F-9.1.6`
- `F-9.3.1`
- `F-9.3.2`
- `F-9.3.3`
- `F-9.3.4`
- `F-9.3.5`
- `F-9.3.6`
- `F-9.3.7`
- `F-9.3.8`
- `F-9.3.9`
- `F-9.3.10`
- `F-9.3.11`
- `F-9.5.1`
- `F-9.5.2`
- `F-9.5.3`
- `F-9.5.4`
- `F-9.5.5`
- `F-9.5.6`
- `F-9.6.1`
- `F-9.6.2`
- `F-9.6.3`
- `F-9.6.4`
- `F-13.2.1`
- `F-13.16.3`
- `F-14.3.1`

### Requirements

- `R-9.3.1`
- `R-9.3.2`
- `R-9.3.3`
- `R-9.3.4`
- `R-9.3.5`
- `R-9.3.6`
- `R-9.3.7`
- `R-9.3.8`
- `R-9.3.9`
- `R-9.3.10`
- `R-9.3.11`
- `R-9.5.1`
- `R-9.5.2`
- `R-9.5.3`
- `R-9.5.4`
- `R-9.5.5`
- `R-9.5.6`
- `R-9.6.1`
- `R-9.6.2`
- `R-9.6.3`
- `R-9.6.4`

### User stories

- `US-9.3.1.1`
- `US-9.3.1.2`
- `US-9.3.1.3`
- `US-9.3.2.1`
- `US-9.3.2.2`
- `US-9.3.3.1`
- `US-9.3.3.2`
- `US-9.3.4.1`
- `US-9.3.4.2`
- `US-9.3.4.3`
- `US-9.3.5.1`
- `US-9.3.5.2`
- `US-9.3.6.1`
- `US-9.3.6.2`
- `US-9.3.7.1`
- `US-9.3.7.2`
- `US-9.3.8.1`
- `US-9.3.8.2`
- `US-9.3.8.3`
- `US-9.3.9.1`
- `US-9.3.9.2`
- `US-9.3.10.1`
- `US-9.3.10.3`
- `US-9.3.11.1`
- `US-9.3.11.3`
- `US-9.5.1.1`
- `US-9.5.1.2`
- `US-9.5.1.3`
- `US-9.5.2.1`
- `US-9.5.2.2`
- `US-9.5.2.3`
- `US-9.5.3.1`
- `US-9.5.3.2`
- `US-9.5.4.1`
- `US-9.5.4.2`
- `US-9.5.5.1`
- `US-9.5.5.2`
- `US-9.5.6.1`
- `US-9.5.6.2`
- `US-9.5.6.3`
- `US-9.6.1.1`
- `US-9.6.1.2`
- `US-9.6.1.3`
- `US-9.6.2.1`
- `US-9.6.2.2`
- `US-9.6.2.3`
- `US-9.6.3.1`
- `US-9.6.3.3`
- `US-9.6.4.1`
- `US-9.6.4.4`

### Test cases

- `TC-9.3.1.1`
- `TC-9.3.1.2`
- `TC-9.3.1.3`
- `TC-9.3.1.4`
- `TC-9.3.2.1`
- `TC-9.3.2.2`
- `TC-9.3.2.3`
- `TC-9.3.3.1`
- `TC-9.3.3.2`
- `TC-9.3.3.3`
- `TC-9.3.4.1`
- `TC-9.3.4.2`
- `TC-9.3.4.3`
- `TC-9.3.4.4`
- `TC-9.3.5.1`
- `TC-9.3.5.2`
- `TC-9.3.5.3`
- `TC-9.3.5.4`
- `TC-9.3.6.1`
- `TC-9.3.7.1`
- `TC-9.3.7.2`
- `TC-9.3.7.3`
- `TC-9.3.8.1`
- `TC-9.3.8.2`
- `TC-9.3.8.3`
- `TC-9.3.8.4`
- `TC-9.3.9.1`
- `TC-9.3.9.2`
- `TC-9.3.9.3`
- `TC-9.3.10.1`
- `TC-9.3.10.2`
- `TC-9.3.10.3`
- `TC-9.3.10.4`
- `TC-9.3.11.1`
- `TC-9.3.11.2`
- `TC-9.3.11.3`
- `TC-9.5.1.1`
- `TC-9.5.2.1`
- `TC-9.5.3.1`
- `TC-9.5.4.1`
- `TC-9.5.5.1`
- `TC-9.5.6.1`
- `TC-9.6.1.1`
- `TC-9.6.2.1`
- `TC-9.6.3.1`
- `TC-9.6.4.1`

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

- `TC-9.3.1.1`
- `TC-9.3.1.2`
- `TC-9.3.1.3`
- `TC-9.3.1.4`
- `TC-9.3.2.1`
- `TC-9.3.2.2`
- `TC-9.3.2.3`
- `TC-9.3.3.1`
- `TC-9.3.3.2`
- `TC-9.3.3.3`
- `TC-9.3.4.1`
- `TC-9.3.4.2`
- `TC-9.3.4.3`
- `TC-9.3.4.4`
- `TC-9.3.5.1`
- `TC-9.3.5.2`
- `TC-9.3.5.3`
- `TC-9.3.5.4`
- `TC-9.3.6.1`
- `TC-9.3.7.1`
- `TC-9.3.7.2`
- `TC-9.3.7.3`
- `TC-9.3.8.1`
- `TC-9.3.8.2`
- `TC-9.3.8.3`
- `TC-9.3.8.4`
- `TC-9.3.9.1`
- `TC-9.3.9.2`
- `TC-9.3.9.3`
- `TC-9.3.10.1`
- `TC-9.3.10.2`
- `TC-9.3.10.3`
- `TC-9.3.10.4`
- `TC-9.3.11.1`
- `TC-9.3.11.2`
- `TC-9.3.11.3`
- `TC-9.5.1.1`
- `TC-9.5.2.1`
- `TC-9.5.3.1`
- `TC-9.5.4.1`
- `TC-9.5.5.1`
- `TC-9.5.6.1`
- `TC-9.6.1.1`
- `TC-9.6.2.1`
- `TC-9.6.3.1`
- `TC-9.6.4.1`

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
