# User Stories — 4.1 Rigid Body Dynamics

## US-4.1.1.1 Set Fixed Timestep in Project Settings
**As a** designer (P-5), **I want to** set physics fixed timestep to 1/60s in the project settings,
**so that** physics runs at consistent 60Hz.

## US-4.1.1.2 Verify Cross-Platform Determinism
**As an** engine tester (P-27), **I want to** run identical 1000-frame simulations on 3 platforms
and compare state, **so that** determinism is verified.

## US-4.1.1.3 Experience Consistent Physics at Any FPS
**As a** player (P-23), **I want** physics to behave identically at 30fps and 144fps, **so that**
gameplay is fair regardless of hardware.

## US-4.1.1.4 Implement Symplectic Euler Integrator
**As an** engine developer (P-26), **I want to** implement symplectic Euler integration with fixed
dt, **so that** energy conservation is maintained.

## US-4.1.1.5 Implement Verlet Integration
**As an** engine developer (P-26), **I want to** implement Verlet integration as an alternative
method, **so that** position-based constraints are more stable.

## US-4.1.1.6 Disable Fast-Math Across All Compilers
**As an** engine developer (P-26), **I want to** disable fast-math compiler optimizations on all
platforms, **so that** IEEE 754 compliance is guaranteed.

## US-4.1.1.7 Replay Deterministic Physics Scenario
**As an** engine tester (P-27), **I want to** record and replay a physics simulation and get
bit-identical results, **so that** bugs are reproducible from saved replays.

## US-4.1.1.8 See Predictable Projectile Trajectories
**As a** player (P-23), **I want** thrown objects to follow the same arc every time, **so that** I
can aim reliably.

## US-4.1.1.9 Query Integration Components in ECS
**As a** game developer (P-15), **I want to** query RigidBody, Velocity, and AngularVelocity
components, **so that** I can read and modify physics state from gameplay systems.

## US-4.1.1.10 Apply External Forces via ECS Components
**As a** game developer (P-15), **I want to** add ExternalForce and ExternalTorque components to
entities, **so that** gameplay systems can push objects without touching the integrator.

## US-4.1.1.11 Validate Rounding Modes Match Across Platforms
**As an** engine tester (P-27), **I want to** assert that floating-point rounding modes are
identical on all build targets, **so that** cross-platform determinism is upheld.

## US-4.1.1.12 Configure Integration Method Per Project
**As a** designer (P-5), **I want to** choose between symplectic Euler and Verlet in project
settings, **so that** I can pick the best integrator for my game's needs.

## US-4.1.2.1 Set Global Substep Count in PhysicsConfig
**As a** designer (P-5), **I want to** set the global substep count in PhysicsConfig, **so that** I
can trade CPU time for constraint stability per project.

## US-4.1.2.2 Override Substep Count Per Entity
**As a** game developer (P-15), **I want to** add a SubstepOverride component to individual
entities, **so that** a swinging wrecking ball stays stable without raising global substeps.

## US-4.1.2.3 Implement Substep Loop in ECS Schedule
**As an** engine developer (P-26), **I want to** repeat integration, broadphase, narrowphase, and
solve systems within a substep loop, **so that** the solver converges correctly.

## US-4.1.2.4 Verify Substep Limits on Mobile
**As an** engine tester (P-27), **I want to** confirm that mobile builds cap substeps at 4 and
disable per-entity overrides, **so that** frame budgets are respected.

## US-4.1.2.5 See Stable Stacking with Higher Substeps
**As a** player (P-23), **I want** stacked crates to remain stable, **so that** physics puzzles feel
solid and fair.

## US-4.1.2.6 Adjust Substep Count at Runtime
**As a** game developer (P-15), **I want to** modify substep count at runtime via the PhysicsConfig
resource, **so that** cutscenes or slow-motion can use more substeps temporarily.

## US-4.1.2.7 Profile Substep Performance Impact
**As an** engine tester (P-27), **I want to** measure CPU cost per substep count, **so that** I can
recommend platform-appropriate defaults.

## US-4.1.2.8 Register SubstepOverride as a Dynamic Component
**As an** engine developer (P-26), **I want to** register SubstepOverride via the component
registry, **so that** it can be added and removed at runtime without recompilation.

## US-4.1.2.9 Set Substep Defaults Per Platform Tier
**As a** designer (P-5), **I want to** configure different default substep counts per platform tier,
**so that** each platform runs within its CPU budget automatically.

## US-4.1.2.10 Test High-Substep Ragdoll Stability
**As an** engine tester (P-27), **I want to** run ragdoll simulations at 32 substeps on high-end PC,
**so that** I can verify stability improvements for vehicle and ragdoll systems.

## US-4.1.3.1 Assign Physics Material to Surface
**As a** designer (P-5), **I want to** assign a PhysicsMaterial with friction and restitution to a
surface, **so that** ice feels slippery and rubber bounces without code.

## US-4.1.3.2 Configure Material Combination Rules
**As a** game developer (P-15), **I want to** set combination rules (average, min, max, multiply)
for friction and restitution, **so that** material pairs interact predictably.

## US-4.1.3.3 Implement Impulse-Based Contact Solver
**As an** engine developer (P-26), **I want to** implement impulse-based contact resolution reading
ContactManifold and PhysicsMaterial, **so that** collisions produce correct responses.

## US-4.1.3.4 See Ball Bounce Believably
**As a** player (P-23), **I want** a ball dropped on a bouncy surface to rebound to a believable
height, **so that** physics interactions feel consistent.

## US-4.1.3.5 Verify Friction Coefficient Ranges
**As an** engine tester (P-27), **I want to** test static and dynamic friction across a range of
values (0.0 to 2.0), **so that** edge cases do not produce instabilities.

## US-4.1.3.6 Apply Per-Triangle Material on Trimesh
**As a** level designer (P-6), **I want to** assign different PhysicsMaterials per triangle on a
terrain mesh, **so that** grass, rock, and mud areas have distinct friction.

## US-4.1.3.7 Slide Correctly on Ice Surfaces
**As a** player (P-23), **I want** my character to slide on ice and grip on stone, **so that**
surface types feel distinct during gameplay.

## US-4.1.3.8 Store Combination Rules in ECS Resource
**As an** engine developer (P-26), **I want to** store MaterialCombinationRules as an ECS resource,
**so that** it is accessible from any system without global state.

## US-4.1.3.9 Test Zero-Restitution Collisions
**As an** engine tester (P-27), **I want to** verify that zero-restitution contacts produce no
bounce, **so that** inelastic collisions work correctly.

## US-4.1.3.10 Preview Material Properties in Editor
**As a** designer (P-5), **I want to** preview friction and restitution effects in the visual
editor, **so that** I can iterate on material feel without launching the game.

## US-4.1.3.11 Write Impulse Results Back to Velocity
**As an** engine developer (P-26), **I want** the solver to write impulse results back to Velocity
and AngularVelocity components, **so that** resolved contacts affect body motion.

## US-4.1.3.12 Test Material Combination Symmetry
**As an** engine tester (P-27), **I want to** verify that material pair combination is symmetric (A
vs B equals B vs A), **so that** collision order does not affect results.

## US-4.1.4.1 Flag Fast Entities for CCD
**As a** game developer (P-15), **I want to** add a CcdEnabled component to fast projectiles, **so
that** bullets and arrows never tunnel through thin walls.

## US-4.1.4.2 Implement Swept-Volume TOI Queries
**As an** engine developer (P-26), **I want to** implement swept-volume time-of-impact queries, **so
that** CCD detects collisions between discrete timesteps.

## US-4.1.4.3 Verify No Tunneling Through Thin Walls
**As an** engine tester (P-27), **I want to** fire a high-speed sphere at a thin wall and confirm it
cannot pass through, **so that** CCD prevents tunneling.

## US-4.1.4.4 Hit Targets Reliably with Fast Projectiles
**As a** player (P-23), **I want** fast-moving arrows to always hit thin targets, **so that** combat
feels reliable.

## US-4.1.4.5 Enforce CCD Entity Budget on Mobile
**As an** engine tester (P-27), **I want to** verify that mobile builds cap CCD entities at 16 with
sphere-only sweeps, **so that** frame budget is maintained.

## US-4.1.4.6 Perform Sub-Step Corrections for CCD
**As an** engine developer (P-26), **I want** CCD to insert sub-step corrections into Velocity and
Transform before the next pass, **so that** tunneling is resolved cleanly.

## US-4.1.4.7 Configure CCD Entity Budget Per Platform
**As a** designer (P-5), **I want to** set maximum CCD-enabled entity counts per platform tier, **so
that** performance scales appropriately across devices.

## US-4.1.4.8 Use Full Convex Sweeps on Desktop
**As an** engine developer (P-26), **I want** desktop CCD to use full convex shape sweeps, **so
that** non-spherical projectiles are handled accurately.

## US-4.1.4.9 Query CCD Against Broadphase Candidates
**As a** game developer (P-15), **I want** CCD sweeps to query against broadphase candidates
efficiently, **so that** only relevant pairs are tested.

## US-4.1.4.10 See Consistent Hits Regardless of Frame Rate
**As a** player (P-23), **I want** projectiles to hit consistently whether I run at 30fps or 120fps,
**so that** my hardware does not affect hit registration.

## US-4.1.4.11 Test CCD with Rotating Objects
**As an** engine tester (P-27), **I want to** verify CCD handles fast-rotating objects (e.g.,
spinning blades), **so that** angular tunneling is also prevented.

## US-4.1.4.12 Disable CCD for Slow Entities Automatically
**As an** engine developer (P-26), **I want** the system to skip CCD processing for entities below a
velocity threshold, **so that** CPU is not wasted on slow objects.

## US-4.1.5.1 Partition Bodies into Islands Automatically
**As a** game developer (P-15), **I want** interacting bodies to be partitioned into simulation
islands automatically, **so that** I do not need to group them manually.

## US-4.1.5.2 Implement Union-Find for Island Computation
**As an** engine developer (P-26), **I want to** build a union-find over entities linked by
ContactManifold and JointConstraint, **so that** island computation is efficient.

## US-4.1.5.3 Solve Independent Islands in Parallel
**As an** engine developer (P-26), **I want** independent islands to be solved in parallel across
CPU cores, **so that** multi-core hardware is fully utilized.

## US-4.1.5.4 Verify Island Merge and Split Each Frame
**As an** engine tester (P-27), **I want to** confirm that islands merge when contacts form and
split when contacts break, **so that** island state is always correct.

## US-4.1.5.5 Experience Smooth Multi-Object Interactions
**As a** player (P-23), **I want** groups of colliding objects to resolve smoothly, **so that**
pile-ups and stacking look natural.

## US-4.1.5.6 Assign Island Component to Each Entity
**As an** engine developer (P-26), **I want to** assign an Island component to each entity, **so
that** downstream systems can query island membership.

## US-4.1.5.7 Enforce Island Limits on Mobile
**As an** engine tester (P-27), **I want to** verify mobile caps at 64 active islands with 32 bodies
per island, **so that** constrained platforms stay within budget.

## US-4.1.5.8 Configure Island Size Limits Per Platform
**As a** designer (P-5), **I want to** set maximum island counts and body limits per platform, **so
that** physics stays within CPU budget on each device.

## US-4.1.5.9 Query Island Membership from Gameplay
**As a** game developer (P-15), **I want to** read the Island component on entities, **so that**
gameplay systems can reason about connected body groups.

## US-4.1.5.10 Recalculate Islands After Joint Destruction
**As an** engine developer (P-26), **I want** island computation to re-run when joints are
destroyed, **so that** severed connections produce new independent islands.

## US-4.1.5.11 Test Parallel Solve Correctness
**As an** engine tester (P-27), **I want to** compare parallel island solve results against serial
solve, **so that** parallelism does not introduce non-determinism.

## US-4.1.5.12 Benefit from Stable Physics in Large Scenes
**As a** player (P-23), **I want** physics in large worlds with many objects to remain stable, **so
that** distant interactions do not glitch.

## US-4.1.6.1 Sleep Bodies at Rest Automatically
**As a** game developer (P-15), **I want** bodies at rest to sleep automatically, **so that** scenes
with thousands of settled objects consume minimal CPU.

## US-4.1.6.2 Configure Sleep Thresholds in SleepConfig
**As a** designer (P-5), **I want to** configure velocity and energy thresholds in a SleepConfig
resource, **so that** sleep sensitivity is tunable per project.

## US-4.1.6.3 Implement Sleeping Marker Component
**As an** engine developer (P-26), **I want to** add a Sleeping marker component when thresholds are
met for the SleepTimer duration, **so that** integration and solving skip sleeping entities.

## US-4.1.6.4 Wake Bodies on Force or Contact Change
**As an** engine developer (P-26), **I want** wake-up to trigger via ECS change detection on
ExternalForce, ExternalTorque, or ContactManifold, **so that** disturbed bodies resume simulation.

## US-4.1.6.5 Verify Sleeping Bodies Skip Integration
**As an** engine tester (P-27), **I want to** confirm sleeping entities are skipped by the
IntegrationSystem, **so that** CPU is actually saved.

## US-4.1.6.6 See Objects Stay Put When Undisturbed
**As a** player (P-23), **I want** resting objects to remain perfectly still, **so that** settled
objects do not jitter or drift.

## US-4.1.6.7 Use Aggressive Sleep Thresholds on Mobile
**As an** engine tester (P-27), **I want to** verify mobile uses aggressive sleep thresholds, **so
that** more bodies sleep and CPU budget is respected.

## US-4.1.6.8 Wake Bodies When Nearby Explosion Occurs
**As a** game developer (P-15), **I want to** apply ExternalForce to sleeping bodies near an
explosion and have them wake up, **so that** gameplay events affect resting objects.

## US-4.1.6.9 Track Sleep Duration with SleepTimer
**As an** engine developer (P-26), **I want** a SleepTimer component to track sustained rest time,
**so that** transient velocity spikes do not prevent sleeping.

## US-4.1.6.10 Tune Sleep Thresholds Per Platform
**As a** designer (P-5), **I want to** set different sleep thresholds per platform tier, **so that**
mobile sleeps more aggressively than desktop.

## US-4.1.6.11 Verify Sleeping Ratio Under Load
**As an** engine tester (P-27), **I want to** measure the percentage of sleeping bodies in a stress
test, **so that** I can validate sleep is effective at scale.

## US-4.1.6.12 See Woken Objects React Immediately
**As a** player (P-23), **I want** sleeping objects to wake up instantly when disturbed, **so that**
interactions feel responsive.

## US-4.1.7.1 Migrate Physics Entities Across Zone Boundaries
**As a** game developer (P-15), **I want** entities crossing zone boundaries to be migrated with all
physics components intact, **so that** objects move seamlessly between zones.

## US-4.1.7.2 Implement ZoneMigrationSystem for Entity Transfer
**As an** engine developer (P-26), **I want to** implement a ZoneMigrationSystem that detects
boundary crossings and transfers entities between worlds, **so that** zone transitions are
automatic.

## US-4.1.7.3 Preserve Momentum During Zone Migration
**As an** engine developer (P-26), **I want** velocity, angular velocity, and contact state to be
preserved during migration, **so that** no discontinuities occur.

## US-4.1.7.4 Verify No Visual Pop at Zone Borders
**As an** engine tester (P-27), **I want to** throw a physics object across a zone border and verify
smooth motion, **so that** zone seams are invisible to players.

## US-4.1.7.5 Throw Objects Across Zone Borders Seamlessly
**As a** player (P-23), **I want to** throw an object across a zone boundary and see it fly
smoothly, **so that** the world feels continuous.

## US-4.1.7.6 Design Multi-Zone Physics Puzzles
**As a** level designer (P-6), **I want** physics objects to work across zone boundaries, **so
that** I can design puzzles spanning multiple zones.

## US-4.1.7.7 Transfer All Physics Components During Migration
**As an** engine developer (P-26), **I want** RigidBody, Velocity, AngularVelocity, Mass, Inertia,
and Collider components all to transfer, **so that** no state is lost.

## US-4.1.7.8 Test Bidirectional Zone Crossing
**As an** engine tester (P-27), **I want to** move an entity back and forth across a zone boundary
rapidly, **so that** repeated migration does not corrupt state.

## US-4.1.7.9 Place Physics Objects Near Zone Edges
**As a** level designer (P-6), **I want to** place interactive physics objects near zone edges
without worrying about boundary issues, **so that** level design is unconstrained.

## US-4.1.7.10 Handle Entity ID Remapping During Migration
**As an** engine developer (P-26), **I want** entity IDs to be remapped using generational indices
during zone transfer, **so that** references remain valid in the destination world.

## US-4.1.7.11 Verify Contact State Preservation
**As an** engine tester (P-27), **I want to** verify that stacked objects crossing a zone boundary
maintain their contact relationships, **so that** stacks do not collapse at borders.

## US-4.1.7.12 Experience Continuous Open World
**As a** player (P-23), **I want** the world to feel like one continuous space, **so that** zone
boundaries are never noticeable during exploration.

## US-4.1.8.1 Configure Character Controller Components
**As a** game developer (P-15), **I want to** add CharacterController, GroundState, StepHeight, and
SlopeLimit components to an entity, **so that** it moves as a controllable character.

## US-4.1.8.2 Detect Ground via Shape Casts
**As an** engine developer (P-26), **I want** the controller to detect ground using shape casts
against the shared spatial index, **so that** ground detection is accurate and efficient.

## US-4.1.8.3 Slide on Steep Slopes
**As a** player (P-23), **I want** my character to slide on slopes steeper than the slope limit,
**so that** terrain feels physically grounded.

## US-4.1.8.4 Climb Steps and Curbs Smoothly
**As a** player (P-23), **I want** my character to step up stairs and curbs automatically, **so
that** movement over small obstacles is seamless.

## US-4.1.8.5 Implement Coyote-Time Grace Period
**As a** game developer (P-15), **I want** a configurable coyote-time grace period after leaving
ground, **so that** platformer-style jumping feels forgiving.

## US-4.1.8.6 Ride Moving Platforms Without Sliding
**As a** player (P-23), **I want** my character to inherit moving platform velocity, **so that** I
do not slide off elevators and conveyor belts.

## US-4.1.8.7 Verify Controller Budget on Mobile
**As an** engine tester (P-27), **I want to** verify mobile caps at 16 controllers with 2 ground
casts per frame, **so that** the frame budget is respected.

## US-4.1.8.8 Design Platforming Sections
**As a** level designer (P-6), **I want** the character controller to handle stairs, slopes, and
ledges reliably, **so that** I can design varied platforming layouts.

## US-4.1.8.9 Configure Slope Limit in Visual Editor
**As a** designer (P-5), **I want to** set slope angle limits in the visual editor, **so that** I
can tune character movement feel without code.

## US-4.1.8.10 Produce DesiredVelocity for Physics Integration
**As an** engine developer (P-26), **I want** the controller to produce a DesiredVelocity component,
**so that** the physics integration system consumes it seamlessly.

## US-4.1.8.11 Test Character Controller on Varied Geometry
**As an** engine tester (P-27), **I want to** test the controller on ramps, stairs, edges, and
moving platforms, **so that** all movement scenarios are validated.

## US-4.1.8.12 Support 1024 Controllers for MMO Crowds
**As an** engine developer (P-26), **I want** high-end PC to support up to 1024 active controllers,
**so that** MMO crowd scenes are feasible.

## US-4.1.9.1 Mark Platforms as MovingPlatform Entities
**As a** game developer (P-15), **I want to** add a MovingPlatform component to kinematic entities,
**so that** elevators and conveyor belts transport passengers.

## US-4.1.9.2 Drive Platform Motion via Spline Paths
**As a** level designer (P-6), **I want to** attach spline paths to moving platforms, **so that** I
can author elevator and rail cart routes visually.

## US-4.1.9.3 Ride Elevator Without Jitter
**As a** player (P-23), **I want to** ride an elevator smoothly without jitter, **so that** platform
rides feel polished.

## US-4.1.9.4 Detect Passengers via Ground-Contact Queries
**As an** engine developer (P-26), **I want** the system to detect entities standing on the platform
via ground-contact queries, **so that** passengers inherit platform velocity.

## US-4.1.9.5 Smooth Acceleration and Deceleration
**As an** engine developer (P-26), **I want** platforms to apply smoothed velocity changes during
acceleration and deceleration, **so that** passengers are not launched.

## US-4.1.9.6 Implement Conveyor Belt Surface Velocity
**As an** engine developer (P-26), **I want** conveyor belts to apply continuous surface velocity to
contacting entities, **so that** objects slide along the belt.

## US-4.1.9.7 Implement One-Way Platform Filtering
**As a** game developer (P-15), **I want** one-way platforms to use directional collision filtering,
**so that** characters pass through from below but land on top.

## US-4.1.9.8 Verify Platform Limits on Mobile
**As an** engine tester (P-27), **I want to** confirm mobile caps at 8 active moving platforms, **so
that** broadphase cost stays within budget.

## US-4.1.9.9 Design Complex Platform Puzzles
**As a** level designer (P-6), **I want** rotating platforms, swinging bridges, and rail carts to
work reliably, **so that** I can create varied traversal challenges.

## US-4.1.9.10 Replicate Platform State for Multiplayer
**As a** game developer (P-15), **I want** platform position, direction, and paused state to be
replicated, **so that** all players see consistent platform positions.

## US-4.1.9.11 Ride Rotating Platform Without Sliding Off
**As a** player (P-23), **I want to** stand on a rotating platform and rotate with it, **so that** I
maintain my position without sliding.

## US-4.1.9.12 Test Passenger Detection Edge Cases
**As an** engine tester (P-27), **I want to** test entities jumping on and off moving platforms at
various speeds, **so that** passenger detection handles all cases.

## US-4.1.10.1 Smooth Character Movement Over Terrain Seams
**As a** player (P-23), **I want** my character to move smoothly over terrain tile seams, **so
that** I do not stutter on geometry edges.

## US-4.1.10.2 Configure Smoothing Half-Life
**As a** designer (P-5), **I want to** set the smoothing half-life parameter, **so that** I can
balance responsiveness versus smoothness for my game.

## US-4.1.10.3 Implement Exponential Moving Average Filter
**As an** engine developer (P-26), **I want to** apply an exponential moving average filter to
ground height and normal, **so that** micro-bouncing on triangle edges is eliminated.

## US-4.1.10.4 Configure Maximum Step-Down Distance
**As a** level designer (P-6), **I want to** set the maximum step-down distance, **so that**
characters snap downward on descents to stay grounded.

## US-4.1.10.5 Verify No Vibration on High-Frequency Terrain
**As an** engine tester (P-27), **I want to** walk a character over high-frequency heightmap noise
and confirm no vibration, **so that** smoothing works correctly.

## US-4.1.10.6 Walk Over Rough Ground Without Bouncing
**As a** player (P-23), **I want** my character to traverse jagged, uneven terrain without bouncing,
**so that** movement feels natural.

## US-4.1.10.7 Feed Smoothed Ground State to Foot IK
**As an** engine developer (P-26), **I want** the smoothed ground state to feed into the foot
placement IK system, **so that** character feet plant naturally on slopes.

## US-4.1.10.8 Adjust Slope Alignment Speed
**As a** designer (P-5), **I want to** configure slope alignment speed, **so that** the character's
visual tilt matches the terrain at a natural rate.

## US-4.1.10.9 Test Smoothing on Tessellated Meshes
**As an** engine tester (P-27), **I want to** test surface smoothing on highly tessellated meshes,
**so that** small triangle edges do not cause jitter.

## US-4.1.10.10 Sample Ground Normal with Shape Cast
**As an** engine developer (P-26), **I want to** cast a shape downward each frame to sample ground
normal and height, **so that** the smoothing filter has accurate input.

## US-4.1.10.11 Design Rough Cave Terrain
**As a** level designer (P-6), **I want** characters to traverse rough cave floors smoothly, **so
that** I can create detailed environments without movement issues.

## US-4.1.10.12 Verify Step-Down on Stair Descent
**As an** engine tester (P-27), **I want to** confirm characters snap down on stair descent within
the step-down distance, **so that** the character stays grounded going downstairs.
