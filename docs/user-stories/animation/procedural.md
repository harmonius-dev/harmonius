# User Stories -- 9.3 Procedural Animation

## IK Solvers

| ID          | Persona                    |
|-------------|----------------------------|
| US-9.3.1.1  | engine developer (P-26)    |
| US-9.3.1.2  | character animator (P-11)  |
| US-9.3.1.3  | engine developer (P-26)    |
| US-9.3.2.1  | engine developer (P-26)    |
| US-9.3.2.2  | character animator (P-11)  |
| US-9.3.3.1  | engine developer (P-26)    |
| US-9.3.3.2  | character animator (P-11)  |

1. **US-9.3.1.1** -- **As an** engine developer (P-26), **I want** an analytical two-bone IK solver
   with pole vector support running as a GPU post-process pass, **so that** limb targeting is fast
   and predictable.

2. **US-9.3.1.2** -- **As a** character animator (P-11), **I want** to set pole vector targets to
   control elbow and knee orientation, **so that** IK solutions look natural for the character's
   body type.

3. **US-9.3.2.1** -- **As an** engine developer (P-26), **I want** a CCD solver for medium-length
   chains with configurable iteration limits and angular constraints, **so that** tails, spines, and
   tentacles animate procedurally.

4. **US-9.3.2.2** -- **As a** character animator (P-11), **I want** to set per-joint angular
   constraints on CCD chains, **so that** procedural motion stays within the character's anatomical
   limits.

5. **US-9.3.1.3** -- **As an** engine developer (P-26), **I want** two-bone IK to run as a GPU
   post-process pass over the skinned skeleton, **so that** IK evaluation scales to 500 chains
   without CPU bottlenecks.

6. **US-9.3.3.1** -- **As an** engine developer (P-26), **I want** a FABRIK solver for long chains
   and multi-end-effector problems, **so that** complex IK setups like multi-limbed creatures are
   solvable.

7. **US-9.3.3.2** -- **As a** character animator (P-11), **I want** FABRIK to support joint
   constraints and multiple target priorities, **so that** IK solutions respect anatomical limits on
   long chains.

## Ragdoll and Look-At

| ID          | Persona                    |
|-------------|----------------------------|
| US-9.3.4.1  | character animator (P-11)  |
| US-9.3.4.2  | engine developer (P-26)    |
| US-9.3.4.3  | game developer (P-15)      |
| US-9.3.5.1  | character animator (P-11)  |
| US-9.3.5.2  | engine developer (P-26)    |

1. **US-9.3.4.1** -- **As a** character animator (P-11), **I want** partial ragdoll with per-bone
   blend weights, **so that** I can blend between animation and physics on specific body parts for
   hit reactions and death.

2. **US-9.3.4.2** -- **As an** engine developer (P-26), **I want** ragdoll simulation running on
   async compute with recovery transitions back to animation, **so that** ragdoll does not block the
   main animation pipeline.

3. **US-9.3.5.1** -- **As a** character animator (P-11), **I want** procedural look-at and aim
   constraints that rotate head and spine bones toward a target, **so that** NPCs track objects of
   interest naturally.

4. **US-9.3.4.3** -- **As a** game developer (P-15), **I want** ragdoll recovery to blend smoothly
   back to animation, **so that** characters regain control after knockback without visual
   discontinuity.

5. **US-9.3.5.1** -- **As a** character animator (P-11), **I want** procedural look-at and aim
   constraints that rotate head and spine bones toward a target, **so that** NPCs track objects of
   interest naturally.

6. **US-9.3.5.2** -- **As an** engine developer (P-26), **I want** look-at and aim constraints to
   blend smoothly with underlying animation and respect joint limits, **so that** procedural
   adjustments do not produce unnatural poses.

## Motion Matching

| ID          | Persona                    |
|-------------|----------------------------|
| US-9.3.6.1  | engine developer (P-26)    |
| US-9.3.6.2  | character animator (P-11)  |

1. **US-9.3.6.1** -- **As an** engine developer (P-26), **I want** motion matching to search a pose
   database for the best continuation of the current pose given a desired trajectory, **so that**
   locomotion transitions are fluid without hand-authored state machine edges.

2. **US-9.3.6.2** -- **As a** character animator (P-11), **I want** motion matching to use weighted
   feature distance metrics tunable per character, **so that** I prioritize trajectory vs. pose vs.
   foot contact per character type.

## Foot Placement and Locomotion

| ID           | Persona                    |
|--------------|----------------------------|
| US-9.3.7.1   | character animator (P-11)  |
| US-9.3.7.2   | engine developer (P-26)    |
| US-9.3.8.1   | rigger (P-10)              |
| US-9.3.8.2   | engine developer (P-26)    |
| US-9.3.8.3   | character animator (P-11)  |
| US-9.3.9.1   | engine developer (P-26)    |
| US-9.3.9.2   | game developer (P-15)      |
| US-9.3.10.1  | character animator (P-11)  |
| US-9.3.10.2  | engine developer (P-26)    |
| US-9.3.10.3  | game developer (P-15)      |
| US-9.3.11.1  | engine developer (P-26)    |
| US-9.3.11.2  | character animator (P-11)  |
| US-9.3.11.3  | engine developer (P-26)    |

1. **US-9.3.7.1** -- **As a** character animator (P-11), **I want** foot placement IK that plants
   feet on uneven terrain, stairs, and slopes, **so that** characters look grounded regardless of
   surface geometry.

2. **US-9.3.7.2** -- **As an** engine developer (P-26), **I want** pelvis height adjustment and
   procedural stride adaptation, **so that** leg extension and step timing respond to movement speed
   and terrain gradient.

3. **US-9.3.8.1** -- **As a** rigger (P-10), **I want** to define locomotion profiles specifying leg
   groups, phase offsets, and gait patterns for arbitrary skeleton topologies, **so that** bipeds,
   quadrupeds, hexapods, and custom creatures all have procedural locomotion.

4. **US-9.3.8.2** -- **As an** engine developer (P-26), **I want** multi-skeleton procedural
   locomotion driven by ECS components, **so that** any skeleton topology animates with data-driven
   gait configuration.

5. **US-9.3.8.3** -- **As a** character animator (P-11), **I want** procedural locomotion to adapt
   stride length and body sway based on movement speed and terrain slope, **so that** creatures move
   naturally across varied environments.

6. **US-9.3.9.1** -- **As an** engine developer (P-26), **I want** physics-based locomotion driven
   by limb torques and ground reaction forces, **so that** characters interact physically with the
   environment during movement.

7. **US-9.3.9.2** -- **As a** game developer (P-15), **I want** physics-driven characters to stumble
   and recover when pushed, **so that** physical interactions produce emergent reactions.

8. **US-9.3.10.1** -- **As a** character animator (P-11), **I want** socket-based attachment points
   for runtime attachment and removal of skeletal sub-hierarchies, **so that** equipment, weapons,
   and dismembered parts are handled dynamically.

9. **US-9.3.10.2** -- **As an** engine developer (P-26), **I want** dismemberment to sever a bone
   chain and spawn the detached sub-mesh as a physics entity, **so that** the remaining skeleton
   adapts locomotion to compensate.

10. **US-9.3.10.3** -- **As a** game developer (P-15), **I want** severed limbs to retain mesh and
    collision data as physics entities, **so that** dismembered parts interact with the world after
    separation.

11. **US-9.3.11.1** -- **As an** engine developer (P-26), **I want** debug visualization for foot
    placement targets, IK chain states, gait phase diagrams, and balance indicators, **so that**
    procedural locomotion issues are diagnosable in the editor.

12. **US-9.3.11.2** -- **As a** character animator (P-11), **I want** per-entity and per-feature
    toggles for debug overlays, **so that** I inspect specific characters without visual clutter
    from other entities.

13. **US-9.3.11.3** -- **As an** engine developer (P-26), **I want** a locomotion profiler panel
    showing per-frame IK iterations, foot plant errors, and physics force magnitudes, **so that**
    performance bottlenecks are identifiable.

## Pose Source Composition and Advanced IK

| ID           | Persona                    |
|--------------|----------------------------|
| US-9.3.12.1  | character animator (P-11)  |
| US-9.3.13.1  | game developer (P-15)      |
| US-9.3.14.1  | character artist (P-9)     |
| US-9.3.15.1  | engine developer (P-26)    |

1. **US-9.3.12.1** -- **As a** character animator (P-11), **I want** a full-body IK solver with
   center-of-mass balance and prioritized end-effector targets, **so that** characters hold
   physically plausible poses when reaching overhead or leaning on objects.
2. **US-9.3.13.1** -- **As a** game developer (P-15), **I want** motion matching, IK, ragdoll,
   spring bones, and keyframe clips to compose freely in the animation layer stack, **so that**
   complex creatures can combine procedural and authored animation without bespoke glue code.
3. **US-9.3.14.1** -- **As a** character artist (P-9), **I want** spring bones on capes and tails to
   collide with the character body, **so that** secondary motion never penetrates the mesh.
4. **US-9.3.15.1** -- **As an** engine developer (P-26), **I want** to load trained neural
   locomotion policies as an AnimationLayerStack pose source, **so that** research-tier learned
   motion works inside the standard animation pipeline without forking.
