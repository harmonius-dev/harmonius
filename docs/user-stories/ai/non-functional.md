# AI Non-Functional User Stories

## US-7.NFR.1

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.NFR.1.1 | engine dev (P-26) | I want all AI processing within the 1ms frame budget | AI does not cause frame-time spikes |  | R-7.NFR.1 |
| US-7.NFR.1.2 | engine dev (P-26) | I want the LOD system to demote agents when budget is exceeded | frame rate stability is maintained automatically |  | R-7.NFR.1 |
| US-7.NFR.1.3 | server admin (P-22) | I want configurable AI frame budgets per platform | mobile and desktop have appropriate AI cost limits |  | R-7.NFR.1 |
| US-7.NFR.1.4 | engine tester (P-27) | I want to verify 5,000 agents stay within 1ms at 60fps | frame budget enforcement is regression-tested |  | R-7.NFR.1 |

## US-7.NFR.2

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.NFR.2.1 | engine dev (P-26) | I want at least 500 A* queries per tick sustained | MMO servers handle mass NPC pathfinding |  | R-7.NFR.2 |
| US-7.NFR.2.2 | server admin (P-22) | I want 95th percentile per-query latency below 0.1ms | pathfinding does not spike individual frames |  | R-7.NFR.2 |
| US-7.NFR.2.3 | engine tester (P-27) | I want to verify 500 queries/tick on a 50-tile NavMesh with p95 under 0.1ms | pathfinding throughput is regression-tested |  | R-7.NFR.2 |

## US-7.NFR.3

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.NFR.3.1 | engine dev (P-26) | I want perception for 1,000 high-LOD agents per tick | sight and hearing evaluation scales to MMO density |  | R-7.NFR.3 |
| US-7.NFR.3.2 | engine dev (P-26) | I want mid-LOD perception deferred to alternating ticks | perception cost is distributed across frames |  | R-7.NFR.3 |
| US-7.NFR.3.3 | engine tester (P-27) | I want to verify 1,000 high-LOD evaluations complete within perception budget | perception throughput is regression-tested |  | R-7.NFR.3 |

## US-7.NFR.4

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.NFR.4.1 | engine dev (P-26) | I want at least 2,000 BT ticks per frame for 20-node trees | behavior tree processing scales to MMO agent counts |  | R-7.NFR.4 |
| US-7.NFR.4.2 | engine tester (P-27) | I want to verify 2,000 tree ticks complete within 0.4ms | BT performance budget is regression-tested |  | R-7.NFR.4 |

## US-7.NFR.5

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.NFR.5.1 | server admin (P-22) | I want at least 50,000 concurrent AI agents per server process | MMO cities have dense populations |  | R-7.NFR.5 |
| US-7.NFR.5.2 | engine dev (P-26) | I want tiered distribution (500 high, 2000 mid, rest low) | full AI only runs on gameplay-critical agents |  | R-7.NFR.5 |
| US-7.NFR.5.3 | engine tester (P-27) | I want to verify 50,000 agents maintain target tick rate | agent count scalability is regression-tested |  | R-7.NFR.5 |

## US-7.NFR.6

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.NFR.6.1 | engine dev (P-26) | I want O(1) per-agent flow field sampling | crowd navigation scales linearly with agent count |  | R-7.NFR.6 |
| US-7.NFR.6.2 | engine tester (P-27) | I want to verify per-agent cost is constant at 1K, 5K, and 10K agents | O(1) scaling is regression-tested |  | R-7.NFR.6 |

## US-7.NFR.7

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.NFR.7.1 | server admin (P-22) | I want NavMesh memory bounded to a configurable budget (default 256 MB) | server memory usage is predictable |  | R-7.NFR.7 |
| US-7.NFR.7.2 | engine dev (P-26) | I want distant tiles evicted when the budget is exceeded | memory stays within bounds automatically |  | R-7.NFR.7 |
| US-7.NFR.7.3 | unknown | As an engine tester (P-27), I want to verify tile eviction triggers at the 256 MB boundary so that memory budget enforcement is regression-tested. |  |  | R-7.NFR.7 |

## US-7.NFR.8

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.NFR.8.1 | engine dev (P-26) | I want high-LOD agents under 4 KB memory overhead | per-agent cost is bounded |  | R-7.NFR.8 |
| US-7.NFR.8.2 | engine dev (P-26) | I want low-LOD crowd entities under 64 bytes overhead | 50,000 entities fit within a reasonable memory footprint |  | R-7.NFR.8 |
| US-7.NFR.8.3 | unknown | As an engine tester (P-27), I want to verify measured per-agent memory matches budget targets so that memory overhead is regression-tested. |  |  | R-7.NFR.8 |

## US-7.NFR.9

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.NFR.9.1 | engine dev (P-26) | I want all paths valid (no segment exits NavMesh) | agents never move through walls or terrain |  | R-7.NFR.9 |
| US-7.NFR.9.2 | engine dev (P-26) | I want paths optimal within 5% of true shortest | navigation quality is consistently high |  | R-7.NFR.9 |
| US-7.NFR.9.3 | engine dev (P-26) | I want path smoothing to reduce waypoints by at least 30% | smoothed paths are visibly more natural |  | R-7.NFR.9 |
| US-7.NFR.9.4 | engine tester (P-27) | I want to verify 1,000 random paths are all valid and within 5% of optimal | path quality is regression-tested |  | R-7.NFR.9 |

## US-7.NFR.10

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.NFR.10.1 | engine dev (P-26) | I want high-LOD agents to react to high-priority events within 2 ticks | AI response time meets quality expectations |  | R-7.NFR.10 |
| US-7.NFR.10.2 | player (P-23) | I want enemies to react immediately when I hit them | combat AI feels responsive |  | R-7.NFR.10 |
| US-7.NFR.10.3 | engine tester (P-27) | I want to verify conditional aborts fire within 1 tick of the triggering condition | abort response time is regression-tested |  | R-7.NFR.10 |
