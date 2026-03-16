# R-7.NFR -- AI Non-Functional Requirements

## Performance Budgets

### R-7.NFR.1 AI Frame Budget

The engine **SHALL** complete all AI processing (perception, behavior trees, utility evaluation,
GOAP planning, steering, crowd simulation) within a configurable per-frame budget (default 1 ms at
60 fps), with the LOD system automatically demoting agents to lower tiers when the budget is
exceeded.

- **Derived from:** F-7.7.5, F-7.3.1, F-7.6.7
- **Rationale:** AI must not cause frame-time spikes; automatic LOD demotion ensures graceful
  degradation under load rather than dropped frames.
- **Verification:** Simulate 5,000 agents at 60 fps and verify total AI processing stays within 1
  ms. Spawn an additional 2,000 agents and verify the LOD system demotes agents to maintain the
  budget without frame drops. Verify the budget is configurable per platform.

### R-7.NFR.2 Pathfinding Query Throughput

The engine **SHALL** sustain at least 500 A* queries per tick on a desktop-class CPU, with 95th
percentile per-query latency below 0.1 ms.

- **Derived from:**
  [F-7.1.3](../../features/ai/navigation.md)
- **Rationale:** MMO servers with thousands of NPCs require high pathfinding throughput without
  individual queries spiking frame time.
- **Verification:** Issue 500 path queries per tick on a 50-tile NavMesh. Verify all complete within
  the tick. Record per-query latency for 10,000 queries and verify the 95th percentile is below 0.1
  ms.

### R-7.NFR.3 Perception Evaluation Throughput

The engine **SHALL** evaluate perception for at least 1,000 high-LOD agents per tick, with mid-LOD
agents deferred to alternating ticks to distribute cost across frames.

- **Derived from:**
  [F-7.6.7](../../features/ai/perception.md)
- **Rationale:** Perception is one of the most expensive AI operations due to spatial queries and
  raycasts; deferring mid-LOD agents halves the per-tick cost.
- **Verification:** Configure 1,000 high-LOD agents and verify perception evaluation completes
  within the perception budget. Configure 2,000 mid-LOD agents and verify they evaluate on
  alternating ticks (1,000 per tick).

### R-7.NFR.4 Behavior Tree Tick Throughput

The engine **SHALL** process at least 2,000 behavior tree ticks per frame for trees with up to 20
nodes, completing within 0.4 ms.

- **Derived from:**
  [F-7.3.1](../../features/ai/behavior-trees.md)
- **Rationale:** Behavior tree ticking is the most frequent AI operation; throughput must scale to
  MMO agent counts.
- **Verification:** Create 2,000 agents with 20-node behavior trees. Tick all trees in a single
  frame and verify completion within 0.4 ms. Verify per-tree cost does not increase with agent
  count.

## Scalability

### R-7.NFR.5 Maximum Concurrent AI Agents

The engine **SHALL** support at least 50,000 concurrent AI agents per server process, distributed
across LOD tiers (default: 500 high, 2,000 mid, remainder low), while maintaining the target tick
rate.

- **Derived from:** F-7.7.4, F-7.7.5
- **Rationale:** MMO cities require dense populations; tiered distribution ensures full AI runs only
  on gameplay-critical agents.
- **Verification:** Spawn 50,000 agents with the default tier distribution. Verify the server
  maintains its target tick rate. Verify high-LOD agents run full BT + perception. Verify low-LOD
  agents use flow-field-only movement.

### R-7.NFR.6 Flow Field Scalability

The engine **SHALL** achieve O(1) per-agent flow field sampling cost, verified by constant per-agent
cost at 1K, 5K, and 10K agent counts (within 5% variance).

- **Derived from:**
  [F-7.7.2](../../features/ai/crowd-simulation.md)
- **Rationale:** Flow fields are the primary navigation mechanism for crowd entities; per-agent cost
  must not increase with population.
- **Verification:** Benchmark per-agent flow field sampling at 1K, 5K, and 10K agents. Verify
  per-agent cost remains constant within 5% variance across all counts.

## Memory Budgets

### R-7.NFR.7 NavMesh Memory Budget

The engine **SHALL** bound NavMesh memory to a configurable budget (default 256 MB), evicting
distant tiles when the budget is exceeded.

- **Derived from:**
  [F-7.1.2](../../features/ai/navigation.md)
- **Rationale:** Server memory must be predictable and bounded; unbounded NavMesh growth risks OOM
  crashes on large worlds.
- **Verification:** Load NavMesh tiles until memory reaches 256 MB. Verify tile eviction triggers
  and memory stays at or below the budget. Verify evicted tiles can be re-loaded when streaming
  moves to their region.

### R-7.NFR.8 Per-Agent AI Memory Overhead

The engine **SHALL** limit high-LOD agent memory overhead to 4 KB (including blackboard, perception
state, behavior tree instance, and steering data) and low-LOD crowd entity overhead to 64 bytes.

- **Derived from:** F-7.7.4, F-7.3.4, F-7.6.6
- **Rationale:** Per-agent memory scales linearly with agent count; bounded overhead ensures 50,000
  agents fit within server memory.
- **Verification:** Measure total AI memory per high-LOD agent via allocator instrumentation and
  verify it is under 4 KB. Measure per low-LOD crowd entity and verify it is under 64 bytes. Verify
  50,000 agents at the default tier distribution fit within a 512 MB AI memory budget.

## Quality

### R-7.NFR.9 Navigation Path Quality

The engine **SHALL** produce paths where every segment lies on valid NavMesh polygons, paths are
within 5% of the true shortest distance, and path smoothing reduces waypoint count by at least 30%.

- **Derived from:** [F-7.1.4](../../features/ai/navigation.md),
  [F-7.1.5](../../features/ai/navigation.md)
- **Rationale:** Invalid paths cause agents to walk through walls; suboptimal paths waste travel
  time and look unintelligent; smoothing improves visual quality.
- **Verification:** Generate 1,000 random paths across a varied NavMesh. Verify every segment lies
  on valid polygons. Compute true shortest distance and verify paths are within 5%. Apply smoothing
  and verify waypoint count is reduced by at least 30% on average.

### R-7.NFR.10 AI Decision Responsiveness

The engine **SHALL** ensure high-LOD agents react to high-priority events (damage, conditional
aborts) within 2 ticks, with conditional aborts firing within 1 tick of the triggering condition.

- **Derived from:** [F-7.3.3](../../features/ai/behavior-trees.md),
  [F-7.6.3](../../features/ai/perception.md)
- **Rationale:** Unresponsive AI feels broken; players expect enemies to react immediately when hit
  or when critical conditions change.
- **Verification:** Apply damage to a high-LOD agent and verify a behavior response within 2 ticks.
  Trigger a conditional abort condition and verify the abort fires within 1 tick. Verify mid-LOD
  agents respond within their reduced tick interval.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/ai/non-functional.md](../../user-stories/ai/non-functional.md). Requirements in this
document are derived from those user stories.
