# User Stories — 7.3 Behavior Trees

## F-7.3.1 — Core Composite & Leaf Nodes

## US-7.3.1.1 Build Behavior Trees with Composite Nodes
**As a** designer (P-5), **I want to** build behavior trees using Sequence, Selector, and Parallel
composite nodes in the visual editor, **so that** I can author complex NPC behaviors without code.

## US-7.3.1.2 Add Leaf Nodes for Actions and Conditions
**As a** designer (P-5), **I want to** add leaf nodes for actions (move to, attack, play animation)
and conditions (is target visible, is health low), **so that** I can define concrete NPC behaviors.

## US-7.3.1.3 Configure Parallel Node Success/Failure Policy
**As a** designer (P-5), **I want to** configure the success and failure policies on Parallel nodes
(succeed on one, succeed on all), **so that** concurrent behaviors resolve as I intend.

## US-7.3.1.4 See Enemies Make Decisions During Combat
**As a** player (P-23), **I want** enemies to make contextual decisions during combat — seeking
cover when hurt, attacking when advantaged, **so that** AI feels intelligent and adaptive.

## US-7.3.1.5 Watch NPCs Follow Multi-Step Routines
**As a** player (P-23), **I want** NPCs to follow multi-step routines (walk to post, salute, stand
guard), **so that** their daily behavior looks scripted and purposeful.

## US-7.3.1.6 See AI React Differently Based on Situation
**As a** player (P-23), **I want** the same enemy type to react differently based on the situation
(patrol vs. alerted vs. in-combat), **so that** AI behavior has visible depth.

## US-7.3.1.7 Implement Composite Node Execution Logic
**As an** engine developer (P-26), **I want to** implement Sequence (fail-fast), Selector
(succeed-fast), and Parallel (configurable policy) composite nodes, **so that** all standard BT
patterns are supported.

## US-7.3.1.8 Implement Leaf Node Trait Interface
**As an** engine developer (P-26), **I want to** define a leaf node trait interface for actions and
conditions, **so that** gameplay code can register project-specific leaf nodes.

## US-7.3.1.9 Reduce BT Tick Rate on Mobile
**As an** engine developer (P-26), **I want to** tick behavior trees at reduced frequency on mobile
(5-10 Hz vs. 20-30 Hz on desktop) via AI LOD, **so that** BT evaluation fits the mobile CPU budget.

## US-7.3.1.10 Verify Sequence Fails on First Child Failure
**As an** engine tester (P-27), **I want to** verify that a Sequence node returns failure
immediately when any child fails, **so that** fail-fast semantics are correct.

## US-7.3.1.11 Verify Selector Succeeds on First Child Success
**As an** engine tester (P-27), **I want to** verify that a Selector node returns success
immediately when any child succeeds, **so that** succeed-fast semantics are correct.

## US-7.3.1.12 Test Parallel Node with All Policy Combinations
**As an** engine tester (P-27), **I want to** test Parallel nodes with all combinations of success
and failure policies, **so that** concurrent execution resolves correctly in every case.

---

## F-7.3.2 — Decorator Nodes

## US-7.3.2.1 Add Decorator Nodes to Behavior Trees
**As a** designer (P-5), **I want to** add Inverter, Repeater, Succeeder, Rate Limiter, and Cooldown
decorator nodes in the editor, **so that** I can modify child behavior without duplicating subtrees.

## US-7.3.2.2 Configure Repeater Loop Count
**As a** designer (P-5), **I want to** configure a Repeater node to loop N times or until failure,
**so that** I can create patrol loops and retry patterns.

## US-7.3.2.3 Set Cooldown Duration on Abilities
**As a** designer (P-5), **I want to** set a Cooldown decorator duration on ability subtrees, **so
that** NPCs wait between uses of expensive actions like special attacks.

## US-7.3.2.4 See NPCs Patrol in Loops
**As a** player (P-23), **I want** guards to patrol in repeating loops along their routes, **so
that** the area feels continuously guarded.

## US-7.3.2.5 See Enemies Wait Between Special Attacks
**As a** player (P-23), **I want** enemies to wait between special attacks rather than spamming
them, **so that** combat has a readable rhythm.

## US-7.3.2.6 Watch NPCs Retry Failed Actions
**As a** player (P-23), **I want** an NPC that fails to open a door to retry a few times before
giving up, **so that** behavior feels persistent and natural.

## US-7.3.2.7 Implement All Standard Decorator Types
**As an** engine developer (P-26), **I want to** implement Inverter, Repeater, Succeeder, Rate
Limiter, and Cooldown decorators, **so that** the full set of behavior modifiers is available.

## US-7.3.2.8 Use Rate Limiter for Mobile Throttling
**As an** engine developer (P-26), **I want** Rate Limiter decorators to throttle expensive subtrees
independently of the global tick rate, **so that** mobile can selectively reduce cost.

## US-7.3.2.9 Ensure Decorators Preserve Child Return Values
**As an** engine developer (P-26), **I want** decorators (except Inverter and Succeeder) to pass
through child return values unmodified, **so that** wrapping does not alter subtree semantics.

## US-7.3.2.10 Verify Inverter Negates Child Result
**As an** engine tester (P-27), **I want to** verify that Inverter returns success when its child
fails and failure when its child succeeds, **so that** negation logic is correct.

## US-7.3.2.11 Test Cooldown Blocks Re-Entry Within Duration
**As an** engine tester (P-27), **I want to** verify that a Cooldown decorator blocks re-entry for
exactly the configured duration, **so that** timing is accurate.

## US-7.3.2.12 Validate Rate Limiter Throttle Frequency
**As an** engine tester (P-27), **I want to** verify that a Rate Limiter ticks its child at the
configured frequency regardless of the parent tick rate, **so that** throttling works.

---

## F-7.3.3 — Conditional Aborts

## US-7.3.3.1 Enable Self-Abort on Composite Nodes
**As a** designer (P-5), **I want to** enable self-abort on a composite node in the editor, **so
that** the node re-evaluates its conditions while a child is running and aborts if they fail.

## US-7.3.3.2 Enable Lower-Priority Abort
**As a** designer (P-5), **I want to** enable lower-priority abort on a composite node, **so that**
a higher-priority branch can interrupt a running lower-priority sibling.

## US-7.3.3.3 Combine Self and Lower-Priority Aborts
**As a** designer (P-5), **I want to** enable both self-abort and lower-priority abort together,
**so that** the node can re-evaluate in all directions for maximum responsiveness.

## US-7.3.3.4 See Raid Boss React to Phase Transitions Instantly
**As a** player (P-23), **I want** a raid boss to immediately switch behaviors when a phase
transition occurs, **so that** boss fights feel responsive and scripted.

## US-7.3.3.5 Watch Guards Immediately React to Threats
**As a** player (P-23), **I want** patrolling guards to immediately break patrol and react when they
detect a threat, **so that** alert response feels instant.

## US-7.3.3.6 See Enemies Abandon Attacks to Dodge
**As a** player (P-23), **I want** enemies to abort their attack animation to dodge an incoming
projectile, **so that** combat AI shows reactive self-preservation.

## US-7.3.3.7 Implement Conditional Abort Re-Evaluation
**As an** engine developer (P-26), **I want to** implement self-abort and lower-priority abort modes
on composite nodes, **so that** conditions are re-evaluated during child execution.

## US-7.3.3.8 Tie Abort Frequency to BT Tick Rate
**As an** engine developer (P-26), **I want** abort re-evaluation frequency to match the BT tick
rate, **so that** mobile agents react slower but remain functionally correct.

## US-7.3.3.9 Ensure Aborted Nodes Clean Up Properly
**As an** engine developer (P-26), **I want** aborted running nodes to receive a cleanup callback,
**so that** interrupted actions release resources and reset state.

## US-7.3.3.10 Verify Self-Abort Interrupts on Condition Failure
**As an** engine tester (P-27), **I want to** verify that self-abort correctly interrupts a running
child when its condition becomes false, **so that** re-evaluation works.

## US-7.3.3.11 Test Lower-Priority Abort Interrupts Siblings
**As an** engine tester (P-27), **I want to** verify that a higher-priority branch aborts a running
lower-priority sibling when its conditions become true, **so that** priority works.

## US-7.3.3.12 Validate No State Leak on Abort
**As an** engine tester (P-27), **I want to** verify that aborting a running subtree does not leak
blackboard state or running action state, **so that** aborts are clean.

---

## F-7.3.4 — Blackboard System

## US-7.3.4.1 Add Blackboard Keys in Editor
**As a** designer (P-5), **I want to** define typed blackboard keys (int, float, entity, vector) in
the editor, **so that** behavior tree nodes can share data through a named interface.

## US-7.3.4.2 Configure Scoped Keys for Squad Sharing
**As a** designer (P-5), **I want to** create group-scoped blackboard keys (threat target, rally
point) shared among squad members, **so that** team coordination uses shared memory.

## US-7.3.4.3 Register Change Observers for Conditional Aborts
**As a** designer (P-5), **I want to** register blackboard key change observers that trigger
conditional aborts, **so that** behavior reacts immediately when key data changes.

## US-7.3.4.4 See Squad Members Coordinate on Shared Target
**As a** player (P-23), **I want** squad members to all attack the same target when their leader
designates one, **so that** team AI feels coordinated.

## US-7.3.4.5 Watch Guards Share Alert Status
**As a** player (P-23), **I want** guards in the same group to share alert status — when one is
alerted, all become alert, **so that** guard groups react as a team.

## US-7.3.4.6 See AI Remember Last Known Player Position
**As a** player (P-23), **I want** AI to remember where they last saw me and search that area when I
break line of sight, **so that** AI has memory.

## US-7.3.4.7 Implement Typed Key-Value Store Per Agent
**As an** engine developer (P-26), **I want to** implement a typed key-value blackboard per agent
with self, group, and global scopes, **so that** data sharing is structured and efficient.

## US-7.3.4.8 Support Observer Pattern for Key Changes
**As an** engine developer (P-26), **I want to** support observers on blackboard keys that fire on
value changes, **so that** conditional aborts and reactive behaviors trigger automatically.

## US-7.3.4.9 Compact Blackboard Storage for Mobile
**As an** engine developer (P-26), **I want to** use compact storage and limit key count per agent
on mobile, **so that** per-agent memory footprint stays within mobile constraints.

## US-7.3.4.10 Verify Scoped Key Visibility Rules
**As an** engine tester (P-27), **I want to** verify that self-scoped keys are invisible to other
agents and group-scoped keys are visible only within the group, **so that** scope isolation works.

## US-7.3.4.11 Test Observer Fires on Key Change
**As an** engine tester (P-27), **I want to** verify that observers fire exactly once per key change
and not on redundant writes of the same value, **so that** notification is correct.

## US-7.3.4.12 Measure Per-Agent Blackboard Memory
**As an** engine tester (P-27), **I want to** measure per-agent blackboard memory usage and verify
it stays within the configured mobile cap, **so that** memory is bounded.

---

## F-7.3.5 — Behavior Tree Assets & Serialization

## US-7.3.5.1 Author Behavior Trees as Data Assets
**As a** designer (P-5), **I want to** author behavior trees as data assets in a declarative format
(RON or JSON), **so that** NPC behaviors are data-driven and version-controlled.

## US-7.3.5.2 Hot-Reload Trees During Development
**As a** designer (P-5), **I want to** hot-reload behavior tree assets while the server is running,
**so that** I can iterate on NPC behavior without restarting.

## US-7.3.5.3 Reference Custom Leaf Nodes by Name
**As a** designer (P-5), **I want to** reference project-specific leaf nodes by registered name in
the tree asset, **so that** custom actions integrate seamlessly with standard nodes.

## US-7.3.5.4 See NPC Behavior Change Immediately After Edit
**As a** player (P-23), **I want** NPC behavior to change immediately when a designer hot-reloads a
behavior tree, **so that** iteration during playtesting is instant.

## US-7.3.5.5 Experience Consistent NPC Behavior Across Sessions
**As a** player (P-23), **I want** NPC behaviors to be consistent across game sessions because they
are loaded from deterministic data assets, **so that** gameplay is reproducible.

## US-7.3.5.6 See NPCs React Identically After Save and Reload
**As a** player (P-23), **I want** NPCs to behave the same after a save/reload cycle, **so that**
behavior trees produce deterministic results from serialized state.

## US-7.3.5.7 Implement Asset Loader for BT Data Format
**As an** engine developer (P-26), **I want to** implement a loader that parses declarative BT
assets and constructs runtime tree instances, **so that** trees are built from data at load time.

## US-7.3.5.8 Support Hot-Reload in Development Builds
**As an** engine developer (P-26), **I want to** support hot-reload of BT assets in development
builds while stripping it from shipping builds, **so that** iteration speed does not affect release.

## US-7.3.5.9 Pre-Compile Binary BT Assets for Mobile
**As an** engine developer (P-26), **I want to** pre-compile BT assets into binary format for
mobile, **so that** mobile loading skips parsing and is faster.

## US-7.3.5.10 Verify Round-Trip Serialization Fidelity
**As an** engine tester (P-27), **I want to** verify that saving and reloading a BT asset produces
an identical tree structure, **so that** serialization has no data loss.

## US-7.3.5.11 Test Hot-Reload Does Not Corrupt Running Trees
**As an** engine tester (P-27), **I want to** hot-reload a BT asset while agents are mid-execution
and verify no crashes or state corruption, **so that** live reload is safe.

## US-7.3.5.12 Benchmark Binary vs. Text Asset Load Times
**As an** engine tester (P-27), **I want to** compare loading times for binary and text BT assets,
**so that** the mobile binary optimization is validated.

---

## F-7.3.6 — Subtree References & Reuse

## US-7.3.6.1 Reference Shared Subtrees in Editor
**As a** designer (P-5), **I want to** reference shared behavior subtree assets (patrol, flee, call
for help) from any NPC's behavior tree, **so that** common patterns are authored once.

## US-7.3.6.2 Override Subtree Parameters Per NPC
**As a** designer (P-5), **I want to** override subtree parameters (patrol radius, flee speed) per
NPC archetype when referencing a shared subtree, **so that** reuse is flexible.

## US-7.3.6.3 Create Library of Reusable Subtrees
**As a** designer (P-5), **I want to** maintain a library of reusable subtrees in a shared asset
folder, **so that** new NPC types can be assembled from existing behavior modules.

## US-7.3.6.4 See Consistent Patrol Behavior Across NPC Types
**As a** player (P-23), **I want** all guard types to patrol in a consistent manner because they
share the same patrol subtree, **so that** behavior is uniform across the faction.

## US-7.3.6.5 See NPC-Specific Variations on Shared Behaviors
**As a** player (P-23), **I want** different NPC types to have variations on shared behaviors
(faster flee, wider patrol), **so that** AI personalities are distinguishable.

## US-7.3.6.6 Watch Unique NPCs Built from Common Modules
**As a** player (P-23), **I want** NPCs to display complex behaviors composed from recognizable
patterns (patrol + engage + flee), **so that** AI feels deep yet consistent.

## US-7.3.6.7 Implement Subtree Reference Node
**As an** engine developer (P-26), **I want to** implement a node type that references another BT
asset by handle and expands or nests it, **so that** modular tree composition is supported.

## US-7.3.6.8 Support Inline Expansion and Nested Scope
**As an** engine developer (P-26), **I want to** support both inline expansion (merged at load time)
and nested scope (separate blackboard) subtree modes, **so that** designers choose the isolation
level.

## US-7.3.6.9 Load Simplified Subtree Variants on Mobile
**As an** engine developer (P-26), **I want to** load platform-specific simplified subtree variants
on mobile, **so that** tree depth and tick cost are reduced on constrained devices.

## US-7.3.6.10 Verify Shared Subtree Changes Propagate
**As an** engine tester (P-27), **I want to** verify that modifying a shared subtree asset and
reloading updates all NPC trees that reference it, **so that** changes propagate correctly.

## US-7.3.6.11 Test Nested Scope Isolates Blackboard
**As an** engine tester (P-27), **I want to** verify that a nested-scope subtree has its own
blackboard that does not leak into the parent tree, **so that** scope isolation is enforced.

## US-7.3.6.12 Validate Circular Reference Detection
**As an** engine tester (P-27), **I want to** verify that circular subtree references are detected
and reported as errors at load time, **so that** infinite recursion is prevented.

---

## F-7.3.7 — Runtime Debugging & Visualization

## US-7.3.7.1 Select Agent for BT Debug Trace
**As a** designer (P-5), **I want to** select any agent in the editor and see its behavior tree
execution trace, **so that** I can diagnose why it chose a specific action.

## US-7.3.7.2 View Live Blackboard Contents
**As a** designer (P-5), **I want to** view the selected agent's blackboard contents in real time,
**so that** I can inspect the data driving its decisions.

## US-7.3.7.3 See Color-Coded Node States in Editor
**As a** designer (P-5), **I want** the editor to render the tree structure with color-coded node
states (running=yellow, success=green, failure=red), **so that** I can see which branch is active.

## US-7.3.7.4 See AI Make Explainable Decisions
**As a** player (P-23), **I want** AI decisions to be visible through their actions (searching where
I was last seen, flanking, retreating when hurt), **so that** AI behavior feels readable.

## US-7.3.7.5 Notice When AI Changes Behavior Unexpectedly
**As a** player (P-23), **I want** AI behavior transitions to be smooth and motivated by visible
stimuli, **so that** changes do not feel random or broken.

## US-7.3.7.6 Report AI Bugs Reproducibly
**As a** player (P-23), **I want** AI behavior to be deterministic and reproducible from trace logs,
**so that** bugs I report can be reliably investigated by the team.

## US-7.3.7.7 Implement Server-Side BT Trace Log
**As an** engine developer (P-26), **I want to** implement a server-side trace log recording node
visits, results, and blackboard mutations per tick per agent, **so that** behavior is debuggable.

## US-7.3.7.8 Build Editor Overlay for Tree Visualization
**As an** engine developer (P-26), **I want to** build an editor overlay rendering the tree
structure with live node states and blackboard data, **so that** designers can debug visually.

## US-7.3.7.9 Keep Trace Log Available in All Builds
**As an** engine developer (P-26), **I want** the trace log to be available in all builds (not
stripped like visualization), **so that** behavior can be debugged in shipping if needed.

## US-7.3.7.10 Verify Trace Log Matches Actual Execution
**As an** engine tester (P-27), **I want to** verify that the trace log accurately reflects the
actual node execution order and results, **so that** debug data is trustworthy.

## US-7.3.7.11 Test Overlay Updates at BT Tick Rate
**As an** engine tester (P-27), **I want to** verify that the editor overlay updates at the BT tick
rate and reflects the current tick's state, **so that** visualization is not stale.

## US-7.3.7.12 Confirm Visualization Is Editor-Only
**As an** engine tester (P-27), **I want to** confirm that the tree visualization overlay is
compiled only in editor builds and stripped from all other configurations, **so that** shipping
builds have no overhead.
