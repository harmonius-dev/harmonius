# R-13.6 — Quest and Dialogue Requirements

## Objective Graph

1. **R-13.6.1** — The engine **SHALL** represent quests as directed acyclic graphs of objectives
   with branching paths, multiple objective types (kill, collect, escort, interact, explore, defend,
   craft), server-authoritative evaluation, and support for repeatables and event chains.
   - **Rationale:** DAG representation with server authority prevents client-side completion
     exploits.
   - **Verification:** Define a quest with 3 branching objectives. Complete each path and verify
     correct transitions. Attempt completing an objective client-side without server validation and
     verify rejection.

2. **R-13.6.2** — The engine **SHALL** evaluate prerequisite conditions (completed quests, level,
   reputation, items, achievements, time, calendar date) with AND/OR/NOT boolean composition before
   making quests available.
   - **Rationale:** Boolean composition enables complex gating without custom code per quest.
   - **Verification:** Define a quest requiring level 10 AND quest-A complete. Verify unavailable at
     level 5. Verify unavailable at level 10 without quest-A. Verify available when both conditions
     met.

3. **R-13.6.3** — The engine **SHALL** maintain a per-player quest journal with active, completed,
   and failed quests, displaying objectives with progress counters, waypoint markers, categories,
   and search.
   - **Rationale:** A structured journal helps players track complex quest states.
   - **Verification:** Accept a quest and verify it appears in the journal. Complete objectives and
     verify progress counters update. Filter by category and verify correct results.

## Branching Dialogue

4. **R-13.6.4** — The engine **SHALL** support branching dialogue trees with NPC lines, player
   response options, conditional branches (quest state, reputation, inventory, class), and side
   effect triggers (quest accept, item grant, reputation change, UI open).
   - **Rationale:** Conditional branches enable dynamic conversations without custom scripting.
   - **Verification:** Define a dialogue with a branch checking reputation. Set reputation below
     threshold and verify the branch is hidden. Set above and verify the branch appears. Select a
     side-effect branch and verify the item grant executes.

5. **R-13.6.5** — The engine **SHALL** manage conversation state including camera behavior
   (over-the-shoulder, speaker switching), gameplay state changes (audio ducking, HUD/input
   suppression), and interruption handling with resumption from the last visited node.
   - **Rationale:** Conversation state management ensures consistent presentation and recovery from
     interruption.
   - **Verification:** Start a conversation and verify camera switches to over-the-shoulder. Verify
     HUD suppresses. Interrupt (leave area) and verify state restores. Return and verify
     conversation resumes at the last node.

## Rewards and World Events

6. **R-13.6.6** — The engine **SHALL** support per-quest reward tables (XP, currency, items,
   reputation, achievement credit) with level-scaling, seasonal multipliers, and
   server-authoritative transactional distribution.
   - **Rationale:** Transactional rewards prevent duplication; scaling keeps rewards relevant.
   - **Verification:** Complete a quest and verify all reward types granted. Attempt completing
     twice and verify the server rejects the duplicate. Change player level and verify reward
     scales.

7. **R-13.6.7** — The engine **SHALL** support server-driven world events altering zone state for
   all players, and quest phasing showing different zone versions per player's quest progress using
   sub-level composition.
   - **Rationale:** World events create shared moments; phasing enables per-player story
     visualization.
   - **Verification:** Trigger a world event and verify all connected clients see the zone change.
     Set two players to different quest phases and verify each sees the correct zone version.
