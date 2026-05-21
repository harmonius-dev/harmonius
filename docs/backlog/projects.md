# Backlog Project Boards

GitHub Projects v2 is the home for backlog views. The Engine Roadmap project is created and
populated by [`scripts/seed-project.sh`](scripts/seed-project.sh). Field configuration (`Status`,
`Priority`) and per-item field values are applied by `seed-project.sh` via the GraphQL
`updateProjectV2Field` and `updateProjectV2ItemFieldValue` mutations. View layout and workflow
automation are the remaining GitHub-UI configuration steps because GitHub does not currently expose
them through the GraphQL API.

## Board

### Engine Roadmap (primary)

Single GitHub Project (`#3` under `cjhowe-us`, <https://github.com/users/cjhowe-us/projects/3>) that
contains every `BL-*` issue.

```bash
bash docs/backlog/scripts/seed-project.sh
```

The script creates the project (idempotent), links every `BL-*` issue, configures the `Status` field
with the six column values, creates the `Priority` field, and sets `Status=Triage` plus
`Priority=PN` on every item from the issue's labels.

### Status field (column model)

The `Status` field's six options correspond to the project board columns:

| Column      | Color  | Description                                  |
|-------------|--------|----------------------------------------------|
| Triage      | Gray   | Newly filed; not yet sized or scheduled       |
| Ready       | Green  | Sized; can be picked up                       |
| In Progress | Yellow | Actively being worked                         |
| Review      | Blue   | Submitted for review                          |
| Blocked     | Purple | Has unmet `blocked_by`                        |
| Done        | Green  | Acceptance criteria met                       |

### Priority field (swimlane model)

Single-select with options `P0`, `P1`, `P2`, `P3`. Used for swim-laning views.

### Sub-board filters (configured per view via the GitHub UI)

Once views are created in the UI, these filters group the same set of items differently.

| Sub-board          | Filter                                                                                |
|--------------------|---------------------------------------------------------------------------------------|
| Foundation         | `label:domain:core-runtime` OR `label:domain:integration` OR `label:domain:meta`     |
| Mid-level systems  | `label:domain:rendering`, `…physics`, `…geometry`, `…ui`, `…input`                   |
| Domain systems     | `label:domain:ai`, `…animation`, `…audio`, `…networking`, `…vfx`, `…simulation`, `…data-systems` |
| Application        | `label:domain:game-framework`, `…tools`, `…content-pipeline`, `…platform`            |
| Coverage and audits| any `label:coverage:*`                                                                |

## Automation

GitHub Projects v2 currently exposes the workflow listing and deletion mutations only. Enabling and
editing built-in workflows requires the GitHub web UI on the **Workflows** tab of the project.

Recommended one-time setup (each workflow takes one click in the UI):

| Built-in workflow             | Configure to                                       |
|-------------------------------|----------------------------------------------------|
| `Item added to project`       | Set `Status` to `Triage`                           |
| `Item closed`                 | Set `Status` to `Done`                             |
| `Pull request merged`         | Set `Status` to `Done`                             |
| `Auto-add sub-issues`         | Already enabled by default; leave on               |

Any further label-driven transitions (e.g. flipping `Status` based on `status:in-progress` labels)
require either a custom GitHub Actions workflow under `.github/workflows/` or manual updates inside
the project. Tracked as a future enhancement under OKR-4 / KR-4.5 follow-up — not blocking the
present cycle.

## Views

Default view: *table* showing all 55 items with their fields. Recommended additional views to create
in the UI:

| View name        | Layout | Group by   | Filter / sort                                  |
|------------------|--------|------------|------------------------------------------------|
| Roadmap          | Board  | `Status`   | swim-lane by `Priority`                         |
| Foundation       | Board  | `Status`   | filter: `Foundation` sub-board (above)          |
| Mid-level        | Board  | `Status`   | filter: `Mid-level systems` sub-board           |
| Domain           | Board  | `Status`   | filter: `Domain systems` sub-board              |
| Application      | Board  | `Status`   | filter: `Application` sub-board                 |
| Coverage         | Table  | `Priority` | filter: any `coverage:*` label                   |

Creating views via the API is not yet supported. The `Roadmap` view in particular gives the column /
swimlane experience documented above — three clicks in the UI to set up.

## Source-of-truth contract

The Engine Roadmap board is a *view* over GitHub Issues, not authoritative storage. Issues
themselves carry the canonical state ([`AGENTS.md`](AGENTS.md)).

- Issue body changes flow through GitHub Issues, not the project board.
- Status, priority, and effort live on issue labels (canonical) and on project fields
  (mirror).
- Project columns derive from the `Status` field. Project swimlanes derive from `Priority`.
- Re-running [`seed-project.sh`](scripts/seed-project.sh) is safe: missing items are
  added, present items are not duplicated.

## Out of scope

- Burndown charts and velocity targets — [`docs/okrs/`](../okrs/index.md) holds the
  outcome targets.
- Time tracking — not modeled.
- Branch policy — see root [`AGENTS.md`](../../AGENTS.md).
