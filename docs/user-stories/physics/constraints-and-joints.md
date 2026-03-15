# User Stories — 4.3 Constraints & Joints

## F-4.3.1 Core Joint Types

**US-4.3.1a** As a designer, I want to connect two entities with a revolute (hinge),
prismatic (slider), fixed (weld), or distance joint so that I can build doors, drawbridges,
sliding mechanisms, and tethered objects visually.

**US-4.3.1b** As a gameplay programmer, I want joints to be ECS entities with `Joint` and
`JointType` components so that I can query, modify, and despawn them using standard ECS
patterns.

## F-4.3.2 Advanced Joint Types

**US-4.3.2a** As a designer, I want cone-twist joints with per-axis angular limits so that
ragdoll shoulders and hips rotate within anatomically plausible ranges.

**US-4.3.2b** As a gameplay programmer, I want a generic 6-DOF joint that can lock, limit,
or free each axis independently so that I can build custom constraint setups for unique
gameplay mechanics.

## F-4.3.3 Joint Motors and Limits

**US-4.3.3a** As a designer, I want to attach a motor to any joint with a target velocity
and maximum force so that I can create powered doors, turrets, and crane arms without
scripting.

**US-4.3.3b** As a player, I want a hinged door to swing open smoothly when pushed and stop
at its limit angle so that mechanical objects behave predictably.

## F-4.3.4 Breakable Joints

**US-4.3.4a** As a designer, I want to set a break force threshold on any joint so that
bridges snap, chains shatter, and vehicle parts detach when forces exceed the configured
limit.

**US-4.3.4b** As a gameplay programmer, I want a `JointBroken` event with both body entity
IDs and the breaking force magnitude so that I can trigger destruction VFX and gameplay
consequences.

## F-4.3.5 Ragdoll Configuration

**US-4.3.5a** As a designer, I want ragdoll definitions to map skeleton bones to joint
types and angular limits so that character death animations transition seamlessly from
animation to physics.

**US-4.3.5b** As a player, I want enemies to crumple realistically when defeated so that
combat feels impactful and each death looks unique.

## F-4.3.6 Joint Chains and Ropes

**US-4.3.6a** As a designer, I want to create ropes, cables, and chains as sequences of
jointed segments so that hanging bridges, drawbridge chains, and ship rigging sway
realistically.

**US-4.3.6b** As a QA engineer, I want to verify that a 32-segment chain remains connected
(no segment separation) under sustained dynamic forces so that rope-like objects never
visually break apart.

## F-4.3.7 Constraint Solvers

**US-4.3.7a** As a gameplay programmer, I want to choose between Sequential Impulse and
Temporal Gauss-Seidel solvers via configuration so that I can trade solver accuracy for
performance depending on the project's constraint complexity.

## F-4.3.8 Limb Severance and Joint Destruction

**US-4.3.8a** As a designer, I want to configure per-joint severance thresholds and per-limb
HP so that progressive limb damage affects gameplay — losing an arm reduces attack damage,
losing a leg slows movement.

**US-4.3.8b** As a player, I want severed limbs to detach as ragdoll entities with VFX at
the separation point so that combat feels visceral and limb damage has visible consequences.

## F-4.3.9 Prosthetic and Limb Replacement

**US-4.3.9a** As a designer, I want to define prosthetic limbs as inventory items with stat
modifiers so that players can recover from limb loss by equipping mechanical arms, peg legs,
or regrown appendages.

**US-4.3.9b** As a player, I want to attach a prosthetic limb and have my character's
movement and abilities adapt immediately so that limb replacement feels meaningful and
responsive.
