# User Stories -- 7.3 Behavior Trees

## Node Types

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.3.1.1  | game designer (P-5)     |
| US-7.3.1.2  | game designer (P-5)     |
| US-7.3.1.3  | game designer (P-5)     |
| US-7.3.1.4  | game developer (P-15)   |
| US-7.3.1.5  | engine developer (P-26) |
| US-7.3.1.6  | engine developer (P-26) |
| US-7.3.2.1  | game designer (P-5)     |
| US-7.3.2.2  | game designer (P-5)     |
| US-7.3.2.3  | level designer (P-6)    |
| US-7.3.2.4  | engine developer (P-26) |
| US-7.3.2.5  | engine developer (P-26) |
| US-7.3.3.1  | game designer (P-5)     |
| US-7.3.3.2  | game designer (P-5)     |
| US-7.3.3.3  | level designer (P-6)    |
| US-7.3.3.4  | engine developer (P-26) |
| US-7.3.3.5  | engine developer (P-26) |

1. **US-7.3.1.1** -- **As a** game designer (P-5), **I want** to build behavior trees using
   Sequence, Selector, and Parallel composite nodes in the visual editor, **so that** I can author
   complex NPC behaviors without writing code.

2. **US-7.3.1.2** -- **As a** game designer (P-5), **I want** to add leaf nodes for actions and
   conditions to behavior trees, **so that** I can define concrete NPC behaviors like move-to,
   attack, and perception checks.

3. **US-7.3.1.3** -- **As a** game designer (P-5), **I want** to configure success and failure
   policies on Parallel nodes, **so that** concurrent child behaviors resolve according to my design
   intent.

4. **US-7.3.1.4** -- **As a** game developer (P-15), **I want** to register project-specific leaf
   node types through a trait interface, **so that** game-specific actions integrate seamlessly with
   the built-in node library.

5. **US-7.3.1.5** -- **As an** engine developer (P-26), **I want** to implement Sequence, Selector,
   and Parallel composite nodes with correct fail-fast, succeed-fast, and configurable-policy
   semantics, **so that** all standard behavior tree patterns are expressible.

6. **US-7.3.1.6** -- **As an** engine developer (P-26), **I want** behavior tree tick frequency to
   be configurable per AI LOD tier, **so that** lower-tier agents consume less CPU while remaining
   functionally correct.

7. **US-7.3.2.1** -- **As a** game designer (P-5), **I want** to wrap child nodes with Inverter,
   Repeater, Succeeder, Rate Limiter, and Cooldown decorators, **so that** I can modify behavior
   without duplicating subtrees.

8. **US-7.3.2.2** -- **As a** game designer (P-5), **I want** to configure Repeater loop count and
   Cooldown duration per decorator instance, **so that** patrol loops and ability cooldowns match my
   design parameters.

9. **US-7.3.2.3** -- **As a** level designer (P-6), **I want** guards to use Repeater decorators on
   patrol routes, **so that** patrol loops are continuous and the area feels guarded at all times.

10. **US-7.3.2.4** -- **As an** engine developer (P-26), **I want** to implement all five decorator
    node types with correct pass-through semantics, **so that** wrapping a child does not alter its
    return value unless the decorator's purpose is to modify it.

11. **US-7.3.2.5** -- **As an** engine developer (P-26), **I want** Rate Limiter decorators to
    throttle subtree ticks independently of the global tick rate, **so that** expensive subtrees can
    be selectively rate-limited on constrained platforms.

12. **US-7.3.3.1** -- **As a** game designer (P-5), **I want** to enable self-abort and
    lower-priority abort modes on composite nodes, **so that** higher-priority branches can
    interrupt running lower-priority siblings.

13. **US-7.3.3.2** -- **As a** game designer (P-5), **I want** conditional aborts to fire within the
    same tick when conditions change, **so that** NPCs react immediately to phase transitions and
    threat detection.

14. **US-7.3.3.3** -- **As a** level designer (P-6), **I want** patrolling guards to abort patrol
    and enter combat the instant they detect a threat, **so that** alert response feels
    instantaneous to the player.

15. **US-7.3.3.4** -- **As an** engine developer (P-26), **I want** to implement self-abort and
    lower-priority abort re-evaluation on composite nodes, **so that** conditions are re-checked
    during child execution.

16. **US-7.3.3.5** -- **As an** engine developer (P-26), **I want** aborted running nodes to receive
    a cleanup callback, **so that** interrupted actions can release resources and reset state
    without leaks.

## Blackboard

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.3.4.1  | game designer (P-5)     |
| US-7.3.4.2  | game designer (P-5)     |
| US-7.3.4.3  | game developer (P-15)   |
| US-7.3.4.4  | engine developer (P-26) |
| US-7.3.4.5  | engine developer (P-26) |

1. **US-7.3.4.1** -- **As a** game designer (P-5), **I want** to define typed blackboard keys with
   self, group, and global scopes, **so that** behavior tree nodes can share data through a
   structured named interface.

2. **US-7.3.4.2** -- **As a** game designer (P-5), **I want** to register change observers on
   blackboard keys that trigger conditional aborts, **so that** behavior reacts immediately when
   shared data changes.

3. **US-7.3.4.3** -- **As a** game developer (P-15), **I want** group-scoped blackboard keys shared
   among squad members, **so that** team coordination data like threat targets and rally points is
   accessible to all members.

4. **US-7.3.4.4** -- **As an** engine developer (P-26), **I want** to implement a typed key-value
   blackboard per agent with scope isolation, **so that** self-scoped keys are invisible to other
   agents and group-scoped keys are visible only within the group.

5. **US-7.3.4.5** -- **As an** engine developer (P-26), **I want** per-agent blackboard memory to be
   bounded by a configurable key count limit, **so that** memory footprint stays within mobile
   platform constraints.

## Assets and Serialization

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.3.5.1  | game designer (P-5)     |
| US-7.3.5.2  | game designer (P-5)     |
| US-7.3.5.3  | game developer (P-15)   |
| US-7.3.5.4  | engine developer (P-26) |
| US-7.3.5.5  | engine developer (P-26) |
| US-7.3.6.1  | game designer (P-5)     |
| US-7.3.6.2  | game designer (P-5)     |
| US-7.3.6.3  | engine developer (P-26) |
| US-7.3.6.4  | engine developer (P-26) |

1. **US-7.3.5.1** -- **As a** game designer (P-5), **I want** to author behavior trees as
   declarative data assets in RON or JSON format, **so that** NPC behaviors are data-driven and
   version-controlled.

2. **US-7.3.5.2** -- **As a** game designer (P-5), **I want** to hot-reload behavior tree assets
   while the game is running, **so that** I can iterate on NPC behavior without restarting.

3. **US-7.3.5.3** -- **As a** game developer (P-15), **I want** to reference project-specific leaf
   nodes by registered name in tree assets, **so that** custom actions integrate with the standard
   node library.

4. **US-7.3.5.4** -- **As an** engine developer (P-26), **I want** to implement a loader that parses
   declarative BT assets and constructs runtime tree instances, **so that** trees are built from
   data at load time.

5. **US-7.3.5.5** -- **As an** engine developer (P-26), **I want** to pre-compile BT assets into a
   binary format for mobile, **so that** mobile loading skips text parsing and is faster.

6. **US-7.3.6.1** -- **As a** game designer (P-5), **I want** to reference shared subtree assets
   from any NPC behavior tree, **so that** common patterns like patrol and flee are authored once
   and reused.

7. **US-7.3.6.2** -- **As a** game designer (P-5), **I want** to override subtree parameters per NPC
   archetype when referencing a shared subtree, **so that** reuse is flexible across different NPC
   types.

8. **US-7.3.6.3** -- **As an** engine developer (P-26), **I want** to support both inline expansion
   and nested scope subtree modes, **so that** designers can choose the blackboard isolation level.

9. **US-7.3.6.4** -- **As an** engine developer (P-26), **I want** circular subtree references to be
   detected and reported as errors at load time, **so that** infinite recursion is prevented.

## Debugging

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.3.7.1  | game designer (P-5)     |
| US-7.3.7.2  | game designer (P-5)     |
| US-7.3.7.3  | engine developer (P-26) |
| US-7.3.7.4  | engine developer (P-26) |

1. **US-7.3.7.1** -- **As a** game designer (P-5), **I want** to select an agent in the editor and
   see its behavior tree execution trace with color-coded node states, **so that** I can diagnose
   why it chose a specific action.

2. **US-7.3.7.2** -- **As a** game designer (P-5), **I want** to view a selected agent's blackboard
   contents in real time, **so that** I can inspect the data driving its decisions during
   playtesting.

3. **US-7.3.7.3** -- **As an** engine developer (P-26), **I want** to implement a per-agent trace
   log recording node visits, results, and blackboard mutations per tick, **so that** behavior is
   debuggable in all build configs.

4. **US-7.3.7.4** -- **As an** engine developer (P-26), **I want** the tree visualization overlay to
   compile only in editor builds and be stripped from shipping builds, **so that** release builds
   have zero debug overhead.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-7.3.1 | game designer (P-5) |
| US-7.3.2 | game designer (P-5) |
| US-7.3.3 | game designer (P-5) |
| US-7.3.4 | game designer (P-5) |
| US-7.3.5 | game designer (P-5) |
| US-7.3.6 | game designer (P-5) |
| US-7.3.7 | game designer (P-5) |

1. **US-7.3.1** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.3.1.1 through US-7.3.1.6 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

2. **US-7.3.2** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.3.2.1 through US-7.3.2.5 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

3. **US-7.3.3** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.3.3.1 through US-7.3.3.5 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

4. **US-7.3.4** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.3.4.1 through US-7.3.4.5 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

5. **US-7.3.5** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.3.5.1 through US-7.3.5.5 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

6. **US-7.3.6** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.3.6.1 through US-7.3.6.4 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

7. **US-7.3.7** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.3.7.1 through US-7.3.7.4 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.
