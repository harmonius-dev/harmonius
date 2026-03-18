# User Stories -- Traversal and Interaction (13.17)

## World Object Interaction (F-13.17.1)

| ID           | Persona              | Features  | Requirements |
|--------------|----------------------|-----------|--------------|
| US-13.17.1.1 | player (P-23)        | F-13.17.1 | R-13.17.1    |
| US-13.17.1.2 | player (P-23)        | F-13.17.1 | R-13.17.1    |
| US-13.17.1.3 | player (P-23)        | F-13.17.1 | R-13.17.1    |
| US-13.17.1.4 | player (P-23)        | F-13.17.1 | R-13.17.1    |
| US-13.17.1.5 | player (P-23)        | F-13.17.1 | R-13.17.1    |
| US-13.17.1.6 | designer (P-5)       | F-13.17.1 | R-13.17.1    |
| US-13.17.1.7 | level designer (P-6) | F-13.17.1 | R-13.17.1    |
| US-13.17.1.8 | tester (P-27)        | F-13.17.1 | R-13.17.1    |

1. **US-13.17.1.1** — **As a** player (P-23), **I want** a UI prompt to appear when I face an
   interactable object within range, **so that** I know interaction is available and which input to
   press.
2. **US-13.17.1.2** — **As a** player (P-23), **I want** instant interactions like flipping a switch
   to execute immediately on press, **so that** simple interactions feel responsive.
3. **US-13.17.1.3** — **As a** player (P-23), **I want** channeled interactions like picking a lock
   to require holding the input, **so that** complex interactions have a time cost.
4. **US-13.17.1.4** — **As a** player (P-23), **I want** multiple interactions on one object to
   appear via a radial menu, **so that** I can choose the appropriate action.
5. **US-13.17.1.5** — **As a** player (P-23), **I want** locked doors to show "locked" and depleted
   nodes to show "empty," **so that** game state is reflected in interaction prompts.
6. **US-13.17.1.6** — **As a** designer (P-5), **I want** to configure interaction type, required
   items, animation, duration, and logic graph per interactable entity, **so that** all interactions
   are data-driven.
7. **US-13.17.1.7** — **As a** level designer (P-6), **I want** to place interactable entities in
   the world with configurable properties, **so that** I can author puzzles and environmental
   storytelling.
8. **US-13.17.1.8** — **As a** tester (P-27), **I want** to verify that a channeled interaction
   cancels if the player moves during the channel, **so that** interrupt behavior works correctly.

## Door and Lock System (F-13.17.2)

| ID           | Persona              | Features  | Requirements |
|--------------|----------------------|-----------|--------------|
| US-13.17.2.1 | player (P-23)        | F-13.17.2 | R-13.17.2    |
| US-13.17.2.2 | player (P-23)        | F-13.17.2 | R-13.17.2    |
| US-13.17.2.3 | player (P-23)        | F-13.17.2 | R-13.17.2    |
| US-13.17.2.4 | designer (P-5)       | F-13.17.2 | R-13.17.2    |
| US-13.17.2.5 | designer (P-5)       | F-13.17.2 | R-13.17.2    |
| US-13.17.2.6 | level designer (P-6) | F-13.17.2 | R-13.17.2    |
| US-13.17.2.7 | tester (P-27)        | F-13.17.2 | R-13.17.2    |

1. **US-13.17.2.1** — **As a** player (P-23), **I want** to open, close, and lock doors through
   interactions, **so that** I can control access to areas.
2. **US-13.17.2.2** — **As a** player (P-23), **I want** locked doors to require a key item and
   optionally support a lockpick minigame, **so that** locked doors present a meaningful obstacle.
3. **US-13.17.2.3** — **As a** player (P-23), **I want** breakable doors to have HP for forced
   entry, **so that** I can brute-force access when I lack a key.
4. **US-13.17.2.4** — **As a** designer (P-5), **I want** to configure door swing type (one-way,
   double, sliding, portcullis), auto-close timer, and access permissions per door, **so that**
   doors are data-driven.
5. **US-13.17.2.5** — **As a** designer (P-5), **I want** door state changes to fire events for
   alarms, quests, and AI awareness, **so that** doors integrate with other systems.
6. **US-13.17.2.6** — **As a** level designer (P-6), **I want** to place doors with configurable
   lock state and key requirements, **so that** I can gate access in level design.
7. **US-13.17.2.7** — **As a** tester (P-27), **I want** to verify that an NPC with a key can open a
   locked door and an NPC without a key pathfinds around it, **so that** AI door interaction is
   correct.

## Physics Pickup and Throw (F-13.17.3)

| ID           | Persona              | Features  | Requirements |
|--------------|----------------------|-----------|--------------|
| US-13.17.3.1 | player (P-23)        | F-13.17.3 | R-13.17.3    |
| US-13.17.3.2 | player (P-23)        | F-13.17.3 | R-13.17.3    |
| US-13.17.3.3 | player (P-23)        | F-13.17.3 | R-13.17.3    |
| US-13.17.3.4 | designer (P-5)       | F-13.17.3 | R-13.17.3    |
| US-13.17.3.5 | level designer (P-6) | F-13.17.3 | R-13.17.3    |
| US-13.17.3.6 | tester (P-27)        | F-13.17.3 | R-13.17.3    |

1. **US-13.17.3.1** — **As a** player (P-23), **I want** to pick up physics objects and carry them
   at a configurable hold point, **so that** I can move objects through the world.
2. **US-13.17.3.2** — **As a** player (P-23), **I want** to throw carried objects with a force based
   on aim direction, **so that** I can use objects as projectiles or puzzle solutions.
3. **US-13.17.3.3** — **As a** player (P-23), **I want** heavy objects to slow my movement speed,
   **so that** carrying weight has a gameplay cost.
4. **US-13.17.3.4** — **As a** designer (P-5), **I want** to configure hold distance, spring
   stiffness, and throw strength per object, **so that** pickup physics feels tunable.
5. **US-13.17.3.5** — **As a** level designer (P-6), **I want** to place grabbable objects on
   pressure plates for puzzles, **so that** physics pickup enables environmental puzzle design.
6. **US-13.17.3.6** — **As a** tester (P-27), **I want** to verify that a thrown object deals damage
   proportional to throw strength, **so that** thrown object damage is correct.

## Traversal Detection (F-13.17.4a)

| ID         | Persona              | Features  | Requirements |
|------------|----------------------|-----------|--------------|
| US-13.17.4 | player (P-23)        | F-13.17.4 | R-13.17.4    |
| US-13.17.4 | designer (P-5)       | F-13.17.4 | R-13.17.4    |
| US-13.17.4 | designer (P-5)       | F-13.17.4 | R-13.17.4    |
| US-13.17.4 | level designer (P-6) | F-13.17.4 | R-13.17.4    |
| US-13.17.4 | tester (P-27)        | F-13.17.4 | R-13.17.4    |

1. **US-13.17.4** — **As a** player (P-23), **I want** nearby geometry automatically classified as
   vault, mantle, wall-run, balance beam, or slide-under opportunities, **so that** traversal
   happens naturally.
2. **US-13.17.4** — **As a** designer (P-5), **I want** to configure height thresholds, cast
   distances, and classification rules for traversal detection, **so that** I can tune which
   geometry triggers which action.
3. **US-13.17.4** — **As a** designer (P-5), **I want** traversal geometry to be auto-detected from
   collision shapes or explicitly tagged in the editor, **so that** both procedural and authored
   levels work.
4. **US-13.17.4** — **As a** level designer (P-6), **I want** to manually tag traversal surfaces in
   the editor for precision, **so that** I can ensure specific obstacles are traversable.
5. **US-13.17.4** — **As a** tester (P-27), **I want** to verify that a 0.5 m obstacle is classified
   as vault and a 1.5 m obstacle as mantle, **so that** height-based classification is correct.

## Vault and Mantle (F-13.17.4b)

| ID         | Persona              | Features  | Requirements |
|------------|----------------------|-----------|--------------|
| US-13.17.4 | player (P-23)        | F-13.17.4 | R-13.17.4    |
| US-13.17.4 | player (P-23)        | F-13.17.4 | R-13.17.4    |
| US-13.17.4 | player (P-23)        | F-13.17.4 | R-13.17.4    |
| US-13.17.4 | designer (P-5)       | F-13.17.4 | R-13.17.4    |
| US-13.17.4 | level designer (P-6) | F-13.17.4 | R-13.17.4    |
| US-13.17.4 | tester (P-27)        | F-13.17.4 | R-13.17.4    |

1. **US-13.17.4** — **As a** player (P-23), **I want** to vault over low obstacles with a one-hand
   hop without stopping, **so that** movement stays fluid at low obstacles.
2. **US-13.17.4** — **As a** player (P-23), **I want** to mantle onto medium obstacles by grabbing
   the edge and pulling up, **so that** I can reach elevated surfaces.
3. **US-13.17.4** — **As a** player (P-23), **I want** vault and mantle to deduct stamina,
   **so that** traversal has an energy cost.
4. **US-13.17.4** — **As a** designer (P-5), **I want** to configure vault and mantle height ranges,
   stamina cost, and minimum approach speed, **so that** I can tune traversal difficulty.
5. **US-13.17.4** — **As a** level designer (P-6), **I want** IK to place character hands on the
   obstacle surface during mantle, **so that** the animation looks physically grounded.
6. **US-13.17.4** — **As a** tester (P-27), **I want** to verify that vault fails when stamina is
   insufficient, **so that** the stamina cost is enforced.

## Wall Run (F-13.17.4c)

| ID         | Persona        | Features  | Requirements |
|------------|----------------|-----------|--------------|
| US-13.17.4 | player (P-23)  | F-13.17.4 | R-13.17.4    |
| US-13.17.4 | player (P-23)  | F-13.17.4 | R-13.17.4    |
| US-13.17.4 | player (P-23)  | F-13.17.4 | R-13.17.4    |
| US-13.17.4 | designer (P-5) | F-13.17.4 | R-13.17.4    |
| US-13.17.4 | tester (P-27)  | F-13.17.4 | R-13.17.4    |

1. **US-13.17.4** — **As a** player (P-23), **I want** to run along vertical walls when sprinting
   near them, **so that** walls become traversal opportunities.
2. **US-13.17.4** — **As a** player (P-23), **I want** a gravity timer to limit wall-run duration
   with gradual descent, **so that** wall-running has natural limits.
3. **US-13.17.4** — **As a** player (P-23), **I want** to jump off the wall at a configurable angle
   during a wall run, **so that** I can chain wall-runs or reach platforms.
4. **US-13.17.4** — **As a** designer (P-5), **I want** to configure wall-run speed, max duration,
   gravity curve, and minimum entry speed, **so that** I can tune the wall-run feel per game.
5. **US-13.17.4** — **As a** tester (P-27), **I want** to verify that wall-run terminates after the
   max duration, **so that** the gravity timer is enforced.

## Crouch Slide (F-13.17.4d)

| ID         | Persona        | Features  | Requirements |
|------------|----------------|-----------|--------------|
| US-13.17.4 | player (P-23)  | F-13.17.4 | R-13.17.4    |
| US-13.17.4 | player (P-23)  | F-13.17.4 | R-13.17.4    |
| US-13.17.4 | player (P-23)  | F-13.17.4 | R-13.17.4    |
| US-13.17.4 | designer (P-5) | F-13.17.4 | R-13.17.4    |
| US-13.17.4 | tester (P-27)  | F-13.17.4 | R-13.17.4    |

1. **US-13.17.4** — **As a** player (P-23), **I want** to trigger a crouch slide while sprinting by
   pressing crouch, **so that** I can maintain momentum through tight spaces.
2. **US-13.17.4** — **As a** player (P-23), **I want** slide distance to scale with entry speed and
   slope, **so that** faster approaches and downhill slides go farther.
3. **US-13.17.4** — **As a** player (P-23), **I want** to pass under low obstacles during a slide,
   **so that** slides open paths that standing cannot reach.
4. **US-13.17.4** — **As a** designer (P-5), **I want** to configure slide deceleration, stamina
   cost, and cooldown, **so that** I can balance slide usage.
5. **US-13.17.4** — **As a** tester (P-27), **I want** to verify that a slide on a downhill slope
   covers more distance than on flat ground, **so that** slope physics is correct.

## Balance Beam (F-13.17.4e)

| ID         | Persona              | Features  | Requirements |
|------------|----------------------|-----------|--------------|
| US-13.17.4 | player (P-23)        | F-13.17.4 | R-13.17.4    |
| US-13.17.4 | player (P-23)        | F-13.17.4 | R-13.17.4    |
| US-13.17.4 | designer (P-5)       | F-13.17.4 | R-13.17.4    |
| US-13.17.4 | level designer (P-6) | F-13.17.4 | R-13.17.4    |
| US-13.17.4 | tester (P-27)        | F-13.17.4 | R-13.17.4    |

1. **US-13.17.4** — **As a** player (P-23), **I want** narrow surfaces to switch me to slow balance
   mode with procedural sway, **so that** traversal includes tension.
2. **US-13.17.4** — **As a** player (P-23), **I want** to fall off the beam if I move too fast, take
   damage, or exceed sway tolerance, **so that** balance requires careful play.
3. **US-13.17.4** — **As a** designer (P-5), **I want** to configure beam width threshold, walk
   speed, sway amount, and fall conditions, **so that** I can tune balance difficulty.
4. **US-13.17.4** — **As a** level designer (P-6), **I want** to tag narrow surfaces as balance
   beams in the editor, **so that** I can control where balance traversal occurs.
5. **US-13.17.4** — **As a** tester (P-27), **I want** to verify that the camera adds subtle wobble
   during balance mode, **so that** visual feedback reinforces instability.

## Free-Climb (F-13.17.5a)

| ID         | Persona              | Features  | Requirements |
|------------|----------------------|-----------|--------------|
| US-13.17.5 | player (P-23)        | F-13.17.5 | R-13.17.5    |
| US-13.17.5 | player (P-23)        | F-13.17.5 | R-13.17.5    |
| US-13.17.5 | player (P-23)        | F-13.17.5 | R-13.17.5    |
| US-13.17.5 | designer (P-5)       | F-13.17.5 | R-13.17.5    |
| US-13.17.5 | level designer (P-6) | F-13.17.5 | R-13.17.5    |
| US-13.17.5 | tester (P-27)        | F-13.17.5 | R-13.17.5    |

1. **US-13.17.5** — **As a** player (P-23), **I want** to climb tagged surfaces with IK-driven hand
   and foot placement on grip points, **so that** vertical traversal feels physical.
2. **US-13.17.5** — **As a** player (P-23), **I want** stamina to drain continuously while climbing,
   with depletion causing me to fall, **so that** climbing has risk.
3. **US-13.17.5** — **As a** player (P-23), **I want** rest points on climb surfaces to pause
   stamina drain, **so that** long climbs have safe checkpoints.
4. **US-13.17.5** — **As a** designer (P-5), **I want** to configure climb speed, stamina drain
   rate, grip spacing, and reach distance, **so that** I can tune climbing difficulty per surface.
5. **US-13.17.5** — **As a** level designer (P-6), **I want** to place grip points as auto-generated
   grids or hand-placed markers, **so that** I have control over climb routes.
6. **US-13.17.5** — **As a** tester (P-27), **I want** to verify that stamina depletion causes the
   character to fall, **so that** the climbing stamina penalty functions.

## Ladder System (F-13.17.5b)

| ID         | Persona              | Features  | Requirements |
|------------|----------------------|-----------|--------------|
| US-13.17.5 | player (P-23)        | F-13.17.5 | R-13.17.5    |
| US-13.17.5 | player (P-23)        | F-13.17.5 | R-13.17.5    |
| US-13.17.5 | designer (P-5)       | F-13.17.5 | R-13.17.5    |
| US-13.17.5 | level designer (P-6) | F-13.17.5 | R-13.17.5    |
| US-13.17.5 | tester (P-27)        | F-13.17.5 | R-13.17.5    |

1. **US-13.17.5** — **As a** player (P-23), **I want** ladders to provide simple vertical movement
   with fixed up/down input, **so that** basic elevation changes are reliable.
2. **US-13.17.5** — **As a** player (P-23), **I want** to dismount mid-ladder by pressing a
   directional input away from it, **so that** I can drop to the ground if needed.
3. **US-13.17.5** — **As a** designer (P-5), **I want** to configure climb speed per ladder,
   **so that** different ladder types can have different speeds.
4. **US-13.17.5** — **As a** level designer (P-6), **I want** to place ladder entities for vertical
   pathfinding, **so that** NPCs and players share vertical routes.
5. **US-13.17.5** — **As a** tester (P-27), **I want** to verify that ladders do not consume
   stamina, **so that** ladder traversal is distinguished from free-climbing.

## Ledge Grab and Shimmy (F-13.17.5c)

| ID         | Persona              | Features  | Requirements |
|------------|----------------------|-----------|--------------|
| US-13.17.5 | player (P-23)        | F-13.17.5 | R-13.17.5    |
| US-13.17.5 | player (P-23)        | F-13.17.5 | R-13.17.5    |
| US-13.17.5 | player (P-23)        | F-13.17.5 | R-13.17.5    |
| US-13.17.5 | designer (P-5)       | F-13.17.5 | R-13.17.5    |
| US-13.17.5 | level designer (P-6) | F-13.17.5 | R-13.17.5    |
| US-13.17.5 | tester (P-27)        | F-13.17.5 | R-13.17.5    |

1. **US-13.17.5** — **As a** player (P-23), **I want** to grab ledges when airborne near a
   horizontal edge, **so that** I can catch myself during falls.
2. **US-13.17.5** — **As a** player (P-23), **I want** to shimmy laterally along a ledge and pull up
   to stand on top, **so that** ledges become viable traversal paths.
3. **US-13.17.5** — **As a** player (P-23), **I want** ledge grab to drain stamina while hanging,
   **so that** I cannot hang indefinitely.
4. **US-13.17.5** — **As a** designer (P-5), **I want** to configure ledge auto-detection or
   explicit tagging, **so that** both authored and procedural levels support ledge traversal.
5. **US-13.17.5** — **As a** level designer (P-6), **I want** to tag ledges that should support
   shimmy in the editor, **so that** I control precise traversal routes.
6. **US-13.17.5** — **As a** tester (P-27), **I want** to verify that IK positions hands on the
   ledge surface during the grab, **so that** the visual contact is physically accurate.

## Swimming and Diving (F-13.17.6)

| ID           | Persona              | Features  | Requirements |
|--------------|----------------------|-----------|--------------|
| US-13.17.6.1 | player (P-23)        | F-13.17.6 | R-13.17.6    |
| US-13.17.6.2 | player (P-23)        | F-13.17.6 | R-13.17.6    |
| US-13.17.6.3 | player (P-23)        | F-13.17.6 | R-13.17.6    |
| US-13.17.6.4 | player (P-23)        | F-13.17.6 | R-13.17.6    |
| US-13.17.6.5 | designer (P-5)       | F-13.17.6 | R-13.17.6    |
| US-13.17.6.6 | level designer (P-6) | F-13.17.6 | R-13.17.6    |
| US-13.17.6.7 | tester (P-27)        | F-13.17.6 | R-13.17.6    |

1. **US-13.17.6.1** — **As a** player (P-23), **I want** water volumes to transition me from ground
   to swim locomotion, **so that** I can navigate aquatic environments.
2. **US-13.17.6.2** — **As a** player (P-23), **I want** diving to enable 3D underwater movement
   with configurable speed and buoyancy, **so that** underwater exploration is possible.
3. **US-13.17.6.3** — **As a** player (P-23), **I want** an oxygen meter that drains while submerged
   with drowning damage at zero, **so that** diving has time pressure.
4. **US-13.17.6.4** — **As a** player (P-23), **I want** underwater visual effects like blue tint
   and caustics when my camera submerges, **so that** the underwater environment feels distinct.
5. **US-13.17.6.5** — **As a** designer (P-5), **I want** to configure swim speed, buoyancy, oxygen
   drain rate, and underwater effects, **so that** I can tune aquatic gameplay.
6. **US-13.17.6.6** — **As a** level designer (P-6), **I want** to define water volumes that trigger
   swim locomotion, **so that** I can design aquatic areas precisely.
7. **US-13.17.6.7** — **As a** tester (P-27), **I want** to verify that entering and exiting water
   plays splash VFX and audio, **so that** surface transitions have feedback.

## Grappling Hook and Zipline (F-13.17.7)

| ID           | Persona              | Features  | Requirements |
|--------------|----------------------|-----------|--------------|
| US-13.17.7.1 | player (P-23)        | F-13.17.7 | R-13.17.7    |
| US-13.17.7.2 | player (P-23)        | F-13.17.7 | R-13.17.7    |
| US-13.17.7.3 | player (P-23)        | F-13.17.7 | R-13.17.7    |
| US-13.17.7.4 | designer (P-5)       | F-13.17.7 | R-13.17.7    |
| US-13.17.7.5 | level designer (P-6) | F-13.17.7 | R-13.17.7    |
| US-13.17.7.6 | tester (P-27)        | F-13.17.7 | R-13.17.7    |

1. **US-13.17.7.1** — **As a** player (P-23), **I want** to fire a grappling hook that attaches to
   surfaces and pull myself toward the anchor, **so that** I can traverse gaps rapidly.
2. **US-13.17.7.2** — **As a** player (P-23), **I want** grapple-swing to use pendulum physics from
   the attachment point, **so that** swinging feels physically grounded.
3. **US-13.17.7.3** — **As a** player (P-23), **I want** to attach to zipline cables and slide along
   them with gravity-driven speed, **so that** I can traverse between high points quickly.
4. **US-13.17.7.4** — **As a** designer (P-5), **I want** to configure grapple range, pull speed,
   and swing parameters per equipment item, **so that** different grapple tools feel distinct.
5. **US-13.17.7.5** — **As a** level designer (P-6), **I want** to define zipline cable entities and
   grapple anchor points in the editor, **so that** I can author traversal routes.
6. **US-13.17.7.6** — **As a** tester (P-27), **I want** to verify that the grappling hook only
   attaches to surfaces within the configured range, **so that** range limits are enforced.
