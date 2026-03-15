# 13.19 — NPC Simulation

## Social Simulation

### F-13.19.1 NPC Relationship and Affinity System

Per-NPC relationship tracking with numeric affinity values (friendship, romance, trust, fear) affected by player
actions. Giving gifts modifies affinity based on NPC-specific preference tables (loved, liked, neutral, disliked,
hated items). Dialogue choices, quest outcomes, and witnessed deeds also shift affinity. Relationship tiers
(stranger -> acquaintance -> friend -> close friend -> romantic partner) unlock new dialogue options, quests, perks,
and story branches. Affinity values are ECS components on NPC entities, persisted through the save system.

- **Requirements:** R-13.19.1
- **Dependencies:** F-13.6.4 (Branching Dialogue), F-13.7.1 (Table Schema), F-13.3.1 (Save System)
- **Platform notes:** None

### F-13.19.2 NPC Personality and Emotion Model

Per-NPC personality traits (courage, compassion, honesty, ambition, humor — configurable axes) that influence dialogue
responses, deed judgment, and AI behavior tree decisions. Dynamic emotional states (happy, angry, afraid, grateful,
jealous, sad) evolve based on witnessed events, dialogue, and relationship changes. Emotions decay toward a
personality-determined baseline over time. Emotional state affects NPC animations (F-9.4.1), dialogue tone, and
available interaction options. Personality is assigned per NPC type in data tables; emotion state is runtime ECS data.

- **Requirements:** R-13.19.2
- **Dependencies:** F-13.19.1, F-7.3.1 (Behavior Trees), F-9.4.1 (Animation State Machine)
- **Platform notes:** None

### F-13.19.3a NPC Deed Memory

NPCs witness player and NPC actions within their perception range (F-7.6.1) and judge them
according to personality values. Positive deeds (gifting, rescuing, completing quests) increase
affinity; negative deeds (theft, assault, property destruction) decrease it. Each deed is stored
as a memory entry with emotional weight and time-based decay. Memory entries are evicted when
they decay below a minimum weight threshold.

- **Requirements:** R-13.19.3a
- **Dependencies:** F-13.19.2, F-7.6.1 (Perception - Sight)
- **Platform notes:** None

### F-13.19.3b Gossip Propagation Network

During social interactions between NPCs, deed memories are shared as gossip — causing reputation
to propagate organically through the NPC social network. Gossip degrades in accuracy and
emotional weight with each retelling. The propagation rate is configurable per NPC archetype
(e.g., tavern gossips spread faster than hermits).

- **Requirements:** R-13.19.3b
- **Dependencies:** F-13.19.3a, F-13.19.4b (Schedule Execution)
- **Platform notes:** None

### F-13.19.3c Emergent Reputation Aggregation

Aggregates per-NPC affinity values across a social group (village, guild, faction) to produce
an emergent reputation score. Helping one NPC in a village gradually improves standing with
the whole village as gossip spreads. Reputation thresholds gate group-wide behaviors such as
shop discounts, hostile reactions, and quest availability.

- **Requirements:** R-13.19.3c
- **Dependencies:** F-13.19.3b, F-13.19.1
- **Platform notes:** None

### F-13.19.4a Schedule Data Model

Defines the data model for NPC daily schedules. Each NPC has a schedule table mapping
time-of-day ranges to locations, activities, and variation rules. Schedule entries specify:
destination location, arrival animation, idle behavior, and interaction availability flags.
Schedules vary by day of the week and season. The schedule system integrates with the in-game
calendar (F-13.25.1) for time tracking. Schedules are authored as data assets in the visual
editor.

- **Requirements:** R-13.19.4a
- **Dependencies:** F-13.7.1 (Table Schema), F-13.25.1 (Calendar)
- **Platform notes:** None

### F-13.19.4b Schedule Execution

Drives NPC movement between schedule locations using the pathfinding system (F-7.1.1). When
the in-game clock crosses a schedule boundary, the NPC navigates to the next destination,
plays the configured arrival animation, and enters the idle behavior for that time slot. NPCs
that fail to reach their destination before the next schedule transition skip to the current
slot.

- **Requirements:** R-13.19.4b
- **Dependencies:** F-13.19.4a, F-7.1.1 (NavMesh Pathfinding), F-7.3.1 (Behavior Trees)
- **Platform notes:** Mobile reduces the number of simultaneously scheduled NPCs (20 vs 50+
  on desktop) to limit pathfinding and behavior tree evaluation cost per frame.

### F-13.19.4c Schedule-Gated Interactions

Gates NPC interactions based on the current schedule slot. Shopkeepers only sell during work
hours, quest givers are unavailable while sleeping, and trainers only accept students during
scheduled training periods. Interaction availability is exposed to the dialogue system
(F-13.6.4) and the UI (F-10.1.1) so players see why an NPC is unavailable.

- **Requirements:** R-13.19.4c
- **Dependencies:** F-13.19.4a, F-13.6.4 (Dialogue), F-10.1.1 (Widget Tree)
- **Platform notes:** None

### F-13.19.5 Ambient Bark System

NPCs emit contextual one-liner dialogue (barks) without entering a full conversation tree. Barks trigger on
proximity, world events, combat state, time of day, weather, or random timer. Each NPC type has a bark pool with
priority, cooldown, and context filter per entry — guards bark about the weather at low priority but shout warnings at
high priority when enemies approach. Barks display as floating text bubbles and optionally play audio clips. The bark
system is separate from the dialogue system (F-13.6.4) and does not pause gameplay or require player input.

- **Requirements:** R-13.19.5
- **Dependencies:** F-13.6.4 (Dialogue), F-5.1.1 (Audio Engine)
- **Platform notes:** None

### F-13.19.6 Threat and Aggro Table System

Per-enemy threat tables tracking hate generated by player actions. Damage, healing, taunts, debuffs, and proximity
contribute threat values configured per ability. Enemies attack the highest-threat target. Tanks generate extra threat
via taunt abilities; healers generate threat equal to healing done divided among engaged enemies. Threat decays over
time when out of combat range. Threat modifiers on abilities (F-13.10.1) allow "low threat" heals and "high threat"
taunts. Aggro radius and line-of-sight checks use the shared spatial index (F-1.9.4). The threat table is exposed to
the AI behavior tree (F-7.3.4) for target-switching decisions.

- **Requirements:** R-13.19.6
- **Dependencies:** F-13.10.1 (Ability Definition), F-7.3.1 (Behavior Trees), F-1.9.4 (Spatial Query)
- **Platform notes:** None
