# User Stories — 7.5 Goal-Oriented Action Planning

## F-7.5.1 — World State Representation

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.5.1.1  | designer (P-5)          | F-7.5.1  | R-7.5.1      |
| US-7.5.1.2  | designer (P-5)          | F-7.5.1  | R-7.5.1      |
| US-7.5.1.3  | designer (P-5)          | F-7.5.1  | R-7.5.1      |
| US-7.5.1.4  | player (P-23)           | F-7.5.1  | R-7.5.1      |
| US-7.5.1.5  | player (P-23)           | F-7.5.1  | R-7.5.1      |
| US-7.5.1.6  | player (P-23)           | F-7.5.1  | R-7.5.1      |
| US-7.5.1.7  | engine developer (P-26) | F-7.5.1  | R-7.5.1      |
| US-7.5.1.8  | engine developer (P-26) | F-7.5.1  | R-7.5.1      |
| US-7.5.1.9  | engine developer (P-26) | F-7.5.1  | R-7.5.1      |
| US-7.5.1.10 | engine tester (P-27)    | F-7.5.1  | R-7.5.1      |
| US-7.5.1.11 | engine tester (P-27)    | F-7.5.1  | R-7.5.1      |
| US-7.5.1.12 | engine tester (P-27)    | F-7.5.1  | R-7.5.1      |

1. **US-7.5.1.1** — I want to define named boolean and integer world-state properties (has_weapon,
   health > 50, is_in_cover, target_visible)
   - **Acceptance:** I can describe AI-relevant conditions without code
2. **US-7.5.1.2** — I want to inspect the current world state of a selected agent in the editor
   - **Acceptance:** I can see which properties are true and diagnose planning failures
3. **US-7.5.1.3** — I want to add and name new world-state properties as part of authoring GOAP
   actions
   - **Acceptance:** the state space matches my game's domain
4. **US-7.5.1.4** — I want NPCs to pause briefly and assess the situation before choosing an action
   - **Acceptance:** decisions look thoughtful rather than instant
5. **US-7.5.1.5** — I want NPCs to change behavior when conditions change (weapon breaks, target
   disappears)
   - **Acceptance:** AI adapts to the current state of the world
6. **US-7.5.1.6** — I want NPCs in the same world state to behave consistently
   - **Acceptance:** AI is predictable and learnable
7. **US-7.5.1.7** — I want to model world state as a fixed-size bitset of boolean and integer
   properties
   - **Acceptance:** states are cheap to copy, compare, and diff
8. **US-7.5.1.8** — I want to support O(1) comparison and diff between world states
   - **Acceptance:** the planner can quickly evaluate preconditions and effects
9. **US-7.5.1.9** — I want world state to be a trivially copyable value type
   - **Acceptance:** the planner can branch states during forward search without allocation
10. **US-7.5.1.10** — I want to verify that boolean and integer properties are correctly encoded and
    decoded from the bitset
    - **Acceptance:** state representation is lossless
11. **US-7.5.1.11** — I want to verify that diffing two world states correctly identifies all
    changed properties
    - **Acceptance:** precondition checks are accurate
12. **US-7.5.1.12** — I want to benchmark copy, compare, and diff operations on world states for
    1000+ concurrent agents
    - **Acceptance:** planner overhead is minimal. ---

## F-7.5.2 — GOAP Forward-Search Planner

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.5.2.1  | designer (P-5)          | F-7.5.2  | R-7.5.2      |
| US-7.5.2.2  | designer (P-5)          | F-7.5.2  | R-7.5.2      |
| US-7.5.2.3  | designer (P-5)          | F-7.5.2  | R-7.5.2      |
| US-7.5.2.4  | player (P-23)           | F-7.5.2  | R-7.5.2      |
| US-7.5.2.5  | player (P-23)           | F-7.5.2  | R-7.5.2      |
| US-7.5.2.6  | player (P-23)           | F-7.5.2  | R-7.5.2      |
| US-7.5.2.7  | engine developer (P-26) | F-7.5.2  | R-7.5.2      |
| US-7.5.2.8  | engine developer (P-26) | F-7.5.2  | R-7.5.2      |
| US-7.5.2.9  | engine developer (P-26) | F-7.5.2  | R-7.5.2      |
| US-7.5.2.10 | engine tester (P-27)    | F-7.5.2  | R-7.5.2      |
| US-7.5.2.11 | engine tester (P-27)    | F-7.5.2  | R-7.5.2      |
| US-7.5.2.12 | engine tester (P-27)    | F-7.5.2  | R-7.5.2      |

1. **US-7.5.2.1** — I want to define goals (kill_target, find_safety, acquire_weapon) for each NPC
   archetype
   - **Acceptance:** the planner generates action sequences toward these goals
2. **US-7.5.2.2** — I want to run the planner in the editor with a selected agent's current state
   and see the generated action plan
   - **Acceptance:** I can validate planner behavior
3. **US-7.5.2.3** — I want to tune the maximum planner search depth per NPC archetype
   - **Acceptance:** plan complexity matches the agent's intended intelligence level
4. **US-7.5.2.4** — I want NPCs to chain logical actions (find weapon, equip weapon, engage target)
   toward a visible goal
   - **Acceptance:** AI behavior appears planned and purposeful
5. **US-7.5.2.5** — I want an NPC that cannot find a weapon to fall back to melee instead of
   standing idle
   - **Acceptance:** the planner produces viable alternatives
6. **US-7.5.2.6** — I want NPCs to execute multi-step plans without awkward pauses between actions
   - **Acceptance:** plan execution feels fluid
7. **US-7.5.2.7** — I want to implement A* forward search from current world state toward goal state
   over the action space
   - **Acceptance:** minimal-cost plans are found
8. **US-7.5.2.8** — I want to limit planner search depth to 4 on mobile (vs. 8 on desktop)
   - **Acceptance:** search time stays within the mobile CPU budget
9. **US-7.5.2.9** — I want to cap planner iterations per tick across all agents
   - **Acceptance:** planning does not exceed the per-tick AI budget
10. **US-7.5.2.10** — I want to verify that the planner finds the lowest-cost action sequence for
    reference scenarios with known optimal solutions
    - **Acceptance:** search is correct
11. **US-7.5.2.11** — I want to verify that the planner gracefully returns no-plan when the goal is
    unreachable
    - **Acceptance:** agents do not hang or crash on impossible goals
12. **US-7.5.2.12** — I want to benchmark the number of plans the planner can compute per tick
    across platforms
    - **Acceptance:** planner throughput meets the agent budget. ---

## F-7.5.3 — Action Preconditions & Effects

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.5.3.1  | designer (P-5)          | F-7.5.3  | R-7.5.3      |
| US-7.5.3.2  | designer (P-5)          | F-7.5.3  | R-7.5.3      |
| US-7.5.3.3  | designer (P-5)          | F-7.5.3  | R-7.5.3      |
| US-7.5.3.4  | player (P-23)           | F-7.5.3  | R-7.5.3      |
| US-7.5.3.5  | player (P-23)           | F-7.5.3  | R-7.5.3      |
| US-7.5.3.6  | player (P-23)           | F-7.5.3  | R-7.5.3      |
| US-7.5.3.7  | engine developer (P-26) | F-7.5.3  | R-7.5.3      |
| US-7.5.3.8  | engine developer (P-26) | F-7.5.3  | R-7.5.3      |
| US-7.5.3.9  | engine developer (P-26) | F-7.5.3  | R-7.5.3      |
| US-7.5.3.10 | engine tester (P-27)    | F-7.5.3  | R-7.5.3      |
| US-7.5.3.11 | engine tester (P-27)    | F-7.5.3  | R-7.5.3      |
| US-7.5.3.12 | engine tester (P-27)    | F-7.5.3  | R-7.5.3      |

1. **US-7.5.3.1** — I want to define world-state preconditions and effects for each GOAP action in
   the editor
   - **Acceptance:** the planner knows when actions are valid and what they change
2. **US-7.5.3.2** — I want to assign a cost to each action so the planner prefers cheaper plans
   (melee attack costs less than crafting a weapon first)
   - **Acceptance:** plan quality reflects designer intent
3. **US-7.5.3.3** — I want to register different action sets per agent archetype
   - **Acceptance:** a warrior and a mage produce different plans from the same goal
4. **US-7.5.3.4** — I want NPCs to use context-appropriate actions (a warrior draws a sword, a mage
   casts a spell)
   - **Acceptance:** plans reflect archetype identity
5. **US-7.5.3.5** — I want NPCs that already have a weapon to skip the "find weapon" step and attack
   directly
   - **Acceptance:** plans are efficient and not redundant
6. **US-7.5.3.6** — I want AI to prefer the cheaper action when two paths lead to the same goal
   - **Acceptance:** plans look practical and resource-conscious
7. **US-7.5.3.7** — I want to implement each action's preconditions and effects as world-state
   deltas
   - **Acceptance:** the planner can apply and check them during search
8. **US-7.5.3.8** — I want to register actions with cost, preconditions, effects, and archetype
   association
   - **Acceptance:** the action registry drives planner behavior
9. **US-7.5.3.9** — I want to register fewer actions per agent archetype on mobile
   - **Acceptance:** planner branching factor is reduced and search time is bounded
10. **US-7.5.3.10** — I want to verify that an action cannot execute when its preconditions are not
    met in the current world state
    - **Acceptance:** gating is enforced
11. **US-7.5.3.11** — I want to verify that executing an action correctly applies its effects to the
    world state
    - **Acceptance:** state transitions are accurate
12. **US-7.5.3.12** — I want to verify that the total plan cost equals the sum of its constituent
    action costs
    - **Acceptance:** cost accounting is correct. ---

## F-7.5.4 — Plan Caching & Reuse

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.5.4.1  | designer (P-5)          | F-7.5.4  | R-7.5.4      |
| US-7.5.4.2  | designer (P-5)          | F-7.5.4  | R-7.5.4      |
| US-7.5.4.3  | designer (P-5)          | F-7.5.4  | R-7.5.4      |
| US-7.5.4.4  | player (P-23)           | F-7.5.4  | R-7.5.4      |
| US-7.5.4.5  | player (P-23)           | F-7.5.4  | R-7.5.4      |
| US-7.5.4.6  | player (P-23)           | F-7.5.4  | R-7.5.4      |
| US-7.5.4.7  | engine developer (P-26) | F-7.5.4  | R-7.5.4      |
| US-7.5.4.8  | engine developer (P-26) | F-7.5.4  | R-7.5.4      |
| US-7.5.4.9  | engine developer (P-26) | F-7.5.4  | R-7.5.4      |
| US-7.5.4.10 | engine tester (P-27)    | F-7.5.4  | R-7.5.4      |
| US-7.5.4.11 | engine tester (P-27)    | F-7.5.4  | R-7.5.4      |
| US-7.5.4.12 | engine tester (P-27)    | F-7.5.4  | R-7.5.4      |

1. **US-7.5.4.1** — I want to see in a debug panel when an NPC reuses a cached plan instead of
   replanning
   - **Acceptance:** I can verify caching is working
2. **US-7.5.4.2** — I want to configure the plan cache TTL
   - **Acceptance:** cached plans are refreshed at an interval matching my game's world-change
     frequency
3. **US-7.5.4.3** — I want to monitor the plan cache hit rate in a debug panel
   - **Acceptance:** I can tune cache size and TTL for optimal performance
4. **US-7.5.4.4** — I want groups of identical NPCs to start acting immediately because they reuse
   cached plans
   - **Acceptance:** there is no visible stagger
5. **US-7.5.4.5** — I want NPCs of the same archetype in the same situation to respond consistently
   - **Acceptance:** behavior is predictable and learnable
6. **US-7.5.4.6** — I want NPCs to replan when conditions invalidate a cached plan
   - **Acceptance:** stale plans do not produce incorrect behavior
7. **US-7.5.4.7** — I want to cache plans keyed by (goal, initial state hash)
   - **Acceptance:** identical planning requests reuse existing results without re-searching
8. **US-7.5.4.8** — I want to invalidate cache entries when registered actions change
   - **Acceptance:** stale plans are never reused after action updates
9. **US-7.5.4.9** — I want to limit cache size to 32 entries on mobile (vs. 256 on desktop)
   - **Acceptance:** memory constraints are respected while cache hits remain critical for mobile
     performance
10. **US-7.5.4.10** — I want to verify that a cache hit returns a plan identical to what a fresh
    search would produce
    - **Acceptance:** caching does not alter plan quality
11. **US-7.5.4.11** — I want to verify that modifying the action registry invalidates all affected
    cache entries
    - **Acceptance:** no stale plans persist
12. **US-7.5.4.12** — I want to benchmark planning throughput with and without caching for 100+
    identical agents
    - **Acceptance:** the cache's performance benefit is quantified. ---

## F-7.5.5 — Replanning Triggers

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.5.5.1  | designer (P-5)          | F-7.5.5  | R-7.5.5      |
| US-7.5.5.2  | designer (P-5)          | F-7.5.5  | R-7.5.5      |
| US-7.5.5.3  | designer (P-5)          | F-7.5.5  | R-7.5.5      |
| US-7.5.5.4  | player (P-23)           | F-7.5.5  | R-7.5.5      |
| US-7.5.5.5  | player (P-23)           | F-7.5.5  | R-7.5.5      |
| US-7.5.5.6  | player (P-23)           | F-7.5.5  | R-7.5.5      |
| US-7.5.5.7  | engine developer (P-26) | F-7.5.5  | R-7.5.5      |
| US-7.5.5.8  | engine developer (P-26) | F-7.5.5  | R-7.5.5      |
| US-7.5.5.9  | engine developer (P-26) | F-7.5.5  | R-7.5.5      |
| US-7.5.5.10 | engine tester (P-27)    | F-7.5.5  | R-7.5.5      |
| US-7.5.5.11 | engine tester (P-27)    | F-7.5.5  | R-7.5.5      |
| US-7.5.5.12 | engine tester (P-27)    | F-7.5.5  | R-7.5.5      |

1. **US-7.5.5.1** — I want to configure which conditions trigger a replan (action precondition
   invalidated, goal changed, high-priority event)
   - **Acceptance:** replanning frequency matches my game's dynamism
2. **US-7.5.5.2** — I want to set a replan cooldown per agent archetype
   - **Acceptance:** rapid condition changes do not cause thrashing
3. **US-7.5.5.3** — I want to remove an NPC's weapon mid-plan and verify it replans to find a new
   weapon or switch to melee
   - **Acceptance:** plan invalidation works correctly
4. **US-7.5.5.4** — I want an NPC that loses its weapon mid-plan to replan immediately (find a new
   weapon or switch to melee)
   - **Acceptance:** NPCs adapt to disruption
5. **US-7.5.5.5** — I want an NPC that is ambushed to drop its current plan and replan for the new
   threat
   - **Acceptance:** surprise attacks provoke immediate reactions
6. **US-7.5.5.6** — I want NPCs to transition smoothly between old and new plans without visibly
   resetting
   - **Acceptance:** replanning is not jarring
7. **US-7.5.5.7** — I want to monitor executing plans for invalidation conditions (preconditions no
   longer hold, goal changed, high-priority event)
   - **Acceptance:** stale plans are detected
8. **US-7.5.5.8** — I want to support both partial replan (retry from current step) and full replan
   (start from scratch)
   - **Acceptance:** the cheapest valid approach is chosen
9. **US-7.5.5.9** — I want mobile to use longer replan cooldowns (1 s vs. 0.3 s on desktop)
   - **Acceptance:** planner invocations per agent per second are reduced
10. **US-7.5.5.10** — I want to verify that a plan is invalidated when an action's preconditions no
    longer hold
    - **Acceptance:** stale plans do not execute impossible actions
11. **US-7.5.5.11** — I want to verify that the replan cooldown prevents multiple replans within the
    cooldown window
    - **Acceptance:** thrashing is avoided
12. **US-7.5.5.12** — I want to verify that high-priority events (ambush, ally down) bypass the
    replan cooldown and trigger immediate replanning
    - **Acceptance:** critical reactions are not delayed. ---

## F-7.5.6 — Goal Prioritization

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.5.6.1  | designer (P-5)          | F-7.5.6  | R-7.5.6      |
| US-7.5.6.2  | designer (P-5)          | F-7.5.6  | R-7.5.6      |
| US-7.5.6.3  | designer (P-5)          | F-7.5.6  | R-7.5.6      |
| US-7.5.6.4  | player (P-23)           | F-7.5.6  | R-7.5.6      |
| US-7.5.6.5  | player (P-23)           | F-7.5.6  | R-7.5.6      |
| US-7.5.6.6  | player (P-23)           | F-7.5.6  | R-7.5.6      |
| US-7.5.6.7  | engine developer (P-26) | F-7.5.6  | R-7.5.6      |
| US-7.5.6.8  | engine developer (P-26) | F-7.5.6  | R-7.5.6      |
| US-7.5.6.9  | engine developer (P-26) | F-7.5.6  | R-7.5.6      |
| US-7.5.6.10 | engine tester (P-27)    | F-7.5.6  | R-7.5.6      |
| US-7.5.6.11 | engine tester (P-27)    | F-7.5.6  | R-7.5.6      |
| US-7.5.6.12 | engine tester (P-27)    | F-7.5.6  | R-7.5.6      |

1. **US-7.5.6.1** — I want to define multiple goals per agent archetype with dynamic priorities
   - **Acceptance:** the agent pursues the most important unsatisfied goal
2. **US-7.5.6.2** — I want to derive goal priorities from utility considerations or blackboard
   values
   - **Acceptance:** priorities adapt to the agent's current context
3. **US-7.5.6.3** — I want to modify blackboard values in the editor and see the agent's active goal
   switch in response
   - **Acceptance:** goal prioritization is observable
4. **US-7.5.6.4** — I want an NPC to switch from "patrol" to "defend self" when attacked
   - **Acceptance:** goal switching is visible and motivated
5. **US-7.5.6.5** — I want NPCs to resume their original goal (patrol, gather resources) after the
   interrupting threat is resolved
   - **Acceptance:** goal hierarchy works
6. **US-7.5.6.6** — I want NPCs to abandon a low-priority gathering task to respond to a combat
   emergency
   - **Acceptance:** urgency drives NPC behavior
7. **US-7.5.6.7** — I want to maintain a scored list of goals per agent with dynamic priorities from
   utility or blackboard values
   - **Acceptance:** the highest-priority unsatisfied goal drives planning
8. **US-7.5.6.8** — I want the agent to abandon its current plan and replan when a higher-priority
   goal emerges
   - **Acceptance:** goal changes produce new action sequences
9. **US-7.5.6.9** — I want to limit concurrent goal candidates to 4 on mobile (vs. 8+ on desktop)
   - **Acceptance:** goal evaluation cost is bounded
10. **US-7.5.6.10** — I want to verify that the planner always plans for the highest-priority
    unsatisfied goal
    - **Acceptance:** goal ordering is correct
11. **US-7.5.6.11** — I want to verify that a satisfied goal does not trigger further planning, and
    the agent moves to the next unsatisfied goal
    - **Acceptance:** goal completion is handled
12. **US-7.5.6.12** — I want to verify that changing a blackboard value updates goal priorities and
    triggers a goal switch when appropriate
    - **Acceptance:** dynamic prioritization works
