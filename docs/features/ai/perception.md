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
- **Platform notes:** Lightweight event-driven check; runs identically on all platforms.

### F-7.6.4 Team & Faction Awareness

Tags each perceived stimulus with the source entity's faction and relationship (hostile, neutral,
friendly). Perception filters allow NPCs to ignore friendly stimuli or escalate hostile ones.
Faction relationships are read from a shared affinity table that gameplay systems can modify at
runtime (reputation changes, betrayals).

- **Requirements:** R-7.6.4
- **Dependencies:** F-7.6.1, F-7.6.2
- **Platform notes:** Lightweight table lookup; runs identically on all platforms.

## Stimuli Management

### F-7.6.5 Stimuli Registration & Expiration

Provides a global stimulus registry where gameplay systems broadcast perception events (noise at
position, visual flash, scent trail). Each stimulus carries a type, intensity, location, radius, and
TTL. The perception system queries the registry spatially and expires stale entries automatically.

- **Requirements:** R-7.6.5
- **Dependencies:** None
- **Platform notes:** Mobile caps active stimuli to 256; desktop to 2048. Mobile uses shorter
  default TTLs to keep the registry compact.

### F-7.6.6 Sense Aging & Memory Decay

Tracks how long ago each known stimulus was last confirmed. Confidence decays over time: a recently
seen enemy has high certainty while a sound heard 10 seconds ago fades to a vague "last known
position." NPCs with better memory archetypes retain stimuli longer, supporting varied AI
personalities.

- **Requirements:** R-7.6.6
- **Dependencies:** F-7.6.5
- **Platform notes:** Mobile uses faster decay rates and shorter maximum memory retention to reduce
  per-agent memory-list size.

### F-7.6.7 Custom Senses & Perception Priority

Allows gameplay code to register project-specific senses (tremor sense, magic detection, thermal
vision) through a trait interface. Each sense declares a processing cost, and the perception
scheduler allocates a per-tick budget, evaluating high-priority senses first and deferring
low-priority ones to subsequent ticks when the budget is exhausted.

- **Requirements:** R-7.6.7
- **Dependencies:** F-7.6.5
- **Platform notes:** Mobile per-tick perception budget is 0.25 ms vs. 1 ms on desktop. Low-priority
  custom senses may tick every few seconds on mobile.

## Environmental Awareness

### F-7.6.8 Smell Sense and Scent Trails

A built-in scent perception system where entities emit scent stimuli that linger in the world and
drift with wind. Each scent source (player, food, blood, smoke, chemicals) registers a scent
stimulus with type, intensity, and decay rate. Scent particles propagate from the source along the
wind field (F-4.7.5) and accumulate in enclosed spaces. AI entities with a `SmellSense` component
detect scents within a configurable radius with intensity-based confidence. Scent trails form as a
series of decaying scent points along a moving entity's path — tracking dogs can follow a trail by
moving toward the strongest adjacent scent point. Scent is blocked by sealed doors and diluted by
rain/water. Trail persistence is configurable per scent type: blood trails last hours, footstep
scent fades in minutes, smoke dissipates in seconds. Scent data is stored in a spatial grid
(separate from the main BVH) optimized for density queries.

- **Requirements:** R-7.6.8
- **Dependencies:** F-7.6.5 (Stimulus Registry), F-4.7.5 (Wind System), F-1.9.4 (Spatial Query)
- **Platform notes:** Scent grid resolution scales with platform tier for performance.

### F-7.6.9 Environmental Evidence and Footprint Detection

AI detects physical evidence left by entities in the world: footprints in deformable surfaces (snow,
mud, sand, wet floors), broken vegetation, disturbed objects, blood drops from injured entities,
shell casings, and opened doors/containers. Each evidence type is an ECS entity with an
`EnvironmentalEvidence` component storing: evidence type, timestamp, source entity (if known),
location, and decay timer. Footprints are generated by the character controller (F-4.1.8) when
moving over deformable terrain materials (identified via physics materials F-4.2.9). Evidence
spawning rate is throttled to maintain performance (one footprint per N steps). Evidence is visible
as decals (F-11.2.5) to both players and AI. AI with a `TrackingSense` component queries nearby
evidence and infers: direction of travel (from footprint orientation), recency (from timestamp),
entity type (from footprint size/shape), and injury state (from blood evidence).

- **Requirements:** R-7.6.9
- **Dependencies:** F-7.6.5 (Stimulus Registry), F-4.1.8 (Character Controller), F-4.2.9 (Physics
  Materials), F-11.2.5 (Footprint Decals)
- **Platform notes:** Evidence entity count is capped per chunk. Mobile uses lower caps.

### F-7.6.10 AI Investigation Behavior

When an AI perceives a stimulus below confirmation threshold (suspicious sound, faint scent, single
footprint) or loses contact with a confirmed target, it enters an investigation behavior. The
investigation system selects a behavior based on stimulus type: **sound** — move to the sound
origin, look around, listen for follow-up sounds. **scent** — follow the scent trail toward
increasing intensity. **footprints** — follow the footprint chain in the direction of travel.
**visual glimpse** — move to the last seen position, scan the area. **disturbed objects** — examine
the object, then search nearby hiding spots. Investigation integrates with the AI alert state
machine (F-13.18.2): unaware → suspicious (investigate) → alerted (if evidence confirms a threat) or
→ unaware (if investigation finds nothing after a timeout). Multiple AI agents coordinate
investigation — the first to investigate a stimulus claims it, others continue patrol unless the
investigator calls for backup. Investigation paths use the navigation system (F-7.1.1) and avoid
revisiting already-searched locations.

- **Requirements:** R-7.6.10
- **Dependencies:** F-7.6.8 (Smell), F-7.6.9 (Evidence), F-7.6.6 (Memory Decay), F-13.18.2 (AI Alert
  States), F-7.1.1 (NavMesh), F-7.3.1 (Behavior Trees)
- **Platform notes:** Mobile uses simplified investigation: skips scent/footprint following, uses
  direct move-to-last-known-position only. Multi-agent coordination disabled.

### F-7.6.11 Multi-Sense Tracking and Pursuit

AI tracks a target using combined input from all available senses, switching between senses as
conditions change. A tracking AI maintains a `TrackingState` with: last confirmed position,
estimated current position (dead reckoning from last known velocity), confidence level, and active
tracking method. Tracking methods: **visual tracking** (maintain line of sight, predict movement
behind cover), **audio tracking** (follow sound of footsteps, estimate position from sound
direction), **scent tracking** (follow scent trail F-7.6.8), **evidence tracking** (follow footprint
chain F-7.6.9), and **predictive tracking** (use knowledge of the map to anticipate where the target
is heading — nearest exit, known hiding spots). The tracker seamlessly transitions between methods:
if the target runs behind a wall, visual tracking switches to audio; if the target stops making
noise, the tracker switches to scent or footprints. Tracking confidence decays over time without
stimulus refresh (F-7.6.6). When confidence reaches zero, the tracker transitions to search patterns
(F-7.8.5). Pack tracking allows multiple AI to share tracking data — one agent's sighting updates
all pack members' estimated position.

- **Requirements:** R-7.6.11
- **Dependencies:** F-7.6.1 (Sight), F-7.6.2 (Hearing), F-7.6.8 (Smell), F-7.6.9 (Evidence), F-7.6.6
  (Memory Decay), F-7.8.5 (Search Patterns)
- **Platform notes:** Mobile limits to sight + hearing tracking only; scent/evidence/ predictive
  tracking disabled. Pack tracking group size capped at 4 on mobile.
