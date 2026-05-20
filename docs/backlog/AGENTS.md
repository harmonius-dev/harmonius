# Backlog — Agent Guide

All rules from the root [AGENTS.md](../../AGENTS.md) apply here.

## Purpose

The backlog catalogs open work items for the Harmonius engine.
**GitHub Issues are the single source of truth.** This directory contains:

- The normative label taxonomy ([`labels.md`](labels.md)) and project board model
  ([`projects.md`](projects.md)) consumed by the seed scripts.
- A one-time seed corpus ([`seed.json`](seed.json)) that bootstraps a fresh repo.
- Seed scripts under [`scripts/`](scripts/) that materialize the seed corpus into GitHub
  via the `gh` CLI.

There are no per-issue markdown files. Once an issue exists in GitHub, every change flows through
the GitHub UI or `gh issue edit`.

## Structure

```text
docs/backlog/
  AGENTS.md
  index.md           — entry point with GitHub search URL filters
  labels.md          — normative label taxonomy
  projects.md        — Engine Roadmap project board model
  seed.json          — issue + label catalog (sorted JSON, one-time seed input)
  scripts/
    README.md
    seed-labels.sh   — gh CLI driver for the labels in seed.json
    seed-issues.sh   — gh CLI driver for the issues in seed.json
    seed-project.sh  — gh CLI driver for the Engine Roadmap project
```

## ID format

| Kind        | Format               | Example                              |
|-------------|----------------------|--------------------------------------|
| Backlog ID  | `BL-NNNN`            | `BL-0007`                            |

The ID lives in the GitHub issue title with prefix: `BL-NNNN: <short title>`. Cross references in
design docs use the search URL `https://github.com/cjhowe-us/harmonius/issues?q=BL-NNNN`, which
resolves to the issue regardless of its assigned numeric ID and regardless of open/closed state.

`NNNN` is monotonic across the backlog. Once published, IDs are immutable. Retired issues remain in
`seed.json` with `status:done` so reseeding into a fresh clone produces the same catalog.

## Issue contract (lives in GitHub)

Every GitHub issue:

1. Title prefix `BL-NNNN: …`.
2. Labels from [`labels.md`](labels.md) only — priority, type, status, effort, exactly one
   `domain:*`, optionally one `coverage:*`, optionally `component:*`.
3. Body sections in this order: **Context**, **Acceptance criteria**, **Verification**,
   **References**.
4. Acceptance criteria are checkboxes (`- [ ] …`).
5. References cite at least one design doc, ADR / PDR, requirement, user-story, or another
   `BL-` issue.

The seed scripts emit issues that meet this contract. Hand-written issues should follow it.

## Status, priority, effort

| Priority | Definition                                                              |
|----------|-------------------------------------------------------------------------|
| `p0`     | Blocks an in-flight ADR/release; engine cannot proceed without it       |
| `p1`     | Required for the current OKR cycle                                      |
| `p2`     | Required for the next OKR cycle                                         |
| `p3`     | Track only; do not staff                                                |

Status is a label, not a free-form field, so it shows up on filters:

| Status label         | Meaning                                          |
|----------------------|--------------------------------------------------|
| `status:triage`      | Newly filed; not yet sized or scheduled          |
| `status:ready`       | Sized; can be picked up                          |
| `status:blocked`     | Has unmet `blocked_by` dependency                |
| `status:in-progress` | Actively being worked                            |
| `status:review`      | Submitted for review                             |
| `status:done`        | Acceptance criteria met (issue closed)           |

Effort is also a label (`effort:s` / `m` / `l` / `xl`). See [`labels.md`](labels.md).

## Workflow

| Stage                       | Action                                                |
|-----------------------------|-------------------------------------------------------|
| Bootstrap a fresh repo      | Run the three seed scripts in order                   |
| File a new issue            | Open it on GitHub directly with the contract above    |
| Update progress             | Edit the issue on GitHub; flip `status:*` labels      |
| Resolve                     | Close the GitHub issue with `status:done` label        |
| Audit / coverage gates      | Run filtered searches; see [`index.md`](index.md)     |

## Re-seeding policy

- The seed scripts are idempotent. Re-running on a populated repo creates only missing
  items.
- Edits to `seed.json` after the initial bootstrap are reseed-only: they create missing
  issues but never edit existing ones. To change an existing issue, edit it on GitHub.
- New `BL-NNNN` IDs added to `seed.json` are appended at the next ordinal. Numbering is
  immutable.

## When to add to `seed.json`

| Trigger                                                | Action                       |
|--------------------------------------------------------|------------------------------|
| Open `design-review.md` item                           | Append a `BL-NNNN` entry     |
| Coverage matrix flags an orphan ID                     | Append a `BL-NNNN` entry     |
| Folder-rule violation in any tree                      | Append a `BL-NNNN` entry     |
| Canonical-owner map lists a duplicate type             | Append a `BL-NNNN` entry     |
| New mid/low-level design doc proposed                  | Append a `BL-NNNN` entry     |
| Architecture or constraint amendment                   | One ADR + one `BL-NNNN`      |

For ad-hoc bug reports, file directly on GitHub without touching `seed.json`. The seed catalog is
reserved for the curated bootstrap set.
