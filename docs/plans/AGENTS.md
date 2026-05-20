# Plans — Agent Guide

All rules from the root [AGENTS.md](../../AGENTS.md) apply here.

## Purpose

Implementation plans pair every primary design under `docs/design/` with a `PLAN-*` file that
holds traceability (R-/F-/US-/TC- IDs), TDD sequencing, integration and constraint conformance
checks, and manual validation evidence pointers.

## Structure

```text
docs/plans/
  AGENTS.md
  index.md
  {domain}/
    {group}.md              — plan body
  cross-cutting/
    {topic}.md              — multi-domain plans
  integration/
    {pair}.md               — integration plans
  progress/
    PLAN-{slug}.md          — dated execution stubs
    phase-{specify,design,plan,release}.md
```

## ID format

| Kind            | Format                              | Example                                 |
|-----------------|-------------------------------------|-----------------------------------------|
| Plan            | `PLAN-{domain}-{slug}`              | `PLAN-core-runtime-graph-runtime`       |
| Progress stub   | same as Plan ID, in `progress/`     | `progress/PLAN-core-runtime-graph-runtime.md` |

Slugs match the design doc filename and never change after publication.

## Status authority

Two status fields exist and they answer different questions:

| Field                       | Where                                    | Meaning              |
|-----------------------------|------------------------------------------|----------------------|
| `status` in plan frontmatter | `{domain}/{slug}.md` plan body          | Intended state       |
| `status` in progress frontmatter | `progress/PLAN-{slug}.md`           | Execution state      |

The progress file is the source of truth for execution state. The plan-body `status` represents
the planning intent (e.g., `not_started` until the plan body itself is finalized). Coverage
matrices and phase rollups read the progress file.

## Rules

1. Every plan corresponds to exactly one design under `docs/design/`. Cross-cutting plans live
   in `cross-cutting/` and may map to no single design.
2. Every plan has a `progress/PLAN-{slug}.md` stub.
3. Plan IDs are immutable after publication. Slugs follow the design filename.
4. Plans cite explicit `R-`, `F-`, `US-`, and `TC-` IDs — never "see requirements" or "as
   specified".
5. Plans do not duplicate design content. They map design sections to TDD sequences.
6. Phase rollup files (`progress/phase-*.md`) summarize multi-plan state. They are append-only
   audit trails — do not rewrite history; add a new entry instead.

## When to create a new plan

| Trigger                                      | Plan placement                       |
|----------------------------------------------|---------------------------------------|
| New subsystem design                         | `{domain}/{slug}.md`                  |
| New integration pair design                  | `integration/{pair}.md`               |
| New cross-cutting deliverable                | `cross-cutting/{topic}.md`            |

Workflow tooling (`workflow` and `artifact` plugins from the marketplace) drives plan creation
and progress tracking. The retired `harmonize` and `plan-orchestrator` agents are no longer
used; legacy event-log entries in phase rollups are preserved as audit trail only.
