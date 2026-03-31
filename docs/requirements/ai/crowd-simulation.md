# R-7.7 -- Crowd Simulation Requirements

## Flocking

1. **R-7.7.1** -- The engine **SHALL** implement Reynolds flocking with configurable separation,
   alignment, and cohesion weights per crowd archetype.
   - **Rationale:** Tunable flocking forces produce varied group behaviors from tight military
     columns to scattered panicked civilians.
   - **Verification:** Configure a tight flock and verify agents maintain close spacing. Configure a
     loose flock and verify wider spread. Verify flocking forces produce visually distinct movement
     for different archetype configurations.

## Flow Fields

1. **R-7.7.2** -- The engine **SHALL** generate 2D grid-based flow fields from a goal position using
   Dijkstra propagation, where each cell stores a direction along the cheapest path.
   - **Rationale:** Flow fields enable mass movement at constant per-agent cost instead of
     individual A* queries.
   - **Verification:** Generate a flow field for a goal position. Verify 1000 agents following the
     field converge on the goal. Verify per-agent cost is constant regardless of agent count.

2. **R-7.7.3** -- The engine **SHALL** tile and cache flow fields keyed by goal cell and cost layer
   version, invalidating stale fields when dynamic obstacles modify the cost grid.
   - **Rationale:** Caching reduces recomputation when multiple goals share overlapping regions;
     invalidation ensures obstacles are respected.
   - **Verification:** Generate two flow fields with overlapping regions and verify cache sharing.
     Modify the cost grid and verify affected fields are invalidated and regenerated.

## Batch Processing

1. **R-7.7.4** -- The engine **SHALL** process crowd NPCs as lightweight entities with minimal
   per-agent state driven by flow fields and flocking forces, avoiding full behavior tree
   evaluation.
   - **Rationale:** Tens of thousands of ambient inhabitants require bounded CPU cost per agent that
     does not scale with behavior tree complexity.
   - **Verification:** Spawn 10,000 crowd agents and verify stable frame rate. Verify crowd agents
     do not evaluate behavior trees. Verify per-agent state consists only of position, velocity,
     archetype ID, and flow field sample.

2. **R-7.7.5** -- The engine **SHALL** assign AI agents to LOD tiers based on distance and gameplay
   relevance, with a global budget scheduler distributing CPU time across tiers.
   - **Rationale:** LOD tiering ensures nearby agents run full AI while distant agents consume
     minimal resources, maintaining stable frame rates.
   - **Verification:** Verify high-LOD agents run full behavior trees. Verify low-LOD agents use
     flow-field-only movement. Verify the budget scheduler maintains target frame rate under load.

3. **R-7.7.6** -- The engine **SHALL** enforce configurable crowd density caps per spatial cell,
   redirecting or despawning overflow agents.
   - **Rationale:** Unbounded density at chokepoints causes unrealistic pileups and degrades
     performance.
   - **Verification:** Spawn agents converging on a single cell. Verify the density cap is enforced.
     Verify overflow agents are redirected or despawned.

## Social and Group Behaviors

1. **R-7.7.7** -- The engine **SHALL** propagate perception and investigation events from an AI
   agent to nearby allies within a configurable communication radius, using the faction affinity
   system to determine recipients.
   - **Rationale:** Knowledge sharing enables coordinated group responses like guard alerts without
     requiring each agent to independently detect every stimulus.
   - **Verification:** Have one guard detect a threat and verify all allied guards within the
     communication radius receive the alert. Verify non-allied agents do not receive it.

2. **R-7.7.8** -- The engine **SHALL** propagate alarm reactions through groups with spatial
   proximity-based delay, producing a wave effect rather than instant uniform response.
   - **Rationale:** Staggered group reactions look natural and provide gameplay opportunities for
     stealth.
   - **Verification:** Trigger an alarm in a flock and verify members react with delay proportional
     to distance from the trigger. Verify the group reassembles after the threat clears.

3. **R-7.7.9** -- The engine **SHALL** determine AI behavior from runtime faction relationships
   stored in an affinity matrix, supporting per-NPC relationship overrides.
   - **Rationale:** Dynamic relationships enable quest-driven faction shifts and individual NPC
     personality without per-agent hardcoding.
   - **Verification:** Set faction A to hostile toward faction B and verify attack-on-sight
     behavior. Override one NPC to friendly and verify it deviates from faction default.

4. **R-7.7.10** -- The engine **SHALL** maintain a per-entity threat table tracking hate from
   damage, healing, taunts, debuffs, and proximity, with configurable aggro transfer thresholds and
   time-based decay.
   - **Rationale:** Threat-based targeting produces readable, tunable combat AI where players can
     influence targeting through gameplay actions.
   - **Verification:** Deal damage and verify threat increases proportionally. Taunt and verify
     immediate aggro transfer. Stop attacking and verify threat decays over time.

5. **R-7.7.11** -- The engine **SHALL** provide predator-prey AI behaviors with stalking, ambush,
   chase, and defensive herd responses, using multi-sense tracking for hunting.
   - **Rationale:** Wildlife simulation requires specialized predator and prey behaviors beyond
     generic combat AI.
   - **Verification:** Verify a predator stalks and chases prey using scent trails. Verify prey
     detects the predator and flees with herd awareness. Verify configurable hunting success rates.
