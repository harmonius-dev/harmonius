# R-13.19 -- NPC Simulation Requirements

## R-13.19.1 NPC Relationship and Affinity System

The engine **SHALL** maintain per-NPC numeric affinity values (friendship, romance, trust, fear) as
ECS components, modifying them from player actions, dialogue choices, quest outcomes, and NPC-specific
gift preference tables, and advancing relationship tiers that gate dialogue options, quests, and perks.

- **Derived from:** [F-13.19.1](../../features/game-framework/npc-simulation.md)
- **Rationale:** Relationship tracking with data-driven preference tables enables socially rich
  RPG systems where player choices meaningfully shape NPC interactions and story progression.
- **Verification:** Configure two NPCs with distinct gift preference tables (loved, liked, neutral,
  disliked, hated). Give each NPC loved and hated items; verify affinity shifts match table values
  within tolerance. Advance affinity past tier thresholds and confirm new dialogue options and quests
  unlock at each tier. Save and reload; verify affinity values and tier persist.

## R-13.19.2 NPC Personality and Emotion Model

The engine **SHALL** assign per-NPC personality traits from data tables and compute dynamic emotional
states (happy, angry, afraid, grateful, jealous, sad) that evolve from witnessed events and dialogue,
decay toward personality-determined baselines over time, and influence animations, dialogue tone, and
available interactions.

- **Derived from:** [F-13.19.2](../../features/game-framework/npc-simulation.md)
- **Rationale:** Personality and emotion give NPCs behavioral depth, making interactions feel
  varied and responsive to player actions rather than static and scripted.
- **Verification:** Configure an NPC with high courage and low compassion. Trigger a threatening
  event and verify the NPC's fear emotion stays below that of a low-courage NPC in the same
  scenario. Verify emotion decays to baseline within the configured time window. Confirm the
  animation state machine receives the current emotional state and selects the matching animation.

## R-13.19.3a NPC Deed Memory

The engine **SHALL** record player and NPC actions witnessed within perception range as deed
memories with emotional weight and time-based decay, judging them against personality values
to adjust affinity, and evicting memories when they decay below a minimum weight threshold.

- **Derived from:** [F-13.19.3a](../../features/game-framework/npc-simulation.md)
- **Rationale:** Deed memory gives NPCs persistent awareness of player behavior, enabling
  consequences for actions taken in an NPC's presence.
- **Verification:** Perform a positive deed (rescue) witnessed by NPC-A. Verify NPC-A's
  affinity increases. Wait for memory decay and confirm emotional weight decreases to zero
  within the configured decay period. Verify the memory is evicted after reaching minimum
  weight.

## R-13.19.3b Gossip Propagation Network

The engine **SHALL** propagate deed memories between NPCs during social interactions as gossip
with degrading accuracy and emotional weight per retelling, at a configurable propagation
rate per NPC archetype.

- **Derived from:** [F-13.19.3b](../../features/game-framework/npc-simulation.md)
- **Rationale:** Gossip propagation creates emergent social dynamics where information about
  player behavior spreads organically through the NPC social network.
- **Verification:** Allow NPC-A (who witnessed a deed) to gossip with NPC-B; verify NPC-B's
  affinity increases by a lesser amount. Allow NPC-B to gossip with NPC-C; verify NPC-C's
  affinity increase is further reduced. Verify gossip frequency matches the configured
  archetype rate.

## R-13.19.3c Emergent Reputation Aggregation

The engine **SHALL** aggregate per-NPC affinity values across social groups to produce
emergent reputation scores that gate group-wide behaviors such as shop discounts, hostile
reactions, and quest availability.

- **Derived from:** [F-13.19.3c](../../features/game-framework/npc-simulation.md)
- **Rationale:** Aggregated reputation rewards consistent positive behavior across a
  community and creates meaningful social consequences for player actions.
- **Verification:** Improve affinity with several NPCs in a village through gossip
  propagation. Verify the village-wide reputation score crosses a threshold and unlocks
  a group-wide behavior (e.g., shop discount). Verify negative deeds reduce the aggregate
  score accordingly.

## R-13.19.4a Schedule Data Model

The engine **SHALL** define NPC daily schedules as data tables mapping time-of-day ranges to
locations, activities, arrival animations, idle behaviors, interaction availability flags, and
variation rules by day of the week and season.

- **Derived from:** [F-13.19.4a](../../features/game-framework/npc-simulation.md)
- **Rationale:** A structured data model enables designers to author diverse NPC routines
  through the visual editor without code changes.
- **Verification:** Configure a shopkeeper NPC with a schedule: home (6:00-8:00), shop
  (8:00-17:00), tavern (17:00-20:00), home (20:00-6:00). Verify the schedule data
  serializes and deserializes correctly. Change the day of the week and verify the
  alternate schedule entry activates.

## R-13.19.4b Schedule Execution

The engine **SHALL** drive NPC movement between schedule locations via the pathfinding system
when the in-game clock crosses schedule boundaries, playing configured arrival animations and
entering idle behaviors at each destination.

- **Derived from:** [F-13.19.4b](../../features/game-framework/npc-simulation.md)
- **Rationale:** Executed schedules make the game world feel alive with NPCs that follow
  believable daily routines rather than standing idle at fixed positions.
- **Verification:** Advance the in-game clock and verify the NPC navigates to each location
  at the correct time. Verify arrival animations play on reaching the destination. Verify
  NPCs that miss a transition skip to the current slot.

## R-13.19.4c Schedule-Gated Interactions

The engine **SHALL** gate NPC interactions based on the current schedule slot, exposing
availability status to the dialogue system and UI so players can see why an NPC is
unavailable.

- **Derived from:** [F-13.19.4c](../../features/game-framework/npc-simulation.md)
- **Rationale:** Schedule-gated interactions encourage players to learn NPC routines and
  plan activities around the in-game clock, adding depth to the simulation.
- **Verification:** Attempt to trade with a shopkeeper outside work hours and verify the
  interaction is rejected with a reason displayed. Verify trade is available during
  configured shop hours.

## R-13.19.5 Ambient Bark System

The engine **SHALL** trigger contextual one-liner NPC dialogue (barks) from priority-sorted,
cooldown-gated bark pools filtered by proximity, world events, combat state, time of day, and
weather, displaying barks as floating text bubbles with optional audio without entering a full
conversation or pausing gameplay.

- **Derived from:** [F-13.19.5](../../features/game-framework/npc-simulation.md)
- **Rationale:** Ambient barks give NPCs incidental personality and contextual awareness without
  requiring the overhead of full dialogue interactions.
- **Verification:** Configure a guard NPC with two barks: low-priority weather comment
  (60 s cooldown) and high-priority enemy warning (5 s cooldown). Trigger both conditions
  simultaneously; verify the high-priority bark plays. Trigger the low-priority bark and attempt
  to trigger it again within the cooldown window; verify it is suppressed. Confirm barks render
  as floating text and do not pause the game loop.

## R-13.19.6 Threat and Aggro Table System

The engine **SHALL** maintain per-enemy threat tables accumulating hate from damage, healing, taunts,
debuffs, and proximity, with configurable per-ability threat modifiers and time-based out-of-combat
decay, targeting the highest-threat actor and exposing the threat table to AI behavior trees for
target-switching decisions.

- **Derived from:** [F-13.19.6](../../features/game-framework/npc-simulation.md)
- **Rationale:** Threat tables enable tank/healer/DPS role differentiation in combat by giving
  players explicit control over enemy targeting through ability choice.
- **Verification:** Spawn an enemy and three player units: tank (taunt ability), healer, and DPS.
  Have all three engage; verify the enemy targets the highest-threat unit. Activate the tank's
  taunt; verify the tank becomes the target. Have the healer generate healing threat; verify
  threat distributes among engaged enemies. Disengage the DPS for the configured decay duration;
  verify its threat drops to zero.

## R-13.19.7 NPC-to-NPC Conversation System

NPCs **SHALL** autonomously engage in conversations with other NPCs based on proximity, shared
faction, relationship level, and topic relevance. The system **SHALL** follow a mini-dialogue
template (greeting, 1-3 topic exchanges, farewell) with topic prioritization: threat warnings
first, quest-relevant information second, social gossip third. Conversations **SHALL** be
visible to players as overhead speech bubbles with optional audio barks. Conversation outcomes
**SHALL** write information to both participants' memory stores (R-13.19.8) with source
attribution. Conversations **SHALL** be interruptible by threats, player interaction, or
schedule obligations. Mobile **SHALL** limit simultaneous NPC conversations to 4 (vs 16 on
desktop).

- **Derived from:** [F-13.19.7](../../features/game-framework/npc-simulation.md)
- **Rationale:** Autonomous NPC conversations create emergent social dynamics visible to
  players, making the game world feel alive and enabling organic information flow between NPCs.
- **Verification:** Place two allied NPCs within proximity range. Verify a conversation
  triggers with greeting, at least one topic exchange, and farewell. Trigger a threat during
  conversation and verify it interrupts. Verify the player sees speech bubbles. Check both
  NPCs' memory stores for the exchanged information with correct source attribution. On mobile,
  start 5 simultaneous conversations and verify only 4 proceed.

## R-13.19.8 NPC Independent Memory System

Each NPC **SHALL** maintain a personal memory store capped at 50 entries (20 on mobile) with
lowest-weight eviction. Each memory entry **SHALL** contain: event type, involved entities,
location, timestamp, emotional weight, and reliability score (direct witness: 1.0, trusted
NPC: 0.7, stranger: 0.3, multi-hop rumor: degraded per hop). Memories **SHALL** decay over
time based on emotional weight — traumatic memories persist indefinitely while routine events
fade within game-hours. The memory system **SHALL** integrate with behavior trees for
decision-making, the dialogue system for contextual responses, and the gossip system for
reading and writing shared information. Low-LOD NPCs **SHALL** freeze memory updates until
promoted to high LOD.

- **Derived from:** [F-13.19.8](../../features/game-framework/npc-simulation.md)
- **Rationale:** Independent memory gives each NPC a unique perspective on events, enabling
  contextual dialogue, believable decision-making, and fair information asymmetry between NPCs.
- **Verification:** Have NPC-A directly witness a theft (reliability 1.0). Have NPC-A tell
  NPC-B (reliability 0.7). Verify NPC-B's memory has the correct degraded reliability. Fill
  an NPC's memory to 50 entries, add another, and verify the lowest-weight entry is evicted.
  Wait for a routine memory's decay period and verify it is removed. Verify a traumatic memory
  persists past the same time window. Query a behavior tree node and verify it reads from
  memory. On mobile, verify the cap is 20 entries.

## R-13.19.9 NPC Environmental Interaction

NPCs **SHALL** interact with the same environmental objects as players, including: opening and
closing doors, pulling levers, pressing buttons, sitting in chairs, eating at tables, using
crafting stations, sleeping in beds, and traversing shortcuts (ladders, gates, bridges). Each
interactable object **SHALL** define an NPC interaction profile specifying: eligible NPC
archetypes, required items (keys for locked doors), interaction animation, and navigation
cost. NPCs **SHALL** evaluate environmental shortcuts during pathfinding and use them when
beneficial (e.g., a locked door the NPC has a key for, a lever that opens a shortcut). Door
state changes by NPCs **SHALL** be visible to players and affect gameplay. NPC environmental
interactions **SHALL** trigger animation events and sound events detectable by other NPCs and
players.

- **Derived from:** [F-13.19.9](../../features/game-framework/npc-simulation.md)
- **Rationale:** NPCs that use the same interactive objects as players create a consistent,
  believable world where environmental state is shared between AI and player systems.
- **Verification:** Place a locked door with a key. Assign the key to an NPC. Verify the NPC
  pathfinds through the door, plays the unlock animation, and the door opens visibly. Verify
  a nearby NPC hears the door sound event. Place a lever-operated shortcut on a patrol route
  and verify the NPC uses it when it saves travel time. Verify a player can walk through a
  door opened by an NPC.

## R-13.19.10 Social-Cue Player Search

NPC search behavior **SHALL** use only socially-acquired information and **SHALL NEVER** use
omniscient position knowledge. Search methods **SHALL** include: asking nearby allied or
neutral NPCs for last-known sightings (querying their memory), checking environmental
evidence (footprints, opened doors, disturbed objects), and querying faction network members
within communication range with non-instant response delay. The queried NPC **SHALL** respond
with their last known position and timestamp from memory, or "no" if they have no information.
Social search **SHALL** radiate outward from the last known position. Mobile **SHALL** limit
NPC queries to 4 nearby NPCs (vs 8 on desktop).

- **Derived from:** [F-13.19.10](../../features/game-framework/npc-simulation.md)
- **Rationale:** Social-cue search creates fair stealth gameplay where NPCs feel intelligent
  but beatable — they gather information through believable channels rather than cheating.
- **Verification:** Hide the player and trigger a search. Verify the searcher approaches a
  nearby NPC and asks for sightings. If the queried NPC has a memory of the player, verify
  the searcher investigates that position. If the queried NPC has no memory, verify the
  searcher moves on to the next method. Verify the searcher never moves directly to the
  player's current hidden position. Verify faction broadcast responses arrive with delay,
  not instantly. On mobile, verify only 4 NPCs are queried.

## R-13.19.11 Quest and Story State NPC Awareness

NPCs **SHALL** modify their behavior, dialogue, and social interactions based on quest and
story state transitions. Each NPC **SHALL** have a `StoryAwareness` component listing
reactive quest states. Reactions **SHALL** include: dialogue changes reflecting quest
knowledge, behavior changes (e.g., merchant pricing adjustments), social conversation topics
about quest events, and faction-wide behavior shifts from major quest completions. Story
awareness **SHALL** integrate with the quest phasing system so NPCs in phased zones see the
appropriate quest state. NPC reactions **SHALL** be configurable in the visual quest editor
as "NPC Reaction" nodes on quest state transitions.

- **Derived from:** [F-13.19.11](../../features/game-framework/npc-simulation.md)
- **Rationale:** Quest-aware NPCs reinforce the impact of player choices by reflecting story
  changes in dialogue, behavior, and social conversation throughout the game world.
- **Verification:** Complete a quest and verify a nearby NPC's dialogue changes to reference
  the completed quest. Verify a merchant NPC adjusts pricing in response to a threatening
  quest event. Verify NPCs discuss the quest event in conversations (F-13.19.7). Verify
  faction members across the map react to a major quest completion. Verify an NPC in a phased
  zone sees the correct quest state. Open the visual quest editor and verify "NPC Reaction"
  nodes are attachable to quest state transitions.

## R-13.19.12 Player-Witnessed NPC Social Behaviors

NPC social interactions **SHALL** be visible and audible to nearby players. Players **SHALL**
be able to: overhear NPC conversations as speech bubbles with optional voiced barks, observe
NPCs performing daily schedule activities, interrupt NPC conversations by approaching (NPCs
acknowledge the player), witness NPC reactions to quest events (cheering, panicking, arguing),
and eavesdrop on conversations for actionable intelligence (quest hints, hidden item
locations, NPC schedules). Eavesdropping range **SHALL** be affected by the player's stealth
state. Mobile **SHALL** reduce the number of visible NPC social interactions based on crowd
budget.

- **Derived from:** [F-13.19.12](../../features/game-framework/npc-simulation.md)
- **Rationale:** Visible NPC social behaviors reward player attention and exploration,
  creating a lived-in world where overhearing NPCs provides meaningful gameplay advantages.
- **Verification:** Stand within earshot of two conversing NPCs and verify speech bubbles and
  audio barks are visible/audible. Walk into a conversation and verify NPCs interrupt and
  acknowledge the player. Trigger a quest event near NPCs and verify visible reactions.
  Place an eavesdrop-tagged conversation with a quest hint; approach in stealth and verify
  the hint is revealed. Approach out of stealth from farther away and verify the hint is
  not revealed (range check). On mobile, verify NPC social interactions are capped by
  crowd budget.

## Non-Functional Requirements

### NFR-13.19.1 NPC Simulation Scalability

The NPC simulation system **SHALL** support up to 200 scheduled NPCs in the active area with
relationship tracking, personality evaluation, emotion updates, and bark triggers completing
within a total budget of 4ms per frame. Gossip propagation **SHALL** be amortized across
frames, processing no more than 10 gossip events per frame.

- **Rationale:** Towns and cities may contain hundreds of NPCs. Simulation must scale without
  degrading frame rate in populated areas.
- **Verification:** Spawn 200 NPCs with daily schedules, personality, and emotion. Measure
  total NPC simulation time per frame. Verify it stays under 4ms. Trigger 50 simultaneous
  gossip events and verify they amortize across frames at 10 per frame.

### NFR-13.19.2 Deed Memory Storage Efficiency

Per-NPC deed memory **SHALL** consume no more than 256 bytes per deed entry. Total deed
memory across all NPCs **SHALL NOT** exceed 4 MB at any time, enforced by evicting the
oldest low-weight memories when the limit is reached.

- **Rationale:** Unbounded memory accumulation from deed tracking could exhaust memory in
  long play sessions. Capped storage with eviction prevents memory growth.
- **Verification:** Generate 1,000 deeds across 200 NPCs. Measure total memory consumption.
  Verify it stays under 4 MB. Generate additional deeds beyond the cap and verify the oldest
  entries are evicted.
