---
children: []
dependencies: []
design_documents:
  - docs/design/ai/steering-crowds.md
execution_mode: sequential
features:
  - F-7.2.1
  - F-7.2.2
  - F-7.2.3
  - F-7.2.4
  - F-7.2.5
  - F-7.2.6
  - F-7.7.1
  - F-7.7.2
  - F-7.7.3
  - F-7.7.4
  - F-7.7.5
  - F-7.7.6
  - F-7.7.7
  - F-7.7.8
  - F-7.7.9
  - F-7.7.10
  - F-7.7.11
  - F-7.8.1
  - F-7.8.6
id: PLAN-ai-steering-crowds
name: Steering Crowds
parent: null
progress_file: docs/plans/progress/PLAN-ai-steering-crowds.md
requirements:
  - R-7.2.1
  - R-7.2.2
  - R-7.2.3
  - R-7.2.4
  - R-7.2.5
  - R-7.2.6
  - R-7.7.1
  - R-7.7.2
  - R-7.7.3
  - R-7.7.4
  - R-7.7.5
  - R-7.7.6
status: not_started
test_cases:
  - TC-7.2.1.1
  - TC-7.2.1.2
  - TC-7.2.2.1
  - TC-7.2.3.1
  - TC-7.2.3.2
  - TC-7.2.3.3
  - TC-7.2.3.4
  - TC-7.2.3.5
  - TC-7.2.3.6
  - TC-7.2.3.7
  - TC-7.2.4.1
  - TC-7.2.4.2
  - TC-7.2.4.3
  - TC-7.2.5.1
  - TC-7.2.5.2
  - TC-7.2.5.3
  - TC-7.2.6.1
  - TC-7.2.6.2
  - TC-7.7.1.1
  - TC-7.7.1.2
  - TC-7.7.2.1
  - TC-7.7.2.2
  - TC-7.7.3.1
  - TC-7.7.3.2
  - TC-7.7.3.3
  - TC-7.7.5.1
  - TC-7.7.5.2
  - TC-7.7.5.3
  - TC-7.7.6.1
  - TC-7.7.6.2
worktree_branch: plan/ai-steering-crowds
---

# Steering Crowds implementation plan

- Plan ID: `PLAN-ai-steering-crowds`
- Progress file: [PLAN-ai-steering-crowds.md](../progress/PLAN-ai-steering-crowds.md)

## Source documents

- Design: [steering-crowds.md](../../design/ai/steering-crowds.md)
- Test cases: [steering-crowds-test-cases.md](../../design/ai/steering-crowds-test-cases.md)
- Progress: [PLAN-ai-steering-crowds.md](../progress/PLAN-ai-steering-crowds.md)

## Linked specification artifacts

### Features (`docs/features`)

- [behavior-trees.md](../../features/ai/behavior-trees.md) — covers `F-7.7.5`
- [crowd-simulation.md](../../features/ai/crowd-simulation.md) — covers `F-7.2.1`, `F-7.2.3`,
  `F-7.2.5`, `F-7.7.1`, `F-7.7.10`, `F-7.7.11`, `F-7.7.2`, `F-7.7.3`...
- [steering-avoidance.md](../../features/ai/steering-avoidance.md) — covers `F-7.2.1`, `F-7.2.2`,
  `F-7.2.3`, `F-7.2.4`, `F-7.2.5`, `F-7.2.6`, `F-7.7.4`
- [tactical-combat.md](../../features/ai/tactical-combat.md) — covers `F-7.2.1`, `F-7.2.5`,
  `F-7.8.1`, `F-7.8.6`

### Requirements (`docs/requirements`)

- [crowd-simulation.md](../../requirements/ai/crowd-simulation.md) — covers `R-7.7.1`, `R-7.7.2`,
  `R-7.7.3`, `R-7.7.4`, `R-7.7.5`, `R-7.7.6`
- [steering-avoidance.md](../../requirements/ai/steering-avoidance.md) — covers `R-7.2.1`,
  `R-7.2.2`, `R-7.2.3`, `R-7.2.4`, `R-7.2.5`, `R-7.2.6`

### User stories (`docs/user-stories`)

- [crowd-simulation.md](../../user-stories/ai/crowd-simulation.md) — covers `US-7.7.1`,
  `US-7.7.1.12`, `US-7.7.11`, `US-7.7.11.4`, `US-7.7.2`, `US-7.7.2.12`, `US-7.7.4`, `US-7.7.4.12`...
- [steering-avoidance.md](../../user-stories/ai/steering-avoidance.md) — covers `US-7.2.1`,
  `US-7.2.1.12`, `US-7.2.2`, `US-7.2.2.12`, `US-7.2.3`, `US-7.2.3.9`, `US-7.2.4`, `US-7.2.4.12`

### Test case sources

- [steering-crowds-test-cases.md](../../design/ai/steering-crowds-test-cases.md)

### Gap closure decisions

- Normalized `US-7.2.1.12` to `US-7.2.1` using user-stories parent-ID mapping.
- Normalized `US-7.2.2.12` to `US-7.2.2` using user-stories parent-ID mapping.
- Normalized `US-7.2.3.9` to `US-7.2.3` using user-stories parent-ID mapping.
- Normalized `US-7.2.4.12` to `US-7.2.4` using user-stories parent-ID mapping.
- Normalized `US-7.7.1.12` to `US-7.7.1` using user-stories parent-ID mapping.
- Normalized `US-7.7.11.4` to `US-7.7.11` using user-stories parent-ID mapping.
- Normalized `US-7.7.2.12` to `US-7.7.2` using user-stories parent-ID mapping.
- Normalized `US-7.7.4.12` to `US-7.7.4` using user-stories parent-ID mapping.
- Normalized `US-7.7.5.11` to `US-7.7.5` using user-stories parent-ID mapping.
- Normalized `US-7.7.6.12` to `US-7.7.6` using user-stories parent-ID mapping.
- Normalized `US-7.7.8.4` to `US-7.7.8` using user-stories parent-ID mapping.
- All IDs in this plan are mapped to specification artifacts.

## Traceability coverage

### Features

- `F-7.2.1`
- `F-7.2.2`
- `F-7.2.3`
- `F-7.2.4`
- `F-7.2.5`
- `F-7.2.6`
- `F-7.7.1`
- `F-7.7.2`
- `F-7.7.3`
- `F-7.7.4`
- `F-7.7.5`
- `F-7.7.6`
- `F-7.7.7`
- `F-7.7.8`
- `F-7.7.9`
- `F-7.7.10`
- `F-7.7.11`
- `F-7.8.1`
- `F-7.8.6`

### Requirements

- `R-7.2.1`
- `R-7.2.2`
- `R-7.2.3`
- `R-7.2.4`
- `R-7.2.5`
- `R-7.2.6`
- `R-7.7.1`
- `R-7.7.2`
- `R-7.7.3`
- `R-7.7.4`
- `R-7.7.5`
- `R-7.7.6`

### User stories

- `US-7.2.1`
- `US-7.2.1.12`
- `US-7.2.2`
- `US-7.2.2.12`
- `US-7.2.3`
- `US-7.2.3.9`
- `US-7.2.4`
- `US-7.2.4.12`
- `US-7.7.1`
- `US-7.7.1.12`
- `US-7.7.2`
- `US-7.7.2.12`
- `US-7.7.4`
- `US-7.7.4.12`
- `US-7.7.5`
- `US-7.7.5.11`
- `US-7.7.6`
- `US-7.7.6.12`
- `US-7.7.8`
- `US-7.7.8.4`
- `US-7.7.11`
- `US-7.7.11.4`

### Test cases

- `TC-7.2.1.1`
- `TC-7.2.1.2`
- `TC-7.2.2.1`
- `TC-7.2.3.1`
- `TC-7.2.3.2`
- `TC-7.2.3.3`
- `TC-7.2.3.4`
- `TC-7.2.3.5`
- `TC-7.2.3.6`
- `TC-7.2.3.7`
- `TC-7.2.4.1`
- `TC-7.2.4.2`
- `TC-7.2.4.3`
- `TC-7.2.5.1`
- `TC-7.2.5.2`
- `TC-7.2.5.3`
- `TC-7.2.6.1`
- `TC-7.2.6.2`
- `TC-7.7.1.1`
- `TC-7.7.1.2`
- `TC-7.7.2.1`
- `TC-7.7.2.2`
- `TC-7.7.3.1`
- `TC-7.7.3.2`
- `TC-7.7.3.3`
- `TC-7.7.5.1`
- `TC-7.7.5.2`
- `TC-7.7.5.3`
- `TC-7.7.6.1`
- `TC-7.7.6.2`

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

- `TC-7.2.1.1`
- `TC-7.2.1.2`
- `TC-7.2.2.1`
- `TC-7.2.3.1`
- `TC-7.2.3.2`
- `TC-7.2.3.3`
- `TC-7.2.3.4`
- `TC-7.2.3.5`
- `TC-7.2.3.6`
- `TC-7.2.3.7`
- `TC-7.2.4.1`
- `TC-7.2.4.2`
- `TC-7.2.4.3`
- `TC-7.2.5.1`
- `TC-7.2.5.2`
- `TC-7.2.5.3`
- `TC-7.2.6.1`
- `TC-7.2.6.2`
- `TC-7.7.1.1`
- `TC-7.7.1.2`
- `TC-7.7.2.1`
- `TC-7.7.2.2`
- `TC-7.7.3.1`
- `TC-7.7.3.2`
- `TC-7.7.3.3`
- `TC-7.7.5.1`
- `TC-7.7.5.2`
- `TC-7.7.5.3`
- `TC-7.7.6.1`
- `TC-7.7.6.2`

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

- `US-7.2.1.12` resolved via parent `US-7.2.1` in
  [steering-avoidance.md](../../user-stories/ai/steering-avoidance.md).
- `US-7.2.2.12` resolved via parent `US-7.2.2` in
  [steering-avoidance.md](../../user-stories/ai/steering-avoidance.md).
- `US-7.2.3.9` resolved via parent `US-7.2.3` in
  [steering-avoidance.md](../../user-stories/ai/steering-avoidance.md).
- `US-7.2.4.12` resolved via parent `US-7.2.4` in
  [steering-avoidance.md](../../user-stories/ai/steering-avoidance.md).
- `US-7.7.1.12` resolved via parent `US-7.7.1` in
  [crowd-simulation.md](../../user-stories/ai/crowd-simulation.md).
- `US-7.7.11.4` resolved via parent `US-7.7.11` in
  [crowd-simulation.md](../../user-stories/ai/crowd-simulation.md).
- `US-7.7.2.12` resolved via parent `US-7.7.2` in
  [crowd-simulation.md](../../user-stories/ai/crowd-simulation.md).
- `US-7.7.4.12` resolved via parent `US-7.7.4` in
  [crowd-simulation.md](../../user-stories/ai/crowd-simulation.md).
- `US-7.7.5.11` resolved via parent `US-7.7.5` in
  [crowd-simulation.md](../../user-stories/ai/crowd-simulation.md).
- `US-7.7.6.12` resolved via parent `US-7.7.6` in
  [crowd-simulation.md](../../user-stories/ai/crowd-simulation.md).
- `US-7.7.8.4` resolved via parent `US-7.7.8` in
  [crowd-simulation.md](../../user-stories/ai/crowd-simulation.md).

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
