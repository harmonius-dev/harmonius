---
children: []
dependencies: []
design_documents:
  - docs/design/integration/input-camera.md
execution_mode: sequential
features: []
id: PLAN-integration-input-camera
name: Integration: Input Camera
parent: null
progress_file: docs/plans/progress/PLAN-integration-input-camera.md
requirements: []
status: not_started
test_cases: []
worktree_branch: plan/integration-input-camera
---

# Integration: Input Camera Implementation Plan

> **Plan ID:** `PLAN-integration-input-camera`
>
> **Agents:** Load the harmonize skill, then this progress file, before edits.

## Execution Instructions

1. Open [progress file](../progress/PLAN-integration-input-camera.md).
2. Create worktree `../harmonius-worktrees/PLAN-integration-input-camera` per template.
3. For each task: red tests from TC rows (if present), then green; no mocks.
4. Finish with `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`,
   `rumdl check .`.

## Source Documents

| Document | Path |
|----------|------|
| Design | [../../design/integration/input-camera.md](../../design/integration/input-camera.md) |
| Test Cases | [../../design/integration/input-camera-test-cases.md](../../design/integration/input-camera-test-cases.md) |
| Progress | [../progress/PLAN-integration-input-camera.md](../progress/PLAN-integration-input-camera.md) |

## Scope

Implement the APIs and data structures defined in the design document. Prefer pure functions
`fn(&Input) -> Output` for transforms; isolate mutation inside ECS command buffers or explicit
phase boundaries per `docs/design/constraints.md`.

### In Scope

- All normative types and traits in the design `classDiagram`.
- Companion test cases (when present), interpreted as ordered TDD leaves.

### Out of Scope

- Features not traced from this design file (spawn a follow-up plan).
- Editor UX polish unless the design file explicitly includes editor sections.

## Task Breakdown

### Phase 1: Red / green from companion TCs

| # | Task | Est | Requirement | Test |
|---|------|-----|-------------|------|
| 1 | Enumerate TC rows; group by game-loop phase | 4 | per design | companion |
| 2 | Implement pure data transforms first; keep IO at edges | 8 | per design | TC-* |
| 3 | Integration tasks after unit scope is green | 8 | per design | TC-*.I* |


## Dependencies

Blocking and parallel edges live in [docs/plans/index.md](../index.md) (topological tiers).

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Duplicate primitive vs another subsystem | M | Resolve in primitives / algorithms plan first |
| Undefined bridge type at integration seam | H | Log in design-review resolution plan |

## Integration Points

List consumers and producers per the design’s Data Flow section when executing (update progress
file as boundaries are verified).

## Test Strategy

- Unit: every TC row with category Unit.
- Integration: TC rows marked Integration; real deps only.
- Benchmarks: numeric gates from performance-budget / design.

## Verification

1. All applicable TC categories pass.
2. Clippy clean; rumdl clean on touched docs.
3. Progress file `status: code_complete`.
