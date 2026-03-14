# 13.6 — Quest & Dialogue

## Quest Graph

### F-13.6.1 Quest Graph System

Represents quests as directed acyclic graphs of objectives, prerequisites, and branching paths.
Each quest node defines an objective type (kill, collect, escort, interact, explore, defend,
craft), completion criteria, and transition edges guarded by conditions. Supports main story
arcs, side quests, daily/weekly repeatables, world quests, and seasonal event chains. The graph
evaluates server-authoritatively to prevent client-side quest completion exploits.

- **Requirements:** R-13.6.1
- **Dependencies:** F-1.5.1 (Typed Event Channels), F-13.1.1 (Game Mode State Machine)
- **Platform notes:** None

### F-13.6.2 Quest Prerequisites and Gating

Defines prerequisite conditions that must be satisfied before a quest becomes available: completed
quests, character level, faction reputation, item possession, achievement unlocks, time-of-day,
and calendar date (for seasonal events). Prerequisites are evaluated lazily when the player
interacts with a quest giver or enters a trigger volume. Supports complex boolean expressions
(AND/OR/NOT) over multiple conditions.

- **Requirements:** R-13.6.2
- **Dependencies:** F-13.6.1
- **Platform notes:** None

### F-13.6.3 Quest Tracking and Journal

Maintains a per-player quest journal with active quests, completed quests, and failed/abandoned
quests. Active quests display objectives with progress counters, waypoint markers, and zone
indicators on the map and HUD. The journal supports quest categories (main story, side, daily,
weekly, event), sorting, filtering, and search. Quest state changes emit events consumed by the
UI, map, and minimap systems.

- **Requirements:** R-13.6.3
- **Dependencies:** F-13.6.1, F-10.1.1 (UI Widget Framework)
- **Platform notes:** None

## Dialogue System

### F-13.6.4 Dialogue Tree System

Authors branching dialogues as tree structures with NPC lines, player response options, and
conditional branches. Conditions on branches can check quest state, faction reputation, inventory
contents, character class, or prior dialogue choices. Dialogue nodes can trigger side effects:
quest accept/complete, item grant/remove, reputation change, shop/bank/trainer UI open, or
cinematic playback. Supports localized text and audio references per dialogue node.

- **Requirements:** R-13.6.4
- **Dependencies:** F-13.6.1, F-1.4.1 (Binary Serialization)
- **Platform notes:** None

### F-13.6.5 NPC Conversation System

Manages the runtime state of an active NPC conversation: camera framing (over-the-shoulder or
close-up shots), NPC idle animations, player character facing, ambient audio ducking, and HUD
suppression. Supports multi-NPC conversations where several characters speak in sequence. Handles
interruption gracefully if the player is attacked or disconnects during dialogue, restoring
gameplay state and marking the conversation as incomplete for resumption.

- **Requirements:** R-13.6.5
- **Dependencies:** F-13.6.4, F-13.5.2 (Cutscene Camera), F-9.4.7 (Montages)
- **Platform notes:** None

## Rewards and Economy

### F-13.6.6 Quest Rewards and Economy Hooks

Defines per-quest reward tables: experience points, currency, items (fixed or choice-of-N),
reputation, achievement credit, and unlocks (recipes, mounts, titles). Reward distribution
respects group loot rules in instanced content (personal loot, need/greed, round-robin). Reward
tables support level-scaling and seasonal multipliers. All reward grants are server-authoritative
and transactional to prevent duplication exploits.

- **Requirements:** R-13.6.6
- **Dependencies:** F-13.6.1, F-8.7.5 (Persistent World State)
- **Platform notes:** None

### F-13.6.7 Dynamic World Events and Phased Quests

Supports server-driven world events that alter zone state for all players: invasion spawns, world
boss activations, territory control shifts, and seasonal festivals. Quest phasing shows different
versions of a zone to players at different quest stages (e.g., a town before and after
destruction). Phasing integrates with the level streaming sub-level system to swap geometry,
NPCs, and interactables per player's quest progress without affecting other players in the
same zone.

- **Requirements:** R-13.6.7
- **Dependencies:** F-13.6.1, F-13.2.4 (Sub-Level Composition), F-8.7.1 (World Sharding)
- **Platform notes:** None
