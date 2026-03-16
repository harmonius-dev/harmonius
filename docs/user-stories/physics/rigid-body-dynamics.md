# User Stories — 4.1 Rigid Body Dynamics

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-4.1.1.1 | designer (P-5) | As a designer (P-5), I want to set physics fixed timestep to 1/60s in the project settings, so that physics runs at consistent 60Hz. | — | — | — |
| US-4.1.1.2 | engine tester (P-27) | As an engine tester (P-27), I want to run identical 1000-frame simulations on 3 platforms and compare state, so that determinism is verified. | — | — | — |
| US-4.1.1.3 | player (P-23) | As a player (P-23), I want physics to behave identically at 30fps and 144fps, so that gameplay is fair regardless of hardware. | — | — | — |
| US-4.1.1.4 | engine developer (P-26) | As an engine developer (P-26), I want to implement symplectic Euler integration with fixed dt, so that energy conservation is maintained. | — | — | — |
| US-4.1.1.5 | engine developer (P-26) | As an engine developer (P-26), I want to implement Verlet integration as an alternative method, so that position-based constraints are more stable. | — | — | — |
| US-4.1.1.6 | engine developer (P-26) | As an engine developer (P-26), I want to disable fast-math compiler optimizations on all platforms, so that IEEE 754 compliance is guaranteed. | — | — | — |
| US-4.1.1.7 | engine tester (P-27) | As an engine tester (P-27), I want to record and replay a physics simulation and get bit-identical results, so that bugs are reproducible from saved replays. | — | — | — |
| US-4.1.1.8 | player (P-23) | As a player (P-23), I want thrown objects to follow the same arc every time, so that I can aim reliably. | — | — | — |
| US-4.1.1.9 | game developer (P-15) | As a game developer (P-15), I want to query RigidBody, Velocity, and AngularVelocity components, so that I can read and modify physics state from gameplay systems. | — | — | — |
| US-4.1.1.10 | game developer (P-15) | As a game developer (P-15), I want to add ExternalForce and ExternalTorque components to entities, so that gameplay systems can push objects without touching the integrator. | — | — | — |
| US-4.1.1.11 | engine tester (P-27) | As an engine tester (P-27), I want to assert that floating-point rounding modes are identical on all build targets, so that cross-platform determinism is upheld. | — | — | — |
| US-4.1.1.12 | designer (P-5) | As a designer (P-5), I want to choose between symplectic Euler and Verlet in project settings, so that I can pick the best integrator for my game's needs. | — | — | — |
| US-4.1.2.1 | designer (P-5) | As a designer (P-5), I want to set the global substep count in PhysicsConfig, so that I can trade CPU time for constraint stability per project. | — | — | — |
| US-4.1.2.2 | game developer (P-15) | As a game developer (P-15), I want to add a SubstepOverride component to individual entities, so that a swinging wrecking ball stays stable without raising global substeps. | — | — | — |
| US-4.1.2.3 | engine developer (P-26) | As an engine developer (P-26), I want to repeat integration, broadphase, narrowphase, and solve systems within a substep loop, so that the solver converges correctly. | — | — | — |
| US-4.1.2.4 | engine tester (P-27) | As an engine tester (P-27), I want to confirm that mobile builds cap substeps at 4 and disable per-entity overrides, so that frame budgets are respected. | — | — | — |
| US-4.1.2.5 | player (P-23) | As a player (P-23), I want stacked crates to remain stable, so that physics puzzles feel solid and fair. | — | — | — |
| US-4.1.2.6 | game developer (P-15) | As a game developer (P-15), I want to modify substep count at runtime via the PhysicsConfig resource, so that cutscenes or slow-motion can use more substeps temporarily. | — | — | — |
| US-4.1.2.7 | engine tester (P-27) | As an engine tester (P-27), I want to measure CPU cost per substep count, so that I can recommend platform-appropriate defaults. | — | — | — |
| US-4.1.2.8 | engine developer (P-26) | As an engine developer (P-26), I want to register SubstepOverride via the component registry, so that it can be added and removed at runtime without recompilation. | — | — | — |
| US-4.1.2.9 | designer (P-5) | As a designer (P-5), I want to configure different default substep counts per platform tier, so that each platform runs within its CPU budget automatically. | — | — | — |
| US-4.1.2.10 | engine tester (P-27) | As an engine tester (P-27), I want to run ragdoll simulations at 32 substeps on high-end PC, so that I can verify stability improvements for vehicle and ragdoll systems. | — | — | — |
| US-4.1.3.1 | designer (P-5) | As a designer (P-5), I want to assign a PhysicsMaterial with friction and restitution to a surface, so that ice feels slippery and rubber bounces without code. | — | — | — |
| US-4.1.3.2 | game developer (P-15) | As a game developer (P-15), I want to set combination rules (average, min, max, multiply) for friction and restitution, so that material pairs interact predictably. | — | — | — |
| US-4.1.3.3 | engine developer (P-26) | As an engine developer (P-26), I want to implement impulse-based contact resolution reading ContactManifold and PhysicsMaterial, so that collisions produce correct responses. | — | — | — |
| US-4.1.3.4 | player (P-23) | As a player (P-23), I want a ball dropped on a bouncy surface to rebound to a believable height, so that physics interactions feel consistent. | — | — | — |
| US-4.1.3.5 | engine tester (P-27) | As an engine tester (P-27), I want to test static and dynamic friction across a range of values (0.0 to 2.0), so that edge cases do not produce instabilities. | — | — | — |
| US-4.1.3.6 | level designer (P-6) | As a level designer (P-6), I want to assign different PhysicsMaterials per triangle on a terrain mesh, so that grass, rock, and mud areas have distinct friction. | — | — | — |
| US-4.1.3.7 | player (P-23) | As a player (P-23), I want my character to slide on ice and grip on stone, so that surface types feel distinct during gameplay. | — | — | — |
| US-4.1.3.8 | engine developer (P-26) | As an engine developer (P-26), I want to store MaterialCombinationRules as an ECS resource, so that it is accessible from any system without global state. | — | — | — |
| US-4.1.3.9 | engine tester (P-27) | As an engine tester (P-27), I want to verify that zero-restitution contacts produce no bounce, so that inelastic collisions work correctly. | — | — | — |
| US-4.1.3.10 | designer (P-5) | As a designer (P-5), I want to preview friction and restitution effects in the visual editor, so that I can iterate on material feel without launching the game. | — | — | — |
| US-4.1.3.11 | engine developer (P-26) | As an engine developer (P-26), I want the solver to write impulse results back to Velocity and AngularVelocity components, so that resolved contacts affect body motion. | — | — | — |
| US-4.1.3.12 | engine tester (P-27) | As an engine tester (P-27), I want to verify that material pair combination is symmetric (A vs B equals B vs A), so that collision order does not affect results. | — | — | — |
| US-4.1.4.1 | game developer (P-15) | As a game developer (P-15), I want to add a CcdEnabled component to fast projectiles, so that bullets and arrows never tunnel through thin walls. | — | — | — |
| US-4.1.4.2 | engine developer (P-26) | As an engine developer (P-26), I want to implement swept-volume time-of-impact queries, so that CCD detects collisions between discrete timesteps. | — | — | — |
| US-4.1.4.3 | engine tester (P-27) | As an engine tester (P-27), I want to fire a high-speed sphere at a thin wall and confirm it cannot pass through, so that CCD prevents tunneling. | — | — | — |
| US-4.1.4.4 | player (P-23) | As a player (P-23), I want fast-moving arrows to always hit thin targets, so that combat feels reliable. | — | — | — |
| US-4.1.4.5 | engine tester (P-27) | As an engine tester (P-27), I want to verify that mobile builds cap CCD entities at 16 with sphere-only sweeps, so that frame budget is maintained. | — | — | — |
| US-4.1.4.6 | engine developer (P-26) | As an engine developer (P-26), I want CCD to insert sub-step corrections into Velocity and Transform before the next pass, so that tunneling is resolved cleanly. | — | — | — |
| US-4.1.4.7 | designer (P-5) | As a designer (P-5), I want to set maximum CCD-enabled entity counts per platform tier, so that performance scales appropriately across devices. | — | — | — |
| US-4.1.4.8 | engine developer (P-26) | As an engine developer (P-26), I want desktop CCD to use full convex shape sweeps, so that non-spherical projectiles are handled accurately. | — | — | — |
| US-4.1.4.9 | game developer (P-15) | As a game developer (P-15), I want CCD sweeps to query against broadphase candidates efficiently, so that only relevant pairs are tested. | — | — | — |
| US-4.1.4.10 | player (P-23) | As a player (P-23), I want projectiles to hit consistently whether I run at 30fps or 120fps, so that my hardware does not affect hit registration. | — | — | — |
| US-4.1.4.11 | engine tester (P-27) | As an engine tester (P-27), I want to verify CCD handles fast-rotating objects (e.g., spinning blades), so that angular tunneling is also prevented. | — | — | — |
| US-4.1.4.12 | engine developer (P-26) | As an engine developer (P-26), I want the system to skip CCD processing for entities below a velocity threshold, so that CPU is not wasted on slow objects. | — | — | — |
| US-4.1.5.1 | game developer (P-15) | As a game developer (P-15), I want interacting bodies to be partitioned into simulation islands automatically, so that I do not need to group them manually. | — | — | — |
| US-4.1.5.2 | engine developer (P-26) | As an engine developer (P-26), I want to build a union-find over entities linked by ContactManifold and JointConstraint, so that island computation is efficient. | — | — | — |
| US-4.1.5.3 | engine developer (P-26) | As an engine developer (P-26), I want independent islands to be solved in parallel across CPU cores, so that multi-core hardware is fully utilized. | — | — | — |
| US-4.1.5.4 | engine tester (P-27) | As an engine tester (P-27), I want to confirm that islands merge when contacts form and split when contacts break, so that island state is always correct. | — | — | — |
| US-4.1.5.5 | player (P-23) | As a player (P-23), I want groups of colliding objects to resolve smoothly, so that pile-ups and stacking look natural. | — | — | — |
| US-4.1.5.6 | engine developer (P-26) | As an engine developer (P-26), I want to assign an Island component to each entity, so that downstream systems can query island membership. | — | — | — |
| US-4.1.5.7 | engine tester (P-27) | As an engine tester (P-27), I want to verify mobile caps at 64 active islands with 32 bodies per island, so that constrained platforms stay within budget. | — | — | — |
| US-4.1.5.8 | designer (P-5) | As a designer (P-5), I want to set maximum island counts and body limits per platform, so that physics stays within CPU budget on each device. | — | — | — |
| US-4.1.5.9 | game developer (P-15) | As a game developer (P-15), I want to read the Island component on entities, so that gameplay systems can reason about connected body groups. | — | — | — |
| US-4.1.5.10 | engine developer (P-26) | As an engine developer (P-26), I want island computation to re-run when joints are destroyed, so that severed connections produce new independent islands. | — | — | — |
| US-4.1.5.11 | engine tester (P-27) | As an engine tester (P-27), I want to compare parallel island solve results against serial solve, so that parallelism does not introduce non-determinism. | — | — | — |
| US-4.1.5.12 | player (P-23) | As a player (P-23), I want physics in large worlds with many objects to remain stable, so that distant interactions do not glitch. | — | — | — |
| US-4.1.6.1 | game developer (P-15) | As a game developer (P-15), I want bodies at rest to sleep automatically, so that scenes with thousands of settled objects consume minimal CPU. | — | — | — |
| US-4.1.6.2 | designer (P-5) | As a designer (P-5), I want to configure velocity and energy thresholds in a SleepConfig resource, so that sleep sensitivity is tunable per project. | — | — | — |
| US-4.1.6.3 | engine developer (P-26) | As an engine developer (P-26), I want to add a Sleeping marker component when thresholds are met for the SleepTimer duration, so that integration and solving skip sleeping entities. | — | — | — |
| US-4.1.6.4 | engine developer (P-26) | As an engine developer (P-26), I want wake-up to trigger via ECS change detection on ExternalForce, ExternalTorque, or ContactManifold, so that disturbed bodies resume simulation. | — | — | — |
| US-4.1.6.5 | engine tester (P-27) | As an engine tester (P-27), I want to confirm sleeping entities are skipped by the IntegrationSystem, so that CPU is actually saved. | — | — | — |
| US-4.1.6.6 | player (P-23) | As a player (P-23), I want resting objects to remain perfectly still, so that settled objects do not jitter or drift. | — | — | — |
| US-4.1.6.7 | engine tester (P-27) | As an engine tester (P-27), I want to verify mobile uses aggressive sleep thresholds, so that more bodies sleep and CPU budget is respected. | — | — | — |
| US-4.1.6.8 | game developer (P-15) | As a game developer (P-15), I want to apply ExternalForce to sleeping bodies near an explosion and have them wake up, so that gameplay events affect resting objects. | — | — | — |
| US-4.1.6.9 | engine developer (P-26) | As an engine developer (P-26), I want a SleepTimer component to track sustained rest time, so that transient velocity spikes do not prevent sleeping. | — | — | — |
| US-4.1.6.10 | designer (P-5) | As a designer (P-5), I want to set different sleep thresholds per platform tier, so that mobile sleeps more aggressively than desktop. | — | — | — |
| US-4.1.6.11 | engine tester (P-27) | As an engine tester (P-27), I want to measure the percentage of sleeping bodies in a stress test, so that I can validate sleep is effective at scale. | — | — | — |
| US-4.1.6.12 | player (P-23) | As a player (P-23), I want sleeping objects to wake up instantly when disturbed, so that interactions feel responsive. | — | — | — |
| US-4.1.7.1 | game developer (P-15) | As a game developer (P-15), I want entities crossing zone boundaries to be migrated with all physics components intact, so that objects move seamlessly between zones. | — | — | — |
| US-4.1.7.2 | engine developer (P-26) | As an engine developer (P-26), I want to implement a ZoneMigrationSystem that detects boundary crossings and transfers entities between worlds, so that zone transitions are automatic. | — | — | — |
| US-4.1.7.3 | engine developer (P-26) | As an engine developer (P-26), I want velocity, angular velocity, and contact state to be preserved during migration, so that no discontinuities occur. | — | — | — |
| US-4.1.7.4 | engine tester (P-27) | As an engine tester (P-27), I want to throw a physics object across a zone border and verify smooth motion, so that zone seams are invisible to players. | — | — | — |
| US-4.1.7.5 | player (P-23) | As a player (P-23), I want to throw an object across a zone boundary and see it fly smoothly, so that the world feels continuous. | — | — | — |
| US-4.1.7.6 | level designer (P-6) | As a level designer (P-6), I want physics objects to work across zone boundaries, so that I can design puzzles spanning multiple zones. | — | — | — |
| US-4.1.7.7 | engine developer (P-26) | As an engine developer (P-26), I want RigidBody, Velocity, AngularVelocity, Mass, Inertia, and Collider components all to transfer, so that no state is lost. | — | — | — |
| US-4.1.7.8 | engine tester (P-27) | As an engine tester (P-27), I want to move an entity back and forth across a zone boundary rapidly, so that repeated migration does not corrupt state. | — | — | — |
| US-4.1.7.9 | level designer (P-6) | As a level designer (P-6), I want to place interactive physics objects near zone edges without worrying about boundary issues, so that level design is unconstrained. | — | — | — |
| US-4.1.7.10 | engine developer (P-26) | As an engine developer (P-26), I want entity IDs to be remapped using generational indices during zone transfer, so that references remain valid in the destination world. | — | — | — |
| US-4.1.7.11 | engine tester (P-27) | As an engine tester (P-27), I want to verify that stacked objects crossing a zone boundary maintain their contact relationships, so that stacks do not collapse at borders. | — | — | — |
| US-4.1.7.12 | player (P-23) | As a player (P-23), I want the world to feel like one continuous space, so that zone boundaries are never noticeable during exploration. | — | — | — |
| US-4.1.8.1 | game developer (P-15) | As a game developer (P-15), I want to add CharacterController, GroundState, StepHeight, and SlopeLimit components to an entity, so that it moves as a controllable character. | — | — | — |
| US-4.1.8.2 | engine developer (P-26) | As an engine developer (P-26), I want the controller to detect ground using shape casts against the shared spatial index, so that ground detection is accurate and efficient. | — | — | — |
| US-4.1.8.3 | player (P-23) | As a player (P-23), I want my character to slide on slopes steeper than the slope limit, so that terrain feels physically grounded. | — | — | — |
| US-4.1.8.4 | player (P-23) | As a player (P-23), I want my character to step up stairs and curbs automatically, so that movement over small obstacles is seamless. | — | — | — |
| US-4.1.8.5 | game developer (P-15) | As a game developer (P-15), I want a configurable coyote-time grace period after leaving ground, so that platformer-style jumping feels forgiving. | — | — | — |
| US-4.1.8.6 | player (P-23) | As a player (P-23), I want my character to inherit moving platform velocity, so that I do not slide off elevators and conveyor belts. | — | — | — |
| US-4.1.8.7 | engine tester (P-27) | As an engine tester (P-27), I want to verify mobile caps at 16 controllers with 2 ground casts per frame, so that the frame budget is respected. | — | — | — |
| US-4.1.8.8 | level designer (P-6) | As a level designer (P-6), I want the character controller to handle stairs, slopes, and ledges reliably, so that I can design varied platforming layouts. | — | — | — |
| US-4.1.8.9 | designer (P-5) | As a designer (P-5), I want to set slope angle limits in the visual editor, so that I can tune character movement feel without code. | — | — | — |
| US-4.1.8.10 | engine developer (P-26) | As an engine developer (P-26), I want the controller to produce a DesiredVelocity component, so that the physics integration system consumes it seamlessly. | — | — | — |
| US-4.1.8.11 | engine tester (P-27) | As an engine tester (P-27), I want to test the controller on ramps, stairs, edges, and moving platforms, so that all movement scenarios are validated. | — | — | — |
| US-4.1.8.12 | engine developer (P-26) | As an engine developer (P-26), I want high-end PC to support up to 1024 active controllers, so that MMO crowd scenes are feasible. | — | — | — |
| US-4.1.9.1 | game developer (P-15) | As a game developer (P-15), I want to add a MovingPlatform component to kinematic entities, so that elevators and conveyor belts transport passengers. | — | — | — |
| US-4.1.9.2 | level designer (P-6) | As a level designer (P-6), I want to attach spline paths to moving platforms, so that I can author elevator and rail cart routes visually. | — | — | — |
| US-4.1.9.3 | player (P-23) | As a player (P-23), I want to ride an elevator smoothly without jitter, so that platform rides feel polished. | — | — | — |
| US-4.1.9.4 | engine developer (P-26) | As an engine developer (P-26), I want the system to detect entities standing on the platform via ground-contact queries, so that passengers inherit platform velocity. | — | — | — |
| US-4.1.9.5 | engine developer (P-26) | As an engine developer (P-26), I want platforms to apply smoothed velocity changes during acceleration and deceleration, so that passengers are not launched. | — | — | — |
| US-4.1.9.6 | engine developer (P-26) | As an engine developer (P-26), I want conveyor belts to apply continuous surface velocity to contacting entities, so that objects slide along the belt. | — | — | — |
| US-4.1.9.7 | game developer (P-15) | As a game developer (P-15), I want one-way platforms to use directional collision filtering, so that characters pass through from below but land on top. | — | — | — |
| US-4.1.9.8 | engine tester (P-27) | As an engine tester (P-27), I want to confirm mobile caps at 8 active moving platforms, so that broadphase cost stays within budget. | — | — | — |
| US-4.1.9.9 | level designer (P-6) | As a level designer (P-6), I want rotating platforms, swinging bridges, and rail carts to work reliably, so that I can create varied traversal challenges. | — | — | — |
| US-4.1.9.10 | game developer (P-15) | As a game developer (P-15), I want platform position, direction, and paused state to be replicated, so that all players see consistent platform positions. | — | — | — |
| US-4.1.9.11 | player (P-23) | As a player (P-23), I want to stand on a rotating platform and rotate with it, so that I maintain my position without sliding. | — | — | — |
| US-4.1.9.12 | engine tester (P-27) | As an engine tester (P-27), I want to test entities jumping on and off moving platforms at various speeds, so that passenger detection handles all cases. | — | — | — |
| US-4.1.10.1 | player (P-23) | As a player (P-23), I want my character to move smoothly over terrain tile seams, so that I do not stutter on geometry edges. | — | — | — |
| US-4.1.10.2 | designer (P-5) | As a designer (P-5), I want to set the smoothing half-life parameter, so that I can balance responsiveness versus smoothness for my game. | — | — | — |
| US-4.1.10.3 | engine developer (P-26) | As an engine developer (P-26), I want to apply an exponential moving average filter to ground height and normal, so that micro-bouncing on triangle edges is eliminated. | — | — | — |
| US-4.1.10.4 | level designer (P-6) | As a level designer (P-6), I want to set the maximum step-down distance, so that characters snap downward on descents to stay grounded. | — | — | — |
| US-4.1.10.5 | engine tester (P-27) | As an engine tester (P-27), I want to walk a character over high-frequency heightmap noise and confirm no vibration, so that smoothing works correctly. | — | — | — |
| US-4.1.10.6 | player (P-23) | As a player (P-23), I want my character to traverse jagged, uneven terrain without bouncing, so that movement feels natural. | — | — | — |
| US-4.1.10.7 | engine developer (P-26) | As an engine developer (P-26), I want the smoothed ground state to feed into the foot placement IK system, so that character feet plant naturally on slopes. | — | — | — |
| US-4.1.10.8 | designer (P-5) | As a designer (P-5), I want to configure slope alignment speed, so that the character's visual tilt matches the terrain at a natural rate. | — | — | — |
| US-4.1.10.9 | engine tester (P-27) | As an engine tester (P-27), I want to test surface smoothing on highly tessellated meshes, so that small triangle edges do not cause jitter. | — | — | — |
| US-4.1.10.10 | engine developer (P-26) | As an engine developer (P-26), I want to cast a shape downward each frame to sample ground normal and height, so that the smoothing filter has accurate input. | — | — | — |
| US-4.1.10.11 | level designer (P-6) | As a level designer (P-6), I want characters to traverse rough cave floors smoothly, so that I can create detailed environments without movement issues. | — | — | — |
| US-4.1.10.12 | engine tester (P-27) | As an engine tester (P-27), I want to confirm characters snap down on stair descent within the step-down distance, so that the character stays grounded going downstairs. | — | — | — |
