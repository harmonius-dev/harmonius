# R-7.NFR -- AI Non-Functional Requirements

Performance, scalability, memory, and quality-of-result requirements for the AI domain. Driven by
the matching user stories in
[`user-stories/ai/non-functional.md`](../../user-stories/ai/non-functional.md). All numeric targets
reference the desktop reference profile in [`cross-cutting.md`](../cross-cutting.md) unless
otherwise stated.

## Frame Budget

| ID         | Requirement                                                                  |
|------------|------------------------------------------------------------------------------|
| R-7.NFR.1  | AI processing fits within 1 ms per frame                                     |
| R-7.NFR.2  | Sustain ≥ 500 A* queries per tick at p95 latency < 0.1 ms                    |
| R-7.NFR.3  | Process ≥ 1 000 high-LOD perception evaluations per tick                     |
| R-7.NFR.4  | Process ≥ 2 000 BT ticks per frame for 20-node trees                         |
| R-7.NFR.5  | Support ≥ 50 000 concurrent AI agents per server process                     |
| R-7.NFR.6  | Per-agent flow-field sampling is O(1)                                        |
| R-7.NFR.7  | NavMesh memory bounded to a configurable budget (default 256 MB)             |
| R-7.NFR.8  | High-LOD ≤ 4 KB / agent; low-LOD ≤ 64 B / agent                              |
| R-7.NFR.9  | Paths are 100% valid and within 5% of true shortest                          |
| R-7.NFR.10 | High-LOD agents react to high-priority events within 2 ticks                 |

1. **R-7.NFR.1** — The engine **SHALL** complete all AI processing within 1 ms per frame at
   60 fps on the desktop reference profile, automatically demoting agent LOD when the budget
   would otherwise be exceeded.
   - **Rationale:** AI sharing the worker pool with physics, animation, and simulation cannot
     exceed its share of the frame budget without causing visible spikes.
   - **Verification:** Run a 5 000-agent stress scene at 60 fps for 60 s. Assert p99 AI phase
     time stays under 1 ms and frame time stays under 16.67 ms.

2. **R-7.NFR.2** — The engine **SHALL** sustain at least 500 A* pathfinding queries per fixed
   tick with 95th-percentile per-query latency below 0.1 ms on the desktop reference profile.
   - **Rationale:** MMO city densities require mass-NPC pathfinding without per-frame spikes.
   - **Verification:** Issue 500 random A* queries on a 50-tile NavMesh per tick for 600 ticks.
     Verify total throughput and p95 query latency.

3. **R-7.NFR.3** — The engine **SHALL** evaluate perception for at least 1 000 high-LOD agents
   per tick and defer mid-LOD perception to alternating ticks.
   - **Rationale:** Sight, hearing, and proximity tests are O(N) per agent; tiered LOD is the
     only way to scale.
   - **Verification:** Configure 1 000 high-LOD + 2 000 mid-LOD agents. Run for 600 ticks and
     verify mid-LOD perception runs every other tick and high-LOD every tick.

4. **R-7.NFR.4** — The engine **SHALL** complete at least 2 000 behavior-tree ticks for
   20-node trees within 0.4 ms per frame.
   - **Rationale:** BT evaluation is the dominant per-agent cost on hot-LOD bands.
   - **Verification:** Tick 2 000 trees of 20 nodes; assert wall-clock under 0.4 ms p95.

5. **R-7.NFR.5** — The engine **SHALL** support at least 50 000 concurrent AI agents per
   server process distributed as 500 high-LOD + 2 000 mid-LOD + 47 500 low-LOD.
   - **Rationale:** MMO city scenes require this scale; full AI runs only on the
     gameplay-critical band.
   - **Verification:** Spawn 50 000 agents with the LOD distribution above; verify the server
     maintains its target tick rate (20 Hz simulation).

6. **R-7.NFR.6** — The engine **SHALL** provide O(1) per-agent flow-field sampling so that
   crowd navigation cost scales linearly with agent count.
   - **Rationale:** O(N²) crowd algorithms collapse beyond a few thousand agents.
   - **Verification:** Measure per-agent sampling cost at 1 K, 5 K, and 10 K agents; assert
     constant per-agent cost.

7. **R-7.NFR.7** — The engine **SHALL** bound NavMesh memory to a configurable budget (default
   256 MB) and evict the most-distant tiles automatically when the budget is exceeded.
   - **Rationale:** Open-world streaming demands a hard memory ceiling.
   - **Verification:** Set the budget to 256 MB. Stream 1 GB of NavMesh tiles. Verify
     resident size never exceeds 256 MB and eviction is LRU by camera distance.

8. **R-7.NFR.8** — The engine **SHALL** keep high-LOD agent overhead under 4 KB per agent and
   low-LOD crowd entity overhead under 64 bytes per entity.
   - **Rationale:** 50 000 agents × per-agent budget must fit in a reasonable working set.
   - **Verification:** Allocate the maximum-LOD distribution. Measure heap residency.
     Assert each band is within budget.

9. **R-7.NFR.9** — The engine **SHALL** return paths that are 100% valid (every segment lies
   inside the NavMesh) and within 5% of the true shortest path, with smoothing reducing
   waypoints by at least 30%.
   - **Rationale:** Path validity is correctness; quality is what players notice.
   - **Verification:** Generate 1 000 random source/destination pairs. Verify every path is
     valid; mean cost ≤ 1.05 × ground-truth cost; mean smoothed waypoint count ≤ 0.7 × raw.

10. **R-7.NFR.10** — The engine **SHALL** dispatch high-priority event reactions to high-LOD
    agents within two simulation ticks of the event, with conditional aborts firing within
    one tick of the triggering condition becoming true.
    - **Rationale:** Reaction latency is the primary perceived-quality metric for combat AI.
    - **Verification:** Inject a high-priority event in tick T; assert the corresponding
      high-LOD agent action is queued by tick T+2 and conditional aborts fire by T+1.
