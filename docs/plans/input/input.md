---
children: []
dependencies: []
design_documents:
  - docs/design/input/input.md
execution_mode: sequential
features:
  - F-1.9.4
  - F-6.1.1
  - F-6.1.2
  - F-6.1.3
  - F-6.1.4
  - F-6.1.5
  - F-6.2.1
  - F-6.2.2
  - F-6.2.3
  - F-6.2.4
  - F-6.2.5
  - F-6.2.6
  - F-6.2.7
  - F-6.2.8
  - F-6.2.9
  - F-6.2.10
  - F-6.2.11
  - F-6.3.1
  - F-6.3.2
  - F-6.3.3
  - F-6.3.4
  - F-6.3.5
  - F-6.4.1
  - F-6.4.2
  - F-6.4.3
  - F-6.4.4
  - F-6.4.5
  - F-6.5.1
  - F-6.5.2
  - F-6.5.3
  - F-6.5.4
  - F-6.5.5
id: PLAN-input-input
name: Input
parent: null
progress_file: docs/plans/progress/PLAN-input-input.md
requirements:
  - R-6.1.1
  - R-6.1.2
  - R-6.1.3
  - R-6.1.4
  - R-6.1.5
  - R-6.1.6
  - R-6.1.7
  - R-6.1.8
  - R-6.1.9
  - R-6.1.11
  - R-6.1.12
  - R-6.1.13
  - R-6.1.14
  - R-6.1.15
  - R-6.2.1
  - R-6.2.2
  - R-6.2.3
  - R-6.2.4
  - R-6.2.5
  - R-6.2.6
  - R-6.2.7
  - R-6.2.8
  - R-6.2.9
  - R-6.2.10
  - R-6.2.11
  - R-6.2.12
  - R-6.2.13
  - R-6.2.14
  - R-6.2.16
  - R-6.2.17
  - R-6.2.18
  - R-6.2.19
  - R-6.2.20
  - R-6.3.1
  - R-6.3.2
  - R-6.3.3
  - R-6.3.4
  - R-6.3.5
  - R-6.3.7
  - R-6.4.1
  - R-6.4.2
  - R-6.4.3
  - R-6.4.4
  - R-6.4.5
  - R-6.4.7
  - R-6.4.10
  - R-6.5.1
  - R-6.5.2
  - R-6.5.3
  - R-6.5.4
  - R-6.5.5
  - R-6.5.6
  - R-6.5.7
  - R-6.5.8
  - R-6.5.10
  - R-6.5.11
status: not_started
test_cases:
  - TC-6.1.1.1
  - TC-6.1.2.1
  - TC-6.1.3.1
  - TC-6.1.4.1
  - TC-6.1.5.1
  - TC-6.1.6.1
  - TC-6.1.7.1
  - TC-6.1.8.1
  - TC-6.1.9.1
  - TC-6.1.11.1
  - TC-6.1.14.1
  - TC-6.1.15.1
  - TC-6.2.1.1
  - TC-6.2.2.1
  - TC-6.2.3.1
  - TC-6.2.4.1
  - TC-6.2.5.1
  - TC-6.2.6.1
  - TC-6.2.7.1
  - TC-6.2.8.1
  - TC-6.2.9.1
  - TC-6.2.10.1
  - TC-6.2.11.1
  - TC-6.2.13.1
  - TC-6.2.14.1
  - TC-6.2.17.1
  - TC-6.2.18.1
  - TC-6.2.19.1
  - TC-6.2.20.1
  - TC-6.3.1.1
  - TC-6.3.2.1
  - TC-6.3.4.1
  - TC-6.3.5.1
  - TC-6.3.7.1
  - TC-6.4.1.1
  - TC-6.4.2.1
  - TC-6.4.3.1
  - TC-6.4.4.1
  - TC-6.4.5.1
  - TC-6.4.7.1
  - TC-6.5.1.1
  - TC-6.5.3.1
  - TC-6.5.4.1
  - TC-6.5.5.1
  - TC-6.5.6.1
  - TC-6.5.8.1
  - TC-6.5.11.1
worktree_branch: plan/input-input
---

# Input implementation plan

- Plan ID: `PLAN-input-input`
- Progress file: [PLAN-input-input.md](../progress/PLAN-input-input.md)

## Source documents

- Design: [input.md](../../design/input/input.md)
- Test cases: [input-test-cases.md](../../design/input/input-test-cases.md)
- Progress: [PLAN-input-input.md](../progress/PLAN-input-input.md)

## Linked specification artifacts

### Features (`docs/features`)

- [device-abstraction.md](../../features/input/device-abstraction.md) — covers `F-6.1.1`, `F-6.1.2`,
  `F-6.1.3`, `F-6.1.4`, `F-6.1.5`
- [gestures.md](../../features/input/gestures.md) — covers `F-6.1.4`, `F-6.3.1`, `F-6.3.2`,
  `F-6.3.3`, `F-6.3.4`, `F-6.3.5`, `F-6.4.1`
- [haptics-and-feedback.md](../../features/input/haptics-and-feedback.md) — covers `F-6.1.3`,
  `F-6.4.1`, `F-6.4.2`, `F-6.4.3`, `F-6.4.4`, `F-6.4.5`
- [input-actions-and-mapping.md](../../features/input/input-actions-and-mapping.md) — covers
  `F-1.9.4`, `F-6.2.1`, `F-6.2.10`, `F-6.2.11`, `F-6.2.2`, `F-6.2.3`, `F-6.2.4`, `F-6.2.5`...
- [vr-input.md](../../features/input/vr-input.md) — covers `F-6.1.1`, `F-6.1.3`, `F-6.2.1`,
  `F-6.4.1`, `F-6.5.1`, `F-6.5.2`, `F-6.5.3`, `F-6.5.4`...

### Requirements (`docs/requirements`)

- [device-abstraction.md](../../requirements/input/device-abstraction.md) — covers `R-6.1.1`,
  `R-6.1.11`, `R-6.1.12`, `R-6.1.13`, `R-6.1.14`, `R-6.1.15`, `R-6.1.2`, `R-6.1.3`...
- [gestures.md](../../requirements/input/gestures.md) — covers `R-6.3.1`, `R-6.3.2`, `R-6.3.3`,
  `R-6.3.4`, `R-6.3.5`, `R-6.3.7`
- [haptics-and-feedback.md](../../requirements/input/haptics-and-feedback.md) — covers `R-6.4.1`,
  `R-6.4.10`, `R-6.4.2`, `R-6.4.3`, `R-6.4.4`, `R-6.4.5`, `R-6.4.7`
- [input-actions-and-mapping.md](../../requirements/input/input-actions-and-mapping.md) — covers
  `R-6.2.1`, `R-6.2.10`, `R-6.2.11`, `R-6.2.12`, `R-6.2.13`, `R-6.2.14`, `R-6.2.16`, `R-6.2.17`...
- [vr-input.md](../../requirements/input/vr-input.md) — covers `R-6.5.1`, `R-6.5.10`, `R-6.5.11`,
  `R-6.5.2`, `R-6.5.3`, `R-6.5.4`, `R-6.5.5`, `R-6.5.6`...

### User stories (`docs/user-stories`)

- No linked user-storie docs found from plan IDs.

### Test case sources

- [input-test-cases.md](../../design/input/input-test-cases.md)

### Gap closure decisions

- No normalization was required for this plan.
- All IDs in this plan are mapped to specification artifacts.

## Traceability coverage

### Features

- `F-1.9.4`
- `F-6.1.1`
- `F-6.1.2`
- `F-6.1.3`
- `F-6.1.4`
- `F-6.1.5`
- `F-6.2.1`
- `F-6.2.2`
- `F-6.2.3`
- `F-6.2.4`
- `F-6.2.5`
- `F-6.2.6`
- `F-6.2.7`
- `F-6.2.8`
- `F-6.2.9`
- `F-6.2.10`
- `F-6.2.11`
- `F-6.3.1`
- `F-6.3.2`
- `F-6.3.3`
- `F-6.3.4`
- `F-6.3.5`
- `F-6.4.1`
- `F-6.4.2`
- `F-6.4.3`
- `F-6.4.4`
- `F-6.4.5`
- `F-6.5.1`
- `F-6.5.2`
- `F-6.5.3`
- `F-6.5.4`
- `F-6.5.5`

### Requirements

- `R-6.1.1`
- `R-6.1.2`
- `R-6.1.3`
- `R-6.1.4`
- `R-6.1.5`
- `R-6.1.6`
- `R-6.1.7`
- `R-6.1.8`
- `R-6.1.9`
- `R-6.1.11`
- `R-6.1.12`
- `R-6.1.13`
- `R-6.1.14`
- `R-6.1.15`
- `R-6.2.1`
- `R-6.2.2`
- `R-6.2.3`
- `R-6.2.4`
- `R-6.2.5`
- `R-6.2.6`
- `R-6.2.7`
- `R-6.2.8`
- `R-6.2.9`
- `R-6.2.10`
- `R-6.2.11`
- `R-6.2.12`
- `R-6.2.13`
- `R-6.2.14`
- `R-6.2.16`
- `R-6.2.17`
- `R-6.2.18`
- `R-6.2.19`
- `R-6.2.20`
- `R-6.3.1`
- `R-6.3.2`
- `R-6.3.3`
- `R-6.3.4`
- `R-6.3.5`
- `R-6.3.7`
- `R-6.4.1`
- `R-6.4.2`
- `R-6.4.3`
- `R-6.4.4`
- `R-6.4.5`
- `R-6.4.7`
- `R-6.4.10`
- `R-6.5.1`
- `R-6.5.2`
- `R-6.5.3`
- `R-6.5.4`
- `R-6.5.5`
- `R-6.5.6`
- `R-6.5.7`
- `R-6.5.8`
- `R-6.5.10`
- `R-6.5.11`

### User stories

- No `US-*` IDs found in linked design artifacts.

### Test cases

- `TC-6.1.1.1`
- `TC-6.1.2.1`
- `TC-6.1.3.1`
- `TC-6.1.4.1`
- `TC-6.1.5.1`
- `TC-6.1.6.1`
- `TC-6.1.7.1`
- `TC-6.1.8.1`
- `TC-6.1.9.1`
- `TC-6.1.11.1`
- `TC-6.1.14.1`
- `TC-6.1.15.1`
- `TC-6.2.1.1`
- `TC-6.2.2.1`
- `TC-6.2.3.1`
- `TC-6.2.4.1`
- `TC-6.2.5.1`
- `TC-6.2.6.1`
- `TC-6.2.7.1`
- `TC-6.2.8.1`
- `TC-6.2.9.1`
- `TC-6.2.10.1`
- `TC-6.2.11.1`
- `TC-6.2.13.1`
- `TC-6.2.14.1`
- `TC-6.2.17.1`
- `TC-6.2.18.1`
- `TC-6.2.19.1`
- `TC-6.2.20.1`
- `TC-6.3.1.1`
- `TC-6.3.2.1`
- `TC-6.3.4.1`
- `TC-6.3.5.1`
- `TC-6.3.7.1`
- `TC-6.4.1.1`
- `TC-6.4.2.1`
- `TC-6.4.3.1`
- `TC-6.4.4.1`
- `TC-6.4.5.1`
- `TC-6.4.7.1`
- `TC-6.5.1.1`
- `TC-6.5.3.1`
- `TC-6.5.4.1`
- `TC-6.5.5.1`
- `TC-6.5.6.1`
- `TC-6.5.8.1`
- `TC-6.5.11.1`

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

- `TC-6.1.1.1`
- `TC-6.1.2.1`
- `TC-6.1.3.1`
- `TC-6.1.4.1`
- `TC-6.1.5.1`
- `TC-6.1.6.1`
- `TC-6.1.7.1`
- `TC-6.1.8.1`
- `TC-6.1.9.1`
- `TC-6.1.11.1`
- `TC-6.1.14.1`
- `TC-6.1.15.1`
- `TC-6.2.1.1`
- `TC-6.2.2.1`
- `TC-6.2.3.1`
- `TC-6.2.4.1`
- `TC-6.2.5.1`
- `TC-6.2.6.1`
- `TC-6.2.7.1`
- `TC-6.2.8.1`
- `TC-6.2.9.1`
- `TC-6.2.10.1`
- `TC-6.2.11.1`
- `TC-6.2.13.1`
- `TC-6.2.14.1`
- `TC-6.2.17.1`
- `TC-6.2.18.1`
- `TC-6.2.19.1`
- `TC-6.2.20.1`
- `TC-6.3.1.1`
- `TC-6.3.2.1`
- `TC-6.3.4.1`
- `TC-6.3.5.1`
- `TC-6.3.7.1`
- `TC-6.4.1.1`
- `TC-6.4.2.1`
- `TC-6.4.3.1`
- `TC-6.4.4.1`
- `TC-6.4.5.1`
- `TC-6.4.7.1`
- `TC-6.5.1.1`
- `TC-6.5.3.1`
- `TC-6.5.4.1`
- `TC-6.5.5.1`
- `TC-6.5.6.1`
- `TC-6.5.8.1`
- `TC-6.5.11.1`

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
