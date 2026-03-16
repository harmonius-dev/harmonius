# User Stories — 7.3 Behavior Trees

## F-7.3.1 — Core Composite & Leaf Nodes

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.3.1.1 | designer (P-5) | I want to build behavior trees using Sequence, Selector, and Parallel composite nodes in the visual editor | I can author complex NPC behaviors without code | F-7.3.1 | R-7.3.1 |
| US-7.3.1.2 | designer (P-5) | I want to add leaf nodes for actions (move to, attack, play animation) and conditions (is target visible, is health low) | I can define concrete NPC behaviors | F-7.3.1 | R-7.3.1 |
| US-7.3.1.3 | designer (P-5) | I want to configure the success and failure policies on Parallel nodes (succeed on one, succeed on all) | concurrent behaviors resolve as I intend | F-7.3.1 | R-7.3.1 |
| US-7.3.1.4 | player (P-23) | I want enemies to make contextual decisions during combat — seeking cover when hurt, attacking when advantaged | AI feels intelligent and adaptive | F-7.3.1 | R-7.3.1 |
| US-7.3.1.5 | player (P-23) | I want NPCs to follow multi-step routines (walk to post, salute, stand guard) | their daily behavior looks scripted and purposeful | F-7.3.1 | R-7.3.1 |
| US-7.3.1.6 | player (P-23) | I want the same enemy type to react differently based on the situation (patrol vs. alerted vs. in-combat) | AI behavior has visible depth | F-7.3.1 | R-7.3.1 |
| US-7.3.1.7 | engine developer (P-26) | I want to implement Sequence (fail-fast), Selector (succeed-fast), and Parallel (configurable policy) composite nodes | all standard BT patterns are supported | F-7.3.1 | R-7.3.1 |
| US-7.3.1.8 | engine developer (P-26) | I want to define a leaf node trait interface for actions and conditions | gameplay code can register project-specific leaf nodes | F-7.3.1 | R-7.3.1 |
| US-7.3.1.9 | engine developer (P-26) | I want to tick behavior trees at reduced frequency on mobile (5-10 Hz vs. 20-30 Hz on desktop) via AI LOD | BT evaluation fits the mobile CPU budget | F-7.3.1 | R-7.3.1 |
| US-7.3.1.10 | engine tester (P-27) | I want to verify that a Sequence node returns failure immediately when any child fails | fail-fast semantics are correct | F-7.3.1 | R-7.3.1 |
| US-7.3.1.11 | engine tester (P-27) | I want to verify that a Selector node returns success immediately when any child succeeds | succeed-fast semantics are correct | F-7.3.1 | R-7.3.1 |
| US-7.3.1.12 | engine tester (P-27) | I want to test Parallel nodes with all combinations of success and failure policies | concurrent execution resolves correctly in every case. --- | F-7.3.1 | R-7.3.1 |

## F-7.3.2 — Decorator Nodes

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.3.2.1 | designer (P-5) | I want to add Inverter, Repeater, Succeeder, Rate Limiter, and Cooldown decorator nodes in the editor | I can modify child behavior without duplicating subtrees | F-7.3.2 | R-7.3.2 |
| US-7.3.2.2 | designer (P-5) | I want to configure a Repeater node to loop N times or until failure | I can create patrol loops and retry patterns | F-7.3.2 | R-7.3.2 |
| US-7.3.2.3 | designer (P-5) | I want to set a Cooldown decorator duration on ability subtrees | NPCs wait between uses of expensive actions like special attacks | F-7.3.2 | R-7.3.2 |
| US-7.3.2.4 | player (P-23) | I want guards to patrol in repeating loops along their routes | the area feels continuously guarded | F-7.3.2 | R-7.3.2 |
| US-7.3.2.5 | player (P-23) | I want enemies to wait between special attacks rather than spamming them | combat has a readable rhythm | F-7.3.2 | R-7.3.2 |
| US-7.3.2.6 | player (P-23) | I want an NPC that fails to open a door to retry a few times before giving up | behavior feels persistent and natural | F-7.3.2 | R-7.3.2 |
| US-7.3.2.7 | engine developer (P-26) | I want to implement Inverter, Repeater, Succeeder, Rate Limiter, and Cooldown decorators | the full set of behavior modifiers is available | F-7.3.2 | R-7.3.2 |
| US-7.3.2.8 | engine developer (P-26) | I want Rate Limiter decorators to throttle expensive subtrees independently of the global tick rate | mobile can selectively reduce cost | F-7.3.2 | R-7.3.2 |
| US-7.3.2.9 | engine developer (P-26) | I want decorators (except Inverter and Succeeder) to pass through child return values unmodified | wrapping does not alter subtree semantics | F-7.3.2 | R-7.3.2 |
| US-7.3.2.10 | engine tester (P-27) | I want to verify that Inverter returns success when its child fails and failure when its child succeeds | negation logic is correct | F-7.3.2 | R-7.3.2 |
| US-7.3.2.11 | engine tester (P-27) | I want to verify that a Cooldown decorator blocks re-entry for exactly the configured duration | timing is accurate | F-7.3.2 | R-7.3.2 |
| US-7.3.2.12 | engine tester (P-27) | I want to verify that a Rate Limiter ticks its child at the configured frequency regardless of the parent tick rate | throttling works. --- | F-7.3.2 | R-7.3.2 |

## F-7.3.3 — Conditional Aborts

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.3.3.1 | designer (P-5) | I want to enable self-abort on a composite node in the editor | the node re-evaluates its conditions while a child is running and aborts if they fail | F-7.3.3 | R-7.3.3 |
| US-7.3.3.2 | designer (P-5) | I want to enable lower-priority abort on a composite node | a higher-priority branch can interrupt a running lower-priority sibling | F-7.3.3 | R-7.3.3 |
| US-7.3.3.3 | designer (P-5) | I want to enable both self-abort and lower-priority abort together | the node can re-evaluate in all directions for maximum responsiveness | F-7.3.3 | R-7.3.3 |
| US-7.3.3.4 | player (P-23) | I want a raid boss to immediately switch behaviors when a phase transition occurs | boss fights feel responsive and scripted | F-7.3.3 | R-7.3.3 |
| US-7.3.3.5 | player (P-23) | I want patrolling guards to immediately break patrol and react when they detect a threat | alert response feels instant | F-7.3.3 | R-7.3.3 |
| US-7.3.3.6 | player (P-23) | I want enemies to abort their attack animation to dodge an incoming projectile | combat AI shows reactive self-preservation | F-7.3.3 | R-7.3.3 |
| US-7.3.3.7 | engine developer (P-26) | I want to implement self-abort and lower-priority abort modes on composite nodes | conditions are re-evaluated during child execution | F-7.3.3 | R-7.3.3 |
| US-7.3.3.8 | engine developer (P-26) | I want abort re-evaluation frequency to match the BT tick rate | mobile agents react slower but remain functionally correct | F-7.3.3 | R-7.3.3 |
| US-7.3.3.9 | engine developer (P-26) | I want aborted running nodes to receive a cleanup callback | interrupted actions release resources and reset state | F-7.3.3 | R-7.3.3 |
| US-7.3.3.10 | engine tester (P-27) | I want to verify that self-abort correctly interrupts a running child when its condition becomes false | re-evaluation works | F-7.3.3 | R-7.3.3 |
| US-7.3.3.11 | engine tester (P-27) | I want to verify that a higher-priority branch aborts a running lower-priority sibling when its conditions become true | priority works | F-7.3.3 | R-7.3.3 |
| US-7.3.3.12 | engine tester (P-27) | I want to verify that aborting a running subtree does not leak blackboard state or running action state | aborts are clean. --- | F-7.3.3 | R-7.3.3 |

## F-7.3.4 — Blackboard System

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.3.4.1 | designer (P-5) | I want to define typed blackboard keys (int, float, entity, vector) in the editor | behavior tree nodes can share data through a named interface | F-7.3.4 | R-7.3.4 |
| US-7.3.4.2 | designer (P-5) | I want to create group-scoped blackboard keys (threat target, rally point) shared among squad members | team coordination uses shared memory | F-7.3.4 | R-7.3.4 |
| US-7.3.4.3 | designer (P-5) | I want to register blackboard key change observers that trigger conditional aborts | behavior reacts immediately when key data changes | F-7.3.4 | R-7.3.4 |
| US-7.3.4.4 | player (P-23) | I want squad members to all attack the same target when their leader designates one | team AI feels coordinated | F-7.3.4 | R-7.3.4 |
| US-7.3.4.5 | player (P-23) | I want guards in the same group to share alert status — when one is alerted, all become alert | guard groups react as a team | F-7.3.4 | R-7.3.4 |
| US-7.3.4.6 | player (P-23) | I want AI to remember where they last saw me and search that area when I break line of sight | AI has memory | F-7.3.4 | R-7.3.4 |
| US-7.3.4.7 | engine developer (P-26) | I want to implement a typed key-value blackboard per agent with self, group, and global scopes | data sharing is structured and efficient | F-7.3.4 | R-7.3.4 |
| US-7.3.4.8 | engine developer (P-26) | I want to support observers on blackboard keys that fire on value changes | conditional aborts and reactive behaviors trigger automatically | F-7.3.4 | R-7.3.4 |
| US-7.3.4.9 | engine developer (P-26) | I want to use compact storage and limit key count per agent on mobile | per-agent memory footprint stays within mobile constraints | F-7.3.4 | R-7.3.4 |
| US-7.3.4.10 | engine tester (P-27) | I want to verify that self-scoped keys are invisible to other agents and group-scoped keys are visible only within the group | scope isolation works | F-7.3.4 | R-7.3.4 |
| US-7.3.4.11 | engine tester (P-27) | I want to verify that observers fire exactly once per key change and not on redundant writes of the same value | notification is correct | F-7.3.4 | R-7.3.4 |
| US-7.3.4.12 | engine tester (P-27) | I want to measure per-agent blackboard memory usage and verify it stays within the configured mobile cap | memory is bounded. --- | F-7.3.4 | R-7.3.4 |

## F-7.3.5 — Behavior Tree Assets & Serialization

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.3.5.1 | designer (P-5) | I want to author behavior trees as data assets in a declarative format (RON or JSON) | NPC behaviors are data-driven and version-controlled | F-7.3.5 | R-7.3.5 |
| US-7.3.5.2 | designer (P-5) | I want to hot-reload behavior tree assets while the server is running | I can iterate on NPC behavior without restarting | F-7.3.5 | R-7.3.5 |
| US-7.3.5.3 | designer (P-5) | I want to reference project-specific leaf nodes by registered name in the tree asset | custom actions integrate seamlessly with standard nodes | F-7.3.5 | R-7.3.5 |
| US-7.3.5.4 | player (P-23) | I want NPC behavior to change immediately when a designer hot-reloads a behavior tree | iteration during playtesting is instant | F-7.3.5 | R-7.3.5 |
| US-7.3.5.5 | player (P-23) | I want NPC behaviors to be consistent across game sessions because they are loaded from deterministic data assets | gameplay is reproducible | F-7.3.5 | R-7.3.5 |
| US-7.3.5.6 | player (P-23) | I want NPCs to behave the same after a save/reload cycle | behavior trees produce deterministic results from serialized state | F-7.3.5 | R-7.3.5 |
| US-7.3.5.7 | engine developer (P-26) | I want to implement a loader that parses declarative BT assets and constructs runtime tree instances | trees are built from data at load time | F-7.3.5 | R-7.3.5 |
| US-7.3.5.8 | engine developer (P-26) | I want to support hot-reload of BT assets in development builds while stripping it from shipping builds | iteration speed does not affect release | F-7.3.5 | R-7.3.5 |
| US-7.3.5.9 | engine developer (P-26) | I want to pre-compile BT assets into binary format for mobile | mobile loading skips parsing and is faster | F-7.3.5 | R-7.3.5 |
| US-7.3.5.10 | engine tester (P-27) | I want to verify that saving and reloading a BT asset produces an identical tree structure | serialization has no data loss | F-7.3.5 | R-7.3.5 |
| US-7.3.5.11 | engine tester (P-27) | I want to hot-reload a BT asset while agents are mid-execution and verify no crashes or state corruption | live reload is safe | F-7.3.5 | R-7.3.5 |
| US-7.3.5.12 | engine tester (P-27) | I want to compare loading times for binary and text BT assets | the mobile binary optimization is validated. --- | F-7.3.5 | R-7.3.5 |

## F-7.3.6 — Subtree References & Reuse

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.3.6.1 | designer (P-5) | I want to reference shared behavior subtree assets (patrol, flee, call for help) from any NPC's behavior tree | common patterns are authored once | F-7.3.6 | R-7.3.6 |
| US-7.3.6.2 | designer (P-5) | I want to override subtree parameters (patrol radius, flee speed) per NPC archetype when referencing a shared subtree | reuse is flexible | F-7.3.6 | R-7.3.6 |
| US-7.3.6.3 | designer (P-5) | I want to maintain a library of reusable subtrees in a shared asset folder | new NPC types can be assembled from existing behavior modules | F-7.3.6 | R-7.3.6 |
| US-7.3.6.4 | player (P-23) | I want all guard types to patrol in a consistent manner because they share the same patrol subtree | behavior is uniform across the faction | F-7.3.6 | R-7.3.6 |
| US-7.3.6.5 | player (P-23) | I want different NPC types to have variations on shared behaviors (faster flee, wider patrol) | AI personalities are distinguishable | F-7.3.6 | R-7.3.6 |
| US-7.3.6.6 | player (P-23) | I want NPCs to display complex behaviors composed from recognizable patterns (patrol + engage + flee) | AI feels deep yet consistent | F-7.3.6 | R-7.3.6 |
| US-7.3.6.7 | engine developer (P-26) | I want to implement a node type that references another BT asset by handle and expands or nests it | modular tree composition is supported | F-7.3.6 | R-7.3.6 |
| US-7.3.6.8 | engine developer (P-26) | I want to support both inline expansion (merged at load time) and nested scope (separate blackboard) subtree modes | designers choose the isolation level | F-7.3.6 | R-7.3.6 |
| US-7.3.6.9 | engine developer (P-26) | I want to load platform-specific simplified subtree variants on mobile | tree depth and tick cost are reduced on constrained devices | F-7.3.6 | R-7.3.6 |
| US-7.3.6.10 | engine tester (P-27) | I want to verify that modifying a shared subtree asset and reloading updates all NPC trees that reference it | changes propagate correctly | F-7.3.6 | R-7.3.6 |
| US-7.3.6.11 | engine tester (P-27) | I want to verify that a nested-scope subtree has its own blackboard that does not leak into the parent tree | scope isolation is enforced | F-7.3.6 | R-7.3.6 |
| US-7.3.6.12 | engine tester (P-27) | I want to verify that circular subtree references are detected and reported as errors at load time | infinite recursion is prevented. --- | F-7.3.6 | R-7.3.6 |

## F-7.3.7 — Runtime Debugging & Visualization

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.3.7.1 | designer (P-5) | I want to select any agent in the editor and see its behavior tree execution trace | I can diagnose why it chose a specific action | F-7.3.7 | R-7.3.7 |
| US-7.3.7.2 | designer (P-5) | I want to view the selected agent's blackboard contents in real time | I can inspect the data driving its decisions | F-7.3.7 | R-7.3.7 |
| US-7.3.7.3 | designer (P-5) | I want the editor to render the tree structure with color-coded node states (running=yellow, success=green, failure=red) | I can see which branch is active | F-7.3.7 | R-7.3.7 |
| US-7.3.7.4 | player (P-23) | I want AI decisions to be visible through their actions (searching where I was last seen, flanking, retreating when hurt) | AI behavior feels readable | F-7.3.7 | R-7.3.7 |
| US-7.3.7.5 | player (P-23) | I want AI behavior transitions to be smooth and motivated by visible stimuli | changes do not feel random or broken | F-7.3.7 | R-7.3.7 |
| US-7.3.7.6 | player (P-23) | I want AI behavior to be deterministic and reproducible from trace logs | bugs I report can be reliably investigated by the team | F-7.3.7 | R-7.3.7 |
| US-7.3.7.7 | engine developer (P-26) | I want to implement a server-side trace log recording node visits, results, and blackboard mutations per tick per agent | behavior is debuggable | F-7.3.7 | R-7.3.7 |
| US-7.3.7.8 | engine developer (P-26) | I want to build an editor overlay rendering the tree structure with live node states and blackboard data | designers can debug visually | F-7.3.7 | R-7.3.7 |
| US-7.3.7.9 | engine developer (P-26) | I want the trace log to be available in all builds (not stripped like visualization) | behavior can be debugged in shipping if needed | F-7.3.7 | R-7.3.7 |
| US-7.3.7.10 | engine tester (P-27) | I want to verify that the trace log accurately reflects the actual node execution order and results | debug data is trustworthy | F-7.3.7 | R-7.3.7 |
| US-7.3.7.11 | engine tester (P-27) | I want to verify that the editor overlay updates at the BT tick rate and reflects the current tick's state | visualization is not stale | F-7.3.7 | R-7.3.7 |
| US-7.3.7.12 | engine tester (P-27) | I want to confirm that the tree visualization overlay is compiled only in editor builds and stripped from all other configurations | shipping builds have no overhead | F-7.3.7 | R-7.3.7 |
