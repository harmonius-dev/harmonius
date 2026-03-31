# AI Non-Functional User Stories

## US-7.NFR.1

| ID           | Persona              | Features | Requirements |
|--------------|----------------------|----------|--------------|
| US-7.NFR.1.1 | engine dev (P-26)    |          | R-7.NFR.1    |
| US-7.NFR.1.2 | engine dev (P-26)    |          | R-7.NFR.1    |
| US-7.NFR.1.3 | server admin (P-22)  |          | R-7.NFR.1    |
| US-7.NFR.1.4 | engine tester (P-27) |          | R-7.NFR.1    |

1. **US-7.NFR.1.1** — I want all AI processing within the 1ms frame budget
   - **Acceptance:** AI does not cause frame-time spikes
2. **US-7.NFR.1.2** — I want the LOD system to demote agents when budget is exceeded
   - **Acceptance:** frame rate stability is maintained automatically
3. **US-7.NFR.1.3** — I want configurable AI frame budgets per platform
   - **Acceptance:** mobile and desktop have appropriate AI cost limits
4. **US-7.NFR.1.4** — I want to verify 5,000 agents stay within 1ms at 60fps
   - **Acceptance:** frame budget enforcement is regression-tested

## US-7.NFR.2

| ID           | Persona              | Features | Requirements |
|--------------|----------------------|----------|--------------|
| US-7.NFR.2.1 | engine dev (P-26)    |          | R-7.NFR.2    |
| US-7.NFR.2.2 | server admin (P-22)  |          | R-7.NFR.2    |
| US-7.NFR.2.3 | engine tester (P-27) |          | R-7.NFR.2    |

1. **US-7.NFR.2.1** — I want at least 500 A* queries per tick sustained
   - **Acceptance:** MMO servers handle mass NPC pathfinding
2. **US-7.NFR.2.2** — I want 95th percentile per-query latency below 0.1ms
   - **Acceptance:** pathfinding does not spike individual frames
3. **US-7.NFR.2.3** — I want to verify 500 queries/tick on a 50-tile NavMesh with p95 under 0.1ms
   - **Acceptance:** pathfinding throughput is regression-tested

## US-7.NFR.3

| ID           | Persona              | Features | Requirements |
|--------------|----------------------|----------|--------------|
| US-7.NFR.3.1 | engine dev (P-26)    |          | R-7.NFR.3    |
| US-7.NFR.3.2 | engine dev (P-26)    |          | R-7.NFR.3    |
| US-7.NFR.3.3 | engine tester (P-27) |          | R-7.NFR.3    |

1. **US-7.NFR.3.1** — I want perception for 1,000 high-LOD agents per tick
   - **Acceptance:** sight and hearing evaluation scales to MMO density
2. **US-7.NFR.3.2** — I want mid-LOD perception deferred to alternating ticks
   - **Acceptance:** perception cost is distributed across frames
3. **US-7.NFR.3.3** — I want to verify 1,000 high-LOD evaluations complete within perception budget
   - **Acceptance:** perception throughput is regression-tested

## US-7.NFR.4

| ID           | Persona              | Features | Requirements |
|--------------|----------------------|----------|--------------|
| US-7.NFR.4.1 | engine dev (P-26)    |          | R-7.NFR.4    |
| US-7.NFR.4.2 | engine tester (P-27) |          | R-7.NFR.4    |

1. **US-7.NFR.4.1** — I want at least 2,000 BT ticks per frame for 20-node trees
   - **Acceptance:** behavior tree processing scales to MMO agent counts
2. **US-7.NFR.4.2** — I want to verify 2,000 tree ticks complete within 0.4ms
   - **Acceptance:** BT performance budget is regression-tested

## US-7.NFR.5

| ID           | Persona              | Features | Requirements |
|--------------|----------------------|----------|--------------|
| US-7.NFR.5.1 | server admin (P-22)  |          | R-7.NFR.5    |
| US-7.NFR.5.2 | engine dev (P-26)    |          | R-7.NFR.5    |
| US-7.NFR.5.3 | engine tester (P-27) |          | R-7.NFR.5    |

1. **US-7.NFR.5.1** — I want at least 50,000 concurrent AI agents per server process
   - **Acceptance:** MMO cities have dense populations
2. **US-7.NFR.5.2** — I want tiered distribution (500 high, 2000 mid, rest low)
   - **Acceptance:** full AI only runs on gameplay-critical agents
3. **US-7.NFR.5.3** — I want to verify 50,000 agents maintain target tick rate
   - **Acceptance:** agent count scalability is regression-tested

## US-7.NFR.6

| ID           | Persona              | Features | Requirements |
|--------------|----------------------|----------|--------------|
| US-7.NFR.6.1 | engine dev (P-26)    |          | R-7.NFR.6    |
| US-7.NFR.6.2 | engine tester (P-27) |          | R-7.NFR.6    |

1. **US-7.NFR.6.1** — I want O(1) per-agent flow field sampling
   - **Acceptance:** crowd navigation scales linearly with agent count
2. **US-7.NFR.6.2** — I want to verify per-agent cost is constant at 1K, 5K, and 10K agents
   - **Acceptance:** O(1) scaling is regression-tested

## US-7.NFR.7

| ID           | Persona             | Features | Requirements |
|--------------|---------------------|----------|--------------|
| US-7.NFR.7.1 | server admin (P-22) |          | R-7.NFR.7    |
| US-7.NFR.7.2 | engine dev (P-26)   |          | R-7.NFR.7    |
| US-7.NFR.7.3 | unknown             |          | R-7.NFR.7    |

1. **US-7.NFR.7.1** — I want NavMesh memory bounded to a configurable budget (default 256 MB)
   - **Acceptance:** server memory usage is predictable
2. **US-7.NFR.7.2** — I want distant tiles evicted when the budget is exceeded
   - **Acceptance:** memory stays within bounds automatically
3. **US-7.NFR.7.3** — As an engine tester (P-27), I want to verify tile eviction triggers at the 256
   MB boundary so that memory budget enforcement is regression-tested.

## US-7.NFR.8

| ID           | Persona           | Features | Requirements |
|--------------|-------------------|----------|--------------|
| US-7.NFR.8.1 | engine dev (P-26) |          | R-7.NFR.8    |
| US-7.NFR.8.2 | engine dev (P-26) |          | R-7.NFR.8    |
| US-7.NFR.8.3 | unknown           |          | R-7.NFR.8    |

1. **US-7.NFR.8.1** — I want high-LOD agents under 4 KB memory overhead
   - **Acceptance:** per-agent cost is bounded
2. **US-7.NFR.8.2** — I want low-LOD crowd entities under 64 bytes overhead
   - **Acceptance:** 50,000 entities fit within a reasonable memory footprint
3. **US-7.NFR.8.3** — As an engine tester (P-27), I want to verify measured per-agent memory matches
   budget targets so that memory overhead is regression-tested.

## US-7.NFR.9

| ID           | Persona              | Features | Requirements |
|--------------|----------------------|----------|--------------|
| US-7.NFR.9.1 | engine dev (P-26)    |          | R-7.NFR.9    |
| US-7.NFR.9.2 | engine dev (P-26)    |          | R-7.NFR.9    |
| US-7.NFR.9.3 | engine dev (P-26)    |          | R-7.NFR.9    |
| US-7.NFR.9.4 | engine tester (P-27) |          | R-7.NFR.9    |

1. **US-7.NFR.9.1** — I want all paths valid (no segment exits NavMesh)
   - **Acceptance:** agents never move through walls or terrain
2. **US-7.NFR.9.2** — I want paths optimal within 5% of true shortest
   - **Acceptance:** navigation quality is consistently high
3. **US-7.NFR.9.3** — I want path smoothing to reduce waypoints by at least 30%
   - **Acceptance:** smoothed paths are visibly more natural
4. **US-7.NFR.9.4** — I want to verify 1,000 random paths are all valid and within 5% of optimal
   - **Acceptance:** path quality is regression-tested

## US-7.NFR.10

| ID            | Persona              | Features | Requirements |
|---------------|----------------------|----------|--------------|
| US-7.NFR.10.1 | engine dev (P-26)    |          | R-7.NFR.10   |
| US-7.NFR.10.2 | player (P-23)        |          | R-7.NFR.10   |
| US-7.NFR.10.3 | engine tester (P-27) |          | R-7.NFR.10   |

1. **US-7.NFR.10.1** — I want high-LOD agents to react to high-priority events within 2 ticks
   - **Acceptance:** AI response time meets quality expectations
2. **US-7.NFR.10.2** — I want enemies to react immediately when I hit them
   - **Acceptance:** combat AI feels responsive
3. **US-7.NFR.10.3** — I want to verify conditional aborts fire within 1 tick of the triggering
   condition
   - **Acceptance:** abort response time is regression-tested
