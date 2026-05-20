# Backlog Project Boards

GitHub Projects v2 is the home for backlog views. The board is created by
[`scripts/seed-project.sh`](scripts/seed-project.sh) and configured manually for columns, swimlanes,
and automation.

## Board

### Engine Roadmap (primary)

Single GitHub Project under the repo owner that contains every `BL-*` issue. Filtered, swimlaned,
and grouped by the views below. Created by:

```bash
bash docs/backlog/scripts/seed-project.sh
```

| Column          | Filter                              |
|-----------------|-------------------------------------|
| Triage          | `status:triage`                     |
| Ready           | `status:ready`                      |
| In Progress     | `status:in-progress`                |
| Review          | `status:review`                     |
| Blocked         | `status:blocked`                    |
| Done            | `status:done` (collapsed by default) |

### Foundation (sub-board)

Filter: `domain:core-runtime` OR `domain:integration` OR `domain:meta`. Use this view for ADR / PDR
follow-up and canonical-owner cleanup. Driven by 2026-Q3 OKRs.

### Mid-level systems (sub-board)

Filter: `domain:rendering` OR `domain:physics` OR `domain:geometry` OR `domain:ui` OR
`domain:input`. Drives the mid/low-level design coverage roadmap.

### Domain systems (sub-board)

Filter: `domain:ai` OR `domain:animation` OR `domain:audio` OR `domain:networking` OR `domain:vfx`
OR `domain:simulation` OR `domain:data-systems`. Per-domain feature work.

### Application (sub-board)

Filter: `domain:game-framework` OR `domain:tools` OR `domain:content-pipeline` OR `domain:platform`.
Editor, save, and shipping concerns.

### Coverage and audits (sub-board)

Filter: any `coverage:*` label. Drives matrix completion and folder-rule audits.

## Swimlanes (in any board)

| Swimlane | Filter      |
|----------|-------------|
| P0       | `p0`        |
| P1       | `p1`        |
| P2       | `p2`        |
| P3       | `p3`        |

## Automation

Automation is configured in the GitHub Projects UI (Workflows tab) or via the GraphQL API. The
[`seed-project.sh`](scripts/seed-project.sh) script does the structural part only — the rules below
are applied by hand on first setup.

| Trigger                                          | Action                          |
|--------------------------------------------------|---------------------------------|
| Issue gets `status:in-progress`                  | Move to In Progress column      |
| Issue gets `status:review`                       | Move to Review column           |
| Issue closes (`status:done`)                     | Move to Done column             |
| Issue gets `status:blocked`                      | Move to Blocked column          |
| New issue with no `status:*`                     | Move to Triage column           |
| All `blocked_by` close                           | Suggest moving to `status:ready`|

## Source-of-truth contract

The Engine Roadmap board is a *view* over GitHub Issues, not authoritative storage. Issues
themselves carry the canonical state ([`AGENTS.md`](AGENTS.md)).

- Issue body changes flow through GitHub Issues, not the project board.
- Status, priority, and effort live on issue labels, not project columns.
- Project columns derive from labels via the automations above.
- Re-running [`seed-project.sh`](scripts/seed-project.sh) is safe: missing items are
  added, present items are not duplicated.

## Out of scope

- Burndown charts and velocity targets — [`docs/okrs/`](../okrs/index.md) holds the
  outcome targets.
- Time tracking — not modeled.
- Branch policy — see root [`AGENTS.md`](../../AGENTS.md).
