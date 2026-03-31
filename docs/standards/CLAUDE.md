# Coding Standards — Agent Guide

All rules from the root [CLAUDE.md](../../CLAUDE.md) apply here.

## Purpose

Language-specific coding standards defining formatting, linting, naming, testing, and best
practices.

## Structure

```text
docs/standards/
  README.md                 — standards index
  {language}.md             — per-language standard
```

Languages: `rust`, `swift`, `python`, `hlsl`, `markdown`, `json`, `toml`, `yaml`.

## Rules

1. Every standard follows this template:
   - Scope
   - Naming Conventions (table)
   - File Organization
   - Formatting Rules
   - Linting Rules
   - Type Checking
   - How to Check and Fix (table)
   - Project-Specific Rules
   - Cache-Friendly Patterns
   - Testing
   - Best Practices (numbered list)
   - Anti-Patterns (numbered list)
2. How to Check and Fix includes exact CLI commands
3. Testing section forbids mocking in all languages
4. Cache-Friendly Patterns covers data-oriented design

## What MUST NOT Be Included

| Prohibited content | Belongs in |
|--------------------|-----------|
| Design details | `docs/design/` |
| Requirements | `docs/requirements/` |
| Implementation code | `src/` |

## When to Create New Files

- One file per language or configuration format
- Add a new standard when a new language is introduced
- Update README.md index when adding a standard

## Formatting Reference

- Headings: `## Section` then `### Subsection`
- Tables: for naming conventions, check/fix commands
- Numbered lists: for best practices and anti-patterns
