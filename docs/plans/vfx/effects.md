---
children: []
dependencies: []
design_documents:
  - docs/design/vfx/effects.md
execution_mode: sequential
features:
  - F-10.1.10
  - F-11.1.1
  - F-11.2.1
  - F-11.2.2
  - F-11.2.3
  - F-11.2.4
  - F-11.2.5
  - F-11.2.6
  - F-11.3.1
  - F-11.3.2
  - F-11.3.3
  - F-11.3.4
  - F-11.3.5
  - F-11.3.6
  - F-11.4.1
  - F-11.4.2
  - F-11.4.3
  - F-11.4.4
  - F-11.4.5
  - F-11.4.6
  - F-11.4.7
  - F-11.5.1
  - F-11.5.2
  - F-11.5.3
  - F-11.5.4
  - F-11.5.5
  - F-11.5.6
  - F-11.5.7
  - F-11.6.1
  - F-11.6.2
  - F-11.6.3
  - F-11.6.4
  - F-11.6.5
  - F-15.8.1
  - F-15.8.12
id: PLAN-vfx-effects
name: Effects
parent: null
progress_file: docs/plans/progress/PLAN-vfx-effects.md
requirements:
  - R-11.1.1
  - R-11.1.3
  - R-11.2.1
  - R-11.2.2
  - R-11.2.3
  - R-11.2.4
  - R-11.2.5
  - R-11.2.6
  - R-11.3.1
  - R-11.3.2
  - R-11.3.3
  - R-11.3.4
  - R-11.3.5
  - R-11.3.6
  - R-11.4.1
  - R-11.4.2
  - R-11.4.3
  - R-11.4.4
  - R-11.4.5
  - R-11.4.6
  - R-11.4.7
  - R-11.5.1
  - R-11.5.2
  - R-11.5.3
  - R-11.5.4
  - R-11.5.5
  - R-11.5.6
  - R-11.5.7
  - R-11.6.1
  - R-11.6.2
  - R-11.6.3
  - R-11.6.4
  - R-11.6.5
  - R-15.8.1
  - R-15.8.5b
status: not_started
test_cases:
  - TC-11.1.1.1
  - TC-11.1.3.1
  - TC-11.2.2.1
  - TC-11.2.3.1
  - TC-11.2.3.2
  - TC-11.2.4.1
  - TC-11.2.4.2
  - TC-11.2.4.3
  - TC-11.2.5.1
  - TC-11.3.1.1
  - TC-11.3.1.2
  - TC-11.3.1.3
  - TC-11.3.4.1
  - TC-11.3.6.1
  - TC-11.3.6.2
  - TC-11.4.1.1
  - TC-11.4.2.1
  - TC-11.4.2.2
  - TC-11.4.3.1
  - TC-11.4.5.1
  - TC-11.4.6.1
  - TC-11.4.7.1
  - TC-11.5.0.1
  - TC-11.5.1.1
  - TC-11.5.2.1
  - TC-11.5.3.1
  - TC-11.5.4.1
  - TC-11.5.5.1
  - TC-11.5.6.1
  - TC-11.5.7.1
  - TC-11.6.1.1
  - TC-11.6.2.1
  - TC-11.6.3.1
  - TC-11.6.4.1
  - TC-11.6.5.1
  - TC-15.8.1.1
worktree_branch: plan/vfx-effects
---

# Effects implementation plan

- Plan ID: `PLAN-vfx-effects`
- Progress file: [PLAN-vfx-effects.md](../progress/PLAN-vfx-effects.md)

## Source documents

- Design: [effects.md](../../design/vfx/effects.md)
- Test cases: [effects-test-cases.md](../../design/vfx/effects-test-cases.md)
- Progress: [PLAN-vfx-effects.md](../progress/PLAN-vfx-effects.md)

## Linked specification artifacts

### Features (`docs/features`)

- [hot-reload.md](../../features/content-pipeline/hot-reload.md) — covers `F-15.8.12`
- [scripting.md](../../features/game-framework/scripting.md) — covers `F-15.8.12`
- [deployment.md](../../features/tools/deployment.md) — covers `F-15.8.12`
- [editor-plugins.md](../../features/tools/editor-plugins.md) — covers `F-15.8.12`
- [logic-graph.md](../../features/tools/logic-graph.md) — covers `F-15.8.12`
- [shared-cache.md](../../features/tools/shared-cache.md) — covers `F-15.8.12`
- [specialized-editors.md](../../features/tools/specialized-editors.md) — covers `F-15.8.12`
- [widget-framework.md](../../features/ui/widget-framework.md) — covers `F-10.1.10`
- [decals.md](../../features/vfx/decals.md) — covers `F-11.2.1`, `F-11.2.2`, `F-11.2.3`, `F-11.2.4`,
  `F-11.2.5`, `F-11.2.6`
- [destruction.md](../../features/vfx/destruction.md) — covers `F-11.1.1`, `F-11.2.1`, `F-11.2.5`,
  `F-11.2.6`, `F-11.3.1`, `F-11.3.5`, `F-11.5.1`, `F-11.5.2`...
- [effect-graph.md](../../features/vfx/effect-graph.md) — covers `F-11.1.1`, `F-11.6.1`, `F-11.6.2`,
  `F-11.6.3`, `F-11.6.4`, `F-11.6.5`, `F-15.8.1`
- [particles.md](../../features/vfx/particles.md) — covers `F-11.1.1`
- [screen-effects.md](../../features/vfx/screen-effects.md) — covers `F-11.3.1`, `F-11.3.2`,
  `F-11.3.3`, `F-11.3.4`, `F-11.3.5`, `F-11.3.6`
- [weather-environmental.md](../../features/vfx/weather-environmental.md) — covers `F-11.1.1`,
  `F-11.3.3`, `F-11.4.1`, `F-11.4.2`, `F-11.4.3`, `F-11.4.4`, `F-11.4.5`, `F-11.4.6`...

### Requirements (`docs/requirements`)

- [logic-graph.md](../../requirements/tools/logic-graph.md) — covers `R-15.8.1`, `R-15.8.5b`
- [decals.md](../../requirements/vfx/decals.md) — covers `R-11.2.1`, `R-11.2.2`, `R-11.2.3`,
  `R-11.2.4`, `R-11.2.5`, `R-11.2.6`
- [destruction.md](../../requirements/vfx/destruction.md) — covers `R-11.5.1`, `R-11.5.2`,
  `R-11.5.3`, `R-11.5.4`, `R-11.5.5`, `R-11.5.6`, `R-11.5.7`
- [effect-graph.md](../../requirements/vfx/effect-graph.md) — covers `R-11.6.1`, `R-11.6.2`,
  `R-11.6.3`, `R-11.6.4`, `R-11.6.5`
- [particles.md](../../requirements/vfx/particles.md) — covers `R-11.1.1`, `R-11.1.3`
- [screen-effects.md](../../requirements/vfx/screen-effects.md) — covers `R-11.3.1`, `R-11.3.2`,
  `R-11.3.3`, `R-11.3.4`, `R-11.3.5`, `R-11.3.6`
- [weather-environmental.md](../../requirements/vfx/weather-environmental.md) — covers `R-11.4.1`,
  `R-11.4.2`, `R-11.4.3`, `R-11.4.4`, `R-11.4.5`, `R-11.4.6`, `R-11.4.7`

### User stories (`docs/user-stories`)

- [decals.md](../../user-stories/vfx/decals.md) — covers `US-11.2.1.1`, `US-11.2.1.3`,
  `US-11.2.2.1`, `US-11.2.2.2`, `US-11.2.3.1`, `US-11.2.3.2`, `US-11.2.4.1`, `US-11.2.4.2`...
- [destruction.md](../../user-stories/vfx/destruction.md) — covers `US-11.5.1.1`, `US-11.5.1.2`,
  `US-11.5.1.3`, `US-11.5.2.1`, `US-11.5.2.2`, `US-11.5.2.3`, `US-11.5.3.1`, `US-11.5.3.2`...
- [screen-effects.md](../../user-stories/vfx/screen-effects.md) — covers `US-11.3.1.1`,
  `US-11.3.1.2`, `US-11.3.1.3`, `US-11.3.2.1`, `US-11.3.2.2`, `US-11.3.2.3`, `US-11.3.3.1`,
  `US-11.3.3.2`...
- [weather-environmental.md](../../user-stories/vfx/weather-environmental.md) — covers
  `US-11.4.1.1`, `US-11.4.1.2`, `US-11.4.1.3`, `US-11.4.2.1`, `US-11.4.2.2`, `US-11.4.2.3`,
  `US-11.4.3.1`, `US-11.4.3.2`...

### Test case sources

- [effects-test-cases.md](../../design/vfx/effects-test-cases.md)

### Gap closure decisions

- Normalized `R-15.8.5b` to `R-15.8.5` using requirements parent-ID mapping.
- All IDs in this plan are mapped to specification artifacts.

## Traceability coverage

### Features

- `F-10.1.10`
- `F-11.1.1`
- `F-11.2.1`
- `F-11.2.2`
- `F-11.2.3`
- `F-11.2.4`
- `F-11.2.5`
- `F-11.2.6`
- `F-11.3.1`
- `F-11.3.2`
- `F-11.3.3`
- `F-11.3.4`
- `F-11.3.5`
- `F-11.3.6`
- `F-11.4.1`
- `F-11.4.2`
- `F-11.4.3`
- `F-11.4.4`
- `F-11.4.5`
- `F-11.4.6`
- `F-11.4.7`
- `F-11.5.1`
- `F-11.5.2`
- `F-11.5.3`
- `F-11.5.4`
- `F-11.5.5`
- `F-11.5.6`
- `F-11.5.7`
- `F-11.6.1`
- `F-11.6.2`
- `F-11.6.3`
- `F-11.6.4`
- `F-11.6.5`
- `F-15.8.1`
- `F-15.8.12`

### Requirements

- `R-11.1.1`
- `R-11.1.3`
- `R-11.2.1`
- `R-11.2.2`
- `R-11.2.3`
- `R-11.2.4`
- `R-11.2.5`
- `R-11.2.6`
- `R-11.3.1`
- `R-11.3.2`
- `R-11.3.3`
- `R-11.3.4`
- `R-11.3.5`
- `R-11.3.6`
- `R-11.4.1`
- `R-11.4.2`
- `R-11.4.3`
- `R-11.4.4`
- `R-11.4.5`
- `R-11.4.6`
- `R-11.4.7`
- `R-11.5.1`
- `R-11.5.2`
- `R-11.5.3`
- `R-11.5.4`
- `R-11.5.5`
- `R-11.5.6`
- `R-11.5.7`
- `R-11.6.1`
- `R-11.6.2`
- `R-11.6.3`
- `R-11.6.4`
- `R-11.6.5`
- `R-15.8.1`
- `R-15.8.5b`

### User stories

- `US-11.2.1.1`
- `US-11.2.1.3`
- `US-11.2.2.1`
- `US-11.2.2.2`
- `US-11.2.3.1`
- `US-11.2.3.2`
- `US-11.2.4.1`
- `US-11.2.4.2`
- `US-11.2.4.3`
- `US-11.2.5.1`
- `US-11.2.5.2`
- `US-11.2.6.1`
- `US-11.2.6.2`
- `US-11.2.6.3`
- `US-11.3.1.1`
- `US-11.3.1.2`
- `US-11.3.1.3`
- `US-11.3.2.1`
- `US-11.3.2.2`
- `US-11.3.2.3`
- `US-11.3.3.1`
- `US-11.3.3.2`
- `US-11.3.3.3`
- `US-11.3.4.1`
- `US-11.3.4.2`
- `US-11.3.5.1`
- `US-11.3.5.2`
- `US-11.3.6.1`
- `US-11.3.6.2`
- `US-11.3.6.3`
- `US-11.4.1.1`
- `US-11.4.1.2`
- `US-11.4.1.3`
- `US-11.4.2.1`
- `US-11.4.2.2`
- `US-11.4.2.3`
- `US-11.4.3.1`
- `US-11.4.3.2`
- `US-11.4.3.3`
- `US-11.4.4.1`
- `US-11.4.4.2`
- `US-11.4.4.3`
- `US-11.4.5.1`
- `US-11.4.5.2`
- `US-11.4.5.3`
- `US-11.4.6.1`
- `US-11.4.6.2`
- `US-11.4.6.3`
- `US-11.4.7.1`
- `US-11.4.7.2`
- `US-11.4.7.3`
- `US-11.5.1.1`
- `US-11.5.1.2`
- `US-11.5.1.3`
- `US-11.5.2.1`
- `US-11.5.2.2`
- `US-11.5.2.3`
- `US-11.5.3.1`
- `US-11.5.3.2`
- `US-11.5.3.3`
- `US-11.5.4.1`
- `US-11.5.4.2`
- `US-11.5.4.3`
- `US-11.5.5.1`
- `US-11.5.5.2`
- `US-11.5.6.1`
- `US-11.5.6.2`
- `US-11.5.6.3`
- `US-11.5.7.1`
- `US-11.5.7.2`
- `US-11.5.7.3`

### Test cases

- `TC-11.1.1.1`
- `TC-11.1.3.1`
- `TC-11.2.2.1`
- `TC-11.2.3.1`
- `TC-11.2.3.2`
- `TC-11.2.4.1`
- `TC-11.2.4.2`
- `TC-11.2.4.3`
- `TC-11.2.5.1`
- `TC-11.3.1.1`
- `TC-11.3.1.2`
- `TC-11.3.1.3`
- `TC-11.3.4.1`
- `TC-11.3.6.1`
- `TC-11.3.6.2`
- `TC-11.4.1.1`
- `TC-11.4.2.1`
- `TC-11.4.2.2`
- `TC-11.4.3.1`
- `TC-11.4.5.1`
- `TC-11.4.6.1`
- `TC-11.4.7.1`
- `TC-11.5.0.1`
- `TC-11.5.1.1`
- `TC-11.5.2.1`
- `TC-11.5.3.1`
- `TC-11.5.4.1`
- `TC-11.5.5.1`
- `TC-11.5.6.1`
- `TC-11.5.7.1`
- `TC-11.6.1.1`
- `TC-11.6.2.1`
- `TC-11.6.3.1`
- `TC-11.6.4.1`
- `TC-11.6.5.1`
- `TC-15.8.1.1`

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

- `TC-11.1.1.1`
- `TC-11.1.3.1`
- `TC-11.2.2.1`
- `TC-11.2.3.1`
- `TC-11.2.3.2`
- `TC-11.2.4.1`
- `TC-11.2.4.2`
- `TC-11.2.4.3`
- `TC-11.2.5.1`
- `TC-11.3.1.1`
- `TC-11.3.1.2`
- `TC-11.3.1.3`
- `TC-11.3.4.1`
- `TC-11.3.6.1`
- `TC-11.3.6.2`
- `TC-11.4.1.1`
- `TC-11.4.2.1`
- `TC-11.4.2.2`
- `TC-11.4.3.1`
- `TC-11.4.5.1`
- `TC-11.4.6.1`
- `TC-11.4.7.1`
- `TC-11.5.0.1`
- `TC-11.5.1.1`
- `TC-11.5.2.1`
- `TC-11.5.3.1`
- `TC-11.5.4.1`
- `TC-11.5.5.1`
- `TC-11.5.6.1`
- `TC-11.5.7.1`
- `TC-11.6.1.1`
- `TC-11.6.2.1`
- `TC-11.6.3.1`
- `TC-11.6.4.1`
- `TC-11.6.5.1`
- `TC-15.8.1.1`

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

- `R-15.8.5b` resolved via parent `R-15.8.5` in
  [logic-graph.md](../../requirements/tools/logic-graph.md).

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
