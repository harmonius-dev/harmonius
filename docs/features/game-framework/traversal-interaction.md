# 13.17 — Traversal and Interaction

## Environmental Interaction

| ID        | Feature                         | Requirements |
|-----------|---------------------------------|--------------|
| F-13.17.1 | World Object Interaction System | R-13.17.1    |
| F-13.17.2 | Door and Lock System            | R-13.17.2    |
| F-13.17.3 | Physics Object Pickup and Throw | R-13.17.3    |

1. **F-13.17.1** — Detect and interact with world objects via look-at raycast or proximity trigger.
   When the player faces an interactable object within range, a UI prompt displays the interaction
   type and input binding (e.g., "E: Open Door"). Interactions are data-driven: each interactable
   entity has an `Interactable` component specifying interaction type, required items/keys,
   animation, duration, and logic graph (F-15.8.4) to execute. Interaction types include: instant
   (flip switch), channeled (hold to pick lock), and automatic (proximity trigger). Multiple
   interactions on one object are supported via a radial menu. Interactions respect game state —
   locked doors require keys, depleted nodes show "empty."
   - **Deps:** F-1.9.4 (Spatial Query), F-15.8.4 (Logic Graphs), F-6.2.1 (Input Actions)
2. **F-13.17.2** — Openable, lockable, and breakable doors as interactable entities. Doors define:
   open/close animation, locked state with key item requirement, lockpick difficulty (for
   lockpicking minigame), breakable HP (for forced entry), auto-close timer, and access permissions
   (per player, team, or faction). Doors can be one-way, double-swing, sliding, or portcullis-style
   with appropriate animations. AI NPCs respect door states — locked doors block pathfinding, and
   NPCs with keys can open locked doors. Door state changes fire events for alarms, quest triggers,
   and AI awareness.
   - **Deps:** F-13.17.1, F-7.1.1 (NavMesh), F-9.4.1 (Animation State Machine)
3. **F-13.17.3** — Pick up, carry, and throw physics-simulated objects in the world. Pickup attaches
   the object to a hold point in front of the character with configurable hold distance and spring
   stiffness. Carried objects collide with the environment and can be used to solve puzzles (place
   on pressure plates), block doorways, or throw at enemies for damage. Throw applies a force vector
   based on aim direction and configurable throw strength. Heavy objects slow the carrier. The
   system works with any entity that has a `RigidBody` and `Grabbable` component.
   - **Deps:** F-4.1.1 (Rigid Body ECS), F-4.4.1 (Ray Casts)

## Traversal

| ID         | Feature                    | Requirements |
|------------|----------------------------|--------------|
| F-13.17.4a | Traversal Detection System | R-13.17.4a   |
| F-13.17.4b | Vault and Mantle Actions   | R-13.17.4b   |
| F-13.17.4c | Wall Run                   | R-13.17.4c   |
| F-13.17.4d | Crouch Slide               | R-13.17.4d   |
| F-13.17.4  | e Balance Beam             | R-13.17.4e   |
| F-13.17.5a | Free-Climb System          | R-13.17.5a   |
| F-13.17.5b | Ladder System              | R-13.17.5b   |
| F-13.17.5c | Ledge Grab and Shimmy      | R-13.17.5c   |
| F-13.17.6  | Swimming and Diving        | R-13.17.6    |
| F-13.17.7  | Grappling Hook and Zipline | R-13.17.7    |

1. **F-13.17.4a** — Geometry analysis system that identifies traversal opportunities near the player
   character. Shape casts (box, sphere, capsule) project forward and downward from the character to
   detect nearby surfaces. Detected geometry is classified by dimensions and orientation: low
   obstacle (vault height), medium obstacle (mantle height), vertical surface (wall-run candidate),
   narrow surface (balance beam candidate), and low gap (slide-under candidate). Height thresholds,
   cast distances, and classification rules are configurable per game. Traversal geometry can be
   explicitly tagged in the level editor or auto-detected from collision shapes at runtime.
   - **Deps:** F-4.1.8 (Character Controller), F-4.4.1 (Shape Casts)
2. **F-13.17.4b** — Context-sensitive vault and mantle traversal over obstacles. Vault triggers on
   low obstacles (configurable height range, e.g., 0.3-0.8 m): the character performs a one-hand
   hop-over animation without stopping. Mantle triggers on medium obstacles (e.g., 0.8-2.0 m): the
   character reaches up, grabs the edge, and pulls up. Both actions require a minimum approach speed
   and deduct configurable stamina. Animations (F-9.4.1) blend from locomotion and IK (F-9.3.1)
   places hands on the obstacle surface. Root motion drives character displacement during the
   traversal.
   - **Deps:** F-13.17.4a, F-9.4.1 (Animation State Machine), F-9.3.1 (IK Solvers)
3. **F-13.17.4c** — Lateral wall-run on vertical surfaces triggered by sprinting along a wall at
   sufficient speed. The character transitions to a wall-run animation with feet on the wall surface
   and IK-driven hand contact. A gravity timer limits wall-run duration — the character gradually
   descends and eventually detaches. Wall-run speed, maximum duration, gravity curve, and minimum
   entry speed are configurable. Jump-off from wall-run launches the character away from the wall at
   a configurable angle. Requires continuous forward input to sustain.
   - **Deps:** F-13.17.4a, F-9.4.1 (Animation State Machine), F-9.3.1 (IK Solvers)
4. **F-13.17.4d** — Momentum-based crouch slide triggered by crouch input while sprinting. The
   character drops into a slide posture with reduced collision height, traveling forward with
   decelerating velocity. Slide distance scales with entry speed. Slopes affect slide: downhill
   increases distance, uphill decreases it. Slide can transition into crouch-walk if the player
   remains crouched, or back to sprint if the player stands. Stamina cost and cooldown are
   configurable. Slide can pass under low obstacles detected by the traversal system (F-13.17.4a).
   - **Deps:** F-13.17.4a, F-4.1.8 (Character Controller), F-9.4.1 (Animation State Machine)
5. **F-13.17.4** — Slow traversal on narrow surfaces detected by the traversal system. When the
   character steps onto a surface narrower than a configurable width threshold, locomotion switches
   to balance mode: reduced walk speed, arms-out balance animation, and procedural lateral sway.
   Camera adds subtle wobble to reinforce instability. Falling off occurs if the player moves too
   fast, takes damage, or exceeds the sway tolerance. Balance surfaces are detected by width
   analysis or explicitly tagged in the level editor.
   - **Deps:** F-13.17.4a, F-9.4.1 (Animation State Machine)
6. **F-13.17.5a** — Climbable surface traversal with IK-driven hand and foot placement. Surfaces
   tagged with a `Climbable` component define grip points as an auto-generated grid or hand-placed
   markers. The climbing system uses IK (F-9.3.1) to position hands and feet on grip points with
   procedural reach animations between grips. Stamina drains continuously while climbing — depletion
   causes the character to fall. Climb speed, stamina drain rate, grip point spacing, and reach
   distance are configurable. Climbable surfaces integrate with AI navigation for NPC climbing. Rest
   points on climb surfaces can pause stamina drain.
   - **Deps:** F-13.17.4a, F-9.3.1 (IK Solvers), F-9.3.5 (Foot Placement)
7. **F-13.17.5b** — Simplified vertical climb on ladder entities. Entering a ladder's interaction
   trigger locks the character to the ladder with fixed vertical movement (up/down input only).
   Enter and exit animations play at the bottom and top of the ladder. Climb speed is configurable.
   The character can dismount mid-ladder by pressing a directional input away from the ladder,
   dropping to the ground. Ladders do not consume stamina. AI NPCs can use ladders for vertical
   pathfinding.
   - **Deps:** F-13.17.1 (Interaction), F-9.4.1 (Animation State Machine)
8. **F-13.17.5c** — Edge detection, lateral movement, and pull-up along horizontal ledges. When the
   character is airborne near a horizontal edge (detected by shape cast), a ledge grab triggers: the
   character catches the edge and hangs. Shimmy allows lateral movement along the edge using
   left/right input. Pull-up transitions the character from hanging to standing on top of the ledge.
   Drop input releases the grab. Ledge grab requires sufficient stamina and drains stamina while
   hanging. Edges are auto-detected from collision geometry or tagged explicitly. IK (F-9.3.1)
   positions hands on the ledge surface.
   - **Deps:** F-13.17.4a, F-9.3.1 (IK Solvers), F-9.4.1 (Animation State Machine)
9. **F-13.17.6** — Water volume detection transitions the character controller from ground
   locomotion to swim locomotion. Surface swimming uses horizontal movement with a bobbing
   animation. Diving transitions to 3D underwater movement with configurable swim speed and
   buoyancy. An oxygen meter drains while submerged; reaching zero causes drowning damage.
   Underwater visual effects (blue tint, caustic light patterns, depth fog from F-11.4.7) activate
   when the camera submerges. Water surface entry/exit plays splash VFX and audio. Swimming stamina
   integrates with the survival system (F-13.14.6) if enabled. Aquatic mounts (F-13.15.3) use faster
   underwater locomotion.
   - **Deps:** F-4.1.8 (Character Controller), F-3.4.1 (Water), F-11.4.7 (Underwater Caustics)
10. **F-13.17.7** — Attach to surfaces or anchor points and traverse via swing or pull. Grappling
    hook: fire a projectile that attaches to valid surfaces, then either pull the character toward
    the anchor (grapple-pull) or swing from the attachment point with pendulum physics
    (grapple-swing). Zipline: attach to cable entities and slide along them with gravity-driven
    speed and optional braking. Both systems use the rope/cable physics from F-4.3.4 for visual and
    physical simulation. Anchor points are defined by level designers or auto-detected from geometry
    (ledge edges, beams). Grapple range, pull speed, and swing parameters are configurable per
    equipment item.
    - **Deps:** F-4.3.4 (Spring and Rope Joints), F-13.10.5 (Ranged Combat - projectile as hook)
