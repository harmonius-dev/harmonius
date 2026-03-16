# User Stories — 4.3 Constraints & Joints

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-4.3.1.1 | game developer (P-15) | As a game developer (P-15), I want to create a Joint entity with JointType::Revolute linking two body entities, so that they are constrained as a hinge. | — | — | — |
| US-4.3.1.2 | game developer (P-15) | As a game developer (P-15), I want to create a Joint with JointType::Prismatic, so that two bodies slide along a single axis. | — | — | — |
| US-4.3.1.3 | game developer (P-15) | As a game developer (P-15), I want to create a Fixed joint, so that two bodies move as a single rigid unit. | — | — | — |
| US-4.3.1.4 | engine developer (P-26) | As an engine developer (P-26), I want to implement a ConstraintSolverSystem querying (Joint, JointType) entities each substep, so that positional and velocity constraints are resolved. | — | — | — |
| US-4.3.1.5 | game developer (P-15) | As a game developer (P-15), I want to create Distance joints, so that two bodies maintain a fixed or range-limited separation. | — | — | — |
| US-4.3.1.6 | player (P-23) | As a player (P-23), I want hinged doors to swing open and closed naturally, so that the world feels physically interactive. | — | — | — |
| US-4.3.1.7 | engine developer (P-26) | As an engine developer (P-26), I want Joint components to store entity references using generational indices, so that stale references are detected. | — | — | — |
| US-4.3.1.8 | engine tester (P-27) | As an engine tester (P-27), I want to verify that joints converge to stable states within the solver iteration budget, so that connected bodies do not drift apart. | — | — | — |
| US-4.3.1.9 | level designer (P-6) | As a level designer (P-6), I want to place doors, drawbridges, and gates using revolute joints, so that interactive objects work correctly. | — | — | — |
| US-4.3.1.10 | designer (P-5) | As a designer (P-5), I want to select joint type and anchor points in the visual editor, so that joints are set up without code. | — | — | — |
| US-4.3.1.11 | engine tester (P-27) | As an engine tester (P-27), I want to apply large forces to jointed bodies and verify stability, so that extreme gameplay conditions do not break constraints. | — | — | — |
| US-4.3.1.12 | player (P-23) | As a player (P-23), I want chained and jointed objects to respond physically when pushed, so that interactions feel realistic. | — | — | — |
| US-4.3.2.1 | game developer (P-15) | As a game developer (P-15), I want to create JointType::Spring joints with configurable stiffness and damping, so that springy connections between bodies work. | — | — | — |
| US-4.3.2.2 | game developer (P-15) | As a game developer (P-15), I want to create ConeTwist joints with angular limits on all three axes, so that ragdoll limbs have anatomically plausible ranges. | — | — | — |
| US-4.3.2.3 | game developer (P-15) | As a game developer (P-15), I want to create Generic6DOF joints with per-axis lock/limit/free modes, so that complex mechanical connections are possible. | — | — | — |
| US-4.3.2.4 | engine developer (P-26) | As an engine developer (P-26), I want the ConstraintSolverSystem to dispatch to specialized routines per JointType variant, so that each type is solved optimally. | — | — | — |
| US-4.3.2.5 | designer (P-5) | As a designer (P-5), I want to configure per-axis locking, limiting, and freeing via a Dof6Config component, so that complex mechanical joints are tunable visually. | — | — | — |
| US-4.3.2.6 | player (P-23) | As a player (P-23), I want ragdoll joints to have natural angular limits, so that fallen characters look believable. | — | — | — |
| US-4.3.2.7 | engine tester (P-27) | As an engine tester (P-27), I want to verify ConeTwist angular limits prevent motion beyond configured bounds, so that ragdoll limbs do not hyperextend. | — | — | — |
| US-4.3.2.8 | level designer (P-6) | As a level designer (P-6), I want to use ConeTwist joints for turret traverse and elevation limits, so that turrets have constrained rotation ranges. | — | — | — |
| US-4.3.2.9 | engine tester (P-27) | As an engine tester (P-27), I want to verify that locking one axis does not affect other axes, so that per-axis DOF control works correctly. | — | — | — |
| US-4.3.2.10 | designer (P-5) | As a designer (P-5), I want to connect objects with spring joints in the visual editor, so that bouncy, elastic connections are easy to author. | — | — | — |
| US-4.3.2.11 | engine developer (P-26) | As an engine developer (P-26), I want to implement ConeTwist constraint solving with three-axis angular limits, so that rotational freedom is bounded correctly. | — | — | — |
| US-4.3.2.12 | player (P-23) | As a player (P-23), I want spring-connected objects to bounce and oscillate naturally, so that spring physics feels right. | — | — | — |
| US-4.3.3.1 | game developer (P-15) | As a game developer (P-15), I want to add a JointMotor component to a joint entity, so that the joint drives toward a target position or velocity. | — | — | — |
| US-4.3.3.2 | game developer (P-15) | As a game developer (P-15), I want to add JointLimits to constrain angular and linear bounds, so that joints do not exceed physical ranges. | — | — | — |
| US-4.3.3.3 | engine developer (P-26) | As an engine developer (P-26), I want the solver to read JointMotor and apply motor drives as additional constraint rows, so that powered joints work correctly. | — | — | — |
| US-4.3.3.4 | designer (P-5) | As a designer (P-5), I want to set motor target position, velocity, and maximum force in the visual editor, so that motorized joints are tuned visually. | — | — | — |
| US-4.3.3.5 | engine tester (P-27) | As an engine tester (P-27), I want to apply a heavy load to a motor-driven joint and confirm it does not exceed max force, so that motor limits are respected. | — | — | — |
| US-4.3.3.6 | player (P-23) | As a player (P-23), I want motorized doors and gates to open and close smoothly, so that automated mechanisms feel polished. | — | — | — |
| US-4.3.3.7 | game developer (P-15) | As a game developer (P-15), I want to configure restitution and softness on JointLimits, so that limit contacts bounce or absorb appropriately. | — | — | — |
| US-4.3.3.8 | engine tester (P-27) | As an engine tester (P-27), I want to verify JointLimits prevent joints from exceeding angular and linear bounds, so that constraint integrity is maintained. | — | — | — |
| US-4.3.3.9 | level designer (P-6) | As a level designer (P-6), I want to use prismatic joints with motors for elevators, so that vertical platforms move at controlled speeds. | — | — | — |
| US-4.3.3.10 | engine developer (P-26) | As an engine developer (P-26), I want the solver to optionally read JointMotor and JointLimits, so that joints without these components work unchanged. | — | — | — |
| US-4.3.3.11 | engine tester (P-27) | As an engine tester (P-27), I want to verify motor-driven joints reach their target velocity under load, so that motor behavior is predictable. | — | — | — |
| US-4.3.3.12 | player (P-23) | As a player (P-23), I want motorized cranes to lift and lower loads smoothly, so that construction mechanisms feel realistic. | — | — | — |
| US-4.3.4.1 | game developer (P-15) | As a game developer (P-15), I want to add a BreakForce component to joints specifying force and torque thresholds, so that joints break under extreme stress. | — | — | — |
| US-4.3.4.2 | engine developer (P-26) | As an engine developer (P-26), I want the solver to check accumulated impulses against BreakForce thresholds each substep, so that breaking is detected accurately. | — | — | — |
| US-4.3.4.3 | engine developer (P-26) | As an engine developer (P-26), I want a JointBroken ECS event emitted when a joint breaks, so that gameplay systems can react to structural failures. | — | — | — |
| US-4.3.4.4 | player (P-23) | As a player (P-23), I want structures to break apart when hit with enough force, so that destruction feels dynamic and responsive. | — | — | — |
| US-4.3.4.5 | designer (P-5) | As a designer (P-5), I want to set break force and torque thresholds per joint in the visual editor, so that breakability is tunable without code. | — | — | — |
| US-4.3.4.6 | engine tester (P-27) | As an engine tester (P-27), I want to apply measured forces to breakable joints and verify they break at the configured threshold, so that break detection is accurate. | — | — | — |
| US-4.3.4.7 | engine developer (P-26) | As an engine developer (P-26), I want broken joint entities despawned via command buffer, so that structural cleanup is deferred safely. | — | — | — |
| US-4.3.4.8 | game developer (P-15) | As a game developer (P-15), I want to listen for JointBroken events, so that I can trigger VFX, sounds, or gameplay consequences on structural failure. | — | — | — |
| US-4.3.4.9 | engine tester (P-27) | As an engine tester (P-27), I want to test breakable joints under tension, compression, shear, and torsion, so that all load directions trigger breaks correctly. | — | — | — |
| US-4.3.4.10 | level designer (P-6) | As a level designer (P-6), I want to create breakable bridge joints, so that heavy impacts cause dramatic bridge collapses. | — | — | — |
| US-4.3.4.11 | engine developer (P-26) | As an engine developer (P-26), I want JointBroken events to include both body entities and the breaking force magnitude, so that consumers have full context. | — | — | — |
| US-4.3.4.12 | player (P-23) | As a player (P-23), I want chain links to snap when overloaded, so that heavy objects can tear free from tethers. | — | — | — |
| US-4.3.5.1 | game developer (P-15) | As a game developer (P-15), I want to define a RagdollDef asset mapping skeleton bones to joint archetypes, so that ragdoll configuration is data-driven. | — | — | — |
| US-4.3.5.2 | game developer (P-15) | As a game developer (P-15), I want ragdoll joints to spawn when transitioning from animation to physics, so that characters ragdoll on death. | — | — | — |
| US-4.3.5.3 | engine developer (P-26) | As an engine developer (P-26), I want to implement a RagdollActivationSystem that spawns joint entities from the RagdollDef, so that ragdoll activation is automatic. | — | — | — |
| US-4.3.5.4 | designer (P-5) | As a designer (P-5), I want to set anatomically plausible joint limits in the ragdoll definition, so that ragdolls do not hyperextend. | — | — | — |
| US-4.3.5.5 | engine tester (P-27) | As an engine tester (P-27), I want to confirm mobile caps at 4 simultaneous ragdolls with 8 bones each, so that performance budgets are met. | — | — | — |
| US-4.3.5.6 | player (P-23) | As a player (P-23), I want defeated characters to ragdoll naturally, so that death animations feel dynamic and unique. | — | — | — |
| US-4.3.5.7 | engine developer (P-26) | As an engine developer (P-26), I want ragdoll joint entities despawned when returning to animation control, so that resources are freed. | — | — | — |
| US-4.3.5.8 | engine tester (P-27) | As an engine tester (P-27), I want to verify ragdolls behave stably on slopes and stairs, so that bodies do not jitter or clip through geometry. | — | — | — |
| US-4.3.5.9 | level designer (P-6) | As a level designer (P-6), I want NPCs with ragdoll definitions to ragdoll correctly in any placement, so that combat encounters look polished. | — | — | — |
| US-4.3.5.10 | engine developer (P-26) | As an engine developer (P-26), I want high-end PC to support 128 simultaneous ragdolls with full skeleton fidelity, so that large battles look realistic. | — | — | — |
| US-4.3.5.11 | engine developer (P-26) | As an engine developer (P-26), I want distant ragdolls replaced with animation blend on all platforms, so that simulation cost scales with distance. | — | — | — |
| US-4.3.5.12 | engine tester (P-27) | As an engine tester (P-27), I want to test the transition from ragdoll back to animation and verify smooth blending, so that get-up animations look natural. | — | — | — |
| US-4.3.6.1 | game developer (P-15) | As a game developer (P-15), I want to create rope entities from a ChainDef asset specifying segment count and stiffness, so that ropes are data-driven. | — | — | — |
| US-4.3.6.2 | engine developer (P-26) | As an engine developer (P-26), I want chain segments connected by Distance or Spring joint entities, so that the chain forms a linked sequence. | — | — | — |
| US-4.3.6.3 | player (P-23) | As a player (P-23), I want ropes and chains to sway and react to wind, so that the world feels alive. | — | — | — |
| US-4.3.6.4 | engine tester (P-27) | As an engine tester (P-27), I want to confirm mobile caps at 8 segments per chain with 4 active chains, so that performance is controlled. | — | — | — |
| US-4.3.6.5 | designer (P-5) | As a designer (P-5), I want to set chain stiffness and segment count in the visual editor, so that rope feel is tunable without code. | — | — | — |
| US-4.3.6.6 | level designer (P-6) | As a level designer (P-6), I want to place rope bridges between platforms, so that traversal challenges are varied. | — | — | — |
| US-4.3.6.7 | engine developer (P-26) | As an engine developer (P-26), I want distant chains to use a verlet fallback without collision, so that simulation cost is reduced at range. | — | — | — |
| US-4.3.6.8 | engine tester (P-27) | As an engine tester (P-27), I want to verify chain segments collide with environment geometry, so that ropes drape correctly over obstacles. | — | — | — |
| US-4.3.6.9 | player (P-23) | As a player (P-23), I want to grab and swing from ropes, so that traversal mechanics feel dynamic and fun. | — | — | — |
| US-4.3.6.10 | game developer (P-15) | As a game developer (P-15), I want to create cable connections between objects using joint chains, so that power lines and tethers work physically. | — | — | — |
| US-4.3.6.11 | engine developer (P-26) | As an engine developer (P-26), I want high-end PC to support 128 segments per chain with unlimited chains, so that detailed rope physics is possible. | — | — | — |
| US-4.3.6.12 | engine tester (P-27) | As an engine tester (P-27), I want to apply extreme tension to chains and verify stability, so that chains do not explode under load. | — | — | — |
| US-4.3.7.1 | designer (P-5) | As a designer (P-5), I want to select sequential impulse or TGS solver in SolverConfig, so that I can choose the best solver for my game. | — | — | — |
| US-4.3.7.2 | engine developer (P-26) | As an engine developer (P-26), I want to implement sequential impulse (SI) constraint solving, so that a lightweight solver is available for mobile platforms. | — | — | — |
| US-4.3.7.3 | engine developer (P-26) | As an engine developer (P-26), I want to implement TGS constraint solving, so that desktop platforms get higher-fidelity stacking and vehicle constraints. | — | — | — |
| US-4.3.7.4 | designer (P-5) | As a designer (P-5), I want to set solver iteration count in SolverConfig, so that I can trade CPU time for convergence quality. | — | — | — |
| US-4.3.7.5 | engine tester (P-27) | As an engine tester (P-27), I want to verify the solver produces identical results given identical entity ordering, so that server-authoritative simulation works. | — | — | — |
| US-4.3.7.6 | player (P-23) | As a player (P-23), I want stacked objects to remain stable, so that physics puzzles and building mechanics work correctly. | — | — | — |
| US-4.3.7.7 | engine tester (P-27) | As an engine tester (P-27), I want to confirm mobile uses SI solver with 4 iterations only, so that CPU budget is respected. | — | — | — |
| US-4.3.7.8 | engine developer (P-26) | As an engine developer (P-26), I want the solver to parallelize across independent islands, so that multi-core hardware is utilized. | — | — | — |
| US-4.3.7.9 | engine tester (P-27) | As an engine tester (P-27), I want to test TGS with 12+ iterations on high-end PC, so that I can validate improved constraint fidelity. | — | — | — |
| US-4.3.7.10 | engine developer (P-26) | As an engine developer (P-26), I want the solver to write solved impulses to Velocity and AngularVelocity components, so that constraints affect body motion. | — | — | — |
| US-4.3.7.11 | game developer (P-15) | As a game developer (P-15), I want the solver to support deterministic client-side prediction, so that multiplayer physics feels responsive. | — | — | — |
| US-4.3.7.12 | player (P-23) | As a player (P-23), I want vehicle suspension and wheel constraints to feel stable, so that driving is enjoyable. | — | — | — |
| US-4.3.8.1 | designer (P-5) | As a designer (P-5), I want to set severance thresholds per joint in the skeleton asset, so that each limb has an appropriate damage tolerance. | — | — | — |
| US-4.3.8.2 | engine developer (P-26) | As an engine developer (P-26), I want the joint connection destroyed and child bone chain separated as independent physics entities when damage exceeds the threshold, so that limb severance is simulated. | — | — | — |
| US-4.3.8.3 | game developer (P-15) | As a game developer (P-15), I want blood and gore VFX to spawn at the separation point on limb severance, so that the visual feedback is immediate. | — | — | — |
| US-4.3.8.4 | player (P-23) | As a player (P-23), I want to see enemy limbs detach when hit hard enough, so that combat feels visceral and impactful. | — | — | — |
| US-4.3.8.5 | engine developer (P-26) | As an engine developer (P-26), I want JointSevered events emitted through observers, so that gameplay systems react to severance (death on head loss, disarm on arm loss). | — | — | — |
| US-4.3.8.6 | game developer (P-15) | As a game developer (P-15), I want per-joint HP tracked via a LimbHealth component, so that progressive limb damage (Kenshi-style) is supported. | — | — | — |
| US-4.3.8.7 | engine tester (P-27) | As an engine tester (P-27), I want to confirm mobile caps at 2 severed limbs with simplified VFX, so that performance is maintained. | — | — | — |
| US-4.3.8.8 | engine developer (P-26) | As an engine developer (P-26), I want the animation system to adjust locomotion after severance (three-legged gait, one-armed combat), so that remaining skeleton compensates. | — | — | — |
| US-4.3.8.9 | game developer (P-15) | As a game developer (P-15), I want gameplay effects to apply stat penalties when limbs are lost, so that severance has mechanical consequences. | — | — | — |
| US-4.3.8.10 | level designer (P-6) | As a level designer (P-6), I want enemies configured with severable limbs, so that combat encounters have dismemberment opportunities. | — | — | — |
| US-4.3.8.11 | engine tester (P-27) | As an engine tester (P-27), I want to test severance with slash, impact, and explosive damage types, so that all damage vectors trigger correctly. | — | — | — |
| US-4.3.8.12 | player (P-23) | As a player (P-23), I want severed limbs to ragdoll independently with their mesh and collision, so that detached parts behave physically. | — | — | — |
| US-4.3.9.1 | game developer (P-15) | As a game developer (P-15), I want to attach replacement limbs to severed joint sockets at runtime, so that prosthetic mechanics work. | — | — | — |
| US-4.3.9.2 | designer (P-5) | As a designer (P-5), I want to define ProstheticDef assets with replacement mesh, bone chain, and stat modifiers, so that prosthetics are data-driven. | — | — | — |
| US-4.3.9.3 | engine developer (P-26) | As an engine developer (P-26), I want attachment to re-establish physics constraints at the socket joint, so that prosthetic limbs are physically connected. | — | — | — |
| US-4.3.9.4 | player (P-23) | As a player (P-23), I want to see my character's prosthetic limb attached and animated, so that the replacement feels like part of the body. | — | — | — |
| US-4.3.9.5 | engine developer (P-26) | As an engine developer (P-26), I want the animation system to re-evaluate locomotion profiles for the prosthetic's properties, so that a peg leg walks differently than a mechanical leg. | — | — | — |
| US-4.3.9.6 | game developer (P-15) | As a game developer (P-15), I want prosthetics to be inventory items equipped through the character equipment system, so that replacement is integrated with progression. | — | — | — |
| US-4.3.9.7 | engine tester (P-27) | As an engine tester (P-27), I want to verify prosthetic attachment creates stable physics constraints, so that replacement limbs do not jitter or detach. | — | — | — |
| US-4.3.9.8 | designer (P-5) | As a designer (P-5), I want ProstheticDef to specify compatible socket types (arm, leg, tail), so that only valid attachments are allowed. | — | — | — |
| US-4.3.9.9 | game developer (P-15) | As a game developer (P-15), I want prosthetic stat modifiers (strength, weight) applied on attachment, so that gameplay balance reflects the replacement. | — | — | — |
| US-4.3.9.10 | player (P-23) | As a player (P-23), I want a snap animation to play when a prosthetic is attached, so that the replacement process feels polished. | — | — | — |
| US-4.3.9.11 | engine tester (P-27) | As an engine tester (P-27), I want to test prosthetic attachment on arm, leg, and tail sockets, so that all socket types work correctly. | — | — | — |
| US-4.3.9.12 | designer (P-5) | As a designer (P-5), I want multiple prosthetic tiers (wooden peg, mechanical arm, magical limb), so that limb replacement is part of character progression. | — | — | — |
