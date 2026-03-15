# User Stories -- 7.7 Crowd Simulation

## US-7.7.1 Populate Cities with Thousands of Ambient NPCs
**As an** AI designer, **I want** to spawn 10,000+ lightweight crowd NPCs in a city using flow
fields and flocking, **so that** the city feels densely populated without exceeding the server
AI budget.

## US-7.7.2 Scale AI Processing Based on Player Distance
**As a** player, **I want** nearby NPCs to behave with full intelligence while distant NPCs
use simplified movement, **so that** the NPCs I interact with feel responsive while the server
stays within its frame budget.

## US-7.7.3 Guard Alerts Barracks When Spotting an Intruder
**As a** player sneaking through a guarded compound, **I want** the guard who spots me to
alert all nearby guards within communication range, **so that** the stealth challenge feels
realistic with coordinated AI responses rather than each guard acting in isolation.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Guard broadcasts alert within 1 tick of spotting intruder | F-7.7.7 | R-7.7.7 |
| All allied guards within radius enter alert state within 2 ticks | F-7.7.7 | R-7.7.7 |
| Guards outside communication radius remain unaware | F-7.7.7 | R-7.7.7 |
| Shared knowledge decays at same rate as direct perception | F-7.7.7 | R-7.7.7 |

## US-7.7.4 Investigation Request Dispatches Nearest Idle Guard
**As a** level designer, **I want** a guard who hears a suspicious noise to send an
investigation request to the nearest idle ally, **so that** AI teams respond realistically
by dispatching a single investigator rather than the entire patrol.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Investigation request sent to nearest idle allied agent | F-7.7.7 | R-7.7.7 |
| Only one ally responds to the request | F-7.7.7 | R-7.7.7 |
| All-clear broadcast after investigation finds nothing | F-7.7.7 | R-7.7.7 |
| Low-LOD agents receive but defer response until promoted | F-7.7.7, F-7.7.5 | R-7.7.7 |

## US-7.7.5 Bird Flock Scatters When Player Approaches
**As a** player walking through a meadow, **I want** a flock of birds to scatter in a wave
from nearest to farthest as I approach, then regroup at a safe distance, **so that** the
wildlife feels alive and reactive.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Alarm propagates with spatial delay producing visible wave | F-7.7.8 | R-7.7.8 |
| Birds scatter upward in burst pattern | F-7.7.8 | R-7.7.8 |
| Flock regroups at safe distance within 30 seconds | F-7.7.8 | R-7.7.8 |
| Stragglers accelerate to rejoin the flock | F-7.7.8, F-7.7.1 | R-7.7.8 |

## US-7.7.6 Herd Stampedes Away from Predator Threat
**As a** player observing wildlife, **I want** a herd of deer to stampede as a group when a
predator is detected, with members maintaining cohesion, **so that** herd behavior looks
natural and coordinated.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Herd stampede triggers with wave delay from threat source | F-7.7.8 | R-7.7.8 |
| Group momentum maintained during stampede | F-7.7.8, F-7.7.1 | R-7.7.8 |
| Herd reassembles at rally point after threat clears | F-7.7.8 | R-7.7.8 |
| At least 5 group reaction patterns supported | F-7.7.8 | R-7.7.8 |

## US-7.7.7 Reputation Changes Faction Behavior from Hostile to Friendly
**As a** player completing quests for a faction, **I want** NPCs who previously attacked me on
sight to become friendly after my reputation improves, **so that** my actions have visible
consequences on the world's social dynamics.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Hostile faction NPCs attack player on sight | F-7.7.9 | R-7.7.9 |
| Completing quest shifts faction relationship to friendly | F-7.7.9 | R-7.7.9 |
| Same NPCs no longer attack after relationship change | F-7.7.9 | R-7.7.9 |
| Individual NPC override takes precedence over faction default | F-7.7.9 | R-7.7.9 |

## US-7.7.8 Wary Faction Warns Before Attacking
**As a** player entering contested territory, **I want** wary NPCs to warn me before turning
hostile if I continue trespassing, **so that** faction interactions feel nuanced with
escalating tension rather than binary attack/ignore.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| At least 6 relationship types available in faction matrix | F-7.7.9 | R-7.7.9 |
| Wary NPCs issue warning before attacking | F-7.7.9 | R-7.7.9 |
| Relationship matrix modifiable at runtime by gameplay systems | F-7.7.9 | R-7.7.9 |
| Relationship determines engagement rules and communication | F-7.7.9 | R-7.7.9 |

## US-7.7.9 Tank Holds Aggro with Taunt Abilities
**As a** tank player in a dungeon group, **I want** enemies to attack me as long as my threat
stays highest, with taunt abilities generating burst threat to reclaim aggro, **so that** the
tank/healer/DPS combat trinity functions correctly.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Enemy attacks highest-threat target | F-7.7.10 | R-7.7.10 |
| Aggro does not transfer below 110% threshold (melee) | F-7.7.10 | R-7.7.10 |
| Taunt generates instant high-threat burst | F-7.7.10 | R-7.7.10 |
| Threat decays when source moves out of combat range | F-7.7.10 | R-7.7.10 |

## US-7.7.10 Tactical AI Targets Lowest-HP Party Member
**As a** player fighting intelligent enemies, **I want** tactical AI archetypes to prioritize
weak targets while berserkers fixate on the highest damage dealer, **so that** different enemy
types require different strategies to counter.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| At least 4 AI targeting archetypes supported | F-7.7.10 | R-7.7.10 |
| Berserker attacks highest damage dealer | F-7.7.10 | R-7.7.10 |
| Tactical archetype attacks lowest-HP target | F-7.7.10 | R-7.7.10 |
| Protector attacks whoever is hitting allies | F-7.7.10 | R-7.7.10 |

## US-7.7.11 Wolf Pack Coordinates to Hunt Deer
**As a** player observing a wilderness ecosystem, **I want** a wolf pack to stalk downwind,
coordinate drivers and ambushers, and chase prey with stamina management, **so that** wildlife
hunting feels like a realistic predator-prey interaction.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Wolves stalk downwind to avoid scent detection | F-7.7.11 | R-7.7.11 |
| Pack coordinates driver and ambusher roles | F-7.7.11 | R-7.7.11 |
| Chase ends when predator or prey exhausts stamina | F-7.7.11 | R-7.7.11 |
| Hunting success rate matches configured value over 100 trials | F-7.7.11 | R-7.7.11 |

## US-7.7.12 Deer Herd Defends Against Predator with Protective Circle
**As a** player watching wildlife, **I want** a deer herd to form protective circles with
mothers shielding young and large males standing to fight, **so that** prey behavior is as
rich and believable as predator behavior.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Deer detect predator via sight/smell/hearing | F-7.7.11 | R-7.7.11 |
| Herd forms protective circle around young | F-7.7.11, F-7.7.8 | R-7.7.11 |
| Prey flee with stamina-aware and terrain-aware pathfinding | F-7.7.11 | R-7.7.11 |
| Predators track prey using scent trails and footprints | F-7.7.11 | R-7.7.11 |
