# R-7.NFR -- AI Non-Functional Requirements

Non-functional requirements for the AI domain covering performance budgets, scalability limits,
memory constraints, and quality thresholds. These complement the functional requirements in
R-7.1 through R-7.8 and trace to the cross-cutting performance budget in R-X.1.1.

---

## Performance

### R-7.NFR.1 AI Frame Budget

The engine **SHALL** complete all AI system processing (behavior trees, utility scoring, GOAP
planning, perception evaluation, steering, crowd simulation) within the AI frame budget defined
by R-X.1.1 (default 1 ms at 60 fps). When the budget is exceeded, the AI LOD system (R-7.7.5)
**SHALL** demote agents to lower tiers until processing fits within the budget.

- **Derived from:** F-7.7.5 (AI LOD), R-X.1.1 (Frame Time Budget Allocation)
- **Rationale:** AI processing that exceeds its frame budget causes frame-time spikes visible
  to the player and steals budget from rendering and physics.
- **Verification:** Stress test: spawn 5,000 AI agents in a dense area. Verify total AI
  processing time does not exceed 1 ms at 60 fps. Verify the LOD system demotes distant
  agents to flow-field-only movement to stay within budget.

### R-7.NFR.2 Pathfinding Query Throughput

The engine **SHALL** sustain at least 500 A* pathfinding queries per tick on a NavMesh with
10,000+ polygons per tile, with 95th percentile per-query latency below 0.1 ms, when queries
are batched across frames per R-7.1.3.

- **Derived from:** F-7.1.3 (A* Pathfinding), F-7.1.14 (HPA*)
- **Rationale:** MMO servers with thousands of NPCs require high pathfinding throughput
  without per-query latency spikes that would starve other AI systems.
- **Verification:** Benchmark: issue 500 queries per tick on a 50-tile NavMesh. Measure
  throughput and verify 95th percentile query latency is below 0.1 ms. Verify total
  pathfinding time stays within the AI frame budget.

### R-7.NFR.3 Perception Evaluation Throughput

The engine **SHALL** evaluate perception (sight, hearing, damage, custom senses) for at least
1,000 high-LOD agents per tick within the perception budget allocated by the AI LOD scheduler
(R-7.7.5). Perception for mid-LOD agents **SHALL** be deferred to alternating ticks.

- **Derived from:** F-7.6.1 -- F-7.6.7 (Perception), F-7.7.5 (AI LOD)
- **Rationale:** Perception is one of the most expensive per-agent AI operations. Budget
  enforcement prevents perception from dominating the AI frame budget.
- **Verification:** Benchmark: run 1,000 high-LOD agents with sight and hearing senses active.
  Verify all evaluations complete within the allocated perception budget. Verify mid-LOD agents
  are evaluated every 2nd tick.

### R-7.NFR.4 Behavior Tree Tick Throughput

The engine **SHALL** tick at least 2,000 behavior trees per frame when trees have an average
depth of 8 nodes and 20 total nodes, completing all ticks within the behavior tree portion of
the AI frame budget.

- **Derived from:** F-7.3.1 -- F-7.3.7 (Behavior Trees), R-7.NFR.1
- **Rationale:** Behavior trees are ticked every frame for high-LOD agents. Per-tree cost must
  remain bounded to support MMO agent counts.
- **Verification:** Benchmark: tick 2,000 behavior trees of 20 nodes each. Measure total time
  and verify it does not exceed 0.4 ms (40% of the default 1 ms AI budget).

---

## Scalability

### R-7.NFR.5 Maximum Concurrent AI Agents

The engine **SHALL** support at least 50,000 concurrent AI agents per server process when using
the AI LOD tiering system (R-7.7.5), with at most 500 high-LOD agents, 2,000 mid-LOD agents,
and the remainder as low-LOD (flow-field-only) crowd entities.

- **Derived from:** F-7.7.4 (Mass Entity Simulation), F-7.7.5 (AI LOD)
- **Rationale:** MMO cities and open-world zones require tens of thousands of ambient NPCs for
  world density while only nearby agents need full AI evaluation.
- **Verification:** Load test: spawn 50,000 agents with the specified LOD distribution. Verify
  the server maintains its target tick rate. Verify all agents exhibit correct behavior for
  their LOD tier.

### R-7.NFR.6 Flow Field Scalability

The engine **SHALL** support at least 10,000 crowd agents sampling a single flow field without
per-agent cost exceeding O(1) (a constant-time lookup per agent per tick), independent of total
agent count.

- **Derived from:** F-7.7.2 (Flow Field Navigation), F-7.7.4 (Mass Entity Simulation)
- **Rationale:** Flow fields exist to amortize pathfinding cost. Per-agent cost must remain
  constant to achieve linear scaling.
- **Verification:** Benchmark: measure per-agent flow field sampling cost at 1,000, 5,000, and
  10,000 agents. Verify cost per agent remains constant (within 5% variance).

---

## Memory

### R-7.NFR.7 NavMesh Memory Budget

The engine **SHALL** keep total NavMesh memory (all loaded layers and tiles) within a
configurable budget (default 256 MB per server process). The streaming system (R-7.1.2)
**SHALL** evict distant tiles when the budget is exceeded, per the system RAM budget policy
in R-X.1.3.

- **Derived from:** F-7.1.2 (NavMesh Tiling & Streaming), F-7.1.12 (Multi-Size Agent NavMeshes)
- **Rationale:** Multi-layer NavMeshes for large open worlds can consume significant memory.
  Budget enforcement prevents OOM conditions.
- **Verification:** Load test: stream NavMesh tiles until the 256 MB budget is reached. Verify
  distant tiles are evicted. Verify agents in evicted tile regions receive fallback movement.

### R-7.NFR.8 Per-Agent AI Memory Overhead

The engine **SHALL** keep per-agent AI memory overhead (blackboard, perception state, steering
state, behavior tree instance) below 4 KB for high-LOD agents and below 64 bytes for low-LOD
crowd entities.

- **Derived from:** F-7.7.4 (Mass Entity Simulation), F-7.3.4 (Blackboard System)
- **Rationale:** With 50,000 agents, per-agent memory overhead directly determines total AI
  memory consumption. Low-LOD entities must be lightweight.
- **Verification:** Measure per-agent memory for high-LOD and low-LOD agents. Verify high-LOD
  is below 4 KB and low-LOD is below 64 bytes.

---

## Quality

### R-7.NFR.9 Navigation Path Quality

All generated paths **SHALL** be valid (no segment exits the NavMesh), optimal within 5% of
the true shortest weighted path, and free of U-turns or backtracking artifacts. Path smoothing
(R-7.1.5) **SHALL** reduce waypoint count by at least 30% compared to unsmoothed paths on
corridors with redundant turns.

- **Derived from:** F-7.1.3 (A* Pathfinding), F-7.1.4 (Funnel Algorithm), F-7.1.5 (Path
  Smoothing)
- **Rationale:** Suboptimal or invalid paths produce visibly wrong NPC movement that breaks
  player immersion.
- **Verification:** Generate 1,000 random paths on a test NavMesh. Verify all are valid. Compare
  path costs to brute-force shortest paths and verify deviation is below 5%. Apply smoothing
  and verify waypoint reduction exceeds 30%.

### R-7.NFR.10 AI Decision Responsiveness

High-LOD AI agents **SHALL** react to high-priority events (damage received, target lost,
phase transition) within 2 simulation ticks. Conditional aborts (R-7.3.3) and replanning
triggers (R-7.5.5) **SHALL** fire within 1 tick of the triggering condition becoming true.

- **Derived from:** F-7.3.3 (Conditional Aborts), F-7.5.5 (Replanning Triggers),
  F-7.6.3 (Damage Sense)
- **Rationale:** AI that delays reaction to critical events appears unresponsive and reduces
  combat quality.
- **Verification:** Trigger a damage event on a high-LOD agent mid-patrol. Verify the behavior
  tree aborts and transitions to combat within 2 ticks. Invalidate a GOAP plan precondition
  and verify replanning triggers within 1 tick.
