# OKR Cycle Index

Each OKR cycle is a calendar quarter. Cycle files are immutable once their retro is appended. New
goals always create a new file.

See [AGENTS.md](AGENTS.md) for cycle file format and authoring rules.

## Active and historical cycles

| Cycle                  | Dates                       | Theme                              | State   |
|------------------------|-----------------------------|------------------------------------|---------|
| [2026-q3](2026-q3.md)  | 2026-07-01 — 2026-09-30     | Foundations cohesion + backlog mig. | Active |

## Conventions

- OKRs reference [`docs/backlog/issues/`](../backlog/index.md) for the work that
  contributes to each KR.
- ADRs and PDRs are the *constraints* OKRs respect; OKRs are the *outcomes* that ADRs
  and PDRs make possible.
- Performance KRs cite the relevant requirement (R-X.Y.Z) and design doc directly.

## Roll-up

When a cycle closes:

1. Append a `## Retro` section to the cycle file.
2. Score each objective 0.0 – 1.0.
3. Move incomplete KRs into new backlog issues or carry them into the next cycle file.
4. Update the State column above to `Closed`.
