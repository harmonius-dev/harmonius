# User Stories — 7.6 Perception

## F-7.6.1 — Sight Sense (Cone + Line of Sight)

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.6.1.1  | designer (P-5)          | F-7.6.1  | R-7.6.1      |
| US-7.6.1.2  | designer (P-5)          | F-7.6.1  | R-7.6.1      |
| US-7.6.1.3  | designer (P-5)          | F-7.6.1  | R-7.6.1      |
| US-7.6.1.4  | player (P-23)           | F-7.6.1  | R-7.6.1      |
| US-7.6.1.5  | player (P-23)           | F-7.6.1  | R-7.6.1      |
| US-7.6.1.6  | player (P-23)           | F-7.6.1  | R-7.6.1      |
| US-7.6.1.7  | engine developer (P-26) | F-7.6.1  | R-7.6.1      |
| US-7.6.1.8  | engine developer (P-26) | F-7.6.1  | R-7.6.1      |
| US-7.6.1.9  | engine developer (P-26) | F-7.6.1  | R-7.6.1      |
| US-7.6.1.10 | engine tester (P-27)    | F-7.6.1  | R-7.6.1      |
| US-7.6.1.11 | engine tester (P-27)    | F-7.6.1  | R-7.6.1      |
| US-7.6.1.12 | engine tester (P-27)    | F-7.6.1  | R-7.6.1      |

1. **US-7.6.1.1** — I want to configure vision cone range, half-angle, and peripheral falloff per
   agent archetype
   - **Acceptance:** eagle-eyed snipers see farther than short-sighted merchants
2. **US-7.6.1.2** — I want to set which trace channel the sight sense uses for line-of-sight
   raycasts
   - **Acceptance:** glass blocks some NPCs but not others
3. **US-7.6.1.3** — I want to preview an agent's vision cone as a debug overlay in the editor
   - **Acceptance:** I can verify the cone covers the intended area
4. **US-7.6.1.4** — I want guards to have realistic vision cones with line-of-sight checks
   - **Acceptance:** I can sneak behind them or hide behind walls and feel rewarded for positioning
5. **US-7.6.1.5** — I want guards to detect me when I step into their vision cone
   - **Acceptance:** sight-based detection feels fair and predictable
6. **US-7.6.1.6** — I want hide behind frosted glass that blocks certain NPCs' line of sight
   - **Acceptance:** environmental stealth mechanics work with trace channels
7. **US-7.6.1.7** — I want to implement sight detection as a vision cone check followed by a
   line-of-sight raycast against the physics scene
   - **Acceptance:** only truly visible targets are detected
8. **US-7.6.1.8** — I want to support per-archetype tuning of range, half-angle, and peripheral
   falloff
   - **Acceptance:** different NPC types have distinct visual capabilities
9. **US-7.6.1.9** — I want to cull targets outside the vision cone before performing expensive
   raycasts
   - **Acceptance:** sight checks scale with visible target count
10. **US-7.6.1.10** — I want to verify that the vision cone geometry matches the configured range
    and half-angle with test targets at boundary positions
    - **Acceptance:** detection boundaries are precise
11. **US-7.6.1.11** — I want to verify that targets inside the vision cone but behind opaque
    geometry are not detected
    - **Acceptance:** line-of-sight blocking works
12. **US-7.6.1.12** — I want to benchmark sight sense evaluation for 500+ agents simultaneously
    - **Acceptance:** perception scales to server-side NPC counts. ---

## F-7.6.2 — Hearing Sense (Radius + Attenuation)

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.6.2.1  | designer (P-5)          | F-7.6.2  | R-7.6.2      |
| US-7.6.2.2  | designer (P-5)          | F-7.6.2  | R-7.6.2      |
| US-7.6.2.3  | designer (P-5)          | F-7.6.2  | R-7.6.2      |
| US-7.6.2.4  | player (P-23)           | F-7.6.2  | R-7.6.2      |
| US-7.6.2.5  | player (P-23)           | F-7.6.2  | R-7.6.2      |
| US-7.6.2.6  | player (P-23)           | F-7.6.2  | R-7.6.2      |
| US-7.6.2.7  | engine developer (P-26) | F-7.6.2  | R-7.6.2      |
| US-7.6.2.8  | engine developer (P-26) | F-7.6.2  | R-7.6.2      |
| US-7.6.2.9  | engine developer (P-26) | F-7.6.2  | R-7.6.2      |
| US-7.6.2.10 | engine tester (P-27)    | F-7.6.2  | R-7.6.2      |
| US-7.6.2.11 | engine tester (P-27)    | F-7.6.2  | R-7.6.2      |
| US-7.6.2.12 | engine tester (P-27)    | F-7.6.2  | R-7.6.2      |

1. **US-7.6.2.1** — I want to configure the hearing radius and attenuation curve per agent archetype
   - **Acceptance:** keen-eared scouts detect sounds farther than oblivious merchants
2. **US-7.6.2.2** — I want to configure how much intervening geometry attenuates sound
   - **Acceptance:** walls muffle footsteps realistically
3. **US-7.6.2.3** — I want to assign intensity values to sound types (whisper, footstep, gunfire,
   explosion)
   - **Acceptance:** louder sounds propagate farther and are detected with higher confidence
4. **US-7.6.2.4** — I want guards to investigate the sound of a thrown object
   - **Acceptance:** I can create distractions to lure them away from their posts
5. **US-7.6.2.5** — I want enemies to detect my footsteps when I run nearby
   - **Acceptance:** moving quietly matters for stealth
6. **US-7.6.2.6** — I want walls between me and a guard to muffle my footsteps
   - **Acceptance:** environmental geometry affects detection
7. **US-7.6.2.7** — I want to implement hearing as a spherical range check with distance attenuation
   and geometry occlusion
   - **Acceptance:** sound detection is physically plausible
8. **US-7.6.2.8** — I want to use simplified server-side collision for sound occlusion queries
   - **Acceptance:** hearing does not depend on the audio engine
9. **US-7.6.2.9** — I want to map stimulus intensity and distance to a detection confidence score
   - **Acceptance:** NPCs distinguish between a whisper nearby and an explosion far away
10. **US-7.6.2.10** — I want to verify that sound detection confidence decreases with distance
    according to the configured attenuation curve
    - **Acceptance:** distance scaling is correct
11. **US-7.6.2.11** — I want to verify that intervening geometry reduces sound detection confidence
    by the configured occlusion factor
    - **Acceptance:** walls block sound
12. **US-7.6.2.12** — I want to verify that a gunshot is detected at 3x the range of a footstep,
    matching configured intensity ratios
    - **Acceptance:** intensity scaling works. ---

## F-7.6.3 — Damage Sense

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.6.3.1  | designer (P-5)          | F-7.6.3  | R-7.6.3      |
| US-7.6.3.2  | designer (P-5)          | F-7.6.3  | R-7.6.3      |
| US-7.6.3.3  | designer (P-5)          | F-7.6.3  | R-7.6.3      |
| US-7.6.3.4  | player (P-23)           | F-7.6.3  | R-7.6.3      |
| US-7.6.3.5  | player (P-23)           | F-7.6.3  | R-7.6.3      |
| US-7.6.3.6  | player (P-23)           | F-7.6.3  | R-7.6.3      |
| US-7.6.3.7  | engine developer (P-26) | F-7.6.3  | R-7.6.3      |
| US-7.6.3.8  | engine developer (P-26) | F-7.6.3  | R-7.6.3      |
| US-7.6.3.9  | engine developer (P-26) | F-7.6.3  | R-7.6.3      |
| US-7.6.3.10 | engine tester (P-27)    | F-7.6.3  | R-7.6.3      |
| US-7.6.3.11 | engine tester (P-27)    | F-7.6.3  | R-7.6.3      |
| US-7.6.3.12 | engine tester (P-27)    | F-7.6.3  | R-7.6.3      |

1. **US-7.6.3.1** — I want to configure damage sense as a high-priority perception event that
   bypasses range and LOS
   - **Acceptance:** attacked NPCs always react
2. **US-7.6.3.2** — I want damage sense to report the direction and magnitude of incoming damage
   - **Acceptance:** NPCs turn toward their attacker
3. **US-7.6.3.3** — I want to set a minimum damage threshold before damage sense triggers
   - **Acceptance:** trivial chip damage does not break stealth
4. **US-7.6.3.4** — I want an attacked NPC to immediately turn toward the direction of the attack
   - **Acceptance:** damage reaction feels instant and aware
5. **US-7.6.3.5** — I want enemies to react to being shot from behind even if they cannot see me
   - **Acceptance:** damage always provokes a response
6. **US-7.6.3.6** — I want NPCs to ignore very minor environmental damage (e.g., a pebble)
   - **Acceptance:** the damage threshold prevents trivial alerts
7. **US-7.6.3.7** — I want to implement damage sense as a lightweight event-driven check that
   registers direction and magnitude
   - **Acceptance:** it bypasses the normal perception pipeline
8. **US-7.6.3.8** — I want damage events to bypass range and line-of-sight checks
   - **Acceptance:** NPCs always react to being attacked regardless of attacker visibility
9. **US-7.6.3.9** — I want damage sense to be a lightweight event check with identical behavior on
   all platforms
   - **Acceptance:** there are no platform-specific edge cases
10. **US-7.6.3.10** — I want to verify that damage sense fires for every damage event above the
    threshold regardless of attacker position
    - **Acceptance:** no attacks go unnoticed
11. **US-7.6.3.11** — I want to verify that the reported damage direction matches the actual
    attacker direction within a tolerance
    - **Acceptance:** NPCs turn correctly
12. **US-7.6.3.12** — I want to verify that damage below the configured threshold does not trigger
    damage sense
    - **Acceptance:** the filter works. ---

## F-7.6.4 — Team & Faction Awareness

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.6.4.1  | designer (P-5)          | F-7.6.4  | R-7.6.4      |
| US-7.6.4.2  | designer (P-5)          | F-7.6.4  | R-7.6.4      |
| US-7.6.4.3  | designer (P-5)          | F-7.6.4  | R-7.6.4      |
| US-7.6.4.4  | player (P-23)           | F-7.6.4  | R-7.6.4      |
| US-7.6.4.5  | player (P-23)           | F-7.6.4  | R-7.6.4      |
| US-7.6.4.6  | player (P-23)           | F-7.6.4  | R-7.6.4      |
| US-7.6.4.7  | engine developer (P-26) | F-7.6.4  | R-7.6.4      |
| US-7.6.4.8  | engine developer (P-26) | F-7.6.4  | R-7.6.4      |
| US-7.6.4.9  | engine developer (P-26) | F-7.6.4  | R-7.6.4      |
| US-7.6.4.10 | engine tester (P-27)    | F-7.6.4  | R-7.6.4      |
| US-7.6.4.11 | engine tester (P-27)    | F-7.6.4  | R-7.6.4      |
| US-7.6.4.12 | engine tester (P-27)    | F-7.6.4  | R-7.6.4      |

1. **US-7.6.4.1** — I want to configure a faction affinity table defining relationships (hostile,
   neutral, friendly) between all factions
   - **Acceptance:** NPC perception filters stimuli by faction
2. **US-7.6.4.2** — I want to override individual NPC faction relationships
   - **Acceptance:** a friendly NPC in a hostile faction behaves differently from its group
3. **US-7.6.4.3** — I want to modify faction relationships at runtime via gameplay events
   (reputation changes, betrayals)
   - **Acceptance:** the world reacts to player actions
4. **US-7.6.4.4** — I want guards to ignore friendly faction NPCs walking past them
   - **Acceptance:** perception correctly filters by affinity
5. **US-7.6.4.5** — I want NPCs to turn hostile after a reputation change, attacking me where they
   previously ignored me
   - **Acceptance:** faction dynamics feel responsive
6. **US-7.6.4.6** — I want hostile NPCs to prioritize hostile targets over neutral ones when both
   are visible
   - **Acceptance:** faction awareness drives targeting
7. **US-7.6.4.7** — I want to tag each perceived stimulus with the source entity's faction and
   relationship
   - **Acceptance:** perception filters can ignore or escalate based on affinity
8. **US-7.6.4.8** — I want to read faction relations from a shared affinity table that gameplay
   systems can modify at runtime
   - **Acceptance:** relationships are centralized and dynamic
9. **US-7.6.4.9** — I want gameplay systems to modify the affinity table at runtime (reputation,
   betrayals)
   - **Acceptance:** faction relationships evolve during gameplay
10. **US-7.6.4.10** — I want to verify that perception filters correctly ignore friendly stimuli
    when configured to do so
    - **Acceptance:** allies do not trigger alert states
11. **US-7.6.4.11** — I want to verify that changing a faction relationship at runtime immediately
    affects perception filtering for all agents of that faction
    - **Acceptance:** changes propagate
12. **US-7.6.4.12** — I want to verify that a per-NPC faction override takes precedence over the
    faction default
    - **Acceptance:** individual overrides work. ---

## F-7.6.5 — Stimuli Registration & Expiration

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.6.5.1  | designer (P-5)          | F-7.6.5  | R-7.6.5      |
| US-7.6.5.2  | designer (P-5)          | F-7.6.5  | R-7.6.5      |
| US-7.6.5.3  | designer (P-5)          | F-7.6.5  | R-7.6.5      |
| US-7.6.5.4  | player (P-23)           | F-7.6.5  | R-7.6.5      |
| US-7.6.5.5  | player (P-23)           | F-7.6.5  | R-7.6.5      |
| US-7.6.5.6  | player (P-23)           | F-7.6.5  | R-7.6.5      |
| US-7.6.5.7  | engine developer (P-26) | F-7.6.5  | R-7.6.5      |
| US-7.6.5.8  | engine developer (P-26) | F-7.6.5  | R-7.6.5      |
| US-7.6.5.9  | engine developer (P-26) | F-7.6.5  | R-7.6.5      |
| US-7.6.5.10 | engine tester (P-27)    | F-7.6.5  | R-7.6.5      |
| US-7.6.5.11 | engine tester (P-27)    | F-7.6.5  | R-7.6.5      |
| US-7.6.5.12 | engine tester (P-27)    | F-7.6.5  | R-7.6.5      |

1. **US-7.6.5.1** — I want gameplay systems to broadcast perception events (noise at position,
   visual flash, scent trail) to a global stimulus registry
   - **Acceptance:** AI can detect them
2. **US-7.6.5.2** — I want to configure the TTL, intensity, and radius of each stimulus type
   - **Acceptance:** events decay and propagate realistically
3. **US-7.6.5.3** — I want to spawn a stimulus in the editor and watch it expire after its TTL
   - **Acceptance:** I can verify expiration timing
4. **US-7.6.5.4** — I want guards to react to noises I create (breaking glass, knocking on doors)
   - **Acceptance:** the stimulus system connects my actions to AI responses
5. **US-7.6.5.5** — I want guards to stop searching after the stimulus expires
   - **Acceptance:** alerts have a natural time limit
6. **US-7.6.5.6** — I want multiple nearby noises to increase AI alertness additively
   - **Acceptance:** causing repeated disturbances eventually draws investigation
7. **US-7.6.5.7** — I want to implement a global stimulus registry with spatial queries and
   automatic TTL expiration
   - **Acceptance:** perception systems query stimuli efficiently
8. **US-7.6.5.8** — I want to cap active stimuli to 256 on mobile and 2048 on desktop
   - **Acceptance:** registry size fits within memory budgets
9. **US-7.6.5.9** — I want mobile to use shorter default TTLs to keep the registry compact
   - **Acceptance:** stale stimuli are cleaned up faster
10. **US-7.6.5.10** — I want to verify that stimuli are removed from the registry exactly when their
    TTL expires
    - **Acceptance:** expiration timing is correct
11. **US-7.6.5.11** — I want to verify that spatial queries return only stimuli within the query
    radius and not those outside
    - **Acceptance:** spatial filtering works
12. **US-7.6.5.12** — I want to stress test the registry at the platform stimulus cap and verify
    correct behavior
    - **Acceptance:** the cap is enforced without data loss. ---

## F-7.6.6 — Sense Aging & Memory Decay

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.6.6.1  | designer (P-5)          | F-7.6.6  | R-7.6.6      |
| US-7.6.6.2  | designer (P-5)          | F-7.6.6  | R-7.6.6      |
| US-7.6.6.3  | designer (P-5)          | F-7.6.6  | R-7.6.6      |
| US-7.6.6.4  | player (P-23)           | F-7.6.6  | R-7.6.6      |
| US-7.6.6.5  | player (P-23)           | F-7.6.6  | R-7.6.6      |
| US-7.6.6.6  | player (P-23)           | F-7.6.6  | R-7.6.6      |
| US-7.6.6.7  | engine developer (P-26) | F-7.6.6  | R-7.6.6      |
| US-7.6.6.8  | engine developer (P-26) | F-7.6.6  | R-7.6.6      |
| US-7.6.6.9  | engine developer (P-26) | F-7.6.6  | R-7.6.6      |
| US-7.6.6.10 | engine tester (P-27)    | F-7.6.6  | R-7.6.6      |
| US-7.6.6.11 | engine tester (P-27)    | F-7.6.6  | R-7.6.6      |
| US-7.6.6.12 | engine tester (P-27)    | F-7.6.6  | R-7.6.6      |

1. **US-7.6.6.1** — I want to configure how long each agent archetype retains perception memory
   - **Acceptance:** smart scouts remember longer than forgetful grunts
2. **US-7.6.6.2** — I want to set different decay rates per stimulus type (visual decays fast,
   damage decays slow)
   - **Acceptance:** memory priority matches threat severity
3. **US-7.6.6.3** — I want to see an agent's perception memory with confidence levels and age in a
   debug panel
   - **Acceptance:** I can tune decay parameters
4. **US-7.6.6.4** — I want guards to gradually lose certainty about my position when they can no
   longer see me
   - **Acceptance:** stealth rewards patience
5. **US-7.6.6.5** — I want AI to search my last known position, then gradually give up and return to
   patrol as memory decays
   - **Acceptance:** escape feels achievable
6. **US-7.6.6.6** — I want veteran NPCs to remember my position longer than novice ones
   - **Acceptance:** NPC quality differences are reflected in gameplay difficulty
7. **US-7.6.6.7** — I want to track stimulus age and decay confidence over time since last
   confirmation
   - **Acceptance:** stale perceptions fade to vague last-known data
8. **US-7.6.6.8** — I want to support per-archetype memory duration and decay rate profiles
   - **Acceptance:** NPC variety is expressed through perception quality
9. **US-7.6.6.9** — I want mobile to use faster decay rates and shorter maximum memory retention
   - **Acceptance:** per-agent memory list size stays small
10. **US-7.6.6.10** — I want to verify that stimulus confidence decays to zero at exactly the
    configured retention duration
    - **Acceptance:** decay timing is correct
11. **US-7.6.6.11** — I want to verify that re-confirming a stimulus (seeing the target again)
    resets confidence to maximum
    - **Acceptance:** refresh works
12. **US-7.6.6.12** — I want to verify that mobile builds use the configured faster decay rates and
    shorter retention
    - **Acceptance:** the platform optimization is active. ---

## F-7.6.7 — Custom Senses & Perception Priority

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.6.7.1  | designer (P-5)          | F-7.6.7  | R-7.6.7      |
| US-7.6.7.2  | designer (P-5)          | F-7.6.7  | R-7.6.7      |
| US-7.6.7.3  | designer (P-5)          | F-7.6.7  | R-7.6.7      |
| US-7.6.7.4  | player (P-23)           | F-7.6.7  | R-7.6.7      |
| US-7.6.7.5  | player (P-23)           | F-7.6.7  | R-7.6.7      |
| US-7.6.7.6  | player (P-23)           | F-7.6.7  | R-7.6.7      |
| US-7.6.7.7  | engine developer (P-26) | F-7.6.7  | R-7.6.7      |
| US-7.6.7.8  | engine developer (P-26) | F-7.6.7  | R-7.6.7      |
| US-7.6.7.9  | engine developer (P-26) | F-7.6.7  | R-7.6.7      |
| US-7.6.7.10 | engine tester (P-27)    | F-7.6.7  | R-7.6.7      |
| US-7.6.7.11 | engine tester (P-27)    | F-7.6.7  | R-7.6.7      |
| US-7.6.7.12 | engine tester (P-27)    | F-7.6.7  | R-7.6.7      |

1. **US-7.6.7.1** — I want to register project-specific senses (tremor sense, magic detection,
   thermal vision) through a trait interface
   - **Acceptance:** the perception system is extensible
2. **US-7.6.7.2** — I want each custom sense to declare its processing cost
   - **Acceptance:** the perception scheduler allocates budget appropriately
3. **US-7.6.7.3** — I want to configure the per-tick perception budget per platform
   - **Acceptance:** high-priority senses always run while low-priority ones defer gracefully
4. **US-7.6.7.4** — I want NPCs with tremor sense to detect me through the ground
   - **Acceptance:** unique senses create novel stealth challenges
5. **US-7.6.7.5** — I want magic-sensitive NPCs to see through illusions that fool normal guards
   - **Acceptance:** custom senses affect gameplay strategy
6. **US-7.6.7.6** — I want NPCs with low-priority custom senses to detect threats slightly slower
   - **Acceptance:** budget deferral does not cause jarring behavior
7. **US-7.6.7.7** — I want to implement a perception scheduler that allocates per-tick CPU budget to
   senses by priority, running high-priority first and deferring low-priority
   - **Acceptance:** perception scales within budget
8. **US-7.6.7.8** — I want to support runtime registration of custom sense types through a trait
   interface
   - **Acceptance:** gameplay code can add new perception modes
9. **US-7.6.7.9** — I want the mobile per-tick perception budget to be 0.25 ms (vs. 1 ms on desktop)
   - **Acceptance:** low-priority custom senses may tick every few seconds
10. **US-7.6.7.10** — I want to verify that high-priority senses are evaluated every tick even when
    the budget is tight
    - **Acceptance:** critical perception is never deferred
11. **US-7.6.7.11** — I want to verify that low-priority senses are deferred to subsequent ticks
    when the budget is exhausted
    - **Acceptance:** scheduling works correctly
12. **US-7.6.7.12** — I want to benchmark the number of custom sense evaluations per tick across
    platforms
    - **Acceptance:** the scheduler scales to the expected sense count. ---

## F-7.6.8 — Smell Sense and Scent Trails

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.6.8.1  | designer (P-5)          | F-7.6.8  | R-7.6.8      |
| US-7.6.8.2  | designer (P-5)          | F-7.6.8  | R-7.6.8      |
| US-7.6.8.3  | designer (P-5)          | F-7.6.8  | R-7.6.8      |
| US-7.6.8.4  | player (P-23)           | F-7.6.8  | R-7.6.8      |
| US-7.6.8.5  | player (P-23)           | F-7.6.8  | R-7.6.8      |
| US-7.6.8.6  | player (P-23)           | F-7.6.8  | R-7.6.8      |
| US-7.6.8.7  | engine developer (P-26) | F-7.6.8  | R-7.6.8      |
| US-7.6.8.8  | engine developer (P-26) | F-7.6.8  | R-7.6.8      |
| US-7.6.8.9  | engine developer (P-26) | F-7.6.8  | R-7.6.8      |
| US-7.6.8.10 | engine tester (P-27)    | F-7.6.8  | R-7.6.8      |
| US-7.6.8.11 | engine tester (P-27)    | F-7.6.8  | R-7.6.8      |
| US-7.6.8.12 | engine tester (P-27)    | F-7.6.8  | R-7.6.8      |

1. **US-7.6.8.1** — I want to configure which entities emit scent (player, food, blood, smoke) with
   type, intensity, and decay rate
   - **Acceptance:** the scent system matches my game's tracking needs
2. **US-7.6.8.2** — I want to set how long each scent type persists (blood trails last hours,
   footstep scent fades in minutes)
   - **Acceptance:** tracking difficulty varies by situation
3. **US-7.6.8.3** — I want to configure how wind affects scent propagation direction and speed
   - **Acceptance:** wind direction adds a strategic element to stealth
4. **US-7.6.8.4** — I want predator AI to track me by scent that drifts with the wind
   - **Acceptance:** I can stay downwind to avoid detection
5. **US-7.6.8.5** — I want tracking dogs to follow my scent trail by moving toward the strongest
   adjacent scent point
   - **Acceptance:** pursuit feels intelligent
6. **US-7.6.8.6** — I want rain to dilute my scent trail, making it harder for trackers
   - **Acceptance:** weather affects stealth gameplay
7. **US-7.6.8.7** — I want to implement a scent spatial grid with intensity-based propagation and
   wind drift
   - **Acceptance:** scent spreads realistically through the environment
8. **US-7.6.8.8** — I want to generate decaying scent points along a moving entity's path
   - **Acceptance:** trackers can follow the trail by seeking the strongest adjacent point
9. **US-7.6.8.9** — I want scent to be blocked by sealed doors and diluted by rain/water
   - **Acceptance:** environmental factors affect scent propagation
10. **US-7.6.8.10** — I want to verify that scent propagates in the downwind direction and is
    detectable at greater range downwind than upwind
    - **Acceptance:** wind drift works
11. **US-7.6.8.11** — I want to verify that each scent type decays at its configured rate and
    expires at the expected time
    - **Acceptance:** persistence tuning works
12. **US-7.6.8.12** — I want to verify that scent does not propagate through sealed doors
    - **Acceptance:** environmental blocking is enforced. ---

## F-7.6.9 — Environmental Evidence and Footprint Detection

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.6.9.1  | designer (P-5)          | F-7.6.9  | R-7.6.9      |
| US-7.6.9.2  | designer (P-5)          | F-7.6.9  | R-7.6.9      |
| US-7.6.9.3  | designer (P-5)          | F-7.6.9  | R-7.6.9      |
| US-7.6.9.4  | player (P-23)           | F-7.6.9  | R-7.6.9      |
| US-7.6.9.5  | player (P-23)           | F-7.6.9  | R-7.6.9      |
| US-7.6.9.6  | player (P-23)           | F-7.6.9  | R-7.6.9      |
| US-7.6.9.7  | engine developer (P-26) | F-7.6.9  | R-7.6.9      |
| US-7.6.9.8  | engine developer (P-26) | F-7.6.9  | R-7.6.9      |
| US-7.6.9.9  | engine developer (P-26) | F-7.6.9  | R-7.6.9      |
| US-7.6.9.10 | engine tester (P-27)    | F-7.6.9  | R-7.6.9      |
| US-7.6.9.11 | engine tester (P-27)    | F-7.6.9  | R-7.6.9      |
| US-7.6.9.12 | engine tester (P-27)    | F-7.6.9  | R-7.6.9      |

1. **US-7.6.9.1** — I want to configure which evidence types spawn (footprints, blood drops, broken
   vegetation, shell casings) and their spawn rates
   - **Acceptance:** evidence density matches performance and gameplay needs
2. **US-7.6.9.2** — I want to set decay timers per evidence type
   - **Acceptance:** footprints in snow last longer than blood on stone
3. **US-7.6.9.3** — I want to configure the query radius for AI with `TrackingSense`
   - **Acceptance:** tracking NPCs search an appropriate area for evidence
4. **US-7.6.9.4** — I want enemy tracking dogs to follow my footprints through snow and mud
   - **Acceptance:** I think about which surfaces I walk on to avoid leaving trails
5. **US-7.6.9.5** — I want enemies to find me by following blood drops I leave when injured
   - **Acceptance:** getting hurt has consequences beyond HP loss
6. **US-7.6.9.6** — I want AI trackers to correctly follow my footprints in the direction I traveled
   - **Acceptance:** tracking looks intelligent
7. **US-7.6.9.7** — I want to implement evidence as ECS entities with type, timestamp, source,
   location, and decay timer
   - **Acceptance:** evidence is queryable and expires
8. **US-7.6.9.8** — I want the character controller to spawn footprint evidence when moving over
   deformable terrain materials
   - **Acceptance:** footprints appear automatically
9. **US-7.6.9.9** — I want to throttle evidence spawning to one footprint per N steps
   - **Acceptance:** evidence count stays within the entity budget
10. **US-7.6.9.10** — I want to verify that footprints spawn only on deformable terrain materials
    (snow, mud, sand) and not on hard surfaces
    - **Acceptance:** material detection works
11. **US-7.6.9.11** — I want to verify that evidence entities are removed when their decay timer
    expires
    - **Acceptance:** no stale evidence persists
12. **US-7.6.9.12** — I want to verify that AI with TrackingSense correctly infers direction of
    travel from footprint orientation
    - **Acceptance:** tracking direction is accurate. ---

## F-7.6.10 — AI Investigation Behavior

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-7.6.10.1  | designer (P-5)          | F-7.6.10 | R-7.6.10     |
| US-7.6.10.2  | designer (P-5)          | F-7.6.10 | R-7.6.10     |
| US-7.6.10.3  | designer (P-5)          | F-7.6.10 | R-7.6.10     |
| US-7.6.10.4  | player (P-23)           | F-7.6.10 | R-7.6.10     |
| US-7.6.10.5  | player (P-23)           | F-7.6.10 | R-7.6.10     |
| US-7.6.10.6  | player (P-23)           | F-7.6.10 | R-7.6.10     |
| US-7.6.10.7  | engine developer (P-26) | F-7.6.10 | R-7.6.10     |
| US-7.6.10.8  | engine developer (P-26) | F-7.6.10 | R-7.6.10     |
| US-7.6.10.9  | engine developer (P-26) | F-7.6.10 | R-7.6.10     |
| US-7.6.10.10 | engine tester (P-27)    | F-7.6.10 | R-7.6.10     |
| US-7.6.10.11 | engine tester (P-27)    | F-7.6.10 | R-7.6.10     |
| US-7.6.10.12 | engine tester (P-27)    | F-7.6.10 | R-7.6.10     |

1. **US-7.6.10.1** — I want to configure investigation behavior per stimulus type (sound: move to
   origin; scent: follow trail; visual: move to last seen position)
   - **Acceptance:** each stimulus produces the appropriate investigation
2. **US-7.6.10.2** — I want to set a timeout after which investigation ends and the agent returns to
   unaware state
   - **Acceptance:** NPCs do not search indefinitely
3. **US-7.6.10.3** — I want the first agent to claim a stimulus for investigation while others
   continue patrol
   - **Acceptance:** only one NPC investigates each event
4. **US-7.6.10.4** — I want guards to investigate suspicious sounds by walking to the source,
   looking around, and listening
   - **Acceptance:** I can use distractions to create openings
5. **US-7.6.10.5** — I want AI investigating a scent to follow the trail toward increasing intensity
   - **Acceptance:** scent-based investigation looks methodical
6. **US-7.6.10.6** — I want an investigating AI to escalate to alerted state when it finds
   confirming evidence
   - **Acceptance:** investigation can lead to full combat engagement
7. **US-7.6.10.7** — I want to integrate investigation with the AI alert state machine (unaware ->
   suspicious -> alerted -> unaware)
   - **Acceptance:** investigation drives state transitions
8. **US-7.6.10.8** — I want to select investigation behavior dynamically based on stimulus type
   - **Acceptance:** each sense triggers the appropriate search pattern
9. **US-7.6.10.9** — I want mobile to use simplified investigation (skip scent/footprint following,
   use direct move-to-position only)
   - **Acceptance:** investigation cost fits the mobile budget
10. **US-7.6.10.10** — I want to verify that only one agent claims a stimulus for investigation and
    others continue patrol
    - **Acceptance:** coordination prevents redundancy
11. **US-7.6.10.11** — I want to verify the full state machine cycle (unaware -> suspicious ->
    alerted -> unaware) during investigation
    - **Acceptance:** transitions are correct
12. **US-7.6.10.12** — I want to verify that an investigation that finds nothing ends after the
    configured timeout and the agent returns to patrol
    - **Acceptance:** timeouts work. ---

## F-7.6.11 — Multi-Sense Tracking and Pursuit

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-7.6.11.1  | designer (P-5)          | F-7.6.11 | R-7.6.11     |
| US-7.6.11.2  | designer (P-5)          | F-7.6.11 | R-7.6.11     |
| US-7.6.11.3  | designer (P-5)          | F-7.6.11 | R-7.6.11     |
| US-7.6.11.4  | player (P-23)           | F-7.6.11 | R-7.6.11     |
| US-7.6.11.5  | player (P-23)           | F-7.6.11 | R-7.6.11     |
| US-7.6.11.6  | player (P-23)           | F-7.6.11 | R-7.6.11     |
| US-7.6.11.7  | engine developer (P-26) | F-7.6.11 | R-7.6.11     |
| US-7.6.11.8  | engine developer (P-26) | F-7.6.11 | R-7.6.11     |
| US-7.6.11.9  | engine developer (P-26) | F-7.6.11 | R-7.6.11     |
| US-7.6.11.10 | engine tester (P-27)    | F-7.6.11 | R-7.6.11     |
| US-7.6.11.11 | engine tester (P-27)    | F-7.6.11 | R-7.6.11     |
| US-7.6.11.12 | engine tester (P-27)    | F-7.6.11 | R-7.6.11     |

1. **US-7.6.11.1** — I want to enable or disable tracking methods (visual, audio, scent, evidence,
   predictive) per agent archetype
   - **Acceptance:** tracking sophistication matches the NPC's role
2. **US-7.6.11.2** — I want to set confidence thresholds for transitioning between tracking methods
   and transitioning to search patterns
   - **Acceptance:** tracking intensity is tunable
3. **US-7.6.11.3** — I want to enable pack tracking so multiple AI agents share tracking data and
   one agent's sighting updates all pack members
   - **Acceptance:** group pursuit is coordinated
4. **US-7.6.11.4** — I want a pursuing enemy to switch from visual to audio tracking when I run
   behind a wall
   - **Acceptance:** breaking line of sight does not end pursuit immediately
5. **US-7.6.11.5** — I want a tracker to switch to following my scent trail when I stop making noise
   - **Acceptance:** escape requires countering multiple detection methods
6. **US-7.6.11.6** — I want a wolf pack where one wolf's sighting updates all pack members' pursuit
   direction
   - **Acceptance:** pack hunting feels coordinated and dangerous
7. **US-7.6.11.7** — I want to implement a `TrackingState` that maintains last confirmed position,
   estimated current position, confidence, and active tracking method
   - **Acceptance:** tracking is stateful
8. **US-7.6.11.8** — I want to implement seamless transitions between tracking methods based on
   sense availability and confidence
   - **Acceptance:** the tracker adapts to changing conditions
9. **US-7.6.11.9** — I want mobile to limit tracking to sight and hearing only, disabling
   scent/evidence/predictive tracking
   - **Acceptance:** tracking cost fits the mobile budget
10. **US-7.6.11.10** — I want to verify that the tracker transitions from visual to audio to scent
    as each sense loses the target
    - **Acceptance:** sense fallback works
11. **US-7.6.11.11** — I want to verify that tracking confidence decaying to zero transitions the
    tracker to search patterns
    - **Acceptance:** failed tracking ends gracefully
12. **US-7.6.11.12** — I want to verify that one pack member's sighting updates the estimated
    position for all pack members within the group
    - **Acceptance:** data sharing works
