# R-13.11 — Selection and Picking Requirements

## Picking

1. **R-13.11.1** — The engine **SHALL** cast rays from the camera through screen-space coordinates
   using the spatial index for broadphase and per-entity collision shapes for precise hit testing,
   supporting single-click, priority-based, and multi-layer pick modes, with results including hit
   entity, world position, surface normal, and hit bone.
   - **Rationale:** Spatial-index-accelerated picking ensures sub-millisecond latency for responsive
     interaction.
   - **Verification:** Click on an entity and verify the correct entity, position, and normal are
     returned. Click where two entities overlap and verify priority-based picks the interactive one.
     Verify multi-layer returns all entities sorted by distance.

2. **R-13.11.2** — The engine **SHALL** support screen-space 2D picking for UI widgets and sprites
   with configurable touch slop radius, z-order respect, and UI-over-world priority.
   - **Rationale:** Touch slop improves finger targeting on mobile; UI priority prevents accidental
     world interaction.
   - **Verification:** Tap a UI button overlapping a world entity and verify the UI button is
     picked. On a touch device, tap near a small button and verify touch slop expands the hit area.

## Selection State

3. **R-13.11.3** — The engine **SHALL** provide a centralized selection system as ECS resources and
   components with selection modes (single, additive, subtractive, toggle, exclusive), change events
   through the observer system, and data-driven genre presets (RTS, RPG, action, builder).
   - **Rationale:** ECS-integrated selection enables efficient queries on selected entities.
   - **Verification:** Select an entity and verify the Selected component is added. Shift-click
     another and verify additive selection. Change to exclusive mode and verify only one entity is
     selectable. Activate the RTS preset and verify box select and control groups are enabled.

4. **R-13.11.4** — The engine **SHALL** provide genre selection presets: RTS (box select, control
   groups, select-all-of-type), RPG (tab-cycling, friendly/hostile filtering), action (auto-target,
   lock-on toggle), and builder (multi-select, gizmos, hierarchy).
   - **Rationale:** Genre presets give projects working selection out of the box.
   - **Verification:** Activate each preset and verify its input bindings and behaviors work. In RTS
     preset, verify box select selects all entities in the rectangle. In RPG preset, verify tab
     cycles through nearby enemies.

## Selection Groups and Commands

5. **R-13.11.5** — The engine **SHALL** support named persistent selection groups with hotkey
   bindings, set operations (union, intersection, difference), and persistence across save/load.
   - **Rationale:** Persistent groups enable tactical unit management in RTS and strategy games.
   - **Verification:** Create group 1 with 5 entities. Save, load, and verify the group persists.
     Create group 2 and verify union of groups 1 and 2 produces the correct set.

6. **R-13.11.6** — The engine **SHALL** dispatch commands (move, attack, stop, patrol) to all
   entities in the current selection, with formation templates for group movement, split commands
   for multi-target dispatch, and a configurable-timeout command undo.
   - **Rationale:** Unified command dispatch with formations enables RTS-style group control.
   - **Verification:** Select 10 entities and issue a move command. Verify all entities move. Issue
     a formation move and verify entities maintain relative positions. Issue a split command and
     verify subgroups go to different targets. Ctrl+Z within timeout and verify undo.

## Selection Visuals

7. **R-13.11.7** — The engine **SHALL** render a screen-space marquee selection box with modifier
   key support, preview highlighting during drag, and configurable appearance.
   - **Rationale:** Visual box selection provides immediate feedback for multi-entity selection.
   - **Verification:** Click and drag a box. Verify entities within are highlighted during drag.
     Release and verify they become selected. Shift+drag and verify additive behavior.

8. **R-13.11.8** — The engine **SHALL** render selection indicators (outlines, ground decals, health
   bars) configurable per genre preset and entity type, with hover preview using a thinner outline.
   - **Rationale:** Visual feedback ensures players always know what is selected or hovered.
   - **Verification:** Select an entity and verify outline renders. Hover over another and verify
     thinner preview outline. Change genre preset and verify indicator style changes.
