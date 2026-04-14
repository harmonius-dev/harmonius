---
children: []
dependencies:
  - PLAN-physics-foundation
design_documents:
  - docs/design/physics/constraints.md
execution_mode: sequential
features: []
id: PLAN-physics-constraints
name: Physics constraints and joints
parent: null
progress_file: docs/plans/progress/PLAN-physics-constraints.md
requirements: []
status: not_started
test_cases: []
worktree_branch: plan/physics-constraints
---

# Physics constraints implementation plan

> **Plan ID:** `PLAN-physics-constraints`
>
> **Agents:** Load the harmonize skill, then this progress file, before edits.

## Execution instructions

1. Open [progress file](../progress/PLAN-physics-constraints.md).
2. Create worktree `../harmonius-worktrees/PLAN-physics-constraints` per template.
3. For each task: red tests from TC rows (if present), then green; no mocks.
4. Finish with `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`, `rumdl check .`.

## Source documents

| Document | Path |
|----------|------|
| Design | [../../design/physics/constraints.md](../../design/physics/constraints.md) |
| Test cases | [../../design/physics/constraints-test-cases.md](../../design/physics/constraints-test-cases.md) |
| Progress | [../progress/PLAN-physics-constraints.md](../progress/PLAN-physics-constraints.md) |

## Scope

Implement joints, constraint rows, island-parallel solving, and solver configuration per design.
Model solver steps as pure `fn(SolverInput<'_>) -> SolverOutput` over immutable snapshots where
feasible; commit impulses and warm-start data only at phase boundaries.

### In scope

- All normative types in the design `classDiagram`.
- Deterministic ordering for island build and constraint rows (no `HashMap` on hot paths).

### Out of scope

- Ragdoll authoring UX in editor tools (covered under animation / tools plans).
- Network rollback policy (see networking plans).

## Task breakdown

### Phase 1: Red / green from companion TCs

| # | Task | Est | Requirement | Test |
|---|------|-----|-------------|------|
| 1 | Joint components + registration; empty world is stable | 4 | per design | companion |
| 2 | SI vs TGS paths share row builder; differ only in iterator policy | 8 | R-4.3.NF1 | companion |
| 3 | Warm-start carryover bit-identical across identical entity order | 8 | determinism | companion |
| 4 | Breakable joints + ragdoll activation budgets | 8 | R-4.3.NF2 | companion |

## Dependencies

- `PLAN-physics-foundation` — rigid bodies and broad-phase hooks required.

## Risk assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Solver shares state with rendering thread | H | FixedUpdate-only mutation; graph edges explicit |
| Joint explosion on mobile | M | SI path + row caps per design budgets |

## Integration points

Consumers: animation (ragdoll), rendering (debug draw optional), networking (state pack). Update the
progress file as each seam is verified.

## Test strategy

- Unit: every TC row with category Unit.
- Integration: TC rows marked Integration; real deps only.
- Benchmarks: R-4.3.NF1–NF3 numeric gates.

## Verification

1. All applicable TC categories pass.
2. Clippy clean; rumdl clean on touched docs.
3. Progress file `status: code_complete`.
