---
children: []
dependencies: []
design_documents:
  - docs/design/physics/foundation.md
execution_mode: sequential
features:
  - F-1.1.34
  - F-1.1.35
  - F-1.9.1
  - F-4.1.1
  - F-4.1.2
  - F-4.1.3
  - F-4.1.4
  - F-4.1.5
  - F-4.1.6
  - F-4.1.7
  - F-4.1.8
  - F-4.1.9
  - F-4.1.10
  - F-4.2.1
  - F-4.2.2
  - F-4.2.3
  - F-4.2.4
  - F-4.2.5
  - F-4.2.6
  - F-4.2.7
  - F-4.2.8
  - F-4.2.9
  - F-4.3.4
  - F-4.7.5
  - F-9.3.5
id: PLAN-physics-foundation
name: Foundation
parent: null
progress_file: docs/plans/progress/PLAN-physics-foundation.md
requirements:
  - R-4.1.1
  - R-4.1.2
  - R-4.1.3
  - R-4.1.4
  - R-4.1.5
  - R-4.1.6
  - R-4.1.7
  - R-4.1.8
  - R-4.1.9
  - R-4.1.10
  - R-4.2.1
  - R-4.2.2
  - R-4.2.3
  - R-4.2.4
  - R-4.2.5
  - R-4.2.6
  - R-4.2.7
  - R-4.2.8
  - R-4.2.9
status: not_started
test_cases:
  - TC-4.1.1.1
  - TC-4.1.1.2
  - TC-4.1.1.3
  - TC-4.1.2.1
  - TC-4.1.3.1
  - TC-4.1.3.2
  - TC-4.1.3.3
  - TC-4.1.4.1
  - TC-4.1.4.2
  - TC-4.1.5.1
  - TC-4.1.5.2
  - TC-4.1.5.3
  - TC-4.1.6.1
  - TC-4.1.6.2
  - TC-4.1.6.3
  - TC-4.1.8.1
  - TC-4.1.8.2
  - TC-4.1.8.3
  - TC-4.1.9.1
  - TC-4.1.10.1
  - TC-4.2.1.1
  - TC-4.2.2.1
  - TC-4.2.2.2
  - TC-4.2.3.1
  - TC-4.2.4.1
  - TC-4.2.5.1
  - TC-4.2.6.1
  - TC-4.2.6.2
  - TC-4.2.7.1
  - TC-4.2.7.2
  - TC-4.2.8.1
  - TC-4.2.8.2
  - TC-4.2.9.1
worktree_branch: plan/physics-foundation
---

# Foundation implementation plan

- Plan ID: `PLAN-physics-foundation`
- Progress file: [PLAN-physics-foundation.md](../progress/PLAN-physics-foundation.md)

## Source documents

- Design: [foundation.md](../../design/physics/foundation.md)
- Test cases: [foundation-test-cases.md](../../design/physics/foundation-test-cases.md)
- Progress: [PLAN-physics-foundation.md](../progress/PLAN-physics-foundation.md)

## Linked specification artifacts

### Features (`docs/features`)

- [collision-detection.md](../../features/physics/collision-detection.md) — covers `F-1.9.1`,
  `F-4.2.1`, `F-4.2.2`, `F-4.2.3`, `F-4.2.4`, `F-4.2.5`, `F-4.2.6`, `F-4.2.7`...
- [constraints-and-joints.md](../../features/physics/constraints-and-joints.md) — covers `F-4.1.1`,
  `F-4.1.3`, `F-4.1.5`, `F-4.2.2`, `F-4.2.7`, `F-4.3.4`
- [destruction-and-fracture.md](../../features/physics/destruction-and-fracture.md) — covers
  `F-1.9.1`, `F-4.1.1`, `F-4.1.6`, `F-4.2.3`, `F-4.2.4`, `F-4.2.7`, `F-4.3.4`
- [fluid-simulation.md](../../features/physics/fluid-simulation.md) — covers `F-4.1.1`
- [rigid-body-dynamics.md](../../features/physics/rigid-body-dynamics.md) — covers `F-1.1.34`,
  `F-1.1.35`, `F-4.1.1`, `F-4.1.10`, `F-4.1.2`, `F-4.1.3`, `F-4.1.4`, `F-4.1.5`...
- [soft-body-and-cloth.md](../../features/physics/soft-body-and-cloth.md) — covers `F-4.1.3`,
  `F-4.7.5`
- [spatial-queries.md](../../features/physics/spatial-queries.md) — covers `F-1.9.1`, `F-4.2.1`,
  `F-4.2.2`, `F-4.2.3`, `F-4.2.6`
- [vehicle-physics.md](../../features/physics/vehicle-physics.md) — covers `F-4.1.1`, `F-4.1.3`

### Requirements (`docs/requirements`)

- [collision-detection.md](../../requirements/physics/collision-detection.md) — covers `R-4.2.1`,
  `R-4.2.2`, `R-4.2.3`, `R-4.2.4`, `R-4.2.5`, `R-4.2.6`, `R-4.2.7`, `R-4.2.8`...
- [rigid-body-dynamics.md](../../requirements/physics/rigid-body-dynamics.md) — covers `R-4.1.1`,
  `R-4.1.10`, `R-4.1.2`, `R-4.1.3`, `R-4.1.4`, `R-4.1.5`, `R-4.1.6`, `R-4.1.7`...

### User stories (`docs/user-stories`)

- [rigid-body-dynamics.md](../../user-stories/physics/rigid-body-dynamics.md) — covers `US-4.1.1`,
  `US-4.1.1.9`, `US-4.1.4`, `US-4.1.4.5`, `US-4.1.5`, `US-4.1.5.5`, `US-4.1.6`, `US-4.1.6.5`

### Test case sources

- [foundation-test-cases.md](../../design/physics/foundation-test-cases.md)

### Gap closure decisions

- Normalized `US-4.1.1.9` to `US-4.1.1` using user-stories parent-ID mapping.
- Normalized `US-4.1.4.5` to `US-4.1.4` using user-stories parent-ID mapping.
- Normalized `US-4.1.5.5` to `US-4.1.5` using user-stories parent-ID mapping.
- Normalized `US-4.1.6.5` to `US-4.1.6` using user-stories parent-ID mapping.
- All IDs in this plan are mapped to specification artifacts.

## Traceability coverage

### Features

- `F-1.1.34`
- `F-1.1.35`
- `F-1.9.1`
- `F-4.1.1`
- `F-4.1.2`
- `F-4.1.3`
- `F-4.1.4`
- `F-4.1.5`
- `F-4.1.6`
- `F-4.1.7`
- `F-4.1.8`
- `F-4.1.9`
- `F-4.1.10`
- `F-4.2.1`
- `F-4.2.2`
- `F-4.2.3`
- `F-4.2.4`
- `F-4.2.5`
- `F-4.2.6`
- `F-4.2.7`
- `F-4.2.8`
- `F-4.2.9`
- `F-4.3.4`
- `F-4.7.5`
- `F-9.3.5`

### Requirements

- `R-4.1.1`
- `R-4.1.2`
- `R-4.1.3`
- `R-4.1.4`
- `R-4.1.5`
- `R-4.1.6`
- `R-4.1.7`
- `R-4.1.8`
- `R-4.1.9`
- `R-4.1.10`
- `R-4.2.1`
- `R-4.2.2`
- `R-4.2.3`
- `R-4.2.4`
- `R-4.2.5`
- `R-4.2.6`
- `R-4.2.7`
- `R-4.2.8`
- `R-4.2.9`

### User stories

- `US-4.1.1`
- `US-4.1.1.9`
- `US-4.1.4`
- `US-4.1.4.5`
- `US-4.1.5`
- `US-4.1.5.5`
- `US-4.1.6`
- `US-4.1.6.5`

### Test cases

- `TC-4.1.1.1`
- `TC-4.1.1.2`
- `TC-4.1.1.3`
- `TC-4.1.2.1`
- `TC-4.1.3.1`
- `TC-4.1.3.2`
- `TC-4.1.3.3`
- `TC-4.1.4.1`
- `TC-4.1.4.2`
- `TC-4.1.5.1`
- `TC-4.1.5.2`
- `TC-4.1.5.3`
- `TC-4.1.6.1`
- `TC-4.1.6.2`
- `TC-4.1.6.3`
- `TC-4.1.8.1`
- `TC-4.1.8.2`
- `TC-4.1.8.3`
- `TC-4.1.9.1`
- `TC-4.1.10.1`
- `TC-4.2.1.1`
- `TC-4.2.2.1`
- `TC-4.2.2.2`
- `TC-4.2.3.1`
- `TC-4.2.4.1`
- `TC-4.2.5.1`
- `TC-4.2.6.1`
- `TC-4.2.6.2`
- `TC-4.2.7.1`
- `TC-4.2.7.2`
- `TC-4.2.8.1`
- `TC-4.2.8.2`
- `TC-4.2.9.1`

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

- `TC-4.1.1.1`
- `TC-4.1.1.2`
- `TC-4.1.1.3`
- `TC-4.1.2.1`
- `TC-4.1.3.1`
- `TC-4.1.3.2`
- `TC-4.1.3.3`
- `TC-4.1.4.1`
- `TC-4.1.4.2`
- `TC-4.1.5.1`
- `TC-4.1.5.2`
- `TC-4.1.5.3`
- `TC-4.1.6.1`
- `TC-4.1.6.2`
- `TC-4.1.6.3`
- `TC-4.1.8.1`
- `TC-4.1.8.2`
- `TC-4.1.8.3`
- `TC-4.1.9.1`
- `TC-4.1.10.1`
- `TC-4.2.1.1`
- `TC-4.2.2.1`
- `TC-4.2.2.2`
- `TC-4.2.3.1`
- `TC-4.2.4.1`
- `TC-4.2.5.1`
- `TC-4.2.6.1`
- `TC-4.2.6.2`
- `TC-4.2.7.1`
- `TC-4.2.7.2`
- `TC-4.2.8.1`
- `TC-4.2.8.2`
- `TC-4.2.9.1`

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

- `US-4.1.1.9` resolved via parent `US-4.1.1` in
  [rigid-body-dynamics.md](../../user-stories/physics/rigid-body-dynamics.md).
- `US-4.1.4.5` resolved via parent `US-4.1.4` in
  [rigid-body-dynamics.md](../../user-stories/physics/rigid-body-dynamics.md).
- `US-4.1.5.5` resolved via parent `US-4.1.5` in
  [rigid-body-dynamics.md](../../user-stories/physics/rigid-body-dynamics.md).
- `US-4.1.6.5` resolved via parent `US-4.1.6` in
  [rigid-body-dynamics.md](../../user-stories/physics/rigid-body-dynamics.md).

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
