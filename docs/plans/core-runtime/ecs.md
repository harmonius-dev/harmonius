---
children: []
dependencies: []
design_documents:
  - docs/design/core-runtime/ecs.md
execution_mode: sequential
features:
  - F-1.1.1
  - F-1.1.2
  - F-1.1.3
  - F-1.1.4
  - F-1.1.5
  - F-1.1.6
  - F-1.1.7
  - F-1.1.8
  - F-1.1.9
  - F-1.1.10
  - F-1.1.11
  - F-1.1.12
  - F-1.1.13
  - F-1.1.14
  - F-1.1.15
  - F-1.1.16
  - F-1.1.17
  - F-1.1.18
  - F-1.1.19
  - F-1.1.20
  - F-1.1.21
  - F-1.1.22
  - F-1.1.23
  - F-1.1.24
  - F-1.1.25
  - F-1.1.26
  - F-1.1.27
  - F-1.1.28
  - F-1.1.29
  - F-1.1.30
  - F-1.1.31
  - F-1.1.32
  - F-1.1.33
  - F-1.1.34
  - F-1.1.35
  - F-1.1.36
  - F-1.1.37
  - F-1.1.38
id: PLAN-core-runtime-ecs
name: Ecs
parent: null
progress_file: docs/plans/progress/PLAN-core-runtime-ecs.md
requirements:
  - R-1.1.1
  - R-1.1.2
  - R-1.1.3
  - R-1.1.4
  - R-1.1.5
  - R-1.1.6
  - R-1.1.7
  - R-1.1.8
  - R-1.1.9
  - R-1.1.10
  - R-1.1.11
  - R-1.1.12
  - R-1.1.13
  - R-1.1.14
  - R-1.1.15
  - R-1.1.16
  - R-1.1.17
  - R-1.1.18
  - R-1.1.19
  - R-1.1.20
  - R-1.1.21
  - R-1.1.22
  - R-1.1.23
  - R-1.1.24
  - R-1.1.25
  - R-1.1.26
  - R-1.1.27
  - R-1.1.28
  - R-1.1.29
  - R-1.1.30
  - R-1.1.31
  - R-1.1.32
  - R-1.1.33
  - R-1.1.34
  - R-1.1.35
  - R-1.1.36
  - R-1.1.37
  - R-1.1.38
  - R-1.1.11a
  - R-1.1.16a
  - R-1.1.17a
  - R-1.1.1a
  - R-1.1.22a
  - R-1.1.25a
  - R-1.1.2a
  - R-1.1.30a
  - R-1.1.32a
  - R-1.1.35a
  - R-1.1.3a
  - R-1.1.9a
status: not_started
test_cases:
  - TC-1.1.1.1
  - TC-1.1.1.2
  - TC-1.1.2.1
  - TC-1.1.2.2
  - TC-1.1.3.1
  - TC-1.1.3.2
  - TC-1.1.4.1
  - TC-1.1.4.2
  - TC-1.1.5.1
  - TC-1.1.6.1
  - TC-1.1.7.1
  - TC-1.1.8.1
  - TC-1.1.8.2
  - TC-1.1.9.1
  - TC-1.1.9.2
  - TC-1.1.10.1
  - TC-1.1.10.2
  - TC-1.1.11.1
  - TC-1.1.11.2
  - TC-1.1.12.1
  - TC-1.1.13.1
  - TC-1.1.14.1
  - TC-1.1.15.1
  - TC-1.1.15.2
  - TC-1.1.16.1
  - TC-1.1.16.2
  - TC-1.1.16.3
  - TC-1.1.17.1
  - TC-1.1.17.2
  - TC-1.1.17.3
  - TC-1.1.18.1
  - TC-1.1.19.1
  - TC-1.1.21.1
  - TC-1.1.22.1
  - TC-1.1.22.2
  - TC-1.1.23.1
  - TC-1.1.24.1
  - TC-1.1.25.1
  - TC-1.1.25.2
  - TC-1.1.26.1
  - TC-1.1.27.1
  - TC-1.1.28.1
  - TC-1.1.29.1
  - TC-1.1.30.1
  - TC-1.1.31.1
  - TC-1.1.32.1
  - TC-1.1.33.1
  - TC-1.1.34.1
  - TC-1.1.35.1
  - TC-1.1.35.2
  - TC-1.1.36.1
  - TC-1.1.37.1
  - TC-1.1.38.1
worktree_branch: plan/core-runtime-ecs
---

# Ecs implementation plan

- Plan ID: `PLAN-core-runtime-ecs`
- Progress file: [PLAN-core-runtime-ecs.md](../progress/PLAN-core-runtime-ecs.md)

## Source documents

- Design: [ecs.md](../../design/core-runtime/ecs.md)
- Test cases: [ecs-test-cases.md](../../design/core-runtime/ecs-test-cases.md)
- Progress: [PLAN-core-runtime-ecs.md](../progress/PLAN-core-runtime-ecs.md)

## Linked specification artifacts

### Features (`docs/features`)

- [algorithms.md](../../features/core-runtime/algorithms.md) — covers `F-1.1.17`
- [entity-component-system.md](../../features/core-runtime/entity-component-system.md) — covers
  `F-1.1.1`, `F-1.1.10`, `F-1.1.11`, `F-1.1.12`, `F-1.1.13`, `F-1.1.14`, `F-1.1.15`, `F-1.1.16`...
- [events-and-messaging.md](../../features/core-runtime/events-and-messaging.md) — covers
  `F-1.1.11`, `F-1.1.16`, `F-1.1.17`, `F-1.1.22`, `F-1.1.23`, `F-1.1.25`, `F-1.1.30`, `F-1.1.31`...
- [game-loop.md](../../features/core-runtime/game-loop.md) — covers `F-1.1.20`, `F-1.1.25`,
  `F-1.1.26`, `F-1.1.38`
- [plugin-system.md](../../features/core-runtime/plugin-system.md) — covers `F-1.1.4`
- [scene-and-transforms.md](../../features/core-runtime/scene-and-transforms.md) — covers
  `F-1.1.11`, `F-1.1.17`, `F-1.1.20`, `F-1.1.22`, `F-1.1.32`
- [serialization.md](../../features/core-runtime/serialization.md) — covers `F-1.1.34`
- [spatial-indexing.md](../../features/core-runtime/spatial-indexing.md) — covers `F-1.1.1`,
  `F-1.1.17`, `F-1.1.22`

### Requirements (`docs/requirements`)

- [entity-component-system.md](../../requirements/core-runtime/entity-component-system.md) — covers
  `R-1.1.1`, `R-1.1.10`, `R-1.1.11`, `R-1.1.11a`, `R-1.1.12`, `R-1.1.13`, `R-1.1.14`, `R-1.1.15`...

### User stories (`docs/user-stories`)

- No linked user-storie docs found from plan IDs.
- Still previously unmapped IDs: `US-1.1.49`, `US-1.1.50`

### Test case sources

- [ecs-test-cases.md](../../design/core-runtime/ecs-test-cases.md)

### Gap closure decisions

- Normalized `R-1.1.11a` to `R-1.1.11` using requirements parent-ID mapping.
- Normalized `R-1.1.16a` to `R-1.1.16` using requirements parent-ID mapping.
- Normalized `R-1.1.17a` to `R-1.1.17` using requirements parent-ID mapping.
- Normalized `R-1.1.1a` to `R-1.1.1` using requirements parent-ID mapping.
- Normalized `R-1.1.22a` to `R-1.1.22` using requirements parent-ID mapping.
- Normalized `R-1.1.25a` to `R-1.1.25` using requirements parent-ID mapping.
- Normalized `R-1.1.2a` to `R-1.1.2` using requirements parent-ID mapping.
- Normalized `R-1.1.30a` to `R-1.1.30` using requirements parent-ID mapping.
- Normalized `R-1.1.32a` to `R-1.1.32` using requirements parent-ID mapping.
- Normalized `R-1.1.35a` to `R-1.1.35` using requirements parent-ID mapping.
- Normalized `R-1.1.3a` to `R-1.1.3` using requirements parent-ID mapping.
- Normalized `R-1.1.9a` to `R-1.1.9` using requirements parent-ID mapping.
- Remaining previously unmapped IDs are implementation-tracked in progress evidence as derived
  acceptance checks until upstream artifacts are added.

## Traceability coverage

### Features

- `F-1.1.1`
- `F-1.1.2`
- `F-1.1.3`
- `F-1.1.4`
- `F-1.1.5`
- `F-1.1.6`
- `F-1.1.7`
- `F-1.1.8`
- `F-1.1.9`
- `F-1.1.10`
- `F-1.1.11`
- `F-1.1.12`
- `F-1.1.13`
- `F-1.1.14`
- `F-1.1.15`
- `F-1.1.16`
- `F-1.1.17`
- `F-1.1.18`
- `F-1.1.19`
- `F-1.1.20`
- `F-1.1.21`
- `F-1.1.22`
- `F-1.1.23`
- `F-1.1.24`
- `F-1.1.25`
- `F-1.1.26`
- `F-1.1.27`
- `F-1.1.28`
- `F-1.1.29`
- `F-1.1.30`
- `F-1.1.31`
- `F-1.1.32`
- `F-1.1.33`
- `F-1.1.34`
- `F-1.1.35`
- `F-1.1.36`
- `F-1.1.37`
- `F-1.1.38`

### Requirements

- `R-1.1.1`
- `R-1.1.2`
- `R-1.1.3`
- `R-1.1.4`
- `R-1.1.5`
- `R-1.1.6`
- `R-1.1.7`
- `R-1.1.8`
- `R-1.1.9`
- `R-1.1.10`
- `R-1.1.11`
- `R-1.1.12`
- `R-1.1.13`
- `R-1.1.14`
- `R-1.1.15`
- `R-1.1.16`
- `R-1.1.17`
- `R-1.1.18`
- `R-1.1.19`
- `R-1.1.20`
- `R-1.1.21`
- `R-1.1.22`
- `R-1.1.23`
- `R-1.1.24`
- `R-1.1.25`
- `R-1.1.26`
- `R-1.1.27`
- `R-1.1.28`
- `R-1.1.29`
- `R-1.1.30`
- `R-1.1.31`
- `R-1.1.32`
- `R-1.1.33`
- `R-1.1.34`
- `R-1.1.35`
- `R-1.1.36`
- `R-1.1.37`
- `R-1.1.38`
- `R-1.1.11a`
- `R-1.1.16a`
- `R-1.1.17a`
- `R-1.1.1a`
- `R-1.1.22a`
- `R-1.1.25a`
- `R-1.1.2a`
- `R-1.1.30a`
- `R-1.1.32a`
- `R-1.1.35a`
- `R-1.1.3a`
- `R-1.1.9a`

### User stories

- `US-1.1.49`
- `US-1.1.50`

### Test cases

- `TC-1.1.1.1`
- `TC-1.1.1.2`
- `TC-1.1.2.1`
- `TC-1.1.2.2`
- `TC-1.1.3.1`
- `TC-1.1.3.2`
- `TC-1.1.4.1`
- `TC-1.1.4.2`
- `TC-1.1.5.1`
- `TC-1.1.6.1`
- `TC-1.1.7.1`
- `TC-1.1.8.1`
- `TC-1.1.8.2`
- `TC-1.1.9.1`
- `TC-1.1.9.2`
- `TC-1.1.10.1`
- `TC-1.1.10.2`
- `TC-1.1.11.1`
- `TC-1.1.11.2`
- `TC-1.1.12.1`
- `TC-1.1.13.1`
- `TC-1.1.14.1`
- `TC-1.1.15.1`
- `TC-1.1.15.2`
- `TC-1.1.16.1`
- `TC-1.1.16.2`
- `TC-1.1.16.3`
- `TC-1.1.17.1`
- `TC-1.1.17.2`
- `TC-1.1.17.3`
- `TC-1.1.18.1`
- `TC-1.1.19.1`
- `TC-1.1.21.1`
- `TC-1.1.22.1`
- `TC-1.1.22.2`
- `TC-1.1.23.1`
- `TC-1.1.24.1`
- `TC-1.1.25.1`
- `TC-1.1.25.2`
- `TC-1.1.26.1`
- `TC-1.1.27.1`
- `TC-1.1.28.1`
- `TC-1.1.29.1`
- `TC-1.1.30.1`
- `TC-1.1.31.1`
- `TC-1.1.32.1`
- `TC-1.1.33.1`
- `TC-1.1.34.1`
- `TC-1.1.35.1`
- `TC-1.1.35.2`
- `TC-1.1.36.1`
- `TC-1.1.37.1`
- `TC-1.1.38.1`

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

- `TC-1.1.1.1`
- `TC-1.1.1.2`
- `TC-1.1.2.1`
- `TC-1.1.2.2`
- `TC-1.1.3.1`
- `TC-1.1.3.2`
- `TC-1.1.4.1`
- `TC-1.1.4.2`
- `TC-1.1.5.1`
- `TC-1.1.6.1`
- `TC-1.1.7.1`
- `TC-1.1.8.1`
- `TC-1.1.8.2`
- `TC-1.1.9.1`
- `TC-1.1.9.2`
- `TC-1.1.10.1`
- `TC-1.1.10.2`
- `TC-1.1.11.1`
- `TC-1.1.11.2`
- `TC-1.1.12.1`
- `TC-1.1.13.1`
- `TC-1.1.14.1`
- `TC-1.1.15.1`
- `TC-1.1.15.2`
- `TC-1.1.16.1`
- `TC-1.1.16.2`
- `TC-1.1.16.3`
- `TC-1.1.17.1`
- `TC-1.1.17.2`
- `TC-1.1.17.3`
- `TC-1.1.18.1`
- `TC-1.1.19.1`
- `TC-1.1.21.1`
- `TC-1.1.22.1`
- `TC-1.1.22.2`
- `TC-1.1.23.1`
- `TC-1.1.24.1`
- `TC-1.1.25.1`
- `TC-1.1.25.2`
- `TC-1.1.26.1`
- `TC-1.1.27.1`
- `TC-1.1.28.1`
- `TC-1.1.29.1`
- `TC-1.1.30.1`
- `TC-1.1.31.1`
- `TC-1.1.32.1`
- `TC-1.1.33.1`
- `TC-1.1.34.1`
- `TC-1.1.35.1`
- `TC-1.1.35.2`
- `TC-1.1.36.1`
- `TC-1.1.37.1`
- `TC-1.1.38.1`

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

- `R-1.1.11a` resolved via parent `R-1.1.11` in
  [entity-component-system.md](../../requirements/core-runtime/entity-component-system.md).
- `R-1.1.16a` resolved via parent `R-1.1.16` in
  [entity-component-system.md](../../requirements/core-runtime/entity-component-system.md).
- `R-1.1.17a` resolved via parent `R-1.1.17` in
  [entity-component-system.md](../../requirements/core-runtime/entity-component-system.md).
- `R-1.1.1a` resolved via parent `R-1.1.1` in
  [entity-component-system.md](../../requirements/core-runtime/entity-component-system.md).
- `R-1.1.22a` resolved via parent `R-1.1.22` in
  [entity-component-system.md](../../requirements/core-runtime/entity-component-system.md).
- `R-1.1.25a` resolved via parent `R-1.1.25` in
  [entity-component-system.md](../../requirements/core-runtime/entity-component-system.md).
- `R-1.1.2a` resolved via parent `R-1.1.2` in
  [entity-component-system.md](../../requirements/core-runtime/entity-component-system.md).
- `R-1.1.30a` resolved via parent `R-1.1.30` in
  [entity-component-system.md](../../requirements/core-runtime/entity-component-system.md).
- `R-1.1.32a` resolved via parent `R-1.1.32` in
  [entity-component-system.md](../../requirements/core-runtime/entity-component-system.md).
- `R-1.1.35a` resolved via parent `R-1.1.35` in
  [entity-component-system.md](../../requirements/core-runtime/entity-component-system.md).
- `R-1.1.3a` resolved via parent `R-1.1.3` in
  [entity-component-system.md](../../requirements/core-runtime/entity-component-system.md).
- `R-1.1.9a` resolved via parent `R-1.1.9` in
  [entity-component-system.md](../../requirements/core-runtime/entity-component-system.md).

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
