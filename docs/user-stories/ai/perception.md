# User Stories — 7.6 Perception

## F-7.6.1 — Sight Sense (Cone + Line of Sight)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.6.1.1 | designer (P-5) | I want to configure vision cone range, half-angle, and peripheral falloff per agent archetype | eagle-eyed snipers see farther than short-sighted merchants | F-7.6.1 | R-7.6.1 |
| US-7.6.1.2 | designer (P-5) | I want to set which trace channel the sight sense uses for line-of-sight raycasts | glass blocks some NPCs but not others | F-7.6.1 | R-7.6.1 |
| US-7.6.1.3 | designer (P-5) | I want to preview an agent's vision cone as a debug overlay in the editor | I can verify the cone covers the intended area | F-7.6.1 | R-7.6.1 |
| US-7.6.1.4 | player (P-23) | I want guards to have realistic vision cones with line-of-sight checks | I can sneak behind them or hide behind walls and feel rewarded for positioning | F-7.6.1 | R-7.6.1 |
| US-7.6.1.5 | player (P-23) | I want guards to detect me when I step into their vision cone | sight-based detection feels fair and predictable | F-7.6.1 | R-7.6.1 |
| US-7.6.1.6 | player (P-23) | I want hide behind frosted glass that blocks certain NPCs' line of sight | environmental stealth mechanics work with trace channels | F-7.6.1 | R-7.6.1 |
| US-7.6.1.7 | engine developer (P-26) | I want to implement sight detection as a vision cone check followed by a line-of-sight raycast against the physics scene | only truly visible targets are detected | F-7.6.1 | R-7.6.1 |
| US-7.6.1.8 | engine developer (P-26) | I want to support per-archetype tuning of range, half-angle, and peripheral falloff | different NPC types have distinct visual capabilities | F-7.6.1 | R-7.6.1 |
| US-7.6.1.9 | engine developer (P-26) | I want to cull targets outside the vision cone before performing expensive raycasts | sight checks scale with visible target count | F-7.6.1 | R-7.6.1 |
| US-7.6.1.10 | engine tester (P-27) | I want to verify that the vision cone geometry matches the configured range and half-angle with test targets at boundary positions | detection boundaries are precise | F-7.6.1 | R-7.6.1 |
| US-7.6.1.11 | engine tester (P-27) | I want to verify that targets inside the vision cone but behind opaque geometry are not detected | line-of-sight blocking works | F-7.6.1 | R-7.6.1 |
| US-7.6.1.12 | engine tester (P-27) | I want to benchmark sight sense evaluation for 500+ agents simultaneously | perception scales to server-side NPC counts. --- | F-7.6.1 | R-7.6.1 |

## F-7.6.2 — Hearing Sense (Radius + Attenuation)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.6.2.1 | designer (P-5) | I want to configure the hearing radius and attenuation curve per agent archetype | keen-eared scouts detect sounds farther than oblivious merchants | F-7.6.2 | R-7.6.2 |
| US-7.6.2.2 | designer (P-5) | I want to configure how much intervening geometry attenuates sound | walls muffle footsteps realistically | F-7.6.2 | R-7.6.2 |
| US-7.6.2.3 | designer (P-5) | I want to assign intensity values to sound types (whisper, footstep, gunfire, explosion) | louder sounds propagate farther and are detected with higher confidence | F-7.6.2 | R-7.6.2 |
| US-7.6.2.4 | player (P-23) | I want guards to investigate the sound of a thrown object | I can create distractions to lure them away from their posts | F-7.6.2 | R-7.6.2 |
| US-7.6.2.5 | player (P-23) | I want enemies to detect my footsteps when I run nearby | moving quietly matters for stealth | F-7.6.2 | R-7.6.2 |
| US-7.6.2.6 | player (P-23) | I want walls between me and a guard to muffle my footsteps | environmental geometry affects detection | F-7.6.2 | R-7.6.2 |
| US-7.6.2.7 | engine developer (P-26) | I want to implement hearing as a spherical range check with distance attenuation and geometry occlusion | sound detection is physically plausible | F-7.6.2 | R-7.6.2 |
| US-7.6.2.8 | engine developer (P-26) | I want to use simplified server-side collision for sound occlusion queries | hearing does not depend on the audio engine | F-7.6.2 | R-7.6.2 |
| US-7.6.2.9 | engine developer (P-26) | I want to map stimulus intensity and distance to a detection confidence score | NPCs distinguish between a whisper nearby and an explosion far away | F-7.6.2 | R-7.6.2 |
| US-7.6.2.10 | engine tester (P-27) | I want to verify that sound detection confidence decreases with distance according to the configured attenuation curve | distance scaling is correct | F-7.6.2 | R-7.6.2 |
| US-7.6.2.11 | engine tester (P-27) | I want to verify that intervening geometry reduces sound detection confidence by the configured occlusion factor | walls block sound | F-7.6.2 | R-7.6.2 |
| US-7.6.2.12 | engine tester (P-27) | I want to verify that a gunshot is detected at 3x the range of a footstep, matching configured intensity ratios | intensity scaling works. --- | F-7.6.2 | R-7.6.2 |

## F-7.6.3 — Damage Sense

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.6.3.1 | designer (P-5) | I want to configure damage sense as a high-priority perception event that bypasses range and LOS | attacked NPCs always react | F-7.6.3 | R-7.6.3 |
| US-7.6.3.2 | designer (P-5) | I want damage sense to report the direction and magnitude of incoming damage | NPCs turn toward their attacker | F-7.6.3 | R-7.6.3 |
| US-7.6.3.3 | designer (P-5) | I want to set a minimum damage threshold before damage sense triggers | trivial chip damage does not break stealth | F-7.6.3 | R-7.6.3 |
| US-7.6.3.4 | player (P-23) | I want an attacked NPC to immediately turn toward the direction of the attack | damage reaction feels instant and aware | F-7.6.3 | R-7.6.3 |
| US-7.6.3.5 | player (P-23) | I want enemies to react to being shot from behind even if they cannot see me | damage always provokes a response | F-7.6.3 | R-7.6.3 |
| US-7.6.3.6 | player (P-23) | I want NPCs to ignore very minor environmental damage (e.g., a pebble) | the damage threshold prevents trivial alerts | F-7.6.3 | R-7.6.3 |
| US-7.6.3.7 | engine developer (P-26) | I want to implement damage sense as a lightweight event-driven check that registers direction and magnitude | it bypasses the normal perception pipeline | F-7.6.3 | R-7.6.3 |
| US-7.6.3.8 | engine developer (P-26) | I want damage events to bypass range and line-of-sight checks | NPCs always react to being attacked regardless of attacker visibility | F-7.6.3 | R-7.6.3 |
| US-7.6.3.9 | engine developer (P-26) | I want damage sense to be a lightweight event check with identical behavior on all platforms | there are no platform-specific edge cases | F-7.6.3 | R-7.6.3 |
| US-7.6.3.10 | engine tester (P-27) | I want to verify that damage sense fires for every damage event above the threshold regardless of attacker position | no attacks go unnoticed | F-7.6.3 | R-7.6.3 |
| US-7.6.3.11 | engine tester (P-27) | I want to verify that the reported damage direction matches the actual attacker direction within a tolerance | NPCs turn correctly | F-7.6.3 | R-7.6.3 |
| US-7.6.3.12 | engine tester (P-27) | I want to verify that damage below the configured threshold does not trigger damage sense | the filter works. --- | F-7.6.3 | R-7.6.3 |

## F-7.6.4 — Team & Faction Awareness

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.6.4.1 | designer (P-5) | I want to configure a faction affinity table defining relationships (hostile, neutral, friendly) between all factions | NPC perception filters stimuli by faction | F-7.6.4 | R-7.6.4 |
| US-7.6.4.2 | designer (P-5) | I want to override individual NPC faction relationships | a friendly NPC in a hostile faction behaves differently from its group | F-7.6.4 | R-7.6.4 |
| US-7.6.4.3 | designer (P-5) | I want to modify faction relationships at runtime via gameplay events (reputation changes, betrayals) | the world reacts to player actions | F-7.6.4 | R-7.6.4 |
| US-7.6.4.4 | player (P-23) | I want guards to ignore friendly faction NPCs walking past them | perception correctly filters by affinity | F-7.6.4 | R-7.6.4 |
| US-7.6.4.5 | player (P-23) | I want NPCs to turn hostile after a reputation change, attacking me where they previously ignored me | faction dynamics feel responsive | F-7.6.4 | R-7.6.4 |
| US-7.6.4.6 | player (P-23) | I want hostile NPCs to prioritize hostile targets over neutral ones when both are visible | faction awareness drives targeting | F-7.6.4 | R-7.6.4 |
| US-7.6.4.7 | engine developer (P-26) | I want to tag each perceived stimulus with the source entity's faction and relationship | perception filters can ignore or escalate based on affinity | F-7.6.4 | R-7.6.4 |
| US-7.6.4.8 | engine developer (P-26) | I want to read faction relations from a shared affinity table that gameplay systems can modify at runtime | relationships are centralized and dynamic | F-7.6.4 | R-7.6.4 |
| US-7.6.4.9 | engine developer (P-26) | I want gameplay systems to modify the affinity table at runtime (reputation, betrayals) | faction relationships evolve during gameplay | F-7.6.4 | R-7.6.4 |
| US-7.6.4.10 | engine tester (P-27) | I want to verify that perception filters correctly ignore friendly stimuli when configured to do so | allies do not trigger alert states | F-7.6.4 | R-7.6.4 |
| US-7.6.4.11 | engine tester (P-27) | I want to verify that changing a faction relationship at runtime immediately affects perception filtering for all agents of that faction | changes propagate | F-7.6.4 | R-7.6.4 |
| US-7.6.4.12 | engine tester (P-27) | I want to verify that a per-NPC faction override takes precedence over the faction default | individual overrides work. --- | F-7.6.4 | R-7.6.4 |

## F-7.6.5 — Stimuli Registration & Expiration

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.6.5.1 | designer (P-5) | I want gameplay systems to broadcast perception events (noise at position, visual flash, scent trail) to a global stimulus registry | AI can detect them | F-7.6.5 | R-7.6.5 |
| US-7.6.5.2 | designer (P-5) | I want to configure the TTL, intensity, and radius of each stimulus type | events decay and propagate realistically | F-7.6.5 | R-7.6.5 |
| US-7.6.5.3 | designer (P-5) | I want to spawn a stimulus in the editor and watch it expire after its TTL | I can verify expiration timing | F-7.6.5 | R-7.6.5 |
| US-7.6.5.4 | player (P-23) | I want guards to react to noises I create (breaking glass, knocking on doors) | the stimulus system connects my actions to AI responses | F-7.6.5 | R-7.6.5 |
| US-7.6.5.5 | player (P-23) | I want guards to stop searching after the stimulus expires | alerts have a natural time limit | F-7.6.5 | R-7.6.5 |
| US-7.6.5.6 | player (P-23) | I want multiple nearby noises to increase AI alertness additively | causing repeated disturbances eventually draws investigation | F-7.6.5 | R-7.6.5 |
| US-7.6.5.7 | engine developer (P-26) | I want to implement a global stimulus registry with spatial queries and automatic TTL expiration | perception systems query stimuli efficiently | F-7.6.5 | R-7.6.5 |
| US-7.6.5.8 | engine developer (P-26) | I want to cap active stimuli to 256 on mobile and 2048 on desktop | registry size fits within memory budgets | F-7.6.5 | R-7.6.5 |
| US-7.6.5.9 | engine developer (P-26) | I want mobile to use shorter default TTLs to keep the registry compact | stale stimuli are cleaned up faster | F-7.6.5 | R-7.6.5 |
| US-7.6.5.10 | engine tester (P-27) | I want to verify that stimuli are removed from the registry exactly when their TTL expires | expiration timing is correct | F-7.6.5 | R-7.6.5 |
| US-7.6.5.11 | engine tester (P-27) | I want to verify that spatial queries return only stimuli within the query radius and not those outside | spatial filtering works | F-7.6.5 | R-7.6.5 |
| US-7.6.5.12 | engine tester (P-27) | I want to stress test the registry at the platform stimulus cap and verify correct behavior | the cap is enforced without data loss. --- | F-7.6.5 | R-7.6.5 |

## F-7.6.6 — Sense Aging & Memory Decay

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.6.6.1 | designer (P-5) | I want to configure how long each agent archetype retains perception memory | smart scouts remember longer than forgetful grunts | F-7.6.6 | R-7.6.6 |
| US-7.6.6.2 | designer (P-5) | I want to set different decay rates per stimulus type (visual decays fast, damage decays slow) | memory priority matches threat severity | F-7.6.6 | R-7.6.6 |
| US-7.6.6.3 | designer (P-5) | I want to see an agent's perception memory with confidence levels and age in a debug panel | I can tune decay parameters | F-7.6.6 | R-7.6.6 |
| US-7.6.6.4 | player (P-23) | I want guards to gradually lose certainty about my position when they can no longer see me | stealth rewards patience | F-7.6.6 | R-7.6.6 |
| US-7.6.6.5 | player (P-23) | I want AI to search my last known position, then gradually give up and return to patrol as memory decays | escape feels achievable | F-7.6.6 | R-7.6.6 |
| US-7.6.6.6 | player (P-23) | I want veteran NPCs to remember my position longer than novice ones | NPC quality differences are reflected in gameplay difficulty | F-7.6.6 | R-7.6.6 |
| US-7.6.6.7 | engine developer (P-26) | I want to track stimulus age and decay confidence over time since last confirmation | stale perceptions fade to vague last-known data | F-7.6.6 | R-7.6.6 |
| US-7.6.6.8 | engine developer (P-26) | I want to support per-archetype memory duration and decay rate profiles | NPC variety is expressed through perception quality | F-7.6.6 | R-7.6.6 |
| US-7.6.6.9 | engine developer (P-26) | I want mobile to use faster decay rates and shorter maximum memory retention | per-agent memory list size stays small | F-7.6.6 | R-7.6.6 |
| US-7.6.6.10 | engine tester (P-27) | I want to verify that stimulus confidence decays to zero at exactly the configured retention duration | decay timing is correct | F-7.6.6 | R-7.6.6 |
| US-7.6.6.11 | engine tester (P-27) | I want to verify that re-confirming a stimulus (seeing the target again) resets confidence to maximum | refresh works | F-7.6.6 | R-7.6.6 |
| US-7.6.6.12 | engine tester (P-27) | I want to verify that mobile builds use the configured faster decay rates and shorter retention | the platform optimization is active. --- | F-7.6.6 | R-7.6.6 |

## F-7.6.7 — Custom Senses & Perception Priority

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.6.7.1 | designer (P-5) | I want to register project-specific senses (tremor sense, magic detection, thermal vision) through a trait interface | the perception system is extensible | F-7.6.7 | R-7.6.7 |
| US-7.6.7.2 | designer (P-5) | I want each custom sense to declare its processing cost | the perception scheduler allocates budget appropriately | F-7.6.7 | R-7.6.7 |
| US-7.6.7.3 | designer (P-5) | I want to configure the per-tick perception budget per platform | high-priority senses always run while low-priority ones defer gracefully | F-7.6.7 | R-7.6.7 |
| US-7.6.7.4 | player (P-23) | I want NPCs with tremor sense to detect me through the ground | unique senses create novel stealth challenges | F-7.6.7 | R-7.6.7 |
| US-7.6.7.5 | player (P-23) | I want magic-sensitive NPCs to see through illusions that fool normal guards | custom senses affect gameplay strategy | F-7.6.7 | R-7.6.7 |
| US-7.6.7.6 | player (P-23) | I want NPCs with low-priority custom senses to detect threats slightly slower | budget deferral does not cause jarring behavior | F-7.6.7 | R-7.6.7 |
| US-7.6.7.7 | engine developer (P-26) | I want to implement a perception scheduler that allocates per-tick CPU budget to senses by priority, running high-priority first and deferring low-priority | perception scales within budget | F-7.6.7 | R-7.6.7 |
| US-7.6.7.8 | engine developer (P-26) | I want to support runtime registration of custom sense types through a trait interface | gameplay code can add new perception modes | F-7.6.7 | R-7.6.7 |
| US-7.6.7.9 | engine developer (P-26) | I want the mobile per-tick perception budget to be 0.25 ms (vs. 1 ms on desktop) | low-priority custom senses may tick every few seconds | F-7.6.7 | R-7.6.7 |
| US-7.6.7.10 | engine tester (P-27) | I want to verify that high-priority senses are evaluated every tick even when the budget is tight | critical perception is never deferred | F-7.6.7 | R-7.6.7 |
| US-7.6.7.11 | engine tester (P-27) | I want to verify that low-priority senses are deferred to subsequent ticks when the budget is exhausted | scheduling works correctly | F-7.6.7 | R-7.6.7 |
| US-7.6.7.12 | engine tester (P-27) | I want to benchmark the number of custom sense evaluations per tick across platforms | the scheduler scales to the expected sense count. --- | F-7.6.7 | R-7.6.7 |

## F-7.6.8 — Smell Sense and Scent Trails

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.6.8.1 | designer (P-5) | I want to configure which entities emit scent (player, food, blood, smoke) with type, intensity, and decay rate | the scent system matches my game's tracking needs | F-7.6.8 | R-7.6.8 |
| US-7.6.8.2 | designer (P-5) | I want to set how long each scent type persists (blood trails last hours, footstep scent fades in minutes) | tracking difficulty varies by situation | F-7.6.8 | R-7.6.8 |
| US-7.6.8.3 | designer (P-5) | I want to configure how wind affects scent propagation direction and speed | wind direction adds a strategic element to stealth | F-7.6.8 | R-7.6.8 |
| US-7.6.8.4 | player (P-23) | I want predator AI to track me by scent that drifts with the wind | I can stay downwind to avoid detection | F-7.6.8 | R-7.6.8 |
| US-7.6.8.5 | player (P-23) | I want tracking dogs to follow my scent trail by moving toward the strongest adjacent scent point | pursuit feels intelligent | F-7.6.8 | R-7.6.8 |
| US-7.6.8.6 | player (P-23) | I want rain to dilute my scent trail, making it harder for trackers | weather affects stealth gameplay | F-7.6.8 | R-7.6.8 |
| US-7.6.8.7 | engine developer (P-26) | I want to implement a scent spatial grid with intensity-based propagation and wind drift | scent spreads realistically through the environment | F-7.6.8 | R-7.6.8 |
| US-7.6.8.8 | engine developer (P-26) | I want to generate decaying scent points along a moving entity's path | trackers can follow the trail by seeking the strongest adjacent point | F-7.6.8 | R-7.6.8 |
| US-7.6.8.9 | engine developer (P-26) | I want scent to be blocked by sealed doors and diluted by rain/water | environmental factors affect scent propagation | F-7.6.8 | R-7.6.8 |
| US-7.6.8.10 | engine tester (P-27) | I want to verify that scent propagates in the downwind direction and is detectable at greater range downwind than upwind | wind drift works | F-7.6.8 | R-7.6.8 |
| US-7.6.8.11 | engine tester (P-27) | I want to verify that each scent type decays at its configured rate and expires at the expected time | persistence tuning works | F-7.6.8 | R-7.6.8 |
| US-7.6.8.12 | engine tester (P-27) | I want to verify that scent does not propagate through sealed doors | environmental blocking is enforced. --- | F-7.6.8 | R-7.6.8 |

## F-7.6.9 — Environmental Evidence and Footprint Detection

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.6.9.1 | designer (P-5) | I want to configure which evidence types spawn (footprints, blood drops, broken vegetation, shell casings) and their spawn rates | evidence density matches performance and gameplay needs | F-7.6.9 | R-7.6.9 |
| US-7.6.9.2 | designer (P-5) | I want to set decay timers per evidence type | footprints in snow last longer than blood on stone | F-7.6.9 | R-7.6.9 |
| US-7.6.9.3 | designer (P-5) | I want to configure the query radius for AI with `TrackingSense` | tracking NPCs search an appropriate area for evidence | F-7.6.9 | R-7.6.9 |
| US-7.6.9.4 | player (P-23) | I want enemy tracking dogs to follow my footprints through snow and mud | I think about which surfaces I walk on to avoid leaving trails | F-7.6.9 | R-7.6.9 |
| US-7.6.9.5 | player (P-23) | I want enemies to find me by following blood drops I leave when injured | getting hurt has consequences beyond HP loss | F-7.6.9 | R-7.6.9 |
| US-7.6.9.6 | player (P-23) | I want AI trackers to correctly follow my footprints in the direction I traveled | tracking looks intelligent | F-7.6.9 | R-7.6.9 |
| US-7.6.9.7 | engine developer (P-26) | I want to implement evidence as ECS entities with type, timestamp, source, location, and decay timer | evidence is queryable and expires | F-7.6.9 | R-7.6.9 |
| US-7.6.9.8 | engine developer (P-26) | I want the character controller to spawn footprint evidence when moving over deformable terrain materials | footprints appear automatically | F-7.6.9 | R-7.6.9 |
| US-7.6.9.9 | engine developer (P-26) | I want to throttle evidence spawning to one footprint per N steps | evidence count stays within the entity budget | F-7.6.9 | R-7.6.9 |
| US-7.6.9.10 | engine tester (P-27) | I want to verify that footprints spawn only on deformable terrain materials (snow, mud, sand) and not on hard surfaces | material detection works | F-7.6.9 | R-7.6.9 |
| US-7.6.9.11 | engine tester (P-27) | I want to verify that evidence entities are removed when their decay timer expires | no stale evidence persists | F-7.6.9 | R-7.6.9 |
| US-7.6.9.12 | engine tester (P-27) | I want to verify that AI with TrackingSense correctly infers direction of travel from footprint orientation | tracking direction is accurate. --- | F-7.6.9 | R-7.6.9 |

## F-7.6.10 — AI Investigation Behavior

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.6.10.1 | designer (P-5) | I want to configure investigation behavior per stimulus type (sound: move to origin; scent: follow trail; visual: move to last seen position) | each stimulus produces the appropriate investigation | F-7.6.10 | R-7.6.10 |
| US-7.6.10.2 | designer (P-5) | I want to set a timeout after which investigation ends and the agent returns to unaware state | NPCs do not search indefinitely | F-7.6.10 | R-7.6.10 |
| US-7.6.10.3 | designer (P-5) | I want the first agent to claim a stimulus for investigation while others continue patrol | only one NPC investigates each event | F-7.6.10 | R-7.6.10 |
| US-7.6.10.4 | player (P-23) | I want guards to investigate suspicious sounds by walking to the source, looking around, and listening | I can use distractions to create openings | F-7.6.10 | R-7.6.10 |
| US-7.6.10.5 | player (P-23) | I want AI investigating a scent to follow the trail toward increasing intensity | scent-based investigation looks methodical | F-7.6.10 | R-7.6.10 |
| US-7.6.10.6 | player (P-23) | I want an investigating AI to escalate to alerted state when it finds confirming evidence | investigation can lead to full combat engagement | F-7.6.10 | R-7.6.10 |
| US-7.6.10.7 | engine developer (P-26) | I want to integrate investigation with the AI alert state machine (unaware -> suspicious -> alerted -> unaware) | investigation drives state transitions | F-7.6.10 | R-7.6.10 |
| US-7.6.10.8 | engine developer (P-26) | I want to select investigation behavior dynamically based on stimulus type | each sense triggers the appropriate search pattern | F-7.6.10 | R-7.6.10 |
| US-7.6.10.9 | engine developer (P-26) | I want mobile to use simplified investigation (skip scent/footprint following, use direct move-to-position only) | investigation cost fits the mobile budget | F-7.6.10 | R-7.6.10 |
| US-7.6.10.10 | engine tester (P-27) | I want to verify that only one agent claims a stimulus for investigation and others continue patrol | coordination prevents redundancy | F-7.6.10 | R-7.6.10 |
| US-7.6.10.11 | engine tester (P-27) | I want to verify the full state machine cycle (unaware -> suspicious -> alerted -> unaware) during investigation | transitions are correct | F-7.6.10 | R-7.6.10 |
| US-7.6.10.12 | engine tester (P-27) | I want to verify that an investigation that finds nothing ends after the configured timeout and the agent returns to patrol | timeouts work. --- | F-7.6.10 | R-7.6.10 |

## F-7.6.11 — Multi-Sense Tracking and Pursuit

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.6.11.1 | designer (P-5) | I want to enable or disable tracking methods (visual, audio, scent, evidence, predictive) per agent archetype | tracking sophistication matches the NPC's role | F-7.6.11 | R-7.6.11 |
| US-7.6.11.2 | designer (P-5) | I want to set confidence thresholds for transitioning between tracking methods and transitioning to search patterns | tracking intensity is tunable | F-7.6.11 | R-7.6.11 |
| US-7.6.11.3 | designer (P-5) | I want to enable pack tracking so multiple AI agents share tracking data and one agent's sighting updates all pack members | group pursuit is coordinated | F-7.6.11 | R-7.6.11 |
| US-7.6.11.4 | player (P-23) | I want a pursuing enemy to switch from visual to audio tracking when I run behind a wall | breaking line of sight does not end pursuit immediately | F-7.6.11 | R-7.6.11 |
| US-7.6.11.5 | player (P-23) | I want a tracker to switch to following my scent trail when I stop making noise | escape requires countering multiple detection methods | F-7.6.11 | R-7.6.11 |
| US-7.6.11.6 | player (P-23) | I want a wolf pack where one wolf's sighting updates all pack members' pursuit direction | pack hunting feels coordinated and dangerous | F-7.6.11 | R-7.6.11 |
| US-7.6.11.7 | engine developer (P-26) | I want to implement a `TrackingState` that maintains last confirmed position, estimated current position, confidence, and active tracking method | tracking is stateful | F-7.6.11 | R-7.6.11 |
| US-7.6.11.8 | engine developer (P-26) | I want to implement seamless transitions between tracking methods based on sense availability and confidence | the tracker adapts to changing conditions | F-7.6.11 | R-7.6.11 |
| US-7.6.11.9 | engine developer (P-26) | I want mobile to limit tracking to sight and hearing only, disabling scent/evidence/predictive tracking | tracking cost fits the mobile budget | F-7.6.11 | R-7.6.11 |
| US-7.6.11.10 | engine tester (P-27) | I want to verify that the tracker transitions from visual to audio to scent as each sense loses the target | sense fallback works | F-7.6.11 | R-7.6.11 |
| US-7.6.11.11 | engine tester (P-27) | I want to verify that tracking confidence decaying to zero transitions the tracker to search patterns | failed tracking ends gracefully | F-7.6.11 | R-7.6.11 |
| US-7.6.11.12 | engine tester (P-27) | I want to verify that one pack member's sighting updates the estimated position for all pack members within the group | data sharing works | F-7.6.11 | R-7.6.11 |
