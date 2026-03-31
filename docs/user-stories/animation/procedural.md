# User Stories -- 9.3 Procedural Animation

## IK Solvers

| ID          | Persona                    |
|-------------|----------------------------|
| US-9.3.1.1  | engine developer (P-26)    |
| US-9.3.1.2  | character animator (P-11)  |
| US-9.3.2.1  | engine developer (P-26)    |
| US-9.3.2.2  | character animator (P-11)  |
| US-9.3.3.1  | engine developer (P-26)    |

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

5. **US-9.3.3.1** -- **As an** engine developer (P-26), **I want** a FABRIK solver for long chains
   and multi-end-effector problems, **so that** complex IK setups like multi-limbed creatures are
   solvable.

## Ragdoll and Look-At

| ID          | Persona                    |
|-------------|----------------------------|
| US-9.3.4.1  | character animator (P-11)  |
| US-9.3.4.2  | engine developer (P-26)    |
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

4. **US-9.3.5.2** -- **As an** engine developer (P-26), **I want** look-at and aim constraints to
   blend smoothly with underlying animation and respect joint limits, **so that** procedural
   adjustments do not produce unnatural poses.

## Foot Placement and Locomotion

| ID           | Persona                    |
|--------------|----------------------------|
| US-9.3.7.1   | character animator (P-11)  |
| US-9.3.7.2   | engine developer (P-26)    |
| US-9.3.8.1   | rigger (P-10)              |
| US-9.3.8.2   | engine developer (P-26)    |
| US-9.3.9.1   | engine developer (P-26)    |
| US-9.3.10.1  | character animator (P-11)  |
| US-9.3.10.2  | engine developer (P-26)    |
| US-9.3.11.1  | engine developer (P-26)    |

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

5. **US-9.3.9.1** -- **As an** engine developer (P-26), **I want** physics-based locomotion driven
   by limb torques and ground reaction forces, **so that** characters interact physically with the
   environment during movement.

6. **US-9.3.10.1** -- **As a** character animator (P-11), **I want** socket-based attachment points
   for runtime attachment and removal of skeletal sub-hierarchies, **so that** equipment, weapons,
   and dismembered parts are handled dynamically.

7. **US-9.3.10.2** -- **As an** engine developer (P-26), **I want** dismemberment to sever a bone
   chain and spawn the detached sub-mesh as a physics entity, **so that** the remaining skeleton
   adapts locomotion to compensate.

8. **US-9.3.11.1** -- **As an** engine developer (P-26), **I want** debug visualization for foot
   placement targets, IK chain states, gait phase diagrams, and balance indicators, **so that**
   procedural locomotion issues are diagnosable in the editor.
