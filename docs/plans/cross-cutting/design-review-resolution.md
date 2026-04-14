---
children: []
dependencies: []
design_documents:
  - docs/design/design-review.md
execution_mode: sequential
features: []
id: PLAN-cross-cutting-design-review-resolution
name: Design review resolution backlog
parent: null
progress_file: docs/plans/progress/PLAN-cross-cutting-design-review-resolution.md
requirements: []
status: not_started
test_cases: []
worktree_branch: plan/cross-cutting-design-review-resolution
---

# Design review resolution plan

> **Plan ID:** `PLAN-cross-cutting-design-review-resolution`
>
> **Agents:** Load the harmonize skill, then this progress file, before edits.

## Execution instructions

1. Open [progress file](../progress/PLAN-cross-cutting-design-review-resolution.md).
2. Treat each backlog item as a doc or code change with its own PR; keep edits small and traceable.
3. Finish each slice with `rumdl check .`; code slices also need `cargo test` / `clippy` as usual.

## Source documents

| Document | Path |
|----------|------|
| Design review | [../../design/design-review.md](../../design/design-review.md) |
| Progress | [../progress/PLAN-cross-cutting-design-review-resolution.md](../progress/PLAN-cross-cutting-design-review-resolution.md) |

## Scope

Execute the prioritized backlog in [design-review.md](../../design/design-review.md): graph-runtime
consolidation, canonical bridge types, deduplicated primitives, async-scope fixes in runtime paths,
and de-duplicated constraint prose (link to `constraints.md` instead of copy/paste).

Work as a pipeline of **pure specification steps** (produce diffs) and **integration steps** (merge
after review). No subsystem feature work beyond what the review lists.

### In scope

- File-level actions enumerated in design-review Section 4 (and follow-up tables).
- Tracing each change to a single owning plan ID in the progress event log.

### Out of scope

- New features not justified by a review finding.
- Rewriting designs that are already consistent with `docs/design/constraints.md`.

## Task breakdown

### Phase 1: Triage and batch

| # | Task | Est | Notes |
|---|------|-----|-------|
| 1 | Import review rows into progress event log with file paths | 2 | one line per finding |
| 2 | Batch A: `GraphRuntime` / directed-graph ownership (core-runtime) | 16 | unblock codegen |
| 3 | Batch B: bridge types (`RenderFrame`, `CompileError`, `IoError`, …) | 16 | one type per PR where possible |
| 4 | Batch C: async audit — reclassify or rewrite runtime `async` uses | 8 | sync engine rule |
| 5 | Batch D: replace duplicated constraint prose with links | 8 | rumdl-only PRs ok |

## Dependencies

None at plan level; individual batches declare plan IDs they touch.

## Risk assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Wide doc churn | M | Ordered batches; always link to review section |
| Bridge type bikeshed | M | Name exactly as in design-review tables |

## Integration points

Touches many `docs/design/**` files and eventually `crates/**` for graph and error types. Record doc
PR URLs in this plan’s progress event log; the harmonize `design-orchestrator` maintains
`docs/plans/progress/phase-design.md` rollups when that file is present in the branch.

## Test strategy

- Documentation: `rumdl check .` on touched trees.
- Code: red tests owned by the subsystem plan that receives the bridge type (spawn child tasks).

## Verification

1. Every Section 4 item is either done or explicitly deferred with reason in the progress log.
2. `rumdl check .` clean for touched docs.
3. Progress file `status: code_complete` when backlog is empty or deferred set is approved.
