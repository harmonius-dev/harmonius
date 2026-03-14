# 7.6 — Perception

## Core Senses

### F-7.6.1 Sight Sense (Cone + Line of Sight)

Detects stimuli within a configurable vision cone defined by range, half-angle, and peripheral
falloff. Performs a line-of-sight raycast against the physics scene to confirm visibility, with an
optional trace channel so glass blocks some NPCs but not others. Supports per-archetype tuning
(eagle-eyed sniper vs. short-sighted merchant).

- **Requirements:** R-7.6.1
- **Dependencies:** None
- **Platform notes:** Raycasts use the server-side physics representation

### F-7.6.2 Hearing Sense (Radius + Attenuation)

Detects auditory stimuli (footsteps, gunfire, speech) within a spherical radius, with intensity
attenuated by distance and an occlusion factor from intervening geometry. Louder stimuli propagate
further and are detected with higher confidence, enabling NPCs to distinguish between a whisper
nearby and an explosion far away.

- **Requirements:** R-7.6.2
- **Dependencies:** None
- **Platform notes:** Occlusion queries use simplified server-side collision, not the audio engine

### F-7.6.3 Damage Sense

Instantly registers the direction and magnitude of incoming damage as a high-priority perception
event, bypassing range and LOS checks. Ensures that an NPC always reacts to being attacked
regardless of whether the attacker is within sight or hearing range.

- **Requirements:** R-7.6.3
- **Dependencies:** None
- **Platform notes:** None

### F-7.6.4 Team & Faction Awareness

Tags each perceived stimulus with the source entity's faction and relationship (hostile, neutral,
friendly). Perception filters allow NPCs to ignore friendly stimuli or escalate hostile ones.
Faction relationships are read from a shared affinity table that gameplay systems can modify at
runtime (reputation changes, betrayals).

- **Requirements:** R-7.6.4
- **Dependencies:** F-7.6.1, F-7.6.2
- **Platform notes:** None

## Stimuli Management

### F-7.6.5 Stimuli Registration & Expiration

Provides a global stimulus registry where gameplay systems broadcast perception events (noise at
position, visual flash, scent trail). Each stimulus carries a type, intensity, location, radius,
and TTL. The perception system queries the registry spatially and expires stale entries
automatically.

- **Requirements:** R-7.6.5
- **Dependencies:** None
- **Platform notes:** None

### F-7.6.6 Sense Aging & Memory Decay

Tracks how long ago each known stimulus was last confirmed. Confidence decays over time: a recently
seen enemy has high certainty while a sound heard 10 seconds ago fades to a vague "last known
position." NPCs with better memory archetypes retain stimuli longer, supporting varied AI
personalities.

- **Requirements:** R-7.6.6
- **Dependencies:** F-7.6.5
- **Platform notes:** None

### F-7.6.7 Custom Senses & Perception Priority

Allows gameplay code to register project-specific senses (smell, tremor sense, magic detection)
through a trait interface. Each sense declares a processing cost, and the perception scheduler
allocates a per-tick budget, evaluating high-priority senses first and deferring low-priority ones
to subsequent ticks when the budget is exhausted.

- **Requirements:** R-7.6.7
- **Dependencies:** F-7.6.5
- **Platform notes:** None
