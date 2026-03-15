# User Stories — 4.7 Soft Body & Cloth

## US-4.7.1.1 Run XPBD Solver on ClothSimulation Entities
**As an** engine developer (P-26), **I want to** implement XpbdSolverSystem querying ClothSimulation
+ Transform entities each tick, **so that** soft body constraints are solved.

## US-4.7.1.2 Configure Constraint Compliance Per Entity
**As a** designer (P-5), **I want to** configure constraint compliance per cloth entity, **so that**
stiffness is tunable independently without coupling to timestep.

## US-4.7.1.3 Store Particle Data in GPU Buffers
**As an** engine developer (P-26), **I want** ClothSimulation to reference GPU buffer handles for
positions, velocities, and constraints, **so that** GPU compute is possible.

## US-4.7.1.4 See Cloth Drape Naturally Over Objects
**As a** player (P-23), **I want** cloth to drape naturally over objects and characters, **so that**
fabrics look realistic.

## US-4.7.1.5 Verify Iteration Limits Per Platform
**As an** engine tester (P-27), **I want to** confirm mobile caps at 2 solver iterations with 128
particles, **so that** cloth stays within frame budget.

## US-4.7.1.6 Support Distance, Bending, and Volume Constraints
**As an** engine developer (P-26), **I want** the solver to handle distance, bending, volume
preservation, and shape-matching constraints, **so that** soft body behavior is rich.

## US-4.7.1.7 Configure Solver Iterations Per Entity
**As a** game developer (P-15), **I want to** set solver iteration count per cloth instance, **so
that** quality-performance tradeoffs are per-entity.

## US-4.7.1.8 Test Solver Stability Under Large Deformations
**As an** engine tester (P-27), **I want to** test XPBD under large deformations, **so that** cloth
does not explode under extreme conditions.

## US-4.7.1.9 Design Scenes with Cloth Decorations
**As a** level designer (P-6), **I want** cloth simulations for banners, curtains, and tapestries,
**so that** environments feel alive.

## US-4.7.1.10 Use GPU Compute on Desktop
**As an** engine developer (P-26), **I want** GPU compute preferred for XPBD on desktop and above,
**so that** particle counts scale to 1024+.

## US-4.7.1.11 See Soft Bodies Deform Under Pressure
**As a** player (P-23), **I want** soft objects to deform when pressed, **so that** interactions
feel tactile.

## US-4.7.1.12 Verify 4096 Particles on High-End PC
**As an** engine tester (P-27), **I want to** test 4096 particles with 16 iterations on high-end PC,
**so that** maximum cloth complexity is validated.

## US-4.7.2.1 Create Cloth Entity with ClothSimulation Component
**As a** game developer (P-15), **I want to** create cloth entities with ClothSimulation components
storing particle buffers and material parameters, **so that** cloth instances are ECS-based.

## US-4.7.2.2 Attach Cloth to Skeleton Bones
**As a** game developer (P-15), **I want to** add ClothAttachment components linking cloth particles
to skeleton bone entities, **so that** capes follow animated characters.

## US-4.7.2.3 Implement ClothSimulationSystem
**As an** engine developer (P-26), **I want to** implement ClothSimulationSystem that steps the
particle-constraint mesh each tick, **so that** cloth animation runs automatically.

## US-4.7.2.4 See Capes Flow Behind Characters
**As a** player (P-23), **I want** character capes and cloaks to flow behind them when running, **so
that** character movement looks dynamic.

## US-4.7.2.5 Configure Stretch, Shear, and Bend Resistance
**As a** designer (P-5), **I want to** set stretch, shear, and bend resistance on cloth materials,
**so that** different fabrics behave distinctly.

## US-4.7.2.6 Verify Cloth Instance Limits Per Platform
**As an** engine tester (P-27), **I want to** confirm mobile caps at 2 active cloth instances
(player only), **so that** frame budget is respected.

## US-4.7.2.7 Replace Distant Cloth with Animation Fallback
**As an** engine developer (P-26), **I want** distant cloth replaced with animation-driven fallback,
**so that** simulation cost is eliminated at range.

## US-4.7.2.8 Design Characters with Flowing Garments
**As a** designer (P-5), **I want** characters to have cloth capes, scarves, and skirts, **so that**
character visual design is rich.

## US-4.7.2.9 Test Cloth Attachment During Animation
**As an** engine tester (P-27), **I want to** test cloth attachment stability during fast character
animations, **so that** cloth does not detach or stretch excessively.

## US-4.7.2.10 Store Mesh Topology in ClothSimulation
**As an** engine developer (P-26), **I want** mesh topology stored in ClothSimulation, **so that**
the system knows particle connectivity for constraint generation.

## US-4.7.2.11 See Banners React to Character Movement
**As a** player (P-23), **I want** carried banners to react to my movement, **so that** held objects
feel physically present.

## US-4.7.2.12 Support 64 Cloth Instances on High-End PC
**As an** engine developer (P-26), **I want** high-end PC to support 64 active cloth instances, **so
that** large scenes have abundant cloth detail.

## US-4.7.3.1 Enable Self-Collision Per Cloth Entity
**As a** game developer (P-15), **I want to** add a SelfCollisionEnabled marker component to cloth
entities, **so that** cloth does not pass through itself.

## US-4.7.3.2 Implement ClothSelfCollisionSystem
**As an** engine developer (P-26), **I want to** implement ClothSelfCollisionSystem using spatial
hashing on particles, **so that** self-intersection is detected.

## US-4.7.3.3 See Cloth Fold Without Clipping Through Itself
**As a** player (P-23), **I want** cloth to fold and drape without passing through itself, **so
that** fabric behavior looks correct.

## US-4.7.3.4 Configure Thickness and Skip Distance
**As a** designer (P-5), **I want to** configure self-collision thickness and skip distance per
cloth entity, **so that** self-collision quality is tunable.

## US-4.7.3.5 Verify Disabled by Default on Mobile
**As an** engine tester (P-27), **I want to** confirm self-collision is disabled on mobile by
default, **so that** GPU budget is preserved.

## US-4.7.3.6 Enable for Player Cloth Only on Switch
**As an** engine tester (P-27), **I want to** verify Switch enables self-collision for player cloth
only with coarse spatial hash, **so that** quality-performance tradeoff is correct.

## US-4.7.3.7 Use BVH-Based Detection on High-End PC
**As an** engine developer (P-26), **I want** high-end PC to use BVH-based self-collision for all
active cloth, **so that** accuracy is maximized.

## US-4.7.3.8 Test Self-Collision Under Folding
**As an** engine tester (P-27), **I want to** test self-collision when cloth is heavily folded, **so
that** extreme configurations do not cause artifacts.

## US-4.7.3.9 Design Characters with Layered Clothing
**As a** designer (P-5), **I want** layered clothing that does not clip through itself, **so that**
complex garments look correct.

## US-4.7.3.10 Experience Correct Cape Folding
**As a** player (P-23), **I want** my cape to fold correctly when I crouch, **so that** cloth
behavior responds to character pose.

## US-4.7.3.11 Implement Spatial Hash for Particle Proximity
**As an** engine developer (P-26), **I want** spatial hashing to detect nearby particle pairs for
self-collision, **so that** detection is O(n) rather than O(n^2).

## US-4.7.3.12 Profile Self-Collision Cost Per Method
**As an** engine tester (P-27), **I want to** profile spatial hash vs BVH self-collision cost, **so
that** I can validate method selection per platform.

## US-4.7.4.1 Generate Contact Constraints Against Rigid Bodies
**As an** engine developer (P-26), **I want** the XPBD solver to read nearby Collider and Transform
components and generate contact constraints against cloth particles, **so that** cloth drapes over
rigid objects.

## US-4.7.4.2 Write Reaction Forces to Rigid Bodies
**As an** engine developer (P-26), **I want** reaction forces written as ExternalForce components on
rigid body entities, **so that** cloth pushes objects back.

## US-4.7.4.3 Drape Cloth Over Environmental Objects
**As a** game developer (P-15), **I want** cloth to drape over tables, rocks, and fences, **so
that** environmental interactions look natural.

## US-4.7.4.4 See Cloth Push Small Objects
**As a** player (P-23), **I want** heavy cloth (a falling curtain) to push small objects aside, **so
that** two-way coupling feels physically correct.

## US-4.7.4.5 Verify Two-Way Coupling Stability
**As an** engine tester (P-27), **I want to** verify two-way cloth-rigid coupling does not cause
instability or oscillation, **so that** the simulation is robust.

## US-4.7.4.6 Resolve All Interaction via ECS Queries
**As an** engine developer (P-26), **I want** all cloth-rigid interaction resolved through ECS
component queries with no separate collision world, **so that** the architecture is unified.

## US-4.7.4.7 Test Cloth Draped Over Moving Objects
**As an** engine tester (P-27), **I want to** test cloth draped over moving objects, **so that**
dynamic interactions work correctly.

## US-4.7.4.8 Design Scenes with Cloth on Furniture
**As a** level designer (P-6), **I want** tablecloths and drapes on furniture, **so that** interiors
feel lived-in and detailed.

## US-4.7.4.9 Configure Coupling Stiffness
**As a** designer (P-5), **I want to** configure cloth-rigid coupling stiffness, **so that** draping
behavior is tunable.

## US-4.7.4.10 See Curtains React When Bumped
**As a** player (P-23), **I want** curtains to react when I walk into them, **so that** the world
feels physically interactive.

## US-4.7.4.11 Test Coupling with Multiple Rigid Bodies
**As an** engine tester (P-27), **I want to** test cloth coupling with multiple simultaneous rigid
bodies, **so that** complex scenes work correctly.

## US-4.7.4.12 Implement Contact Constraint Generation
**As an** engine developer (P-26), **I want** contact constraints generated from cloth particle vs
collider overlap tests, **so that** contact resolution is geometrically accurate.

## US-4.7.5.1 Create WindSource Entities
**As a** game developer (P-15), **I want to** create WindSource entities with type, direction,
strength, and turbulence parameters, **so that** wind affects cloth.

## US-4.7.5.2 Implement WindFieldGenerationSystem
**As an** engine developer (P-26), **I want to** implement WindFieldGenerationSystem that samples
all WindSource entities into a shared 3D wind field texture, **so that** wind is represented
uniformly.

## US-4.7.5.3 Implement ClothWindSystem
**As an** engine developer (P-26), **I want to** implement ClothWindSystem that reads the shared
wind field and applies forces to cloth particles, **so that** cloth reacts to wind.

## US-4.7.5.4 See Banners Flutter in the Wind
**As a** player (P-23), **I want** flags and banners to flutter in the wind, **so that** outdoor
environments feel alive.

## US-4.7.5.5 Configure Wind Source Types
**As a** designer (P-5), **I want to** choose between directional, point, and vortex wind sources,
**so that** different wind effects are possible.

## US-4.7.5.6 Verify Wind Field Resolution Per Platform
**As an** engine tester (P-27), **I want to** confirm mobile uses 32x32x16 wind field with max 2
sources and no turbulence, **so that** budget is controlled.

## US-4.7.5.7 Share Wind Field with Foliage and Particles
**As an** engine developer (P-26), **I want** cloth, hair, foliage, and particles to all sample from
the same shared wind field texture, **so that** wind is consistent across systems.

## US-4.7.5.8 Design Windy Mountain Environments
**As a** level designer (P-6), **I want** strong directional wind on mountain peaks, **so that**
cloth and foliage react to environmental wind.

## US-4.7.5.9 Configure Turbulence Noise Parameters
**As a** designer (P-5), **I want to** set turbulence noise frequency and amplitude on wind sources,
**so that** wind feels natural and varied.

## US-4.7.5.10 Test Wind on Multiple Cloth Instances
**As an** engine tester (P-27), **I want to** test wind affecting multiple cloth instances
simultaneously, **so that** all cloths respond correctly.

## US-4.7.5.11 See Sails Billow in Storm
**As a** player (P-23), **I want** ship sails to billow in strong wind, **so that** sailing feels
dynamic and weather-responsive.

## US-4.7.5.12 Support 16 Wind Sources on Desktop
**As an** engine developer (P-26), **I want** desktop to support 16 wind sources with turbulence and
128x128x64 wind field, **so that** complex wind environments work.

## US-4.7.6.1 Detect Constraint Strain Exceeding Threshold
**As an** engine developer (P-26), **I want** ClothTearingSystem to check constraint strain against
configurable thresholds, **so that** tearing is detected per constraint.

## US-4.7.6.2 Split Cloth Topology on Tear
**As an** engine developer (P-26), **I want** the system to split cloth mesh topology, update
constraint buffers, and spawn new cloth entities for separated patches, **so that** cloth tears into
independent pieces.

## US-4.7.6.3 See Sails Tear in Battle
**As a** player (P-23), **I want** ship sails to tear when hit by projectiles, **so that** naval
combat damage is visible.

## US-4.7.6.4 Configure Tear Strain Threshold
**As a** designer (P-5), **I want to** configure tear strain threshold per cloth material, **so
that** different fabrics tear at different forces.

## US-4.7.6.5 Verify Disabled on Mobile
**As an** engine tester (P-27), **I want to** confirm mobile uses pre-authored torn mesh swap
instead of runtime tearing, **so that** GPU budget is preserved.

## US-4.7.6.6 Generate Proper Boundary Normals on Tear
**As an** engine developer (P-26), **I want** torn edges to generate proper boundary normals, **so
that** rendering of tear edges looks correct.

## US-4.7.6.7 Test Multiple Tears Per Cloth
**As an** engine tester (P-27), **I want to** test multiple tears on a single cloth instance, **so
that** progressive tearing works correctly.

## US-4.7.6.8 Design Destructible Banners
**As a** level designer (P-6), **I want** banners and flags that tear from battle damage, **so
that** siege environments show wear.

## US-4.7.6.9 Limit Tears Per Frame on Switch
**As an** engine tester (P-27), **I want to** verify Switch caps at 1 tear event per frame with max
2 patches, **so that** performance is controlled.

## US-4.7.6.10 See Banner Damaged in Battle
**As a** player (P-23), **I want** battle-damaged banners to show realistic tears, **so that** the
aftermath of combat is visible.

## US-4.7.6.11 Configure Maximum Tear Patches
**As a** designer (P-5), **I want to** configure maximum tear patches per cloth entity, **so that**
tearing complexity is bounded.

## US-4.7.6.12 Support Unlimited Tears on High-End PC
**As an** engine developer (P-26), **I want** high-end PC to support unlimited tears with dynamic
topology updates, **so that** detailed cloth destruction works.

## US-4.7.7.1 Add ClothLod Component to Cloth Entities
**As a** game developer (P-15), **I want to** add ClothLod components to cloth entities, **so that**
simulation fidelity adapts based on distance.

## US-4.7.7.2 Implement ClothLodSystem
**As an** engine developer (P-26), **I want to** implement ClothLodSystem that computes distance
from camera and adjusts particle count, iterations, and update frequency, **so that** cost scales
with visibility.

## US-4.7.7.3 See Nearby Cloth with Full Detail
**As a** player (P-23), **I want** nearby cloth to simulate at full detail, **so that** close-up
fabric looks realistic.

## US-4.7.7.4 Replace Distant Cloth with Animation Fallback
**As an** engine developer (P-26), **I want** cloth at extreme distance replaced with animation
fallback at zero simulation cost, **so that** distant cloth is free.

## US-4.7.7.5 Verify LOD Tiers Per Platform
**As an** engine tester (P-27), **I want to** confirm mobile uses 2 LOD tiers (full, fallback) with
transition at 5m, **so that** LOD behavior matches platform spec.

## US-4.7.7.6 Configure LOD Transition Distances
**As a** designer (P-5), **I want to** set LOD transition distances per platform tier, **so that**
quality-distance tradeoffs are tunable.

## US-4.7.7.7 Interpolate Particle Positions During LOD Transitions
**As an** engine developer (P-26), **I want** particle positions interpolated during LOD tier
transitions, **so that** visual popping is avoided.

## US-4.7.7.8 Test LOD Transition Smoothness
**As an** engine tester (P-27), **I want to** verify LOD transitions do not cause visible popping,
**so that** quality transitions are seamless.

## US-4.7.7.9 See Distant Cloth Still Moving
**As a** player (P-23), **I want** distant cloth to still appear to move (via animation fallback),
**so that** the world looks alive at all distances.

## US-4.7.7.10 Configure Particle Count Per LOD Tier
**As a** designer (P-5), **I want to** set particle count and iteration count per LOD tier, **so
that** each tier has appropriate quality settings.

## US-4.7.7.11 Support 4 LOD Tiers on Desktop
**As an** engine developer (P-26), **I want** desktop to support 4 LOD tiers with transitions at
15m/30m/60m, **so that** cloth quality scales smoothly with distance.

## US-4.7.7.12 Profile LOD System Performance Savings
**As an** engine tester (P-27), **I want to** profile CPU/GPU savings from cloth LOD at various
distances, **so that** the LOD system's benefit is quantified.
