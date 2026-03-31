# 7.5 — Goal-Oriented Action Planning

## World State

### F-7.5.1 World State Representation

Models AI-relevant world state as a fixed-size bitset of named boolean and integer properties
(has_weapon, health > 50, is_in_cover, target_visible). World states are cheap to copy, compare, and
diff, enabling fast forward-search planning for thousands of NPCs per tick.

- **Dependencies:** None
- **Platform notes:** Compact bitset format; runs identically on all platforms.

## Planner

### F-7.5.2 GOAP Forward-Search Planner

A* search over the action space from the current world state toward a goal state. Each action
declares preconditions and effects as world-state deltas. The planner finds a minimal-cost sequence
of actions that satisfies the goal, producing a plan that the agent executes step by step.

- **Dependencies:** F-7.5.1
- **Platform notes:** Mobile limits planner search depth to 4 actions (vs. 8 on desktop) and caps
  planner iterations per tick to stay within CPU budget.

### F-7.5.3 Action Preconditions & Effects

Each GOAP action defines a set of world-state preconditions that must hold before it can execute and
a set of effects that modify the world state upon completion. Actions also carry a cost used by the
planner to prefer cheaper plans (e.g., melee attack costs less than crafting a weapon to attack).

- **Dependencies:** F-7.5.1
- **Platform notes:** Mobile registers fewer actions per agent archetype to reduce planner branching
  factor and keep search time bounded.

## Plan Management

### F-7.5.4 Plan Caching & Reuse

Caches recently computed plans keyed by (goal, initial-state-hash) so identical planning requests
across multiple NPCs of the same archetype reuse existing plans without re-searching. Cache entries
are invalidated when registered actions change or a configurable TTL expires.

- **Dependencies:** F-7.5.2
- **Platform notes:** Mobile cache holds fewer entries (32 vs. 256 on desktop) due to memory
  constraints; cache hits are critical for mobile performance.

### F-7.5.5 Replanning Triggers

Monitors executing plans for invalidation conditions: an action's preconditions no longer hold, the
goal has changed, or a blackboard observer fires a high-priority event (e.g., ambushed, ally down).
Triggers partial or full replan with a cooldown to prevent thrashing under rapidly changing
conditions.

- **Dependencies:** F-7.5.2, F-7.3.4
- **Platform notes:** Mobile uses longer replan cooldowns (1 s vs. 0.3 s on desktop) to reduce
  planner invocations per agent per second.

### F-7.5.6 Goal Prioritization

Maintains a scored list of goals per agent, each with a dynamic priority derived from utility
considerations or blackboard values. The highest-priority unsatisfied goal drives the planner. When
the active goal is satisfied or a higher-priority goal emerges, the agent abandons the current plan
and replans for the new goal.

- **Dependencies:** F-7.5.2, F-7.4.1
- **Platform notes:** Mobile limits concurrent goal candidates to 4 (vs. 8+ on desktop). Goal
  re-evaluation frequency tied to the BT tick rate.
