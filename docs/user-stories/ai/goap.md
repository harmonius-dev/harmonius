# User Stories -- 7.5 Goal-Oriented Action Planning

## World State

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.5.1.1  | game designer (P-5)     |
| US-7.5.1.2  | game designer (P-5)     |
| US-7.5.1.3  | engine developer (P-26) |
| US-7.5.1.4  | engine developer (P-26) |

1. **US-7.5.1.1** -- **As a** game designer (P-5), **I want** to define named boolean and integer
   world-state properties for AI agents, **so that** I can describe AI-relevant conditions like
   has_weapon and target_visible without code.

2. **US-7.5.1.2** -- **As a** game designer (P-5), **I want** to inspect the current world state of
   a selected agent in the editor, **so that** I can diagnose planning failures by seeing which
   properties are true or false.

3. **US-7.5.1.3** -- **As an** engine developer (P-26), **I want** to model world state as a
   fixed-size bitset of boolean and integer properties, **so that** states are cheap to copy,
   compare, and diff.

4. **US-7.5.1.4** -- **As an** engine developer (P-26), **I want** world state to be a trivially
   copyable value type with O(1) comparison and diff, **so that** the planner can branch states
   during forward search without heap allocation.

## Planner

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.5.2.1  | game designer (P-5)     |
| US-7.5.2.2  | game designer (P-5)     |
| US-7.5.2.3  | engine developer (P-26) |
| US-7.5.2.4  | engine developer (P-26) |

1. **US-7.5.2.1** -- **As a** game designer (P-5), **I want** to define goals like kill_target,
   find_safety, and acquire_weapon for each NPC archetype, **so that** the planner generates
   contextual action sequences.

2. **US-7.5.2.2** -- **As a** game designer (P-5), **I want** to run the planner in the editor with
   a selected agent's current state and see the generated plan, **so that** I can validate planner
   behavior during authoring.

3. **US-7.5.2.3** -- **As an** engine developer (P-26), **I want** to implement A* forward search
   from the current world state toward a goal state over the action space, **so that** minimal-cost
   plans are found.

4. **US-7.5.2.4** -- **As an** engine developer (P-26), **I want** to cap planner iterations per
   tick and limit search depth per platform tier, **so that** planning never exceeds the per-tick AI
   CPU budget.

## Action Definition

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.5.3.1  | game designer (P-5)     |
| US-7.5.3.2  | game designer (P-5)     |
| US-7.5.3.3  | game developer (P-15)   |
| US-7.5.3.4  | engine developer (P-26) |

1. **US-7.5.3.1** -- **As a** game designer (P-5), **I want** to define world-state preconditions
   and effects for each GOAP action in the editor, **so that** the planner knows when actions are
   valid and what they change.

2. **US-7.5.3.2** -- **As a** game designer (P-5), **I want** to assign a cost to each action,
   **so that** the planner prefers cheaper plans over expensive alternatives.

3. **US-7.5.3.3** -- **As a** game developer (P-15), **I want** to register different action sets
   per agent archetype, **so that** a warrior and a mage produce different plans from the same goal.

4. **US-7.5.3.4** -- **As an** engine developer (P-26), **I want** each action's preconditions and
   effects to be stored as world-state deltas, **so that** the planner can apply and check them
   during search.

## Plan Management

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.5.4.1  | game designer (P-5)     |
| US-7.5.4.2  | engine developer (P-26) |
| US-7.5.4.3  | engine developer (P-26) |
| US-7.5.5.1  | game designer (P-5)     |
| US-7.5.5.2  | game designer (P-5)     |
| US-7.5.5.3  | engine developer (P-26) |
| US-7.5.5.4  | engine developer (P-26) |
| US-7.5.6.1  | game designer (P-5)     |
| US-7.5.6.2  | game designer (P-5)     |
| US-7.5.6.3  | engine developer (P-26) |
| US-7.5.6.4  | engine developer (P-26) |

1. **US-7.5.4.1** -- **As a** game designer (P-5), **I want** to monitor plan cache hit rate in a
   debug panel, **so that** I can tune cache size and TTL for optimal performance.

2. **US-7.5.4.2** -- **As an** engine developer (P-26), **I want** to cache plans keyed by goal and
   initial state hash, **so that** identical planning requests reuse existing results without
   re-searching.

3. **US-7.5.4.3** -- **As an** engine developer (P-26), **I want** cache entries to invalidate when
   registered actions change or a TTL expires, **so that** stale plans are never reused.

4. **US-7.5.5.1** -- **As a** game designer (P-5), **I want** to configure which conditions trigger
   replanning, **so that** replanning frequency matches my game's dynamism.

5. **US-7.5.5.2** -- **As a** game designer (P-5), **I want** to set a replan cooldown per agent
   archetype, **so that** rapid condition changes do not cause plan thrashing.

6. **US-7.5.5.3** -- **As an** engine developer (P-26), **I want** to monitor executing plans for
   invalidation conditions like failed preconditions or goal changes, **so that** stale plans are
   detected and replaced.

7. **US-7.5.5.4** -- **As an** engine developer (P-26), **I want** high-priority events like
   ambushes to bypass the replan cooldown, **so that** critical reactions are never delayed.

8. **US-7.5.6.1** -- **As a** game designer (P-5), **I want** to define multiple goals per agent
   archetype with dynamic priorities, **so that** the agent pursues the most important unsatisfied
   goal.

9. **US-7.5.6.2** -- **As a** game designer (P-5), **I want** goal priorities to derive from utility
   scores or blackboard values, **so that** priorities adapt to the agent's current context.

10. **US-7.5.6.3** -- **As an** engine developer (P-26), **I want** the agent to abandon its current
    plan and replan when a higher-priority goal emerges, **so that** goal changes produce new action
    sequences.

11. **US-7.5.6.4** -- **As an** engine developer (P-26), **I want** concurrent goal candidates to be
    limited per platform tier, **so that** goal evaluation cost is bounded on mobile.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-7.5.1 | game designer (P-5) |
| US-7.5.2 | game designer (P-5) |
| US-7.5.3 | game designer (P-5) |
| US-7.5.4 | game designer (P-5) |
| US-7.5.5 | game designer (P-5) |
| US-7.5.6 | game designer (P-5) |

1. **US-7.5.1** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.5.1.1 through US-7.5.1.4 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

2. **US-7.5.2** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.5.2.1 through US-7.5.2.4 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

3. **US-7.5.3** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.5.3.1 through US-7.5.3.4 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

4. **US-7.5.4** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.5.4.1 through US-7.5.4.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

5. **US-7.5.5** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.5.5.1 through US-7.5.5.4 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

6. **US-7.5.6** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.5.6.1 through US-7.5.6.4 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.
