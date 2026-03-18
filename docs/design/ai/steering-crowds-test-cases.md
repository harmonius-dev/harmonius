# Steering & Crowd Simulation Test Cases

Companion test cases for [steering-crowds.md](steering-crowds.md).

## Unit Tests

### TC-7.2.3.1 Seek Toward Target

| # | Requirement |
|---|-------------|
| 1 | R-7.2.3     |
| 2 | R-7.2.3     |

1. **#1** — Agent at (0,0,0), target at (10,0,0), max_speed=5
   - **Expected:** Force vector points toward (10,0,0); magnitude > 0
2. **#2** — Agent at (10,0,0), target at (10,0,0)
   - **Expected:** Force vector = (0,0,0) (at target)

### TC-7.2.3.2 Flee Away From Threat

| # | Requirement |
|---|-------------|
| 1 | R-7.2.3     |

1. **#1** — Agent at (0,0,0), threat at (5,0,0)
   - **Expected:** Force vector points away from threat (negative X)

### TC-7.2.3.3 Arrive Deceleration

| # | Requirement |
|---|-------------|
| 1 | R-7.2.3     |
| 2 | R-7.2.3     |

1. **#1** — Agent at (0,0,0), target at (10,0,0), decel_radius=5m; position at (7,0,0)
   - **Expected:** Force magnitude < max_force (within decel zone)
2. **#2** — Agent at (0,0,0), target at (10,0,0), decel_radius=5m; position at (2,0,0)
   - **Expected:** Force magnitude = max_force (outside decel zone)

### TC-7.2.3.4 Arrive Stops at Target

| # | Requirement |
|---|-------------|
| 1 | R-7.2.3     |

1. **#1** — Arrive behavior, simulate 200 ticks at dt=0.016
   - **Expected:** Final position within 0.1m of target

### TC-7.2.3.5 Wander Stays Bounded

| # | Requirement |
|---|-------------|
| 1 | R-7.2.3     |

1. **#1** — Wander for 1000 ticks, initial position=(0,0,0), wander_radius=50m
   - **Expected:** All positions within 50m of origin

### TC-7.2.3.6 Pursuit Intercepts

| # | Requirement |
|---|-------------|
| 1 | R-7.2.3     |

1. **#1** — Pursuer at (0,0,0), target moving at (1,0,0) from (20,0,0)
   - **Expected:** Pursuer reaches target faster than naive seek

### TC-7.2.3.7 Evade Escapes

| # | Requirement |
|---|-------------|
| 1 | R-7.2.3     |

1. **#1** — Evader at (0,0,0), threat at (5,0,0) moving toward evader
   - **Expected:** Distance increases over 100 ticks

### TC-7.2.4.1 Weighted Blend Sum

| # | Requirement |
|---|-------------|
| 1 | R-7.2.4     |
| 2 | R-7.2.4     |

1. **#1** — Force A=(1,0,0) weight=0.6; Force B=(0,1,0) weight=0.4
   - **Expected:** Blended = (0.6, 0.4, 0)
2. **#2** — Force A=(2,0,0) weight=0.5; Force B=(-2,0,0) weight=0.5
   - **Expected:** Blended = (0, 0, 0)

### TC-7.2.4.2 Priority Blend Order

| # | Requirement |
|---|-------------|
| 1 | R-7.2.4     |

1. **#1** — Priority 0: avoidance=(1,0,0); Priority 1: seek=(0,1,0); budget=1.0
   - **Expected:** Avoidance fully satisfied; seek gets remainder

### TC-7.2.4.3 Priority No Zero Force

| # | Requirement |
|---|-------------|
| 1 | R-7.2.4     |

1. **#1** — Two opposing equal-magnitude forces via priority blend
   - **Expected:** Output magnitude > 0 (higher priority dominates)

### TC-7.2.1.1 ORCA No Overlap

| # | Requirement |
|---|-------------|
| 1 | R-7.2.1     |

1. **#1** — 200 agents in 10x10m area, radius=0.3m, 100 ticks
   - **Expected:** Zero pairwise distance < 2 * radius

### TC-7.2.1.2 ORCA Deadlock Free

| # | Requirement |
|---|-------------|
| 1 | R-7.2.1     |

1. **#1** — 10 agents going left, 10 going right, 2m wide passage
   - **Expected:** All 20 agents reach goals within 300 ticks

### TC-7.2.2.1 Obstacle No Clip

| # | Requirement |
|---|-------------|
| 1 | R-7.2.2     |

1. **#1** — Agent with 5 feeler rays, 3 static walls, 500 ticks
   - **Expected:** Agent center never within wall_thickness of any wall

### TC-7.2.5.1 Formation Offsets

| # | Requirement |
|---|-------------|
| 1 | R-7.2.5     |

1. **#1** — V-formation, 5 members, leader at (0,0,0) moving (1,0,0)
   - **Expected:** Each member within 0.5m of configured offset from leader

### TC-7.2.5.2 Formation Narrow Adjust

| # | Requirement |
|---|-------------|
| 1 | R-7.2.5     |

1. **#1** — Line formation, 4m wide, enters 2m corridor
   - **Expected:** Formation compresses; no member clips walls

### TC-7.2.5.3 Formation Reassign

| # | Requirement |
|---|-------------|
| 1 | R-7.2.5     |

1. **#1** — 5-member formation; member 3 removed
   - **Expected:** Remaining 4 reassign to valid slots; no empty interior slots

### TC-7.2.6.1 Group Cohesion

| # | Requirement |
|---|-------------|
| 1 | R-7.2.6     |

1. **#1** — 8-member group, 3 sharp turns over 200 ticks
   - **Expected:** Group radius stays within 1.5x initial radius

### TC-7.2.6.2 Group Velocity Convergence

| # | Requirement |
|---|-------------|
| 1 | R-7.2.6     |

1. **#1** — 8-member group, initial random velocities, shared goal direction
   - **Expected:** All member velocities within 10% of shared velocity after 60 ticks

### TC-7.7.1.1 Flocking Separation

| # | Requirement |
|---|-------------|
| 1 | R-7.7.1     |

1. **#1** — 50-member flock, separation_distance=0.5m, 300 ticks
   - **Expected:** Minimum pairwise distance > 0.5m

### TC-7.7.1.2 Flocking Cohesion

| # | Requirement |
|---|-------------|
| 1 | R-7.7.1     |

1. **#1** — 50-member flock, cohesion_radius=20m, 300 ticks
   - **Expected:** All members within 20m of flock center

### TC-7.7.2.1 Flow Field Correctness

| # | Requirement |
|---|-------------|
| 1 | R-7.7.2     |

1. **#1** — 64x64 flow field, goal at (63,63); sample 100 random cells
   - **Expected:** Following direction from each cell eventually reaches goal

### TC-7.7.2.2 Flow Field Constant Cost

| # | Requirement |
|---|-------------|
| 1 | R-7.7.2     |
| 2 | R-7.7.2     |

1. **#1** — Flow field sample with 100 agents
   - **Expected:** Per-agent sample time O(1) regardless of agent count
2. **#2** — Flow field sample with 10000 agents
   - **Expected:** Per-agent sample time within 2x of 100-agent case

### TC-7.7.3.1 Flow Cache Hit

| # | Requirement |
|---|-------------|
| 1 | R-7.7.3     |

1. **#1** — Request flow field (goal=G, cost_version=1); request same again
   - **Expected:** Second request returns cached field (no recompute)

### TC-7.7.3.2 Flow Cache Invalidation

| # | Requirement |
|---|-------------|
| 1 | R-7.7.3     |

1. **#1** — Cached field (goal=G, cost_version=1); cost_version changes to 2
   - **Expected:** Stale entry evicted; fresh field computed

### TC-7.7.3.3 Flow Cache LRU Eviction

| # | Requirement |
|---|-------------|
| 1 | R-7.7.3     |

1. **#1** — Cache capacity=4; insert 5 distinct flow fields
   - **Expected:** Oldest (least recently used) field evicted

### TC-7.7.6.1 Density Cap Enforced

| # | Requirement |
|---|-------------|
| 1 | R-7.7.6     |

1. **#1** — Cell cap=20; 30 agents in cell
   - **Expected:** After enforcement, cell has <= 20 agents

### TC-7.7.6.2 Density Redirect

| # | Requirement |
|---|-------------|
| 1 | R-7.7.6     |

1. **#1** — Cell cap=20; 25 agents; 5 overflow
   - **Expected:** 5 agents receive redirect action to adjacent cell

### TC-7.7.5.1 LOD Assignment

| # | Requirement |
|---|-------------|
| 1 | R-7.7.5     |
| 2 | R-7.7.5     |
| 3 | R-7.7.5     |

1. **#1** — Agent at 10m from camera, high_radius=20m
   - **Expected:** Assigned High tier
2. **#2** — Agent at 50m from camera, mid_radius=40m
   - **Expected:** Assigned Low tier
3. **#3** — Agent at 30m from camera, high_radius=20m, mid_radius=40m
   - **Expected:** Assigned Mid tier

### TC-7.7.5.2 LOD Force High

| # | Requirement |
|---|-------------|
| 1 | R-7.7.5     |

1. **#1** — Agent at 100m from camera, force_high_lod=true
   - **Expected:** Assigned High tier (override)

### TC-7.7.5.3 LOD Budget Cap

| # | Requirement |
|---|-------------|
| 1 | R-7.7.5     |

1. **#1** — max_high_lod=50; 100 agents within high_radius
   - **Expected:** Only 50 assigned High; remaining 50 assigned Mid

## Integration Tests

### TC-7.2.3.I1 Steering Pipeline End-to-End

| # | Requirement |
|---|-------------|
| 1 | R-7.2.3     |

1. **#1** — 100 agents with mixed behaviors (seek, flee, arrive), 300 ticks
   - **Expected:** No NaN in positions; zero pairwise overlaps

### TC-7.7.4.I1 Crowd 10K Stability

| # | Requirement |
|---|-------------|
| 1 | R-7.7.4     |

1. **#1** — 10000 crowd agents, 600 ticks
   - **Expected:** CPU time scales linearly with agent count; no exponential blowup

### TC-7.7.5.I1 Mixed LOD Transition

| # | Requirement |
|---|-------------|
| 1 | R-7.7.5     |

1. **#1** — Agent transitions High -> Mid -> Low -> High via camera movement
   - **Expected:** No behavior discontinuity at transitions; smooth velocity

### TC-7.2.5.I1 Formation Through Crowd

| # | Requirement |
|---|-------------|
| 1 | R-7.2.5     |

1. **#1** — 5-member formation moves through 200 crowd agents
   - **Expected:** Members maintain offsets within 1.0m tolerance

### TC-7.7.6.I1 Density With Flow Field

| # | Requirement |
|---|-------------|
| 1 | R-7.7.6     |

1. **#1** — Overflow agents redirected from capped cell
   - **Expected:** Redirected agents reach goal via alternate flow field

### TC-7.7.4.I2 GPU Crowd Matches CPU

| # | Requirement |
|---|-------------|
| 1 | R-7.7.4     |

1. **#1** — 1000 agents, GPU crowd path vs CPU path
   - **Expected:** Positions match within 0.01m tolerance per tick

## Benchmarks

### TC-7.2.3.B1 Steering Behavior Per Agent

| # | Metric              | Target   | Requirement |
|---|---------------------|----------|-------------|
| 1 | Wall time per agent | < 0.5 us | R-7.2.3     |

1. **1** — 6 active behaviors (seek, flee, arrive, wander, pursuit, evade)

### TC-7.2.1.B1 ORCA Solve Per Agent

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 12 neighbors per agent | Wall time per agent | < 2 us | R-7.2.1 |

### TC-7.2.2.B1 Feeler Rays Per Agent

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 5 feeler rays per agent | Wall time per agent | < 1 us | R-7.2.2 |

### TC-7.2.4.B1 Blending Per Agent

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 5 blending layers per agent | Wall time per agent | < 0.2 us | R-7.2.4 |

### TC-7.7.1.B1 Flocking Per Agent

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 16m query radius | Wall time per agent | < 1 us | R-7.7.1 |

### TC-7.7.2.B1 Flow Field Generation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 256x256 grid, Dijkstra generation | Wall time | < 5 ms | R-7.7.2 |

### TC-7.7.2.B2 Flow Field Sample Per Agent

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single cell lookup | Wall time per agent | < 0.05 us | R-7.7.2 |

### TC-7.7.4.B1 Mass Entity Tick Per Agent

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Minimal per-agent state, flow field sample only | Wall time per agent | < 0.5 us | R-7.7.4 |

### TC-7.7.6.B1 Density Enforcement Per Cell

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single cell, 50 agents, cap=20 | Wall time per cell | < 0.1 us | R-7.7.6 |

### TC-7.7.5.B1 LOD Assignment Per Agent

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Distance calculation + tier assignment | Wall time per agent | < 0.1 us | R-7.7.5 |

### TC-7.7.4.B2 GPU Crowd Dispatch

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10000 agents, GPU compute dispatch | GPU time | < 1 ms | R-7.7.4 |
