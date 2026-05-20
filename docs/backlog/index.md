# Backlog Index

Docs-native, GitHub-ready issue catalog. Source of truth for engineering and documentation work
items. See [AGENTS.md](AGENTS.md) for issue file format, [labels.md](labels.md) for the label
taxonomy, and [projects.md](projects.md) for the GitHub Projects board model.

## Counts (snapshot 2026-05-20)

| Status       | Count |
|--------------|------:|
| Triage       | 55    |
| Ready        | 0     |
| In Progress  | 0     |
| Review       | 0     |
| Blocked      | 0     |
| Done         | 0     |

55 issues seeded by the 2026-05 deslop pass. Counts refresh when
[docs/coverage/audits/](../coverage/audits/) is regenerated.

## How to find issues

| You want…                                  | Look at                                       |
|---------------------------------------------|-----------------------------------------------|
| All P0 / P1 work                           | Filter `priority` in [issues/](issues/)       |
| Open canonical-owner consolidations        | `BL-0001` … `BL-0014`                          |
| Open `design-review.md` items              | `BL-0015` … `BL-0042`                          |
| Misplaced / mis-formatted artifacts        | `BL-0043` … `BL-0049`                          |
| Mid/low-level design coverage roadmap      | `BL-0050` … `BL-0055`                          |

## Catalog

### Canonical-owner consolidations (`BL-0001` … `BL-0014`)

Each issue closes one row of [`design/canonical-owners.md`](../design/canonical-owners.md).

| ID                                                                | Title                                                  | Pri | Effort |
|-------------------------------------------------------------------|--------------------------------------------------------|-----|--------|
| [BL-0001](issues/BL-0001-modop-unification.md)                    | Unify `ModOp` across attributes-effects and containers | P1  | M      |
| [BL-0002](issues/BL-0002-shadingmodel-rename.md)                  | Resolve `ShadingModel` vs `ShadingModelId` duplication | P1  | S      |
| [BL-0003](issues/BL-0003-voicestream-ownership.md)                | Split `VoiceStream` (transport vs codec) ownership     | P1  | M      |
| [BL-0004](issues/BL-0004-uniformgrid-rename.md)                   | Rename gameplay grid `CellGrid`, AOI grid `AoiGrid`    | P1  | M      |
| [BL-0005](issues/BL-0005-conditionexpr-registry-scope.md)         | Decide `ConditionExpr` registry scope (global vs per)  | P1  | M      |
| [BL-0006](issues/BL-0006-blackboard-as-component.md)              | Replace `Blackboard` singleton with ECS component      | P1  | L      |
| [BL-0007](issues/BL-0007-compileerror-single-owner.md)            | Move `CompileError` to single owner (error.md)         | P1  | S      |
| [BL-0008](issues/BL-0008-hashmap-archetype.md)                    | Remove `HashMap` from ECS `Archetype` hot path         | P1  | M      |
| [BL-0009](issues/BL-0009-material-type-unification.md)            | Declare canonical `Material` type                      | P1  | L      |
| [BL-0010](issues/BL-0010-pso-cache-design.md)                     | Design pipeline-state-object cache                     | P1  | L      |
| [BL-0011](issues/BL-0011-meshlet-asset-pipeline.md)               | Specify meshlet asset pipeline end-to-end              | P1  | L      |
| [BL-0012](issues/BL-0012-2d-3d-render-graph-merge.md)             | Unify 2D and 3D render graph passes                    | P2  | L      |
| [BL-0013](issues/BL-0013-camera-split.md)                         | Camera split: brain vs lens (game-framework vs render) | P1  | M      |
| [BL-0014](issues/BL-0014-frameworld-promote.md)                   | Promote `FrameContext` to canonical type               | P2  | M      |

### Design-review backlog migration (`BL-0015` … `BL-0042`)

These are the still-open items from [`docs/design/design-review.md`](../design/design-review.md)
after the 2026-05 status update.

| ID                                                                | Title                                                   | Pri | Effort |
|-------------------------------------------------------------------|---------------------------------------------------------|-----|--------|
| [BL-0015](issues/BL-0015-game-loop-ecs-circular.md)               | Resolve game-loop ↔ ECS circular dependency            | P1  | M      |
| [BL-0016](issues/BL-0016-codegen-pipeline-doc.md)                 | Author `codegen-pipeline.md`                            | P2  | L      |
| [BL-0017](issues/BL-0017-skinning-weight-format.md)               | Specify skinning weight format                          | P2  | M      |
| [BL-0018](issues/BL-0018-cloth-fracture-formats.md)               | Specify cloth constraint and fracture pattern formats   | P2  | L      |
| [BL-0019](issues/BL-0019-fluid-solver-variant.md)                 | Pick fluid solver variant (SPH / FLIP / Eulerian)       | P2  | M      |
| [BL-0020](issues/BL-0020-animation-compression-acl.md)            | Specify ACL or equivalent animation compression         | P2  | M      |
| [BL-0021](issues/BL-0021-determinism-contracts.md)                | Determinism contracts for animation/proc/event-logs     | P2  | M      |
| [BL-0022](issues/BL-0022-quic-stream-multiplexing.md)             | Specify QUIC stream multiplexing policy                 | P1  | M      |
| [BL-0023](issues/BL-0023-replication-delta.md)                    | Specify replication delta compression                   | P1  | L      |
| [BL-0024](issues/BL-0024-save-schema-versioning.md)               | Specify save schema versioning + migration              | P1  | L      |
| [BL-0025](issues/BL-0025-shader-compiler-service.md)              | Consolidate shader compilation into one service         | P2  | M      |
| [BL-0026](issues/BL-0026-game-time-pause-determinism.md)          | `GameTime`, pause, determinism for simulation primitives | P1 | M      |
| [BL-0027](issues/BL-0027-pathfinding-algorithm-pick.md)           | Pick A* variant + nav-mesh parameters                   | P2  | M      |
| [BL-0028](issues/BL-0028-replicate-data-systems.md)               | Replication protocol for the four data primitives       | P2  | L      |
| [BL-0029](issues/BL-0029-data-systems-conflict-resolution.md)     | Conflict-resolution spec for effects/containers         | P2  | M      |
| [BL-0030](issues/BL-0030-data-tables-query-api.md)                | Data tables full query / secondary index API            | P2  | M      |
| [BL-0031](issues/BL-0031-grids-vs-bvh.md)                         | Grids-vs-BVH decision matrix                            | P2  | S      |
| [BL-0032](issues/BL-0032-anim-event-canonical.md)                 | Canonical `AnimEvent` location                          | P2  | S      |
| [BL-0033](issues/BL-0033-channel-capacity-formula.md)             | Adopt single capacity formula in capacities doc         | P3  | S      |
| [BL-0034](issues/BL-0034-undo-redo-deepen.md)                     | Deepen `tools/undo-redo.md` (history budget, sync)      | P3  | M      |
| [BL-0035](issues/BL-0035-input-ime-support.md)                    | Add IME support spec to input.md                        | P3  | M      |
| [BL-0036](issues/BL-0036-audio-ducking.md)                        | Specify audio ducking (voice > SFX > music)             | P3  | S      |
| [BL-0037](issues/BL-0037-hrtf-sofa-loading.md)                    | Specify HRTF SOFA profile loading                       | P3  | S      |
| [BL-0038](issues/BL-0038-anti-cheat-coverage.md)                  | Complete anti-cheat surface (input/teleport/recon)      | P2  | M      |
| [BL-0039](issues/BL-0039-accessibility-wcag.md)                   | Define WCAG compliance test plan                        | P3  | M      |
| [BL-0040](issues/BL-0040-async-cleanup-design.md)                 | Drain residual async/await prose in design tree         | P2  | M      |
| [BL-0041](issues/BL-0041-classic-steering-primitives.md)          | Add Align/Separate/ObstacleAvoid/Hide/Interpose         | P3  | S      |
| [BL-0042](issues/BL-0042-frame-context-promotion.md)              | Promote `FrameContext` to first-class type              | P2  | M      |

### Misplaced / mis-formatted artifacts (`BL-0043` … `BL-0049`)

These are the artifacts the deslop pass identified beyond the obvious surgical fixes.

| ID                                                                | Title                                                   | Pri | Effort |
|-------------------------------------------------------------------|---------------------------------------------------------|-----|--------|
| [BL-0043](issues/BL-0043-architecture-module-tables-audit.md)     | Audit `architecture.md` Module Reference for parity     | P1  | M      |
| [BL-0044](issues/BL-0044-thin-requirement-files.md)               | Backfill thin requirement files (≤4 SHALL statements)   | P2  | L      |
| [BL-0045](issues/BL-0045-thin-story-files.md)                     | Backfill thin user-story files                          | P2  | L      |
| [BL-0046](issues/BL-0046-game-specific-tagging.md)                | Apply `[game-specific]` tag consistently                 | P3  | M      |
| [BL-0047](issues/BL-0047-umbrella-stories-cleanup.md)             | Remove umbrella/parent compound stories                 | P3  | M      |
| [BL-0048](issues/BL-0048-readme-counts-sync.md)                   | Sync `requirements/`, `user-stories/`, `features/` README counts | P2 | S |
| [BL-0049](issues/BL-0049-plans-frontmatter-status.md)             | Reconcile plan frontmatter `status` with progress       | P2  | L      |

### Mid/low-level design coverage roadmap (`BL-0050` … `BL-0055`)

Each issue is one mid/low-level design split that
[`design/coverage-roadmap.md`](../design/coverage-roadmap.md) catalogs.

| ID                                                                | Title                                                   | Pri | Effort |
|-------------------------------------------------------------------|---------------------------------------------------------|-----|--------|
| [BL-0050](issues/BL-0050-audio-decompose.md)                      | Decompose audio domain into 5 mid-level designs         | P2  | XL     |
| [BL-0051](issues/BL-0051-input-decompose.md)                      | Decompose input domain into 5 mid-level designs         | P2  | XL     |
| [BL-0052](issues/BL-0052-ui-decompose.md)                         | Decompose UI domain (layout, rendering, accessibility)  | P2  | XL     |
| [BL-0053](issues/BL-0053-attributes-rendering-integration.md)     | Author missing integration: attributes ↔ rendering      | P2  | M      |
| [BL-0054](issues/BL-0054-event-logs-animation-integration.md)     | Author missing integration: event-logs ↔ animation      | P2  | M      |
| [BL-0055](issues/BL-0055-vfx-audio-integration.md)                | Author missing integration: vfx ↔ audio                 | P2  | M      |

## Sourcing

This backlog seeds from:

1. The 2026-04-12 [`design-review.md`](../design/design-review.md) tasks not yet
   landed.
2. The 2026-05-20 deslop audit findings recorded in
   [`coverage/audits/2026-05-audit.md`](../coverage/audits/2026-05-audit.md).
3. The [`design/canonical-owners.md`](../design/canonical-owners.md) duplicate-type
   table.
4. The [`design/coverage-roadmap.md`](../design/coverage-roadmap.md) split list.
