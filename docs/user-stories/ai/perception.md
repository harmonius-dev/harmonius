# User Stories — 7.6 Perception

## F-7.6.1 — Sight Sense (Cone + Line of Sight)

## US-7.6.1.1 Configure Vision Cone Per Agent Archetype
**As a** designer (P-5), **I want to** configure vision cone range, half-angle, and peripheral
falloff per agent archetype, **so that** eagle-eyed snipers see farther than short-sighted
merchants.

## US-7.6.1.2 Set Trace Channel for LOS Checks
**As a** designer (P-5), **I want to** set which trace channel the sight sense uses for
line-of-sight raycasts, **so that** glass blocks some NPCs but not others.

## US-7.6.1.3 Preview Vision Cone in Editor
**As a** designer (P-5), **I want to** preview an agent's vision cone as a debug overlay in
the editor, **so that** I can verify the cone covers the intended area.

## US-7.6.1.4 Sneak Past Guards Using Cover
**As a** player (P-23), **I want** guards to have realistic vision cones with line-of-sight
checks, **so that** I can sneak behind them or hide behind walls and feel rewarded for
positioning.

## US-7.6.1.5 See Guards Detect Me When I Step into Their View
**As a** player (P-23), **I want** guards to detect me when I step into their vision cone,
**so that** sight-based detection feels fair and predictable.

## US-7.6.1.6 Hide Behind Glass That Blocks Guard Vision
**As a** player (P-23), **I want** to hide behind frosted glass that blocks certain NPCs'
line of sight, **so that** environmental stealth mechanics work with trace channels.

## US-7.6.1.7 Implement Cone + Raycast Sight Detection
**As an** engine developer (P-26), **I want to** implement sight detection as a vision cone
check followed by a line-of-sight raycast against the physics scene, **so that** only truly
visible targets are detected.

## US-7.6.1.8 Support Per-Archetype Vision Parameters
**As an** engine developer (P-26), **I want to** support per-archetype tuning of range,
half-angle, and peripheral falloff, **so that** different NPC types have distinct visual
capabilities.

## US-7.6.1.9 Optimize Sight Checks with Frustum Culling
**As an** engine developer (P-26), **I want to** cull targets outside the vision cone before
performing expensive raycasts, **so that** sight checks scale with visible target count.

## US-7.6.1.10 Verify Cone Geometry Is Correct
**As an** engine tester (P-27), **I want to** verify that the vision cone geometry matches
the configured range and half-angle with test targets at boundary positions, **so that**
detection boundaries are precise.

## US-7.6.1.11 Test LOS Blocked by Geometry
**As an** engine tester (P-27), **I want to** verify that targets inside the vision cone but
behind opaque geometry are not detected, **so that** line-of-sight blocking works.

## US-7.6.1.12 Benchmark Sight Checks for 500+ Agents
**As an** engine tester (P-27), **I want to** benchmark sight sense evaluation for 500+
agents simultaneously, **so that** perception scales to server-side NPC counts.

---

## F-7.6.2 — Hearing Sense (Radius + Attenuation)

## US-7.6.2.1 Configure Hearing Radius Per Agent
**As a** designer (P-5), **I want to** configure the hearing radius and attenuation curve per
agent archetype, **so that** keen-eared scouts detect sounds farther than oblivious merchants.

## US-7.6.2.2 Set Occlusion Factor for Geometry
**As a** designer (P-5), **I want to** configure how much intervening geometry attenuates
sound, **so that** walls muffle footsteps realistically.

## US-7.6.2.3 Assign Sound Intensity Per Stimulus Type
**As a** designer (P-5), **I want to** assign intensity values to sound types (whisper,
footstep, gunfire, explosion), **so that** louder sounds propagate farther and are detected
with higher confidence.

## US-7.6.2.4 Distract Guards by Throwing Objects
**As a** player (P-23), **I want** guards to investigate the sound of a thrown object,
**so that** I can create distractions to lure them away from their posts.

## US-7.6.2.5 Hear Enemies Detect My Footsteps
**As a** player (P-23), **I want** enemies to detect my footsteps when I run nearby,
**so that** moving quietly matters for stealth.

## US-7.6.2.6 Use Walls to Muffle My Approach
**As a** player (P-23), **I want** walls between me and a guard to muffle my footsteps,
**so that** environmental geometry affects detection.

## US-7.6.2.7 Implement Spherical Hearing with Attenuation
**As an** engine developer (P-26), **I want to** implement hearing as a spherical range check
with distance attenuation and geometry occlusion, **so that** sound detection is physically
plausible.

## US-7.6.2.8 Use Server-Side Collision for Occlusion
**As an** engine developer (P-26), **I want to** use simplified server-side collision for
sound occlusion queries, **so that** hearing does not depend on the audio engine.

## US-7.6.2.9 Support Intensity-Based Confidence Scoring
**As an** engine developer (P-26), **I want to** map stimulus intensity and distance to a
detection confidence score, **so that** NPCs distinguish between a whisper nearby and an
explosion far away.

## US-7.6.2.10 Verify Sound Attenuation by Distance
**As an** engine tester (P-27), **I want to** verify that sound detection confidence decreases
with distance according to the configured attenuation curve, **so that** distance scaling is
correct.

## US-7.6.2.11 Test Geometry Occlusion Reduces Detection
**As an** engine tester (P-27), **I want to** verify that intervening geometry reduces sound
detection confidence by the configured occlusion factor, **so that** walls block sound.

## US-7.6.2.12 Validate Intensity Differentiates Sound Types
**As an** engine tester (P-27), **I want to** verify that a gunshot is detected at 3x the
range of a footstep, matching configured intensity ratios, **so that** intensity scaling
works.

---

## F-7.6.3 — Damage Sense

## US-7.6.3.1 Configure Damage Sense Priority
**As a** designer (P-5), **I want to** configure damage sense as a high-priority perception
event that bypasses range and LOS, **so that** attacked NPCs always react.

## US-7.6.3.2 Set Damage Direction Reporting
**As a** designer (P-5), **I want** damage sense to report the direction and magnitude of
incoming damage, **so that** NPCs turn toward their attacker.

## US-7.6.3.3 Configure Damage Threshold for Reaction
**As a** designer (P-5), **I want to** set a minimum damage threshold before damage sense
triggers, **so that** trivial chip damage does not break stealth.

## US-7.6.3.4 See Attacked NPC Turn Toward Attacker
**As a** player (P-23), **I want** an attacked NPC to immediately turn toward the direction
of the attack, **so that** damage reaction feels instant and aware.

## US-7.6.3.5 Watch Enemies React Even When Shot from Behind
**As a** player (P-23), **I want** enemies to react to being shot from behind even if they
cannot see me, **so that** damage always provokes a response.

## US-7.6.3.6 See NPCs Remain Oblivious to Insignificant Damage
**As a** player (P-23), **I want** NPCs to ignore very minor environmental damage (e.g., a
pebble), **so that** the damage threshold prevents trivial alerts.

## US-7.6.3.7 Implement Event-Driven Damage Detection
**As an** engine developer (P-26), **I want to** implement damage sense as a lightweight
event-driven check that registers direction and magnitude, **so that** it bypasses the normal
perception pipeline.

## US-7.6.3.8 Bypass Range and LOS for Damage Events
**As an** engine developer (P-26), **I want** damage events to bypass range and line-of-sight
checks, **so that** NPCs always react to being attacked regardless of attacker visibility.

## US-7.6.3.9 Ensure Damage Sense Runs on All Platforms Identically
**As an** engine developer (P-26), **I want** damage sense to be a lightweight event check
with identical behavior on all platforms, **so that** there are no platform-specific edge
cases.

## US-7.6.3.10 Verify Damage Sense Always Fires on Attack
**As an** engine tester (P-27), **I want to** verify that damage sense fires for every damage
event above the threshold regardless of attacker position, **so that** no attacks go
unnoticed.

## US-7.6.3.11 Test Damage Direction Accuracy
**As an** engine tester (P-27), **I want to** verify that the reported damage direction
matches the actual attacker direction within a tolerance, **so that** NPCs turn correctly.

## US-7.6.3.12 Validate Threshold Filters Trivial Damage
**As an** engine tester (P-27), **I want to** verify that damage below the configured
threshold does not trigger damage sense, **so that** the filter works.

---

## F-7.6.4 — Team & Faction Awareness

## US-7.6.4.1 Configure Faction Affinity Table
**As a** designer (P-5), **I want to** configure a faction affinity table defining
relationships (hostile, neutral, friendly) between all factions, **so that** NPC perception
filters stimuli by faction.

## US-7.6.4.2 Set Per-NPC Faction Overrides
**As a** designer (P-5), **I want to** override individual NPC faction relationships,
**so that** a friendly NPC in a hostile faction behaves differently from its group.

## US-7.6.4.3 Modify Faction Relations at Runtime
**As a** designer (P-5), **I want to** modify faction relationships at runtime via gameplay
events (reputation changes, betrayals), **so that** the world reacts to player actions.

## US-7.6.4.4 See Guards Ignore Friendly NPCs
**As a** player (P-23), **I want** guards to ignore friendly faction NPCs walking past them,
**so that** perception correctly filters by affinity.

## US-7.6.4.5 Watch Formerly Friendly NPCs Turn Hostile
**As a** player (P-23), **I want** NPCs to turn hostile after a reputation change, attacking
me where they previously ignored me, **so that** faction dynamics feel responsive.

## US-7.6.4.6 See Hostile NPCs Prioritize Threats Over Neutrals
**As a** player (P-23), **I want** hostile NPCs to prioritize hostile targets over neutral
ones when both are visible, **so that** faction awareness drives targeting.

## US-7.6.4.7 Implement Faction-Tagged Stimuli Filtering
**As an** engine developer (P-26), **I want to** tag each perceived stimulus with the source
entity's faction and relationship, **so that** perception filters can ignore or escalate
based on affinity.

## US-7.6.4.8 Read Faction Relations from Shared Affinity Table
**As an** engine developer (P-26), **I want to** read faction relations from a shared affinity
table that gameplay systems can modify at runtime, **so that** relationships are centralized
and dynamic.

## US-7.6.4.9 Support Runtime Affinity Table Modification
**As an** engine developer (P-26), **I want** gameplay systems to modify the affinity table at
runtime (reputation, betrayals), **so that** faction relationships evolve during gameplay.

## US-7.6.4.10 Verify Friendly Stimuli Are Filtered
**As an** engine tester (P-27), **I want to** verify that perception filters correctly ignore
friendly stimuli when configured to do so, **so that** allies do not trigger alert states.

## US-7.6.4.11 Test Runtime Affinity Change Propagation
**As an** engine tester (P-27), **I want to** verify that changing a faction relationship at
runtime immediately affects perception filtering for all agents of that faction, **so that**
changes propagate.

## US-7.6.4.12 Validate Individual Override Precedence
**As an** engine tester (P-27), **I want to** verify that a per-NPC faction override takes
precedence over the faction default, **so that** individual overrides work.

---

## F-7.6.5 — Stimuli Registration & Expiration

## US-7.6.5.1 Register Perception Events from Gameplay Systems
**As a** designer (P-5), **I want** gameplay systems to broadcast perception events (noise at
position, visual flash, scent trail) to a global stimulus registry, **so that** AI can detect
them.

## US-7.6.5.2 Configure Stimulus TTL and Radius
**As a** designer (P-5), **I want to** configure the TTL, intensity, and radius of each
stimulus type, **so that** events decay and propagate realistically.

## US-7.6.5.3 Test Stimulus Expiration in Editor
**As a** designer (P-5), **I want to** spawn a stimulus in the editor and watch it expire
after its TTL, **so that** I can verify expiration timing.

## US-7.6.5.4 See Guards React to Noises I Create
**As a** player (P-23), **I want** guards to react to noises I create (breaking glass,
knocking on doors), **so that** the stimulus system connects my actions to AI responses.

## US-7.6.5.5 Watch Alert Fade After Stimulus Expires
**As a** player (P-23), **I want** guards to stop searching after the stimulus expires,
**so that** alerts have a natural time limit.

## US-7.6.5.6 See Multiple Stimuli Stack Appropriately
**As a** player (P-23), **I want** multiple nearby noises to increase AI alertness
additively, **so that** causing repeated disturbances eventually draws investigation.

## US-7.6.5.7 Implement Global Spatial Stimulus Registry
**As an** engine developer (P-26), **I want to** implement a global stimulus registry with
spatial queries and automatic TTL expiration, **so that** perception systems query stimuli
efficiently.

## US-7.6.5.8 Cap Active Stimuli Per Platform
**As an** engine developer (P-26), **I want to** cap active stimuli to 256 on mobile and 2048
on desktop, **so that** registry size fits within memory budgets.

## US-7.6.5.9 Use Shorter Default TTLs on Mobile
**As an** engine developer (P-26), **I want** mobile to use shorter default TTLs to keep the
registry compact, **so that** stale stimuli are cleaned up faster.

## US-7.6.5.10 Verify Stimuli Expire After TTL
**As an** engine tester (P-27), **I want to** verify that stimuli are removed from the
registry exactly when their TTL expires, **so that** expiration timing is correct.

## US-7.6.5.11 Test Spatial Query Returns Nearby Stimuli Only
**As an** engine tester (P-27), **I want to** verify that spatial queries return only stimuli
within the query radius and not those outside, **so that** spatial filtering works.

## US-7.6.5.12 Stress Test Registry at Maximum Stimulus Count
**As an** engine tester (P-27), **I want to** stress test the registry at the platform
stimulus cap and verify correct behavior, **so that** the cap is enforced without data loss.

---

## F-7.6.6 — Sense Aging & Memory Decay

## US-7.6.6.1 Configure Memory Retention Duration Per Archetype
**As a** designer (P-5), **I want to** configure how long each agent archetype retains
perception memory, **so that** smart scouts remember longer than forgetful grunts.

## US-7.6.6.2 Set Decay Rate Per Stimulus Type
**As a** designer (P-5), **I want to** set different decay rates per stimulus type (visual
decays fast, damage decays slow), **so that** memory priority matches threat severity.

## US-7.6.6.3 Preview Memory State in Debug View
**As a** designer (P-5), **I want to** see an agent's perception memory with confidence
levels and age in a debug panel, **so that** I can tune decay parameters.

## US-7.6.6.4 See Guards Lose Track of Me Over Time
**As a** player (P-23), **I want** guards to gradually lose certainty about my position when
they can no longer see me, **so that** stealth rewards patience.

## US-7.6.6.5 Watch AI Search Last Known Position Then Give Up
**As a** player (P-23), **I want** AI to search my last known position, then gradually give
up and return to patrol as memory decays, **so that** escape feels achievable.

## US-7.6.6.6 See Veteran NPCs Track Longer Than Novices
**As a** player (P-23), **I want** veteran NPCs to remember my position longer than novice
ones, **so that** NPC quality differences are reflected in gameplay difficulty.

## US-7.6.6.7 Implement Time-Based Confidence Decay
**As an** engine developer (P-26), **I want to** track stimulus age and decay confidence over
time since last confirmation, **so that** stale perceptions fade to vague last-known data.

## US-7.6.6.8 Support Per-Archetype Memory Profiles
**As an** engine developer (P-26), **I want to** support per-archetype memory duration and
decay rate profiles, **so that** NPC variety is expressed through perception quality.

## US-7.6.6.9 Use Faster Decay on Mobile
**As an** engine developer (P-26), **I want** mobile to use faster decay rates and shorter
maximum memory retention, **so that** per-agent memory list size stays small.

## US-7.6.6.10 Verify Confidence Reaches Zero at Configured Time
**As an** engine tester (P-27), **I want to** verify that stimulus confidence decays to zero
at exactly the configured retention duration, **so that** decay timing is correct.

## US-7.6.6.11 Test Stimulus Refresh Resets Confidence
**As an** engine tester (P-27), **I want to** verify that re-confirming a stimulus (seeing
the target again) resets confidence to maximum, **so that** refresh works.

## US-7.6.6.12 Validate Mobile Uses Faster Decay Rates
**As an** engine tester (P-27), **I want to** verify that mobile builds use the configured
faster decay rates and shorter retention, **so that** the platform optimization is active.

---

## F-7.6.7 — Custom Senses & Perception Priority

## US-7.6.7.1 Register Custom Senses via Trait Interface
**As a** designer (P-5), **I want to** register project-specific senses (tremor sense, magic
detection, thermal vision) through a trait interface, **so that** the perception system is
extensible.

## US-7.6.7.2 Declare Processing Cost Per Custom Sense
**As a** designer (P-5), **I want** each custom sense to declare its processing cost,
**so that** the perception scheduler allocates budget appropriately.

## US-7.6.7.3 Configure Perception Budget Per Platform
**As a** designer (P-5), **I want to** configure the per-tick perception budget per platform,
**so that** high-priority senses always run while low-priority ones defer gracefully.

## US-7.6.7.4 See NPCs with Tremor Sense Detect Underground
**As a** player (P-23), **I want** NPCs with tremor sense to detect me through the ground,
**so that** unique senses create novel stealth challenges.

## US-7.6.7.5 Watch Magic-Sensitive NPCs See Through Illusions
**As a** player (P-23), **I want** magic-sensitive NPCs to see through illusions that fool
normal guards, **so that** custom senses affect gameplay strategy.

## US-7.6.7.6 Notice Low-Priority Senses Activate Gradually
**As a** player (P-23), **I want** NPCs with low-priority custom senses to detect threats
slightly slower, **so that** budget deferral does not cause jarring behavior.

## US-7.6.7.7 Implement Perception Scheduler with Budget
**As an** engine developer (P-26), **I want to** implement a perception scheduler that
allocates per-tick CPU budget to senses by priority, running high-priority first and
deferring low-priority, **so that** perception scales within budget.

## US-7.6.7.8 Support Custom Sense Registration
**As an** engine developer (P-26), **I want to** support runtime registration of custom sense
types through a trait interface, **so that** gameplay code can add new perception modes.

## US-7.6.7.9 Apply Tighter Budget on Mobile
**As an** engine developer (P-26), **I want** the mobile per-tick perception budget to be 0.25
ms (vs. 1 ms on desktop), **so that** low-priority custom senses may tick every few seconds.

## US-7.6.7.10 Verify High-Priority Senses Run Every Tick
**As an** engine tester (P-27), **I want to** verify that high-priority senses are evaluated
every tick even when the budget is tight, **so that** critical perception is never deferred.

## US-7.6.7.11 Test Low-Priority Sense Deferral
**As an** engine tester (P-27), **I want to** verify that low-priority senses are deferred to
subsequent ticks when the budget is exhausted, **so that** scheduling works correctly.

## US-7.6.7.12 Benchmark Custom Sense Throughput
**As an** engine tester (P-27), **I want to** benchmark the number of custom sense evaluations
per tick across platforms, **so that** the scheduler scales to the expected sense count.

---

## F-7.6.8 — Smell Sense and Scent Trails

## US-7.6.8.1 Configure Scent Sources Per Entity Type
**As a** designer (P-5), **I want to** configure which entities emit scent (player, food,
blood, smoke) with type, intensity, and decay rate, **so that** the scent system matches my
game's tracking needs.

## US-7.6.8.2 Set Scent Trail Persistence Per Type
**As a** designer (P-5), **I want to** set how long each scent type persists (blood trails
last hours, footstep scent fades in minutes), **so that** tracking difficulty varies by
situation.

## US-7.6.8.3 Configure Wind Effect on Scent Drift
**As a** designer (P-5), **I want to** configure how wind affects scent propagation direction
and speed, **so that** wind direction adds a strategic element to stealth.

## US-7.6.8.4 Stay Downwind to Avoid Detection
**As a** player (P-23), **I want** predator AI to track me by scent that drifts with the
wind, **so that** I can stay downwind to avoid detection.

## US-7.6.8.5 See Tracking Dogs Follow My Scent Trail
**As a** player (P-23), **I want** tracking dogs to follow my scent trail by moving toward
the strongest adjacent scent point, **so that** pursuit feels intelligent.

## US-7.6.8.6 Use Rain to Wash Away My Scent
**As a** player (P-23), **I want** rain to dilute my scent trail, making it harder for
trackers, **so that** weather affects stealth gameplay.

## US-7.6.8.7 Implement Scent Spatial Grid and Propagation
**As an** engine developer (P-26), **I want to** implement a scent spatial grid with
intensity-based propagation and wind drift, **so that** scent spreads realistically through
the environment.

## US-7.6.8.8 Generate Scent Trails Along Entity Paths
**As an** engine developer (P-26), **I want to** generate decaying scent points along a moving
entity's path, **so that** trackers can follow the trail by seeking the strongest adjacent
point.

## US-7.6.8.9 Block Scent Through Sealed Doors
**As an** engine developer (P-26), **I want** scent to be blocked by sealed doors and diluted
by rain/water, **so that** environmental factors affect scent propagation.

## US-7.6.8.10 Verify Scent Propagates Downwind
**As an** engine tester (P-27), **I want to** verify that scent propagates in the downwind
direction and is detectable at greater range downwind than upwind, **so that** wind drift
works.

## US-7.6.8.11 Test Scent Decay Matches Configured Rates
**As an** engine tester (P-27), **I want to** verify that each scent type decays at its
configured rate and expires at the expected time, **so that** persistence tuning works.

## US-7.6.8.12 Validate Sealed Doors Block Scent Propagation
**As an** engine tester (P-27), **I want to** verify that scent does not propagate through
sealed doors, **so that** environmental blocking is enforced.

---

## F-7.6.9 — Environmental Evidence and Footprint Detection

## US-7.6.9.1 Configure Evidence Types and Spawn Rules
**As a** designer (P-5), **I want to** configure which evidence types spawn (footprints, blood
drops, broken vegetation, shell casings) and their spawn rates, **so that** evidence density
matches performance and gameplay needs.

## US-7.6.9.2 Set Evidence Decay Timers Per Type
**As a** designer (P-5), **I want to** set decay timers per evidence type, **so that**
footprints in snow last longer than blood on stone.

## US-7.6.9.3 Configure AI Tracking Sense Radius
**As a** designer (P-5), **I want to** configure the query radius for AI with `TrackingSense`,
**so that** tracking NPCs search an appropriate area for evidence.

## US-7.6.9.4 See Tracking Dogs Follow My Footprints in Snow
**As a** player (P-23), **I want** enemy tracking dogs to follow my footprints through snow
and mud, **so that** I think about which surfaces I walk on to avoid leaving trails.

## US-7.6.9.5 Watch AI Follow Blood Trail to Injured Player
**As a** player (P-23), **I want** enemies to find me by following blood drops I leave when
injured, **so that** getting hurt has consequences beyond HP loss.

## US-7.6.9.6 See AI Infer Travel Direction from Footprints
**As a** player (P-23), **I want** AI trackers to correctly follow my footprints in the
direction I traveled, **so that** tracking looks intelligent.

## US-7.6.9.7 Implement EnvironmentalEvidence ECS Component
**As an** engine developer (P-26), **I want to** implement evidence as ECS entities with type,
timestamp, source, location, and decay timer, **so that** evidence is queryable and expires.

## US-7.6.9.8 Generate Footprints from Character Controller
**As an** engine developer (P-26), **I want** the character controller to spawn footprint
evidence when moving over deformable terrain materials, **so that** footprints appear
automatically.

## US-7.6.9.9 Throttle Evidence Spawn Rate for Performance
**As an** engine developer (P-26), **I want to** throttle evidence spawning to one footprint
per N steps, **so that** evidence count stays within the entity budget.

## US-7.6.9.10 Verify Evidence Spawns on Correct Materials
**As an** engine tester (P-27), **I want to** verify that footprints spawn only on deformable
terrain materials (snow, mud, sand) and not on hard surfaces, **so that** material detection
works.

## US-7.6.9.11 Test Evidence Decay and Cleanup
**As an** engine tester (P-27), **I want to** verify that evidence entities are removed when
their decay timer expires, **so that** no stale evidence persists.

## US-7.6.9.12 Validate AI Infers Correct Direction from Footprints
**As an** engine tester (P-27), **I want to** verify that AI with TrackingSense correctly
infers direction of travel from footprint orientation, **so that** tracking direction is
accurate.

---

## F-7.6.10 — AI Investigation Behavior

## US-7.6.10.1 Configure Investigation Behavior Per Stimulus Type
**As a** designer (P-5), **I want to** configure investigation behavior per stimulus type
(sound: move to origin; scent: follow trail; visual: move to last seen position), **so that**
each stimulus produces the appropriate investigation.

## US-7.6.10.2 Set Investigation Timeout
**As a** designer (P-5), **I want to** set a timeout after which investigation ends and the
agent returns to unaware state, **so that** NPCs do not search indefinitely.

## US-7.6.10.3 Configure Multi-Agent Investigation Claiming
**As a** designer (P-5), **I want** the first agent to claim a stimulus for investigation
while others continue patrol, **so that** only one NPC investigates each event.

## US-7.6.10.4 See Guards Investigate Suspicious Sounds
**As a** player (P-23), **I want** guards to investigate suspicious sounds by walking to the
source, looking around, and listening, **so that** I can use distractions to create openings.

## US-7.6.10.5 Watch AI Follow Scent Trail During Investigation
**As a** player (P-23), **I want** AI investigating a scent to follow the trail toward
increasing intensity, **so that** scent-based investigation looks methodical.

## US-7.6.10.6 See AI Escalate to Alert After Finding Evidence
**As a** player (P-23), **I want** an investigating AI to escalate to alerted state when it
finds confirming evidence, **so that** investigation can lead to full combat engagement.

## US-7.6.10.7 Implement Alert State Machine Integration
**As an** engine developer (P-26), **I want to** integrate investigation with the AI alert
state machine (unaware -> suspicious -> alerted -> unaware), **so that** investigation
drives state transitions.

## US-7.6.10.8 Select Investigation Behavior by Stimulus Type
**As an** engine developer (P-26), **I want to** select investigation behavior dynamically
based on stimulus type, **so that** each sense triggers the appropriate search pattern.

## US-7.6.10.9 Simplify Investigation on Mobile
**As an** engine developer (P-26), **I want** mobile to use simplified investigation (skip
scent/footprint following, use direct move-to-position only), **so that** investigation
cost fits the mobile budget.

## US-7.6.10.10 Verify Only One Agent Claims Each Stimulus
**As an** engine tester (P-27), **I want to** verify that only one agent claims a stimulus
for investigation and others continue patrol, **so that** coordination prevents redundancy.

## US-7.6.10.11 Test State Machine Transitions During Investigation
**As an** engine tester (P-27), **I want to** verify the full state machine cycle (unaware ->
suspicious -> alerted -> unaware) during investigation, **so that** transitions are correct.

## US-7.6.10.12 Validate Investigation Timeout Returns to Patrol
**As an** engine tester (P-27), **I want to** verify that an investigation that finds nothing
ends after the configured timeout and the agent returns to patrol, **so that** timeouts work.

---

## F-7.6.11 — Multi-Sense Tracking and Pursuit

## US-7.6.11.1 Configure Tracking Methods Per Agent Archetype
**As a** designer (P-5), **I want to** enable or disable tracking methods (visual, audio,
scent, evidence, predictive) per agent archetype, **so that** tracking sophistication matches
the NPC's role.

## US-7.6.11.2 Set Tracking Confidence Thresholds
**As a** designer (P-5), **I want to** set confidence thresholds for transitioning between
tracking methods and transitioning to search patterns, **so that** tracking intensity is
tunable.

## US-7.6.11.3 Enable Pack Tracking for Group AI
**As a** designer (P-5), **I want to** enable pack tracking so multiple AI agents share
tracking data and one agent's sighting updates all pack members, **so that** group pursuit
is coordinated.

## US-7.6.11.4 See Enemies Switch from Sight to Sound Tracking
**As a** player (P-23), **I want** a pursuing enemy to switch from visual to audio tracking
when I run behind a wall, **so that** breaking line of sight does not end pursuit
immediately.

## US-7.6.11.5 Watch Tracker Switch to Scent When I Stop Moving
**As a** player (P-23), **I want** a tracker to switch to following my scent trail when I stop
making noise, **so that** escape requires countering multiple detection methods.

## US-7.6.11.6 See Pack Coordinate Their Pursuit
**As a** player (P-23), **I want** a wolf pack where one wolf's sighting updates all pack
members' pursuit direction, **so that** pack hunting feels coordinated and dangerous.

## US-7.6.11.7 Implement TrackingState with Multi-Sense Input
**As an** engine developer (P-26), **I want to** implement a `TrackingState` that maintains
last confirmed position, estimated current position, confidence, and active tracking method,
**so that** tracking is stateful.

## US-7.6.11.8 Implement Seamless Sense Transition Logic
**As an** engine developer (P-26), **I want to** implement seamless transitions between
tracking methods based on sense availability and confidence, **so that** the tracker adapts
to changing conditions.

## US-7.6.11.9 Limit Mobile to Sight + Hearing Tracking Only
**As an** engine developer (P-26), **I want** mobile to limit tracking to sight and hearing
only, disabling scent/evidence/predictive tracking, **so that** tracking cost fits the mobile
budget.

## US-7.6.11.10 Verify Tracking Method Transitions
**As an** engine tester (P-27), **I want to** verify that the tracker transitions from visual
to audio to scent as each sense loses the target, **so that** sense fallback works.

## US-7.6.11.11 Test Confidence Decay Triggers Search Patterns
**As an** engine tester (P-27), **I want to** verify that tracking confidence decaying to zero
transitions the tracker to search patterns, **so that** failed tracking ends gracefully.

## US-7.6.11.12 Validate Pack Tracking Shares Data Correctly
**As an** engine tester (P-27), **I want to** verify that one pack member's sighting updates
the estimated position for all pack members within the group, **so that** data sharing works.
