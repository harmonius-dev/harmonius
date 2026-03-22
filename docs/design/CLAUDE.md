# Design Documents — Agent Guide

All rules from the root [CLAUDE.md](../../CLAUDE.md) apply here.

## Purpose

Design documents for every engine subsystem covering architecture, API, data flow, platform
considerations, and test plans.

## Structure

```text
docs/design/
  {domain}/
    {group}.md              — design document
    {group}-test-cases.md   — companion test cases
  constraints.md            — project-wide constraints
  plan.md                   — design plan and wave schedule
```

Domains: `ai`, `animation`, `audio`, `content-pipeline`, `core-runtime`, `game-framework`,
`geometry`, `input`, `networking`, `physics`, `platform`, `rendering`, `tools`, `ui`, `vfx`.

## Rules

1. Every design follows this template:
   - Requirements Trace
   - Overview
   - Architecture (Mermaid diagrams)
   - API Design (Rust pseudocode)
   - Data Flow
   - Platform Considerations
   - Test Plan (summary referencing companion file)
   - Open Questions
2. Every design MUST have a companion `{group}-test-cases.md` file
3. Every design MUST have a Mermaid `classDiagram` covering ALL types: structs, enums (with
   variants), traits, type aliases, and their relationships
4. All Mermaid diagrams must be rendered via MCP to verify correctness
5. All references MUST use specific IDs (R-X.Y.Z, F-X.Y.Z, US-X.Y.Z) — never "see requirements" or
   "as specified"

## Immutability and Functional Patterns

- Prefer immutable data structures by default
- Mutable containers holding immutable data over fully mutable structures
- Avoid `Arc`, `Rc`, `Cell`, `RefCell` — use owned values, generational indices, or scoped borrows
- Only make methods/arguments `&mut` when necessary
- When mutability is required for performance, document the tradeoff explicitly
- Pure functions for all transform pipelines — `fn(Input) -> Output` with no side effects

## What MUST NOT Be Included

| Prohibited content | Belongs in |
|--------------------|-----------|
| Requirements (`SHALL` statements) | `docs/requirements/` |
| Feature definitions (F-IDs) | `docs/features/` |
| User stories | `docs/user-stories/` |
| Business justification | `docs/features/` |
| Implementation code | `src/` |

## When to Create New Files

- One design file per feature group (see `plan.md`)
- Create companion `-test-cases.md` alongside every design
- Do not split a design unless it exceeds 1000 lines

## Test Case Companion Files

Every design has a companion `{group}-test-cases.md`:

- Test case IDs use `TC-X.Y.Z.N` format
- Every test row links to a specific R-X.Y.Z or F-X.Y.Z
- Input and expected output are explicit, not prose
- Categories: Unit Tests, Integration Tests, Benchmarks
- Benchmarks link to performance requirements (R-X.Y.Za)
- The design's Test Plan section becomes a summary referencing the companion file

## Formatting Reference

- Headings: `## Section` then `### Subsection`
- Diagrams: Mermaid only (class, flowchart, sequence)
- Code: Rust pseudocode in fenced blocks
- Tables: for requirements trace, API summary, platform comparison
