# User Stories — Agent Guide

All rules from the root [CLAUDE.md](../../CLAUDE.md) apply here.

## Purpose

User stories describing WHO wants WHAT and WHY from a specific
persona's perspective. Stories are written BEFORE requirements.

## Structure

```text
docs/user-stories/
  README.md                 — user story index
  CLAUDE.md                 — this file
  personas.md               — persona definitions
  {domain}/
    {topic}.md              — user stories
```

One file per domain topic in its domain subdirectory.
`personas.md` stays at root level.

## Rules

1. Story IDs use `US-X.Y.Z` format where X is the domain
   number, Y is the group, Z is the story
2. Format: "As a {persona}, I want {action} so that {benefit}"
   — always use "I want" (not "I need")
3. Each story = 1 persona + 1 action + 1 use case + 1
   feature — granular, not compound
4. NO acceptance criteria — those belong in requirements
5. NO links to features or requirements
6. Personas are defined in `personas.md` only
7. Tables use `| ID | Persona |` columns only
8. Game-specific stories tagged with `[game-specific]`
9. Stories are written BEFORE requirements — they drive
   requirement creation
10. For game-specific features, include gamer (P-23) and game
    developer (P-15) personas

## What MUST NOT Be Included

| Prohibited content | Belongs in |
|--------------------|-----------|
| Requirements (`SHALL` statements) | `docs/requirements/` |
| Feature definitions | `docs/features/` |
| Design details | `docs/design/` |
| Persona definitions (inline) | `personas.md` |
| Acceptance criteria | `docs/requirements/` |
| Links to features | Removed — not used |
| Links to requirements | Removed — not used |

## When to Create New Files

- One file per domain topic in its domain subdirectory
  (e.g., `core-runtime/async-io.md`)
- Many stories per feature (granular)
- New personas go in `personas.md`, not inline

## Formatting Reference

- Headings: `## Domain` then `### US-X.Y.Z`
- Body: "As a {persona}, I want {action} so that {benefit}"
- Tables: `| ID | Persona |` for story summary
- Tag game-specific stories with `[game-specific]`
