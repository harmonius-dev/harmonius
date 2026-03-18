# 13.6 — Quest & Dialogue

## Quest Graph

| ID       | Feature                        | Requirements |
|----------|--------------------------------|--------------|
| F-13.6.1 | Quest Graph System             | R-13.6.1     |
| F-13.6.2 | Quest Prerequisites and Gating | R-13.6.2     |
| F-13.6.3 | Quest Tracking and Journal     | R-13.6.3     |

1. **F-13.6.1** — Represents quests as directed acyclic graphs of objectives, prerequisites, and
   branching paths. Each quest node defines an objective type (kill, collect, escort, interact,
   explore, defend, craft), completion criteria, and transition edges guarded by conditions.
   Supports main story arcs, side quests, daily/weekly repeatables, world quests, and seasonal event
   chains. The graph evaluates server-authoritatively to prevent client-side quest completion
   exploits.
   - **Deps:** F-1.5.1 (Typed Event Channels), F-13.1.1 (Game Mode State Machine)
2. **F-13.6.2** — Defines prerequisite conditions that must be satisfied before a quest becomes
   available: completed quests, character level, faction reputation, item possession, achievement
   unlocks, time-of-day, and calendar date (for seasonal events). Prerequisites are evaluated lazily
   when the player interacts with a quest giver or enters a trigger volume. Supports complex boolean
   expressions (AND/OR/NOT) over multiple conditions.
   - **Deps:** F-13.6.1
3. **F-13.6.3** — Maintains a per-player quest journal with active quests, completed quests, and
   failed/abandoned quests. Active quests display objectives with progress counters, waypoint
   markers, and zone indicators on the map and HUD. The journal supports quest categories (main
   story, side, daily, weekly, event), sorting, filtering, and search. Quest state changes emit
   events consumed by the UI, map, and minimap systems.
   - **Deps:** F-13.6.1, F-10.1.1 (UI Widget Framework)

## Dialogue System

| ID        | Feature                                  | Requirements |
|-----------|------------------------------------------|--------------|
| F-13.6.4  | Dialogue Tree System                     | R-13.6.4     |
| F-13.6.5a | Conversation Camera and Framing          | R-13.6.5a    |
| F-13.6.5b | Conversation Gameplay State              | R-13.6.5b    |
| F-13.6.5c | Conversation Interruption and Resumption | R-13.6.5c    |

1. **F-13.6.4** — Authors branching dialogues as tree structures with NPC lines, player response
   options, and conditional branches. Conditions on branches can check quest state, faction
   reputation, inventory contents, character class, or prior dialogue choices. Dialogue nodes can
   trigger side effects: quest accept/complete, item grant/remove, reputation change,
   shop/bank/trainer UI open, or cinematic playback. Supports localized text and audio references
   per dialogue node.
   - **Deps:** F-13.6.1, F-1.4.1 (Binary Serialization)
2. **F-13.6.5a** — Manages camera behavior during NPC conversations: over-the-shoulder and close-up
   shots, player character facing toward the NPC, and NPC idle animations during dialogue. Supports
   multi-NPC conversations where several characters speak in sequence with automatic camera
   switching between speakers.
   - **Deps:** F-13.6.4, F-13.5.2 (Cutscene Camera), F-9.4.7 (Montages)
3. **F-13.6.5b** — Manages gameplay state changes during active NPC conversations: ambient audio
   ducking, HUD element suppression, and gameplay input suppression. State changes apply on
   conversation start and restore on conversation end. Configurable per conversation asset — some
   dialogues suppress HUD fully while others retain minimap and health.
   - **Deps:** F-13.6.4, F-5.1.1 (Audio Engine), F-10.3.1 (HUD Widgets)
4. **F-13.6.5c** — Handles interruption of active NPC conversations when the player is attacked,
   disconnects, or leaves the conversation area. On interruption, gameplay state (audio, HUD, input)
   is restored immediately and the conversation is marked as incomplete with its current node index
   saved. Resumption returns the player to the last visited dialogue node.
   - **Deps:** F-13.6.5a, F-13.6.5b, F-13.3.1 (Save System)

## Rewards and Economy

| ID        | Feature                         | Requirements |
|-----------|---------------------------------|--------------|
| F-13.6.6  | Quest Rewards and Economy Hooks | R-13.6.6     |
| F-13.6.7a | Server-Driven World Events      | R-13.6.7a    |
| F-13.6.7b | Quest Phasing System            | R-13.6.7b    |

1. **F-13.6.6** — Defines per-quest reward tables: experience points, currency, items (fixed or
   choice-of-N), reputation, achievement credit, and unlocks (recipes, mounts, titles). Reward
   distribution respects group loot rules in instanced content (personal loot, need/greed,
   round-robin). Reward tables support level-scaling and seasonal multipliers. All reward grants are
   server-authoritative and transactional to prevent duplication exploits.
   - **Deps:** F-13.6.1, F-8.7.5 (Persistent World State)
2. **F-13.6.7a** — Supports server-driven world events that alter zone state for all players:
   invasion spawns, world boss activations, territory control shifts, and seasonal festivals. Events
   are triggered by server-side conditions (time, player count thresholds, quest completion rates)
   and broadcast zone state changes to all connected clients simultaneously.
   - **Deps:** F-13.6.1, F-8.7.1 (World Sharding)
3. **F-13.6.7b** — Shows different versions of a zone to players at different quest stages (e.g., a
   town before and after destruction). Phasing integrates with the level streaming sub-level system
   to swap geometry, NPCs, and interactables per player's quest progress without affecting other
   players in the same zone. Phase mappings are authored as data assets per quest node.
   - **Deps:** F-13.6.1, F-13.2.4 (Sub-Level Composition)
