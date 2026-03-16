# Features — Agent Guide

## Purpose

This directory contains feature definitions for all Harmonius engine subsystems. Features define
WHAT the engine provides, organized by domain.

## Structure

```text
docs/features/
  README.md                 — feature index
  {domain}/
    {topic}.md              — feature definitions
```

Domains match the design directory structure.

## Rules

1. Feature IDs use `F-X.Y.Z` format where X is the domain number, Y is the group, Z is the feature
2. Each feature has: ID, name, description, priority, and acceptance criteria
3. Features describe capability, not implementation
4. Group related features under domain headings
5. Cross-reference requirements with R-X.Y.Z IDs
6. 100-character line limit (tables exempt)

## What MUST NOT Be Included

| Prohibited content | Belongs in |
|--------------------|-----------|
| Requirements (`SHALL` statements) | `docs/requirements/` |
| User stories | `docs/user-stories/` |
| Design details | `docs/design/` |
| Implementation code | `src/` |

## When to Create New Files

- One file per domain topic (e.g., `ai/navigation.md`)
- Split when a file exceeds 500 lines
- New domains require a new subdirectory

## Formatting Reference

- Headings: `## Domain` then `### F-X.Y.Z Feature Name`
- Tables: for feature summary, priority, dependencies
- Lists: for acceptance criteria
