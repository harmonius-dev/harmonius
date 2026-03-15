# User Stories — 13.11 Selection and Picking

## F-13.11.1 3D World Picking

## US-13.11.1.1 Configure Pick Priority and Filtering Rules

**As a** designer (P-5), **I want to** configure pick priority (interactive objects over
scenery) and filtering rules (only entities with Selectable component) in the visual editor,
**so that** picking behavior matches the game's interaction design.

## US-13.11.1.2 Click to Select Entities in the 3D World

**As a** player (P-23), **I want to** click on entities in the 3D world and have the nearest
selectable entity picked instantly, **so that** I can interact with the game world responsively.

## US-13.11.1.3 Verify Sub-Millisecond Picking Latency

**As an** engine tester (P-27), **I want to** perform 1000 pick operations and verify all
complete in under 1ms each, **so that** picking is responsive enough for 60 FPS gameplay.

## US-13.11.1.4 Verify Multi-Layer Pick Returns Sorted Results

**As an** engine tester (P-27), **I want to** perform a pick through multiple overlapping
entities and verify all are returned sorted by distance with correct hit positions and
normals, **so that** pick-through scenarios work correctly.

## F-13.11.2 2D Screen-Space Picking

## US-13.11.2.1 Configure Touch Slop Radius for Mobile

**As a** designer (P-5), **I want to** configure touch slop radius per widget or entity type on
touch devices, **so that** finger targeting is accurate on mobile screens.

## US-13.11.2.2 Pick UI Widgets and 2D Entities

**As a** player (P-23), **I want to** click on UI widgets and 2D game entities with screen-
space testing, **so that** 2D interactions work accurately with UI always taking priority.

## US-13.11.2.3 Verify UI Picks Take Priority Over World Picks

**As an** engine tester (P-27), **I want to** click where a UI widget overlaps a world entity
and verify the UI widget is picked, **so that** UI always takes priority over world picks.

## US-13.11.2.4 Verify Per-Pixel Alpha Mask Picking for Sprites

**As an** engine tester (P-27), **I want to** click on the transparent area of a sprite and
verify no pick occurs, while clicking on the opaque area succeeds, **so that** per-pixel
alpha mask picking works correctly.

## F-13.11.3 Selection State Management

## US-13.11.3.1 Configure Selection Mode Rules and Modifier Keys

**As a** designer (P-5), **I want to** configure selection mode rules and modifier key bindings
in the visual editor, **so that** selection behavior matches the game's genre conventions.

## US-13.11.3.2 Use Selection Modes (Single, Additive, Subtractive)

**As a** player (P-23), **I want to** use single click, Shift+click (additive), Ctrl+click
(subtractive), and toggle selection modes, **so that** I can build selections precisely.

## US-13.11.3.3 Verify Selection Change Events Fire Correctly

**As an** engine tester (P-27), **I want to** add and remove entities from the selection and
verify selection change events fire through the observer system, **so that** UI, audio, and
gameplay systems are notified of every selection change.

## US-13.11.3.4 Verify Selected Component Is Added/Removed Correctly

**As an** engine tester (P-27), **I want to** select and deselect entities and verify the
Selected component is added on select and removed on deselect, **so that** ECS queries on
Selected entities work correctly.

## F-13.11.4a RTS Selection Preset

## US-13.11.4a.1 Configure RTS Selection Input Bindings

**As a** designer (P-5), **I want to** customize RTS selection input bindings, filtering rules,
and visual feedback in the visual editor, **so that** the RTS preset matches my game's
conventions.

## US-13.11.4a.2 Use Box Select and Control Groups

**As a** player (P-23), **I want to** box select units, double-click to select all of same
type, and assign/recall control groups with Ctrl+1-9 / 1-9, **so that** I can manage armies
using familiar RTS conventions.

## US-13.11.4a.3 Select All Idle or All Military Units

**As a** player (P-23), **I want to** select all idle units or all military units with dedicated
hotkeys, **so that** I can quickly find and command unused forces.

## US-13.11.4a.4 Verify RTS Preset Activates Correct Behaviors

**As an** engine tester (P-27), **I want to** enable the RTS preset and verify box select,
control groups, select-all-of-type, and subgroup tabs all function correctly, **so that** the
preset provides a complete RTS selection experience.

## F-13.11.4b RPG Selection Preset

## US-13.11.4b.1 Configure RPG Targeting Priorities

**As a** designer (P-5), **I want to** configure tab-cycle ordering with optional priority for
quest targets, **so that** targeting behavior helps players find relevant enemies.

## US-13.11.4b.2 Tab-Cycle Through Nearby Enemies

**As a** player (P-23), **I want to** press Tab to cycle through nearby enemies with hostile
filtering and distance-based ordering, **so that** I can target enemies quickly during combat.

## US-13.11.4b.3 See Target-of-Target Display

**As a** player (P-23), **I want to** see the target-of-target display when I have an enemy
selected, **so that** I know who the enemy is attacking.

## US-13.11.4b.4 Verify RPG Preset Tab-Cycling Correctness

**As an** engine tester (P-27), **I want to** verify tab-cycling visits all nearby hostile
entities in distance order without skipping or repeating, **so that** the RPG preset's
targeting cycle is correct.

## F-13.11.4c Action Selection Preset

## US-13.11.4c.1 Configure Lock-On Camera Behavior

**As a** designer (P-5), **I want to** configure lock-on camera centering behavior and target
switching arc angle, **so that** the action preset feels right for my game's combat pace.

## US-13.11.4c.2 Auto-Target and Lock On to Enemies

**As a** player (P-23), **I want to** auto-target the nearest enemy and toggle lock-on with
camera focus, **so that** I can stay focused on my target during fast-paced action combat.

## US-13.11.4c.3 Switch Targets With Right-Stick Flick

**As a** player (P-23), **I want to** switch targets by flicking the right stick to cycle
through enemies within a configurable arc, **so that** target switching is fast and intuitive
on gamepad.

## US-13.11.4c.4 Verify Lock-On Maintains Camera Focus

**As an** engine tester (P-27), **I want to** lock on to a target and verify the camera stays
centered on the target through all movement and combat actions, **so that** lock-on camera
focus is stable.

## F-13.11.4d Builder/Sandbox Selection Preset

## US-13.11.4d.1 Configure Builder Selection Behavior

**As a** designer (P-5), **I want to** configure hierarchy selection (select parent selects
children) and group/ungroup controls, **so that** the builder preset matches the game's
construction mechanics.

## US-13.11.4d.2 Multi-Select With Transform Gizmos

**As a** player (P-23), **I want to** multi-select objects with transform gizmos and group/
ungroup operations, **so that** I can manipulate complex constructions efficiently.

## US-13.11.4d.3 Use Lasso Selection for Irregular Regions

**As a** player (P-23), **I want to** use lasso selection in addition to box select for
irregular selection regions, **so that** I can select objects in complex arrangements.

## US-13.11.4d.4 Verify Hierarchy Selection Propagation

**As an** engine tester (P-27), **I want to** select a parent entity and verify all children
are also selected, then deselect the parent and verify children are deselected, **so that**
hierarchy selection propagation is correct.

## F-13.11.5 Runtime Selection Groups

## US-13.11.5.1 Configure Group Display in HUD

**As a** designer (P-5), **I want to** configure how groups display in the HUD group bar with
icons and hotkey labels, **so that** group visibility matches the game's UI design.

## US-13.11.5.2 Create and Recall Selection Groups

**As a** player (P-23), **I want to** assign selected entities to named groups with hotkey
bindings (Ctrl+number) and recall them (number), **so that** I can quickly switch between
saved selections during gameplay.

## US-13.11.5.3 Perform Set Operations on Groups

**As a** player (P-23), **I want to** perform union, intersection, and difference operations
between groups, **so that** I can combine or refine selections without manual picking.

## US-13.11.5.4 Verify Groups Persist Across Save/Load

**As an** engine tester (P-27), **I want to** create groups, save, load, and verify all groups
and their entity memberships are preserved, **so that** groups survive save/load cycles.

## US-13.11.5.5 Verify Group Membership Component Tracks Correctly

**As an** engine tester (P-27), **I want to** add and remove entities from groups and verify the
GroupMembership component updates correctly on each entity, **so that** ECS queries on group
membership are accurate.

## F-13.11.6a Basic Command Dispatch

## US-13.11.6a.1 Configure Command Dispatch Rules

**As a** designer (P-5), **I want to** configure which commands each entity type supports (move
only for mobile units, not buildings), **so that** commands respect entity capabilities.

## US-13.11.6a.2 Issue Commands to Selected Units

**As a** player (P-23), **I want to** issue move, attack, ability, stop, patrol, and hold
position commands to all selected units, **so that** my selection acts on my orders.

## US-13.11.6a.3 Verify Commands Respect Entity Capabilities

**As an** engine tester (P-27), **I want to** send a move command to a mixed selection of mobile
units and stationary buildings and verify only mobile units move, **so that** capability
filtering works correctly.

## US-13.11.6a.4 Verify Commands Route Through the Ability System

**As an** engine tester (P-27), **I want to** issue commands and verify they route through the
ability system for execution, **so that** command dispatch uses the shared ability
infrastructure.

## F-13.11.6b Formation Movement

## US-13.11.6b.1 Configure Formation Templates

**As a** designer (P-5), **I want to** define formation templates with shape, spacing, and
facing rules in the visual editor, **so that** formation behavior matches the game's
tactical design.

## US-13.11.6b.2 Move Units in Formation

**As a** player (P-23), **I want to** selected units to maintain relative positions during
group movement using formation templates (line, box, wedge, circle), **so that** my army
moves as a cohesive formation.

## US-13.11.6b.3 Verify Units Steer to Formation Slots

**As an** engine tester (P-27), **I want to** issue a formation move and verify each unit
steers to its assigned formation slot using the steering system, **so that** formation
positions are maintained during movement.

## F-13.11.6c Split and Subgroup Commands

## US-13.11.6c.1 Configure Split Methods

**As a** designer (P-5), **I want to** configure split methods (even split, type-based, manual
marquee within selection), **so that** splitting behavior matches the game's tactical options.

## US-13.11.6c.2 Split Selection Into Subgroups

**As a** player (P-23), **I want to** split my selection into subgroups and send each to a
different target, **so that** I can execute multi-pronged tactical maneuvers.

## US-13.11.6c.3 Verify Type-Based Split Correctness

**As an** engine tester (P-27), **I want to** split a mixed melee/ranged selection by type and
verify each subgroup contains only the correct entity types, **so that** type-based splitting
is accurate.

## F-13.11.6d Command History and Undo

## US-13.11.6d.1 Configure Undo Timeout Window

**As a** designer (P-5), **I want to** configure the undo timeout window duration, **so that**
the undo window matches the game's pacing.

## US-13.11.6d.2 Undo a Misclicked Command

**As a** player (P-23), **I want to** press Ctrl+Z to cancel my last command within a
configurable timeout window (default 2 seconds), **so that** I can recover from misclicks
during fast-paced gameplay.

## US-13.11.6d.3 Verify Undo Restores Pre-Command State

**As an** engine tester (P-27), **I want to** issue a move command, undo within the timeout, and
verify all entities return to their pre-command positions and targets, **so that** undo fully
restores the previous state.

## US-13.11.6d.4 Verify Undo Expires After Timeout

**As an** engine tester (P-27), **I want to** issue a command, wait past the timeout, attempt
undo, and verify it is rejected, **so that** the timeout window is enforced.

## F-13.11.7 2D Selection Box (Marquee Select)

## US-13.11.7.1 Configure Box Selection Appearance

**As a** designer (P-5), **I want to** configure the selection box color, opacity, and border
style in the visual editor, **so that** the box matches the game's UI style.

## US-13.11.7.2 Drag to Box-Select Units

**As a** player (P-23), **I want to** click-and-drag a rectangle to select all entities whose
screen-space bounds intersect the box, **so that** I can quickly select groups of units.

## US-13.11.7.3 Use Modifier Keys With Box Selection

**As a** player (P-23), **I want to** Shift+drag to add to existing selection and Ctrl+drag to
subtract, **so that** box selection composes with my current selection.

## US-13.11.7.4 Verify Box Selection Performance With Hundreds of Entities

**As an** engine tester (P-27), **I want to** box-select 500 entities simultaneously and verify
the operation completes within 1 frame, **so that** RTS-scale selection is performant.

## F-13.11.8 Selection Indicators and Visual Feedback

## US-13.11.8.1 Configure Selection Indicator Styles Per Genre

**As a** designer (P-5), **I want to** configure visual indicator style per genre preset and per
entity type (hero units get distinct indicators), **so that** selection visuals match the
game's art direction.

## US-13.11.8.2 See Selection Outlines on Selected Entities

**As a** player (P-23), **I want to** selected entities to display colored outlines (team-
colored for RTS, white for RPG) and hovered entities to display thinner preview outlines,
**so that** I always know what is selected and what I am about to select.

## US-13.11.8.3 See Selection Circles for RTS Units

**As a** player (P-23), **I want to** RTS units to show projected ground decal selection
circles with team-colored rings and health bar segments, **so that** unit positions and
status are visible.

## US-13.11.8.4 Verify Selection Visuals Update in Real-Time

**As an** engine tester (P-27), **I want to** rapidly change the selection and verify visual
indicators update within the same frame, **so that** selection visuals are never stale.
