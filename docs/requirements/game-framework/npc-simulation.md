# R-13.19 — NPC Simulation Requirements

## Social Simulation

1. **R-13.19.1** — The engine **SHALL** support per-NPC numeric affinity values (configurable axes)
   as ECS components, modified by player actions and saved through the persistence system, with
   configurable relationship tiers gating interactions.
   - **Rationale:** ECS-based affinity integrates with queries and persistence without custom
     systems.
   - **Verification:** Set affinity to 50 on an NPC. Perform a positive action and verify increase.
     Cross a tier threshold and verify new interactions unlock. Save, load, and verify affinity
     persists.

2. **R-13.19.2** — The engine **SHALL** support per-NPC personality traits (configurable axes) and
   dynamic emotional states that evolve based on events and decay toward a personality-determined
   baseline.
   - **Rationale:** Personality-driven emotion creates varied NPC reactions from the same events.
   - **Verification:** Set personality traits on two NPCs. Trigger the same event and verify
     different emotional responses. Wait and verify emotion decays toward baseline.

3. **R-13.19.3** — The engine **SHALL** support NPC deed memory with emotional weight, time-based
   decay, and gossip propagation between NPCs with configurable accuracy degradation per hop.
   - **Rationale:** Gossip propagation creates emergent reputation without centralized tracking.
   - **Verification:** Perform a deed near NPC-A. Verify NPC-A stores a memory. After NPC-A gossips
     to NPC-B, verify NPC-B has a lower-confidence memory. Verify memories decay below threshold and
     are evicted.

4. **R-13.19.4** — The engine **SHALL** aggregate per-NPC affinity across social groups to produce
   emergent reputation scores with configurable thresholds gating group-wide behaviors.
   - **Rationale:** Group reputation enables village/faction-wide reactions from individual
     interactions.
   - **Verification:** Improve affinity with 3 of 5 village NPCs. Verify the village reputation
     score increases. Cross a threshold and verify group-wide behavior change (e.g., shop discount).

## Schedules and Barks

5. **R-13.19.5** — The engine **SHALL** support per-NPC daily schedule tables mapping time-of-day
   ranges to locations and activities, with pathfinding-driven movement between schedule slots and
   schedule-gated interactions.
   - **Rationale:** Schedules create believable NPC routines and gate interactions by time.
   - **Verification:** Define a schedule with 3 slots. Advance the clock and verify NPC navigates to
     each location. Verify interaction is rejected outside the scheduled slot.

6. **R-13.19.6** — The engine **SHALL** support ambient bark pools per NPC type with priority,
   cooldown, and context filters (proximity, combat, weather), displayed as floating text with
   optional audio.
   - **Rationale:** Barks add life without the overhead of full dialogue trees.
   - **Verification:** Approach an NPC and verify a proximity bark triggers. Verify cooldown
     prevents immediate repeat. Verify high-priority barks preempt low-priority ones.

7. **R-13.19.7** — The engine **SHALL** support per-enemy threat tables tracking accumulated hate
   from player actions, with configurable threat modifiers per ability and decay over time.
   - **Rationale:** Threat tables enable AI target selection based on player behavior.
   - **Verification:** Deal damage and verify threat increases on the target's table. Apply a taunt
     and verify threat spike. Wait out of combat range and verify threat decays.

## NPC Social Interactions

8. **R-13.19.8** — The engine **SHALL** support autonomous NPC-to-NPC conversations triggered by
   proximity and relationship, exchanging memory topics, with visible speech bubbles.
   - **Rationale:** NPC conversations create a living world without player involvement.
   - **Verification:** Place two allied NPCs in proximity. Verify conversation triggers with speech
     bubbles. Verify memory entries are updated after conversation.

9. **R-13.19.9** — The engine **SHALL** support per-NPC memory stores of witnessed events with
   emotional weight, reliability score (direct vs. gossip), capacity limits, and lowest-weight
   eviction.
   - **Rationale:** Bounded memory ensures NPC simulation scales without unbounded growth.
   - **Verification:** Fill an NPC's memory to capacity. Add a new high-weight memory and verify the
     lowest-weight entry is evicted. Verify reliability degrades for gossip-sourced memories.

10. **R-13.19.10** — The engine **SHALL** support NPC interaction with environmental objects (doors,
    chairs, crafting stations) using the same interaction system as players, with NPC-specific
    profiles and pathfinding integration.
    - **Rationale:** Shared interaction systems ensure NPCs and players interact with the world
      consistently.
    - **Verification:** Place a locked door with a key. Give an NPC the key and verify the NPC opens
      the door during pathfinding. Verify the door state change is visible to players.

11. **R-13.19.11** — The engine **SHALL** support NPC awareness of quest and story state changes,
    modifying dialogue, behavior, and social topics based on configurable state reactions.
    - **Rationale:** Story-aware NPCs reinforce the impact of player decisions.
    - **Verification:** Complete a quest. Verify the NPC's dialogue changes to reference the
      completion. Verify NPCs discuss the quest event in conversations.
