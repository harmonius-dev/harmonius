# User Stories — Gameplay Primitives (13.1)

## Game Mode State Machine

| ID          | Persona              |
|-------------|----------------------|
| US-13.1.1.1 | game designer (P-5)  |
| US-13.1.1.2 | game developer (P-15)|
| US-13.1.1.3 | player (P-23)        |

1. **US-13.1.1.1** — **As a** game designer (P-5), **I want** to define a hierarchical state machine
   governing rules, scoring, and phase transitions per game session, **so that** game modes are
   data-driven.
2. **US-13.1.1.2** — **As a** game developer (P-15), **I want** nested sub-modes for encounters
   within a larger session, **so that** phase logic composes without duplication.
3. **US-13.1.1.3** — **As a** player (P-23), **I want** win/loss conditions and respawn policies to
   vary by mode, **so that** each game mode plays differently.

## Game State Manager

| ID          | Persona              |
|-------------|----------------------|
| US-13.1.2.1 | game designer (P-5)  |
| US-13.1.2.2 | game developer (P-15)|
| US-13.1.2.3 | player (P-23)        |

1. **US-13.1.2.1** — **As a** game designer (P-5), **I want** top-level game state lifecycle
   management (menu, loading, in-game, paused), **so that** transitions trigger resource loading and
   UI swaps.
2. **US-13.1.2.2** — **As a** game developer (P-15), **I want** state transitions to fire events
   consumed by loading, UI, and input systems, **so that** each system reacts automatically.
3. **US-13.1.2.3** — **As a** player (P-23), **I want** the game to handle disconnection and
   reconnection gracefully, **so that** I resume without losing progress.

## Player Controller

| ID          | Persona              |
|-------------|----------------------|
| US-13.1.3.1 | game designer (P-5)  |
| US-13.1.3.2 | game developer (P-15)|
| US-13.1.3.3 | player (P-23)        |

1. **US-13.1.3.1** — **As a** game designer (P-5), **I want** input context switching (exploration,
   combat, vehicle, cinematic) on the player controller, **so that** inputs change by gameplay mode.
2. **US-13.1.3.2** — **As a** game developer (P-15), **I want** the controller to mediate between
   input actions and the possessed pawn, **so that** I can route inputs to any entity.
3. **US-13.1.3.3** — **As a** player (P-23), **I want** targeting and queued ability inputs to be
   responsive, **so that** combat input feels immediate.

## Pawn and Character System

| ID          | Persona              |
|-------------|----------------------|
| US-13.1.4.1 | game designer (P-5)  |
| US-13.1.4.2 | game developer (P-15)|
| US-13.1.4.3 | player (P-23)        |

1. **US-13.1.4.1** — **As a** game designer (P-5), **I want** to separate pawns (any possessable
   entity) from characters (pawns with stats and equipment), **so that** possession transfer works
   for vehicles and spectating.
2. **US-13.1.4.2** — **As a** game developer (P-15), **I want** characters to carry stats, equipment
   slots, and faction as ECS components, **so that** gameplay systems query them uniformly.
3. **US-13.1.4.3** — **As a** player (P-23), **I want** to possess different pawns (mounts,
   vehicles) and return to my character, **so that** gameplay variety exists.

## Ability Framework

| ID          | Persona              |
|-------------|----------------------|
| US-13.1.5.1 | game designer (P-5)  |
| US-13.1.5.2 | game developer (P-15)|
| US-13.1.6.1 | game designer (P-5)  |
| US-13.1.6.2 | player (P-23)        |

1. **US-13.1.5.1** — **As a** game designer (P-5), **I want** a data-driven ability framework
   defining activation conditions, costs, targeting, and effects per ability, **so that** abilities
   are authored without code.
2. **US-13.1.5.2** — **As a** game developer (P-15), **I want** abilities to produce gameplay
   effects validated server-side, **so that** the system is authoritative.
3. **US-13.1.6.1** — **As a** game designer (P-5), **I want** effects with duration, stacking rules,
   and tick intervals, **so that** buffs and debuffs are configurable.
4. **US-13.1.6.2** — **As a** player (P-23), **I want** effects to display as icons with timers in
   the HUD, **so that** I track active buffs and debuffs.

## Damage and Death

| ID          | Persona              |
|-------------|----------------------|
| US-13.1.7.1 | game designer (P-5)  |
| US-13.1.7.2 | game developer (P-15)|
| US-13.1.8.1 | game designer (P-5)  |
| US-13.1.8.2 | player (P-23)        |

1. **US-13.1.7.1** — **As a** game designer (P-5), **I want** a configurable damage pipeline with
   scaling, mitigation, critical strike, and damage type resistances, **so that** combat math is
   tunable.
2. **US-13.1.7.2** — **As a** game developer (P-15), **I want** the damage pipeline to produce
   events consumed by health, combat log, floating text, and replication, **so that** damage is
   observable.
3. **US-13.1.8.1** — **As a** game designer (P-5), **I want** death, respawn, and encounter reset as
   configurable state transitions, **so that** recovery rules vary by mode.
4. **US-13.1.8.2** — **As a** player (P-23), **I want** clear respawn options (checkpoint,
   graveyard) after death, **so that** I know how to re-enter gameplay.

## Modular System Registration

| ID          | Persona                |
|-------------|------------------------|
| US-13.1.9.1 | game designer (P-5)    |
| US-13.1.9.2 | engine developer (P-26)|
| US-13.1.9.3 | game developer (P-15)  |

1. **US-13.1.9.1** — **As a** game designer (P-5), **I want** to enable or disable gameplay systems
   per project in the project file, **so that** unused systems do not clutter the editor.
2. **US-13.1.9.2** — **As a** engine developer (P-26), **I want** system dependencies validated at
   project load, **so that** enabling combat requires physics and animation.
3. **US-13.1.9.3** — **As a** game developer (P-15), **I want** enabling a system to automatically
   enable its transitive dependencies with confirmation, **so that** setup is effortless.

## Rust Plugin API

| ID           | Persona                |
|--------------|------------------------|
| US-13.1.10.1 | game developer (P-15)  |
| US-13.1.10.2 | game developer (P-15)  |
| US-13.1.10.3 | engine developer (P-26)|

1. **US-13.1.10.1** — **As a** game developer (P-15), **I want** a stable Rust plugin API exposing
   ECS world access, asset hooks, and editor extension points, **so that** I extend the engine
   without forking.
2. **US-13.1.10.2** — **As a** game developer (P-15), **I want** a plugin template generator that
   scaffolds new plugin projects, **so that** I start developing quickly.
3. **US-13.1.10.3** — **As a** engine developer (P-26), **I want** plugins to declare engine version
   compatibility with ABI validation on load, **so that** incompatible plugins are rejected.
