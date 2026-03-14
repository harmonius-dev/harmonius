# 13.1 — Gameplay Primitives

## Game Modes

### F-13.1.1 Game Mode State Machine

Defines a hierarchical state machine that governs the rules, scoring, win/loss conditions, and
phase transitions for a game session. Each mode (open world, dungeon, raid, battleground, arena)
declares its player count limits, team composition rules, and respawn policies. Supports nested
sub-modes for encounters within a larger session, such as boss phases inside a raid instance.

- **Requirements:** R-13.1.1
- **Dependencies:** F-1.5.1 (Typed Event Channels), F-1.1.9 (Run Criteria)
- **Platform notes:** None

### F-13.1.2 Game State Manager

Manages the top-level game state lifecycle: main menu, character select, loading, in-game, paused,
disconnected, and shutdown. State transitions trigger resource loading/unloading, UI layer swaps,
and input context changes. Supports server-authoritative state synchronization so the client and
server agree on the current phase for instanced content like dungeons and raids.

- **Requirements:** R-13.1.2
- **Dependencies:** F-13.1.1, F-1.5.6 (Singleton Resources)
- **Platform notes:** None

## Player Control

### F-13.1.3 Player Controller

Routes player input into gameplay actions by mediating between the input action system and the
controlled pawn entity. Handles input context switching (exploration, combat, mounted, vehicle,
cinematic), camera ownership, and UI focus arbitration. In an MMO context, the controller also
manages targeting (tab-target, action-target, soft-lock) and queued ability inputs.

- **Requirements:** R-13.1.3
- **Dependencies:** F-6.2.1 (Input Actions), F-13.1.4
- **Platform notes:** None

### F-13.1.4 Pawn and Character System

Separates the concept of a controllable pawn (any entity that can be possessed) from a character
(a pawn with movement, collision, and gameplay attributes). Pawns support possession transfer for
mechanics like mind control, spectator mode, and vehicle mounting. Characters carry stats, equipment
slots, faction, and level data. Designed for MMO scale where thousands of NPC characters coexist
with player characters.

- **Requirements:** R-13.1.4
- **Dependencies:** F-1.1.1 (Archetype Storage), F-4.1.3 (Character Controller)
- **Platform notes:** None

## Ability System

### F-13.1.5 Gameplay Ability System

A data-driven ability framework where each ability is an asset defining activation conditions, costs
(mana, energy, rage, cooldowns), targeting rules (self, single target, AoE cone/circle/line),
cast time, channeling behavior, and interrupt priority. Abilities produce gameplay effects on
activation and can be queued, combo-chained, or cancelled. The system handles global cooldown (GCD),
shared cooldown groups, and per-ability cooldowns with server-authoritative validation.

- **Requirements:** R-13.1.5
- **Dependencies:** F-13.1.4, F-1.5.1 (Typed Event Channels)
- **Platform notes:** None

### F-13.1.6 Gameplay Effect System

Applies stat modifications, buffs, debuffs, damage-over-time, healing-over-time, and crowd-control
effects to characters. Effects have duration, stacking rules (refresh, stack count, diminishing
returns), tick intervals, and snapshot-vs-dynamic attribute binding. Supports effect inhibition
(immunity, dispel, purge) and priority ordering for conflicting effects such as competing crowd
controls.

- **Requirements:** R-13.1.6
- **Dependencies:** F-13.1.5, F-13.1.4
- **Platform notes:** None

## Damage and Death

### F-13.1.7 Damage Model

Computes final damage from base values through a configurable pipeline of modifiers: attack power
scaling, armor/resistance mitigation, critical strike, block, parry, dodge, absorb shields, and
vulnerability multipliers. Supports multiple damage schools (physical, fire, frost, shadow, etc.)
with per-school resistances. Produces a damage event consumed by the health system, combat log,
floating combat text, and network replication.

- **Requirements:** R-13.1.7
- **Dependencies:** F-13.1.6, F-13.1.4
- **Platform notes:** None

### F-13.1.8 Death, Respawn, and Encounter Reset

Handles character death transitions: ragdoll activation, death state, spirit/ghost phase, respawn
point selection (graveyard, checkpoint, instance entrance), and resurrection by other players.
For instanced content, manages encounter resets (wipe recovery) that restore boss health, despawn
adds, and reset phase state. Supports configurable respawn timers including debuffs for repeated
deaths.

- **Requirements:** R-13.1.8
- **Dependencies:** F-13.1.7, F-13.1.1, F-9.4.7 (Animation Montages)
- **Platform notes:** None
