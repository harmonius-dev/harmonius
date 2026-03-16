# User Stories -- 9.3 Procedural Animation

## F-9.3.1

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.3.1.1 | character animator (P-11) | I want two-bone IK with pole vector targets to position hands on weapon grips, ledges, and mount handles | characters interact physically with objects without authoring per-object hand-placement clips | F-9.3.1 | R-9.3.1 |
| US-9.3.1.2 | engine developer (P-26) | I want two-bone IK running as a GPU post-process pass over skinned skeletons | 500+ simultaneous IK chains (desktop) evaluate without CPU overhead | F-9.3.1 | R-9.3.1 |
| US-9.3.1.3 | engine tester (P-27) | I want verify that mobile supports 20-40 active IK chains per frame, Switch supports 80, and desktop supports 500+ | IK budget is enforced per platform | F-9.3.1 | R-9.3.1 |

## F-9.3.2

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.3.2.1 | character animator (P-11) | I want CCD IK for medium-length chains (3-8 bones) with configurable iteration limits and angular constraints per joint | tails, spines, and tentacles track targets smoothly | F-9.3.2 | R-9.3.2 |
| US-9.3.2.2 | engine tester (P-27) | I want verify that mobile uses 2-4 CCD iterations, Switch uses 6, and desktop uses 8-12 | CCD accuracy trades correctly against per-platform compute budgets | F-9.3.2 | R-9.3.2 |

## F-9.3.3

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.3.3.1 | character animator (P-11) | I want FABRIK IK for long chains with multiple end- effector targets and joint constraints | spider legs and branching skeletal structures solve correctly with position-space efficiency | F-9.3.3 | R-9.3.3 |
| US-9.3.3.2 | engine tester (P-27) | I want verify that mobile uses 2-3 FABRIK iterations with fewer targets while desktop uses 6-8 | multi-end-effector problems are simplified appropriately on mobile | F-9.3.3 | R-9.3.3 |

## F-9.3.4

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.3.4.1 | character animator (P-11) | I want per-bone blend weights between animation and ragdoll physics | partial ragdoll (upper body ragdolls on hit while legs animate) produces smooth transitions | F-9.3.4 | R-9.3.4 |
| US-9.3.4.2 | game developer (P-15) | I want ragdoll-to-animated recovery transitions that blend from physics pose back to animation over a configurable duration | characters regain composure naturally after knockback rather than snapping to idle | F-9.3.4 | R-9.3.4 |
| US-9.3.4.3 | engine tester (P-27) | I want verify that mobile supports 4-8 ragdoll capsules, Switch supports 12, and desktop supports 16-20 | ragdoll complexity scales per platform | F-9.3.4 | R-9.3.4 |

## F-9.3.5

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.3.5.1 | game designer (P-5) | I want look-at constraints that procedurally rotate head and spine bones to track a world-space target with angle limits | NPCs visually respond to players walking past without authored look animations | F-9.3.5 | R-9.3.5 |
| US-9.3.5.2 | character animator (P-11) | I want aim constraints that orient weapon-holding arms toward a target with smooth blending and joint limits | aiming poses look natural on top of any locomotion state | F-9.3.5 | R-9.3.5 |

## F-9.3.6

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.3.6.1 | character animator (P-11) | I want motion matching that searches a pose database for the best continuation given the current pose and desired trajectory | character movement feels responsive and natural without hand-authored state machine locomotion | F-9.3.6 | R-9.3.6 |
| US-9.3.6.2 | engine developer (P-26) | I want benchmark pose database search at varying database sizes (small on mobile, full mocap library on desktop) | I can set per- platform database size limits that fit within CPU budgets | F-9.3.6 | R-9.3.6 |

## F-9.3.7

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.3.7.1 | player (P-23) | I want characters' feet to plant correctly on stairs, slopes, and uneven ground with IK-adjusted pelvis height | locomotion looks grounded and believable on any surface | F-9.3.7 | R-9.3.7 |
| US-9.3.7.2 | engine tester (P-27) | I want verify that mobile uses 2 raycasts (feet only) and desktop uses 4 (feet + pelvis + slope probe), and that foot placement is disabled for distant characters on mobile | raycast cost scales per platform | F-9.3.7 | R-9.3.7 |

## F-9.3.8

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.3.8.1 | character animator (P-11) | I want procedural locomotion that adapts to arbitrary skeleton topologies with configurable gait patterns (walk, trot, canter, gallop) | bipeds, quadrupeds, hexapods, and serpentine creatures locomote correctly from a single system | F-9.3.8 | R-9.3.8 |
| US-9.3.8.2 | game developer (P-15) | I want locomotion profiles defined as ECS components (LocomotionProfile, GaitState, FootGroup) specifying leg groups, phase offsets, and stride curves | creature locomotion is data-driven and configurable per entity | F-9.3.8 | R-9.3.8 |
| US-9.3.8.3 | engine tester (P-27) | I want verify that hexapod+ topologies use simplified gait patterns on mobile with reduced IK iterations | complex creatures remain animatable on lower-tier hardware | F-9.3.8 | R-9.3.8 |

## F-9.3.9

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.3.9.1 | player (P-23) | I want characters to stumble when hit by impacts and recover balance using physical forces | combat feels physically grounded with emergent reactions rather than canned hit animations | F-9.3.9 | R-9.3.9 |
| US-9.3.9.2 | engine developer (P-26) | I want measure per-limb force computation cost for physics-based locomotion | I can confirm it is limited to hero/player characters on mobile and scales to multiple characters on desktop | F-9.3.9 | R-9.3.9 |

## F-9.3.10

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.3.10.1 | player (P-23) | I want dismemberment to sever a bone chain from a skeleton, spawning the detached sub-mesh as a physics-simulated entity while the remaining skeleton adapts locomotion | combat dismemberment feels visceral and consequential | F-9.3.10 | R-9.3.10 |
| US-9.3.10.2 | game developer (P-15) | I want runtime attachment and removal of skeletal sub-hierarchies (weapons, armor plates, holsters) via ECS command buffers | equipment changes are immediate and data-driven | F-9.3.10 | R-9.3.10 |
| US-9.3.10.3 | engine tester (P-27) | I want verify that mobile caps active dismembered parts at 2-4 with simplified ragdoll and desktop supports 8-16 | physics entity count from dismemberment stays within platform budgets | F-9.3.10 | R-9.3.10 |

## F-9.3.11

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.3.11.1 | character animator (P-11) | I want debug overlays showing foot placement targets, IK chain states, ground normals, gait phase diagrams, and balance center-of-mass | I can diagnose foot sliding and IK solve failures in the animation preview | F-9.3.11 | R-9.3.11 |
| US-9.3.11.2 | engine developer (P-26) | I want a locomotion profiler panel showing IK iterations, foot plant errors, and physics force magnitudes per frame | I can identify animation performance regressions and solve quality issues | F-9.3.11 | R-9.3.11 |
| US-9.3.11.3 | engine tester (P-27) | I want confirm that locomotion debug visualization is compiled out of shipping builds on all platforms | debug rendering has zero cost in production | F-9.3.11 | R-9.3.11 |
