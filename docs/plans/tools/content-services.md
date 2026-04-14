---
children: []
dependencies: []
design_documents:
  - docs/design/tools/content-services.md
execution_mode: sequential
features:
  - F-15.7.1
  - F-15.7.2
  - F-15.7.3
  - F-15.7.4
  - F-15.7.5
  - F-15.7.6
  - F-15.7.7
  - F-15.7.8
  - F-15.9.2
  - F-15.9.3
  - F-15.9.4
  - F-15.9.5
  - F-15.9.7
  - F-15.9.8
  - F-15.9.9
  - F-15.9.10
  - F-15.13.1
  - F-15.13.2
  - F-15.13.3
  - F-15.19.1
  - F-15.19.2
  - F-15.19.3
  - F-15.19.4
  - F-15.19.5
  - F-15.19.6
  - F-15.19.7
id: PLAN-tools-content-services
name: Content Services
parent: null
progress_file: docs/plans/progress/PLAN-tools-content-services.md
requirements:
  - R-15.7.1
  - R-15.7.2
  - R-15.7.3
  - R-15.7.4
  - R-15.7.5
  - R-15.7.6
  - R-15.7.7
  - R-15.7.8
  - R-15.9.2
  - R-15.9.3
  - R-15.9.4
  - R-15.9.5
  - R-15.9.7
  - R-15.9.8
  - R-15.9.9
  - R-15.9.10
  - R-15.9.1a
  - R-15.9.1b
  - R-15.9.1c
  - R-15.9.6a
  - R-15.9.6b
  - R-15.9.6c
  - R-15.13.1
  - R-15.13.2
  - R-15.13.3
  - R-15.19.1
  - R-15.19.2
  - R-15.19.3
  - R-15.19.4
  - R-15.19.5
  - R-15.19.6
  - R-15.19.7
status: not_started
test_cases:
  - TC-15.7.1.1
  - TC-15.7.1.2
  - TC-15.7.2.1
  - TC-15.7.2.2
  - TC-15.7.3.1
  - TC-15.7.4.1
  - TC-15.7.5.1
  - TC-15.7.5.2
  - TC-15.7.6.1
  - TC-15.7.6.2
  - TC-15.7.7.1
  - TC-15.7.7.2
  - TC-15.7.8.1
  - TC-15.9.2.1
  - TC-15.9.2.2
  - TC-15.9.3.1
  - TC-15.9.4.1
  - TC-15.9.5.1
  - TC-15.9.7.1
  - TC-15.9.8.1
  - TC-15.9.10.1
  - TC-15.9.10.2
  - TC-15.13.1.1
  - TC-15.13.1.2
  - TC-15.13.1.3
  - TC-15.13.1.4
  - TC-15.13.2.1
  - TC-15.13.2.2
  - TC-15.13.2.3
  - TC-15.13.3.1
  - TC-15.13.3.2
  - TC-15.13.3.3
  - TC-15.19.1.1
  - TC-15.19.2.1
  - TC-15.19.3.1
  - TC-15.19.4.1
  - TC-15.19.4.2
  - TC-15.19.5.1
  - TC-15.19.5.2
  - TC-15.19.6.1
  - TC-15.19.6.2
  - TC-15.19.7.1
  - TC-15.19.7.2
worktree_branch: plan/tools-content-services
---

# Content Services implementation plan

- Plan ID: `PLAN-tools-content-services`
- Progress file: [PLAN-tools-content-services.md](../progress/PLAN-tools-content-services.md)

## Source documents

- Design: [content-services.md](../../design/tools/content-services.md)
- Test cases: [content-services-test-cases.md](../../design/tools/content-services-test-cases.md)
- Progress: [PLAN-tools-content-services.md](../progress/PLAN-tools-content-services.md)

## Linked specification artifacts

### Features (`docs/features`)

- [ai-assistant.md](../../features/tools/ai-assistant.md) — covers `F-15.7.3`, `F-15.7.4`,
  `F-15.7.5`, `F-15.7.6`, `F-15.9.10`, `F-15.9.2`, `F-15.9.3`, `F-15.9.4`...
- [ai-cloud-backend.md](../../features/tools/ai-cloud-backend.md) — covers `F-15.7.1`, `F-15.7.4`
- [ai-governance.md](../../features/tools/ai-governance.md) — covers `F-15.7.1`, `F-15.7.2`,
  `F-15.7.3`, `F-15.7.4`, `F-15.7.5`, `F-15.7.6`, `F-15.7.7`, `F-15.7.8`
- [asset-store.md](../../features/tools/asset-store.md) — covers `F-15.7.1`, `F-15.7.6`, `F-15.7.7`
- [documentation.md](../../features/tools/documentation.md) — covers `F-15.19.1`, `F-15.19.2`,
  `F-15.19.3`, `F-15.19.4`, `F-15.19.5`, `F-15.19.6`, `F-15.19.7`
- [localization-editor.md](../../features/tools/localization-editor.md) — covers `F-15.13.1`,
  `F-15.13.2`, `F-15.13.3`
- [remote-editing.md](../../features/tools/remote-editing.md) — covers `F-15.7.1`
- [specialized-editors.md](../../features/tools/specialized-editors.md) — covers `F-15.13.1`

### Requirements (`docs/requirements`)

- [ai-assistant.md](../../requirements/tools/ai-assistant.md) — covers `R-15.9.10`, `R-15.9.1a`,
  `R-15.9.1b`, `R-15.9.1c`, `R-15.9.2`, `R-15.9.3`, `R-15.9.4`, `R-15.9.5`...
- [ai-governance.md](../../requirements/tools/ai-governance.md) — covers `R-15.7.1`, `R-15.7.2`,
  `R-15.7.3`, `R-15.7.4`, `R-15.7.5`, `R-15.7.6`, `R-15.7.7`, `R-15.7.8`
- [documentation.md](../../requirements/tools/documentation.md) — covers `R-15.19.1`, `R-15.19.2`,
  `R-15.19.3`, `R-15.19.4`, `R-15.19.5`, `R-15.19.6`, `R-15.19.7`
- [localization-editor.md](../../requirements/tools/localization-editor.md) — covers `R-15.13.1`,
  `R-15.13.2`, `R-15.13.3`

### User stories (`docs/user-stories`)

- [ai-assistant.md](../../user-stories/tools/ai-assistant.md) — covers `US-15.9.1`, `US-15.9.10`,
  `US-15.9.10.1`, `US-15.9.10.6`, `US-15.9.2`, `US-15.9.2.1`, `US-15.9.2.6`, `US-15.9.3`...
- [ai-governance.md](../../user-stories/tools/ai-governance.md) — covers `US-15.7.1`, `US-15.7.1.1`,
  `US-15.7.1.5`, `US-15.7.2`, `US-15.7.2.1`, `US-15.7.2.4`, `US-15.7.3`, `US-15.7.3.1`...
- [documentation.md](../../user-stories/tools/documentation.md) — covers `US-15.19.1`,
  `US-15.19.1.1`, `US-15.19.2`, `US-15.19.2.1`, `US-15.19.3`, `US-15.19.3.1`, `US-15.19.4`,
  `US-15.19.4.1`...
- [localization-editor.md](../../user-stories/tools/localization-editor.md) — covers `US-15.13.1`,
  `US-15.13.1.1`, `US-15.13.2`, `US-15.13.2.1`, `US-15.13.3`, `US-15.13.3.1`

### Test case sources

- [content-services-test-cases.md](../../design/tools/content-services-test-cases.md)

### Gap closure decisions

- Normalized `R-15.9.1a` to `R-15.9.1` using requirements parent-ID mapping.
- Normalized `R-15.9.1b` to `R-15.9.1` using requirements parent-ID mapping.
- Normalized `R-15.9.1c` to `R-15.9.1` using requirements parent-ID mapping.
- Normalized `R-15.9.6a` to `R-15.9.6` using requirements parent-ID mapping.
- Normalized `R-15.9.6b` to `R-15.9.6` using requirements parent-ID mapping.
- Normalized `R-15.9.6c` to `R-15.9.6` using requirements parent-ID mapping.
- Normalized `US-15.7.1.5` to `US-15.7.1` using user-stories parent-ID mapping.
- Normalized `US-15.7.2.4` to `US-15.7.2` using user-stories parent-ID mapping.
- Normalized `US-15.7.3.4` to `US-15.7.3` using user-stories parent-ID mapping.
- Normalized `US-15.7.4.4` to `US-15.7.4` using user-stories parent-ID mapping.
- Normalized `US-15.7.5.5` to `US-15.7.5` using user-stories parent-ID mapping.
- Normalized `US-15.7.6.4` to `US-15.7.6` using user-stories parent-ID mapping.
- Normalized `US-15.7.7.6` to `US-15.7.7` using user-stories parent-ID mapping.
- Normalized `US-15.7.8.4` to `US-15.7.8` using user-stories parent-ID mapping.
- Normalized `US-15.9.10.6` to `US-15.9.10` using user-stories parent-ID mapping.
- Normalized `US-15.9.2.6` to `US-15.9.2` using user-stories parent-ID mapping.
- Normalized `US-15.9.3.6` to `US-15.9.3` using user-stories parent-ID mapping.
- Normalized `US-15.9.4.4` to `US-15.9.4` using user-stories parent-ID mapping.
- Normalized `US-15.9.5.4` to `US-15.9.5` using user-stories parent-ID mapping.
- Normalized `US-15.9.7.5` to `US-15.9.7` using user-stories parent-ID mapping.
- All IDs in this plan are mapped to specification artifacts.

## Traceability coverage

### Features

- `F-15.7.1`
- `F-15.7.2`
- `F-15.7.3`
- `F-15.7.4`
- `F-15.7.5`
- `F-15.7.6`
- `F-15.7.7`
- `F-15.7.8`
- `F-15.9.2`
- `F-15.9.3`
- `F-15.9.4`
- `F-15.9.5`
- `F-15.9.7`
- `F-15.9.8`
- `F-15.9.9`
- `F-15.9.10`
- `F-15.13.1`
- `F-15.13.2`
- `F-15.13.3`
- `F-15.19.1`
- `F-15.19.2`
- `F-15.19.3`
- `F-15.19.4`
- `F-15.19.5`
- `F-15.19.6`
- `F-15.19.7`

### Requirements

- `R-15.7.1`
- `R-15.7.2`
- `R-15.7.3`
- `R-15.7.4`
- `R-15.7.5`
- `R-15.7.6`
- `R-15.7.7`
- `R-15.7.8`
- `R-15.9.2`
- `R-15.9.3`
- `R-15.9.4`
- `R-15.9.5`
- `R-15.9.7`
- `R-15.9.8`
- `R-15.9.9`
- `R-15.9.10`
- `R-15.9.1a`
- `R-15.9.1b`
- `R-15.9.1c`
- `R-15.9.6a`
- `R-15.9.6b`
- `R-15.9.6c`
- `R-15.13.1`
- `R-15.13.2`
- `R-15.13.3`
- `R-15.19.1`
- `R-15.19.2`
- `R-15.19.3`
- `R-15.19.4`
- `R-15.19.5`
- `R-15.19.6`
- `R-15.19.7`

### User stories

- `US-15.7.1`
- `US-15.7.1.1`
- `US-15.7.1.5`
- `US-15.7.2`
- `US-15.7.2.1`
- `US-15.7.2.4`
- `US-15.7.3`
- `US-15.7.3.1`
- `US-15.7.3.4`
- `US-15.7.4`
- `US-15.7.4.1`
- `US-15.7.4.4`
- `US-15.7.5`
- `US-15.7.5.1`
- `US-15.7.5.5`
- `US-15.7.6`
- `US-15.7.6.1`
- `US-15.7.6.4`
- `US-15.7.7`
- `US-15.7.7.1`
- `US-15.7.7.6`
- `US-15.7.8`
- `US-15.7.8.1`
- `US-15.7.8.4`
- `US-15.9.1`
- `US-15.9.2`
- `US-15.9.2.1`
- `US-15.9.2.6`
- `US-15.9.3`
- `US-15.9.3.1`
- `US-15.9.3.6`
- `US-15.9.4`
- `US-15.9.4.1`
- `US-15.9.4.4`
- `US-15.9.5`
- `US-15.9.5.1`
- `US-15.9.5.4`
- `US-15.9.6`
- `US-15.9.7`
- `US-15.9.7.1`
- `US-15.9.7.5`
- `US-15.9.8`
- `US-15.9.8.1`
- `US-15.9.8.5`
- `US-15.9.9`
- `US-15.9.9.1`
- `US-15.9.9.4`
- `US-15.9.10`
- `US-15.9.10.1`
- `US-15.9.10.6`
- `US-15.13.1`
- `US-15.13.1.1`
- `US-15.13.2`
- `US-15.13.2.1`
- `US-15.13.3`
- `US-15.13.3.1`
- `US-15.19.1`
- `US-15.19.1.1`
- `US-15.19.2`
- `US-15.19.2.1`
- `US-15.19.3`
- `US-15.19.3.1`
- `US-15.19.4`
- `US-15.19.4.1`
- `US-15.19.5`
- `US-15.19.5.1`
- `US-15.19.6`
- `US-15.19.6.1`
- `US-15.19.7`
- `US-15.19.7.1`

### Test cases

- `TC-15.7.1.1`
- `TC-15.7.1.2`
- `TC-15.7.2.1`
- `TC-15.7.2.2`
- `TC-15.7.3.1`
- `TC-15.7.4.1`
- `TC-15.7.5.1`
- `TC-15.7.5.2`
- `TC-15.7.6.1`
- `TC-15.7.6.2`
- `TC-15.7.7.1`
- `TC-15.7.7.2`
- `TC-15.7.8.1`
- `TC-15.9.2.1`
- `TC-15.9.2.2`
- `TC-15.9.3.1`
- `TC-15.9.4.1`
- `TC-15.9.5.1`
- `TC-15.9.7.1`
- `TC-15.9.8.1`
- `TC-15.9.10.1`
- `TC-15.9.10.2`
- `TC-15.13.1.1`
- `TC-15.13.1.2`
- `TC-15.13.1.3`
- `TC-15.13.1.4`
- `TC-15.13.2.1`
- `TC-15.13.2.2`
- `TC-15.13.2.3`
- `TC-15.13.3.1`
- `TC-15.13.3.2`
- `TC-15.13.3.3`
- `TC-15.19.1.1`
- `TC-15.19.2.1`
- `TC-15.19.3.1`
- `TC-15.19.4.1`
- `TC-15.19.4.2`
- `TC-15.19.5.1`
- `TC-15.19.5.2`
- `TC-15.19.6.1`
- `TC-15.19.6.2`
- `TC-15.19.7.1`
- `TC-15.19.7.2`

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

- `TC-15.7.1.1`
- `TC-15.7.1.2`
- `TC-15.7.2.1`
- `TC-15.7.2.2`
- `TC-15.7.3.1`
- `TC-15.7.4.1`
- `TC-15.7.5.1`
- `TC-15.7.5.2`
- `TC-15.7.6.1`
- `TC-15.7.6.2`
- `TC-15.7.7.1`
- `TC-15.7.7.2`
- `TC-15.7.8.1`
- `TC-15.9.2.1`
- `TC-15.9.2.2`
- `TC-15.9.3.1`
- `TC-15.9.4.1`
- `TC-15.9.5.1`
- `TC-15.9.7.1`
- `TC-15.9.8.1`
- `TC-15.9.10.1`
- `TC-15.9.10.2`
- `TC-15.13.1.1`
- `TC-15.13.1.2`
- `TC-15.13.1.3`
- `TC-15.13.1.4`
- `TC-15.13.2.1`
- `TC-15.13.2.2`
- `TC-15.13.2.3`
- `TC-15.13.3.1`
- `TC-15.13.3.2`
- `TC-15.13.3.3`
- `TC-15.19.1.1`
- `TC-15.19.2.1`
- `TC-15.19.3.1`
- `TC-15.19.4.1`
- `TC-15.19.4.2`
- `TC-15.19.5.1`
- `TC-15.19.5.2`
- `TC-15.19.6.1`
- `TC-15.19.6.2`
- `TC-15.19.7.1`
- `TC-15.19.7.2`

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

- `R-15.9.1a` resolved via parent `R-15.9.1` in
  [ai-assistant.md](../../requirements/tools/ai-assistant.md).
- `R-15.9.1b` resolved via parent `R-15.9.1` in
  [ai-assistant.md](../../requirements/tools/ai-assistant.md).
- `R-15.9.1c` resolved via parent `R-15.9.1` in
  [ai-assistant.md](../../requirements/tools/ai-assistant.md).
- `R-15.9.6a` resolved via parent `R-15.9.6` in
  [ai-assistant.md](../../requirements/tools/ai-assistant.md).
- `R-15.9.6b` resolved via parent `R-15.9.6` in
  [ai-assistant.md](../../requirements/tools/ai-assistant.md).
- `R-15.9.6c` resolved via parent `R-15.9.6` in
  [ai-assistant.md](../../requirements/tools/ai-assistant.md).
- `US-15.7.1.5` resolved via parent `US-15.7.1` in
  [ai-governance.md](../../user-stories/tools/ai-governance.md).
- `US-15.7.2.4` resolved via parent `US-15.7.2` in
  [ai-governance.md](../../user-stories/tools/ai-governance.md).
- `US-15.7.3.4` resolved via parent `US-15.7.3` in
  [ai-governance.md](../../user-stories/tools/ai-governance.md).
- `US-15.7.4.4` resolved via parent `US-15.7.4` in
  [ai-governance.md](../../user-stories/tools/ai-governance.md).
- `US-15.7.5.5` resolved via parent `US-15.7.5` in
  [ai-governance.md](../../user-stories/tools/ai-governance.md).
- `US-15.7.6.4` resolved via parent `US-15.7.6` in
  [ai-governance.md](../../user-stories/tools/ai-governance.md).
- `US-15.7.7.6` resolved via parent `US-15.7.7` in
  [ai-governance.md](../../user-stories/tools/ai-governance.md).
- `US-15.7.8.4` resolved via parent `US-15.7.8` in
  [ai-governance.md](../../user-stories/tools/ai-governance.md).
- `US-15.9.10.6` resolved via parent `US-15.9.10` in
  [ai-assistant.md](../../user-stories/tools/ai-assistant.md).
- `US-15.9.2.6` resolved via parent `US-15.9.2` in
  [ai-assistant.md](../../user-stories/tools/ai-assistant.md).
- `US-15.9.3.6` resolved via parent `US-15.9.3` in
  [ai-assistant.md](../../user-stories/tools/ai-assistant.md).
- `US-15.9.4.4` resolved via parent `US-15.9.4` in
  [ai-assistant.md](../../user-stories/tools/ai-assistant.md).
- `US-15.9.5.4` resolved via parent `US-15.9.5` in
  [ai-assistant.md](../../user-stories/tools/ai-assistant.md).
- `US-15.9.7.5` resolved via parent `US-15.9.7` in
  [ai-assistant.md](../../user-stories/tools/ai-assistant.md).
- `US-15.9.8.5` resolved via parent `US-15.9.8` in
  [ai-assistant.md](../../user-stories/tools/ai-assistant.md).
- `US-15.9.9.4` resolved via parent `US-15.9.9` in
  [ai-assistant.md](../../user-stories/tools/ai-assistant.md).

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
