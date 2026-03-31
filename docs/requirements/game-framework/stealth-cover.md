# R-13.18 -- Stealth and Cover Requirements

## Visibility Query

1. **R-13.18.1** -- The engine **SHALL** compute a per-frame visibility score for entities based on
   ambient light level, shadow state, movement speed, equipment modifiers, and active ability
   overrides, usable as a detection range multiplier by the perception system (F-7.6.1).
   - **Rationale:** A computed visibility score is the engine primitive for any light-based stealth
     system.
   - **Verification:** Place an entity in shadow and verify reduced visibility score. Activate an
     invisibility ability and verify visibility drops to zero.

2. **R-13.18.2** -- The engine **SHALL** provide a multi-state AI awareness system (unaware,
   suspicious, searching, alerted, lost-target) with hysteresis-based transitions, per-state
   perception sensitivity multipliers, and behavior tree subtree selection per state.
   - **Rationale:** Gradual awareness with hysteresis prevents instant detection from brief glimpses
     and is the engine primitive for stealth AI.
   - **Verification:** Briefly expose an entity to an AI and verify it transitions to suspicious but
     not alerted. Sustain detection for the configured duration and verify transition to alerted.

3. **R-13.18.3** -- The engine **SHALL** propagate noise events with per-action intensity, distance
   attenuation, geometry occlusion, and weapon attachment modifiers through the shared spatial index
   to the perception system (F-7.6.2).
   - **Rationale:** Noise propagation with occlusion is the engine primitive for hearing-based
     detection in stealth gameplay.
   - **Verification:** Fire an unsuppressed weapon and verify AI at the configured range is alerted.
     Place a wall between source and AI and verify attenuated detection.

4. **R-13.18.4** -- The engine **SHALL** provide context-sensitive synchronized two-character
   animations triggered by approach angle and target awareness state, with configurable noise levels
   and lethal/ non-lethal outcomes.
   - **Rationale:** Synchronized takedowns are the engine primitive for stealth melee actions in
     action games.
   - **Verification:** Approach from behind an unaware target and verify the takedown triggers.
     Verify a loud takedown alerts nearby AI within the configured noise radius.

## Cover Volume

1. **R-13.18.5** -- The engine **SHALL** identify cover positions from world geometry with half,
   full, and directional classification; provide snap-to-cover with peek, blind fire, and
   cover-to-cover sprint; and expose the same cover points to AI scoring systems.
   - **Rationale:** Auto-detected directional cover with AI sharing is the engine primitive for
     cover-based combat systems.
   - **Verification:** Verify half cover provides the configured defense bonus from the covered
     direction. Flank the cover and verify the bonus is negated.
