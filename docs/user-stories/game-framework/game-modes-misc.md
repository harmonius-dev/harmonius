# User Stories -- Game Modes and Systems (13.24)

## Wave Spawning System (F-13.24.1)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.24.1.1 | player (P-23) | **As a** player (P-23), **I want** enemy waves to spawn with escalating difficulty, **so that** survival and arena modes provide increasing challenge. |  | F-13.24.1 | R-13.24.1 |
| US-13.24.1.2 | player (P-23) | **As a** player (P-23), **I want** wave state (current wave, enemies remaining, next wave timer) displayed in the HUD, **so that** I can track progress. |  | F-13.24.1 | R-13.24.1 |
| US-13.24.1.3 | designer (P-5) | **As a** designer (P-5), **I want** to configure enemy composition, spawn points, spawn timing, inter-wave delay, and difficulty scaling per wave, **so that** wave design is data-driven. |  | F-13.24.1 | R-13.24.1 |
| US-13.24.1.4 | designer (P-5) | **As a** designer (P-5), **I want** endless mode to procedurally generate waves using escalation rules, **so that** infinite play is supported. |  | F-13.24.1 | R-13.24.1 |
| US-13.24.1.5 | creative director (P-2) | **As a** creative director (P-2), **I want** boss waves triggered at configurable intervals, **so that** marquee enemies appear at climactic moments. |  | F-13.24.1 | R-13.24.1 |
| US-13.24.1.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that waves spawn at the configured intervals with the correct enemy types, **so that** wave definitions execute accurately. |  | F-13.24.1 | R-13.24.1 |
## Tower Targeting and Upgrade (F-13.24.2)
| US-13.24.2.1 | player (P-23) | **As a** player (P-23), **I want** towers to auto-fire at targets using configurable targeting modes (nearest, strongest, fastest), **so that** tower placement is strategic. |  | F-13.24.2 | R-13.24.2 |
| US-13.24.2.2 | player (P-23) | **As a** player (P-23), **I want** to upgrade towers along linear or branching paths with per-tier stat improvements, **so that** towers evolve during the match. |  | F-13.24.2 | R-13.24.2 |
| US-13.24.2.3 | designer (P-5) | **As a** designer (P-5), **I want** to define tower stats, targeting modes, upgrade tiers, and costs in gameplay databases, **so that** tower design is data-driven. |  | F-13.24.2 | R-13.24.2 |
| US-13.24.2.4 | designer (P-5) | **As a** designer (P-5), **I want** tower roles (damage, support, slow, spawner) differentiated by ability and effect, **so that** tower placement decisions are meaningful. |  | F-13.24.2 | R-13.24.2 |
| US-13.24.2.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a tower set to "nearest" always targets the closest enemy, **so that** targeting mode logic is correct. |  | F-13.24.2 | R-13.24.2 |
## Score and Combo System (F-13.24.3)
| US-13.24.3.1 | player (P-23) | **As a** player (P-23), **I want** combo counters to increment on successive scoring events within a timing window and multiply points, **so that** sustained performance is rewarded. |  | F-13.24.3 | R-13.24.3 |
| US-13.24.3.2 | player (P-23) | **As a** player (P-23), **I want** a final grade (S/A/B/C/D or stars) based on total score thresholds, **so that** I have a clear performance rating. |  | F-13.24.3 | R-13.24.3 |
| US-13.24.3.3 | designer (P-5) | **As a** designer (P-5), **I want** to configure per-event point values, combo window, and grade thresholds, **so that** scoring is tunable per game mode. |  | F-13.24.3 | R-13.24.3 |
| US-13.24.3.4 | creative director (P-2) | **As a** creative director (P-2), **I want** the scoring system usable across action, rhythm, racing, and arcade modes, **so that** scoring is a reusable framework. |  | F-13.24.3 | R-13.24.3 |
| US-13.24.3.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that the combo resets after the window expires, **so that** the timing window is enforced. |  | F-13.24.3 | R-13.24.3 |
## Feedback Stack Asset (F-13.24.4a)
| US-13.24.4 | player (P-23) | **As a** player (P-23), **I want** gameplay events like hit confirmation and level up to trigger multi-sensory feedback, **so that** impactful moments feel satisfying. |  | F-13.24.4 | R-13.24.4 |
| US-13.24.4 | designer (P-5) | **As a** designer (P-5), **I want** to compose feedback stacks from reusable entries with timing, intensity, and execution flags, **so that** I iterate on game feel without code. |  | F-13.24.4 | R-13.24.4 |
| US-13.24.4 | creative director (P-2) | **As a** creative director (P-2), **I want** feedback stacks authored visually in the asset editor, **so that** game feel polish is a creative workflow. |  | F-13.24.4 | R-13.24.4 |
| US-13.24.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a feedback stack with parallel entries fires all entries simultaneously, **so that** execution mode works correctly. |  | F-13.24.4 | R-13.24.4 |
## Hit-Stop and Time Scale (F-13.24.4b)
| US-13.24.4 | player (P-23) | **As a** player (P-23), **I want** hit-stop to freeze the action briefly on hit confirmation, **so that** impactful hits have dramatic emphasis. |  | F-13.24.4 | R-13.24.4 |
| US-13.24.4 | player (P-23) | **As a** player (P-23), **I want** slow-motion ramps during key moments like finishing blows, **so that** climactic actions feel cinematic. |  | F-13.24.4 | R-13.24.4 |
| US-13.24.4 | designer (P-5) | **As a** designer (P-5), **I want** to configure freeze frame count and time scale curves as feedback entries, **so that** time manipulation is tunable. |  | F-13.24.4 | R-13.24.4 |
| US-13.24.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that per-entity time scale overrides allow one entity at normal speed while others are slowed, **so that** bullet-time works correctly. |  | F-13.24.4 | R-13.24.4 |
| US-13.24.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that time scale effects are local-only in networked games, **so that** multiplayer timing is not affected. |  | F-13.24.4 | R-13.24.4 |
## Feedback Entry Types Library (F-13.24.4c)
| US-13.24.4 | player (P-23) | **As a** player (P-23), **I want** camera shake, haptic pulses, particle bursts, and screen flashes to fire together on impactful events, **so that** feedback is multi-sensory. |  | F-13.24.4 | R-13.24.4 |
| US-13.24.4 | designer (P-5) | **As a** designer (P-5), **I want** a library of built-in feedback types with configurable parameter schemas, **so that** I can build effects from standard building blocks. |  | F-13.24.4 | R-13.24.4 |
| US-13.24.4 | designer (P-5) | **As a** designer (P-5), **I want** new entry types registerable via the extensibility API, **so that** custom feedback effects are possible. |  | F-13.24.4 | R-13.24.4 |
| US-13.24.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that each feedback type (shake, haptic, particle, sound, flash) triggers independently, **so that** all types function correctly. |  | F-13.24.4 | R-13.24.4 |
## Fast Travel Network (F-13.24.5)
| US-13.24.5.1 | player (P-23) | **As a** player (P-23), **I want** to discover waypoints through world exploration and fast travel between unlocked ones, **so that** backtracking is reduced. |  | F-13.24.5 | R-13.24.5 |
| US-13.24.5.2 | player (P-23) | **As a** player (P-23), **I want** to select a destination from the map UI with names, icons, and optional travel costs, **so that** I can choose my destination easily. |  | F-13.24.5 | R-13.24.5 |
| US-13.24.5.3 | designer (P-5) | **As a** designer (P-5), **I want** to configure waypoint discovery triggers, travel costs, and cooldowns, **so that** I can balance fast travel convenience. |  | F-13.24.5 | R-13.24.5 |
| US-13.24.5.4 | level designer (P-6) | **As a** level designer (P-6), **I want** to place waypoint entities with discovery trigger volumes, **so that** fast travel points integrate with world design. |  | F-13.24.5 | R-13.24.5 |
| US-13.24.5.5 | creative director (P-2) | **As a** creative director (P-2), **I want** hearthstone/recall abilities to teleport to a bound waypoint, **so that** players have a home point. |  | F-13.24.5 | R-13.24.5 |
| US-13.24.5.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that undiscovered waypoints do not appear in the travel menu, **so that** discovery gating is enforced. |  | F-13.24.5 | R-13.24.5 |
## Respawn and Graveyard System (F-13.24.6)
| US-13.24.6.1 | player (P-23) | **As a** player (P-23), **I want** to respawn at the nearest unlocked graveyard on death, **so that** I resume play near where I died. |  | F-13.24.6 | R-13.24.6 |
| US-13.24.6.2 | player (P-23) | **As a** player (P-23), **I want** an optional spirit form to navigate back to my corpse for revival, **so that** death has a recovery mechanic. |  | F-13.24.6 | R-13.24.6 |
| US-13.24.6.3 | player (P-23) | **As a** player (P-23), **I want** resurrection sickness to apply temporary stat penalties after respawning, **so that** death has consequences. |  | F-13.24.6 | R-13.24.6 |
| US-13.24.6.4 | designer (P-5) | **As a** designer (P-5), **I want** to configure respawn timers per game mode (instant in casual, delayed in competitive), **so that** death penalty fits the mode. |  | F-13.24.6 | R-13.24.6 |
| US-13.24.6.5 | creative director (P-2) | **As a** creative director (P-2), **I want** other players to be able to resurrect fallen players using abilities, **so that** teamwork bypasses the graveyard system. |  | F-13.24.6 | R-13.24.6 |
| US-13.24.6.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that the system selects the nearest unlocked graveyard to the death location, **so that** graveyard selection is correct. |  | F-13.24.6 | R-13.24.6 |
## Tutorial Step System (F-13.24.7a)
| US-13.24.7 | player (P-23) | **As a** player (P-23), **I want** tutorials to guide me with sequential steps that resume at the correct step after interruption, **so that** onboarding is smooth. |  | F-13.24.7 | R-13.24.7 |
| US-13.24.7 | player (P-23) | **As a** player (P-23), **I want** a skip option to dismiss the entire tutorial, **so that** experienced players are not forced through onboarding. |  | F-13.24.7 | R-13.24.7 |
| US-13.24.7 | designer (P-5) | **As a** designer (P-5), **I want** to author tutorial steps as data assets with trigger conditions, instruction text, and completion actions, **so that** tutorials are data-driven. |  | F-13.24.7 | R-13.24.7 |
| US-13.24.7 | creative director (P-2) | **As a** creative director (P-2), **I want** tutorial steps to execute through logic graphs for custom completion conditions, **so that** tutorials can require complex interactions. |  | F-13.24.7 | R-13.24.7 |
| US-13.24.7 | tester (P-27) | **As a** tester (P-27), **I want** to verify that tutorial progress persists across sessions, **so that** interrupted tutorials resume correctly. |  | F-13.24.7 | R-13.24.7 |
## Tutorial Visual Overlays (F-13.24.7b)
| US-13.24.7 | player (P-23) | **As a** player (P-23), **I want** spotlight overlays and arrow callouts highlighting the relevant control or object, **so that** I know exactly where to look. |  | F-13.24.7 | R-13.24.7 |
| US-13.24.7 | player (P-23) | **As a** player (P-23), **I want** forced input sequences to block progression until I perform the instructed action, **so that** I learn through practice. |  | F-13.24.7 | R-13.24.7 |
| US-13.24.7 | designer (P-5) | **As a** designer (P-5), **I want** to define overlays per tutorial step by referencing widget IDs or world entities, **so that** highlights are data-driven. |  | F-13.24.7 | R-13.24.7 |
| US-13.24.7 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a spotlight dims everything except the target widget, **so that** the highlight is visually correct. |  | F-13.24.7 | R-13.24.7 |
## Toast Notification System (F-13.24.7c)
| US-13.24.7 | player (P-23) | **As a** player (P-23), **I want** non-blocking pop-up notifications for achievements, loot, and quest updates, **so that** I am informed without interrupting gameplay. |  | F-13.24.7 | R-13.24.7 |
| US-13.24.7 | designer (P-5) | **As a** designer (P-5), **I want** to configure toast icon, title, body, duration, priority, and screen position, **so that** notifications are data-driven. |  | F-13.24.7 | R-13.24.7 |
| US-13.24.7 | creative director (P-2) | **As a** creative director (P-2), **I want** toast priority levels to control display order, **so that** important notifications appear first. |  | F-13.24.7 | R-13.24.7 |
| US-13.24.7 | tester (P-27) | **As a** tester (P-27), **I want** to verify that simultaneous toasts queue rather than overlap, **so that** the queuing system works correctly. |  | F-13.24.7 | R-13.24.7 |
## Free Camera Controller (F-13.24.8a)
| US-13.24.8 | player (P-23) | **As a** player (P-23), **I want** a free-flight camera for virtual photography with position, rotation, roll, and FOV controls, **so that** I can frame screenshots from any angle. |  | F-13.24.8 | R-13.24.8 |
| US-13.24.8 | player (P-23) | **As a** player (P-23), **I want** gameplay to pause in single-player when photo mode is active, **so that** I can compose shots without time pressure. |  | F-13.24.8 | R-13.24.8 |
| US-13.24.8 | designer (P-5) | **As a** designer (P-5), **I want** photo mode to optionally hide the player character and UI, **so that** clean screenshots are possible. |  | F-13.24.8 | R-13.24.8 |
| US-13.24.8 | creative director (P-2) | **As a** creative director (P-2), **I want** the photo mode camera constrained in multiplayer to prevent passing through walls, **so that** it does not enable exploits. |  | F-13.24.8 | R-13.24.8 |
| US-13.24.8 | tester (P-27) | **As a** tester (P-27), **I want** to verify that photo mode pauses time in single-player but not in multiplayer, **so that** the pause behavior is mode-dependent. |  | F-13.24.8 | R-13.24.8 |
## Photo Mode Visual Controls (F-13.24.8b)
| US-13.24.8 | player (P-23) | **As a** player (P-23), **I want** depth of field, exposure, color grading filters, and composition aids in photo mode, **so that** I can create visually polished screenshots. |  | F-13.24.8 | R-13.24.8 |
| US-13.24.8 | player (P-23) | **As a** player (P-23), **I want** a time-of-day override in photo mode, **so that** I can set the lighting to my preferred conditions. |  | F-13.24.8 | R-13.24.8 |
| US-13.24.8 | designer (P-5) | **As a** designer (P-5), **I want** all photo mode adjustments to be temporary and revert on exit, **so that** gameplay settings are not permanently altered. |  | F-13.24.8 | R-13.24.8 |
| US-13.24.8 | tester (P-27) | **As a** tester (P-27), **I want** to verify that rule-of-thirds and golden ratio overlays display correctly, **so that** composition aids are accurate. |  | F-13.24.8 | R-13.24.8 |
## Photo Capture and Gallery (F-13.24.8c)
| US-13.24.8 | player (P-23) | **As a** player (P-23), **I want** to capture screenshots at configurable resolution with optional supersampling, **so that** I get high-quality images. |  | F-13.24.8 | R-13.24.8 |
| US-13.24.8 | player (P-23) | **As a** player (P-23), **I want** an in-game gallery storing captured screenshots with metadata, **so that** I can browse my virtual photography. |  | F-13.24.8 | R-13.24.8 |
| US-13.24.8 | designer (P-5) | **As a** designer (P-5), **I want** screenshot capture to integrate with platform screenshot APIs where available, **so that** captures feed into platform galleries. |  | F-13.24.8 | R-13.24.8 |
| US-13.24.8 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a 4x supersampled capture renders at the higher resolution and downsamples correctly, **so that** supersampling produces accurate results. |  | F-13.24.8 | R-13.24.8 |
