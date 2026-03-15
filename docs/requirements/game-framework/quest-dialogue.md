# R-13.6 — Quest & Dialogue Requirements

## Quest Graph

### R-13.6.1 Quest Graph System

The engine **SHALL** represent quests as directed acyclic graphs of typed objective nodes (kill,
collect, escort, interact, explore, defend, craft) with conditional transition edges, evaluating all
quest state transitions server-authoritatively.

- **Derived from:** [F-13.6.1](../../features/game-framework/quest-dialogue.md)
- **Rationale:** DAG-based quest modeling enables branching story paths and non-linear progression
  while server-authoritative evaluation prevents client-side completion exploits.
- **Verification:** Unit test: construct a quest DAG with 5 branching paths and assert that
  completing objectives in different orders produces the correct terminal states. Integration test:
  verify that a tampered client completion message is rejected by the server.

### R-13.6.2 Quest Prerequisites and Gating

The engine **SHALL** evaluate prerequisite conditions (completed quests, character level, faction
reputation, item possession, achievement unlocks, time-of-day, calendar date) as composable boolean
expressions (AND/OR/NOT) before making a quest available to a player.

- **Derived from:** [F-13.6.2](../../features/game-framework/quest-dialogue.md)
- **Rationale:** Prerequisite gating enforces narrative and progression ordering, and lazy
  evaluation on interaction avoids per-frame cost for all quests.
- **Verification:** Unit test: define prerequisites using nested AND/OR/NOT expressions and assert
  correct availability for players meeting and not meeting each condition. Test edge cases:
  exactly-at-level threshold, reputation boundary values, and time-of-day wraparound.

### R-13.6.3 Quest Tracking and Journal

The engine **SHALL** maintain a per-player quest journal with categorized active, completed, and
failed/abandoned quests, emitting state-change events consumed by UI, map, and minimap systems, and
displaying objective progress counters and waypoint markers for active quests.

- **Derived from:** [F-13.6.3](../../features/game-framework/quest-dialogue.md)
- **Rationale:** A centralized journal with event-driven updates enables decoupled UI and map
  systems to reactively display quest progress without polling.
- **Verification:** Integration test: accept a quest, advance objectives, and verify journal state,
  progress counters, and emitted events at each step. Verify filtering by category (main story,
  side, daily, weekly, event) returns correct subsets.

## Dialogue System

### R-13.6.4 Dialogue Tree System

The engine **SHALL** support branching dialogue trees with conditional branches (checking quest
state, faction reputation, inventory, character class, and prior choices) and side-effect triggers
(quest accept/complete, item grant/remove, reputation change, UI open, cinematic playback) per
dialogue node, with localized text and audio references.

- **Derived from:** [F-13.6.4](../../features/game-framework/quest-dialogue.md)
- **Rationale:** Condition-driven branching and side effects enable dialogues that respond to player
  state and drive game progression without hardcoded scripts.
- **Verification:** Unit test: traverse a dialogue tree with 3 conditional branches and assert
  correct path selection for each condition state. Verify side effects (item grant, reputation
  change) are applied exactly once upon node activation. Test localization by loading two locales
  and asserting distinct text per node.

### R-13.6.5a Conversation Camera and Framing

The engine **SHALL** control camera behavior during NPC conversations, including over-the-shoulder
and close-up shots, player character facing, NPC idle animations, and automatic camera switching in
multi-NPC conversations.

- **Derived from:** [F-13.6.5a](../../features/game-framework/quest-dialogue.md)
- **Rationale:** Cinematic camera framing during conversations creates immersive NPC interactions
  that feel polished and narrative-driven.
- **Verification:** Start a conversation and verify camera framing activates with the configured
  shot type. Start a multi-NPC conversation and verify the camera switches between speakers
  automatically.

### R-13.6.5b Conversation Gameplay State

The engine **SHALL** suppress HUD elements, duck ambient audio, and suppress gameplay inputs during
active NPC conversations, restoring all state on conversation end, with per-conversation
configurability for which elements are suppressed.

- **Derived from:** [F-13.6.5b](../../features/game-framework/quest-dialogue.md)
- **Rationale:** Audio ducking and HUD suppression focus player attention on the dialogue without
  distracting gameplay elements competing for attention.
- **Verification:** Start a conversation and verify HUD suppression, audio ducking, and input
  suppression activate. End the conversation and verify all state restores. Configure a conversation
  that retains minimap and verify selective suppression works.

### R-13.6.5c Conversation Interruption and Resumption

The engine **SHALL** handle conversation interruptions (combat, disconnect, area departure) by
immediately restoring gameplay state and marking the conversation as incomplete with a saved node
index, enabling resumption from the last visited dialogue node.

- **Derived from:** [F-13.6.5c](../../features/game-framework/quest-dialogue.md)
- **Rationale:** Interruption handling prevents softlocks when external events occur during
  dialogue, and resumption avoids forcing players to restart long conversations.
- **Verification:** Simulate a combat interruption mid-dialogue and verify gameplay state restores
  and the conversation is marked incomplete. Re-engage the NPC and verify the conversation resumes
  from the last visited node.

## Rewards and Economy

### R-13.6.6 Quest Rewards and Economy Hooks

The engine **SHALL** distribute quest rewards (XP, currency, items, reputation, achievements,
unlocks) from per-quest reward tables respecting group loot rules, with level-scaling and seasonal
multipliers, using server-authoritative transactional grants to prevent duplication.

- **Derived from:** [F-13.6.6](../../features/game-framework/quest-dialogue.md)
- **Rationale:** Server-authoritative transactional reward grants prevent duplication exploits, and
  table-driven rewards enable designer tuning without code changes.
- **Verification:** Unit test: complete a quest and verify all reward types are granted with correct
  level-scaled values. Simulate concurrent reward claims and assert no duplication. Test group loot
  distribution modes (personal, need/greed, round-robin) with a 5-player group.

### R-13.6.7a Server-Driven World Events

The engine **SHALL** support server-driven world events (invasions, world bosses, territory shifts,
seasonal festivals) that alter zone state for all connected players simultaneously, triggered by
server-side conditions.

- **Derived from:** [F-13.6.7a](../../features/game-framework/quest-dialogue.md)
- **Rationale:** Server-driven world events create a living, dynamic world that responds to
  aggregate player actions and scheduled content.
- **Verification:** Trigger a world event and verify all connected players observe the zone state
  change simultaneously. Verify event activation conditions (time, player count, quest completion
  rate) are evaluated server-side.

### R-13.6.7b Quest Phasing System

The engine **SHALL** support per-player quest phasing that presents different zone versions
(geometry, NPCs, interactables) based on individual quest progress via sub-level streaming, without
affecting other players in the same zone.

- **Derived from:** [F-13.6.7b](../../features/game-framework/quest-dialogue.md)
- **Rationale:** Quest phasing enables personalized world states that reflect each player's story
  progress without zone instancing overhead.
- **Verification:** Advance two players to different quest phases in the same zone and verify each
  sees the correct geometry and NPC set. Verify phase transitions swap sub-levels without affecting
  other players.

## Non-Functional Requirements

### R-13.6.NF1 Maximum Active Quests Per Player

The engine **SHALL** support at least 50 simultaneously active quests per player without degrading
objective tracking, event handling, or journal UI performance.

- **Derived from:** F-13.6.3
- **Rationale:** MMO players commonly track dozens of quests across multiple categories; the system
  must scale without per-frame overhead growing linearly with active quest count.
- **Verification:** Activate 50 quests on a single player. Advance objectives on 10 quests
  simultaneously and verify journal updates, waypoint markers, and event emissions all complete
  within the frame budget. Measure per-frame quest tracking overhead and verify it stays under 0.5
  ms.

### R-13.6.NF2 Dialogue Tree Evaluation Latency

The engine **SHALL** evaluate dialogue tree branch conditions and present the next dialogue node
within 5 ms of player input, ensuring conversations feel responsive without perceptible delay.

- **Derived from:** F-13.6.4, F-13.6.5
- **Rationale:** Perceptible pauses between dialogue selections and response display break
  conversational immersion and frustrate players.
- **Verification:** Create a dialogue tree with 100 nodes and 20 conditional branches checking quest
  state, faction reputation, and inventory. Select a response and measure time from input to the
  next node display. Verify latency stays under 5 ms across all paths.
