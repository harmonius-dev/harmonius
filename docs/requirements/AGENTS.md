# Requirements — Agent Guide

All rules from the root [AGENTS.md](../../AGENTS.md) apply here.

## Purpose

Functional and non-functional requirements for all engine subsystems. Requirements define what the
system SHALL do.

## Structure

```text
docs/requirements/
  README.md                 — requirements index
  AGENTS.md                 — this file
  cross-cutting.md          — cross-domain requirements
  {domain}/
    {topic}.md              — requirements
```

One file per domain topic in its domain subdirectory. `cross-cutting.md` stays at root level.

## Rules

1. Requirement IDs use `R-X.Y.Z` format where X is the domain
   number, Y is the group, Z is the requirement
2. Each requirement uses `SHALL` for mandatory and `SHOULD` for
   recommended (RFC 2119)
3. Requirements are testable and measurable
4. Performance requirements include specific targets
   (e.g., "< 2ms per frame")
5. Requirements are NOT mapped 1:1 to features
6. No "Derived From" tables — requirements stand on their own
7. No same-domain feature links — cross-domain links only
8. Each requirement includes Rationale and Verification
9. Acceptance criteria belong here, not in user stories
10. Describe engine primitives only — no game-specific
    requirements

## Requirement Format

```markdown
N. **R-X.Y.Z** — The engine **SHALL** {requirement}.

- **Rationale:** {why}
- **Verification:** {how to test}
```

## What MUST NOT Be Included

| Prohibited content | Belongs in |
|--------------------|-----------|
| Feature definitions | `docs/features/` |
| User stories | `docs/user-stories/` |
| Design details | `docs/design/` |
| Pseudocode or implementation | `src/` |
| Persona definitions | `docs/user-stories/` |
| "Derived From" tables | Removed — not used |
| Same-domain feature links | Removed — cross-domain only |
| Game-specific requirements | Rewrite as engine primitives |

## When to Create New Files

- One file per domain topic in its domain subdirectory
  (e.g., `core-runtime/async-io.md`)
- Performance requirements alongside functional ones
- Cross-cutting requirements in `cross-cutting.md` at root

## Formatting Reference

- Headings: `## Domain` then numbered requirement list
- Body: `The engine SHALL...` statement
- Each requirement: Rationale + Verification
- Tables: `| ID | Requirement |` for summary
