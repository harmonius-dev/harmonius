# R-7.6 -- Perception Requirements

## Core Senses

1. **R-7.6.1** -- The engine **SHALL** detect stimuli within a configurable vision cone with range,
   half-angle, peripheral falloff, and line-of-sight raycast confirmation against configurable trace
   channels.
   - **Rationale:** Vision is the primary AI sense; configurable cones and trace channels enable
     per-archetype visual awareness tuning.
   - **Verification:** Configure a 90-degree cone with 30 m range. Place a target inside the cone
     and verify detection. Place it outside and verify no detection. Block LOS with glass and verify
     channel-specific blocking behavior.

2. **R-7.6.2** -- The engine **SHALL** detect auditory stimuli within a spherical radius with
   distance attenuation and geometry occlusion.
   - **Rationale:** Sound-based perception enables NPCs to respond to events outside line of sight
     with physically plausible falloff.
   - **Verification:** Emit a sound at 10 m and verify detection with expected confidence. Place a
     wall between source and listener and verify reduced confidence from occlusion.

3. **R-7.6.3** -- The engine **SHALL** register incoming damage as a high-priority perception event
   that bypasses range and LOS checks.
   - **Rationale:** An NPC must always react to being attacked regardless of whether the attacker is
     within sight or hearing range.
   - **Verification:** Damage an agent from behind a wall beyond hearing range and verify the damage
     sense fires with direction and magnitude.

4. **R-7.6.4** -- The engine **SHALL** tag perceived stimuli with the source entity's faction and
   relationship, with runtime-modifiable faction affinity.
   - **Rationale:** Faction-aware perception enables NPCs to filter friendly vs. hostile stimuli
     without per-agent configuration.
   - **Verification:** Verify an NPC ignores a friendly-tagged stimulus. Change the faction
     relationship to hostile and verify the same stimulus is now escalated.

## Stimuli Management

1. **R-7.6.5** -- The engine **SHALL** provide a global stimulus registry where gameplay systems
   broadcast perception events with type, intensity, location, radius, and TTL, with automatic
   expiration of stale entries.
   - **Rationale:** A central registry decouples stimulus producers from perception consumers and
     bounds active stimulus memory.
   - **Verification:** Register a stimulus with 5 s TTL. Verify it is queryable for 5 s and absent
     afterward. Verify the registry cap is enforced per platform.

2. **R-7.6.6** -- The engine **SHALL** track time since each stimulus was last confirmed and decay
   confidence over time, with per-archetype retention tuning.
   - **Rationale:** Memory decay enables varied AI personalities and prevents NPCs from acting on
     arbitrarily old information.
   - **Verification:** Confirm a stimulus and verify high confidence. Wait without re-confirmation
     and verify confidence decays. Verify a high-memory archetype retains stimuli longer.

3. **R-7.6.7** -- The engine **SHALL** allow registration of project-specific senses through a trait
   interface, with a per-tick perception budget that evaluates high-priority senses first.
   - **Rationale:** Games require custom senses; budgeted evaluation ensures perception cost stays
     within platform limits.
   - **Verification:** Register a custom sense and verify it is evaluated during perception ticks.
     Exhaust the perception budget and verify low-priority senses are deferred.

## Environmental Awareness

1. **R-7.6.8** -- The engine **SHALL** simulate scent stimuli that propagate with wind, linger in
   enclosed spaces, decay over time, and are blocked by sealed geometry and diluted by rain.
   - **Rationale:** Scent-based tracking enables hunting and stealth gameplay with environmental
     interaction.
   - **Verification:** Emit a scent upwind and verify it reaches a downwind AI. Seal a door and
     verify scent does not pass. Trigger rain and verify accelerated dilution.

2. **R-7.6.9** -- The engine **SHALL** generate environmental evidence entities (footprints, blood,
   broken vegetation) with type, timestamp, source, location, and decay timer, queryable by AI with
   a tracking sense.
   - **Rationale:** Physical evidence enables AI tracking beyond direct perception, adding depth to
     stealth and hunting gameplay.
   - **Verification:** Move an entity across deformable terrain and verify footprint evidence is
     generated. Query evidence with a tracking sense and verify direction, recency, and entity type
     are inferable.

3. **R-7.6.10** -- The engine **SHALL** provide an investigation behavior system that selects search
   patterns based on stimulus type and integrates with the alert state machine.
   - **Rationale:** Sub-threshold stimuli should trigger investigation rather than immediate combat,
     creating tension and stealth gameplay opportunities.
   - **Verification:** Trigger a suspicious sound and verify the AI moves to the origin and scans.
     Trigger a scent trail and verify the AI follows it. Verify investigation transitions the alert
     state from unaware to suspicious.

4. **R-7.6.11** -- The engine **SHALL** support multi-sense tracking that combines sight, hearing,
   scent, and evidence inputs, seamlessly switching between senses as conditions change, with
   confidence decay triggering search patterns.
   - **Rationale:** Realistic tracking requires integrating all available sense data rather than
     relying on a single sense.
   - **Verification:** Track a target visually, then block LOS and verify the tracker switches to
     hearing. Stop all stimuli and verify confidence decays to zero, triggering a search pattern.
