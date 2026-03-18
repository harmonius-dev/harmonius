# User Stories -- Stealth and Cover (13.18)

## Player Visibility and Stealth (F-13.18.1)

| ID           | Persona        | Features  | Requirements |
|--------------|----------------|-----------|--------------|
| US-13.18.1.1 | player (P-23)  | F-13.18.1 | R-13.18.1    |
| US-13.18.1.2 | player (P-23)  | F-13.18.1 | R-13.18.1    |
| US-13.18.1.3 | player (P-23)  | F-13.18.1 | R-13.18.1    |
| US-13.18.1.4 | player (P-23)  | F-13.18.1 | R-13.18.1    |
| US-13.18.1.5 | designer (P-5) | F-13.18.1 | R-13.18.1    |
| US-13.18.1.6 | designer (P-5) | F-13.18.1 | R-13.18.1    |
| US-13.18.1.7 | tester (P-27)  | F-13.18.1 | R-13.18.1    |
| US-13.18.1.8 | tester (P-27)  | F-13.18.1 | R-13.18.1    |

1. **US-13.18.1.1** — **As a** player (P-23), **I want** my visibility score computed from ambient
   light, shadow state, movement speed, and equipment, **so that** stealth depends on environmental
   conditions.
2. **US-13.18.1.2** — **As a** player (P-23), **I want** crouching and staying stationary to reduce
   my visibility, **so that** posture and stillness are viable stealth tactics.
3. **US-13.18.1.3** — **As a** player (P-23), **I want** dark clothing to reduce visibility and
   reflective armor to increase it, **so that** gear choice affects stealth capability.
4. **US-13.18.1.4** — **As a** player (P-23), **I want** a stealth HUD indicator showing my current
   visibility level, **so that** I know how detectable I am at any moment.
5. **US-13.18.1.5** — **As a** designer (P-5), **I want** to configure visibility modifiers per
   equipment piece, posture, and ability, **so that** I can balance stealth across loadouts.
6. **US-13.18.1.6** — **As a** designer (P-5), **I want** AI detection range scaled by the
   visibility score, **so that** darkness forces enemies closer before detecting the player.
7. **US-13.18.1.7** — **As a** tester (P-27), **I want** to verify that an invisibility ability sets
   visibility to zero, **so that** the ability override functions correctly.
8. **US-13.18.1.8** — **As a** tester (P-27), **I want** to verify that standing in a shadow reduces
   detection range by the configured multiplier, **so that** light-based stealth works correctly.

## AI Alert State Machine (F-13.18.2)

| ID           | Persona        | Features  | Requirements |
|--------------|----------------|-----------|--------------|
| US-13.18.2.1 | player (P-23)  | F-13.18.2 | R-13.18.2    |
| US-13.18.2.2 | player (P-23)  | F-13.18.2 | R-13.18.2    |
| US-13.18.2.3 | player (P-23)  | F-13.18.2 | R-13.18.2    |
| US-13.18.2.4 | designer (P-5) | F-13.18.2 | R-13.18.2    |
| US-13.18.2.5 | tester (P-27)  | F-13.18.2 | R-13.18.2    |
| US-13.18.2.6 | tester (P-27)  | F-13.18.2 | R-13.18.2    |

1. **US-13.18.2.1** — **As a** player (P-23), **I want** AI enemies to transition through unaware,
   suspicious, searching, alerted, and lost-target states, **so that** detection is gradual and
   readable.
2. **US-13.18.2.2** — **As a** player (P-23), **I want** question mark and exclamation mark icons
   above enemy heads indicating their awareness state, **so that** I can gauge detection at a
   glance.
3. **US-13.18.2.3** — **As a** player (P-23), **I want** AI to require sustained detection before
   transitioning from suspicious to alerted, **so that** brief glimpses do not trigger instant
   combat.
4. **US-13.18.2.4** — **As a** designer (P-5), **I want** to configure detection duration
   thresholds, perception sensitivity per state, and behavior tree subtree mapping per state,
   **so that** I can tune alert behavior per enemy type.
5. **US-13.18.2.5** — **As a** tester (P-27), **I want** to verify that an AI in "searching" state
   returns to patrol after the configured timeout, **so that** the lost-target transition works
   correctly.
6. **US-13.18.2.6** — **As a** tester (P-27), **I want** to verify that hysteresis prevents instant
   transitions between suspicious and alerted, **so that** flickering is eliminated.

## Noise Generation and Distraction (F-13.18.3)

| ID           | Persona        | Features  | Requirements |
|--------------|----------------|-----------|--------------|
| US-13.18.3.1 | player (P-23)  | F-13.18.3 | R-13.18.3    |
| US-13.18.3.2 | player (P-23)  | F-13.18.3 | R-13.18.3    |
| US-13.18.3.3 | player (P-23)  | F-13.18.3 | R-13.18.3    |
| US-13.18.3.4 | player (P-23)  | F-13.18.3 | R-13.18.3    |
| US-13.18.3.5 | designer (P-5) | F-13.18.3 | R-13.18.3    |
| US-13.18.3.6 | tester (P-27)  | F-13.18.3 | R-13.18.3    |
| US-13.18.3.7 | tester (P-27)  | F-13.18.3 | R-13.18.3    |

1. **US-13.18.3.1** — **As a** player (P-23), **I want** my actions to generate noise at varying
   intensity (low for doors, medium for sprinting, high for gunfire), **so that** careless play
   risks detection.
2. **US-13.18.3.2** — **As a** player (P-23), **I want** to throw distraction objects like rocks or
   bottles to create noise at the impact location, **so that** I can lure AI away from patrol
   routes.
3. **US-13.18.3.3** — **As a** player (P-23), **I want** closed doors and thick walls to attenuate
   noise, **so that** I can use architecture to mask my sounds.
4. **US-13.18.3.4** — **As a** player (P-23), **I want** silenced weapons to reduce gunfire noise
   intensity, **so that** suppressors have stealth utility beyond cosmetics.
5. **US-13.18.3.5** — **As a** designer (P-5), **I want** to configure noise intensity per action
   type and per weapon attachment, **so that** I can balance the stealth economy.
6. **US-13.18.3.6** — **As a** tester (P-27), **I want** to verify that noise propagation respects
   sound occlusion through walls, **so that** architecture-based noise masking works correctly.
7. **US-13.18.3.7** — **As a** tester (P-27), **I want** to verify that a thrown distraction object
   causes the nearest AI to investigate the impact point, **so that** distraction works as intended.

## Stealth Takedown (F-13.18.4)

| ID           | Persona        | Features  | Requirements |
|--------------|----------------|-----------|--------------|
| US-13.18.4.1 | player (P-23)  | F-13.18.4 | R-13.18.4    |
| US-13.18.4.2 | player (P-23)  | F-13.18.4 | R-13.18.4    |
| US-13.18.4.3 | player (P-23)  | F-13.18.4 | R-13.18.4    |
| US-13.18.4.4 | designer (P-5) | F-13.18.4 | R-13.18.4    |
| US-13.18.4.5 | tester (P-27)  | F-13.18.4 | R-13.18.4    |
| US-13.18.4.6 | tester (P-27)  | F-13.18.4 | R-13.18.4    |

1. **US-13.18.4.1** — **As a** player (P-23), **I want** to perform a silent takedown on an unaware
   enemy from behind, **so that** I can eliminate threats without alerting others.
2. **US-13.18.4.2** — **As a** player (P-23), **I want** takedown type to vary between silent, loud,
   and non-lethal, **so that** I have stealth options matching different situations.
3. **US-13.18.4.3** — **As a** player (P-23), **I want** to pick up and hide unconscious or dead
   bodies, **so that** I can prevent other AI from discovering my handiwork.
4. **US-13.18.4.4** — **As a** designer (P-5), **I want** to configure takedown types, noise levels,
   and position requirements per takedown, **so that** I can create diverse stealth action options.
5. **US-13.18.4.5** — **As a** tester (P-27), **I want** to verify that a loud takedown alerts
   nearby AI within the configured noise radius, **so that** the noise system integrates with
   takedowns.
6. **US-13.18.4.6** — **As a** tester (P-27), **I want** to verify that a takedown requires the
   player behind the target and the target in unaware state, **so that** preconditions are enforced.

## Cover Point Detection and Usage (F-13.18.5)

| ID           | Persona        | Features  | Requirements |
|--------------|----------------|-----------|--------------|
| US-13.18.5.1 | player (P-23)  | F-13.18.5 | R-13.18.5    |
| US-13.18.5.2 | player (P-23)  | F-13.18.5 | R-13.18.5    |
| US-13.18.5.3 | player (P-23)  | F-13.18.5 | R-13.18.5    |
| US-13.18.5.4 | player (P-23)  | F-13.18.5 | R-13.18.5    |
| US-13.18.5.5 | designer (P-5) | F-13.18.5 | R-13.18.5    |
| US-13.18.5.6 | designer (P-5) | F-13.18.5 | R-13.18.5    |
| US-13.18.5.7 | tester (P-27)  | F-13.18.5 | R-13.18.5    |
| US-13.18.5.8 | tester (P-27)  | F-13.18.5 | R-13.18.5    |

1. **US-13.18.5.1** — **As a** player (P-23), **I want** to snap to cover via an input action with a
   smooth transition, **so that** I can quickly take cover during firefights.
2. **US-13.18.5.2** — **As a** player (P-23), **I want** to peek left or right from cover to aim and
   shoot with partial exposure, **so that** cover lets me fight with reduced risk.
3. **US-13.18.5.3** — **As a** player (P-23), **I want** to blind fire over or around cover with
   reduced accuracy, **so that** I can suppress enemies without exposing myself.
4. **US-13.18.5.4** — **As a** player (P-23), **I want** cover-to-cover sprint between adjacent
   cover points, **so that** I can reposition under fire.
5. **US-13.18.5.5** — **As a** designer (P-5), **I want** cover points automatically identified from
   world geometry with half and full cover classification, **so that** level geometry doubles as
   combat infrastructure.
6. **US-13.18.5.6** — **As a** designer (P-5), **I want** directional cover that protects from
   specific angles, **so that** flanking negates cover bonuses.
7. **US-13.18.5.7** — **As a** tester (P-27), **I want** to verify that AI agents use the same cover
   system and select cover points by scoring, **so that** AI cover behavior is consistent with the
   player.
8. **US-13.18.5.8** — **As a** tester (P-27), **I want** to verify that flanking an enemy in cover
   negates their cover bonus, **so that** directional cover is correctly computed.
