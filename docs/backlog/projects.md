# Backlog Project Boards

GitHub Projects model for the backlog. Boards are *views* over the issue set, not authoritative
storage. The markdown files in [issues/](issues/) are the source of truth.

## Boards

### Engine Roadmap (primary)

Single project that contains every backlog issue. Filtered, swimlaned, and grouped by the views
below.

| Column          | Filter                              |
|-----------------|-------------------------------------|
| Triage          | `status:triage`                     |
| Ready           | `status:ready`                      |
| In Progress     | `status:in-progress`                |
| Review          | `status:review`                     |
| Blocked         | `status:blocked`                    |
| Done            | `status:done` (collapsed by default) |

### Foundation (sub-board)

Filter: `domain:core-runtime` OR `domain:integration` OR `domain:meta`. Use this board for
ADR/PDR/SC follow-up and canonical-owner cleanup. Driven by 2026-Q3 OKRs.

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

These rules run on the GitHub project once the labels are mirrored:

| Trigger                                          | Action                          |
|--------------------------------------------------|---------------------------------|
| Issue gets `status:in-progress`                  | Move to In Progress column      |
| Issue gets `status:review`                       | Move to Review column           |
| Issue closes (`status:done`)                     | Move to Done column             |
| Issue gets `status:blocked`                      | Move to Blocked column          |
| New issue with no `status:*`                     | Move to Triage column           |
| All `blocked_by` close                           | Suggest moving to `status:ready` |

Automation rules are advisory in this doc — the workflow plugin or a GitHub Actions workflow under
`.github/workflows/` is the implementation venue and out of scope here.

## Mirroring policy

1. Each `BL-NNNN` markdown file maps to one GitHub issue with a stable title prefix
   `BL-NNNN: …`.
2. The markdown body is the canonical body. Light editorial differences in the GitHub
   issue body are tolerated; substantive changes round-trip back to the markdown file.
3. Closed GitHub issues set `status: done` in the markdown frontmatter.
4. Re-opened GitHub issues revert to the prior status if known, or `triage` otherwise.
5. Labels on the GitHub side track [labels.md](labels.md) verbatim.

## Out of scope

- Burndown charts and velocity targets — [docs/okrs/](../okrs/index.md) holds the
  outcome targets.
- Time tracking — not modeled.
- Branch policy — see root [AGENTS.md](../../AGENTS.md).
