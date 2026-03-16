# User Stories -- NPC Simulation (13.19)

## NPC Relationship and Affinity (F-13.19.1)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.19.1.1 | player (P-23) | **As a** player (P-23), **I want** NPCs to track numeric affinity values for friendship, romance, trust, and fear based on my actions, **so that** relationships evolve from my behavior. |  | F-13.19.1 | R-13.19.1 |
| US-13.19.1.2 | player (P-23) | **As a** player (P-23), **I want** giving gifts to modify affinity based on NPC-specific preference tables, **so that** learning NPC tastes is rewarding. |  | F-13.19.1 | R-13.19.1 |
| US-13.19.1.3 | player (P-23) | **As a** player (P-23), **I want** relationship tiers to unlock new dialogue, quests, and perks, **so that** deepening a relationship provides gameplay rewards. |  | F-13.19.1 | R-13.19.1 |
| US-13.19.1.4 | designer (P-5) | **As a** designer (P-5), **I want** to configure gift preferences, affinity modifiers, and tier thresholds per NPC in data tables, **so that** relationship design is data-driven. |  | F-13.19.1 | R-13.19.1 |
| US-13.19.1.5 | story director (P-4) | **As a** story director (P-4), **I want** relationship tiers to gate story branches, **so that** player relationships drive narrative outcomes. |  | F-13.19.1 | R-13.19.1 |
| US-13.19.1.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that giving a hated gift reduces affinity by the configured amount, **so that** preference tables are applied correctly. |  | F-13.19.1 | R-13.19.1 |
## NPC Personality and Emotion (F-13.19.2)
| US-13.19.2.1 | player (P-23) | **As a** player (P-23), **I want** NPCs to have personality traits that influence their dialogue and behavior, **so that** each NPC feels unique. |  | F-13.19.2 | R-13.19.2 |
| US-13.19.2.2 | player (P-23) | **As a** player (P-23), **I want** NPC emotional states (happy, angry, afraid) to affect their animations and available interactions, **so that** emotions are visible and impactful. |  | F-13.19.2 | R-13.19.2 |
| US-13.19.2.3 | designer (P-5) | **As a** designer (P-5), **I want** to assign personality traits per NPC type and configure emotional decay rates toward personality baselines, **so that** I can differentiate NPC temperaments. |  | F-13.19.2 | R-13.19.2 |
| US-13.19.2.4 | story director (P-4) | **As a** story director (P-4), **I want** NPC emotions to affect dialogue tone, **so that** conversations feel contextually appropriate. |  | F-13.19.2 | R-13.19.2 |
| US-13.19.2.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a fear emotion decays toward the personality baseline over time, **so that** emotional decay works correctly. |  | F-13.19.2 | R-13.19.2 |
## NPC Deed Memory (F-13.19.3a)
| US-13.19.3 | player (P-23) | **As a** player (P-23), **I want** NPCs to witness my actions within their perception range and judge them by personality, **so that** my behavior has social consequences. |  | F-13.19.3 | R-13.19.3 |
| US-13.19.3 | player (P-23) | **As a** player (P-23), **I want** positive deeds to increase affinity and negative deeds to decrease it, **so that** my actions shape relationships. |  | F-13.19.3 | R-13.19.3 |
| US-13.19.3 | designer (P-5) | **As a** designer (P-5), **I want** to configure deed types, emotional weights, and time-based decay per deed category, **so that** memory significance is tunable. |  | F-13.19.3 | R-13.19.3 |
| US-13.19.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that memories below the minimum weight threshold are evicted, **so that** the eviction policy functions correctly. |  | F-13.19.3 | R-13.19.3 |
## Gossip Propagation (F-13.19.3b)
| US-13.19.3 | player (P-23) | **As a** player (P-23), **I want** NPCs to share my deed memories as gossip during social interactions, **so that** my reputation spreads organically. |  | F-13.19.3 | R-13.19.3 |
| US-13.19.3 | player (P-23) | **As a** player (P-23), **I want** gossip to degrade in accuracy and weight with each retelling, **so that** rumors are imperfect reflections of truth. |  | F-13.19.3 | R-13.19.3 |
| US-13.19.3 | designer (P-5) | **As a** designer (P-5), **I want** to configure propagation rate per NPC archetype, **so that** tavern gossips spread faster than hermits. |  | F-13.19.3 | R-13.19.3 |
| US-13.19.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that gossip accuracy degrades after multiple hops, **so that** multi-hop degradation works correctly. |  | F-13.19.3 | R-13.19.3 |
## Emergent Reputation (F-13.19.3c)
| US-13.19.3 | player (P-23) | **As a** player (P-23), **I want** helping one NPC in a village to gradually improve my standing with the whole village as gossip spreads, **so that** consistent behavior is rewarded community-wide. |  | F-13.19.3 | R-13.19.3 |
| US-13.19.3 | designer (P-5) | **As a** designer (P-5), **I want** to configure reputation thresholds that gate group-wide behaviors like shop discounts or hostile reactions, **so that** I can design faction-level consequences. |  | F-13.19.3 | R-13.19.3 |
| US-13.19.3 | story director (P-4) | **As a** story director (P-4), **I want** emergent reputation to gate quest availability across factions, **so that** player standing drives story access. |  | F-13.19.3 | R-13.19.3 |
| US-13.19.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that reputation aggregation across a group reflects the sum of individual NPC affinities, **so that** the aggregation formula is correct. |  | F-13.19.3 | R-13.19.3 |
## Schedule Data Model (F-13.19.4a)
| US-13.19.4 | designer (P-5) | **As a** designer (P-5), **I want** to define NPC daily schedules as data tables with time-of-day ranges, locations, activities, and variation rules, **so that** NPC routines are data-driven. |  | F-13.19.4 | R-13.19.4 |
| US-13.19.4 | designer (P-5) | **As a** designer (P-5), **I want** schedules to vary by day of the week and season, **so that** NPC behavior changes over time. |  | F-13.19.4 | R-13.19.4 |
| US-13.19.4 | story director (P-4) | **As a** story director (P-4), **I want** schedule entries to specify interaction availability flags, **so that** quest NPCs are only available at appropriate times. |  | F-13.19.4 | R-13.19.4 |
| US-13.19.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a schedule authored in the visual editor loads and drives NPC behavior at the correct times, **so that** the data pipeline is correct. |  | F-13.19.4 | R-13.19.4 |
## Schedule Execution (F-13.19.4b)
| US-13.19.4 | player (P-23) | **As a** player (P-23), **I want** to see NPCs walk between schedule locations at the correct times, **so that** the world feels alive with routines. |  | F-13.19.4 | R-13.19.4 |
| US-13.19.4 | player (P-23) | **As a** player (P-23), **I want** NPCs to play configured arrival animations and idle behaviors at each location, **so that** schedules feel natural. |  | F-13.19.4 | R-13.19.4 |
| US-13.19.4 | designer (P-5) | **As a** designer (P-5), **I want** NPCs that miss a schedule window to skip to the current slot, **so that** late NPCs recover gracefully. |  | F-13.19.4 | R-13.19.4 |
| US-13.19.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that an NPC navigates to the correct location when the in-game clock crosses a schedule boundary, **so that** schedule transitions trigger. |  | F-13.19.4 | R-13.19.4 |
## Schedule-Gated Interactions (F-13.19.4c)
| US-13.19.4 | player (P-23) | **As a** player (P-23), **I want** shopkeepers to only sell during work hours, **so that** NPC availability feels realistic. |  | F-13.19.4 | R-13.19.4 |
| US-13.19.4 | player (P-23) | **As a** player (P-23), **I want** the UI to tell me why an NPC is unavailable, **so that** I can plan when to return. |  | F-13.19.4 | R-13.19.4 |
| US-13.19.4 | designer (P-5) | **As a** designer (P-5), **I want** schedule slots to gate specific interactions, **so that** I can control NPC availability by time of day. |  | F-13.19.4 | R-13.19.4 |
| US-13.19.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a sleeping NPC cannot be interacted with for quests, **so that** schedule gating is enforced. |  | F-13.19.4 | R-13.19.4 |
## Ambient Bark System (F-13.19.5)
| US-13.19.5.1 | player (P-23) | **As a** player (P-23), **I want** NPCs to emit contextual one-liners based on proximity, events, and time of day, **so that** the world feels lively. |  | F-13.19.5 | R-13.19.5 |
| US-13.19.5.2 | player (P-23) | **As a** player (P-23), **I want** barks displayed as floating text bubbles with optional audio, **so that** I can read or hear NPC comments. |  | F-13.19.5 | R-13.19.5 |
| US-13.19.5.3 | designer (P-5) | **As a** designer (P-5), **I want** to configure bark pools per NPC type with priority, cooldown, and context filters, **so that** barks feel varied and contextual. |  | F-13.19.5 | R-13.19.5 |
| US-13.19.5.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that high-priority barks preempt low-priority ones, **so that** priority ordering is correct. |  | F-13.19.5 | R-13.19.5 |
## Threat and Aggro Table (F-13.19.6)
| US-13.19.6.1 | player (P-23) | **As a** player (P-23), **I want** enemies to attack the highest-threat target based on damage, healing, and taunts, **so that** threat management is a viable combat strategy. |  | F-13.19.6 | R-13.19.6 |
| US-13.19.6.2 | designer (P-5) | **As a** designer (P-5), **I want** to configure threat values per ability and threat modifiers like "low threat" heals, **so that** I can balance tank/healer/DPS roles. |  | F-13.19.6 | R-13.19.6 |
| US-13.19.6.3 | designer (P-5) | **As a** designer (P-5), **I want** threat to decay over time when out of combat range, **so that** disengaged players drop off the threat table. |  | F-13.19.6 | R-13.19.6 |
| US-13.19.6.4 | story director (P-4) | **As a** story director (P-4), **I want** aggro radius and line-of-sight integrated with the spatial index, **so that** encounters trigger spatially. |  | F-13.19.6 | R-13.19.6 |
| US-13.19.6.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a taunt ability places the caster at the top of the threat table, **so that** the taunt mechanic works correctly. |  | F-13.19.6 | R-13.19.6 |
## NPC-to-NPC Conversations (F-13.19.7)
| US-13.19.7.1 | player (P-23) | **As a** player (P-23), **I want** NPCs to autonomously converse with each other during daily routines, **so that** the social world feels self-sustaining. |  | F-13.19.7 | R-13.19.7 |
| US-13.19.7.2 | player (P-23) | **As a** player (P-23), **I want** NPC conversations to follow greeting, topic exchange, and farewell phases, **so that** conversations feel structured. |  | F-13.19.7 | R-13.19.7 |
| US-13.19.7.3 | player (P-23) | **As a** player (P-23), **I want** to see NPC conversations as overhead speech bubbles, **so that** I can observe social interactions in the world. |  | F-13.19.7 | R-13.19.7 |
| US-13.19.7.4 | designer (P-5) | **As a** designer (P-5), **I want** to define conversation templates with topic slots, priority rules, and interruption conditions, **so that** NPC dialogue structure is data-driven. |  | F-13.19.7 | R-13.19.7 |
| US-13.19.7.5 | story director (P-4) | **As a** story director (P-4), **I want** conversation topics to include quest state changes and faction events, **so that** NPC chatter reflects story progression. |  | F-13.19.7 | R-13.19.7 |
| US-13.19.7.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that threat warnings are exchanged before gossip topics in conversations, **so that** topic priority is correct. |  | F-13.19.7 | R-13.19.7 |
## NPC Independent Memory (F-13.19.8)
| US-13.19.8.1 | player (P-23) | **As a** player (P-23), **I want** NPCs to remember specific events they witnessed with emotional weight, **so that** past interactions have lasting consequences. |  | F-13.19.8 | R-13.19.8 |
| US-13.19.8.2 | player (P-23) | **As a** player (P-23), **I want** NPCs to reference specific past events in dialogue, **so that** conversations feel personal and context-aware. |  | F-13.19.8 | R-13.19.8 |
| US-13.19.8.3 | designer (P-5) | **As a** designer (P-5), **I want** to configure memory capacity, emotional decay curves, and reliability scores per NPC archetype, **so that** I can balance memory realism. |  | F-13.19.8 | R-13.19.8 |
| US-13.19.8.4 | story director (P-4) | **As a** story director (P-4), **I want** traumatic memories to persist indefinitely while routine memories fade, **so that** significant events are never forgotten. |  | F-13.19.8 | R-13.19.8 |
| US-13.19.8.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that filling memory to capacity evicts the lowest-weight entry, **so that** the eviction policy is correct. |  | F-13.19.8 | R-13.19.8 |
## NPC Environmental Interaction (F-13.19.9)
| US-13.19.9.1 | player (P-23) | **As a** player (P-23), **I want** NPCs to open doors, use ladders, sit in chairs, and eat at tables, **so that** NPCs feel like they inhabit the world. |  | F-13.19.9 | R-13.19.9 |
| US-13.19.9.2 | player (P-23) | **As a** player (P-23), **I want** a guard opening a locked door during patrol to create a new path I can use, **so that** NPC behavior dynamically changes the gameplay space. |  | F-13.19.9 | R-13.19.9 |
| US-13.19.9.3 | designer (P-5) | **As a** designer (P-5), **I want** to define NPC interaction profiles per interactable object specifying which archetypes can use it and what items are required, **so that** NPC environmental interactions are data-driven. |  | F-13.19.9 | R-13.19.9 |
| US-13.19.9.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that an NPC with a key pathfinds through a locked door and one without the key pathfinds around it, **so that** navigation cost integration is correct. |  | F-13.19.9 | R-13.19.9 |
## Social-Cue Player Search (F-13.19.10)
| US-13.19.10.1 | player (P-23) | **As a** player (P-23), **I want** searching NPCs to ask nearby NPCs about my location rather than knowing it omnisciently, **so that** pursuit feels fair and believable. |  | F-13.19.10 | R-13.19.10 |
| US-13.19.10.2 | player (P-23) | **As a** player (P-23), **I want** to evade pursuit by staying in areas where no NPC witnessed me, **so that** social awareness is a stealth mechanic. |  | F-13.19.10 | R-13.19.10 |
| US-13.19.10.3 | designer (P-5) | **As a** designer (P-5), **I want** to configure faction communication range and response delay per faction type, **so that** search behavior fits each faction's fiction. |  | F-13.19.10 | R-13.19.10 |
| US-13.19.10.4 | story director (P-4) | **As a** story director (P-4), **I want** organized factions to broadcast search queries with trickle-in responses simulating messenger delay, **so that** pursuit escalates organically. |  | F-13.19.10 | R-13.19.10 |
| US-13.19.10.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that no NPC moves directly to the player's position without social or evidence-based information, **so that** omniscient tracking is impossible. |  | F-13.19.10 | R-13.19.10 |
## Quest and Story State Awareness (F-13.19.11)
| US-13.19.11.1 | player (P-23) | **As a** player (P-23), **I want** NPCs to greet me differently after I complete a quest, **so that** the world acknowledges my accomplishments. |  | F-13.19.11 | R-13.19.11 |
| US-13.19.11.2 | player (P-23) | **As a** player (P-23), **I want** merchants to adjust prices and stock after nearby story events, **so that** the economy reacts to the narrative. |  | F-13.19.11 | R-13.19.11 |
| US-13.19.11.3 | designer (P-5) | **As a** designer (P-5), **I want** to attach NPC Reaction nodes to quest state transitions in the visual quest editor, **so that** I can orchestrate world reactions without code. |  | F-13.19.11 | R-13.19.11 |
| US-13.19.11.4 | story director (P-4) | **As a** story director (P-4), **I want** major quest completions to shift faction behaviors across all members, **so that** the game world evolves with the story. |  | F-13.19.11 | R-13.19.11 |
| US-13.19.11.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that all faction NPCs update behavior within the configured propagation window after a quest completion, **so that** faction response is reliable. |  | F-13.19.11 | R-13.19.11 |
## Player-Witnessed NPC Social Behaviors (F-13.19.12)
| US-13.19.12.1 | player (P-23) | **As a** player (P-23), **I want** to overhear NPC conversations that reveal quest hints or hidden item locations, **so that** paying attention to the world rewards exploration. |  | F-13.19.12 | R-13.19.12 |
| US-13.19.12.2 | player (P-23) | **As a** player (P-23), **I want** to interrupt conversing NPCs and have them acknowledge me, **so that** NPC conversations do not block interaction. |  | F-13.19.12 | R-13.19.12 |
| US-13.19.12.3 | player (P-23) | **As a** player (P-23), **I want** nearby NPCs to visibly react to quest events with cheering, panic, or arguing, **so that** my actions have visible social impact. |  | F-13.19.12 | R-13.19.12 |
| US-13.19.12.4 | designer (P-5) | **As a** designer (P-5), **I want** to tag conversation topics as eavesdroppable with defined intelligence content and proximity range, **so that** I control what players learn from listening. |  | F-13.19.12 | R-13.19.12 |
| US-13.19.12.5 | story director (P-4) | **As a** story director (P-4), **I want** NPC routines (eating, working, sleeping) to be visible in the game world, **so that** the environment feels lived-in. |  | F-13.19.12 | R-13.19.12 |
| US-13.19.12.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a stealthed player can eavesdrop from farther away than a non-stealthed player, **so that** stealth integration affects eavesdrop range correctly. |  | F-13.19.12 | R-13.19.12 |
