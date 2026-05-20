# Backlog — Agent Guide

All rules from the root [AGENTS.md](../../AGENTS.md) apply here.

## Purpose

The backlog is a docs-native, GitHub-ready list of open work items. Every issue has a markdown file
with a stable `BL-NNNN` ID, a title, labels, and an acceptance contract. Mirroring to GitHub is a
one-way sync; the markdown files are the source of truth.

## Structure

```text
docs/backlog/
  AGENTS.md
  index.md
  labels.md
  projects.md
  issues/
    BL-NNNN-{slug}.md
```

## ID format

| Kind        | Format               | Example                              |
|-------------|----------------------|--------------------------------------|
| Backlog ID  | `BL-NNNN-{slug}`     | `BL-0007-modop-unification`          |

`NNNN` is monotonic across the whole backlog. Slugs are kebab-case and stable after publication.

## Issue file format

Every issue file has:

1. Frontmatter (YAML, sorted keys):
   - `id` — `BL-NNNN`
   - `title` — single line
   - `labels` — sorted array
   - `priority` — `P0` | `P1` | `P2` | `P3`
   - `effort` — `S` | `M` | `L` | `XL`
   - `status` — `triage` | `ready` | `blocked` | `in-progress` | `review` | `done`
   - `blocked_by` — sorted array of `BL-NNNN` IDs
   - `blocks` — sorted array of `BL-NNNN` IDs
   - `domain` — engine domain (matches a `docs/{tree}/{domain}/` name)
   - `created` — `YYYY-MM-DD`
2. Markdown body:
   - **Context** — what is broken or missing.
   - **Acceptance criteria** — checkboxes (`- [ ] …`).
   - **Verification** — how done is measured.
   - **References** — links to design, requirements, user-stories, ADRs.

## Rules

1. Every issue traces to at least one design doc, requirement, user-story, or ADR.
2. Acceptance criteria are testable. Replace "fixes the issue" with concrete
   measurable conditions.
3. Issue IDs are immutable. Slugs are immutable after publication.
4. Status transitions go forward; backwards moves require explicit note in the body.
5. Closed (`done`) issues stay in `issues/` for audit history. Do not delete.
6. Add to [index.md](index.md) on creation.
7. Use [labels.md](labels.md) taxonomy verbatim — do not invent ad-hoc labels.

## Priority semantics

| Priority | Definition                                                              |
|----------|-------------------------------------------------------------------------|
| `P0`     | Blocks an in-flight ADR/release; engine cannot proceed without it       |
| `P1`     | Required for the current OKR cycle                                      |
| `P2`     | Required for the next OKR cycle                                         |
| `P3`     | Nice-to-have; track but do not staff                                    |

## When to create a new issue

| Trigger                                                | Result                  |
|--------------------------------------------------------|-------------------------|
| Open `design-review.md` item                           | One issue per item      |
| Coverage matrix flags an orphan ID                     | One issue per orphan    |
| Folder-rule violation in any tree                      | One issue per violation |
| Canonical-owner map lists a duplicate type             | One issue per type      |
| New mid/low-level design doc proposed                  | One issue per doc       |
| Architecture or constraint amendment                   | One ADR + one issue     |
