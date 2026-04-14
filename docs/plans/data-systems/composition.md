---
children: []
dependencies: []
design_documents:
  - docs/design/data-systems/composition.md
execution_mode: sequential
features:
  - F-1.1.1
  - F-1.4.1
  - F-1.5.1
  - F-11.3.1
  - F-16.5.1
  - F-16.5.2
  - F-16.5.3
  - F-16.5.4
  - F-16.5.5
  - F-16.5.6
  - F-16.5.7
  - F-16.6.1
id: PLAN-data-systems-composition
name: Composition
parent: null
progress_file: docs/plans/progress/PLAN-data-systems-composition.md
requirements:
  - R-16.5.1
  - R-16.5.2
  - R-16.5.3
  - R-16.5.4
  - R-16.5.5
  - R-16.5.6
  - R-16.5.7
status: not_started
test_cases:
  - TC-16.5.1.1
  - TC-16.5.1.2
  - TC-16.5.1.3
  - TC-16.5.1.4
  - TC-16.5.1.5
  - TC-16.5.1.6
  - TC-16.5.1.7
  - TC-16.5.1.8
  - TC-16.5.2.1
  - TC-16.5.2.2
  - TC-16.5.2.3
  - TC-16.5.2.4
  - TC-16.5.2.5
  - TC-16.5.2.6
  - TC-16.5.3.1
  - TC-16.5.3.2
  - TC-16.5.3.3
  - TC-16.5.3.4
  - TC-16.5.4.1
  - TC-16.5.4.2
  - TC-16.5.4.3
  - TC-16.5.6.1
  - TC-16.5.6.2
  - TC-16.5.7.1
  - TC-16.5.7.2
  - TC-16.5.7.3
worktree_branch: plan/data-systems-composition
---

# Composition implementation plan

- Plan ID: `PLAN-data-systems-composition`
- Progress file: [PLAN-data-systems-composition.md](../progress/PLAN-data-systems-composition.md)

## Source documents

- Design: [composition.md](../../design/data-systems/composition.md)
- Test cases: [composition-test-cases.md](../../design/data-systems/composition-test-cases.md)
- Progress: [PLAN-data-systems-composition.md](../progress/PLAN-data-systems-composition.md)

## Linked specification artifacts

### Features (`docs/features`)

- [attributes-effects.md](../../features/data-systems/attributes-effects.md) — covers `F-1.1.1`,
  `F-1.5.1`
- [containers-slots.md](../../features/data-systems/containers-slots.md) — covers `F-1.1.1`
- [data-tables.md](../../features/data-systems/data-tables.md) — covers `F-1.1.1`, `F-1.4.1`
- [directed-graphs.md](../../features/data-systems/directed-graphs.md) — covers `F-1.5.1`
- [camera-system.md](../../features/game-framework/camera-system.md) — covers `F-11.3.1`
- [game-modes-misc.md](../../features/game-framework/game-modes-misc.md) — covers `F-11.3.1`
- [destruction.md](../../features/vfx/destruction.md) — covers `F-11.3.1`
- [screen-effects.md](../../features/vfx/screen-effects.md) — covers `F-11.3.1`
- Still previously unmapped IDs: `F-16.5.1`, `F-16.5.2`, `F-16.5.3`, `F-16.5.4`, `F-16.5.5`,
  `F-16.5.6`, `F-16.5.7`, `F-16.6.1`

### Requirements (`docs/requirements`)

- No linked requirement docs found from plan IDs.
- Still previously unmapped IDs: `R-16.5.1`, `R-16.5.2`, `R-16.5.3`, `R-16.5.4`, `R-16.5.5`,
  `R-16.5.6`, `R-16.5.7`

### User stories (`docs/user-stories`)

- No linked user-storie docs found from plan IDs.
- Still previously unmapped IDs: `US-16.5.1`, `US-16.5.2`, `US-16.5.3`, `US-16.5.4`, `US-16.5.5`,
  `US-16.5.6`, `US-16.5.7`

### Test case sources

- [composition-test-cases.md](../../design/data-systems/composition-test-cases.md)

### Gap closure decisions

- No normalization was required for this plan.
- Remaining previously unmapped IDs are implementation-tracked in progress evidence as derived
  acceptance checks until upstream artifacts are added.

## Traceability coverage

### Features

- `F-1.1.1`
- `F-1.4.1`
- `F-1.5.1`
- `F-11.3.1`
- `F-16.5.1`
- `F-16.5.2`
- `F-16.5.3`
- `F-16.5.4`
- `F-16.5.5`
- `F-16.5.6`
- `F-16.5.7`
- `F-16.6.1`

### Requirements

- `R-16.5.1`
- `R-16.5.2`
- `R-16.5.3`
- `R-16.5.4`
- `R-16.5.5`
- `R-16.5.6`
- `R-16.5.7`

### User stories

- `US-16.5.1`
- `US-16.5.2`
- `US-16.5.3`
- `US-16.5.4`
- `US-16.5.5`
- `US-16.5.6`
- `US-16.5.7`

### Test cases

- `TC-16.5.1.1`
- `TC-16.5.1.2`
- `TC-16.5.1.3`
- `TC-16.5.1.4`
- `TC-16.5.1.5`
- `TC-16.5.1.6`
- `TC-16.5.1.7`
- `TC-16.5.1.8`
- `TC-16.5.2.1`
- `TC-16.5.2.2`
- `TC-16.5.2.3`
- `TC-16.5.2.4`
- `TC-16.5.2.5`
- `TC-16.5.2.6`
- `TC-16.5.3.1`
- `TC-16.5.3.2`
- `TC-16.5.3.3`
- `TC-16.5.3.4`
- `TC-16.5.4.1`
- `TC-16.5.4.2`
- `TC-16.5.4.3`
- `TC-16.5.6.1`
- `TC-16.5.6.2`
- `TC-16.5.7.1`
- `TC-16.5.7.2`
- `TC-16.5.7.3`

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

- `TC-16.5.1.1`
- `TC-16.5.1.2`
- `TC-16.5.1.3`
- `TC-16.5.1.4`
- `TC-16.5.1.5`
- `TC-16.5.1.6`
- `TC-16.5.1.7`
- `TC-16.5.1.8`
- `TC-16.5.2.1`
- `TC-16.5.2.2`
- `TC-16.5.2.3`
- `TC-16.5.2.4`
- `TC-16.5.2.5`
- `TC-16.5.2.6`
- `TC-16.5.3.1`
- `TC-16.5.3.2`
- `TC-16.5.3.3`
- `TC-16.5.3.4`
- `TC-16.5.4.1`
- `TC-16.5.4.2`
- `TC-16.5.4.3`
- `TC-16.5.6.1`
- `TC-16.5.6.2`
- `TC-16.5.7.1`
- `TC-16.5.7.2`
- `TC-16.5.7.3`

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
