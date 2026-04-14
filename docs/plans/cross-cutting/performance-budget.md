---
children: []
dependencies:
  - PLAN-integration-high-level
design_documents:
  - docs/design/performance-budget.md
execution_mode: sequential
features: []
id: PLAN-cross-cutting-performance-budget
name: Performance budget
parent: null
progress_file: docs/plans/progress/PLAN-cross-cutting-performance-budget.md
requirements: []
status: not_started
test_cases: []
worktree_branch: plan/cross-cutting-performance-budget
---

# Performance budget plan

> **Plan ID:** `PLAN-cross-cutting-performance-budget`
>
> **Agents:** Load the harmonize skill, then this progress file, before edits.

## Execution instructions

1. Open [progress file](../progress/PLAN-cross-cutting-performance-budget.md).
2. Keep budget tables derived from
   [integration/high-level.md](../../design/integration/high-level.md); this file extends tiers and
   frame rates.
3. Benchmarks that enforce budgets live in subsystem **test-cases** files — wire TC IDs here.

## Source documents

| Document | Path |
|----------|------|
| Performance budget | [../../design/performance-budget.md](../../design/performance-budget.md) |
| High-level integration | [../../design/integration/high-level.md](../../design/integration/high-level.md) |
| Progress | [../progress/PLAN-cross-cutting-performance-budget.md](../progress/PLAN-cross-cutting-performance-budget.md) |

## Scope

Maintain numeric budgets as **immutable reference data** in docs; encode enforcement as benchmark
tests owned by subsystems (pure timing functions + fixed hardware profile where required).

### In scope

- Doc edits when high-level integration budgets change.
- Cross-links to `TC-*` benchmark rows that implement each budget line.

### Out of scope

- Implementing subsystem features (owned by domain plans).

## Task breakdown

| # | Task | Est | Notes |
|---|------|-----|-------|
| 1 | Map each budget row to an existing or new TC benchmark ID | 4 | table in progress log |
| 2 | Remove drift vs `integration/high-level.md` | 2 | single PR |
| 3 | Add CI threshold hooks only where benchmarks already exist | 8 | no mock timers |

## Dependencies

- `PLAN-integration-high-level` — authoritative 60 fps frame model.

## Risk assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Benchmark flakiness | M | Pin scenario data; use deterministic seeds |

## Integration points

Profiler, game loop, and rendering plans consume this doc for regression messaging.

## Test strategy

- No standalone test file for this doc; verification is via linked subsystem benchmarks.

## Verification

1. Every budget table row links to a TC or explicit “not yet instrumented”.
2. `rumdl check .` clean.
3. Progress file `status: code_complete`.
