---
children: []
dependencies: []
design_documents:
  - docs/design/constraints.md
execution_mode: sequential
features: []
id: PLAN-cross-cutting-design-constraints
name: Project-wide design constraints
parent: null
progress_file: docs/plans/progress/PLAN-cross-cutting-design-constraints.md
requirements: []
status: not_started
test_cases: []
worktree_branch: plan/cross-cutting-design-constraints
---

# Project constraints plan

> **Plan ID:** `PLAN-cross-cutting-design-constraints`
>
> **Agents:** Load the harmonize skill, then this progress file, before edits.

## Execution instructions

1. Open [progress file](../progress/PLAN-cross-cutting-design-constraints.md).
2. Keep `docs/design/constraints.md` the single source of truth; other docs link here.
3. When constraints imply CI checks (e.g. grep guards), add them in small PRs with clear messages.

## Source documents

| Document | Path |
|----------|------|
| Constraints | [../../design/constraints.md](../../design/constraints.md) |
| Progress | [../progress/PLAN-cross-cutting-design-constraints.md](../progress/PLAN-cross-cutting-design-constraints.md) |

## Scope

Maintain and evolve project-wide constraints. Prefer **declarative rules** (tables) and
**pure verification** (linters, static checks) over narrative duplication.

### In scope

- Edits to `docs/design/constraints.md` that trace to harmonize specify artifacts when normative.
- Removing duplicate constraint paragraphs elsewhere in favor of links.

### Out of scope

- Subsystem-specific API design (use the relevant subsystem plan).

## Task breakdown

| # | Task | Est | Notes |
|---|------|-----|-------|
| 1 | Audit duplicate constraint prose; open dedupe PRs | 4 | link targets only |
| 2 | Align codegen / lint rules with constraint tables | 8 | optional automation |
| 3 | Verify `docs/design/constraints.md` references stay 100-char clean | 1 | rumdl |

## Dependencies

None.

## Risk assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Constraint drift vs code | H | Pair doc changes with lint or test gates |

## Integration points

Every subsystem plan’s “Verification” section should cite `constraints.md` for global rules.

## Test strategy

- `rumdl check .` on `docs/`.
- Add repo checks only when they encode a single boolean rule (no flaky network).

## Verification

1. No contradictory constraint statements in `docs/design/`.
2. Progress file `status: code_complete` when audit + optional automation land.
