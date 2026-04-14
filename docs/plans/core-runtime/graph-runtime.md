---
children: []
dependencies: []
design_documents:
  - docs/design/core-runtime/graph-runtime.md
execution_mode: sequential
features: []
id: PLAN-core-runtime-graph-runtime
name: Graph Runtime
parent: null
progress_file: docs/plans/progress/PLAN-core-runtime-graph-runtime.md
requirements: []
status: not_started
test_cases: []
worktree_branch: plan/core-runtime-graph-runtime
---

# Graph Runtime Implementation Plan

> **Plan ID:** `PLAN-core-runtime-graph-runtime`
>
> **Agents:** Load the harmonize skill, then this progress file, before edits.

## Execution Instructions

1. Open [progress file](../progress/PLAN-core-runtime-graph-runtime.md).
2. Create worktree `../harmonius-worktrees/PLAN-core-runtime-graph-runtime` per template.
3. For each task: red tests from TC rows (if present), then green; no mocks.
4. Finish with `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`,
   `rumdl check .`.

## Source Documents

| Document | Path |
|----------|------|
| Design | [../../design/core-runtime/graph-runtime.md](../../design/core-runtime/graph-runtime.md) |
| Test Cases | [../../design/core-runtime/graph-runtime-test-cases.md](../../design/core-runtime/graph-runtime-test-cases.md) |
| Progress | [../progress/PLAN-core-runtime-graph-runtime.md](../progress/PLAN-core-runtime-graph-runtime.md) |

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
