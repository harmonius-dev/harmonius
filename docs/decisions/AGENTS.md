# Decisions — Agent Guide

All rules from the root [AGENTS.md](../../AGENTS.md) apply here.

## Purpose

Architecture Decision Records (ADRs) and Product Decision Records (PDRs) capture the
"why" behind significant architectural and product choices. ADRs document engineering
decisions that shape the codebase. PDRs document product, monetization, and policy
decisions that downstream engineering must respect.

## Structure

```text
docs/decisions/
  AGENTS.md
  index.md
  adr/
    ADR-NNNN-{slug}.md
  pdr/
    PDR-NNNN-{slug}.md
```

## ID format

| Kind | Format            | Example                      |
|------|-------------------|------------------------------|
| ADR  | `ADR-NNNN-{slug}` | `ADR-0002-vulkan-sole-gpu`   |
| PDR  | `PDR-NNNN-{slug}` | `PDR-0001-apache-2-no-royal` |

`NNNN` is a four-digit zero-padded ordinal that never repeats. Slugs are kebab-case,
short, and stable (an ADR's slug must not change once published; the slug may shorten a
long title at write-time, but never edit it later).

## Status lifecycle

| Status       | Meaning                                                                |
|--------------|------------------------------------------------------------------------|
| `Proposed`   | Drafted, not accepted yet                                              |
| `Accepted`   | Active, current decision                                               |
| `Superseded` | A later ADR/PDR replaces this one — link the successor                 |
| `Deprecated` | Decision still on record but no longer applicable                      |
| `Reversed`   | Decision was Accepted and later explicitly undone (rare; link reversal) |

A `Superseded` record stays in place — never delete or rewrite history. Add a
"Superseded by" link at the top, and update its successor with a "Supersedes" link.

## Required sections (ADR)

1. **Title** — single line, sentence case, drops the `ADR-NNNN` prefix.
2. **Status** — one of the lifecycle values, plus date.
3. **Context** — one paragraph; the constraint or pressure the decision answers.
4. **Decision** — one paragraph; what we will do.
5. **Consequences** — bulleted list of trade-offs, follow-ups, and obligations.
6. **Alternatives Considered** — bulleted list with one-line rationale per option.
7. **References** — links to design docs, requirements, constraints, prior art.

## Required sections (PDR)

1. **Title** — single line, sentence case.
2. **Status** — one of the lifecycle values, plus date.
3. **Context** — market, customer, or business pressure.
4. **Decision** — the policy.
5. **Engineering implications** — what code must do or not do as a consequence.
6. **Reversibility** — what it would take to undo this decision.
7. **References** — links to business docs, monetization analysis, ADRs.

## Rules

1. ADRs answer engineering questions; PDRs answer product/policy questions. If a record
   is mostly engineering with one policy hook, keep it as an ADR and reference a PDR.
2. Each record fits on one page (≤300 lines including frontmatter).
3. Never edit an `Accepted` ADR's Decision text in place; cut a new ADR that supersedes
   it. Editorial fixes (typos, broken links) are allowed.
4. Decision IDs are immutable. Slugs are immutable after publication.
5. Every ADR/PDR is listed in [index.md](index.md) with status and date.
6. Cross-link from `architecture.md` and `design/constraints.md` to the relevant ADRs.
7. Do not include implementation pseudocode. Decisions reference designs; designs hold
   the code shape.

## When to create a new record

| Trigger                                                           | Record |
|-------------------------------------------------------------------|--------|
| Cross-cutting choice that constrains multiple designs             | ADR    |
| Choosing among 2+ comparable libraries / patterns / architectures | ADR    |
| Reversing or significantly amending a prior decision              | ADR    |
| Pricing, licensing, royalties, or marketplace policy              | PDR    |
| AI ethics or privacy posture                                      | PDR    |
| Console / platform-holder commercial terms                        | PDR    |
| Distribution or publishing terms                                  | PDR    |
