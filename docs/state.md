# Repo State Snapshot

Captured on 2026-04-18 after worktree and cache cleanup.

## Summary

| Metric | Before | After |
|--------|-------:|------:|
| Worktrees | 117 | 1 (main) |
| Remote `plan/*` branches | 126 | 121 |
| Local `plan/*` branches | 139 | 121 |
| `target/` + `.rumdl_cache/` dirs | 200+ | 0 |
| Ad-hoc tracking files in root | 35 | 0 |

## Cleanup log

1. Removed untracked tracking artifacts from root: `rebase-77-*`, `pr-77*`, `merge-*.log`,
   `worktree-*`, `gh-*`, `pwd.txt`, `mergeable*`, `ref-check.txt`, `terminal-test.txt`,
   `branch-check.txt`.
2. Staged deletions for tracked tracking files: `docs/plans/in-flight.md`, `docs/plans/locks.md`,
   `docs/plans/.ready_dispatch.tsv`.
3. Removed `docs/plans/harmonize-run-lock.md` and `docs/plans/worktree-state.json` (untracked
   harmonize run state).
4. Wiped all `target/` and `.rumdl_cache/` directories from the main worktree and every plan
   worktree.
5. Removed all 116 plan worktrees with `git worktree remove`.
6. Deleted 5 remote branches that had zero commits ahead of `origin/main`.
7. Deleted 13 stale local-only branches that had zero commits ahead of `origin/main` and no
   corresponding remote.

## Dropped branches (no significant progress)

Removed from both remote and local. Each pointed at an older snapshot of `main` with no unique work.

| Branch | Tip commit |
|--------|------------|
| `plan/content-pipeline-asset-processing` | `06dad97` |
| `plan/core-runtime-graph-runtime` | `06dad97` |
| `plan/core-runtime-hot-reload-protocol` | `06dad97` |
| `plan/geometry-world-geometry` | `a53a91b` |
| `plan/integration-attributes-effects-animation` | `a53a91b` |

## Stale local-only branches removed

All 13 had zero commits ahead of `origin/main` and no matching remote.

1. `plan/animation-skeletal`
2. `plan/animation-state-machine`
3. `plan/audio-audio`
4. `plan/core-runtime-change-detection`
5. `plan/core-runtime-ecs`
6. `plan/core-runtime-game-loop`
7. `plan/game-framework-scripting`
8. `plan/integration-animation-audio`
9. `plan/integration-animation-timelines`
10. `plan/integration-animation-vfx`
11. `plan/integration-data-tables-ui`
12. `plan/integration-geometry-vfx`
13. `plan/physics-foundation`

## Outstanding state

### `plan/integration-attributes-effects-physics` (diverged)

- Local ref at `b6ceebc`, remote ref at `ed350cc`.
- Diverges 70 / 2 commits. Local was rebased onto an older `main` before the branch was rebased on
  the remote side.
- Top two commits on each side carry the same subjects — the feature work
  (`feat(integration): IR-2.6 attributes-effects physics contracts`) exists on both. Tree diff vs
  remote is mostly progress-note drift for unrelated plans.
- **Source of truth**: the remote. PR #67 tracks the remote branch.
- **Suggested follow-up**: delete the local ref once the PR merges:
  `git branch -D plan/integration-attributes-effects-physics`.

### `plan/core-runtime-memory-async-io` (rebase aborted)

- The worktree was mid interactive rebase onto `e8a86eb` with all conflicts resolved but no
  `git rebase --continue` completion.
- Rebase was aborted; branch is now clean and identical to
  `origin/plan/core-runtime-memory-async-io`.
- PR #77 still tracks this branch and can be rebased again from a fresh worktree when work resumes.

## Active plan branches

121 branches retained on the remote. Each has an open PR (verified `gh pr list --state open` on
2026-04-18). Count is commits ahead of `origin/main`.

### AI (3)

| Branch | Commits |
|--------|--------:|
| `plan/ai-behavior` | 2 |
| `plan/ai-navigation` | 3 |
| `plan/ai-steering-crowds` | 4 |

Last commits:

1. `ai-behavior` — review: pr-reviewer — test hygiene, GOAP planner docs
2. `ai-navigation` — review: mark PLAN-ai-navigation submitted after PR review
3. `ai-steering-crowds` — review: address PR 53 supervisor findings

### Animation (1)

| Branch | Commits |
|--------|--------:|
| `plan/animation-procedural` | 3 |

Last commits:

1. `animation-procedural` — review: address supervisor findings on procedural

### Content Pipeline (1)

| Branch | Commits |
|--------|--------:|
| `plan/content-pipeline-asset-pipeline` | 5 |

Last commits:

1. `content-pipeline-asset-pipeline` — review: address PR 79 supervisor findings

### Core Runtime (11)

| Branch | Commits |
|--------|--------:|
| `plan/core-runtime-algorithms` | 1 |
| `plan/core-runtime-console-variables` | 1 |
| `plan/core-runtime-error` | 5 |
| `plan/core-runtime-events-plugins` | 3 |
| `plan/core-runtime-ids` | 2 |
| `plan/core-runtime-io` | 3 |
| `plan/core-runtime-memory-async-io` | 3 |
| `plan/core-runtime-primitives` | 3 |
| `plan/core-runtime-reflection-serialization` | 3 |
| `plan/core-runtime-scene-transforms` | 4 |
| `plan/core-runtime-spatial-index` | 4 |

Last commits:

1. `core-runtime-algorithms` — feat(core): add HandleMap for TC-1.7.4.1-4
2. `core-runtime-console-variables` — feat(core-runtime): add ConVar registry
3. `core-runtime-error` — review: address supervisor findings on error PR
4. `core-runtime-events-plugins` — review: deterministic graph and event API
5. `core-runtime-ids` — docs(plan): PLAN-core-runtime-ids progress to code_complete
6. `core-runtime-io` — review: address PR 60 consolidated findings
7. `core-runtime-memory-async-io` — review: fix progress rumdl, event log timestamp
8. `core-runtime-primitives` — review: address PR 58 supervisor findings
9. `core-runtime-reflection-serialization` — progress: submitted for human review
10. `core-runtime-scene-transforms` — docs: drop broken test-case audit link
11. `core-runtime-spatial-index` — review: rumdl gate + progress submitted for PR 73

### Cross Cutting (2)

| Branch | Commits |
|--------|--------:|
| `plan/cross-cutting-design-constraints` | 2 |
| `plan/cross-cutting-design-review-resolution` | 2 |

Last commits:

1. `cross-cutting-design-constraints` — green: design constraints repo scans
2. `cross-cutting-design-review-resolution` — green: align hot-reload feature copy

### Data Systems (5)

| Branch | Commits |
|--------|--------:|
| `plan/data-systems-attributes-effects` | 3 |
| `plan/data-systems-composition` | 4 |
| `plan/data-systems-containers-slots` | 4 |
| `plan/data-systems-data-tables` | 3 |
| `plan/data-systems-directed-graphs` | 3 |

Last commits:

1. `data-systems-attributes-effects` — docs: fix README audit link for rumdl MD057
2. `data-systems-composition` — review: composition PR correctness and tests
3. `data-systems-containers-slots` — review: log pr-reviewer outcome in progress
4. `data-systems-data-tables` — docs: sync PLAN-data-systems-data-tables progress
5. `data-systems-directed-graphs` — review: fix dead-node DCE and traversal order

### Game Framework (3)

| Branch | Commits |
|--------|--------:|
| `plan/game-framework-camera` | 4 |
| `plan/game-framework-localization` | 2 |
| `plan/game-framework-save-system` | 3 |

Last commits:

1. `game-framework-camera` — review: address PR 52 review-supervisor findings
2. `game-framework-localization` — review: localization PR24 review fixes
3. `game-framework-save-system` — green: TC-13.3.4.4 transactional copy_slot rollback

### Geometry (1)

| Branch | Commits |
|--------|--------:|
| `plan/geometry-procedural-generation` | 2 |

Last commits:

1. `geometry-procedural-generation` — review: remove broken README link for rumdl

### Input (1)

| Branch | Commits |
|--------|--------:|
| `plan/input-input` | 4 |

Last commits:

1. `input-input` — review: clippy tests, README link, scope docs

### Integration (55)

| Branch | Commits |
|--------|--------:|
| `plan/integration-ai-animation` | 11 |
| `plan/integration-ai-data-tables` | 5 |
| `plan/integration-ai-event-logs` | 3 |
| `plan/integration-ai-grids-volumes` | 4 |
| `plan/integration-ai-physics` | 6 |
| `plan/integration-ai-scripting` | 3 |
| `plan/integration-ai-spatial-awareness` | 4 |
| `plan/integration-animation-physics` | 4 |
| `plan/integration-animation-rendering` | 4 |
| `plan/integration-asset-pipeline-build-deploy` | 5 |
| `plan/integration-asset-pipeline-rendering` | 3 |
| `plan/integration-attributes-effects-physics` | 2 |
| `plan/integration-audio-camera` | 3 |
| `plan/integration-audio-physics` | 4 |
| `plan/integration-audio-spatial-awareness` | 3 |
| `plan/integration-containers-slots-rendering` | 5 |
| `plan/integration-containers-slots-ui` | 4 |
| `plan/integration-directed-graphs-scripting` | 1 |
| `plan/integration-editor-animation` | 5 |
| `plan/integration-editor-asset-pipeline` | 4 |
| `plan/integration-editor-core-runtime` | 4 |
| `plan/integration-editor-physics` | 1 |
| `plan/integration-editor-rendering` | 5 |
| `plan/integration-event-logs-ui` | 3 |
| `plan/integration-grids-volumes-physics` | 3 |
| `plan/integration-high-level` | 2 |
| `plan/integration-input-camera` | 2 |
| `plan/integration-input-ui` | 3 |
| `plan/integration-localization-ui` | 3 |
| `plan/integration-networking-audio` | 3 |
| `plan/integration-networking-ecs` | 6 |
| `plan/integration-networking-physics` | 2 |
| `plan/integration-networking-save-system` | 4 |
| `plan/integration-physics-geometry` | 2 |
| `plan/integration-physics-spatial-index` | 3 |
| `plan/integration-profiler-game-loop` | 4 |
| `plan/integration-profiler-rendering` | 2 |
| `plan/integration-rendering-camera` | 3 |
| `plan/integration-rendering-geometry` | 2 |
| `plan/integration-rendering-grids-volumes` | 1 |
| `plan/integration-rendering-physics` | 1 |
| `plan/integration-rendering-scripting` | 1 |
| `plan/integration-rendering-ui` | 1 |
| `plan/integration-rendering-vfx` | 1 |
| `plan/integration-save-system-profiler` | 2 |
| `plan/integration-save-system-serialization` | 3 |
| `plan/integration-scripting-data-tables` | 1 |
| `plan/integration-scripting-ecs` | 2 |
| `plan/integration-scripting-ui` | 1 |
| `plan/integration-shared-conventions` | 4 |
| `plan/integration-shared-messaging-capacities` | 1 |
| `plan/integration-timelines-audio` | 1 |
| `plan/integration-timelines-camera` | 2 |
| `plan/integration-timelines-scripting` | 2 |
| `plan/integration-ui-physics` | 2 |

Last commits:

1. `integration-ai-animation` — fix: remove broken README audit link for rumdl
2. `integration-ai-data-tables` — review: FM7 snapshot invalidation and BT parity
3. `integration-ai-event-logs` — review: address PR #55 supervisor findings
4. `integration-ai-grids-volumes` — docs: update integration test count
5. `integration-ai-physics` — review: address PR 67 consolidated findings
6. `integration-ai-scripting` — review: address supervisor bridge and scope notes
7. `integration-ai-spatial-awareness` — review: refresh progress after supervisor
8. `integration-animation-physics` — review: address animation-physics PR findings
9. `integration-animation-rendering` — review: address animation-rendering PR
10. `integration-asset-pipeline-build-deploy` — docs(progress): rumdl line wrap
11. `integration-asset-pipeline-rendering` — chore(plan): mark submitted
12. `integration-attributes-effects-physics` — docs(plan): mark code complete
13. `integration-audio-camera` — review: advance prev on full audio queue
14. `integration-audio-physics` — review: MPSC-safe queue and cooldown FIFO
15. `integration-audio-spatial-awareness` — review: address PR #7 findings
16. `integration-containers-slots-rendering` — review: address PR review findings
17. `integration-containers-slots-ui` — docs(progress): sync progress
18. `integration-directed-graphs-scripting` — feat: directed graphs ↔ scripting
19. `integration-editor-animation` — review: bone overlay outcome and TC tests
20. `integration-editor-asset-pipeline` — review: address PR findings
21. `integration-editor-core-runtime` — review: FM-1 drain order, undo pairs
22. `integration-editor-physics` — feat: editor-physics integration harness
23. `integration-editor-rendering` — chore(plan): mark code_complete
24. `integration-event-logs-ui` — feat: add harmonius_event_logs_ui integration
25. `integration-grids-volumes-physics` — review: supervisor findings on seam
26. `integration-high-level` — green: high-level channel capacities
27. `integration-input-camera` — docs(progress): code complete
28. `integration-input-ui` — review: input_ui clippy, dispatch spine, IR-4.2.5
29. `integration-localization-ui` — review: add B1 benches and close review gaps
30. `integration-networking-audio` — review: address PR supervisor findings
31. `integration-networking-ecs` — review: avoid broken rustdoc link
32. `integration-networking-physics` — merge: origin/main into branch
33. `integration-networking-save-system` — docs: sync progress with main
34. `integration-physics-geometry` — review: address PR supervisor findings
35. `integration-physics-spatial-index` — docs: fix README audit link for rumdl
36. `integration-profiler-game-loop` — review: fix profiler scope balance
37. `integration-profiler-rendering` — review: address PR supervisor findings
38. `integration-rendering-camera` — bench: add view_uniform four-view pack
39. `integration-rendering-geometry` — docs(plans): link PR 101
40. `integration-rendering-grids-volumes` — feat: CPU contracts
41. `integration-rendering-physics` — Add rendering-physics integration crate
42. `integration-rendering-scripting` — feat: contracts crate
43. `integration-rendering-ui` — green: TC-IR-3.6 phase, batch, platform
44. `integration-rendering-vfx` — Add integration_rendering_vfx crate and tests
45. `integration-save-system-profiler` — chore(progress): mark code complete
46. `integration-save-system-serialization` — feat: add harmonius_save_integration
47. `integration-scripting-data-tables` — feat(integration): IR-2.9 crate
48. `integration-scripting-ecs` — docs(progress): code complete
49. `integration-scripting-ui` — feat: harness for scripting↔UI contracts
50. `integration-shared-conventions` — chore(plans): refresh event log
51. `integration-shared-messaging-capacities` — Implement shared capacity table
52. `integration-timelines-audio` — feat: add timelines_audio_integration crate
53. `integration-timelines-camera` — docs(progress): code complete
54. `integration-timelines-scripting` — docs: link draft PR 114
55. `integration-ui-physics` — docs: sync progress

### Networking (3)

| Branch | Commits |
|--------|--------:|
| `plan/networking-network-infrastructure` | 2 |
| `plan/networking-network-services` | 3 |
| `plan/networking-network-transport` | 3 |

Last commits:

1. `networking-network-infrastructure` — green: TC-8.7.1.3, 8.7.2–8.7.4 MMO net
2. `networking-network-services` — feat(network-services): session, replay
3. `networking-network-transport` — review: address PR 51 transport findings

### Physics (1)

| Branch | Commits |
|--------|--------:|
| `plan/physics-advanced` | 1 |

Last commits:

1. `physics-advanced` — feat(physics): add harmonius_physics advanced TC suite

### Platform (6)

| Branch | Commits |
|--------|--------:|
| `plan/platform-console-integration` | 3 |
| `plan/platform-crash-reporting` | 3 |
| `plan/platform-platform-services` | 3 |
| `plan/platform-telemetry` | 3 |
| `plan/platform-threading` | 3 |
| `plan/platform-windowing` | 2 |

Last commits:

1. `platform-console-integration` — review: tidy progress event log
2. `platform-crash-reporting` — review: address crash reporting PR findings
3. `platform-platform-services` — green: platform services stubs and TC tests
4. `platform-telemetry` — review: platform-telemetry PR findings
5. `platform-threading` — review: parallel task graph on thread pool
6. `platform-windowing` — green: Window stub, surface traits, TC-14.1.1.I1

### Rendering (9)

| Branch | Commits |
|--------|--------:|
| `plan/rendering-2d` | 3 |
| `plan/rendering-camera-rendering` | 4 |
| `plan/rendering-meshlets` | 3 |
| `plan/rendering-pipeline-state-cache` | 3 |
| `plan/rendering-render-effects` | 2 |
| `plan/rendering-render-pipeline` | 3 |
| `plan/rendering-render-styles` | 4 |
| `plan/rendering-rendering-core` | 3 |
| `plan/rendering-shader-variants` | 4 |

Last commits:

1. `rendering-2d` — docs(progress): pr-review complete for PR #66
2. `rendering-camera-rendering` — progress: log pr-reviewer follow-up for PR 38
3. `rendering-meshlets` — review: meshlet vertex indices and GPU layout
4. `rendering-pipeline-state-cache` — review: PSO cache API paths and revive
5. `rendering-render-effects` — docs(progress): mark code complete
6. `rendering-render-pipeline` — review: address PR-85 supervisor findings
7. `rendering-render-styles` — docs: note 61 render_styles unit tests
8. `rendering-rendering-core` — green: TC-2.3.1.1, TC-2.10.5.1, cone/barrier
9. `rendering-shader-variants` — review: submit shader variants PR progress

### Simulation (5)

| Branch | Commits |
|--------|--------:|
| `plan/simulation-event-logs` | 3 |
| `plan/simulation-game-loop-phases` | 3 |
| `plan/simulation-grids-volumes` | 1 |
| `plan/simulation-spatial-awareness` | 4 |
| `plan/simulation-timelines` | 2 |

Last commits:

1. `simulation-event-logs` — review: threshold rkyv and PR review hygiene
2. `simulation-game-loop-phases` — review: supervisor findings for phases
3. `simulation-grids-volumes` — feat(sim): add harmonius_sim grids and volumes
4. `simulation-spatial-awareness` — green: spatial awareness core tests
5. `simulation-timelines` — review: address PR consolidated findings

### Tools (11)

| Branch | Commits |
|--------|--------:|
| `plan/tools-build-deploy` | 3 |
| `plan/tools-content-services` | 5 |
| `plan/tools-editor-core` | 4 |
| `plan/tools-level-world` | 7 |
| `plan/tools-plugin-marketplace` | 3 |
| `plan/tools-profiler` | 3 |
| `plan/tools-scene-versioning` | 2 |
| `plan/tools-selection-model` | 3 |
| `plan/tools-team-tools` | 5 |
| `plan/tools-undo-redo` | 4 |
| `plan/tools-visual-editors` | 1 |

Last commits:

1. `tools-build-deploy` — feat(build-deploy): cover plan unit TCs
2. `tools-content-services` — feat(content-services): blake3 audit TCs
3. `tools-editor-core` — feat(editor-core): add harmonius_editor_core crate
4. `tools-level-world` — green: TC-15.2.3.1 TC-15.2.3.2 (property overrides)
5. `tools-plugin-marketplace` — review: trust key binding and install signatures
6. `tools-profiler` — green: PLAN-tools-profiler TC-15.5 unit suite
7. `tools-scene-versioning` — docs(progress): mark code complete
8. `tools-selection-model` — review: address PR 49 selection_model findings
9. `tools-team-tools` — docs: progress code_complete
10. `tools-undo-redo` — green: TC-15.1.3.5 session persist and spill undo
11. `tools-visual-editors` — feat(tools): add visual_editors crate for editor TCs

### UI (1)

| Branch | Commits |
|--------|--------:|
| `plan/ui-ui-framework` | 3 |

Last commits:

1. `ui-ui-framework` — green: TC-10.1.1.2 TC-10.1.1.3 TC-10.1.1.4 TC-10.1.3.1

### VFX (2)

| Branch | Commits |
|--------|--------:|
| `plan/vfx-effects` | 4 |
| `plan/vfx-particles` | 3 |

Last commits:

1. `vfx-effects` — green: PLAN-vfx-effects unit TC coverage
2. `vfx-particles` — feat(vfx): expand particles CPU helpers and TC tests
