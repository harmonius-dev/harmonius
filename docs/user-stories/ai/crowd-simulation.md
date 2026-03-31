# User Stories -- 7.7 Crowd Simulation

## Flocking

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.7.1.1  | game designer (P-5)     |
| US-7.7.1.2  | game designer (P-5)     |
| US-7.7.1.3  | engine developer (P-26) |

1. **US-7.7.1.1** -- **As a** game designer (P-5), **I want** to tune flocking weights for
   separation, alignment, and cohesion per crowd archetype, **so that** military columns, wandering
   herds, and panicked civilians each move distinctively.

2. **US-7.7.1.2** -- **As a** game designer (P-5), **I want** to control the neighbor query radius
   per archetype, **so that** flock density matches the intended group behavior.

3. **US-7.7.1.3** -- **As an** engine developer (P-26), **I want** to implement Reynolds flocking
   with weighted separation, alignment, and cohesion forces, **so that** crowd agents produce varied
   group movement patterns.

## Flow Fields

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.7.2.1  | game designer (P-5)     |
| US-7.7.2.2  | engine developer (P-26) |
| US-7.7.2.3  | engine developer (P-26) |
| US-7.7.3.1  | game designer (P-5)     |
| US-7.7.3.2  | engine developer (P-26) |

1. **US-7.7.2.1** -- **As a** game designer (P-5), **I want** thousands of crowd agents to navigate
   toward goals by sampling a shared flow field, **so that** mass movement has constant per-agent
   cost instead of individual A* queries.

2. **US-7.7.2.2** -- **As an** engine developer (P-26), **I want** to generate 2D grid-based flow
   fields from a goal position using Dijkstra propagation, **so that** each cell stores a direction
   along the cheapest path.

3. **US-7.7.2.3** -- **As an** engine developer (P-26), **I want** to limit concurrent active flow
   fields and cell resolution per platform tier, **so that** mobile devices stay within memory and
   compute budgets.

4. **US-7.7.3.1** -- **As a** game designer (P-5), **I want** flow fields to invalidate when dynamic
   obstacles modify the cost grid, **so that** crowd navigation responds to world changes.

5. **US-7.7.3.2** -- **As an** engine developer (P-26), **I want** to tile and cache flow fields
   keyed by goal cell and cost layer version, **so that** multiple goals share cached fields and
   reduce recomputation.

## Batch Processing

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.7.4.1  | game designer (P-5)     |
| US-7.7.4.2  | engine developer (P-26) |
| US-7.7.5.1  | game designer (P-5)     |
| US-7.7.5.2  | engine developer (P-26) |
| US-7.7.6.1  | game designer (P-5)     |
| US-7.7.6.2  | engine developer (P-26) |

1. **US-7.7.4.1** -- **As a** game designer (P-5), **I want** tens of thousands of ambient city
   inhabitants to be simulated as lightweight entities, **so that** cities feel populated without
   overwhelming the CPU.

2. **US-7.7.4.2** -- **As an** engine developer (P-26), **I want** crowd NPCs processed with minimal
   per-agent state driven entirely by flow fields and flocking forces, **so that** batch processing
   avoids full behavior tree evaluation.

3. **US-7.7.5.1** -- **As a** game designer (P-5), **I want** AI agents to be assigned LOD tiers
   based on distance and gameplay relevance, **so that** nearby agents run full behavior trees while
   distant agents use simplified movement.

4. **US-7.7.5.2** -- **As an** engine developer (P-26), **I want** a global budget scheduler that
   distributes CPU time across LOD tiers, **so that** frame rate stays stable regardless of active
   agent count.

5. **US-7.7.6.1** -- **As a** game designer (P-5), **I want** configurable crowd density caps per
   spatial cell, **so that** unrealistic pileups at chokepoints and spawn areas are prevented.

6. **US-7.7.6.2** -- **As an** engine developer (P-26), **I want** overflow agents to be redirected
   or despawned when density exceeds the threshold, **so that** both plausibility and performance
   are maintained.

## Social and Group Behaviors

| ID           | Persona                 |
|--------------|-------------------------|
| US-7.7.7.1   | game designer (P-5)     |
| US-7.7.7.2   | engine developer (P-26) |
| US-7.7.8.1   | game designer (P-5)     |
| US-7.7.8.2   | engine developer (P-26) |
| US-7.7.9.1   | game designer (P-5)     |
| US-7.7.9.2   | game developer (P-15)   |
| US-7.7.10.1  | game designer (P-5)     |
| US-7.7.10.2  | engine developer (P-26) |
| US-7.7.11.1  | game designer (P-5)     |
| US-7.7.11.2  | game designer (P-5)     |

1. **US-7.7.7.1** -- **As a** game designer (P-5), **I want** AI agents to share perception events
   with nearby allies, **so that** a guard who spots an intruder alerts all guards in the barracks.

2. **US-7.7.7.2** -- **As an** engine developer (P-26), **I want** knowledge sharing to use the
   faction affinity system to determine broadcast recipients, **so that** only allied agents receive
   shared perception data.

3. **US-7.7.8.1** -- **As a** game designer (P-5), **I want** groups of animals or NPCs to react to
   threats as a collective with a natural wave-like alarm propagation, **so that** flocks scatter,
   herds stampede, and civilians flee with visible group dynamics.

4. **US-7.7.8.2** -- **As an** engine developer (P-26), **I want** alarm propagation to use spatial
   proximity with configurable delay, **so that** members farther from the trigger react later for a
   natural wave effect.

5. **US-7.7.9.1** -- **As a** game designer (P-5), **I want** AI behavior to be determined by
   runtime faction relationships ranging from aggressive to allied, **so that** quest completion and
   betrayals alter NPC behavior.

6. **US-7.7.9.2** -- **As a** game developer (P-15), **I want** individual relationship overrides
   per NPC, **so that** specific NPCs can deviate from their faction defaults.

7. **US-7.7.10.1** -- **As a** game designer (P-5), **I want** combat AI to select attack targets
   from a threat table tracking hate from each attacker, **so that** damage, healing, taunts, and
   proximity all influence targeting.

8. **US-7.7.10.2** -- **As an** engine developer (P-26), **I want** threat to decay over time and
   aggro transfer to require exceeding a configurable threshold, **so that** targeting is stable and
   predictable.

9. **US-7.7.11.1** -- **As a** game designer (P-5), **I want** predator AI to stalk, ambush, and
   chase prey using sight, scent, and tracking evidence, **so that** wildlife hunting feels
   realistic.

10. **US-7.7.11.2** -- **As a** game designer (P-5), **I want** prey AI to maintain herd awareness,
    detect predators, and use defensive behaviors, **so that** the ecological simulation has
    balanced predator-prey dynamics.
