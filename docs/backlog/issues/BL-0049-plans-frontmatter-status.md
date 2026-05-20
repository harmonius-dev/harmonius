---
blocked_by: []
blocks: []
created: 2026-05-20
domain: plans
effort: L
id: BL-0049
labels: [coverage:audit, domain:plans, p2, status:triage, type:plan]
priority: P2
status: triage
title: Reconcile plan frontmatter status with progress files
---

## Reconcile plan frontmatter `status` with progress files

### Context

All 141 plan body files carry `status: not_started` in their YAML frontmatter. Progress stubs report
96 `code_complete`, 23 `started`, 12 `submitted`, 10 `not_started`. The deslop pass added
[plans/AGENTS.md](../../plans/AGENTS.md) clarifying that progress files are the source of truth —
but the contradiction in the plan-body files still creates noise.

### Acceptance criteria

- [ ] Either: rewrite plan-body frontmatter to either drop `status` (intent only) or sync
      with progress; pick one and apply uniformly.
- [ ] Document the chosen approach in `plans/AGENTS.md`.
- [ ] Coverage matrices read from progress files only.

### Verification

A spot-check of 20 random plans shows consistent frontmatter; no plan body claims `not_started`
while its progress file says `code_complete`.

### References

- [docs/plans/AGENTS.md](../../plans/AGENTS.md) "Status authority"
- [docs/plans/index.md](../../plans/index.md)
