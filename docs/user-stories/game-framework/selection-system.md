# User Stories — 13.11 Selection and Picking

## F-13.11.1 3D World Picking

| ID           | Persona              | Features  | Requirements |
|--------------|----------------------|-----------|--------------|
| US-13.11.1.1 | designer (P-5)       | F-13.11.1 | R-13.11.1    |
| US-13.11.1.2 | player (P-23)        | F-13.11.1 | R-13.11.1    |
| US-13.11.1.3 | engine tester (P-27) | F-13.11.1 | R-13.11.1    |
| US-13.11.1.4 | engine tester (P-27) | F-13.11.1 | R-13.11.1    |

1. **US-13.11.1.1** — I want to configure pick priority (interactive objects over scenery) and
   filtering rules (only entities with Selectable component) in the visual editor so that picking
   behavior matches the game's interaction design
2. **US-13.11.1.2** — I want to click on entities in the 3D world and have the nearest selectable
   entity picked instantly so that I can interact with the game world responsively
3. **US-13.11.1.3** — I want to perform 1000 pick operations and verify all complete in under 1ms
   each so that picking is responsive enough for 60 FPS gameplay
4. **US-13.11.1.4** — I want to perform a pick through multiple overlapping entities and verify all
   are returned sorted by distance with correct hit positions and normals so that pick-through
   scenarios work correctly

## F-13.11.2 2D Screen-Space Picking

| ID           | Persona              | Features  | Requirements |
|--------------|----------------------|-----------|--------------|
| US-13.11.2.1 | designer (P-5)       | F-13.11.2 | R-13.11.2    |
| US-13.11.2.2 | player (P-23)        | F-13.11.2 | R-13.11.2    |
| US-13.11.2.3 | engine tester (P-27) | F-13.11.2 | R-13.11.2    |
| US-13.11.2.4 | engine tester (P-27) | F-13.11.2 | R-13.11.2    |

1. **US-13.11.2.1** — I want to configure touch slop radius per widget or entity type on touch
   devices so that finger targeting is accurate on mobile screens
2. **US-13.11.2.2** — I want to click on UI widgets and 2D game entities with screen- space testing
   so that 2D interactions work accurately with UI always taking priority
3. **US-13.11.2.3** — I want to click where a UI widget overlaps a world entity and verify the UI
   widget is picked so that UI always takes priority over world picks
4. **US-13.11.2.4** — I want to click on the transparent area of a sprite and verify no pick occurs,
   while clicking on the opaque area succeeds so that per-pixel alpha mask picking works correctly

## F-13.11.3 Selection State Management

| ID           | Persona              | Features  | Requirements |
|--------------|----------------------|-----------|--------------|
| US-13.11.3.1 | designer (P-5)       | F-13.11.3 | R-13.11.3    |
| US-13.11.3.2 | player (P-23)        | F-13.11.3 | R-13.11.3    |
| US-13.11.3.3 | engine tester (P-27) | F-13.11.3 | R-13.11.3    |
| US-13.11.3.4 | engine tester (P-27) | F-13.11.3 | R-13.11.3    |

1. **US-13.11.3.1** — I want to configure selection mode rules and modifier key bindings in the
   visual editor so that selection behavior matches the game's genre conventions
2. **US-13.11.3.2** — I want to use single click, Shift+click (additive), Ctrl+click (subtractive),
   and toggle selection modes so that I can build selections precisely
3. **US-13.11.3.3** — I want to add and remove entities from the selection and verify selection
   change events fire through the observer system so that UI, audio, and gameplay systems are
   notified of every selection change
4. **US-13.11.3.4** — I want to select and deselect entities and verify the Selected component is
   added on select and removed on deselect so that ECS queries on Selected entities work correctly

## F-13.11.4a RTS Selection Preset

| ID         | Persona              | Features  | Requirements |
|------------|----------------------|-----------|--------------|
| US-13.11.4 | designer (P-5)       | F-13.11.4 | R-13.11.4    |
| US-13.11.4 | player (P-23)        | F-13.11.4 | R-13.11.4    |
| US-13.11.4 | player (P-23)        | F-13.11.4 | R-13.11.4    |
| US-13.11.4 | engine tester (P-27) | F-13.11.4 | R-13.11.4    |

1. **US-13.11.4** — I want to customize RTS selection input bindings, filtering rules, and visual
   feedback in the visual editor so that the RTS preset matches my game's conventions
2. **US-13.11.4** — I want to box select units, double-click to select all of same type, and
   assign/recall control groups with Ctrl+1-9 / 1-9 so that I can manage armies using familiar RTS
   conventions
3. **US-13.11.4** — I want to select all idle units or all military units with dedicated hotkeys so
   that I can quickly find and command unused forces
4. **US-13.11.4** — I want to enable the RTS preset and verify box select, control groups,
   select-all-of-type, and subgroup tabs all function correctly so that the preset provides a
   complete RTS selection experience

## F-13.11.4b RPG Selection Preset

| ID         | Persona              | Features  | Requirements |
|------------|----------------------|-----------|--------------|
| US-13.11.4 | designer (P-5)       | F-13.11.4 | R-13.11.4    |
| US-13.11.4 | player (P-23)        | F-13.11.4 | R-13.11.4    |
| US-13.11.4 | player (P-23)        | F-13.11.4 | R-13.11.4    |
| US-13.11.4 | engine tester (P-27) | F-13.11.4 | R-13.11.4    |

1. **US-13.11.4** — I want to configure tab-cycle ordering with optional priority for quest targets
   so that targeting behavior helps players find relevant enemies
2. **US-13.11.4** — I want to press Tab to cycle through nearby enemies with hostile filtering and
   distance-based ordering so that I can target enemies quickly during combat
3. **US-13.11.4** — I want to see the target-of-target display when I have an enemy selected so that
   I know who the enemy is attacking
4. **US-13.11.4** — I want to verify tab-cycling visits all nearby hostile entities in distance
   order without skipping or repeating so that the RPG preset's targeting cycle is correct

## F-13.11.4c Action Selection Preset

| ID         | Persona              | Features  | Requirements |
|------------|----------------------|-----------|--------------|
| US-13.11.4 | designer (P-5)       | F-13.11.4 | R-13.11.4    |
| US-13.11.4 | player (P-23)        | F-13.11.4 | R-13.11.4    |
| US-13.11.4 | player (P-23)        | F-13.11.4 | R-13.11.4    |
| US-13.11.4 | engine tester (P-27) | F-13.11.4 | R-13.11.4    |

1. **US-13.11.4** — I want to configure lock-on camera centering behavior and target switching arc
   angle so that the action preset feels right for my game's combat pace
2. **US-13.11.4** — I want to auto-target the nearest enemy and toggle lock-on with camera focus so
   that I can stay focused on my target during fast-paced action combat
3. **US-13.11.4** — I want to switch targets by flicking the right stick to cycle through enemies
   within a configurable arc so that target switching is fast and intuitive on gamepad
4. **US-13.11.4** — I want to lock on to a target and verify the camera stays centered on the target
   through all movement and combat actions so that lock-on camera focus is stable

## F-13.11.4d Builder/Sandbox Selection Preset

| ID         | Persona              | Features  | Requirements |
|------------|----------------------|-----------|--------------|
| US-13.11.4 | designer (P-5)       | F-13.11.4 | R-13.11.4    |
| US-13.11.4 | player (P-23)        | F-13.11.4 | R-13.11.4    |
| US-13.11.4 | player (P-23)        | F-13.11.4 | R-13.11.4    |
| US-13.11.4 | engine tester (P-27) | F-13.11.4 | R-13.11.4    |

1. **US-13.11.4** — I want to configure hierarchy selection (select parent selects children) and
   group/ungroup controls so that the builder preset matches the game's construction mechanics
2. **US-13.11.4** — I want to multi-select objects with transform gizmos and group/ ungroup
   operations so that I can manipulate complex constructions efficiently
3. **US-13.11.4** — I want to use lasso selection in addition to box select for irregular selection
   regions so that I can select objects in complex arrangements
4. **US-13.11.4** — I want to select a parent entity and verify all children are also selected, then
   deselect the parent and verify children are deselected so that hierarchy selection propagation is
   correct

## F-13.11.5 Runtime Selection Groups

| ID           | Persona              | Features  | Requirements |
|--------------|----------------------|-----------|--------------|
| US-13.11.5.1 | designer (P-5)       | F-13.11.5 | R-13.11.5    |
| US-13.11.5.2 | player (P-23)        | F-13.11.5 | R-13.11.5    |
| US-13.11.5.3 | player (P-23)        | F-13.11.5 | R-13.11.5    |
| US-13.11.5.4 | engine tester (P-27) | F-13.11.5 | R-13.11.5    |
| US-13.11.5.5 | engine tester (P-27) | F-13.11.5 | R-13.11.5    |

1. **US-13.11.5.1** — I want to configure how groups display in the HUD group bar with icons and
   hotkey labels so that group visibility matches the game's UI design
2. **US-13.11.5.2** — I want to assign selected entities to named groups with hotkey bindings
   (Ctrl+number) and recall them (number) so that I can quickly switch between saved selections
   during gameplay
3. **US-13.11.5.3** — I want to perform union, intersection, and difference operations between
   groups so that I can combine or refine selections without manual picking
4. **US-13.11.5.4** — I want to create groups, save, load, and verify all groups and their entity
   memberships are preserved so that groups survive save/load cycles
5. **US-13.11.5.5** — I want to add and remove entities from groups and verify the GroupMembership
   component updates correctly on each entity so that ECS queries on group membership are accurate

## F-13.11.6a Basic Command Dispatch

| ID         | Persona              | Features  | Requirements |
|------------|----------------------|-----------|--------------|
| US-13.11.6 | designer (P-5)       | F-13.11.6 | R-13.11.6    |
| US-13.11.6 | player (P-23)        | F-13.11.6 | R-13.11.6    |
| US-13.11.6 | engine tester (P-27) | F-13.11.6 | R-13.11.6    |
| US-13.11.6 | engine tester (P-27) | F-13.11.6 | R-13.11.6    |

1. **US-13.11.6** — I want to configure which commands each entity type supports (move only for
   mobile units, not buildings) so that commands respect entity capabilities
2. **US-13.11.6** — I want to issue move, attack, ability, stop, patrol, and hold position commands
   to all selected units so that my selection acts on my orders
3. **US-13.11.6** — I want to send a move command to a mixed selection of mobile units and
   stationary buildings and verify only mobile units move so that capability filtering works
   correctly
4. **US-13.11.6** — I want to issue commands and verify they route through the ability system for
   execution so that command dispatch uses the shared ability infrastructure

## F-13.11.6b Formation Movement

| ID         | Persona              | Features  | Requirements |
|------------|----------------------|-----------|--------------|
| US-13.11.6 | designer (P-5)       | F-13.11.6 | R-13.11.6    |
| US-13.11.6 | player (P-23)        | F-13.11.6 | R-13.11.6    |
| US-13.11.6 | engine tester (P-27) | F-13.11.6 | R-13.11.6    |

1. **US-13.11.6** — I want to define formation templates with shape, spacing, and facing rules in
   the visual editor so that formation behavior matches the game's tactical design
2. **US-13.11.6** — I want to selected units to maintain relative positions during group movement
   using formation templates (line, box, wedge, circle) so that my army moves as a cohesive
   formation
3. **US-13.11.6** — I want to issue a formation move and verify each unit steers to its assigned
   formation slot using the steering system so that formation positions are maintained during
   movement

## F-13.11.6c Split and Subgroup Commands

| ID         | Persona              | Features  | Requirements |
|------------|----------------------|-----------|--------------|
| US-13.11.6 | designer (P-5)       | F-13.11.6 | R-13.11.6    |
| US-13.11.6 | player (P-23)        | F-13.11.6 | R-13.11.6    |
| US-13.11.6 | engine tester (P-27) | F-13.11.6 | R-13.11.6    |

1. **US-13.11.6** — I want to configure split methods (even split, type-based, manual marquee within
   selection) so that splitting behavior matches the game's tactical options
2. **US-13.11.6** — I want to split my selection into subgroups and send each to a different target
   so that I can execute multi-pronged tactical maneuvers
3. **US-13.11.6** — I want to split a mixed melee/ranged selection by type and verify each subgroup
   contains only the correct entity types so that type-based splitting is accurate

## F-13.11.6d Command History and Undo

| ID         | Persona              | Features  | Requirements |
|------------|----------------------|-----------|--------------|
| US-13.11.6 | designer (P-5)       | F-13.11.6 | R-13.11.6    |
| US-13.11.6 | player (P-23)        | F-13.11.6 | R-13.11.6    |
| US-13.11.6 | engine tester (P-27) | F-13.11.6 | R-13.11.6    |
| US-13.11.6 | engine tester (P-27) | F-13.11.6 | R-13.11.6    |

1. **US-13.11.6** — I want to configure the undo timeout window duration so that the undo window
   matches the game's pacing
2. **US-13.11.6** — I want to press Ctrl+Z to cancel my last command within a configurable timeout
   window (default 2 seconds) so that I can recover from misclicks during fast-paced gameplay
3. **US-13.11.6** — I want to issue a move command, undo within the timeout, and verify all entities
   return to their pre-command positions and targets so that undo fully restores the previous state
4. **US-13.11.6** — I want to issue a command, wait past the timeout, attempt undo, and verify it is
   rejected so that the timeout window is enforced

## F-13.11.7 2D Selection Box (Marquee Select)

| ID           | Persona              | Features  | Requirements |
|--------------|----------------------|-----------|--------------|
| US-13.11.7.1 | designer (P-5)       | F-13.11.7 | R-13.11.7    |
| US-13.11.7.2 | player (P-23)        | F-13.11.7 | R-13.11.7    |
| US-13.11.7.3 | player (P-23)        | F-13.11.7 | R-13.11.7    |
| US-13.11.7.4 | engine tester (P-27) | F-13.11.7 | R-13.11.7    |

1. **US-13.11.7.1** — I want to configure the selection box color, opacity, and border style in the
   visual editor so that the box matches the game's UI style
2. **US-13.11.7.2** — I want to click-and-drag a rectangle to select all entities whose screen-space
   bounds intersect the box so that I can quickly select groups of units
3. **US-13.11.7.3** — I want to Shift+drag to add to existing selection and Ctrl+drag to subtract so
   that box selection composes with my current selection
4. **US-13.11.7.4** — I want to box-select 500 entities simultaneously and verify the operation
   completes within 1 frame so that RTS-scale selection is performant

## F-13.11.8 Selection Indicators and Visual Feedback

| ID           | Persona              | Features  | Requirements |
|--------------|----------------------|-----------|--------------|
| US-13.11.8.1 | designer (P-5)       | F-13.11.8 | R-13.11.8    |
| US-13.11.8.2 | player (P-23)        | F-13.11.8 | R-13.11.8    |
| US-13.11.8.3 | player (P-23)        | F-13.11.8 | R-13.11.8    |
| US-13.11.8.4 | engine tester (P-27) | F-13.11.8 | R-13.11.8    |

1. **US-13.11.8.1** — I want to configure visual indicator style per genre preset and per entity
   type (hero units get distinct indicators) so that selection visuals match the game's art
   direction
2. **US-13.11.8.2** — I want to selected entities to display colored outlines (team- colored for
   RTS, white for RPG) and hovered entities to display thinner preview outlines so that I always
   know what is selected and what I am about to select
3. **US-13.11.8.3** — I want to RTS units to show projected ground decal selection circles with
   team-colored rings and health bar segments so that unit positions and status are visible
4. **US-13.11.8.4** — I want to rapidly change the selection and verify visual indicators update
   within the same frame so that selection visuals are never stale
