# R-7.6 -- Perception Requirements

## R-7.6.1 Sight Sense (Cone and Line of Sight)

The engine **SHALL** detect stimuli within a configurable vision cone (range, half-angle,
peripheral falloff) and confirm visibility via a line-of-sight raycast against the physics
scene, with per-archetype tuning and optional trace channel filtering.

- **Derived from:** [F-7.6.1](../../features/ai/perception.md)
- **Rationale:** Vision is the primary AI sense; cone-based detection with LOS confirmation
  prevents NPCs from seeing through walls or detecting entities behind them.
- **Verification:** Place a target at the edge of the vision cone half-angle and verify it is
  detected. Move it 1 degree outside the cone and verify it is not detected. Place a wall
  between the agent and a target within the cone and verify the LOS raycast blocks detection.
  Configure two archetypes (range 50 m vs. 10 m) and verify each detects targets only within
  its configured range.

## R-7.6.2 Hearing Sense (Radius and Attenuation)

The engine **SHALL** detect auditory stimuli within a spherical radius with intensity attenuated
by distance and an occlusion factor from intervening geometry, allowing NPCs to distinguish
stimuli by loudness and proximity.

- **Derived from:** [F-7.6.2](../../features/ai/perception.md)
- **Rationale:** Hearing allows NPCs to react to off-screen events (footsteps, gunfire);
  attenuation and occlusion prevent unrealistic omniscient detection.
- **Verification:** Emit a sound stimulus with intensity 1.0 at distance 0 m and verify
  detection confidence is 1.0. Emit the same stimulus at max radius and verify confidence
  is near 0.0. Place a wall between emitter and listener and verify the occlusion factor
  reduces confidence below the unoccluded value at the same distance. Verify a whisper
  (intensity 0.1) is detected at 5 m but not at 20 m.

## R-7.6.3 Damage Sense

The engine **SHALL** instantly register incoming damage direction and magnitude as a high-priority
perception event, bypassing range and line-of-sight checks, ensuring NPCs always react to being
attacked regardless of the attacker's position.

- **Derived from:** [F-7.6.3](../../features/ai/perception.md)
- **Rationale:** An NPC must react to being hit even if the attacker is invisible or behind
  cover; damage is an undeniable signal that requires no sensory confirmation.
- **Verification:** Apply damage to an agent from a source outside its vision cone and hearing
  range. Verify the perception system registers the event within the same tick. Verify the
  event includes the correct damage direction (within 5 degrees) and magnitude (exact match).
  Verify the event is tagged as high priority and is processed before pending sight/hearing
  stimuli.

## R-7.6.4 Team and Faction Awareness

The engine **SHALL** tag each perceived stimulus with the source entity's faction and relationship
(hostile, neutral, friendly), reading relationships from a shared affinity table modifiable at
runtime, and support perception filters that ignore friendly stimuli or escalate hostile ones.

- **Derived from:** [F-7.6.4](../../features/ai/perception.md)
- **Rationale:** NPCs must differentiate friend from foe to avoid attacking allies; runtime-
  modifiable affinities support reputation changes and betrayal scenarios.
- **Verification:** Configure faction A as hostile to faction B and friendly to faction C.
  Verify agent A perceives faction B entities as hostile and faction C entities as friendly.
  Enable a "ignore friendly" filter and verify faction C stimuli are discarded. Modify the
  affinity table at runtime to make faction C hostile and verify subsequent perception events
  reflect the new relationship.

## R-7.6.5 Stimuli Registration and Expiration

The engine **SHALL** provide a global stimulus registry where gameplay systems broadcast
perception events (noise, visual flash, scent) carrying type, intensity, location, radius, and
TTL, with spatial querying and automatic expiration of stale entries.

- **Derived from:** [F-7.6.5](../../features/ai/perception.md)
- **Rationale:** A centralized registry decouples stimulus producers (gameplay) from consumers
  (perception) and ensures stale stimuli are cleaned up automatically.
- **Verification:** Register a stimulus with TTL of 5 seconds. Verify a spatial query at the
  stimulus location returns it. Wait 6 seconds and verify the same query returns no results.
  Register 100 stimuli at different locations and verify a spatial query with a 10 m radius
  returns only stimuli within that radius. Verify registering a stimulus with zero TTL is
  rejected.

## R-7.6.6 Sense Aging and Memory Decay

The engine **SHALL** track the elapsed time since each known stimulus was last confirmed and
decay confidence over time, with per-archetype memory duration controlling how long stimuli
are retained before fading to a last-known-position estimate.

- **Derived from:** [F-7.6.6](../../features/ai/perception.md)
- **Rationale:** NPCs should not have perfect memory; aging simulates realistic uncertainty
  about stale information and supports varied AI personalities (sharp vs. forgetful).
- **Verification:** Detect a stimulus and record its initial confidence (1.0). After 5 seconds
  without reconfirmation, verify confidence has decayed below 0.5. After the archetype's
  memory duration, verify confidence reaches 0.0 and the stimulus is removed. Configure a
  second archetype with 2x memory duration and verify its stimulus persists twice as long.

## R-7.6.7 Custom Senses and Perception Priority

The engine **SHALL** allow registration of project-specific senses (smell, tremor, magic
detection) through a trait interface, with each sense declaring a processing cost, and a
perception scheduler that allocates a per-tick budget evaluating high-priority senses first
and deferring low-priority ones when the budget is exhausted.

- **Derived from:** [F-7.6.7](../../features/ai/perception.md)
- **Rationale:** Different games need different senses; a budget-based scheduler prevents
  custom senses from causing frame-time spikes.
- **Verification:** Register a custom "tremor_sense" via the trait interface and verify it is
  invoked during perception evaluation. Set a per-tick budget of 1 ms. Register 5 custom
  senses with costs summing to 2 ms and verify only the highest-priority senses are evaluated
  within the budget, with remaining senses deferred to the next tick. Verify deferred senses
  are processed in the subsequent tick.

## R-7.6.8 Smell Sense and Scent Trails

The engine **SHALL** provide a built-in scent perception system where entities emit scent
stimuli that propagate along the wind field, accumulate in enclosed spaces, and form
followable trails along a moving entity's path. AI with a `SmellSense` component **SHALL**
detect scents within a configurable radius with intensity-based confidence. Scent **SHALL**
be blocked by sealed doors and diluted by rain/water. Trail persistence **SHALL** be
configurable per scent type (blood: hours, footsteps: minutes, smoke: seconds).

- **Derived from:** [F-7.6.8](../../features/ai/perception.md)
- **Rationale:** Scent is a critical sense for hunting/tracking AI in survival, horror, and
  stealth games. Wind-driven propagation creates emergent gameplay (stay downwind).
- **Verification:** Emit a scent source; verify AI detects it within range. Move the source
  along a path; verify a scent trail forms. Place a sealed door between source and AI; verify
  scent is blocked. Activate rain; verify trail decay rate increases.

## R-7.6.9 Environmental Evidence and Footprint Detection

The engine **SHALL** generate environmental evidence entities (footprints, blood drops, broken
vegetation, disturbed objects) when characters traverse deformable surfaces or interact with
the world. AI with a `TrackingSense` component **SHALL** query nearby evidence and infer
direction of travel, recency, entity type, and injury state. Evidence **SHALL** be visible
as decals to both players and AI. Evidence entity count **SHALL** be capped per chunk.

- **Derived from:** [F-7.6.9](../../features/ai/perception.md)
- **Rationale:** Physical evidence makes tracking feel grounded and gives players agency —
  they can avoid leaving tracks by choosing surfaces carefully.
- **Verification:** Walk a character through snow; verify footprint decals spawn at regular
  intervals. Have AI with TrackingSense query footprints; verify it determines correct
  direction of travel. Injure a character; verify blood drop evidence spawns. Verify
  evidence decays after its configured timer.

## R-7.6.10 AI Investigation Behavior

The engine **SHALL** provide investigation behaviors triggered by below-threshold stimuli or
lost-target events. Investigation type **SHALL** be selected by stimulus type: sound → move
to origin, scent → follow trail, footprints → follow chain, visual → scan last-seen area.
Investigation **SHALL** integrate with the AI alert state machine (R-13.18.2): unaware →
suspicious → alerted or → unaware based on investigation outcome. Multiple AI **SHALL**
coordinate investigation to avoid redundant searches.

- **Derived from:** [F-7.6.10](../../features/ai/perception.md)
- **Rationale:** Investigation bridges the gap between "unaware" and "combat" states, creating
  tension and giving players a chance to evade detection.
- **Verification:** Play a sound near an AI; verify it moves to the sound origin and searches.
  Place a scent trail; verify the AI follows it. Have two AI perceive the same sound; verify
  only one investigates while the other continues patrol.

## R-7.6.11 Multi-Sense Tracking and Pursuit

The engine **SHALL** provide a tracking system that combines input from all available senses
(sight, hearing, smell, evidence) and seamlessly transitions between tracking methods as
conditions change. Tracking confidence **SHALL** decay without stimulus refresh and transition
to search patterns (R-7.8.5) at zero confidence. Pack tracking **SHALL** share sighting data
across all pack members — one agent's confirmation updates all members' estimated position.

- **Derived from:** [F-7.6.11](../../features/ai/perception.md)
- **Rationale:** Multi-sense tracking creates believable, persistent pursuit that players must
  actively counter (break line of sight AND mask scent AND avoid leaving footprints).
- **Verification:** Have AI visually track a target; target runs behind a wall; verify AI
  switches to audio tracking. Target stops moving; verify AI switches to scent/footprint
  tracking. All senses fail; verify AI enters search pattern. Verify pack member sighting
  updates the tracker's estimated position within 1 tick.
