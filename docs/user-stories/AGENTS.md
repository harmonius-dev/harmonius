# User Stories — Agent Guide

All rules from the root [AGENTS.md](../../AGENTS.md) apply here.

## Purpose

User stories describing WHO wants WHAT and WHY from a specific persona's perspective.

## Structure

```text
docs/user-stories/
  README.md                 — user story index
  personas.md               — persona definitions
  {domain}/
    {topic}.md              — user stories
```

Domains match the design directory structure.

## Rules

1. Story IDs use `US-X.Y.Z` format where X is the domain number, Y is the group, Z is the story
2. Format: "As a {persona}, I want {action} so that {benefit}"
3. Each story = 1 persona + 1 action + 1 use case + 1 feature — granular, not compound
4. Reference specific features with F-X.Y.Z IDs
5. Personas are defined in `personas.md` only

## What MUST NOT Be Included

| Prohibited content | Belongs in |
|--------------------|-----------|
| Requirements (`SHALL` statements) | `docs/requirements/` |
| Feature definitions | `docs/features/` |
| Design details | `docs/design/` |
| Persona definitions (inline) | `personas.md` |

## When to Create New Files

- One file per domain topic
- Many stories per feature (granular)
- New personas go in `personas.md`, not inline

## Formatting Reference

- Headings: `## Domain` then `### US-X.Y.Z`
- Body: "As a {persona}, I want {action} so that {benefit}"
- Tables: for story summary, priority, feature trace
- Lists: for acceptance criteria
