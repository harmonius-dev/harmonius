---
blocked_by: []
blocks: []
created: 2026-05-20
domain: cross-cutting
effort: M
id: BL-0040
labels: [coverage:design, domain:cross-cutting, p2, status:triage, type:tech-debt]
priority: P2
status: triage
title: Drain residual async/await prose in design tree
---

# Drain residual async/await prose in design tree

## Context

The 2026-04-12 review P0 #6 mostly closed the async-purge work for game-runtime code. Per
[ADR-0004](../../decisions/adr/ADR-0004-no-async-in-engine.md), engine / editor / game
server are sync. Backend services on Kubernetes may use Tokio.

A grep for `async fn|\.await|tokio::|Future<` across `docs/design/` still hits ~30 files,
mostly in migration-table prose, RF / TODO sections, or test expected outputs. This issue
finishes that cleanup.

## Acceptance criteria

- [ ] All RF / TODO prose that says "replace async with sync" is either applied (and the
      RF marked APPLIED) or moved to a backlog issue.
- [ ] All migration-comparison tables are clearly labeled as historical.
- [ ] `networking/network-infrastructure.md` (TiKV examples) is the only legitimate
      async/await source in the design tree, scoped per ADR-0004 to backend services.

## Verification

`grep -rE 'async fn|\.await|tokio::|Future<' docs/design/` returns hits only in
`network-infrastructure.md`, the `constraints.md` prohibition text, the design-review status
update, and explicitly-historical migration sections.

## References

- [ADR-0004 No async in engine](../../decisions/adr/ADR-0004-no-async-in-engine.md)
- [docs/design/design-review.md P0 #6](../../design/design-review.md)
- [docs/coverage/audits/2026-05-audit.md](../../coverage/audits/2026-05-audit.md)
