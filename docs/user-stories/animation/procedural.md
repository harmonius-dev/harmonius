# User Stories -- 9.3 Procedural Animation

## F-9.3.1

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.3.1.1 | character animator (P-11) | F-9.3.1  | R-9.3.1      |
| US-9.3.1.2 | engine developer (P-26)   | F-9.3.1  | R-9.3.1      |
| US-9.3.1.3 | engine tester (P-27)      | F-9.3.1  | R-9.3.1      |

1. **US-9.3.1.1** — I want two-bone IK with pole vector targets to position hands on weapon grips,
   ledges, and mount handles
   - **Acceptance:** characters interact physically with objects without authoring per-object
     hand-placement clips
2. **US-9.3.1.2** — I want two-bone IK running as a GPU post-process pass over skinned skeletons
   - **Acceptance:** 500+ simultaneous IK chains (desktop) evaluate without CPU overhead
3. **US-9.3.1.3** — I want verify that mobile supports 20-40 active IK chains per frame, Switch
   supports 80, and desktop supports 500+
   - **Acceptance:** IK budget is enforced per platform

## F-9.3.2

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.3.2.1 | character animator (P-11) | F-9.3.2  | R-9.3.2      |
| US-9.3.2.2 | engine tester (P-27)      | F-9.3.2  | R-9.3.2      |

1. **US-9.3.2.1** — I want CCD IK for medium-length chains (3-8 bones) with configurable iteration
   limits and angular constraints per joint
   - **Acceptance:** tails, spines, and tentacles track targets smoothly
2. **US-9.3.2.2** — I want verify that mobile uses 2-4 CCD iterations, Switch uses 6, and desktop
   uses 8-12
   - **Acceptance:** CCD accuracy trades correctly against per-platform compute budgets

## F-9.3.3

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.3.3.1 | character animator (P-11) | F-9.3.3  | R-9.3.3      |
| US-9.3.3.2 | engine tester (P-27)      | F-9.3.3  | R-9.3.3      |

1. **US-9.3.3.1** — I want FABRIK IK for long chains with multiple end- effector targets and joint
   constraints
   - **Acceptance:** spider legs and branching skeletal structures solve correctly with
     position-space efficiency
2. **US-9.3.3.2** — I want verify that mobile uses 2-3 FABRIK iterations with fewer targets while
   desktop uses 6-8
   - **Acceptance:** multi-end-effector problems are simplified appropriately on mobile

## F-9.3.4

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.3.4.1 | character animator (P-11) | F-9.3.4  | R-9.3.4      |
| US-9.3.4.2 | game developer (P-15)     | F-9.3.4  | R-9.3.4      |
| US-9.3.4.3 | engine tester (P-27)      | F-9.3.4  | R-9.3.4      |

1. **US-9.3.4.1** — I want per-bone blend weights between animation and ragdoll physics
   - **Acceptance:** partial ragdoll (upper body ragdolls on hit while legs animate) produces smooth
     transitions
2. **US-9.3.4.2** — I want ragdoll-to-animated recovery transitions that blend from physics pose
   back to animation over a configurable duration
   - **Acceptance:** characters regain composure naturally after knockback rather than snapping to
     idle
3. **US-9.3.4.3** — I want verify that mobile supports 4-8 ragdoll capsules, Switch supports 12, and
   desktop supports 16-20
   - **Acceptance:** ragdoll complexity scales per platform

## F-9.3.5

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.3.5.1 | game designer (P-5)       | F-9.3.5  | R-9.3.5      |
| US-9.3.5.2 | character animator (P-11) | F-9.3.5  | R-9.3.5      |

1. **US-9.3.5.1** — I want look-at constraints that procedurally rotate head and spine bones to
   track a world-space target with angle limits
   - **Acceptance:** NPCs visually respond to players walking past without authored look animations
2. **US-9.3.5.2** — I want aim constraints that orient weapon-holding arms toward a target with
   smooth blending and joint limits
   - **Acceptance:** aiming poses look natural on top of any locomotion state

## F-9.3.6

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.3.6.1 | character animator (P-11) | F-9.3.6  | R-9.3.6      |
| US-9.3.6.2 | engine developer (P-26)   | F-9.3.6  | R-9.3.6      |

1. **US-9.3.6.1** — I want motion matching that searches a pose database for the best continuation
   given the current pose and desired trajectory
   - **Acceptance:** character movement feels responsive and natural without hand-authored state
     machine locomotion
2. **US-9.3.6.2** — I want benchmark pose database search at varying database sizes (small on
   mobile, full mocap library on desktop)
   - **Acceptance:** I can set per- platform database size limits that fit within CPU budgets

## F-9.3.7

| ID         | Persona              | Features | Requirements |
|------------|----------------------|----------|--------------|
| US-9.3.7.1 | player (P-23)        | F-9.3.7  | R-9.3.7      |
| US-9.3.7.2 | engine tester (P-27) | F-9.3.7  | R-9.3.7      |

1. **US-9.3.7.1** — I want characters' feet to plant correctly on stairs, slopes, and uneven ground
   with IK-adjusted pelvis height
   - **Acceptance:** locomotion looks grounded and believable on any surface
2. **US-9.3.7.2** — I want verify that mobile uses 2 raycasts (feet only) and desktop uses 4 (feet +
   pelvis + slope probe), and that foot placement is disabled for distant characters on mobile
   - **Acceptance:** raycast cost scales per platform

## F-9.3.8

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.3.8.1 | character animator (P-11) | F-9.3.8  | R-9.3.8      |
| US-9.3.8.2 | game developer (P-15)     | F-9.3.8  | R-9.3.8      |
| US-9.3.8.3 | engine tester (P-27)      | F-9.3.8  | R-9.3.8      |

1. **US-9.3.8.1** — I want procedural locomotion that adapts to arbitrary skeleton topologies with
   configurable gait patterns (walk, trot, canter, gallop)
   - **Acceptance:** bipeds, quadrupeds, hexapods, and serpentine creatures locomote correctly from
     a single system
2. **US-9.3.8.2** — I want locomotion profiles defined as ECS components (LocomotionProfile,
   GaitState, FootGroup) specifying leg groups, phase offsets, and stride curves
   - **Acceptance:** creature locomotion is data-driven and configurable per entity
3. **US-9.3.8.3** — I want verify that hexapod+ topologies use simplified gait patterns on mobile
   with reduced IK iterations
   - **Acceptance:** complex creatures remain animatable on lower-tier hardware

## F-9.3.9

| ID         | Persona                 | Features | Requirements |
|------------|-------------------------|----------|--------------|
| US-9.3.9.1 | player (P-23)           | F-9.3.9  | R-9.3.9      |
| US-9.3.9.2 | engine developer (P-26) | F-9.3.9  | R-9.3.9      |

1. **US-9.3.9.1** — I want characters to stumble when hit by impacts and recover balance using
   physical forces
   - **Acceptance:** combat feels physically grounded with emergent reactions rather than canned hit
     animations
2. **US-9.3.9.2** — I want measure per-limb force computation cost for physics-based locomotion
   - **Acceptance:** I can confirm it is limited to hero/player characters on mobile and scales to
     multiple characters on desktop

## F-9.3.10

| ID          | Persona               | Features | Requirements |
|-------------|-----------------------|----------|--------------|
| US-9.3.10.1 | player (P-23)         | F-9.3.10 | R-9.3.10     |
| US-9.3.10.2 | game developer (P-15) | F-9.3.10 | R-9.3.10     |
| US-9.3.10.3 | engine tester (P-27)  | F-9.3.10 | R-9.3.10     |

1. **US-9.3.10.1** — I want dismemberment to sever a bone chain from a skeleton, spawning the
   detached sub-mesh as a physics-simulated entity while the remaining skeleton adapts locomotion
   - **Acceptance:** combat dismemberment feels visceral and consequential
2. **US-9.3.10.2** — I want runtime attachment and removal of skeletal sub-hierarchies (weapons,
   armor plates, holsters) via ECS command buffers
   - **Acceptance:** equipment changes are immediate and data-driven
3. **US-9.3.10.3** — I want verify that mobile caps active dismembered parts at 2-4 with simplified
   ragdoll and desktop supports 8-16
   - **Acceptance:** physics entity count from dismemberment stays within platform budgets

## F-9.3.11

| ID          | Persona                   | Features | Requirements |
|-------------|---------------------------|----------|--------------|
| US-9.3.11.1 | character animator (P-11) | F-9.3.11 | R-9.3.11     |
| US-9.3.11.2 | engine developer (P-26)   | F-9.3.11 | R-9.3.11     |
| US-9.3.11.3 | engine tester (P-27)      | F-9.3.11 | R-9.3.11     |

1. **US-9.3.11.1** — I want debug overlays showing foot placement targets, IK chain states, ground
   normals, gait phase diagrams, and balance center-of-mass
   - **Acceptance:** I can diagnose foot sliding and IK solve failures in the animation preview
2. **US-9.3.11.2** — I want a locomotion profiler panel showing IK iterations, foot plant errors,
   and physics force magnitudes per frame
   - **Acceptance:** I can identify animation performance regressions and solve quality issues
3. **US-9.3.11.3** — I want confirm that locomotion debug visualization is compiled out of shipping
   builds on all platforms
   - **Acceptance:** debug rendering has zero cost in production
