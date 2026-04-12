# User Stories -- Stealth and Cover (13.18)

## Stealth

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.18.1.1  | game designer (P-5)     |
| US-13.18.1.2  | game developer (P-15)   |
| US-13.18.1.3  | player (P-23)           |
| US-13.18.1.4  | player (P-23)           |
| US-13.18.2.1  | game designer (P-5)     |
| US-13.18.2.2  | game developer (P-15)   |
| US-13.18.2.3  | player (P-23)           |
| US-13.18.3.1  | game designer (P-5)     |
| US-13.18.3.2  | player (P-23)           |
| US-13.18.3.3  | player (P-23)           |
| US-13.18.4.1  | game designer (P-5)     |
| US-13.18.4.2  | player (P-23)           |
| US-13.18.4.3  | player (P-23)           |

1. **US-13.18.1.1** -- **As a** game designer (P-5), **I want** to configure visibility modifiers
   per equipment piece, posture, and ability, **so that** I balance stealth across loadouts.

2. **US-13.18.1.2** -- **As a** game developer (P-15), **I want** per-frame visibility score
   computed from ambient light, shadow state, movement speed, and equipment, **so that** detection
   is environmentally grounded.

3. **US-13.18.1.3** -- [game-specific] **As a** player (P-23), **I want** crouching and staying
   stationary to reduce my visibility, **so that** posture and stillness are viable stealth tactics.

4. **US-13.18.1.4** -- [game-specific] **As a** player (P-23), **I want** a stealth HUD indicator
   showing my current visibility level, **so that** I know how detectable I am at any moment.

5. **US-13.18.2.1** -- **As a** game designer (P-5), **I want** to configure detection duration
   thresholds and perception sensitivity per AI state, **so that** I tune alert behavior per enemy
   type.

6. **US-13.18.2.2** -- **As a** game developer (P-15), **I want** AI to transition through unaware,
   suspicious, searching, alerted, and lost- target states with hysteresis, **so that** detection is
   gradual and flicker-free.

7. **US-13.18.2.3** -- [game-specific] **As a** player (P-23), **I want** question mark and
   exclamation mark icons above enemy heads indicating awareness state, **so that** I gauge
   detection at a glance.

8. **US-13.18.3.1** -- **As a** game designer (P-5), **I want** to configure noise intensity per
   action type and per weapon attachment, **so that** I balance the stealth economy via data.

9. **US-13.18.3.2** -- [game-specific] **As a** player (P-23), **I want** to throw distraction
   objects to create noise at the impact location, **so that** I lure AI away from patrol routes.

10. **US-13.18.3.3** -- [game-specific] **As a** player (P-23), **I want** closed doors and thick
    walls to attenuate noise, **so that** I use architecture to mask my sounds.

11. **US-13.18.4.1** -- **As a** game designer (P-5), **I want** to configure takedown types, noise
    levels, and position requirements per takedown, **so that** stealth action options are
    data-driven.

12. **US-13.18.4.2** -- [game-specific] **As a** player (P-23), **I want** to perform a silent
    takedown on an unaware enemy from behind, **so that** I eliminate threats without alerting
    others.

13. **US-13.18.4.3** -- [game-specific] **As a** player (P-23), **I want** to pick up and hide
    unconscious or dead bodies, **so that** I prevent other AI from discovering my handiwork.

## Cover

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.18.5.1  | game designer (P-5)     |
| US-13.18.5.2  | game developer (P-15)   |
| US-13.18.5.3  | player (P-23)           |
| US-13.18.5.4  | player (P-23)           |

1. **US-13.18.5.1** -- **As a** game designer (P-5), **I want** cover points automatically
   identified from world geometry with half, full, and directional classification, **so that** level
   geometry doubles as combat infrastructure.

2. **US-13.18.5.2** -- **As a** game developer (P-15), **I want** AI agents to use the same cover
   point system with scoring-based selection, **so that** AI cover behavior is consistent with the
   player.

3. **US-13.18.5.3** -- [game-specific] **As a** player (P-23), **I want** to snap to cover via input
   action and peek left/right to aim and shoot with partial exposure, **so that** cover lets me
   fight with reduced risk.

4. **US-13.18.5.4** -- [game-specific] **As a** player (P-23), **I want** cover-to-cover sprint
   between adjacent cover points, **so that** I can reposition under fire.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-13.18.1 | game designer (P-5) |
| US-13.18.2 | game designer (P-5) |
| US-13.18.3 | game designer (P-5) |
| US-13.18.4 | game designer (P-5) |
| US-13.18.5 | game designer (P-5) |

1. **US-13.18.1** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.18.1.1 through US-13.18.1.4 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

2. **US-13.18.2** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.18.2.1 through US-13.18.2.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

3. **US-13.18.3** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.18.3.1 through US-13.18.3.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

4. **US-13.18.4** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.18.4.1 through US-13.18.4.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

5. **US-13.18.5** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.18.5.1 through US-13.18.5.4 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.
