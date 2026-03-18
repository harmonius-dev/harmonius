# R-13.17 -- Traversal and Interaction Requirements

| ID         | Derived From                                                         |
|------------|----------------------------------------------------------------------|
| R-13.17.1  | [F-13.17.1](../../features/game-framework/traversal-interaction.md)  |
| R-13.17.2  | [F-13.17.2](../../features/game-framework/traversal-interaction.md)  |
| R-13.17.3  | [F-13.17.3](../../features/game-framework/traversal-interaction.md)  |
| R-13.17.4a | [F-13.17.4a](../../features/game-framework/traversal-interaction.md) |
| R-13.17.4b | [F-13.17.4b](../../features/game-framework/traversal-interaction.md) |
| R-13.17.4c | [F-13.17.4c](../../features/game-framework/traversal-interaction.md) |
| R-13.17.4d | [F-13.17.4d](../../features/game-framework/traversal-interaction.md) |
| R-13.17.4  | [F-13.17.4e](../../features/game-framework/traversal-interaction.md) |
| R-13.17.5a | [F-13.17.5a](../../features/game-framework/traversal-interaction.md) |
| R-13.17.5b | [F-13.17.5b](../../features/game-framework/traversal-interaction.md) |
| R-13.17.5c | [F-13.17.5c](../../features/game-framework/traversal-interaction.md) |
| R-13.17.6  | [F-13.17.6](../../features/game-framework/traversal-interaction.md)  |
| R-13.17.7  | [F-13.17.7](../../features/game-framework/traversal-interaction.md)  |

1. **R-13.17.1** — The engine **SHALL** detect interactable objects via look-at raycast or proximity
   trigger, display a context-sensitive UI prompt with interaction type and input binding, and
   execute data-driven interaction logic (instant, channeled, or automatic) defined by an
   Interactable component referencing a logic graph, with support for multiple interactions via
   radial menu.
   - **Rationale:** Data-driven interactions with logic graph execution let designers create new
     interactable objects entirely in the visual editor without code changes.
   - **Verification:** Place three interactable objects: an instant switch, a channeled lock, and an
     automatic proximity trigger. Verify the switch toggles immediately on input, the lock requires
     hold duration to complete, and the trigger fires on proximity entry. Add two interactions to
     one object and confirm the radial menu appears for selection.
2. **R-13.17.2** — The engine **SHALL** support openable, lockable, and breakable doors with
   key-item requirements, lockpick difficulty, breakable HP, auto-close timers, and access
   permissions, where AI NPCs respect door states for pathfinding and door state changes fire events
   for alarms and quest triggers.
   - **Rationale:** Doors as stateful interactables that integrate with AI pathfinding and event
     systems create meaningful spatial barriers for both gameplay and level design.
   - **Verification:** Lock a door with a key requirement. Verify a player without the key cannot
     open it, a player with the key can, and an AI NPC with the key opens it during pathfinding.
     Break a door by dealing damage exceeding its HP and confirm it is destroyed. Verify the
     auto-close timer closes the door after the configured duration.
3. **R-13.17.3** — The engine **SHALL** allow picking up, carrying, and throwing entities with
   RigidBody and Grabbable components, where carried objects attach to a hold point with spring
   stiffness, collide with the environment, slow the carrier based on weight, and apply a force
   vector on throw based on aim direction and configurable throw strength.
   - **Rationale:** Physics-based object interaction enables emergent puzzle solving and combat
     scenarios using the existing rigid body simulation.
   - **Verification:** Pick up a physics object and verify it attaches to the hold point and
     collides with walls when the player moves near them. Carry a heavy object and confirm movement
     speed decreases. Throw the object at an enemy and verify it applies damage. Place a carried
     object on a pressure plate and confirm the plate activates.
4. **R-13.17.4a** — The engine **SHALL** detect traversal opportunities via shape casts, classify
   geometry by dimensions and orientation (low obstacle, medium obstacle, vertical surface, narrow
   surface, low gap), with configurable height thresholds and cast distances, and support both
   editor tagging and auto-detection from collision shapes.
   - **Rationale:** Reliable geometry detection and classification is the foundation for all
     context-sensitive traversal actions.
   - **Verification:** Place a low wall, medium wall, vertical surface, narrow beam, and low gap.
     Verify the detection system classifies each correctly. Verify auto-detection works without
     editor tags on standard collision shapes. Verify tagged geometry overrides auto-detection.
5. **R-13.17.4b** — The engine **SHALL** execute vault over low obstacles and mantle onto medium
   obstacles with configurable height ranges, minimum approach speed, stamina cost,
   root-motion-driven displacement, and IK hand placement.
   - **Rationale:** Vault and mantle are the most common traversal actions and must feel fluid with
     correct hand placement for visual believability.
   - **Verification:** Run toward a low wall and verify vault triggers with the correct animation
     and IK hand placement. Run toward a medium wall and verify mantle triggers. Approach below
     minimum speed and verify neither action triggers. Verify stamina is deducted.
6. **R-13.17.4c** — The engine **SHALL** support lateral wall-run on vertical surfaces with
   configurable speed, maximum duration via gravity timer, minimum entry speed, IK hand contact, and
   jump-off at a configurable angle.
   - **Rationale:** Wall-run transforms vertical surfaces into traversal opportunities, expanding
     movement options in vertical level designs.
   - **Verification:** Sprint along a vertical wall and verify wall-run activates. Verify the
     character descends over time per the gravity timer. Verify jump-off launches away from the wall
     at the configured angle. Approach below minimum speed and verify wall-run does not trigger.
7. **R-13.17.4d** — The engine **SHALL** support momentum-based crouch slide triggered by crouch
   input while sprinting, with distance scaling by entry speed, slope interaction (downhill extends,
   uphill shortens), configurable stamina cost, and ability to pass under low obstacles.
   - **Rationale:** Crouch slide maintains momentum through tight spaces and adds dynamic movement
     options that reward timing and positioning.
   - **Verification:** Sprint and trigger slide. Verify slide distance scales with entry speed.
     Slide downhill and verify extended distance. Slide under a low obstacle and verify the
     character passes through. Verify stamina is deducted.
8. **R-13.17.4** — The engine **SHALL** switch locomotion to balance mode on narrow surfaces below a
   configurable width threshold, with reduced walk speed, balance animation with procedural sway,
   camera wobble, and fall-off on excessive speed or damage.
   - **Rationale:** Balance traversal adds tension and skill-based challenge to narrow surface
     navigation, diversifying the traversal vocabulary.
   - **Verification:** Step onto a narrow surface and verify balance mode activates with reduced
     speed and sway animation. Move too fast and verify the character falls off. Take damage while
     balancing and verify fall-off triggers. Walk carefully to the end and verify normal locomotion
     resumes.
9. **R-13.17.5a** — The engine **SHALL** support free-climb on surfaces tagged with a Climbable
   component, with IK-driven hand/foot placement on grip points (auto-generated or hand-placed),
   continuous stamina drain causing fall on depletion, configurable climb speed, and AI NPC climbing
   integration.
   - **Rationale:** IK-driven climbing with stamina constraints creates skill-based vertical
     traversal that integrates with the survival and animation systems.
   - **Verification:** Climb a tagged surface and verify IK places hands and feet on grip points.
     Drain stamina to zero during climbing and confirm the character falls. Verify climb speed
     matches the configured value. Verify an AI NPC can climb the same surface.
10. **R-13.17.5b** — The engine **SHALL** support ladder traversal with fixed vertical movement,
    enter/exit animations at bottom and top, configurable climb speed, mid-ladder dismount, and no
    stamina consumption.
    - **Rationale:** Ladders provide simple, reliable vertical movement for basic elevation changes
      without the complexity of the free-climb system.
    - **Verification:** Enter a ladder from the bottom and verify the enter animation plays. Climb
      up and verify fixed vertical movement. Reach the top and verify the exit animation plays.
      Dismount mid-ladder and verify the character drops. Verify stamina is not consumed.
11. **R-13.17.5c** — The engine **SHALL** detect horizontal edges for ledge grab when the character
    is airborne, support lateral shimmy movement along the edge, pull-up to the top, drop release,
    and stamina drain while hanging, with IK hand placement.
    - **Rationale:** Ledge mechanics expand traversal options by making horizontal edges into
      climbable paths, rewarding spatial awareness.
    - **Verification:** Jump near a horizontal edge and verify ledge grab triggers. Shimmy left and
      right and verify lateral movement along the edge. Pull up and verify the character stands on
      top. Drop and verify release. Deplete stamina while hanging and verify the character falls.
12. **R-13.17.6** — The engine **SHALL** transition the character controller to swim locomotion on
    water volume entry, support surface swimming and 3D underwater diving with an oxygen meter that
    causes drowning damage on depletion, activate underwater visual effects when the camera
    submerges, and integrate with survival stamina and aquatic mounts.
    - **Rationale:** Water locomotion with oxygen management creates distinct environmental
      traversal challenges that integrate with survival and mount systems.
    - **Verification:** Enter a water volume and verify the character switches to swim locomotion.
      Dive underwater and confirm the oxygen meter drains. Deplete oxygen and verify drowning damage
      is applied. Confirm underwater visual effects (blue tint, depth fog) activate when the camera
      submerges. Mount an aquatic mount and verify faster underwater speed.
13. **R-13.17.7** — The engine **SHALL** support grapple-pull and grapple-swing traversal via
    projectile attachment to valid surfaces with pendulum physics, and zipline traversal along cable
    entities with gravity-driven speed and optional braking, using rope/cable physics for visual and
    physical simulation.
    - **Rationale:** Grapple and zipline mechanics extend vertical and horizontal mobility using the
      existing rope physics system, enabling dynamic traversal level design.
    - **Verification:** Fire a grappling hook at a valid surface and verify grapple-pull moves the
      character toward the anchor. Fire at a swing point and confirm pendulum physics with realistic
      arc. Attach to a zipline and verify gravity-driven descent with speed matching cable angle.
      Engage braking and confirm the character decelerates along the cable.

## Non-Functional Requirements

| ID          | Derived From |
|-------------|--------------|
| NFR-13.17.1 |              |
| NFR-13.17.2 |              |

1. **NFR-13.17.1** — Shape-cast traversal opportunity detection **SHALL** complete within 0.5ms per
   frame. Context-sensitive action classification (vault, mantle, climb, slide, wall-run, balance)
   **SHALL** resolve within 1 frame of the player reaching the detection zone. Animation transition
   from locomotion to traversal action **SHALL** begin within 2 frames of detection.
   - **Rationale:** Traversal fluidity depends on near-instant detection and animation response. Any
     perceptible delay between approaching geometry and executing the action breaks flow.
   - **Verification:** Measure time from entering a traversal detection zone to action
     classification. Verify it resolves within 1 frame. Measure animation transition start time and
     verify it begins within 2 frames.
2. **NFR-13.17.2** — The interaction detection system **SHALL** support up to 200 interactable
   entities within the player's proximity query range without exceeding 1ms per frame for raycast
   and proximity evaluation.
   - **Rationale:** Dense environments (towns, dungeons) contain many interactable objects.
     Detection must remain performant to avoid frame rate impact in content-heavy areas.
   - **Verification:** Place 200 interactable entities within proximity range. Measure per-frame
     interaction detection time. Verify it stays under 1ms.
