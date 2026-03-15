# User Stories -- NPC Simulation (13.19)

## NPC Relationship and Affinity (F-13.19.1)

## US-13.19.1.1
**As a** player (P-23), **I want** NPCs to track numeric affinity values for friendship, romance,
trust, and fear based on my actions, **so that** relationships evolve from my behavior.

## US-13.19.1.2
**As a** player (P-23), **I want** giving gifts to modify affinity based on NPC-specific preference
tables, **so that** learning NPC tastes is rewarding.

## US-13.19.1.3
**As a** player (P-23), **I want** relationship tiers to unlock new dialogue, quests, and perks,
**so that** deepening a relationship provides gameplay rewards.

## US-13.19.1.4
**As a** designer (P-5), **I want** to configure gift preferences, affinity modifiers, and tier
thresholds per NPC in data tables, **so that** relationship design is data-driven.

## US-13.19.1.5
**As a** story director (P-4), **I want** relationship tiers to gate story branches, **so that**
player relationships drive narrative outcomes.

## US-13.19.1.6
**As a** tester (P-27), **I want** to verify that giving a hated gift reduces affinity by the
configured amount, **so that** preference tables are applied correctly.

## NPC Personality and Emotion (F-13.19.2)

## US-13.19.2.1
**As a** player (P-23), **I want** NPCs to have personality traits that influence their dialogue and
behavior, **so that** each NPC feels unique.

## US-13.19.2.2
**As a** player (P-23), **I want** NPC emotional states (happy, angry, afraid) to affect their
animations and available interactions, **so that** emotions are visible and impactful.

## US-13.19.2.3
**As a** designer (P-5), **I want** to assign personality traits per NPC type and configure
emotional decay rates toward personality baselines, **so that** I can differentiate NPC
temperaments.

## US-13.19.2.4
**As a** story director (P-4), **I want** NPC emotions to affect dialogue tone, **so that**
conversations feel contextually appropriate.

## US-13.19.2.5
**As a** tester (P-27), **I want** to verify that a fear emotion decays toward the personality
baseline over time, **so that** emotional decay works correctly.

## NPC Deed Memory (F-13.19.3a)

## US-13.19.3a.1
**As a** player (P-23), **I want** NPCs to witness my actions within their perception range and
judge them by personality, **so that** my behavior has social consequences.

## US-13.19.3a.2
**As a** player (P-23), **I want** positive deeds to increase affinity and negative deeds to
decrease it, **so that** my actions shape relationships.

## US-13.19.3a.3
**As a** designer (P-5), **I want** to configure deed types, emotional weights, and time-based decay
per deed category, **so that** memory significance is tunable.

## US-13.19.3a.4
**As a** tester (P-27), **I want** to verify that memories below the minimum weight threshold are
evicted, **so that** the eviction policy functions correctly.

## Gossip Propagation (F-13.19.3b)

## US-13.19.3b.1
**As a** player (P-23), **I want** NPCs to share my deed memories as gossip during social
interactions, **so that** my reputation spreads organically.

## US-13.19.3b.2
**As a** player (P-23), **I want** gossip to degrade in accuracy and weight with each retelling,
**so that** rumors are imperfect reflections of truth.

## US-13.19.3b.3
**As a** designer (P-5), **I want** to configure propagation rate per NPC archetype, **so that**
tavern gossips spread faster than hermits.

## US-13.19.3b.4
**As a** tester (P-27), **I want** to verify that gossip accuracy degrades after multiple hops, **so
that** multi-hop degradation works correctly.

## Emergent Reputation (F-13.19.3c)

## US-13.19.3c.1
**As a** player (P-23), **I want** helping one NPC in a village to gradually improve my standing
with the whole village as gossip spreads, **so that** consistent behavior is rewarded
community-wide.

## US-13.19.3c.2
**As a** designer (P-5), **I want** to configure reputation thresholds that gate group-wide
behaviors like shop discounts or hostile reactions, **so that** I can design faction-level
consequences.

## US-13.19.3c.3
**As a** story director (P-4), **I want** emergent reputation to gate quest availability across
factions, **so that** player standing drives story access.

## US-13.19.3c.4
**As a** tester (P-27), **I want** to verify that reputation aggregation across a group reflects the
sum of individual NPC affinities, **so that** the aggregation formula is correct.

## Schedule Data Model (F-13.19.4a)

## US-13.19.4a.1
**As a** designer (P-5), **I want** to define NPC daily schedules as data tables with time-of-day
ranges, locations, activities, and variation rules, **so that** NPC routines are data-driven.

## US-13.19.4a.2
**As a** designer (P-5), **I want** schedules to vary by day of the week and season, **so that** NPC
behavior changes over time.

## US-13.19.4a.3
**As a** story director (P-4), **I want** schedule entries to specify interaction availability
flags, **so that** quest NPCs are only available at appropriate times.

## US-13.19.4a.4
**As a** tester (P-27), **I want** to verify that a schedule authored in the visual editor loads and
drives NPC behavior at the correct times, **so that** the data pipeline is correct.

## Schedule Execution (F-13.19.4b)

## US-13.19.4b.1
**As a** player (P-23), **I want** to see NPCs walk between schedule locations at the correct times,
**so that** the world feels alive with routines.

## US-13.19.4b.2
**As a** player (P-23), **I want** NPCs to play configured arrival animations and idle behaviors at
each location, **so that** schedules feel natural.

## US-13.19.4b.3
**As a** designer (P-5), **I want** NPCs that miss a schedule window to skip to the current slot,
**so that** late NPCs recover gracefully.

## US-13.19.4b.4
**As a** tester (P-27), **I want** to verify that an NPC navigates to the correct location when the
in-game clock crosses a schedule boundary, **so that** schedule transitions trigger.

## Schedule-Gated Interactions (F-13.19.4c)

## US-13.19.4c.1
**As a** player (P-23), **I want** shopkeepers to only sell during work hours, **so that** NPC
availability feels realistic.

## US-13.19.4c.2
**As a** player (P-23), **I want** the UI to tell me why an NPC is unavailable, **so that** I can
plan when to return.

## US-13.19.4c.3
**As a** designer (P-5), **I want** schedule slots to gate specific interactions, **so that** I can
control NPC availability by time of day.

## US-13.19.4c.4
**As a** tester (P-27), **I want** to verify that a sleeping NPC cannot be interacted with for
quests, **so that** schedule gating is enforced.

## Ambient Bark System (F-13.19.5)

## US-13.19.5.1
**As a** player (P-23), **I want** NPCs to emit contextual one-liners based on proximity, events,
and time of day, **so that** the world feels lively.

## US-13.19.5.2
**As a** player (P-23), **I want** barks displayed as floating text bubbles with optional audio,
**so that** I can read or hear NPC comments.

## US-13.19.5.3
**As a** designer (P-5), **I want** to configure bark pools per NPC type with priority, cooldown,
and context filters, **so that** barks feel varied and contextual.

## US-13.19.5.4
**As a** tester (P-27), **I want** to verify that high-priority barks preempt low-priority ones,
**so that** priority ordering is correct.

## Threat and Aggro Table (F-13.19.6)

## US-13.19.6.1
**As a** player (P-23), **I want** enemies to attack the highest-threat target based on damage,
healing, and taunts, **so that** threat management is a viable combat strategy.

## US-13.19.6.2
**As a** designer (P-5), **I want** to configure threat values per ability and threat modifiers like
"low threat" heals, **so that** I can balance tank/healer/DPS roles.

## US-13.19.6.3
**As a** designer (P-5), **I want** threat to decay over time when out of combat range, **so that**
disengaged players drop off the threat table.

## US-13.19.6.4
**As a** story director (P-4), **I want** aggro radius and line-of-sight integrated with the spatial
index, **so that** encounters trigger spatially.

## US-13.19.6.5
**As a** tester (P-27), **I want** to verify that a taunt ability places the caster at the top of
the threat table, **so that** the taunt mechanic works correctly.

## NPC-to-NPC Conversations (F-13.19.7)

## US-13.19.7.1
**As a** player (P-23), **I want** NPCs to autonomously converse with each other during daily
routines, **so that** the social world feels self-sustaining.

## US-13.19.7.2
**As a** player (P-23), **I want** NPC conversations to follow greeting, topic exchange, and
farewell phases, **so that** conversations feel structured.

## US-13.19.7.3
**As a** player (P-23), **I want** to see NPC conversations as overhead speech bubbles, **so that**
I can observe social interactions in the world.

## US-13.19.7.4
**As a** designer (P-5), **I want** to define conversation templates with topic slots, priority
rules, and interruption conditions, **so that** NPC dialogue structure is data-driven.

## US-13.19.7.5
**As a** story director (P-4), **I want** conversation topics to include quest state changes and
faction events, **so that** NPC chatter reflects story progression.

## US-13.19.7.6
**As a** tester (P-27), **I want** to verify that threat warnings are exchanged before gossip topics
in conversations, **so that** topic priority is correct.

## NPC Independent Memory (F-13.19.8)

## US-13.19.8.1
**As a** player (P-23), **I want** NPCs to remember specific events they witnessed with emotional
weight, **so that** past interactions have lasting consequences.

## US-13.19.8.2
**As a** player (P-23), **I want** NPCs to reference specific past events in dialogue, **so that**
conversations feel personal and context-aware.

## US-13.19.8.3
**As a** designer (P-5), **I want** to configure memory capacity, emotional decay curves, and
reliability scores per NPC archetype, **so that** I can balance memory realism.

## US-13.19.8.4
**As a** story director (P-4), **I want** traumatic memories to persist indefinitely while routine
memories fade, **so that** significant events are never forgotten.

## US-13.19.8.5
**As a** tester (P-27), **I want** to verify that filling memory to capacity evicts the
lowest-weight entry, **so that** the eviction policy is correct.

## NPC Environmental Interaction (F-13.19.9)

## US-13.19.9.1
**As a** player (P-23), **I want** NPCs to open doors, use ladders, sit in chairs, and eat at
tables, **so that** NPCs feel like they inhabit the world.

## US-13.19.9.2
**As a** player (P-23), **I want** a guard opening a locked door during patrol to create a new path
I can use, **so that** NPC behavior dynamically changes the gameplay space.

## US-13.19.9.3
**As a** designer (P-5), **I want** to define NPC interaction profiles per interactable object
specifying which archetypes can use it and what items are required, **so that** NPC environmental
interactions are data-driven.

## US-13.19.9.4
**As a** tester (P-27), **I want** to verify that an NPC with a key pathfinds through a locked door
and one without the key pathfinds around it, **so that** navigation cost integration is correct.

## Social-Cue Player Search (F-13.19.10)

## US-13.19.10.1
**As a** player (P-23), **I want** searching NPCs to ask nearby NPCs about my location rather than
knowing it omnisciently, **so that** pursuit feels fair and believable.

## US-13.19.10.2
**As a** player (P-23), **I want** to evade pursuit by staying in areas where no NPC witnessed me,
**so that** social awareness is a stealth mechanic.

## US-13.19.10.3
**As a** designer (P-5), **I want** to configure faction communication range and response delay per
faction type, **so that** search behavior fits each faction's fiction.

## US-13.19.10.4
**As a** story director (P-4), **I want** organized factions to broadcast search queries with
trickle-in responses simulating messenger delay, **so that** pursuit escalates organically.

## US-13.19.10.5
**As a** tester (P-27), **I want** to verify that no NPC moves directly to the player's position
without social or evidence-based information, **so that** omniscient tracking is impossible.

## Quest and Story State Awareness (F-13.19.11)

## US-13.19.11.1
**As a** player (P-23), **I want** NPCs to greet me differently after I complete a quest, **so
that** the world acknowledges my accomplishments.

## US-13.19.11.2
**As a** player (P-23), **I want** merchants to adjust prices and stock after nearby story events,
**so that** the economy reacts to the narrative.

## US-13.19.11.3
**As a** designer (P-5), **I want** to attach NPC Reaction nodes to quest state transitions in the
visual quest editor, **so that** I can orchestrate world reactions without code.

## US-13.19.11.4
**As a** story director (P-4), **I want** major quest completions to shift faction behaviors across
all members, **so that** the game world evolves with the story.

## US-13.19.11.5
**As a** tester (P-27), **I want** to verify that all faction NPCs update behavior within the
configured propagation window after a quest completion, **so that** faction response is reliable.

## Player-Witnessed NPC Social Behaviors (F-13.19.12)

## US-13.19.12.1
**As a** player (P-23), **I want** to overhear NPC conversations that reveal quest hints or hidden
item locations, **so that** paying attention to the world rewards exploration.

## US-13.19.12.2
**As a** player (P-23), **I want** to interrupt conversing NPCs and have them acknowledge me, **so
that** NPC conversations do not block interaction.

## US-13.19.12.3
**As a** player (P-23), **I want** nearby NPCs to visibly react to quest events with cheering,
panic, or arguing, **so that** my actions have visible social impact.

## US-13.19.12.4
**As a** designer (P-5), **I want** to tag conversation topics as eavesdroppable with defined
intelligence content and proximity range, **so that** I control what players learn from listening.

## US-13.19.12.5
**As a** story director (P-4), **I want** NPC routines (eating, working, sleeping) to be visible in
the game world, **so that** the environment feels lived-in.

## US-13.19.12.6
**As a** tester (P-27), **I want** to verify that a stealthed player can eavesdrop from farther away
than a non-stealthed player, **so that** stealth integration affects eavesdrop range correctly.
