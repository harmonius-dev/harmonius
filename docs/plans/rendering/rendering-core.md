---
children: []
dependencies: []
design_documents:
  - docs/design/rendering/rendering-core.md
execution_mode: sequential
features:
  - F-2.3.1
  - F-2.3.2
  - F-2.3.3
  - F-2.3.4
  - F-2.3.5
  - F-2.3.6
  - F-2.3.7
  - F-2.3.8
  - F-2.3.9
  - F-2.3.10
  - F-2.3.11
  - F-2.3.12
  - F-2.3.13
  - F-2.4.1
  - F-2.4.2
  - F-2.4.10
  - F-2.4.11
  - F-2.4.12
  - F-2.4.13
  - F-2.4.14
  - F-2.4.15
  - F-2.4.16
  - F-2.4.17
  - F-2.4.18
  - F-2.4.19
  - F-2.4.20
  - F-2.4.21
  - F-2.4.22
  - F-2.4.23
  - F-2.10.1
  - F-2.10.2
  - F-2.10.3
  - F-2.10.4
  - F-2.10.5
  - F-2.10.6
  - F-2.10.7
  - F-2.10.8
  - F-2.10.9
  - F-2.10.10
id: PLAN-rendering-rendering-core
name: Rendering Core
parent: null
progress_file: docs/plans/progress/PLAN-rendering-rendering-core.md
requirements:
  - R-2.3.1
  - R-2.3.2
  - R-2.3.3
  - R-2.3.4
  - R-2.3.5
  - R-2.3.6
  - R-2.3.7
  - R-2.3.8
  - R-2.3.9
  - R-2.3.10
  - R-2.3.11
  - R-2.3.12
  - R-2.3.13
  - R-2.3.14
  - R-2.4.1
  - R-2.4.2
  - R-2.4.3
  - R-2.4.10
  - R-2.4.11
  - R-2.4.12
  - R-2.4.13
  - R-2.4.14
  - R-2.4.15
  - R-2.4.16
  - R-2.4.17
  - R-2.4.18
  - R-2.4.19
  - R-2.4.20
  - R-2.4.21
  - R-2.4.22
  - R-2.4.23
  - R-2.4.24
  - R-2.10.1
  - R-2.10.2
  - R-2.10.3
  - R-2.10.4
  - R-2.10.5
  - R-2.10.6
  - R-2.10.7
  - R-2.10.8
  - R-2.10.9
  - R-2.10.10
  - R-2.10.4a
status: not_started
test_cases:
  - TC-2.3.1.1
  - TC-2.3.1.2
  - TC-2.3.2.1
  - TC-2.3.2.2
  - TC-2.3.3.1
  - TC-2.3.4.1
  - TC-2.3.4.2
  - TC-2.3.5.1
  - TC-2.3.6.1
  - TC-2.3.7.1
  - TC-2.3.8.1
  - TC-2.3.9.1
  - TC-2.3.10.1
  - TC-2.3.11.1
  - TC-2.3.12.1
  - TC-2.3.13.1
  - TC-2.3.14.1
  - TC-2.4.1.1
  - TC-2.4.2.1
  - TC-2.4.3.1
  - TC-2.4.10.1
  - TC-2.4.11.1
  - TC-2.4.12.1
  - TC-2.4.13.1
  - TC-2.4.14.1
  - TC-2.4.15.1
  - TC-2.4.16.1
  - TC-2.4.17.1
  - TC-2.4.18.1
  - TC-2.4.19.1
  - TC-2.4.20.1
  - TC-2.4.21.1
  - TC-2.4.22.1
  - TC-2.4.23.1
  - TC-2.4.24.1
  - TC-2.10.1.1
  - TC-2.10.2.1
  - TC-2.10.3.1
  - TC-2.10.4.1
  - TC-2.10.4.2
  - TC-2.10.5.1
  - TC-2.10.6.1
  - TC-2.10.7.1
  - TC-2.10.8.1
  - TC-2.10.9.1
  - TC-2.10.10.1
worktree_branch: plan/rendering-rendering-core
---

# Rendering Core implementation plan

- Plan ID: `PLAN-rendering-rendering-core`
- Progress file: [PLAN-rendering-rendering-core.md](../progress/PLAN-rendering-rendering-core.md)

## Source documents

- Design: [rendering-core.md](../../design/rendering/rendering-core.md)
- Test cases: [rendering-core-test-cases.md](../../design/rendering/rendering-core-test-cases.md)
- Progress: [PLAN-rendering-rendering-core.md](../progress/PLAN-rendering-rendering-core.md)

## Linked specification artifacts

### Features (`docs/features`)

- [advanced-materials.md](../../features/rendering/advanced-materials.md) — covers `F-2.10.5`,
  `F-2.3.1`
- [advanced-rendering.md](../../features/rendering/advanced-rendering.md) — covers `F-2.4.2`
- [anti-aliasing-upscaling.md](../../features/rendering/anti-aliasing-upscaling.md) — covers
  `F-2.4.1`
- [core-rendering.md](../../features/rendering/core-rendering.md) — covers `F-2.3.1`, `F-2.3.10`,
  `F-2.3.11`, `F-2.3.12`, `F-2.3.13`, `F-2.3.2`, `F-2.3.3`, `F-2.3.4`...
- [environment.md](../../features/rendering/environment.md) — covers `F-2.3.2`, `F-2.3.7`
- [first-person-rendering.md](../../features/rendering/first-person-rendering.md) — covers
  `F-2.10.1`, `F-2.10.4`
- [lighting.md](../../features/rendering/lighting.md) — covers `F-2.3.1`, `F-2.3.9`, `F-2.4.1`,
  `F-2.4.10`, `F-2.4.11`, `F-2.4.12`, `F-2.4.13`, `F-2.4.14`...
- [post-processing.md](../../features/rendering/post-processing.md) — covers `F-2.4.2`
- [scene-rendering-pipeline.md](../../features/rendering/scene-rendering-pipeline.md) — covers
  `F-2.10.1`, `F-2.10.10`, `F-2.10.2`, `F-2.10.3`, `F-2.10.4`, `F-2.10.5`, `F-2.10.6`, `F-2.10.7`...
- [stylized-effects.md](../../features/rendering/stylized-effects.md) — covers `F-2.10.1`,
  `F-2.10.5`, `F-2.3.1`

### Requirements (`docs/requirements`)

- [core-rendering.md](../../requirements/rendering/core-rendering.md) — covers `R-2.3.1`,
  `R-2.3.10`, `R-2.3.11`, `R-2.3.12`, `R-2.3.13`, `R-2.3.2`, `R-2.3.3`, `R-2.3.4`...
- [lighting.md](../../requirements/rendering/lighting.md) — covers `R-2.4.1`, `R-2.4.10`,
  `R-2.4.11`, `R-2.4.12`, `R-2.4.13`, `R-2.4.14`, `R-2.4.15`, `R-2.4.16`...
- [scene-rendering-pipeline.md](../../requirements/rendering/scene-rendering-pipeline.md) — covers
  `R-2.10.1`, `R-2.10.2`, `R-2.10.3`, `R-2.10.4`, `R-2.10.4a`, `R-2.10.5`, `R-2.10.6`, `R-2.10.7`...
- Still previously unmapped IDs: `R-2.10.10`, `R-2.4.18`, `R-2.4.19`, `R-2.4.20`, `R-2.4.21`,
  `R-2.4.22`, `R-2.4.23`

### User stories (`docs/user-stories`)

- [core-rendering.md](../../user-stories/rendering/core-rendering.md) — covers `US-2.3.1.1`,
  `US-2.3.1.2`, `US-2.3.1.3`, `US-2.3.10.1`, `US-2.3.10.2`, `US-2.3.11.1`, `US-2.3.11.2`,
  `US-2.3.11.3`...
- [lighting.md](../../user-stories/rendering/lighting.md) — covers `US-2.4.1.1`, `US-2.4.1.2`,
  `US-2.4.10.1`, `US-2.4.10.2`, `US-2.4.11.1`, `US-2.4.11.2`, `US-2.4.12.1`, `US-2.4.12.2`...
- [scene-rendering-pipeline.md](../../user-stories/rendering/scene-rendering-pipeline.md) — covers
  `US-2.10.1.1`, `US-2.10.1.2`, `US-2.10.10.1`, `US-2.10.10.2`, `US-2.10.2.1`, `US-2.10.2.2`,
  `US-2.10.3.1`, `US-2.10.3.2`...

### Test case sources

- [rendering-core-test-cases.md](../../design/rendering/rendering-core-test-cases.md)

### Gap closure decisions

- Normalized `US-2.10.2.2` to `US-2.10.2` using user-stories parent-ID mapping.
- Normalized `US-2.10.3.3` to `US-2.10.3` using user-stories parent-ID mapping.
- Normalized `US-2.3.1.3` to `US-2.3.1` using user-stories parent-ID mapping.
- Normalized `US-2.3.11.3` to `US-2.3.11` using user-stories parent-ID mapping.
- Normalized `US-2.3.3.2` to `US-2.3.3` using user-stories parent-ID mapping.
- Normalized `US-2.3.4.3` to `US-2.3.4` using user-stories parent-ID mapping.
- Normalized `US-2.3.5.2` to `US-2.3.5` using user-stories parent-ID mapping.
- Normalized `US-2.3.6.2` to `US-2.3.6` using user-stories parent-ID mapping.
- Normalized `US-2.3.7.3` to `US-2.3.7` using user-stories parent-ID mapping.
- Normalized `US-2.4.15.2` to `US-2.4.15` using user-stories parent-ID mapping.
- Normalized `US-2.4.16.2` to `US-2.4.16` using user-stories parent-ID mapping.
- Normalized `US-2.4.17.2` to `US-2.4.17` using user-stories parent-ID mapping.
- Normalized `US-2.4.19.2` to `US-2.4.19` using user-stories parent-ID mapping.
- Normalized `US-2.4.21.2` to `US-2.4.21` using user-stories parent-ID mapping.
- Normalized `US-2.4.22.2` to `US-2.4.22` using user-stories parent-ID mapping.
- Remaining previously unmapped IDs are implementation-tracked in progress evidence as derived
  acceptance checks until upstream artifacts are added.

## Traceability coverage

### Features

- `F-2.3.1`
- `F-2.3.2`
- `F-2.3.3`
- `F-2.3.4`
- `F-2.3.5`
- `F-2.3.6`
- `F-2.3.7`
- `F-2.3.8`
- `F-2.3.9`
- `F-2.3.10`
- `F-2.3.11`
- `F-2.3.12`
- `F-2.3.13`
- `F-2.4.1`
- `F-2.4.2`
- `F-2.4.10`
- `F-2.4.11`
- `F-2.4.12`
- `F-2.4.13`
- `F-2.4.14`
- `F-2.4.15`
- `F-2.4.16`
- `F-2.4.17`
- `F-2.4.18`
- `F-2.4.19`
- `F-2.4.20`
- `F-2.4.21`
- `F-2.4.22`
- `F-2.4.23`
- `F-2.10.1`
- `F-2.10.2`
- `F-2.10.3`
- `F-2.10.4`
- `F-2.10.5`
- `F-2.10.6`
- `F-2.10.7`
- `F-2.10.8`
- `F-2.10.9`
- `F-2.10.10`

### Requirements

- `R-2.3.1`
- `R-2.3.2`
- `R-2.3.3`
- `R-2.3.4`
- `R-2.3.5`
- `R-2.3.6`
- `R-2.3.7`
- `R-2.3.8`
- `R-2.3.9`
- `R-2.3.10`
- `R-2.3.11`
- `R-2.3.12`
- `R-2.3.13`
- `R-2.3.14`
- `R-2.4.1`
- `R-2.4.2`
- `R-2.4.3`
- `R-2.4.10`
- `R-2.4.11`
- `R-2.4.12`
- `R-2.4.13`
- `R-2.4.14`
- `R-2.4.15`
- `R-2.4.16`
- `R-2.4.17`
- `R-2.4.18`
- `R-2.4.19`
- `R-2.4.20`
- `R-2.4.21`
- `R-2.4.22`
- `R-2.4.23`
- `R-2.4.24`
- `R-2.10.1`
- `R-2.10.2`
- `R-2.10.3`
- `R-2.10.4`
- `R-2.10.5`
- `R-2.10.6`
- `R-2.10.7`
- `R-2.10.8`
- `R-2.10.9`
- `R-2.10.10`
- `R-2.10.4a`

### User stories

- `US-2.3.1.1`
- `US-2.3.1.2`
- `US-2.3.1.3`
- `US-2.3.2.1`
- `US-2.3.2.2`
- `US-2.3.3.1`
- `US-2.3.3.2`
- `US-2.3.4.1`
- `US-2.3.4.2`
- `US-2.3.4.3`
- `US-2.3.5.1`
- `US-2.3.5.2`
- `US-2.3.6.1`
- `US-2.3.6.2`
- `US-2.3.7.1`
- `US-2.3.7.2`
- `US-2.3.7.3`
- `US-2.3.8.1`
- `US-2.3.8.2`
- `US-2.3.9.1`
- `US-2.3.9.2`
- `US-2.3.10.1`
- `US-2.3.10.2`
- `US-2.3.11.1`
- `US-2.3.11.2`
- `US-2.3.11.3`
- `US-2.3.12.1`
- `US-2.3.12.2`
- `US-2.3.13.1`
- `US-2.3.13.2`
- `US-2.4.1.1`
- `US-2.4.1.2`
- `US-2.4.2.1`
- `US-2.4.2.2`
- `US-2.4.10.1`
- `US-2.4.10.2`
- `US-2.4.11.1`
- `US-2.4.11.2`
- `US-2.4.12.1`
- `US-2.4.12.2`
- `US-2.4.13.1`
- `US-2.4.13.2`
- `US-2.4.14.1`
- `US-2.4.14.2`
- `US-2.4.15.1`
- `US-2.4.15.2`
- `US-2.4.16.1`
- `US-2.4.16.2`
- `US-2.4.17.1`
- `US-2.4.17.2`
- `US-2.4.18.1`
- `US-2.4.18.2`
- `US-2.4.19.1`
- `US-2.4.19.2`
- `US-2.4.20.1`
- `US-2.4.20.2`
- `US-2.4.21.1`
- `US-2.4.21.2`
- `US-2.4.22.1`
- `US-2.4.22.2`
- `US-2.4.23.1`
- `US-2.4.23.2`
- `US-2.10.1.1`
- `US-2.10.1.2`
- `US-2.10.2.1`
- `US-2.10.2.2`
- `US-2.10.3.1`
- `US-2.10.3.2`
- `US-2.10.3.3`
- `US-2.10.4.1`
- `US-2.10.4.2`
- `US-2.10.5.1`
- `US-2.10.5.2`
- `US-2.10.6.1`
- `US-2.10.6.2`
- `US-2.10.7.1`
- `US-2.10.7.2`
- `US-2.10.8.1`
- `US-2.10.8.2`
- `US-2.10.9.1`
- `US-2.10.9.2`
- `US-2.10.10.1`
- `US-2.10.10.2`

### Test cases

- `TC-2.3.1.1`
- `TC-2.3.1.2`
- `TC-2.3.2.1`
- `TC-2.3.2.2`
- `TC-2.3.3.1`
- `TC-2.3.4.1`
- `TC-2.3.4.2`
- `TC-2.3.5.1`
- `TC-2.3.6.1`
- `TC-2.3.7.1`
- `TC-2.3.8.1`
- `TC-2.3.9.1`
- `TC-2.3.10.1`
- `TC-2.3.11.1`
- `TC-2.3.12.1`
- `TC-2.3.13.1`
- `TC-2.3.14.1`
- `TC-2.4.1.1`
- `TC-2.4.2.1`
- `TC-2.4.3.1`
- `TC-2.4.10.1`
- `TC-2.4.11.1`
- `TC-2.4.12.1`
- `TC-2.4.13.1`
- `TC-2.4.14.1`
- `TC-2.4.15.1`
- `TC-2.4.16.1`
- `TC-2.4.17.1`
- `TC-2.4.18.1`
- `TC-2.4.19.1`
- `TC-2.4.20.1`
- `TC-2.4.21.1`
- `TC-2.4.22.1`
- `TC-2.4.23.1`
- `TC-2.4.24.1`
- `TC-2.10.1.1`
- `TC-2.10.2.1`
- `TC-2.10.3.1`
- `TC-2.10.4.1`
- `TC-2.10.4.2`
- `TC-2.10.5.1`
- `TC-2.10.6.1`
- `TC-2.10.7.1`
- `TC-2.10.8.1`
- `TC-2.10.9.1`
- `TC-2.10.10.1`

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

- `TC-2.3.1.1`
- `TC-2.3.1.2`
- `TC-2.3.2.1`
- `TC-2.3.2.2`
- `TC-2.3.3.1`
- `TC-2.3.4.1`
- `TC-2.3.4.2`
- `TC-2.3.5.1`
- `TC-2.3.6.1`
- `TC-2.3.7.1`
- `TC-2.3.8.1`
- `TC-2.3.9.1`
- `TC-2.3.10.1`
- `TC-2.3.11.1`
- `TC-2.3.12.1`
- `TC-2.3.13.1`
- `TC-2.3.14.1`
- `TC-2.4.1.1`
- `TC-2.4.2.1`
- `TC-2.4.3.1`
- `TC-2.4.10.1`
- `TC-2.4.11.1`
- `TC-2.4.12.1`
- `TC-2.4.13.1`
- `TC-2.4.14.1`
- `TC-2.4.15.1`
- `TC-2.4.16.1`
- `TC-2.4.17.1`
- `TC-2.4.18.1`
- `TC-2.4.19.1`
- `TC-2.4.20.1`
- `TC-2.4.21.1`
- `TC-2.4.22.1`
- `TC-2.4.23.1`
- `TC-2.4.24.1`
- `TC-2.10.1.1`
- `TC-2.10.2.1`
- `TC-2.10.3.1`
- `TC-2.10.4.1`
- `TC-2.10.4.2`
- `TC-2.10.5.1`
- `TC-2.10.6.1`
- `TC-2.10.7.1`
- `TC-2.10.8.1`
- `TC-2.10.9.1`
- `TC-2.10.10.1`

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

- `US-2.10.2.2` resolved via parent `US-2.10.2` in
  [scene-rendering-pipeline.md](../../user-stories/rendering/scene-rendering-pipeline.md).
- `US-2.10.3.3` resolved via parent `US-2.10.3` in
  [scene-rendering-pipeline.md](../../user-stories/rendering/scene-rendering-pipeline.md).
- `US-2.3.1.3` resolved via parent `US-2.3.1` in
  [core-rendering.md](../../user-stories/rendering/core-rendering.md).
- `US-2.3.11.3` resolved via parent `US-2.3.11` in
  [core-rendering.md](../../user-stories/rendering/core-rendering.md).
- `US-2.3.3.2` resolved via parent `US-2.3.3` in
  [core-rendering.md](../../user-stories/rendering/core-rendering.md).
- `US-2.3.4.3` resolved via parent `US-2.3.4` in
  [core-rendering.md](../../user-stories/rendering/core-rendering.md).
- `US-2.3.5.2` resolved via parent `US-2.3.5` in
  [core-rendering.md](../../user-stories/rendering/core-rendering.md).
- `US-2.3.6.2` resolved via parent `US-2.3.6` in
  [core-rendering.md](../../user-stories/rendering/core-rendering.md).
- `US-2.3.7.3` resolved via parent `US-2.3.7` in
  [core-rendering.md](../../user-stories/rendering/core-rendering.md).
- `US-2.4.15.2` resolved via parent `US-2.4.15` in
  [lighting.md](../../user-stories/rendering/lighting.md).
- `US-2.4.16.2` resolved via parent `US-2.4.16` in
  [lighting.md](../../user-stories/rendering/lighting.md).
- `US-2.4.17.2` resolved via parent `US-2.4.17` in
  [lighting.md](../../user-stories/rendering/lighting.md).
- `US-2.4.19.2` resolved via parent `US-2.4.19` in
  [lighting.md](../../user-stories/rendering/lighting.md).
- `US-2.4.21.2` resolved via parent `US-2.4.21` in
  [lighting.md](../../user-stories/rendering/lighting.md).
- `US-2.4.22.2` resolved via parent `US-2.4.22` in
  [lighting.md](../../user-stories/rendering/lighting.md).

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
