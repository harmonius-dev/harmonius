# OKRs — Agent Guide

All rules from the root [AGENTS.md](../../AGENTS.md) apply here.

## Purpose

Quarterly Objectives and Key Results (OKRs) anchor docs and design work to measurable outcomes. They
link the design-review backlog and `docs/backlog/` issues to specific quarter-bounded targets. OKRs
are not mandates for individuals; they describe what "done" looks like for a unit of work in a given
cycle.

## Structure

```text
docs/okrs/
  AGENTS.md
  index.md
  YYYY-qN.md
```

`YYYY-qN.md` files are calendar-quarter cycle files (`2026-q3.md`, etc.). Each cycle file owns its
objectives until the cycle closes and a retro is appended.

## ID format

| Kind        | Format       | Example      |
|-------------|--------------|--------------|
| Objective   | `O-N`        | `O-2`        |
| Key Result  | `KR-N.M`     | `KR-2.3`     |

IDs reset each cycle file. The cycle file is the namespace. To reference an OKR from elsewhere, use
`2026-q3:O-2` or `2026-q3:KR-2.3`.

## Required sections (cycle file)

1. **Title and dates** — `# 2026 Q3 OKRs` plus `Cycle: 2026-07-01 — 2026-09-30`.
2. **Context** — one paragraph explaining the cycle theme.
3. **Objectives** — each objective gets a heading `## O-N — {Title}` with:
   - One-paragraph rationale.
   - Table of key results: `| ID | Key Result | Baseline | Target | Status |`.
   - Optional links to backlog issues that contribute.
4. **Retro** — appended at end of cycle. `### O-N retro` for each objective with score
   (0.0–1.0), what worked, what did not, follow-ups.

## Rules

1. OKRs are aspirational — score 0.7 is a "good" outcome, 1.0 is "lucky". Avoid
   sandbagging.
2. Key Results are measurable. Replace adjectives with numbers (counts, percentages,
   latencies, file counts).
3. Each cycle file lists no more than 5 objectives, each with no more than 5 KRs.
4. Cycle files are immutable after the retro is appended. New goals create a new file.
5. KRs must reference at least one backlog issue, ADR, PDR, or design doc.
6. Link from [index.md](index.md) on cycle creation.

## When to create a new cycle file

- Start of a calendar quarter (the working cadence).
- Mid-cycle pivot is allowed but rare; record by appending an "Amendment" subsection at
  the bottom of the in-flight cycle file with date and rationale, never by editing
  prior text.
