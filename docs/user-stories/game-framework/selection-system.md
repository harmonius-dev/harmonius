# User Stories — Selection and Picking (13.11)

## Picking

| ID           | Persona              |
|--------------|----------------------|
| US-13.11.1.1 | game designer (P-5)  |
| US-13.11.1.2 | game developer (P-15)|
| US-13.11.2.1 | game designer (P-5)  |
| US-13.11.2.2 | player (P-23)        |

1. **US-13.11.1.1** — **As a** game designer (P-5), **I want** ray-based 3D picking with
   priority-based filtering using the spatial index, **so that** interactive objects take precedence
   over scenery.
2. **US-13.11.1.2** — **As a** game developer (P-15), **I want** pick results to include hit entity,
   world position, surface normal, and hit bone, **so that** gameplay logic reacts to precise hit
   data.
3. **US-13.11.2.1** — **As a** game designer (P-5), **I want** screen-space 2D picking for UI
   widgets and sprites with configurable touch slop, **so that** picking works on touch devices.
4. **US-13.11.2.2** — **As a** player (P-23), **I want** UI picks to take priority over world picks,
   **so that** I always interact with visible UI elements first.

## Selection State

| ID            | Persona              |
|---------------|----------------------|
| US-13.11.3.1  | game designer (P-5)  |
| US-13.11.3.2  | game developer (P-15)|
| US-13.11.4a.1 | game designer (P-5)  |
| US-13.11.4b.1 | game designer (P-5)  |
| US-13.11.4c.1 | player (P-23)        |
| US-13.11.4d.1 | game designer (P-5)  |

1. **US-13.11.3.1** — **As a** game designer (P-5), **I want** selection modes (single, additive,
   subtractive, toggle, exclusive) configurable per project, **so that** selection behavior matches
   the genre.
2. **US-13.11.3.2** — **As a** game developer (P-15), **I want** selection state as ECS components
   with change events through the observer system, **so that** UI and gameplay react to selection
   changes.
3. **US-13.11.4a.1** — **As a** game designer (P-5), **I want** an RTS selection preset with box
   select, control groups, and select-all-of-type, **so that** RTS projects start with working
   selection.
4. **US-13.11.4b.1** — **As a** game designer (P-5), **I want** an RPG selection preset with
   tab-cycling, friendly/hostile filtering, and target-of-target, **so that** RPG projects start
   with working targeting.
5. **US-13.11.4c.1** — **As a** player (P-23), **I want** auto-target nearest enemy with lock-on
   toggle, **so that** action game targeting is intuitive.
6. **US-13.11.4d.1** — **As a** game designer (P-5), **I want** a builder/sandbox preset with
   multi-select, transform gizmos, and hierarchy selection, **so that** builder projects start with
   working selection.

## Selection Groups and Commands

| ID            | Persona              |
|---------------|----------------------|
| US-13.11.5.1  | game designer (P-5)  |
| US-13.11.5.2  | player (P-23)        |
| US-13.11.6a.1 | game designer (P-5)  |
| US-13.11.6b.1 | game designer (P-5)  |
| US-13.11.6c.1 | game designer (P-5)  |
| US-13.11.6d.1 | player (P-23)        |

1. **US-13.11.5.1** — **As a** game designer (P-5), **I want** named persistent selection groups
   with hotkey bindings and set operations, **so that** group management is flexible.
2. **US-13.11.5.2** — [game-specific] **As a** player (P-23), **I want** control groups (Ctrl+1-9)
   that persist across save/load, **so that** my unit organization is preserved.
3. **US-13.11.6a.1** — **As a** game designer (P-5), **I want** commands (move, attack, stop,
   patrol) dispatched to all selected entities, **so that** group control is unified.
4. **US-13.11.6b.1** — **As a** game designer (P-5), **I want** formation templates (line, box,
   wedge, circle) for group movement, **so that** units maintain relative positions.
5. **US-13.11.6c.1** — **As a** game designer (P-5), **I want** split commands dividing selection
   into subgroups sent to different targets, **so that** tactical multi-point commands work.
6. **US-13.11.6d.1** — **As a** player (P-23), **I want** Ctrl+Z to cancel the last command within a
   timeout, **so that** I undo accidental orders.

## Selection Visuals

| ID           | Persona              |
|--------------|----------------------|
| US-13.11.7.1 | game designer (P-5)  |
| US-13.11.7.2 | player (P-23)        |
| US-13.11.8.1 | game designer (P-5)  |
| US-13.11.8.2 | player (P-23)        |

1. **US-13.11.7.1** — **As a** game designer (P-5), **I want** a screen-space marquee selection box
   with modifier key support and configurable appearance, **so that** box selection is visual.
2. **US-13.11.7.2** — **As a** player (P-23), **I want** entities highlighted during drag to preview
   the selection before release, **so that** I see what I will select.
3. **US-13.11.8.1** — **As a** game designer (P-5), **I want** selection indicators (outlines,
   ground decals, health bars) configurable per genre preset and entity type, **so that** selection
   feedback matches the art style.
4. **US-13.11.8.2** — **As a** player (P-23), **I want** selected entities to show colored outlines
   and hovered entities to show a thinner preview, **so that** selection state is always visible.
