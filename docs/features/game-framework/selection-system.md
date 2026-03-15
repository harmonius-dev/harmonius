# 13.11 — Selection and Picking

## Picking

### F-13.11.1 3D World Picking

Cast rays from the camera through screen-space coordinates to identify entities in the 3D world.
The picking system uses the shared spatial index (F-1.9.4) for broadphase acceleration and
per-entity collision shapes for precise hit testing. Supports single-click pick (nearest entity
under cursor), priority-based pick (interactive objects take precedence over scenery), and
multi-layer pick (return all entities along the ray, sorted by distance, for pick-through
scenarios). Pick results include the hit entity, world-space position, surface normal, and hit
bone/socket for skeletal meshes. The system operates on the main thread with sub-millisecond
latency for responsive interaction. Pick filtering uses component-based queries — only entities
with a `Selectable` component are eligible for picking. All picking rules and priority
configurations are data-driven and authored in the visual editor.

- **Requirements:** R-13.11.1
- **Dependencies:** F-1.9.4 (Unified Spatial Query), F-4.4.1 (Ray Casts),
  F-2.10.1 (ECS-to-Renderer Bridge)
- **Platform notes:** None

### F-13.11.2 2D Screen-Space Picking

Pick UI widgets and 2D game entities using screen-space coordinate testing. For UI, the pick
traverses the widget tree (F-10.1.1) in reverse render order, testing each widget's bounding
rectangle against the cursor position with support for non-rectangular hit areas (circular
buttons, custom hit masks). For 2D game entities, the pick tests sprite bounding boxes and
optional per-pixel alpha masks against the cursor, respecting z-order. On touch devices, pick
areas are expanded by a configurable touch slop radius to improve finger targeting accuracy.
Pick results distinguish between UI picks and world picks, with UI always taking priority. Touch
slop radius and hit area overrides are configured per widget or entity type in the visual editor.

- **Requirements:** R-13.11.2
- **Dependencies:** F-10.1.1 (Widget Tree), F-10.5.1 (Sprite Rendering)
- **Platform notes:** Touch slop expansion is active on iOS and Android by default
  (configurable).

## Selection State

### F-13.11.3 Selection State Management

A centralized selection system implemented as ECS resources and components. Each player
controller maintains a `SelectionSet` resource — an ordered collection of entity references
representing the current selection. Entities receive a `Selected` component when added to the
selection set, enabling efficient ECS queries (e.g., "iterate all selected entities with a
`Health` component"). Selection modes: single (click replaces selection), additive (Shift+click
adds to selection), subtractive (Ctrl+click removes from selection), toggle (click toggles
membership), and exclusive (only one entity selectable at a time). Selection change events fire
through the observer system (F-1.1.30), notifying UI, audio, and gameplay systems when the
selection changes. Selection state is replicated in multiplayer (F-8.2.1) for spectator and team
visibility. All selection mode rules and modifier key bindings are data-driven and configured
through the visual editor.

- **Requirements:** R-13.11.3
- **Dependencies:** F-13.11.1, F-1.1.30 (Observers), F-1.1.1 (ECS)
- **Platform notes:** None

### F-13.11.4a RTS Selection Preset

Pre-configured RTS selection behavior activated per-project through the modular system
(F-13.1.9). Includes box select (marquee), double-click to select all of same type, control
groups (Ctrl+1-9 assign, 1-9 recall), select-all-of-type, select all idle units, select all
military, and subgroup tabs within selection. Configures input bindings (F-6.2.1), filtering
rules, and visual feedback. Customizable in the visual editor without code.

- **Requirements:** R-13.11.4a
- **Dependencies:** F-13.11.3, F-13.1.9 (Modular Systems), F-6.2.1 (Input Actions)
- **Platform notes:** None

### F-13.11.4b RPG Selection Preset

Pre-configured RPG selection behavior: single-target selection with tab-cycling through nearby
enemies, friendly/hostile filtering, and target-of-target display. Tab-cycle ordering is
distance-based with optional priority for quest targets. Configures input bindings (F-6.2.1)
and visual feedback. Customizable in the visual editor without code.

- **Requirements:** R-13.11.4b
- **Dependencies:** F-13.11.3, F-13.1.9 (Modular Systems), F-6.2.1 (Input Actions)
- **Platform notes:** None

### F-13.11.4c Action Selection Preset

Pre-configured action game selection behavior: auto-target nearest enemy, lock-on toggle with
camera focus, and target switching via right-stick flick. Lock-on maintains camera centered on
the target. Target switching cycles through enemies within a configurable arc. Customizable in
the visual editor without code.

- **Requirements:** R-13.11.4c
- **Dependencies:** F-13.11.3, F-13.1.9 (Modular Systems), F-6.2.1 (Input Actions)
- **Platform notes:** None

### F-13.11.4d Builder/Sandbox Selection Preset

Pre-configured builder/sandbox selection behavior: multi-select with transform gizmos,
group/ungroup operations, and hierarchy selection (select parent selects children). Supports
lasso selection in addition to box select for irregular regions. Customizable in the visual
editor without code.

- **Requirements:** R-13.11.4d
- **Dependencies:** F-13.11.3, F-13.1.9 (Modular Systems), F-6.2.1 (Input Actions)
- **Platform notes:** None

## Selection Groups

### F-13.11.5 Runtime Selection Groups

Named, persistent groups of entities that players create and modify at runtime. Groups are
stored as ECS resources containing entity reference lists with a group name, icon, and hotkey
binding. Players assign entities to groups through input actions (Ctrl+number for RTS control
groups, custom bindings for other genres). Groups support set operations: union (merge two
groups), intersection (select entities in both groups), difference (remove one group from
another). Group membership is tracked via a `GroupMembership` component on each entity, enabling
efficient queries like "all entities in group 3 with Velocity > 0." Groups persist across
save/load (F-13.3.1) and are displayed in the HUD group bar (F-10.3.1). In multiplayer, groups
are player-local and not replicated. Group definitions, icons, and hotkey bindings are
configured through the visual editor.

- **Requirements:** R-13.11.5
- **Dependencies:** F-13.11.3, F-6.2.1 (Input Actions), F-13.3.1 (Save/Load)
- **Platform notes:** None

### F-13.11.6a Basic Command Dispatch

Issues commands (move, attack, ability, stop, patrol, hold position) to all entities in the
current selection or a specific named group. The command dispatch system routes commands
through the ability system (F-13.10.2) for individual entity execution. Commands respect
entity capabilities — a "move" command sent to a mixed selection of mobile units and
stationary buildings only moves the mobile units.

- **Requirements:** R-13.11.6a
- **Dependencies:** F-13.11.5, F-13.10.2 (Ability Activation)
- **Platform notes:** None

### F-13.11.6b Formation Movement

Group-level movement commands maintain relative positions between entities using formation
templates. Formation templates define shape (line, box, wedge, circle), spacing, and facing
rules. Entities steer to their assigned formation slot using the steering system (F-7.2.1).
Formation templates are data-driven and configured in the visual editor.

- **Requirements:** R-13.11.6b
- **Dependencies:** F-13.11.6a, F-7.2.1 (Steering Behaviors)
- **Platform notes:** None

### F-13.11.6c Split and Subgroup Commands

Split commands divide the current selection into subgroups and send each subgroup to a
different target (e.g., half to point A, half to point B). Subgroup division supports even
split, type-based split (melee vs ranged), and manual subgroup selection via marquee within
the selected group.

- **Requirements:** R-13.11.6c
- **Dependencies:** F-13.11.6a, F-13.11.5
- **Platform notes:** None

### F-13.11.6d Command History and Undo

Command history is tracked per player. Ctrl+Z cancels the last issued command within a
configurable timeout window (default 2 seconds). Undo restores entities to their pre-command
state (position, target, formation). Only the most recent command is undoable. Timeout and
undo behavior are data-driven and configured in the visual editor.

- **Requirements:** R-13.11.6d
- **Dependencies:** F-13.11.6a
- **Platform notes:** None

## Selection Visuals

### F-13.11.7 2D Selection Box (Marquee Select)

Click-and-drag rectangle selection in screen space. The selection box is rendered as a
semi-transparent filled rectangle with a solid border, using the UI rendering system (F-10.4.1).
As the box is dragged, entities whose screen-space bounds intersect the rectangle are added to a
preview selection (highlighted but not committed). On mouse release, the preview becomes the
active selection. Box selection supports modifier keys: Shift+drag adds to existing selection,
Ctrl+drag subtracts. The box tests against 3D entity screen projections (bounding sphere
projected to screen circle) for 3D games and against sprite bounds for 2D games.
Performance-optimized for selecting hundreds of entities simultaneously (RTS scenarios). Box
appearance (color, opacity, border style) is configured in the visual editor.

- **Requirements:** R-13.11.7
- **Dependencies:** F-13.11.3, F-10.4.1 (UI Rendering),
  F-2.10.1 (ECS-to-Renderer Bridge)
- **Platform notes:** On touch devices, the selection box activates via long-press-and-drag
  gesture (F-6.3.2).

### F-13.11.8 Selection Indicators and Visual Feedback

Visual feedback for selected and hovered entities using the outline and highlight rendering
effects (F-2.11.1, F-2.11.2). Selected entities display a colored outline (team-colored for
RTS, white for RPG target). Hovered entities display a thinner preview outline. Selection
circles (projected ground decals) mark unit positions in RTS games with team-colored rings and
health bar segments. For 2D games, selected sprites receive a 2D outline effect (pixel-perfect
edge detection on the sprite alpha mask) and an optional bounce or glow animation. The visual
style is configurable per genre preset (F-13.11.4) and per entity type (hero units get a
distinct selection indicator). All selection visuals update in real-time as the selection
changes. Indicator styles, colors, and animations are authored in the visual editor.

- **Requirements:** R-13.11.8
- **Dependencies:** F-13.11.3, F-2.11.1 (Outline Rendering),
  F-2.11.2 (Highlight/Glow), F-11.2.1 (Decals)
- **Platform notes:** None
