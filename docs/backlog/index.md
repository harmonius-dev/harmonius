# Backlog Index

The backlog lives in [GitHub Issues](https://github.com/cjhowe-us/harmonius/issues). This file is
the navigation entry point. It links to filtered GitHub searches by category and priority.

See [`AGENTS.md`](AGENTS.md) for the issue contract, [`labels.md`](labels.md) for the label
taxonomy, [`projects.md`](projects.md) for the GitHub Projects board model, and
[`scripts/`](scripts/) for the one-time seed scripts that bootstrap a fresh repo from
[`seed.json`](seed.json).

## Snapshot 2026-05-20

| Status                           | Count |
|----------------------------------|------:|
| Seed entries in `seed.json`      |    55 |
| GitHub labels (canonical)        |    55 |
| GitHub issues (live)             |    55 |
| Engine Roadmap GitHub Project    | deferred (needs `project,read:project` scopes) |

The 2026-05 deslop pass authored the 55 entries in `seed.json` and materialized them into GitHub via
[`scripts/seed-labels.sh`](scripts/seed-labels.sh) and
[`scripts/seed-issues.sh`](scripts/seed-issues.sh). All 55 issues exist with `BL-NNNN: …` titles and
the labels listed in the seed. From this point on, **GitHub is the source of truth**; updates flow
through GitHub directly.

The Engine Roadmap GitHub Project is deferred until the operator's `gh` token includes `project` and
`read:project` scopes. Run [`scripts/seed-project.sh`](scripts/seed-project.sh) after
`gh auth refresh -s project,read:project --hostname github.com` completes.

The [2026-05 audit](../coverage/audits/2026-05-audit.md) breaks down the seed by priority and
domain.

## Filtered views (GitHub search URLs)

These URLs filter all GitHub issues by stable criteria. Bookmark them as your view into the backlog.

| View                          | URL                                                                                  |
|-------------------------------|--------------------------------------------------------------------------------------|
| All `BL-*` issues             | <https://github.com/cjhowe-us/harmonius/issues?q=BL-+in%3Atitle>                     |
| Open `BL-*` issues            | <https://github.com/cjhowe-us/harmonius/issues?q=BL-+in%3Atitle+is%3Aopen>           |
| Closed `BL-*` issues          | <https://github.com/cjhowe-us/harmonius/issues?q=BL-+in%3Atitle+is%3Aclosed>         |
| Priority P0                   | <https://github.com/cjhowe-us/harmonius/issues?q=BL-+in%3Atitle+label%3Ap0>           |
| Priority P1                   | <https://github.com/cjhowe-us/harmonius/issues?q=BL-+in%3Atitle+label%3Ap1>           |
| Priority P2                   | <https://github.com/cjhowe-us/harmonius/issues?q=BL-+in%3Atitle+label%3Ap2>           |
| Priority P3                   | <https://github.com/cjhowe-us/harmonius/issues?q=BL-+in%3Atitle+label%3Ap3>           |

## Categorical filters

The 55 seeded issues fall into four buckets. Each bucket has a stable filter URL.

### Canonical-owner consolidations (`BL-0001` … `BL-0014`)

Each issue closes one row of [`design/canonical-owners.md`](../design/canonical-owners.md).

Filter:
<https://github.com/cjhowe-us/harmonius/issues?q=BL-+in%3Atitle+label%3A%22type%3Atech-debt%22+label%3A%22coverage%3Adesign%22>

| ID       | Short title (search URL resolves to issue) |
|----------|--------------------------------------------|
| [BL-0001](https://github.com/cjhowe-us/harmonius/issues?q=BL-0001) | Unify `ModOp` across attributes-effects and containers-slots |
| [BL-0002](https://github.com/cjhowe-us/harmonius/issues?q=BL-0002) | Resolve `ShadingModel` vs `ShadingModelId` duplication       |
| [BL-0003](https://github.com/cjhowe-us/harmonius/issues?q=BL-0003) | Split `VoiceStream` ownership (transport vs codec)           |
| [BL-0004](https://github.com/cjhowe-us/harmonius/issues?q=BL-0004) | Rename gameplay grid `CellGrid`, AOI grid `AoiGrid`          |
| [BL-0005](https://github.com/cjhowe-us/harmonius/issues?q=BL-0005) | `ConditionExpr` registry scope                               |
| [BL-0006](https://github.com/cjhowe-us/harmonius/issues?q=BL-0006) | Replace `Blackboard` singleton with ECS component            |
| [BL-0007](https://github.com/cjhowe-us/harmonius/issues?q=BL-0007) | Move `CompileError` to single owner                          |
| [BL-0008](https://github.com/cjhowe-us/harmonius/issues?q=BL-0008) | Remove `HashMap` from ECS Archetype hot path                 |
| [BL-0009](https://github.com/cjhowe-us/harmonius/issues?q=BL-0009) | Declare canonical `Material` type                            |
| [BL-0010](https://github.com/cjhowe-us/harmonius/issues?q=BL-0010) | Pipeline-state-object cache                                  |
| [BL-0011](https://github.com/cjhowe-us/harmonius/issues?q=BL-0011) | Meshlet asset pipeline                                       |
| [BL-0012](https://github.com/cjhowe-us/harmonius/issues?q=BL-0012) | Unify 2D and 3D render graph                                 |
| [BL-0013](https://github.com/cjhowe-us/harmonius/issues?q=BL-0013) | Camera split (brain vs lens)                                 |
| [BL-0014](https://github.com/cjhowe-us/harmonius/issues?q=BL-0014) | Promote `FrameContext` to canonical type                     |

### Design-review backlog migration (`BL-0015` … `BL-0042`)

Open items from [`docs/design/design-review.md`](../design/design-review.md) Status Update.

Filter:
<https://github.com/cjhowe-us/harmonius/issues?q=BL-+in%3Atitle+label%3A%22domain%3Adesign-review%22>
(or by per-domain label).

| ID       | Short title |
|----------|-------------|
| [BL-0015](https://github.com/cjhowe-us/harmonius/issues?q=BL-0015) | Resolve game-loop ↔ ECS circular dependency |
| [BL-0016](https://github.com/cjhowe-us/harmonius/issues?q=BL-0016) | Author `codegen-pipeline.md`                |
| [BL-0017](https://github.com/cjhowe-us/harmonius/issues?q=BL-0017) | Skinning weight format                       |
| [BL-0018](https://github.com/cjhowe-us/harmonius/issues?q=BL-0018) | Cloth + fracture formats                    |
| [BL-0019](https://github.com/cjhowe-us/harmonius/issues?q=BL-0019) | Fluid solver variant                         |
| [BL-0020](https://github.com/cjhowe-us/harmonius/issues?q=BL-0020) | Animation compression (ACL)                  |
| [BL-0021](https://github.com/cjhowe-us/harmonius/issues?q=BL-0021) | Determinism contracts                        |
| [BL-0022](https://github.com/cjhowe-us/harmonius/issues?q=BL-0022) | QUIC stream multiplexing                     |
| [BL-0023](https://github.com/cjhowe-us/harmonius/issues?q=BL-0023) | Replication delta                            |
| [BL-0024](https://github.com/cjhowe-us/harmonius/issues?q=BL-0024) | Save schema versioning                       |
| [BL-0025](https://github.com/cjhowe-us/harmonius/issues?q=BL-0025) | Single shader-compiler service               |
| [BL-0026](https://github.com/cjhowe-us/harmonius/issues?q=BL-0026) | `GameTime`, pause, determinism for sim       |
| [BL-0027](https://github.com/cjhowe-us/harmonius/issues?q=BL-0027) | Pathfinding algorithm pick                   |
| [BL-0028](https://github.com/cjhowe-us/harmonius/issues?q=BL-0028) | Replicate data-systems primitives            |
| [BL-0029](https://github.com/cjhowe-us/harmonius/issues?q=BL-0029) | Data systems conflict resolution             |
| [BL-0030](https://github.com/cjhowe-us/harmonius/issues?q=BL-0030) | Data tables query API                        |
| [BL-0031](https://github.com/cjhowe-us/harmonius/issues?q=BL-0031) | Grids vs BVH decision matrix                 |
| [BL-0032](https://github.com/cjhowe-us/harmonius/issues?q=BL-0032) | `AnimEvent` canonical location               |
| [BL-0033](https://github.com/cjhowe-us/harmonius/issues?q=BL-0033) | Channel capacity formula                     |
| [BL-0034](https://github.com/cjhowe-us/harmonius/issues?q=BL-0034) | Undo / redo deepening                        |
| [BL-0035](https://github.com/cjhowe-us/harmonius/issues?q=BL-0035) | Input IME support                            |
| [BL-0036](https://github.com/cjhowe-us/harmonius/issues?q=BL-0036) | Audio ducking                                 |
| [BL-0037](https://github.com/cjhowe-us/harmonius/issues?q=BL-0037) | HRTF SOFA loading                            |
| [BL-0038](https://github.com/cjhowe-us/harmonius/issues?q=BL-0038) | Anti-cheat coverage                          |
| [BL-0039](https://github.com/cjhowe-us/harmonius/issues?q=BL-0039) | WCAG compliance                              |
| [BL-0040](https://github.com/cjhowe-us/harmonius/issues?q=BL-0040) | Async cleanup in design                      |
| [BL-0041](https://github.com/cjhowe-us/harmonius/issues?q=BL-0041) | Classic steering primitives                  |
| [BL-0042](https://github.com/cjhowe-us/harmonius/issues?q=BL-0042) | Adopt `FrameContext` in integration docs     |

### Misplaced / mis-formatted artifacts (`BL-0043` … `BL-0049`)

Filter: <https://github.com/cjhowe-us/harmonius/issues?q=BL-+in%3Atitle+label%3A%22domain%3Ameta%22>

| ID       | Short title |
|----------|-------------|
| [BL-0043](https://github.com/cjhowe-us/harmonius/issues?q=BL-0043) | Architecture Module Reference parity audit  |
| [BL-0044](https://github.com/cjhowe-us/harmonius/issues?q=BL-0044) | Backfill thin requirement files             |
| [BL-0045](https://github.com/cjhowe-us/harmonius/issues?q=BL-0045) | Backfill thin user-story files              |
| [BL-0046](https://github.com/cjhowe-us/harmonius/issues?q=BL-0046) | Apply `[game-specific]` tag consistently    |
| [BL-0047](https://github.com/cjhowe-us/harmonius/issues?q=BL-0047) | Remove umbrella / parent stories            |
| [BL-0048](https://github.com/cjhowe-us/harmonius/issues?q=BL-0048) | Sync README counts                           |
| [BL-0049](https://github.com/cjhowe-us/harmonius/issues?q=BL-0049) | Plan frontmatter status reconciliation       |

### Mid/low-level design coverage roadmap (`BL-0050` … `BL-0055`)

Filter:
<https://github.com/cjhowe-us/harmonius/issues?q=BL-+in%3Atitle+label%3A%22effort%3Axl%22+OR+label%3A%22type%3Adesign%22>

| ID       | Short title |
|----------|-------------|
| [BL-0050](https://github.com/cjhowe-us/harmonius/issues?q=BL-0050) | Audio decompose                              |
| [BL-0051](https://github.com/cjhowe-us/harmonius/issues?q=BL-0051) | Input decompose                              |
| [BL-0052](https://github.com/cjhowe-us/harmonius/issues?q=BL-0052) | UI decompose                                 |
| [BL-0053](https://github.com/cjhowe-us/harmonius/issues?q=BL-0053) | attributes ↔ rendering integration           |
| [BL-0054](https://github.com/cjhowe-us/harmonius/issues?q=BL-0054) | event-logs ↔ animation integration           |
| [BL-0055](https://github.com/cjhowe-us/harmonius/issues?q=BL-0055) | vfx ↔ audio integration                      |

## Sourcing

The seed catalog [`seed.json`](seed.json) was assembled from:

1. The 2026-04-12 [`design-review.md`](../design/design-review.md) tasks not yet landed.
2. The 2026-05-20 deslop audit findings recorded in
   [`coverage/audits/2026-05-audit.md`](../coverage/audits/2026-05-audit.md).
3. The [`design/canonical-owners.md`](../design/canonical-owners.md) duplicate-type table.
4. The [`design/coverage-roadmap.md`](../design/coverage-roadmap.md) split list.
