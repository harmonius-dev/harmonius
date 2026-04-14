---
branch: plan/simulation-game-loop-phases
last_updated: 2026-04-14T17:53:30Z
plan_id: PLAN-simulation-game-loop-phases
pr_number: 27
pr_review_status: complete
pr_url: https://github.com/cjhowe-us/harmonius/pull/27
started_at: 2026-04-14T05:17:32Z
status: submitted
worktree_path: /Users/cjhowe/Code/harmonius-worktrees/PLAN-simulation-game-loop-phases
---

# Progress: Simulation Game Loop Phases

Plan file: [game-loop-phases.md](../simulation/game-loop-phases.md)

## Implementation scope (this PR)

Bootstrap workspace member `crates/simulation_phases` with `Phase`, `SimSet`, `FixedTimestep`,
`configure_simulation_order`, change-visibility helpers, `PrimitiveId` / `PrimitiveTickCompleted`,
and a deterministic `run_scheduled_systems` runner. Unit tests map to TC-1.1.2.1–1.1.2.6,
TC-1.1.22.1–1.1.22.2, and TC-17.1.1.1, TC-17.2.1.1, TC-17.3.1.1, TC-17.4.1.1 from the companion
test-case doc. Integration and benchmark rows (TC-1.1.2.I*, TC-1.1.22.I*, TC-1.1.2.B*) register as
`#[ignore]` placeholders until harnesses exist.

## Status checklist

- [x] Worktree created and branch aligned with plan metadata
- [x] Draft PR opened and linked in frontmatter
- [x] Design and companion test-case docs reviewed
- [x] Requirement and user-story trace matrix completed
- [x] Red phase complete with failing tests for uncovered scope (folded into same commit as green
      for this bootstrap slice)
- [x] Green phase complete with minimal passing implementation
- [x] Refactor phase complete with no regressions (minimal surface)
- [ ] Integration validation complete across documented boundaries (partial: ordering contracts
      only; full-frame ECS integration TCs deferred)
- [x] Constraint conformance checks complete (crate `#![deny(unsafe_code)]`, clippy `-D warnings`)
- [ ] Manual validation complete with screenshot and video evidence
- [x] `cargo test --workspace` passes (in plan worktree)
- [x] `cargo clippy --workspace -- -D warnings` passes
- [x] `rumdl check .` passes for touched docs (`rumdl check` on this progress file; repo-wide
      `rumdl check .` may still report unrelated paths)
- [x] Evidence links logged in this file
- [x] Review findings addressed and checklist re-verified
- [x] PR marked ready for human review (`status: submitted`)
- [ ] Merge detected and progress archived by orchestrator

## Implementation readiness gate

- [x] Linked spec artifact section reviewed (features/requirements/user-stories).
- [x] Gap closure decisions accepted or escalated.
- [x] Open questions resolution section reviewed and signed off.
- [x] Derived tests added for previously unmapped IDs (if any). (N/A — companion lists explicit TCs;
      first batch implemented)

## TDD launch readiness

- [x] All previously unmapped ID mappings triaged in plan gap-closure section
- [x] Red test inventory split by requirement and user story
- [x] First failing test batch selected for implementation loop
- [ ] Evidence capture folders prepared (screenshots/videos/logs) (deferred until editor / gameplay
      harness)

## Evidence registry

- **Worktree:** `/Users/cjhowe/Code/harmonius-worktrees/PLAN-simulation-game-loop-phases`
- **Commands (2026-04-14Z):** `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`,
  `cargo fmt` in the plan worktree.
- **PR:** <https://github.com/cjhowe-us/harmonius/pull/27>

## Event log

- 2026-04-14T02:02:00Z — plan-orchestrator — dispatch-only: background plan-implementer dispatched
  (orchestrator pass; no PR merge).

- 2026-04-14T05:17:32Z — plan-implementer — started, worktree + draft PR #27 created; landed
  `simulation_phases` crate with plan TC unit coverage.

- 2026-04-14T05:17:32Z — plan-implementer — `status: code_complete`,
  `pr_review_status: not_started`; follow-up: integration-frame / benchmark / manual capture rows
  from companion test-case doc.

- 2026-04-14T11:36:12Z — pr-reviewer — design conformance and standards pass; one minor API-doc
  clarification on `ScheduledSystem::sim_set` for [`Phase::Simulation`]; `cargo test --workspace`
  and `cargo clippy --workspace -- -D warnings` pass in plan worktree; `rumdl check` clean on this
  progress file; PR #27 undrafted for human review.

- 2026-04-14T17:53:30Z — pr-reviewer — follow-up pass: addressed seven consolidated findings (design
  change markers, AppBuilder scope note, deferred companion TC stubs with `#[ignore]`, hardened
  TC-1.1.22.2 assertions, `SimSet` ordinal vs `Ord` test, `Phase::Custom` / runner docs,
  thread-local grid harness); `cargo test --workspace` and `cargo clippy --workspace -- -D warnings`
  pass; `rumdl check` clean on touched paths.
