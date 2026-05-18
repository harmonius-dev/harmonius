# Features — Agent Guide

All rules from the root [AGENTS.md](../../AGENTS.md) apply here.

## Purpose

Feature definitions for all engine subsystems. Features define WHAT the engine provides.

## Structure

```text
docs/features/
  README.md                 — feature index
  AGENTS.md                 — this file
  {domain}/
    {topic}.md              — feature definitions
```

One file per domain topic in its domain subdirectory.

## Rules

1. Feature IDs use `F-X.Y.Z` format where X is the domain
   number, Y is the group, Z is the feature
2. Feature tables use `| ID | Feature |` columns only — no
   Requirements column
3. Features describe capability, not implementation
4. Group related features under domain headings
5. No cross-reference links to requirements or user stories

## What MUST NOT Be Included

| Prohibited content | Belongs in |
|--------------------|-----------|
| Requirements (`SHALL` statements) | `docs/requirements/` |
| User stories | `docs/user-stories/` |
| Design details | `docs/design/` |
| Implementation code | `src/` |
| Links to requirements | `docs/requirements/` |
| Links to user stories | `docs/user-stories/` |

## When to Create New Files

- One file per domain topic in its domain subdirectory
  (e.g., `ai/navigation.md`)
- Split when a file exceeds 500 lines

## Formatting Reference

- Headings: `## Domain` then `### F-X.Y.Z Feature Name`
- Tables: `| ID | Feature |` for feature summary
- Lists: for feature details
