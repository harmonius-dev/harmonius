---
branch: plan/core-runtime-events-plugins
last_updated: 2026-04-14T17:58:13Z
plan_id: PLAN-core-runtime-events-plugins
pr_number: 75
pr_review_status: complete
pr_url: https://github.com/cjhowe-us/harmonius/pull/75
started_at: 2026-04-14T05:27:19Z
status: submitted
worktree_path: /Users/cjhowe/Code/harmonius-worktrees/PLAN-core-runtime-events-plugins
---

# Progress: Core Runtime Events Plugins

Plan file: [events-plugins.md](../core-runtime/events-plugins.md)

## Status checklist

- [x] Worktree created and branch aligned with plan metadata
- [x] Draft PR opened and linked in frontmatter
- [x] Design and companion test-case docs reviewed
- [x] Requirement and user-story trace matrix completed (via TC tests)
- [x] Red phase complete with failing tests for uncovered scope
- [x] Green phase complete with minimal passing implementation
- [x] Refactor phase complete with no regressions
- [x] Integration validation complete across documented boundaries
- [x] Constraint conformance checks complete (clippy `-D warnings`, scoped tests)
- [ ] Manual validation complete with screenshot and video evidence (deferred: CI-style only)
- [x] `cargo test --workspace` passes
- [x] `cargo clippy --workspace -- -D warnings` passes
- [x] `rumdl check` on this progress file passes
- [x] Evidence links logged in this file (test run + PR)
- [x] Review findings addressed and checklist re-verified
- [x] PR marked ready for human review (`status: submitted`)
- [ ] Merge detected and progress archived by orchestrator
- [x] Code complete marker set

## Implementation readiness gate

- [x] Linked spec artifact section reviewed (features/requirements/user-stories).
- [x] Gap closure decisions accepted or escalated.
- [x] Open questions resolution section reviewed and signed off.
- [x] Derived tests added for previously unmapped IDs (if any).

## TDD launch readiness

- [x] All previously unmapped ID mappings triaged in plan gap-closure section
- [x] Red test inventory split by requirement and user story
- [x] First failing test batch selected for implementation loop
- [ ] Evidence capture folders prepared (screenshots/videos/logs) — not used for this PR

## Evidence registry

- Tests: `cargo test -p harmonius-core` — 41 integration tests (`tc_plan_events_plugins.rs`).
- PR: <https://github.com/cjhowe-us/harmonius/pull/75> (ready for review).

## Event log

- 2026-04-14T05:27:19Z — started, worktree + draft PR created
- 2026-04-14T05:27:19Z — code complete, awaiting review (harmonius-core + TC tests)
- 2026-04-14T17:58:13Z — submitted for human review after pr-reviewer pass; consolidated review
  reported 27 findings — code fixes landed for graph determinism, non-panicking topo relaxation,
  real cycle paths on sort failure, `BuiltApp`/`StreamConfig` `Debug`, test-only hot-reload sleep,
  `ObserverRegisterError`, `EventWriter`/`EventReader`, and related tests; design-parity gaps
  (propagation, full `App` surface, hot-reload fidelity) left for follow-up; repo-wide
  `rumdl check .` still fails on baseline unrelated to this branch
