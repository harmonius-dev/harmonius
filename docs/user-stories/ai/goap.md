# User Stories — 7.5 Goal-Oriented Action Planning

## F-7.5.1 — World State Representation

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.5.1.1 | designer (P-5) | I want to define named boolean and integer world-state properties (has_weapon, health > 50, is_in_cover, target_visible) | I can describe AI-relevant conditions without code | F-7.5.1 | R-7.5.1 |
| US-7.5.1.2 | designer (P-5) | I want to inspect the current world state of a selected agent in the editor | I can see which properties are true and diagnose planning failures | F-7.5.1 | R-7.5.1 |
| US-7.5.1.3 | designer (P-5) | I want to add and name new world-state properties as part of authoring GOAP actions | the state space matches my game's domain | F-7.5.1 | R-7.5.1 |
| US-7.5.1.4 | player (P-23) | I want NPCs to pause briefly and assess the situation before choosing an action | decisions look thoughtful rather than instant | F-7.5.1 | R-7.5.1 |
| US-7.5.1.5 | player (P-23) | I want NPCs to change behavior when conditions change (weapon breaks, target disappears) | AI adapts to the current state of the world | F-7.5.1 | R-7.5.1 |
| US-7.5.1.6 | player (P-23) | I want NPCs in the same world state to behave consistently | AI is predictable and learnable | F-7.5.1 | R-7.5.1 |
| US-7.5.1.7 | engine developer (P-26) | I want to model world state as a fixed-size bitset of boolean and integer properties | states are cheap to copy, compare, and diff | F-7.5.1 | R-7.5.1 |
| US-7.5.1.8 | engine developer (P-26) | I want to support O(1) comparison and diff between world states | the planner can quickly evaluate preconditions and effects | F-7.5.1 | R-7.5.1 |
| US-7.5.1.9 | engine developer (P-26) | I want world state to be a trivially copyable value type | the planner can branch states during forward search without allocation | F-7.5.1 | R-7.5.1 |
| US-7.5.1.10 | engine tester (P-27) | I want to verify that boolean and integer properties are correctly encoded and decoded from the bitset | state representation is lossless | F-7.5.1 | R-7.5.1 |
| US-7.5.1.11 | engine tester (P-27) | I want to verify that diffing two world states correctly identifies all changed properties | precondition checks are accurate | F-7.5.1 | R-7.5.1 |
| US-7.5.1.12 | engine tester (P-27) | I want to benchmark copy, compare, and diff operations on world states for 1000+ concurrent agents | planner overhead is minimal. --- | F-7.5.1 | R-7.5.1 |

## F-7.5.2 — GOAP Forward-Search Planner

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.5.2.1 | designer (P-5) | I want to define goals (kill_target, find_safety, acquire_weapon) for each NPC archetype | the planner generates action sequences toward these goals | F-7.5.2 | R-7.5.2 |
| US-7.5.2.2 | designer (P-5) | I want to run the planner in the editor with a selected agent's current state and see the generated action plan | I can validate planner behavior | F-7.5.2 | R-7.5.2 |
| US-7.5.2.3 | designer (P-5) | I want to tune the maximum planner search depth per NPC archetype | plan complexity matches the agent's intended intelligence level | F-7.5.2 | R-7.5.2 |
| US-7.5.2.4 | player (P-23) | I want NPCs to chain logical actions (find weapon, equip weapon, engage target) toward a visible goal | AI behavior appears planned and purposeful | F-7.5.2 | R-7.5.2 |
| US-7.5.2.5 | player (P-23) | I want an NPC that cannot find a weapon to fall back to melee instead of standing idle | the planner produces viable alternatives | F-7.5.2 | R-7.5.2 |
| US-7.5.2.6 | player (P-23) | I want NPCs to execute multi-step plans without awkward pauses between actions | plan execution feels fluid | F-7.5.2 | R-7.5.2 |
| US-7.5.2.7 | engine developer (P-26) | I want to implement A* forward search from current world state toward goal state over the action space | minimal-cost plans are found | F-7.5.2 | R-7.5.2 |
| US-7.5.2.8 | engine developer (P-26) | I want to limit planner search depth to 4 on mobile (vs. 8 on desktop) | search time stays within the mobile CPU budget | F-7.5.2 | R-7.5.2 |
| US-7.5.2.9 | engine developer (P-26) | I want to cap planner iterations per tick across all agents | planning does not exceed the per-tick AI budget | F-7.5.2 | R-7.5.2 |
| US-7.5.2.10 | engine tester (P-27) | I want to verify that the planner finds the lowest-cost action sequence for reference scenarios with known optimal solutions | search is correct | F-7.5.2 | R-7.5.2 |
| US-7.5.2.11 | engine tester (P-27) | I want to verify that the planner gracefully returns no-plan when the goal is unreachable | agents do not hang or crash on impossible goals | F-7.5.2 | R-7.5.2 |
| US-7.5.2.12 | engine tester (P-27) | I want to benchmark the number of plans the planner can compute per tick across platforms | planner throughput meets the agent budget. --- | F-7.5.2 | R-7.5.2 |

## F-7.5.3 — Action Preconditions & Effects

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.5.3.1 | designer (P-5) | I want to define world-state preconditions and effects for each GOAP action in the editor | the planner knows when actions are valid and what they change | F-7.5.3 | R-7.5.3 |
| US-7.5.3.2 | designer (P-5) | I want to assign a cost to each action so the planner prefers cheaper plans (melee attack costs less than crafting a weapon first) | plan quality reflects designer intent | F-7.5.3 | R-7.5.3 |
| US-7.5.3.3 | designer (P-5) | I want to register different action sets per agent archetype | a warrior and a mage produce different plans from the same goal | F-7.5.3 | R-7.5.3 |
| US-7.5.3.4 | player (P-23) | I want NPCs to use context-appropriate actions (a warrior draws a sword, a mage casts a spell) | plans reflect archetype identity | F-7.5.3 | R-7.5.3 |
| US-7.5.3.5 | player (P-23) | I want NPCs that already have a weapon to skip the "find weapon" step and attack directly | plans are efficient and not redundant | F-7.5.3 | R-7.5.3 |
| US-7.5.3.6 | player (P-23) | I want AI to prefer the cheaper action when two paths lead to the same goal | plans look practical and resource-conscious | F-7.5.3 | R-7.5.3 |
| US-7.5.3.7 | engine developer (P-26) | I want to implement each action's preconditions and effects as world-state deltas | the planner can apply and check them during search | F-7.5.3 | R-7.5.3 |
| US-7.5.3.8 | engine developer (P-26) | I want to register actions with cost, preconditions, effects, and archetype association | the action registry drives planner behavior | F-7.5.3 | R-7.5.3 |
| US-7.5.3.9 | engine developer (P-26) | I want to register fewer actions per agent archetype on mobile | planner branching factor is reduced and search time is bounded | F-7.5.3 | R-7.5.3 |
| US-7.5.3.10 | engine tester (P-27) | I want to verify that an action cannot execute when its preconditions are not met in the current world state | gating is enforced | F-7.5.3 | R-7.5.3 |
| US-7.5.3.11 | engine tester (P-27) | I want to verify that executing an action correctly applies its effects to the world state | state transitions are accurate | F-7.5.3 | R-7.5.3 |
| US-7.5.3.12 | engine tester (P-27) | I want to verify that the total plan cost equals the sum of its constituent action costs | cost accounting is correct. --- | F-7.5.3 | R-7.5.3 |

## F-7.5.4 — Plan Caching & Reuse

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.5.4.1 | designer (P-5) | I want to see in a debug panel when an NPC reuses a cached plan instead of replanning | I can verify caching is working | F-7.5.4 | R-7.5.4 |
| US-7.5.4.2 | designer (P-5) | I want to configure the plan cache TTL | cached plans are refreshed at an interval matching my game's world-change frequency | F-7.5.4 | R-7.5.4 |
| US-7.5.4.3 | designer (P-5) | I want to monitor the plan cache hit rate in a debug panel | I can tune cache size and TTL for optimal performance | F-7.5.4 | R-7.5.4 |
| US-7.5.4.4 | player (P-23) | I want groups of identical NPCs to start acting immediately because they reuse cached plans | there is no visible stagger | F-7.5.4 | R-7.5.4 |
| US-7.5.4.5 | player (P-23) | I want NPCs of the same archetype in the same situation to respond consistently | behavior is predictable and learnable | F-7.5.4 | R-7.5.4 |
| US-7.5.4.6 | player (P-23) | I want NPCs to replan when conditions invalidate a cached plan | stale plans do not produce incorrect behavior | F-7.5.4 | R-7.5.4 |
| US-7.5.4.7 | engine developer (P-26) | I want to cache plans keyed by (goal, initial state hash) | identical planning requests reuse existing results without re-searching | F-7.5.4 | R-7.5.4 |
| US-7.5.4.8 | engine developer (P-26) | I want to invalidate cache entries when registered actions change | stale plans are never reused after action updates | F-7.5.4 | R-7.5.4 |
| US-7.5.4.9 | engine developer (P-26) | I want to limit cache size to 32 entries on mobile (vs. 256 on desktop) | memory constraints are respected while cache hits remain critical for mobile performance | F-7.5.4 | R-7.5.4 |
| US-7.5.4.10 | engine tester (P-27) | I want to verify that a cache hit returns a plan identical to what a fresh search would produce | caching does not alter plan quality | F-7.5.4 | R-7.5.4 |
| US-7.5.4.11 | engine tester (P-27) | I want to verify that modifying the action registry invalidates all affected cache entries | no stale plans persist | F-7.5.4 | R-7.5.4 |
| US-7.5.4.12 | engine tester (P-27) | I want to benchmark planning throughput with and without caching for 100+ identical agents | the cache's performance benefit is quantified. --- | F-7.5.4 | R-7.5.4 |

## F-7.5.5 — Replanning Triggers

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.5.5.1 | designer (P-5) | I want to configure which conditions trigger a replan (action precondition invalidated, goal changed, high-priority event) | replanning frequency matches my game's dynamism | F-7.5.5 | R-7.5.5 |
| US-7.5.5.2 | designer (P-5) | I want to set a replan cooldown per agent archetype | rapid condition changes do not cause thrashing | F-7.5.5 | R-7.5.5 |
| US-7.5.5.3 | designer (P-5) | I want to remove an NPC's weapon mid-plan and verify it replans to find a new weapon or switch to melee | plan invalidation works correctly | F-7.5.5 | R-7.5.5 |
| US-7.5.5.4 | player (P-23) | I want an NPC that loses its weapon mid-plan to replan immediately (find a new weapon or switch to melee) | NPCs adapt to disruption | F-7.5.5 | R-7.5.5 |
| US-7.5.5.5 | player (P-23) | I want an NPC that is ambushed to drop its current plan and replan for the new threat | surprise attacks provoke immediate reactions | F-7.5.5 | R-7.5.5 |
| US-7.5.5.6 | player (P-23) | I want NPCs to transition smoothly between old and new plans without visibly resetting | replanning is not jarring | F-7.5.5 | R-7.5.5 |
| US-7.5.5.7 | engine developer (P-26) | I want to monitor executing plans for invalidation conditions (preconditions no longer hold, goal changed, high-priority event) | stale plans are detected | F-7.5.5 | R-7.5.5 |
| US-7.5.5.8 | engine developer (P-26) | I want to support both partial replan (retry from current step) and full replan (start from scratch) | the cheapest valid approach is chosen | F-7.5.5 | R-7.5.5 |
| US-7.5.5.9 | engine developer (P-26) | I want mobile to use longer replan cooldowns (1 s vs. 0.3 s on desktop) | planner invocations per agent per second are reduced | F-7.5.5 | R-7.5.5 |
| US-7.5.5.10 | engine tester (P-27) | I want to verify that a plan is invalidated when an action's preconditions no longer hold | stale plans do not execute impossible actions | F-7.5.5 | R-7.5.5 |
| US-7.5.5.11 | engine tester (P-27) | I want to verify that the replan cooldown prevents multiple replans within the cooldown window | thrashing is avoided | F-7.5.5 | R-7.5.5 |
| US-7.5.5.12 | engine tester (P-27) | I want to verify that high-priority events (ambush, ally down) bypass the replan cooldown and trigger immediate replanning | critical reactions are not delayed. --- | F-7.5.5 | R-7.5.5 |

## F-7.5.6 — Goal Prioritization

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.5.6.1 | designer (P-5) | I want to define multiple goals per agent archetype with dynamic priorities | the agent pursues the most important unsatisfied goal | F-7.5.6 | R-7.5.6 |
| US-7.5.6.2 | designer (P-5) | I want to derive goal priorities from utility considerations or blackboard values | priorities adapt to the agent's current context | F-7.5.6 | R-7.5.6 |
| US-7.5.6.3 | designer (P-5) | I want to modify blackboard values in the editor and see the agent's active goal switch in response | goal prioritization is observable | F-7.5.6 | R-7.5.6 |
| US-7.5.6.4 | player (P-23) | I want an NPC to switch from "patrol" to "defend self" when attacked | goal switching is visible and motivated | F-7.5.6 | R-7.5.6 |
| US-7.5.6.5 | player (P-23) | I want NPCs to resume their original goal (patrol, gather resources) after the interrupting threat is resolved | goal hierarchy works | F-7.5.6 | R-7.5.6 |
| US-7.5.6.6 | player (P-23) | I want NPCs to abandon a low-priority gathering task to respond to a combat emergency | urgency drives NPC behavior | F-7.5.6 | R-7.5.6 |
| US-7.5.6.7 | engine developer (P-26) | I want to maintain a scored list of goals per agent with dynamic priorities from utility or blackboard values | the highest-priority unsatisfied goal drives planning | F-7.5.6 | R-7.5.6 |
| US-7.5.6.8 | engine developer (P-26) | I want the agent to abandon its current plan and replan when a higher-priority goal emerges | goal changes produce new action sequences | F-7.5.6 | R-7.5.6 |
| US-7.5.6.9 | engine developer (P-26) | I want to limit concurrent goal candidates to 4 on mobile (vs. 8+ on desktop) | goal evaluation cost is bounded | F-7.5.6 | R-7.5.6 |
| US-7.5.6.10 | engine tester (P-27) | I want to verify that the planner always plans for the highest-priority unsatisfied goal | goal ordering is correct | F-7.5.6 | R-7.5.6 |
| US-7.5.6.11 | engine tester (P-27) | I want to verify that a satisfied goal does not trigger further planning, and the agent moves to the next unsatisfied goal | goal completion is handled | F-7.5.6 | R-7.5.6 |
| US-7.5.6.12 | engine tester (P-27) | I want to verify that changing a blackboard value updates goal priorities and triggers a goal switch when appropriate | dynamic prioritization works | F-7.5.6 | R-7.5.6 |
