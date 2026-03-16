# Requirements — Agent Guide

All rules from the root [AGENTS.md](../../AGENTS.md) apply here.

## Purpose

Functional and non-functional requirements for all engine subsystems. Requirements define what the
system SHALL do.

## Structure

```text
docs/requirements/
  README.md                 — requirements index
  {domain}/
    {topic}.md              — requirements
  cross-cutting.md          — cross-domain requirements
```

Domains match the design directory structure.

## Rules

1. Requirement IDs use `R-X.Y.Z` format where X is the domain number, Y is the group, Z is the
   requirement
2. Each requirement uses `SHALL` for mandatory and `SHOULD` for recommended (RFC 2119)
3. Requirements are testable and measurable
4. Performance requirements include specific targets (e.g., "< 2ms per frame")
5. Cross-reference features with F-X.Y.Z IDs

## What MUST NOT Be Included

| Prohibited content | Belongs in |
|--------------------|-----------|
| Feature definitions | `docs/features/` |
| User stories | `docs/user-stories/` |
| Design details | `docs/design/` |
| Pseudocode or implementation | `src/` |
| Persona definitions | `docs/user-stories/` |

## When to Create New Files

- One file per domain topic
- Performance requirements alongside functional ones
- Cross-cutting requirements in `cross-cutting.md`

## Formatting Reference

- Headings: `## Domain` then `### R-X.Y.Z`
- Body: `The system SHALL...` statement
- Tables: for requirement summary, priority, trace
- Lists: for sub-requirements and conditions
