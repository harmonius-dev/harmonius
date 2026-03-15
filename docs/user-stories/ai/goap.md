# User Stories — 7.5 Goal-Oriented Action Planning

## F-7.5.1 — World State Representation

## US-7.5.1.1 Define World State Properties in Editor
**As a** designer (P-5), **I want to** define named boolean and integer world-state properties
(has_weapon, health > 50, is_in_cover, target_visible), **so that** I can describe AI-relevant
conditions without code.

## US-7.5.1.2 Preview Current World State Per Agent
**As a** designer (P-5), **I want to** inspect the current world state of a selected agent in
the editor, **so that** I can see which properties are true and diagnose planning failures.

## US-7.5.1.3 Configure World State Property Names and Types
**As a** designer (P-5), **I want to** add and name new world-state properties as part of
authoring GOAP actions, **so that** the state space matches my game's domain.

## US-7.5.1.4 See NPCs Assess Situations Before Acting
**As a** player (P-23), **I want** NPCs to pause briefly and assess the situation before
choosing an action, **so that** decisions look thoughtful rather than instant.

## US-7.5.1.5 Watch NPCs React to Changed Conditions
**As a** player (P-23), **I want** NPCs to change behavior when conditions change (weapon
breaks, target disappears), **so that** AI adapts to the current state of the world.

## US-7.5.1.6 See AI Behave Consistently Given Same Conditions
**As a** player (P-23), **I want** NPCs in the same world state to behave consistently,
**so that** AI is predictable and learnable.

## US-7.5.1.7 Implement Fixed-Size Bitset World State
**As an** engine developer (P-26), **I want to** model world state as a fixed-size bitset of
boolean and integer properties, **so that** states are cheap to copy, compare, and diff.

## US-7.5.1.8 Support State Comparison and Diff Operations
**As an** engine developer (P-26), **I want to** support O(1) comparison and diff between
world states, **so that** the planner can quickly evaluate preconditions and effects.

## US-7.5.1.9 Ensure World State Is Copy-Friendly
**As an** engine developer (P-26), **I want** world state to be a trivially copyable value
type, **so that** the planner can branch states during forward search without allocation.

## US-7.5.1.10 Verify Bitset Correctly Encodes All Property Types
**As an** engine tester (P-27), **I want to** verify that boolean and integer properties are
correctly encoded and decoded from the bitset, **so that** state representation is lossless.

## US-7.5.1.11 Test World State Diff Accuracy
**As an** engine tester (P-27), **I want to** verify that diffing two world states correctly
identifies all changed properties, **so that** precondition checks are accurate.

## US-7.5.1.12 Benchmark World State Operations at Scale
**As an** engine tester (P-27), **I want to** benchmark copy, compare, and diff operations on
world states for 1000+ concurrent agents, **so that** planner overhead is minimal.

---

## F-7.5.2 — GOAP Forward-Search Planner

## US-7.5.2.1 Define Goals for NPC Archetypes
**As a** designer (P-5), **I want to** define goals (kill_target, find_safety, acquire_weapon)
for each NPC archetype, **so that** the planner generates action sequences toward these goals.

## US-7.5.2.2 Test Planner Output in Editor
**As a** designer (P-5), **I want to** run the planner in the editor with a selected agent's
current state and see the generated action plan, **so that** I can validate planner behavior.

## US-7.5.2.3 Tune Planner Search Depth
**As a** designer (P-5), **I want to** tune the maximum planner search depth per NPC
archetype, **so that** plan complexity matches the agent's intended intelligence level.

## US-7.5.2.4 See NPCs Chain Actions Toward Goals
**As a** player (P-23), **I want** NPCs to chain logical actions (find weapon, equip weapon,
engage target) toward a visible goal, **so that** AI behavior appears planned and purposeful.

## US-7.5.2.5 Watch AI Find Alternative Plans
**As a** player (P-23), **I want** an NPC that cannot find a weapon to fall back to melee
instead of standing idle, **so that** the planner produces viable alternatives.

## US-7.5.2.6 See AI Execute Multi-Step Plans Smoothly
**As a** player (P-23), **I want** NPCs to execute multi-step plans without awkward pauses
between actions, **so that** plan execution feels fluid.

## US-7.5.2.7 Implement A* Forward Search Over Action Space
**As an** engine developer (P-26), **I want to** implement A* forward search from current
world state toward goal state over the action space, **so that** minimal-cost plans are found.

## US-7.5.2.8 Limit Search Depth on Mobile
**As an** engine developer (P-26), **I want to** limit planner search depth to 4 on mobile
(vs. 8 on desktop), **so that** search time stays within the mobile CPU budget.

## US-7.5.2.9 Cap Planner Iterations Per Tick
**As an** engine developer (P-26), **I want to** cap planner iterations per tick across all
agents, **so that** planning does not exceed the per-tick AI budget.

## US-7.5.2.10 Verify Planner Finds Optimal Plans
**As an** engine tester (P-27), **I want to** verify that the planner finds the lowest-cost
action sequence for reference scenarios with known optimal solutions, **so that** search is
correct.

## US-7.5.2.11 Test Planner Handles Unsolvable Goals
**As an** engine tester (P-27), **I want to** verify that the planner gracefully returns
no-plan when the goal is unreachable, **so that** agents do not hang or crash on impossible
goals.

## US-7.5.2.12 Benchmark Planner Throughput Per Tick
**As an** engine tester (P-27), **I want to** benchmark the number of plans the planner can
compute per tick across platforms, **so that** planner throughput meets the agent budget.

---

## F-7.5.3 — Action Preconditions & Effects

## US-7.5.3.1 Define Preconditions and Effects Per Action
**As a** designer (P-5), **I want to** define world-state preconditions and effects for each
GOAP action in the editor, **so that** the planner knows when actions are valid and what they
change.

## US-7.5.3.2 Assign Costs to Actions
**As a** designer (P-5), **I want to** assign a cost to each action so the planner prefers
cheaper plans (melee attack costs less than crafting a weapon first), **so that** plan quality
reflects designer intent.

## US-7.5.3.3 Configure Actions Per Agent Archetype
**As a** designer (P-5), **I want to** register different action sets per agent archetype,
**so that** a warrior and a mage produce different plans from the same goal.

## US-7.5.3.4 See NPCs Use Context-Appropriate Actions
**As a** player (P-23), **I want** NPCs to use context-appropriate actions (a warrior draws a
sword, a mage casts a spell), **so that** plans reflect archetype identity.

## US-7.5.3.5 Watch NPCs Skip Unnecessary Actions
**As a** player (P-23), **I want** NPCs that already have a weapon to skip the "find weapon"
step and attack directly, **so that** plans are efficient and not redundant.

## US-7.5.3.6 See AI Choose Cheaper Alternatives
**As a** player (P-23), **I want** AI to prefer the cheaper action when two paths lead to the
same goal, **so that** plans look practical and resource-conscious.

## US-7.5.3.7 Implement Precondition and Effect World-State Deltas
**As an** engine developer (P-26), **I want to** implement each action's preconditions and
effects as world-state deltas, **so that** the planner can apply and check them during search.

## US-7.5.3.8 Register Actions with Cost and Metadata
**As an** engine developer (P-26), **I want to** register actions with cost, preconditions,
effects, and archetype association, **so that** the action registry drives planner behavior.

## US-7.5.3.9 Limit Actions Per Archetype on Mobile
**As an** engine developer (P-26), **I want to** register fewer actions per agent archetype on
mobile, **so that** planner branching factor is reduced and search time is bounded.

## US-7.5.3.10 Verify Preconditions Gate Action Execution
**As an** engine tester (P-27), **I want to** verify that an action cannot execute when its
preconditions are not met in the current world state, **so that** gating is enforced.

## US-7.5.3.11 Test Effects Correctly Modify World State
**As an** engine tester (P-27), **I want to** verify that executing an action correctly
applies its effects to the world state, **so that** state transitions are accurate.

## US-7.5.3.12 Validate Plan Cost Reflects Action Costs
**As an** engine tester (P-27), **I want to** verify that the total plan cost equals the sum
of its constituent action costs, **so that** cost accounting is correct.

---

## F-7.5.4 — Plan Caching & Reuse

## US-7.5.4.1 Observe Cached Plans Reused Across NPCs
**As a** designer (P-5), **I want to** see in a debug panel when an NPC reuses a cached plan
instead of replanning, **so that** I can verify caching is working.

## US-7.5.4.2 Configure Cache TTL
**As a** designer (P-5), **I want to** configure the plan cache TTL, **so that** cached plans
are refreshed at an interval matching my game's world-change frequency.

## US-7.5.4.3 Monitor Cache Hit Rate in Debug
**As a** designer (P-5), **I want to** monitor the plan cache hit rate in a debug panel,
**so that** I can tune cache size and TTL for optimal performance.

## US-7.5.4.4 See Groups of NPCs Act Without Planning Delay
**As a** player (P-23), **I want** groups of identical NPCs to start acting immediately
because they reuse cached plans, **so that** there is no visible stagger.

## US-7.5.4.5 Watch NPCs Respond Consistently to Same Scenario
**As a** player (P-23), **I want** NPCs of the same archetype in the same situation to
respond consistently, **so that** behavior is predictable and learnable.

## US-7.5.4.6 See NPCs Adapt When Conditions Change
**As a** player (P-23), **I want** NPCs to replan when conditions invalidate a cached plan,
**so that** stale plans do not produce incorrect behavior.

## US-7.5.4.7 Implement Goal+State Hash-Keyed Plan Cache
**As an** engine developer (P-26), **I want to** cache plans keyed by (goal, initial state
hash), **so that** identical planning requests reuse existing results without re-searching.

## US-7.5.4.8 Invalidate Cache on Action Set Change
**As an** engine developer (P-26), **I want to** invalidate cache entries when registered
actions change, **so that** stale plans are never reused after action updates.

## US-7.5.4.9 Limit Cache Size on Mobile
**As an** engine developer (P-26), **I want to** limit cache size to 32 entries on mobile
(vs. 256 on desktop), **so that** memory constraints are respected while cache hits remain
critical for mobile performance.

## US-7.5.4.10 Verify Cache Hit Returns Identical Plan
**As an** engine tester (P-27), **I want to** verify that a cache hit returns a plan identical
to what a fresh search would produce, **so that** caching does not alter plan quality.

## US-7.5.4.11 Test Cache Invalidation on Action Change
**As an** engine tester (P-27), **I want to** verify that modifying the action registry
invalidates all affected cache entries, **so that** no stale plans persist.

## US-7.5.4.12 Benchmark Planning With vs. Without Cache
**As an** engine tester (P-27), **I want to** benchmark planning throughput with and without
caching for 100+ identical agents, **so that** the cache's performance benefit is quantified.

---

## F-7.5.5 — Replanning Triggers

## US-7.5.5.1 Configure Replanning Trigger Conditions
**As a** designer (P-5), **I want to** configure which conditions trigger a replan (action
precondition invalidated, goal changed, high-priority event), **so that** replanning
frequency matches my game's dynamism.

## US-7.5.5.2 Set Replan Cooldown Duration
**As a** designer (P-5), **I want to** set a replan cooldown per agent archetype, **so that**
rapid condition changes do not cause thrashing.

## US-7.5.5.3 Test Replanning on Weapon Loss
**As a** designer (P-5), **I want to** remove an NPC's weapon mid-plan and verify it replans
to find a new weapon or switch to melee, **so that** plan invalidation works correctly.

## US-7.5.5.4 See AI Adapt When Plans Are Disrupted
**As a** player (P-23), **I want** an NPC that loses its weapon mid-plan to replan
immediately (find a new weapon or switch to melee), **so that** NPCs adapt to disruption.

## US-7.5.5.5 Watch AI Replan When Ambushed
**As a** player (P-23), **I want** an NPC that is ambushed to drop its current plan and
replan for the new threat, **so that** surprise attacks provoke immediate reactions.

## US-7.5.5.6 See AI Smoothly Transition Between Plans
**As a** player (P-23), **I want** NPCs to transition smoothly between old and new plans
without visibly resetting, **so that** replanning is not jarring.

## US-7.5.5.7 Monitor Executing Plans for Invalidation
**As an** engine developer (P-26), **I want to** monitor executing plans for invalidation
conditions (preconditions no longer hold, goal changed, high-priority event), **so that**
stale plans are detected.

## US-7.5.5.8 Support Partial and Full Replanning
**As an** engine developer (P-26), **I want to** support both partial replan (retry from
current step) and full replan (start from scratch), **so that** the cheapest valid approach
is chosen.

## US-7.5.5.9 Use Longer Replan Cooldowns on Mobile
**As an** engine developer (P-26), **I want** mobile to use longer replan cooldowns (1 s vs.
0.3 s on desktop), **so that** planner invocations per agent per second are reduced.

## US-7.5.5.10 Verify Replan Triggers on Precondition Failure
**As an** engine tester (P-27), **I want to** verify that a plan is invalidated when an
action's preconditions no longer hold, **so that** stale plans do not execute impossible
actions.

## US-7.5.5.11 Test Replan Cooldown Prevents Thrashing
**As an** engine tester (P-27), **I want to** verify that the replan cooldown prevents
multiple replans within the cooldown window, **so that** thrashing is avoided.

## US-7.5.5.12 Validate High-Priority Events Override Cooldown
**As an** engine tester (P-27), **I want to** verify that high-priority events (ambush, ally
down) bypass the replan cooldown and trigger immediate replanning, **so that** critical
reactions are not delayed.

---

## F-7.5.6 — Goal Prioritization

## US-7.5.6.1 Define Multiple Goals Per Agent
**As a** designer (P-5), **I want to** define multiple goals per agent archetype with dynamic
priorities, **so that** the agent pursues the most important unsatisfied goal.

## US-7.5.6.2 Tie Goal Priority to Utility Scores
**As a** designer (P-5), **I want to** derive goal priorities from utility considerations or
blackboard values, **so that** priorities adapt to the agent's current context.

## US-7.5.6.3 Test Goal Switching in Editor
**As a** designer (P-5), **I want to** modify blackboard values in the editor and see the
agent's active goal switch in response, **so that** goal prioritization is observable.

## US-7.5.6.4 See NPCs Shift Goals Based on Urgency
**As a** player (P-23), **I want** an NPC to switch from "patrol" to "defend self" when
attacked, **so that** goal switching is visible and motivated.

## US-7.5.6.5 Watch NPCs Resume Original Goals After Threats Clear
**As a** player (P-23), **I want** NPCs to resume their original goal (patrol, gather
resources) after the interrupting threat is resolved, **so that** goal hierarchy works.

## US-7.5.6.6 See AI Abandon Low-Priority Goals for High-Priority
**As a** player (P-23), **I want** NPCs to abandon a low-priority gathering task to respond
to a combat emergency, **so that** urgency drives NPC behavior.

## US-7.5.6.7 Implement Scored Goal List Per Agent
**As an** engine developer (P-26), **I want to** maintain a scored list of goals per agent
with dynamic priorities from utility or blackboard values, **so that** the highest-priority
unsatisfied goal drives planning.

## US-7.5.6.8 Trigger Replan on Goal Switch
**As an** engine developer (P-26), **I want** the agent to abandon its current plan and replan
when a higher-priority goal emerges, **so that** goal changes produce new action sequences.

## US-7.5.6.9 Limit Concurrent Goal Candidates on Mobile
**As an** engine developer (P-26), **I want to** limit concurrent goal candidates to 4 on
mobile (vs. 8+ on desktop), **so that** goal evaluation cost is bounded.

## US-7.5.6.10 Verify Highest-Priority Goal Drives Planner
**As an** engine tester (P-27), **I want to** verify that the planner always plans for the
highest-priority unsatisfied goal, **so that** goal ordering is correct.

## US-7.5.6.11 Test Goal Satisfaction Stops Replanning
**As an** engine tester (P-27), **I want to** verify that a satisfied goal does not trigger
further planning, and the agent moves to the next unsatisfied goal, **so that** goal
completion is handled.

## US-7.5.6.12 Validate Goal Priority Updates on Blackboard Change
**As an** engine tester (P-27), **I want to** verify that changing a blackboard value updates
goal priorities and triggers a goal switch when appropriate, **so that** dynamic
prioritization works.
