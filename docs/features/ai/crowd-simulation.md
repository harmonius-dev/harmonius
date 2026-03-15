# 7.7 — Crowd Simulation

## Flocking

### F-7.7.1 Flocking Behaviors (Separation, Alignment, Cohesion)

Implements Reynolds flocking with three weighted forces: separation (avoid crowding neighbors),
alignment (steer toward average heading), and cohesion (steer toward average position). Weights
are tunable per crowd archetype to produce varied group behaviors — tight military columns, loose
wandering herds, or scattered panicked civilians.

- **Requirements:** R-7.7.1
- **Dependencies:** F-7.2.3
- **Platform notes:** Neighbor query radius reduced on mobile (8 m vs. 16 m desktop) to
  lower per-agent cost. Flock sizes limited by crowd count budget (see F-7.7.4).

## Flow Fields

### F-7.7.2 Flow Field Navigation

Generates a 2D grid-based flow field from a goal position using Dijkstra propagation. Each cell
stores a direction vector pointing along the cheapest path to the goal. Thousands of crowd agents
sample the field directly instead of running individual A* queries, enabling mass movement at
constant per-agent cost.

- **Requirements:** R-7.7.2
- **Dependencies:** F-7.1.1
- **Platform notes:** Mobile uses coarser flow field resolution (2 m cells vs. 1 m on
  desktop) and limits concurrent active flow fields to 4.

### F-7.7.3 Flow Field Streaming & Caching

Tiles flow fields to match the world streaming grid and caches recently computed fields keyed by
(goal cell, cost layer version). Stale fields are invalidated when dynamic obstacles modify the
cost grid. Multiple goals share cached fields when their regions overlap, reducing recomputation
in city districts with many points of interest.

- **Requirements:** R-7.7.3
- **Dependencies:** F-7.7.2, F-7.1.2
- **Platform notes:** Mobile cache holds fewer entries (8 vs. 32 on desktop) to limit
  memory; eviction is more aggressive under memory pressure.

## Mass Simulation

### F-7.7.4 Mass Entity Simulation

Processes crowd NPCs as lightweight entities with minimal per-agent state (position, velocity,
archetype ID, flow field sample). Avoids full behavior tree evaluation by driving movement entirely
from flow fields and flocking forces. Supports tens of thousands of ambient city inhabitants on the
server with a bounded CPU footprint.

- **Requirements:** R-7.7.4
- **Dependencies:** F-7.7.1, F-7.7.2
- **Platform notes:** Crowd count scales by platform: mobile 100, Switch 500, console 2000,
  desktop 10,000+. Mobile uses wider despawn radius to keep active count low.

### F-7.7.5 AI Level of Detail (Processing Budget)

Assigns each AI agent a LOD tier based on distance to the nearest player and gameplay relevance.
High-LOD agents run full behavior trees and per-tick perception; mid-LOD agents tick at reduced
frequency; low-LOD agents use flow-field-only movement. A global budget scheduler distributes
available CPU time across tiers to maintain a stable frame rate.

- **Requirements:** R-7.7.5
- **Dependencies:** F-7.3.1, F-7.7.4
- **Platform notes:** Mobile high-LOD radius is 20 m vs. 60 m on desktop. Mobile runs fewer
  agents at high LOD (8 vs. 64). LOD tier thresholds are platform-configurable.

### F-7.7.6 Density Management

Monitors crowd density per spatial cell and enforces configurable caps to prevent unrealistic
pileups at chokepoints, event locations, and spawn areas. When density exceeds the threshold,
overflow agents are redirected to alternative routes or despawned if they are ambient crowd entities,
maintaining both simulation plausibility and server performance.

- **Requirements:** R-7.7.6
- **Dependencies:** F-7.7.4, F-7.2.1
- **Platform notes:** Mobile enforces lower per-cell density caps (4 vs. 16 on desktop) and
  despawns overflow agents more aggressively to stay within crowd budget.

## Social and Group Behaviors

### F-7.7.7 Knowledge Sharing and Event Propagation

AI agents share perception and investigation events with nearby allies. When one agent detects
a threat, witnesses a crime, or completes an investigation, it broadcasts a knowledge event to
all allied agents within a configurable communication radius. Recipients update their own
perception state with the shared knowledge — a guard who spots an intruder alerts all guards
in the barracks. Knowledge sharing uses the faction affinity system (F-7.6.4) to determine
who receives broadcasts (same faction, allied factions). Shared knowledge decays with the
same memory aging rules as direct perception (F-7.6.6). Communication types: **immediate
alert** (threat detected — all allies enter combat), **investigation request** (suspicious
event — nearest idle ally investigates), **all-clear** (investigation found nothing — allies
stand down), and **report** (deferred information — share at next social interaction). NPCs
at different AI LOD tiers (F-7.7.5) receive knowledge events but may respond with reduced
urgency (low-LOD agents acknowledge but don't change behavior until promoted to high-LOD).

- **Requirements:** R-7.7.7
- **Dependencies:** F-7.6.4 (Faction Awareness), F-7.6.6 (Memory Decay), F-7.6.10
  (Investigation), F-7.7.5 (AI LOD)
- **Platform notes:** Mobile limits broadcast radius to 15 m (vs. 50 m desktop) and caps
  event recipients to 8 per broadcast to bound CPU cost.

### F-7.7.8 Shared Awareness and Synchronized Group Reactions

Groups of animals or NPCs react to stimuli as a collective with synchronized behaviors. When
one member of a flock/herd/school detects a threat, the alarm propagates through the group
with a configurable delay (simulating reaction wave). Reactions: **birds** scatter upward in a
burst then regroup at a safe distance, **fish** schools dart away in coordinated turns with
stragglers catching up, **herds** stampede away from the threat with group momentum,
**civilians** panic and flee toward exits with crowd crush prevention, **guards** form a
defensive line facing the threat. Alarm propagation uses spatial proximity — members farther
from the trigger react later, creating a natural wave effect rather than instant uniform
response. Group cohesion is maintained during flight: individuals that fall too far behind
accelerate to rejoin. After the threat clears, the group reassembles at a rally point or
returns to the pre-alarm formation over a configurable calm-down period.

- **Requirements:** R-7.7.8
- **Dependencies:** F-7.7.1 (Flocking), F-7.6.5 (Stimulus Registry), F-7.2.5 (Formation
  Movement)
- **Platform notes:** Mobile limits group size to 8 members; desktop supports 32+. Alarm
  propagation uses instant-uniform on mobile (skips wave delay) for simplicity.

### F-7.7.9 Faction-Based Behavioral Relationships

AI behavior is determined by the runtime faction relationship between the AI entity and every
other entity it perceives. Relationship types: **aggressive** (attack on sight, pursue if
fleeing, call reinforcements), **hostile** (attack if threatened or if the target enters
claimed territory, but don't pursue far), **wary** (watch closely, warn before attacking,
retreat if outmatched), **neutral** (ignore unless provoked, defend self only), **friendly**
(assist in combat, share resources, trade), **allied** (full cooperation, share knowledge
F-7.7.7, joint formations F-7.8.3). Relationships are stored in a faction affinity matrix
(F-7.6.4) that gameplay systems modify at runtime — completing a quest for faction A may shift
faction B from neutral to hostile. Individual relationship overrides allow specific NPCs to
deviate from faction defaults (a friendly orc in an otherwise hostile faction). Relationship
determines: engagement rules, communication willingness, trade availability, and quest
offering.

- **Requirements:** R-7.7.9
- **Dependencies:** F-7.6.4 (Faction Awareness), F-13.12.5 (Reputation), F-7.3.1
  (Behavior Trees)
- **Platform notes:** Lightweight data lookup; runs identically on all platforms.

### F-7.7.10 Threat Table and Aggro Targeting

Combat AI selects its attack target from a threat table that tracks hate generated by each
attacker. Threat sources: direct damage (1:1 threat per damage point), healing (threat equal
to healing done, divided among all engaged enemies), taunt abilities (instant high-threat
burst), debuffs (configurable threat per debuff type), and proximity (being near the enemy
generates passive threat). The highest-threat target is attacked. Threat modifiers on
abilities (F-13.10.1) allow "low threat" heals and "high threat" taunts. Threat decays over
time when the source is out of combat range. Aggro transfer occurs when a new entity exceeds
the current target's threat by a configurable threshold (typically 110% for melee, 130% for
ranged). The threat table is exposed to the behavior tree (F-7.3.1) for conditional target
switching and to the HUD (F-10.3.4) for tank threat indicators. AI archetypes configure
threat behavior: **berserker** (attack highest damage dealer), **tactical** (attack lowest HP
target), **protector** (attack whoever is hitting allies), **coward** (flee from highest
threat source).

- **Requirements:** R-7.7.10
- **Dependencies:** F-13.10.1 (Ability Definition), F-7.3.1 (Behavior Trees), F-7.6.4
  (Faction Awareness), F-10.3.4 (Nameplates)
- **Platform notes:** Primarily server-side. Mobile clients display threat UI but do not
  compute threat tables locally.

### F-7.7.11 Animal Tracking and Hunting Behaviors

Predator-prey AI behaviors for wildlife simulation. Predators: patrol territory, detect prey
via sight/smell/hearing (F-7.6.1/7.6.8/7.6.2), stalk (approach slowly using cover, staying
downwind for scent-aware prey), ambush (wait at known prey paths or water sources), chase
(pursue with stamina management — give up if prey outruns), and kill (attack and consume,
entering a feeding state that lowers alertness). Pack hunting: coordinated roles where drivers
flush prey toward ambushers (F-7.8.2 flanking). Prey: graze/forage, maintain herd awareness
(F-7.7.8), detect predators via perception, flee (with stamina and terrain-aware pathfinding),
and use defensive behaviors (form protective circles, mothers shield young, large prey stand
and fight). Tracking: predators follow prey using scent trails (F-7.6.8), footprints
(F-7.6.9), and visual evidence (disturbed grass, blood). Hunting success rate is configurable
per predator-prey pair for ecological balance (F-3.6.38 ecosystem simulation).

- **Requirements:** R-7.7.11
- **Dependencies:** F-7.6.8 (Smell), F-7.6.9 (Evidence), F-7.6.11 (Multi-Sense Tracking),
  F-7.7.8 (Shared Awareness), F-7.8.2 (Flanking), F-3.6.38 (Ecosystem Simulation)
- **Platform notes:** Mobile uses simplified hunting: sight-only tracking, no pack
  coordination, no scent/footprint following. Wildlife count limited by F-7.7.4 budget.
