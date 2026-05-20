# AI Non-Functional User Stories

Non-functional stories for the AI domain (X = 7). Performance, scalability, memory, and
quality-of-result targets that drive the matching `R-7.NFR.*` requirements in
[`requirements/ai/non-functional.md`](../../requirements/ai/non-functional.md).

Folder rules: stories use the `As a {persona} (P-N), I want {action}, so that {benefit}` form
with no acceptance criteria, no feature links, and no requirement links inline. Persona
definitions live in [`personas.md`](../personas.md).

## Frame Budget

| ID           | Persona              |
|--------------|----------------------|
| US-7.NFR.1.1 | engine dev (P-26)    |
| US-7.NFR.1.2 | engine dev (P-26)    |
| US-7.NFR.1.3 | server admin (P-22)  |
| US-7.NFR.1.4 | engine tester (P-27) |

1. **US-7.NFR.1.1** — As an engine dev (P-26), I want all AI processing to fit within the 1 ms
   per-frame budget, so that AI never causes frame spikes.
2. **US-7.NFR.1.2** — As an engine dev (P-26), I want the AI LOD system to demote agents when
   the budget is exceeded, so that frame stability is preserved automatically.
3. **US-7.NFR.1.3** — As a server admin (P-22), I want configurable AI frame budgets per
   platform, so that mobile and desktop can run with appropriate cost limits.
4. **US-7.NFR.1.4** — As an engine tester (P-27), I want to verify 5 000 agents fit within
   1 ms at 60 fps, so that the budget enforcement is regression-tested.

## Pathfinding Throughput

| ID           | Persona              |
|--------------|----------------------|
| US-7.NFR.2.1 | engine dev (P-26)    |
| US-7.NFR.2.2 | server admin (P-22)  |
| US-7.NFR.2.3 | engine tester (P-27) |

1. **US-7.NFR.2.1** — As an engine dev (P-26), I want at least 500 A* queries per tick
   sustained, so that mass-NPC scenarios are practical.
2. **US-7.NFR.2.2** — As a server admin (P-22), I want 95th-percentile per-query latency below
   0.1 ms, so that pathfinding never spikes individual frames.
3. **US-7.NFR.2.3** — As an engine tester (P-27), I want to verify 500 queries per tick at p95
   under 0.1 ms on a 50-tile NavMesh, so that pathfinding throughput is regression-tested.

## Perception Throughput

| ID           | Persona              |
|--------------|----------------------|
| US-7.NFR.3.1 | engine dev (P-26)    |
| US-7.NFR.3.2 | engine dev (P-26)    |
| US-7.NFR.3.3 | engine tester (P-27) |

1. **US-7.NFR.3.1** — As an engine dev (P-26), I want perception for 1 000 high-LOD agents per
   tick, so that sight and hearing scale to MMO densities.
2. **US-7.NFR.3.2** — As an engine dev (P-26), I want mid-LOD perception deferred to
   alternating ticks, so that perception cost spreads across frames.
3. **US-7.NFR.3.3** — As an engine tester (P-27), I want to verify 1 000 high-LOD evaluations
   complete inside the perception budget, so that perception throughput is regression-tested.

## Behavior Tree Throughput

| ID           | Persona              |
|--------------|----------------------|
| US-7.NFR.4.1 | engine dev (P-26)    |
| US-7.NFR.4.2 | engine tester (P-27) |

1. **US-7.NFR.4.1** — As an engine dev (P-26), I want at least 2 000 BT ticks per frame for
   20-node trees, so that BT processing scales to MMO agent counts.
2. **US-7.NFR.4.2** — As an engine tester (P-27), I want to verify 2 000 tree ticks complete
   inside 0.4 ms, so that BT performance is regression-tested.

## Agent Concurrency

| ID           | Persona              |
|--------------|----------------------|
| US-7.NFR.5.1 | server admin (P-22)  |
| US-7.NFR.5.2 | engine dev (P-26)    |
| US-7.NFR.5.3 | engine tester (P-27) |

1. **US-7.NFR.5.1** — As a server admin (P-22), I want at least 50 000 concurrent AI agents per
   server process, so that MMO cities can host dense populations.
2. **US-7.NFR.5.2** — As an engine dev (P-26), I want tiered LOD distribution (500 high,
   2 000 mid, rest low), so that full AI runs only on gameplay-critical agents.
3. **US-7.NFR.5.3** — As an engine tester (P-27), I want to verify 50 000 agents maintain the
   target tick rate, so that agent-count scalability is regression-tested.

## Crowd Sampling

| ID           | Persona              |
|--------------|----------------------|
| US-7.NFR.6.1 | engine dev (P-26)    |
| US-7.NFR.6.2 | engine tester (P-27) |

1. **US-7.NFR.6.1** — As an engine dev (P-26), I want O(1) per-agent flow-field sampling, so
   that crowd navigation scales linearly with agent count.
2. **US-7.NFR.6.2** — As an engine tester (P-27), I want to verify per-agent cost stays
   constant at 1 K, 5 K, and 10 K agents, so that O(1) scaling is regression-tested.

## NavMesh Memory

| ID           | Persona              |
|--------------|----------------------|
| US-7.NFR.7.1 | server admin (P-22)  |
| US-7.NFR.7.2 | engine dev (P-26)    |
| US-7.NFR.7.3 | engine tester (P-27) |

1. **US-7.NFR.7.1** — As a server admin (P-22), I want NavMesh memory bounded to a configurable
   budget (default 256 MB), so that server memory stays predictable.
2. **US-7.NFR.7.2** — As an engine dev (P-26), I want distant tiles evicted when the budget is
   exceeded, so that memory stays within bounds automatically.
3. **US-7.NFR.7.3** — As an engine tester (P-27), I want to verify tile eviction triggers at
   the 256 MB boundary, so that the memory budget is regression-tested.

## Per-Agent Memory

| ID           | Persona              |
|--------------|----------------------|
| US-7.NFR.8.1 | engine dev (P-26)    |
| US-7.NFR.8.2 | engine dev (P-26)    |
| US-7.NFR.8.3 | engine tester (P-27) |

1. **US-7.NFR.8.1** — As an engine dev (P-26), I want high-LOD agents to use under 4 KB of
   memory each, so that per-agent cost is bounded.
2. **US-7.NFR.8.2** — As an engine dev (P-26), I want low-LOD crowd entities to use under 64
   bytes each, so that 50 000 entities fit in a reasonable footprint.
3. **US-7.NFR.8.3** — As an engine tester (P-27), I want to verify measured per-agent memory
   matches the budget targets, so that overhead is regression-tested.

## Path Quality

| ID           | Persona              |
|--------------|----------------------|
| US-7.NFR.9.1 | engine dev (P-26)    |
| US-7.NFR.9.2 | engine dev (P-26)    |
| US-7.NFR.9.3 | engine dev (P-26)    |
| US-7.NFR.9.4 | engine tester (P-27) |

1. **US-7.NFR.9.1** — As an engine dev (P-26), I want every returned path to be valid (no
   segment exits the NavMesh), so that agents never move through walls or terrain.
2. **US-7.NFR.9.2** — As an engine dev (P-26), I want paths optimal within 5% of true shortest,
   so that navigation quality is consistently high.
3. **US-7.NFR.9.3** — As an engine dev (P-26), I want path smoothing to reduce waypoints by at
   least 30%, so that smoothed paths look natural.
4. **US-7.NFR.9.4** — As an engine tester (P-27), I want to verify 1 000 random paths are all
   valid and within 5% of optimal, so that path quality is regression-tested.

## Reaction Latency

| ID            | Persona              |
|---------------|----------------------|
| US-7.NFR.10.1 | engine dev (P-26)    |
| US-7.NFR.10.2 | gamer (P-23)         |
| US-7.NFR.10.3 | engine tester (P-27) |

1. **US-7.NFR.10.1** — As an engine dev (P-26), I want high-LOD agents to react to high-priority
   events within two ticks, so that AI response time meets quality expectations.
2. **US-7.NFR.10.2** — As a gamer (P-23), I want enemies to react immediately when I hit them,
   so that combat AI feels responsive.
3. **US-7.NFR.10.3** — As an engine tester (P-27), I want to verify conditional aborts fire
   within one tick of the triggering condition, so that abort response time is regression-tested.
