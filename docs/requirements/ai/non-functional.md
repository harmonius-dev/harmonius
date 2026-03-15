# R-7.NFR -- AI Non-Functional User Stories

## US-7.NFR.1 AI Frame Budget

### US-7.NFR.1.1
As an **engine dev (P-26)**, I want all AI processing within the 1ms frame budget so that AI does
not cause frame-time spikes.

### US-7.NFR.1.2
As an **engine dev (P-26)**, I want the LOD system to demote agents when budget is exceeded so that
frame rate stability is maintained automatically.

### US-7.NFR.1.3
As a **server admin (P-22)**, I want configurable AI frame budgets per platform so that mobile and
desktop have appropriate AI cost limits.

### US-7.NFR.1.4
As an **engine tester (P-27)**, I want to verify 5,000 agents stay within 1ms at 60fps so that frame
budget enforcement is regression-tested.

---

## US-7.NFR.2 Pathfinding Query Throughput

### US-7.NFR.2.1
As an **engine dev (P-26)**, I want at least 500 A* queries per tick sustained so that MMO servers
handle mass NPC pathfinding.

### US-7.NFR.2.2
As a **server admin (P-22)**, I want 95th percentile per-query latency below 0.1ms so that
pathfinding does not spike individual frames.

### US-7.NFR.2.3
As an **engine tester (P-27)**, I want to verify 500 queries/tick on a 50-tile NavMesh with p95
under 0.1ms so that pathfinding throughput is regression-tested.

---

## US-7.NFR.3 Perception Evaluation Throughput

### US-7.NFR.3.1
As an **engine dev (P-26)**, I want perception for 1,000 high-LOD agents per tick so that sight and
hearing evaluation scales to MMO density.

### US-7.NFR.3.2
As an **engine dev (P-26)**, I want mid-LOD perception deferred to alternating ticks so that
perception cost is distributed across frames.

### US-7.NFR.3.3
As an **engine tester (P-27)**, I want to verify 1,000 high-LOD evaluations complete within
perception budget so that perception throughput is regression-tested.

---

## US-7.NFR.4 Behavior Tree Tick Throughput

### US-7.NFR.4.1
As an **engine dev (P-26)**, I want at least 2,000 BT ticks per frame for 20-node trees so that
behavior tree processing scales to MMO agent counts.

### US-7.NFR.4.2
As an **engine tester (P-27)**, I want to verify 2,000 tree ticks complete within 0.4ms so that BT
performance budget is regression-tested.

---

## US-7.NFR.5 Maximum Concurrent AI Agents

### US-7.NFR.5.1
As a **server admin (P-22)**, I want at least 50,000 concurrent AI agents per server process so that
MMO cities have dense populations.

### US-7.NFR.5.2
As an **engine dev (P-26)**, I want tiered distribution (500 high, 2000 mid, rest low) so that full
AI only runs on gameplay-critical agents.

### US-7.NFR.5.3
As an **engine tester (P-27)**, I want to verify 50,000 agents maintain target tick rate so that
agent count scalability is regression-tested.

---

## US-7.NFR.6 Flow Field Scalability

### US-7.NFR.6.1
As an **engine dev (P-26)**, I want O(1) per-agent flow field sampling so that crowd navigation
scales linearly with agent count.

### US-7.NFR.6.2
As an **engine tester (P-27)**, I want to verify per-agent cost is constant at 1K, 5K, and 10K
agents so that O(1) scaling is regression-tested.

---

## US-7.NFR.7 NavMesh Memory Budget

### US-7.NFR.7.1
As a **server admin (P-22)**, I want NavMesh memory bounded to a configurable budget (default 256
MB) so that server memory usage is predictable.

### US-7.NFR.7.2
As an **engine dev (P-26)**, I want distant tiles evicted when the budget is exceeded so that memory
stays within bounds automatically.

### US-7.NFR.7.3
As an **engine tester (P-27)**, I want to verify tile eviction triggers at the 256 MB boundary so
that memory budget enforcement is regression-tested.

---

## US-7.NFR.8 Per-Agent AI Memory Overhead

### US-7.NFR.8.1
As an **engine dev (P-26)**, I want high-LOD agents under 4 KB memory overhead so that per-agent
cost is bounded.

### US-7.NFR.8.2
As an **engine dev (P-26)**, I want low-LOD crowd entities under 64 bytes overhead so that 50,000
entities fit within a reasonable memory footprint.

### US-7.NFR.8.3
As an **engine tester (P-27)**, I want to verify measured per-agent memory matches budget targets so
that memory overhead is regression-tested.

---

## US-7.NFR.9 Navigation Path Quality

### US-7.NFR.9.1
As an **engine dev (P-26)**, I want all paths valid (no segment exits NavMesh) so that agents never
move through walls or terrain.

### US-7.NFR.9.2
As an **engine dev (P-26)**, I want paths optimal within 5% of true shortest so that navigation
quality is consistently high.

### US-7.NFR.9.3
As an **engine dev (P-26)**, I want path smoothing to reduce waypoints by at least 30% so that
smoothed paths are visibly more natural.

### US-7.NFR.9.4
As an **engine tester (P-27)**, I want to verify 1,000 random paths are all valid and within 5% of
optimal so that path quality is regression-tested.

---

## US-7.NFR.10 AI Decision Responsiveness

### US-7.NFR.10.1
As an **engine dev (P-26)**, I want high-LOD agents to react to high-priority events within 2 ticks
so that AI response time meets quality expectations.

### US-7.NFR.10.2
As a **player (P-23)**, I want enemies to react immediately when I hit them so that combat AI feels
responsive.

### US-7.NFR.10.3
As an **engine tester (P-27)**, I want to verify conditional aborts fire within 1 tick of the
triggering condition so that abort response time is regression-tested.
