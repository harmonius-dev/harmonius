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

## Manual UI follow-up

GitHub does not expose mutations for enabling project workflows or creating views. Both must be
configured once through the GitHub Projects web UI. Each item below has a deep link plus a short
step list. Total time: ~5 minutes.

### Step 1 — Enable three workflows

Open the project's Workflows tab: <https://github.com/users/cjhowe-us/projects/3/workflows>. Three
of the six built-in workflows need configuring:

#### Item added to project — set `Status` to `Triage`

1. Open <https://github.com/users/cjhowe-us/projects/3/workflows/6>.
2. Toggle **Status** at the top to **On**.
3. Under **When**, leave `Item is added to project` selected.
4. Under **Set value**, pick field `Status`, value `Triage`.
5. Click **Save and turn on workflow**.

#### Item closed — set `Status` to `Done`

1. Open <https://github.com/users/cjhowe-us/projects/3/workflows/1>.
2. Toggle **Status** to **On**.
3. Under **When**, leave `An item in the project is closed` selected.
4. Under **Set value**, pick field `Status`, value `Done`.
5. Click **Save and turn on workflow**.

#### Pull request merged — set `Status` to `Done`

1. Open <https://github.com/users/cjhowe-us/projects/3/workflows/2>.
2. Toggle **Status** to **On**.
3. Under **When**, leave `A pull request in the project is merged` selected.
4. Under **Set value**, pick field `Status`, value `Done`.
5. Click **Save and turn on workflow**.

The other three workflows can stay at their defaults: `Auto-close issue` (#3), `Auto-add sub-issues`
(#4, already enabled), and `Pull request linked to issue` (#5).

### Step 2 — Create the Roadmap view

The canonical board view: columns by Status, swimlanes by Priority.

1. Open <https://github.com/users/cjhowe-us/projects/3>.
2. Click the **+** tab in the tab strip (right of the existing view tabs).
3. Pick **Board** layout, then **Save**.
4. Double-click the new tab header and rename it to `Roadmap`.
5. Click the **down-arrow** on the `Roadmap` tab to open view options.
6. **Group by** → `Status` (gives the six columns).
7. **Slice by** → `Priority` (gives the swimlanes).
8. The view auto-saves; the green `Saved` indicator confirms.

### Step 3 (optional) — Sub-board views per domain

Each sub-board is the Roadmap view with a label filter. Repeat Step 2 with a different filter for
each row below.

| View name           | Layout | Group by | Slice by | Filter (paste into the filter bar)                                                                        |
|---------------------|--------|----------|----------|-----------------------------------------------------------------------------------------------------------|
| Foundation          | Board  | Status   | Priority | `label:"domain:core-runtime","domain:integration","domain:meta"`                                          |
| Mid-level systems   | Board  | Status   | Priority | `label:"domain:rendering","domain:physics","domain:geometry","domain:ui","domain:input"`                  |
| Domain systems      | Board  | Status   | Priority | `label:"domain:ai","domain:animation","domain:audio","domain:networking","domain:vfx","domain:simulation","domain:data-systems"` |
| Application         | Board  | Status   | Priority | `label:"domain:game-framework","domain:tools","domain:content-pipeline","domain:platform"`                |
| Coverage and audits | Table  | Priority | —        | `label:"coverage:audit","coverage:design","coverage:reqs","coverage:stories","coverage:tests"`            |

After creating each view, drag the tab into your preferred order in the tab strip.

### Beyond the built-in workflows

Label-driven transitions (e.g. flip `Status=In Progress` when an issue gets the `status:in-progress`
label) are handled by the
[`sync-project-status` workflow](../../.github/workflows/sync-project-status.yml). The companion
script [`sync-project-status.sh`](../../.github/workflows/sync-project-status.sh) uses
`gh api graphql` to call `updateProjectV2ItemFieldValue` whenever a `BL-*` issue gains or loses a
`status:*` label. Tracked as BL-0056; ties into OKR-4 / KR-4.5.

The workflow requires a fine-grained PAT stored as the `PROJECT_SYNC_TOKEN` repo secret (Issues:
Read, Projects: Read and write). Rotate the token every 90 days.

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
