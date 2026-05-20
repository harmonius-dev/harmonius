# Coverage — Agent Guide

All rules from the root [AGENTS.md](../../AGENTS.md) apply here.

## Purpose

The coverage tree quantifies how complete the documentation corpus is across the requirement /
feature / user-story / design / test pipelines. Matrices live here. Audits live in
[audits/](audits/) and are dated, append-only.

## Structure

```text
docs/coverage/
  AGENTS.md
  index.md
  requirements-matrix.md
  user-stories-matrix.md
  audits/
    YYYY-MM-audit.md
```

## Matrix format

Each matrix is a per-domain section. The columns are stable across all matrices:

| Column        | Meaning                                                     |
|---------------|-------------------------------------------------------------|
| `ID`          | The thing covered (`R-X.Y.Z`, `US-X.Y.Z`)                   |
| `Persona`     | (US matrix only) the persona who wants the story            |
| `Feature`     | linked `F-X.Y.Z` if any                                     |
| `Design`      | linked design doc                                           |
| `Test`        | linked `TC-X.Y.Z.N` or test-case file                       |
| `Status`      | `Owned` / `Orphan` / `Partial` / `Reversed`                 |

Empty cells signal coverage gaps. Each gap rolls up to a backlog issue.

## Audit format

`audits/YYYY-MM-audit.md` snapshots the corpus at a point in time:

1. **Top of doc** — generation date, branch, command.
2. **File counts per tree** (requirements, user-stories, features, design, plans,
   business, decisions, okrs, backlog, coverage).
3. **Orphan ID counts** — IDs in source but not in matrices, vice versa.
4. **Folder-rule violations** — count and pointer per folder.
5. **Stale references** — count of `compio`, `descriptor heap`, AWS service names,
   removed dependency names.
6. **Backlog snapshot** — open / closed counts by priority.
7. **Findings** — paragraph per cluster.
8. **Follow-up issues** — table linking each finding to a backlog issue.

## Rules

1. Audits are append-only. Never edit a closed audit. New audits supersede old ones via
   the `index.md` table.
2. Matrices are *generated* from filesystem facts, not authored from scratch. The
   generation procedure is documented in `index.md`.
3. Matrices may be partial — domains the current pass has not reached are noted as
   "Summary only" and tracked by a backlog issue.
4. Coverage gates are advisory until a corresponding ADR/PDR makes them normative.
