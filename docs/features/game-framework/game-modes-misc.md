# 13.24 — Game Modes and Systems

## Wave and Tower Defense

### F-13.24.1 Wave Spawning System

Configurable enemy wave spawner for tower defense, survival, and arena game modes. Wave definitions
specify: enemy composition (entity types and counts), spawn points (single or randomized from a
pool), spawn timing (all at once, staggered, continuous stream), inter-wave delay, and difficulty
scaling curve. Endless mode procedurally generates waves using configurable escalation rules
(increase enemy count, add tougher types, reduce spawn delay). Wave state (current wave, enemies
remaining, next wave timer) is exposed as ECS resources for UI display and gameplay logic. Boss
waves trigger special spawns at configurable intervals.

- **Requirements:** R-13.24.1
- **Dependencies:** F-1.1.1 (ECS), F-7.1.1 (NavMesh)
- **Platform notes:** None

### F-13.24.2 Tower Targeting and Upgrade System

Configurable targeting modes for tower/turret entities: first-in-path, nearest, strongest, weakest,
most HP, least HP, and fastest. Towers auto-fire at targets within range using the ability system
(F-13.10.1). Tower upgrade paths define linear or branching stat improvements (damage, range, fire
rate, special effects) with per-tier costs. Upgrading changes the tower's visual mesh and stat
profile. Tower roles include: damage (single target or AoE), support (buff nearby towers or debuff
enemies), slow (apply speed reduction), and spawner (summon allied units). All tower stats and
upgrades are gameplay database entries.

- **Requirements:** R-13.24.2
- **Dependencies:** F-13.10.1 (Abilities), F-13.7.1 (Table Schema)
- **Platform notes:** None

## Score and Feedback

### F-13.24.3 Score and Combo System

Reusable scoring framework with per-event point values, multipliers, and combo chains. Base points
are awarded for game events (enemy kill, objective capture, trick completion). Combo counters
increment on successive scoring events within a timing window and multiply points (2x, 3x, etc.).
Combo breaks reset the counter after the window expires. Score multipliers from temporary power-ups
or performance streaks stack multiplicatively. Final grade calculation (S/A/B/C/D or 1-3 stars) maps
total score to configurable thresholds. High scores are tracked per level and globally. The system
is generic — used for action games, rhythm games, racing, and arcade modes.

- **Requirements:** R-13.24.3
- **Dependencies:** F-1.1.30 (Observers), F-13.7.5 (Formula DSL)
- **Platform notes:** None

### F-13.24.4a Feedback Stack Asset and Triggering

Composable feedback presets that chain multiple juice effects into a single triggerable asset. A
feedback stack asset contains an ordered list of feedback entries, each with a type identifier,
timing (delay offset, duration), intensity scalar, and parallel-vs-sequential execution flag. Stacks
are triggered by gameplay events (hit confirmed, ability activated, level up) through the observer
system (F-1.1.30). Multiple stacks can fire concurrently with independent lifetimes. Feedback stacks
are authored visually in the asset editor — never in code.

- **Requirements:** R-13.24.4a
- **Dependencies:** F-1.1.30 (Observers)
- **Platform notes:** None
- **User story:** As a designer, I want to compose feedback stacks from reusable entries so that I
  can iterate on game feel without programmer support.

### F-13.24.4b Hit-Stop and Time Scale Effects

Frame-precise time manipulation effects for combat feedback. Hit-stop freezes all or selective
entities for a configurable frame count on hit confirmation. Slow-motion ramps smoothly interpolate
the global time scale from 1.0 to a target value (e.g., 0.1) and back over a configured duration.
Per-entity time scale overrides enable bullet-time effects where one entity runs at normal speed
while the world slows. Time scale effects are feedback entry types usable within feedback stacks
(F-13.24.4a). Hit-stop and time scale respect multiplayer constraints (local-only in networked
games).

- **Requirements:** R-13.24.4b
- **Dependencies:** F-13.24.4a, F-1.1.1 (ECS)
- **Platform notes:** None
- **User story:** As a designer, I want hit-stop and slow-motion effects so that impactful moments
  in combat have dramatic emphasis.

### F-13.24.4c Feedback Entry Types Library

Built-in library of feedback entry types usable in feedback stacks. Entry types include: camera
shake (frequency, amplitude, decay), haptic pulse (intensity, duration per F-6.4.1), particle burst
(asset reference, spawn count), sound cue (audio asset, volume, pitch), chromatic aberration
(intensity ramp), screen flash (color, opacity, fade curve), and squash-and-stretch (scale axes,
intensity, spring parameters). Each type defines its parameter schema for the visual editor. New
entry types are registerable via the extensibility API.

- **Requirements:** R-13.24.4c
- **Dependencies:** F-13.24.4a, F-11.3.1 (Screen Shake), F-6.4.1 (Haptics), F-5.1.1 (Audio Engine)
- **Platform notes:** None
- **User story:** As a designer, I want a library of ready-made feedback types so that I can build
  rich multi-sensory effects from standard building blocks.

## Exploration

### F-13.24.5 Fast Travel Network

Named waypoints (shrines, teleporters, campfires) that players discover through world exploration.
Discovered waypoints are unlocked for fast travel — selecting a destination from the map UI or
interacting with a waypoint opens a travel menu showing available destinations with names, icons,
and optional travel costs (currency, cooldown). Travel triggers a loading transition (or seamless
zone streaming) and repositions the player at the destination. Hearthstone/recall abilities
(F-13.10.1) teleport to a bound waypoint. The fast travel network is defined by level designers
placing waypoint entities with discovery trigger volumes.

- **Requirements:** R-13.24.5
- **Dependencies:** F-13.17.1 (Interaction), F-13.2.1 (Level Streaming), F-4.2.8 (Trigger Volumes)
- **Platform notes:** None

### F-13.24.6 Respawn and Graveyard System

Designated respawn locations in the world activated on player death. The system selects the nearest
unlocked graveyard to the death location. On death, players optionally enter a spirit/ghost form
with restricted movement and abilities, navigating back to their corpse for revival (corpse-run
mechanic). Resurrection sickness debuff applies temporary stat penalties after respawning.
Graveyards are discoverable waypoints that double as respawn points. In multiplayer, other players
can resurrect fallen players using abilities, bypassing the graveyard system. Respawn timers are
configurable per game mode (instant in casual, delayed in competitive).

- **Requirements:** R-13.24.6
- **Dependencies:** F-13.10.3 (Gameplay Effects), F-4.2.8 (Trigger Volumes)
- **Platform notes:** None

### F-13.24.7a Tutorial Step System

Data-driven tutorial framework with sequential step definitions, triggers, and progression tracking.
Each tutorial step is a data asset specifying: trigger condition (enter zone, open menu, first
login), instruction text, required player input or action for completion, and next step reference.
Tutorial progress persists across sessions so interrupted tutorials resume at the correct step. A
skip option allows players to dismiss the entire tutorial. Steps execute through the logic graph
system (F-15.8.4) for custom completion conditions.

- **Requirements:** R-13.24.7a
- **Dependencies:** F-6.2.1 (Input Actions), F-15.8.4 (Logic Graphs)
- **Platform notes:** None
- **User story:** As a designer, I want to author tutorial sequences as data assets so that
  onboarding flows can be iterated without code changes.

### F-13.24.7b Tutorial Visual Overlays

Visual overlay system for tutorial highlighting and callouts. Spotlight overlays dim the screen
except for a highlighted UI widget or world entity, drawing player attention. Arrow callouts point
from instruction text to the relevant control or object. Tooltip callouts display contextual help
text anchored to a widget or world position. Overlays are defined per tutorial step (F-13.24.7a) by
referencing a widget ID or world entity. Forced input sequences block progression until the player
performs the instructed action.

- **Requirements:** R-13.24.7b
- **Dependencies:** F-13.24.7a, F-10.1.1 (Widget Tree)
- **Platform notes:** None
- **User story:** As a player, I want spotlight overlays and arrow callouts so that I know exactly
  where to look and what to do during tutorials.

### F-13.24.7c Toast Notification System

Non-blocking pop-up notification system independent of the tutorial framework. Toast notifications
display brief messages for achievements, loot pickups, quest updates, and status changes. Toasts
appear in a configurable screen region (top, bottom, corner), queue if multiple fire simultaneously,
and auto-dismiss after a configurable duration. Each toast defines: icon, title text, body text,
duration, and optional action button. Toast priority levels control display order. The system is
usable by any gameplay system via the observer pattern (F-1.1.30).

- **Requirements:** R-13.24.7c
- **Dependencies:** F-10.1.1 (Widget Tree), F-1.1.30 (Observers)
- **Platform notes:** None
- **User story:** As a player, I want non-blocking pop-up notifications so that I am informed of
  achievements and loot without interrupting gameplay.

### F-13.24.8a Free Camera Controller

Free-flight camera activated from the pause menu for virtual photography. Controls include: position
(free flight with configurable speed), rotation (pitch, yaw), roll, and FOV/zoom slider. Gameplay
pauses in single-player when photo mode is active. In multiplayer, the camera is constrained to
prevent passing through walls and time does not pause. Entering photo mode optionally hides the
player character and UI. The camera controller is a dedicated ECS system that overrides the gameplay
camera while active.

- **Requirements:** R-13.24.8a
- **Dependencies:** F-13.5.2 (Camera Tracks)
- **Platform notes:** None
- **User story:** As a player, I want a free-flight camera in photo mode so that I can frame
  screenshots from any angle.

### F-13.24.8b Photo Mode Visual Controls

Post-processing and visual overrides for photo mode composition. Controls include: depth of field
(focus distance, aperture), exposure compensation, brightness/contrast/saturation sliders, color
grading filters (vintage, noir, warm, cool, custom LUT), time-of-day override, and chromatic
aberration. Composition aids overlay rule-of-thirds grid and golden ratio lines. All adjustments are
temporary and revert when exiting photo mode. Controls are exposed through a photo mode UI panel
with real-time preview.

- **Requirements:** R-13.24.8b
- **Dependencies:** F-13.24.8a, F-2.9.1 (Post-Processing), F-10.1.1 (Widget Tree)
- **Platform notes:** None
- **User story:** As a player, I want DOF, color grading, and composition guides so that I can
  create visually polished screenshots.

### F-13.24.8c Photo Capture and Gallery

Screenshot capture at configurable resolution with optional supersampling (2x, 4x). Captured images
are saved to disk in PNG format. An in-game gallery stores captured screenshots with metadata (date,
location, settings). The gallery is accessible from the main menu. Platform screenshot APIs are used
where available (Steam screenshot, console capture) for platform integration. Supersampled captures
render at the higher resolution then downsample for the final image.

- **Requirements:** R-13.24.8c
- **Dependencies:** F-13.24.8a, F-13.24.8b
- **Platform notes:** Screenshots use platform screenshot APIs where available (Steam screenshot,
  console capture).
- **User story:** As a player, I want to capture high-resolution screenshots and browse them in an
  in-game gallery so that I can revisit my best virtual photographs.
