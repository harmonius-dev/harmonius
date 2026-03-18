# 13.1 — Gameplay Primitives

## Game Modes

| ID       | Feature                 | Requirements |
|----------|-------------------------|--------------|
| F-13.1.1 | Game Mode State Machine | R-13.1.1     |
| F-13.1.2 | Game State Manager      | R-13.1.2     |

1. **F-13.1.1** — Defines a hierarchical state machine that governs the rules, scoring, win/loss
   conditions, and phase transitions for a game session. Each mode (open world, dungeon, raid,
   battleground, arena) declares its player count limits, team composition rules, and respawn
   policies. Supports nested sub-modes for encounters within a larger session, such as boss phases
   inside a raid instance.
   - **Deps:** F-1.5.1 (Typed Event Channels), F-1.1.9 (Run Criteria)
2. **F-13.1.2** — Manages the top-level game state lifecycle: main menu, character select, loading,
   in-game, paused, disconnected, and shutdown. State transitions trigger resource
   loading/unloading, UI layer swaps, and input context changes. Supports server-authoritative state
   synchronization so the client and server agree on the current phase for instanced content like
   dungeons and raids.
   - **Deps:** F-13.1.1, F-1.5.6 (Singleton Resources)

## Player Control

| ID       | Feature                   | Requirements |
|----------|---------------------------|--------------|
| F-13.1.3 | Player Controller         | R-13.1.3     |
| F-13.1.4 | Pawn and Character System | R-13.1.4     |

1. **F-13.1.3** — Routes player input into gameplay actions by mediating between the input action
   system and the controlled pawn entity. Handles input context switching (exploration, combat,
   mounted, vehicle, cinematic), camera ownership, and UI focus arbitration. In an MMO context, the
   controller also manages targeting (tab-target, action-target, soft-lock) and queued ability
   inputs.
   - **Deps:** F-6.2.1 (Input Actions), F-13.1.4
2. **F-13.1.4** — Separates the concept of a controllable pawn (any entity that can be possessed)
   from a character (a pawn with movement, collision, and gameplay attributes). Pawns support
   possession transfer for mechanics like mind control, spectator mode, and vehicle mounting.
   Characters carry stats, equipment slots, faction, and level data. Designed for MMO scale where
   thousands of NPC characters coexist with player characters.
   - **Deps:** F-1.1.1 (Archetype Storage), F-4.1.3 (Character Controller)

## Ability System

| ID       | Feature                 | Requirements |
|----------|-------------------------|--------------|
| F-13.1.5 | Gameplay Ability System | R-13.1.5     |
| F-13.1.6 | Gameplay Effect System  | R-13.1.6     |

1. **F-13.1.5** — A data-driven ability framework where each ability is an asset defining activation
   conditions, costs (mana, energy, rage, cooldowns), targeting rules (self, single target, AoE
   cone/circle/line), cast time, channeling behavior, and interrupt priority. Abilities produce
   gameplay effects on activation and can be queued, combo-chained, or cancelled. The system handles
   global cooldown (GCD), shared cooldown groups, and per-ability cooldowns with
   server-authoritative validation.
   - **Deps:** F-13.1.4, F-1.5.1 (Typed Event Channels)
2. **F-13.1.6** — Applies stat modifications, buffs, debuffs, damage-over-time, healing-over-time,
   and crowd-control effects to characters. Effects have duration, stacking rules (refresh, stack
   count, diminishing returns), tick intervals, and snapshot-vs-dynamic attribute binding. Supports
   effect inhibition (immunity, dispel, purge) and priority ordering for conflicting effects such as
   competing crowd controls.
   - **Deps:** F-13.1.5, F-13.1.4

## Damage and Death

| ID       | Feature                             | Requirements |
|----------|-------------------------------------|--------------|
| F-13.1.7 | Damage Model                        | R-13.1.7     |
| F-13.1.8 | Death, Respawn, and Encounter Reset | R-13.1.8     |

1. **F-13.1.7** — Computes final damage from base values through a configurable pipeline of
   modifiers: attack power scaling, armor/resistance mitigation, critical strike, block, parry,
   dodge, absorb shields, and vulnerability multipliers. Supports multiple damage schools (physical,
   fire, frost, shadow, etc.) with per-school resistances. Produces a damage event consumed by the
   health system, combat log, floating combat text, and network replication.
   - **Deps:** F-13.1.6, F-13.1.4
2. **F-13.1.8** — Handles character death transitions: ragdoll activation, death state, spirit/ghost
   phase, respawn point selection (graveyard, checkpoint, instance entrance), and resurrection by
   other players. For instanced content, manages encounter resets (wipe recovery) that restore boss
   health, despawn adds, and reset phase state. Supports configurable respawn timers including
   debuffs for repeated deaths.
   - **Deps:** F-13.1.7, F-13.1.1, F-9.4.7 (Animation Montages)

## Modular Systems

| ID       | Feature                     | Requirements |
|----------|-----------------------------|--------------|
| F-13.1.9 | Modular System Registration | R-13.1.9     |

1. **F-13.1.9** — Every gameplay system (physics, audio, AI, networking, animation, UI, VFX, combat,
   inventory, quests) registers through the plugin system (F-1.6.1) with declared dependencies and
   is independently enableable per project. The project file (F-15.15.4) specifies which systems are
   active; disabled systems are excluded from compilation, reducing binary size, editor UI clutter,
   and runtime memory. System dependencies are validated at project load — enabling combat requires
   physics and animation; enabling networking requires serialization. The editor UI hides panels,
   inspectors, and tools for disabled systems. Enabling a system mid-project automatically enables
   its transitive dependencies with user confirmation.
   - **Deps:** F-1.6.1 (Plugin System), F-15.15.4 (Project File Format)

## Developer Extensibility

| ID        | Feature                        | Requirements |
|-----------|--------------------------------|--------------|
| F-13.1.10 | Rust Plugin API for Developers | R-13.1.10    |

1. **F-13.1.10** — A stable Rust API for third-party developers to extend the engine with custom
   systems, components, and editor tools. Plugins are Rust crates that depend on the engine's public
   API crate, compiled as dynamic libraries (.dylib/.dll/.so) loaded at editor and runtime startup.
   The plugin API exposes: ECS world access (register components, add systems, query entities),
   asset pipeline hooks (custom importers, processors), editor extension points (custom inspector
   panels, viewport overlays, menu items), and rendering hooks (custom render passes, post-process
   effects). Plugins declare their engine version compatibility; the loader validates ABI
   compatibility before loading. A plugin template generator scaffolds new plugin projects with
   build configuration and boilerplate.
   - **Deps:** F-1.6.1 (Plugin System), F-1.1.4 (Component Registration), F-1.3.1 (Type Registry)
   - **Platform:** Dynamic library loading uses dlopen on macOS/Linux, LoadLibrary on Windows. ABI
     stability requires a C-compatible plugin interface layer even though plugins are authored in
     Rust.
