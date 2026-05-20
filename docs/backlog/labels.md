# Backlog Label Taxonomy

GitHub label model used by all issues. Labels are sorted alphabetically inside each
group when listed in issue frontmatter. Add new labels by amending this file first.

## Priority

| Label  | Meaning                                                            |
|--------|--------------------------------------------------------------------|
| `p0`   | Blocks an in-flight ADR/release                                    |
| `p1`   | Required for the current OKR cycle                                 |
| `p2`   | Required for the next OKR cycle                                    |
| `p3`   | Track only; do not staff                                           |

## Type

| Label              | Meaning                                                  |
|--------------------|----------------------------------------------------------|
| `type:adr`         | Authoring or amending an Architecture Decision Record    |
| `type:bug`         | Documentation contradicts implementation or itself       |
| `type:design`      | Design doc work (new file, refactor, mid/low-level split) |
| `type:feature`     | Feature index/spec changes                               |
| `type:infra`       | OSS-stack, K8s, deployment, observability                |
| `type:pdr`         | Authoring or amending a Product Decision Record          |
| `type:plan`        | Implementation plan or progress changes                  |
| `type:requirement` | R-X.Y.Z creation, rewrite, or audit                      |
| `type:tech-debt`   | Cleanup, deduplication, redundancy removal               |
| `type:user-story`  | US-X.Y.Z creation, rewrite, or audit                     |

## Status

| Label                | Meaning                                          |
|----------------------|--------------------------------------------------|
| `status:triage`      | Newly filed; not yet sized or scheduled          |
| `status:ready`       | Sized; can be picked up                          |
| `status:blocked`     | Has unmet `blocked_by` issue(s)                  |
| `status:in-progress` | Actively being worked                            |
| `status:review`      | Submitted for review                             |
| `status:done`        | Acceptance criteria met                          |

## Effort

| Label       | Engineering size                                                         |
|-------------|--------------------------------------------------------------------------|
| `effort:s`  | Single doc; one section; ≤2 hours of focused writing                     |
| `effort:m`  | Multiple sections of one doc, or one new short doc; ½ day                |
| `effort:l`  | New design doc with companion test-cases; ~1 day                         |
| `effort:xl` | Multi-doc refactor or coordinated cross-tree edits                       |

## Domain (one per issue)

`domain:ai`, `domain:animation`, `domain:audio`, `domain:business`,
`domain:content-pipeline`, `domain:core-runtime`, `domain:cross-cutting`,
`domain:data-systems`, `domain:decisions`, `domain:design-review`,
`domain:game-framework`, `domain:geometry`, `domain:input`, `domain:integration`,
`domain:meta`, `domain:networking`, `domain:okrs`, `domain:physics`, `domain:platform`,
`domain:plans`, `domain:rendering`, `domain:simulation`, `domain:tools`, `domain:ui`,
`domain:vfx`.

`domain:meta` covers the four programs themselves (decisions, okrs, backlog,
coverage). `domain:cross-cutting` covers `cross-cutting.md`-scoped requirements and
performance budgets. `domain:integration` covers `design/integration/` pair docs.
`domain:design-review` covers the `design/design-review.md` backlog migration.

## Component (optional, multiple allowed)

Free-form short tag for the specific component the issue touches:
`component:ecs`, `component:game-loop`, `component:scheduler`, `component:render-graph`,
`component:asset-pipeline`, `component:hot-reload`, `component:editor-core`,
`component:profiler`, `component:save-system`, `component:network-transport`, …

Add new component tags in this file before applying.

## Coverage gate (optional)

| Label              | Meaning                                                  |
|--------------------|----------------------------------------------------------|
| `coverage:design`  | Closes a design-coverage gap                             |
| `coverage:reqs`    | Closes a requirements-coverage gap                       |
| `coverage:stories` | Closes a user-story coverage gap                         |
| `coverage:tests`   | Closes a test-case coverage gap                          |
| `coverage:audit`   | Audit-driven cleanup                                     |
