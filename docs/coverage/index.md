# Coverage Index

This is the entry point for coverage matrices and dated audits. Coverage answers two
questions: (1) does every requirement / user story have matching feature, design, and
test artifacts; (2) does every artifact trace back to a requirement or user story.

See [AGENTS.md](AGENTS.md) for matrix and audit format.

## Matrices

| Matrix                                                                         | Purpose                                                  |
|--------------------------------------------------------------------------------|----------------------------------------------------------|
| [requirements-matrix.md](requirements-matrix.md)                               | `R-X.Y.Z` → feature / user-story / design / test         |
| [user-stories-matrix.md](user-stories-matrix.md)                               | `US-X.Y.Z` → persona / requirement / design / test       |

## Audits

| Audit                                                                          | Date       | Branch                                |
|--------------------------------------------------------------------------------|------------|---------------------------------------|
| [audits/2026-05-audit.md](audits/2026-05-audit.md)                             | 2026-05-20 | `cursor/deslop-docs-corpus-e306`      |

## Generation

Matrices and audits are produced by inspecting the filesystem (Glob + Grep). The
canonical recipe:

1. Enumerate every `R-X.Y.Z` in `docs/requirements/`, every `US-X.Y.Z` in
   `docs/user-stories/`, every `F-X.Y.Z` in `docs/features/`, every `TC-X.Y.Z.N` in
   `docs/design/*/*-test-cases.md`.
2. Map each ID to its source file and to the cross-references the source contains.
3. Produce per-domain tables and orphan lists.
4. Write the audit dated `YYYY-MM-audit.md`.

The current matrices are partial — they fully cover Core Runtime, Data Systems,
Simulation, and Networking. Other domains carry a "Summary only" line and are tracked
by [`docs/backlog/issues/`](../backlog/index.md) for completion.

## Coverage gates (advisory)

| Gate                                                       | Threshold |
|------------------------------------------------------------|-----------|
| Each requirement has a feature link                        | 100%      |
| Each requirement has at least one user-story               | ≥ 90%     |
| Each requirement has at least one test-case row            | 100%      |
| Each user-story has a requirement link                     | ≥ 90%     |
| Each design doc has a companion `*-test-cases.md`          | 100%      |
| Each integration doc references `shared-conventions.md`    | 100%      |

These gates are advisory until promoted to ADRs (proposed: ADR-0015 to make them
normative).
