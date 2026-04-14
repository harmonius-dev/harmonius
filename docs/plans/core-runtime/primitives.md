---
children: []
dependencies: []
design_documents:
  - docs/design/core-runtime/primitives.md
execution_mode: sequential
features:
  - F-1.7.4
  - F-1.7.5
  - F-1.7.6
  - F-1.9.2
  - F-1.9.3
  - F-1.9.4
  - F-1.9.5
  - F-1.9.6
  - F-1.9.7
id: PLAN-core-runtime-primitives
name: Primitives
parent: null
progress_file: docs/plans/progress/PLAN-core-runtime-primitives.md
requirements:
  - R-1.7.4a
  - R-1.9.2a
  - R-1.9.4a
  - R-1.9.6a
status: not_started
test_cases:
  - TC-1.7.4.1
  - TC-1.7.4.2
  - TC-1.7.4.3
  - TC-1.7.4.4
  - TC-1.7.4.5
  - TC-1.7.5.1
  - TC-1.7.5.2
  - TC-1.7.5.3
  - TC-1.7.6.1
  - TC-1.7.6.2
  - TC-1.9.2.1
  - TC-1.9.2.2
  - TC-1.9.2.3
  - TC-1.9.3.1
  - TC-1.9.3.2
  - TC-1.9.4.1
  - TC-1.9.4.2
  - TC-1.9.4.3
  - TC-1.9.5.1
  - TC-1.9.6.1
  - TC-1.9.6.2
  - TC-1.9.6.3
  - TC-1.9.6.4
  - TC-1.9.7.1
  - TC-1.9.7.2
worktree_branch: plan/core-runtime-primitives
---

# Primitives implementation plan

- Plan ID: `PLAN-core-runtime-primitives`
- Progress file: [PLAN-core-runtime-primitives.md](../progress/PLAN-core-runtime-primitives.md)

## Source documents

- Design: [primitives.md](../../design/core-runtime/primitives.md)
- Test cases: [primitives-test-cases.md](../../design/core-runtime/primitives-test-cases.md)
- Progress: [PLAN-core-runtime-primitives.md](../progress/PLAN-core-runtime-primitives.md)

## Linked specification artifacts

### Features (`docs/features`)

- [algorithms.md](../../features/core-runtime/algorithms.md) — covers `F-1.7.6`
- [async-io.md](../../features/core-runtime/async-io.md) — covers `F-1.7.4`
- [entity-component-system.md](../../features/core-runtime/entity-component-system.md) — covers
  `F-1.7.4`
- [memory-management.md](../../features/core-runtime/memory-management.md) — covers `F-1.7.4`,
  `F-1.7.5`, `F-1.7.6`
- [scene-and-transforms.md](../../features/core-runtime/scene-and-transforms.md) — covers `F-1.9.4`
- [spatial-indexing.md](../../features/core-runtime/spatial-indexing.md) — covers `F-1.9.2`,
  `F-1.9.3`, `F-1.9.4`, `F-1.9.5`, `F-1.9.6`, `F-1.9.7`

### Requirements (`docs/requirements`)

- [memory-management.md](../../requirements/core-runtime/memory-management.md) — covers `R-1.7.4a`
- [spatial-indexing.md](../../requirements/core-runtime/spatial-indexing.md) — covers `R-1.9.2a`,
  `R-1.9.4a`, `R-1.9.6a`

### User stories (`docs/user-stories`)

- No linked user-storie docs found from plan IDs.

### Test case sources

- [primitives-test-cases.md](../../design/core-runtime/primitives-test-cases.md)

### Gap closure decisions

- Normalized `R-1.7.4a` to `R-1.7.4` using requirements parent-ID mapping.
- Normalized `R-1.9.2a` to `R-1.9.2` using requirements parent-ID mapping.
- Normalized `R-1.9.4a` to `R-1.9.4` using requirements parent-ID mapping.
- Normalized `R-1.9.6a` to `R-1.9.6` using requirements parent-ID mapping.
- All IDs in this plan are mapped to specification artifacts.

## Traceability coverage

### Features

- `F-1.7.4`
- `F-1.7.5`
- `F-1.7.6`
- `F-1.9.2`
- `F-1.9.3`
- `F-1.9.4`
- `F-1.9.5`
- `F-1.9.6`
- `F-1.9.7`

### Requirements

- `R-1.7.4a`
- `R-1.9.2a`
- `R-1.9.4a`
- `R-1.9.6a`

### User stories

- No `US-*` IDs found in linked design artifacts.

### Test cases

- `TC-1.7.4.1`
- `TC-1.7.4.2`
- `TC-1.7.4.3`
- `TC-1.7.4.4`
- `TC-1.7.4.5`
- `TC-1.7.5.1`
- `TC-1.7.5.2`
- `TC-1.7.5.3`
- `TC-1.7.6.1`
- `TC-1.7.6.2`
- `TC-1.9.2.1`
- `TC-1.9.2.2`
- `TC-1.9.2.3`
- `TC-1.9.3.1`
- `TC-1.9.3.2`
- `TC-1.9.4.1`
- `TC-1.9.4.2`
- `TC-1.9.4.3`
- `TC-1.9.5.1`
- `TC-1.9.6.1`
- `TC-1.9.6.2`
- `TC-1.9.6.3`
- `TC-1.9.6.4`
- `TC-1.9.7.1`
- `TC-1.9.7.2`

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

- `TC-1.7.4.1`
- `TC-1.7.4.2`
- `TC-1.7.4.3`
- `TC-1.7.4.4`
- `TC-1.7.4.5`
- `TC-1.7.5.1`
- `TC-1.7.5.2`
- `TC-1.7.5.3`
- `TC-1.7.6.1`
- `TC-1.7.6.2`
- `TC-1.9.2.1`
- `TC-1.9.2.2`
- `TC-1.9.2.3`
- `TC-1.9.3.1`
- `TC-1.9.3.2`
- `TC-1.9.4.1`
- `TC-1.9.4.2`
- `TC-1.9.4.3`
- `TC-1.9.5.1`
- `TC-1.9.6.1`
- `TC-1.9.6.2`
- `TC-1.9.6.3`
- `TC-1.9.6.4`
- `TC-1.9.7.1`
- `TC-1.9.7.2`

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

- `R-1.7.4a` resolved via parent `R-1.7.4` in
  [memory-management.md](../../requirements/core-runtime/memory-management.md).
- `R-1.9.2a` resolved via parent `R-1.9.2` in
  [spatial-indexing.md](../../requirements/core-runtime/spatial-indexing.md).
- `R-1.9.4a` resolved via parent `R-1.9.4` in
  [spatial-indexing.md](../../requirements/core-runtime/spatial-indexing.md).
- `R-1.9.6a` resolved via parent `R-1.9.6` in
  [spatial-indexing.md](../../requirements/core-runtime/spatial-indexing.md).

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
