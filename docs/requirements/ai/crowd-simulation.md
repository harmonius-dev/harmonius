# R-7.7 -- Crowd Simulation User Stories

## US-7.7.1 Flocking Behaviors (Separation, Alignment, Cohesion)

### US-7.7.1.1
As a **designer (P-5)**, I want Reynolds flocking with tunable weights per archetype
so that I can create tight columns, loose herds, or panicked mobs.

### US-7.7.1.2
As a **player (P-23)**, I want flocks and herds to move naturally
so that wildlife and crowds feel alive.

### US-7.7.1.3
As an **engine tester (P-27)**, I want to verify separation maintains minimum pairwise
distance above 0.5m
so that flocking correctness is regression-tested.

---

## US-7.7.2 Flow Field Navigation

### US-7.7.2.1
As an **engine dev (P-26)**, I want Dijkstra-propagated flow fields from goal positions
so that thousands of agents navigate at constant per-agent cost.

### US-7.7.2.2
As a **designer (P-5)**, I want flow fields for mass crowd movement
so that city populations navigate efficiently.

### US-7.7.2.3
As an **engine tester (P-27)**, I want to verify per-agent flow field cost is constant
regardless of agent count
so that O(1) per-agent scaling is regression-tested.

---

## US-7.7.3 Flow Field Streaming & Caching

### US-7.7.3.1
As an **engine dev (P-26)**, I want flow fields tiled and cached by goal/cost version
so that recomputation is avoided when the world is stable.

### US-7.7.3.2
As an **engine dev (P-26)**, I want cache invalidation on dynamic obstacle changes
so that stale flow fields are recomputed.

### US-7.7.3.3
As an **engine tester (P-27)**, I want to verify cached fields are reused for identical
goal/cost requests
so that cache hit behavior is regression-tested.

---

## US-7.7.4 Mass Entity Simulation

### US-7.7.4.1
As an **engine dev (P-26)**, I want lightweight crowd entities with minimal state
so that tens of thousands simulate within bounded CPU.

### US-7.7.4.2
As a **player (P-23)**, I want bustling city crowds with thousands of NPCs
so that the world feels populated and alive.

### US-7.7.4.3
As a **server admin (P-22)**, I want crowd count scaling by platform tier
so that each platform runs within its CPU budget.

### US-7.7.4.4
As an **engine tester (P-27)**, I want to verify 10,000 crowd entities maintain linear
CPU scaling
so that mass simulation scalability is regression-tested.

---

## US-7.7.5 AI Level of Detail (Processing Budget)

### US-7.7.5.1
As a **designer (P-5)**, I want AI LOD tiers based on distance and relevance
so that nearby NPCs get full behavior while distant ones use cheap movement.

### US-7.7.5.2
As an **engine dev (P-26)**, I want a global budget scheduler for AI processing
so that total AI CPU stays within frame budget.

### US-7.7.5.3
As a **server admin (P-22)**, I want configurable LOD tier thresholds per platform
so that mobile and desktop have appropriate AI budgets.

### US-7.7.5.4
As an **engine tester (P-27)**, I want to verify total AI processing stays within the
configured budget
so that budget enforcement is regression-tested.

---

## US-7.7.6 Density Management

### US-7.7.6.1
As a **designer (P-5)**, I want configurable density caps per spatial cell
so that chokepoints do not have unrealistic agent pileups.

### US-7.7.6.2
As a **player (P-23)**, I want crowd density to look realistic at events and chokepoints
so that agent stacking does not break immersion.

### US-7.7.6.3
As an **engine tester (P-27)**, I want to verify cells never exceed the configured
density cap
so that density enforcement is regression-tested.

---

## US-7.7.7 Knowledge Sharing and Event Propagation

### US-7.7.7.1
As a **designer (P-5)**, I want guards to alert nearby allies when they spot an intruder
so that detection creates coordinated response.

### US-7.7.7.2
As a **designer (P-5)**, I want 4 communication types (alert, investigation request,
all-clear, report)
so that AI coordination uses varied message types.

### US-7.7.7.3
As a **player (P-23)**, I want a guard's alert to bring reinforcements
so that detection has escalating consequences.

### US-7.7.7.4
As an **engine tester (P-27)**, I want to verify all allies within communication radius
enter alert state within 2 ticks
so that knowledge propagation speed is regression-tested.

---

## US-7.7.8 Shared Awareness and Synchronized Group Reactions

### US-7.7.8.1
As a **designer (P-5)**, I want alarm propagation with spatial delay (wave effect)
so that group reactions look natural rather than instant.

### US-7.7.8.2
As a **designer (P-5)**, I want 5 reaction patterns (scatter, school turn, stampede,
panic, formation)
so that different creature types react distinctively.

### US-7.7.8.3
As a **player (P-23)**, I want bird flocks to scatter when I approach
so that wildlife reacts visibly to my presence.

### US-7.7.8.4
As a **player (P-23)**, I want groups to reassemble after threats clear
so that wildlife returns to normal behavior.

### US-7.7.8.5
As an **engine tester (P-27)**, I want to verify alarm wave delay propagates from nearest
to farthest member
so that spatial delay ordering is regression-tested.

---

## US-7.7.9 Faction-Based Behavioral Relationships

### US-7.7.9.1
As a **designer (P-5)**, I want 6 relationship types (aggressive through allied)
so that AI behavior spans the full hostility spectrum.

### US-7.7.9.2
As a **designer (P-5)**, I want runtime-modifiable faction relationships
so that quest completion changes NPC behavior.

### US-7.7.9.3
As a **designer (P-5)**, I want individual NPC relationship overrides
so that specific NPCs deviate from faction defaults.

### US-7.7.9.4
As a **player (P-23)**, I want my reputation to visibly change NPC reactions
so that my choices have consequences in the world.

### US-7.7.9.5
As an **engine tester (P-27)**, I want to verify an individual override takes precedence
over the faction default
so that override priority is regression-tested.

---

## US-7.7.10 Threat Table and Aggro Targeting

### US-7.7.10.1
As a **designer (P-5)**, I want threat tables tracking damage, healing, taunts, debuffs,
and proximity
so that combat targeting is based on accumulated hate.

### US-7.7.10.2
As a **designer (P-5)**, I want configurable aggro transfer thresholds (110% melee,
130% ranged)
so that tanks can hold aggro with appropriate effort.

### US-7.7.10.3
As a **designer (P-5)**, I want 4 targeting archetypes (berserker, tactical, protector,
coward)
so that different enemies use different target selection strategies.

### US-7.7.10.4
As a **player (P-23)**, I want tanks to hold aggro reliably when maintaining threat
so that the tank/healer/DPS combat role system works.

### US-7.7.10.5
As an **engine tester (P-27)**, I want to verify aggro does NOT transfer below 110%
threshold
so that aggro transfer precision is regression-tested.

---

## US-7.7.11 Animal Tracking and Hunting Behaviors

### US-7.7.11.1
As a **designer (P-5)**, I want predator AI with stalk, ambush, and chase behaviors
so that wildlife hunting looks realistic.

### US-7.7.11.2
As a **designer (P-5)**, I want pack hunting with driver/ambusher coordination
so that wolf packs feel tactically organized.

### US-7.7.11.3
As a **designer (P-5)**, I want prey AI with flee, herd defense, and stamina management
so that prey animals react realistically to predators.

### US-7.7.11.4
As a **designer (P-5)**, I want configurable hunting success rate per predator-prey pair
so that ecological balance is tunable.

### US-7.7.11.5
As a **player (P-23)**, I want to observe realistic wildlife hunting in the game world
so that the ecosystem feels alive and immersive.

### US-7.7.11.6
As an **engine tester (P-27)**, I want to verify hunting success rate matches the
configured percentage over 100 trials
so that success rate accuracy is regression-tested.
