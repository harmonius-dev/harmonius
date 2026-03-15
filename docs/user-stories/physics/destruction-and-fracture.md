# User Stories — 4.6 Destruction & Fracture

## US-4.6.1.1 Generate Voronoi Fracture Patterns at Build Time
**As a** designer (P-5), **I want to** generate 3D Voronoi fracture patterns at build time, **so
that** no runtime mesh generation is needed.

## US-4.6.1.2 Seed Fracture Points with Multiple Methods
**As a** designer (P-5), **I want to** seed fracture points using random, impact-directed, and
artist-guided placement, **so that** fracture patterns suit the object.

## US-4.6.1.3 Implement Voronoi Decomposition Pipeline
**As an** engine developer (P-26), **I want to** implement 3D Voronoi decomposition producing convex
hull fragments, connectivity graph, and joint configs, **so that** fracture assets are complete.

## US-4.6.1.4 Reference Fracture Asset via Destructible Component
**As a** game developer (P-15), **I want** entities with a Destructible component to reference a
fracture asset by handle, **so that** destruction is data-driven.

## US-4.6.1.5 See Objects Shatter Into Fragments
**As a** player (P-23), **I want** destructible objects to shatter into fragments when hit, **so
that** destruction feels dynamic.

## US-4.6.1.6 Verify Fragment Limits Per Platform
**As an** engine tester (P-27), **I want to** confirm mobile caps at 8 fragments per fracture asset,
**so that** runtime performance is controlled.

## US-4.6.1.7 Bake Platform-Specific Fragment LODs
**As an** engine developer (P-26), **I want** platform-specific LOD variants baked at build time,
**so that** fragment count scales with device capability.

## US-4.6.1.8 Place Destructible Objects in Levels
**As a** level designer (P-6), **I want to** place destructible objects with pre-baked fracture
assets, **so that** destruction is predictable and performant.

## US-4.6.1.9 Author Fracture Seeds in Visual Editor
**As a** designer (P-5), **I want to** place fracture seed points visually in the editor, **so
that** art-directed shattering is easy to author.

## US-4.6.1.10 Test Convex Hull Validity
**As an** engine tester (P-27), **I want to** verify all generated fragments are valid convex hulls,
**so that** collision detection works correctly on fragments.

## US-4.6.1.11 Support 256 Fragments on High-End PC
**As an** engine developer (P-26), **I want** high-end PC to support 256 fragments per fracture
asset, **so that** detailed destruction is possible.

## US-4.6.1.12 Experience Art-Directed Destruction
**As a** player (P-23), **I want** objects to break in visually interesting patterns, **so that**
destruction looks hand-crafted rather than random.

## US-4.6.2.1 Import Pre-Fractured Meshes from DCC Tools
**As a** designer (P-5), **I want to** import pre-fractured meshes from DCC tools (Houdini, Maya,
Blender), **so that** artists control the destruction aesthetic.

## US-4.6.2.2 Store Fragment Data in Fracture Asset
**As an** engine developer (P-26), **I want** fragment geometry, connectivity graph, and joint break
thresholds stored in a single fracture asset, **so that** loading is efficient.

## US-4.6.2.3 Spawn Fragment Entities on Fracture Activation
**As an** engine developer (P-26), **I want** fragment entities spawned with RigidBody, Collider,
and DebrisLifetime components on fracture, **so that** fragments simulate physically.

## US-4.6.2.4 Connect Adjacent Fragments with Breakable Joints
**As an** engine developer (P-26), **I want** adjacent fragments connected by breakable Joint
entities, **so that** partial fracture is possible.

## US-4.6.2.5 See Castle Walls Break Art-Directed
**As a** player (P-23), **I want** castle walls and towers to break in art-directed patterns, **so
that** siege destruction looks cinematic.

## US-4.6.2.6 Author Hero Object Destruction
**As a** level designer (P-6), **I want to** author art-directed destruction for hero objects like
bridges and towers, **so that** key moments have unique breakage patterns.

## US-4.6.2.7 Test Pre-Fractured Asset Loading
**As an** engine tester (P-27), **I want to** verify pre-fractured assets load correctly and produce
valid fragment entities, **so that** authored destruction works.

## US-4.6.2.8 Configure Per-Fragment Break Thresholds
**As a** designer (P-5), **I want to** set break thresholds per fragment connection, **so that**
some connections are stronger than others.

## US-4.6.2.9 Reference Fracture Asset from Destructible Component
**As a** game developer (P-15), **I want** Destructible components to reference pre-fractured
assets, **so that** art-directed destruction integrates with the ECS.

## US-4.6.2.10 Test Fragment Connectivity Graph
**As an** engine tester (P-27), **I want to** verify the connectivity graph correctly identifies
adjacent fragments, **so that** joint placement is accurate.

## US-4.6.2.11 Design Bridges with Authored Fractures
**As a** level designer (P-6), **I want to** author bridge destruction patterns, **so that**
collapsing bridges create dramatic gameplay moments.

## US-4.6.2.12 See Fragments Fall With Ragdoll-Like Physics
**As a** player (P-23), **I want** fragments to tumble and bounce after separation, **so that**
debris behavior looks physically convincing.

## US-4.6.3.1 Trigger Fracture When Damage Exceeds Threshold
**As a** game developer (P-15), **I want** fracture to trigger when DamageHealth reaches the
Destructible threshold, **so that** objects break after enough damage.

## US-4.6.3.2 Implement FractureActivationSystem
**As an** engine developer (P-26), **I want to** implement FractureActivationSystem that queries
Destructible + DamageHealth entities and triggers fracture, **so that** activation is automatic.

## US-4.6.3.3 Despawn Intact Entity on Fracture
**As an** engine developer (P-26), **I want** the intact entity despawned and fragment entities
spawned on fracture activation, **so that** the transition is clean.

## US-4.6.3.4 Propagate Impact Direction to Fragments
**As an** engine developer (P-26), **I want** impact propagation direction and magnitude to
influence which joints break, **so that** fracture spreads realistically.

## US-4.6.3.5 Destroy Environment with Heavy Weapons
**As a** player (P-23), **I want** heavy weapons to destroy walls and barriers, **so that** I can
create new paths through the environment.

## US-4.6.3.6 Verify Fracture Activation Budget Per Platform
**As an** engine tester (P-27), **I want to** confirm mobile caps at 1 fracture activation per frame
with 8 fragments, **so that** hitches are prevented.

## US-4.6.3.7 Budget Fragment Spawning Per Frame
**As an** engine developer (P-26), **I want** fragment spawning budgeted per frame, **so that**
large-scale battles do not hitch from simultaneous destructions.

## US-4.6.3.8 Design Destructible Barriers
**As a** level designer (P-6), **I want** walls and barriers with fracture activation, **so that**
players can break through to reach new areas.

## US-4.6.3.9 Configure Damage Threshold Per Object
**As a** designer (P-5), **I want to** set the fracture damage threshold per Destructible component,
**so that** different objects have different toughness.

## US-4.6.3.10 Test Staggered Spawning on Mobile
**As an** engine tester (P-27), **I want to** verify fragment spawning is staggered across frames on
mobile, **so that** frame spikes are avoided.

## US-4.6.3.11 Connect Fragments with Breakable Joints
**As an** engine developer (P-26), **I want** adjacent fragments connected by breakable joints that
separate based on impact direction, **so that** partial destruction works.

## US-4.6.3.12 See Walls Crumble Gradually Under Fire
**As a** player (P-23), **I want** walls to crumble gradually under sustained fire, **so that**
destruction feels progressive.

## US-4.6.4.1 Track Cumulative Damage with DamageHealth
**As a** game developer (P-15), **I want** DamageHealth components to track cumulative damage as a
scalar integrity value, **so that** objects weaken over time.

## US-4.6.4.2 Implement DamageAccumulationSystem
**As an** engine developer (P-26), **I want to** implement DamageAccumulationSystem that processes
contact events and subtracts damage based on impulse magnitude, **so that** impact damage is
automatic.

## US-4.6.4.3 Show Visual Cracking Before Full Fracture
**As a** designer (P-5), **I want** visual cracking stages driven by DamageHealth thresholds, **so
that** players see objects weakening before they break.

## US-4.6.4.4 See Objects Crack Before Shattering
**As a** player (P-23), **I want to** see cracks forming on damaged objects before they shatter,
**so that** destruction is telegraphed visually.

## US-4.6.4.5 Replicate DamageHealth Authoritatively
**As an** engine developer (P-26), **I want** server-authoritative DamageHealth replication via ECS
state replication, **so that** clients cannot cheat damage values.

## US-4.6.4.6 Verify Damage Accumulation From Impacts
**As an** engine tester (P-27), **I want to** verify damage accumulates correctly from impact
impulse magnitudes, **so that** damage calculation is accurate.

## US-4.6.4.7 Configure Cracking Thresholds Per Object
**As a** designer (P-5), **I want to** configure visual cracking stage thresholds per Destructible
object, **so that** damage feedback is tunable.

## US-4.6.4.8 Test Damage from Multiple Impact Types
**As an** engine tester (P-27), **I want to** test damage from projectile, explosion, and melee
impacts, **so that** all damage sources contribute correctly.

## US-4.6.4.9 Read Damage State from Gameplay Systems
**As a** game developer (P-15), **I want to** read DamageHealth from gameplay systems, **so that** I
can trigger abilities or effects based on object integrity.

## US-4.6.4.10 Weaken Objects to Set Up Destruction
**As a** player (P-23), **I want to** weaken objects with repeated hits before the final break, **so
that** destruction requires effort and planning.

## US-4.6.4.11 Test Anti-Cheat on DamageHealth
**As an** engine tester (P-27), **I want to** verify clients cannot modify DamageHealth locally to
destroy objects faster, **so that** server authority is enforced.

## US-4.6.4.12 Design Multi-Hit Destructible Puzzles
**As a** level designer (P-6), **I want** objects that require multiple hits to destroy, **so that**
destruction puzzles have depth.

## US-4.6.5.1 Detect Unsupported Fragments After Break
**As an** engine developer (P-26), **I want** StructuralAnalysisSystem to traverse the fragment
connectivity graph and identify fragments without a path to an anchor, **so that** unsupported
pieces fall.

## US-4.6.5.2 Enable Cascading Collapse of Buildings
**As a** game developer (P-15), **I want** destroying a load-bearing fragment to cause cascading
collapse, **so that** structural destruction has chain reactions.

## US-4.6.5.3 See Building Collapse When Foundation Breaks
**As a** player (P-23), **I want** buildings to collapse when their foundation is destroyed, **so
that** structural destruction feels realistic.

## US-4.6.5.4 Implement Graph Connectivity Traversal
**As an** engine developer (P-26), **I want to** implement graph traversal over fragment and Joint
entities to determine structural support, **so that** collapse detection uses only ECS queries.

## US-4.6.5.5 Despawn Joints on Unsupported Fragments
**As an** engine developer (P-26), **I want** Joint entities on unsupported fragments despawned,
**so that** disconnected pieces fall under gravity.

## US-4.6.5.6 Verify Disabled by Default on Mobile
**As an** engine tester (P-27), **I want to** confirm mobile uses pre-baked collapse sequences
instead of runtime analysis, **so that** CPU budget is met.

## US-4.6.5.7 Design Load-Bearing Structures
**As a** level designer (P-6), **I want** structures with identifiable load-bearing fragments, **so
that** strategic destruction gameplay is possible.

## US-4.6.5.8 Configure Anchor Points Per Structure
**As a** designer (P-5), **I want to** designate grounded anchor fragments per structure, **so
that** the analysis system knows which fragments support the whole.

## US-4.6.5.9 Test Collapse with Maximum Fragment Count
**As an** engine tester (P-27), **I want to** test structural collapse with maximum fragment counts
(256 on desktop, 1024 on high-end), **so that** performance at scale is validated.

## US-4.6.5.10 Re-Evaluate Connectivity on Each Break
**As an** engine developer (P-26), **I want** the system to re-evaluate connectivity when any
fragment is destroyed or any Joint breaks, **so that** cascading collapse is progressive.

## US-4.6.5.11 Experience Dramatic Siege Destruction
**As a** player (P-23), **I want** siege weapons to cause entire fortification sections to collapse,
**so that** siege gameplay is spectacular.

## US-4.6.5.12 Verify Simplified Analysis on Switch
**As an** engine tester (P-27), **I want to** verify Switch uses simplified graph traversal with max
32 fragments per structure, **so that** analysis cost is bounded.

## US-4.6.6.1 Set Debris Time-to-Live
**As a** designer (P-5), **I want to** configure debris time-to-live on DebrisLifetime components,
**so that** fragment cleanup is tunable.

## US-4.6.6.2 Implement DebrisLifetimeSystem
**As an** engine developer (P-26), **I want to** implement DebrisLifetimeSystem that decrements
timers and despawns expired debris, **so that** fragment cleanup is automatic.

## US-4.6.6.3 Enforce Maximum Debris Count
**As an** engine developer (P-26), **I want** a maximum debris entity count enforced by despawning
the oldest when the cap is exceeded, **so that** memory and simulation budgets are maintained.

## US-4.6.6.4 See Debris Settle Then Fade
**As a** player (P-23), **I want** debris to settle, then gradually disappear, **so that** the
battlefield is cleaned up over time.

## US-4.6.6.5 Verify Debris Limits Per Platform
**As an** engine tester (P-27), **I want to** confirm mobile caps at 32 debris with 3-second TTL,
**so that** simulation budget is controlled.

## US-4.6.6.6 Transition Debris Through Sleep States
**As an** engine developer (P-26), **I want** debris to transition through active, settling, and
sleeping states using standard sleep logic, **so that** resting debris costs nothing.

## US-4.6.6.7 Configure Debris TTL Per Platform
**As a** designer (P-5), **I want to** set different debris TTL values per platform tier, **so
that** cleanup timing matches device capability.

## US-4.6.6.8 Test Debris Count at Budget Limit
**As an** engine tester (P-27), **I want to** spawn debris up to the platform cap and verify oldest
are despawned first, **so that** budget enforcement works.

## US-4.6.6.9 See Active Battlefield Debris
**As a** player (P-23), **I want** debris to linger long enough to see the aftermath of destruction,
**so that** battles feel impactful.

## US-4.6.6.10 Query All DebrisLifetime Entities
**As a** game developer (P-15), **I want to** query all DebrisLifetime entities, **so that**
gameplay systems can interact with debris (e.g., picking up fragments).

## US-4.6.6.11 Support 2048 Debris on High-End PC
**As an** engine developer (P-26), **I want** high-end PC to support 2048 debris with 30-second TTL,
**so that** extended destruction sequences persist.

## US-4.6.6.12 Test Debris Sleep Transition
**As an** engine tester (P-27), **I want to** verify debris transitions to sleeping state after
settling, **so that** resting debris has zero simulation cost.

## US-4.6.7.1 Recycle Despawned Fragment Entities
**As an** engine developer (P-26), **I want** despawned fragment entities returned to a pool and
reused by resetting components, **so that** allocation churn is eliminated.

## US-4.6.7.2 Implement DebrisLodSystem
**As an** engine developer (P-26), **I want to** implement DebrisLodSystem that reduces shape
complexity and simulation fidelity for distant fragments, **so that** cost scales with distance.

## US-4.6.7.3 Convert Distant Debris to Visual Particles
**As an** engine developer (P-26), **I want** debris beyond a configurable distance to have
RigidBody and Collider removed and become visual-only particles, **so that** simulation cost drops
to zero.

## US-4.6.7.4 See Nearby Debris with Full Detail
**As a** player (P-23), **I want** nearby debris to have full physics and collision, **so that**
close-up destruction looks detailed.

## US-4.6.7.5 Configure LOD Distances Per Platform
**As a** designer (P-5), **I want to** set LOD transition and particle fallback distances per
platform, **so that** visual quality scales with capability.

## US-4.6.7.6 Verify Pool Size Per Platform
**As an** engine tester (P-27), **I want to** confirm mobile pool size is 32 with LOD at 10m and
particle at 20m, **so that** platform budgets are respected.

## US-4.6.7.7 Eliminate Allocation Churn During Destruction
**As an** engine developer (P-26), **I want** pooled entities reset with new fragment data instead
of allocated fresh, **so that** destruction events cause no GC pressure.

## US-4.6.7.8 Test LOD Transition Smoothness
**As an** engine tester (P-27), **I want to** verify debris LOD transitions are smooth without
visible popping, **so that** quality transitions are seamless.

## US-4.6.7.9 See Distant Debris as Particles
**As a** player (P-23), **I want** distant debris to appear as lightweight particles, **so that**
the battlefield has visual presence without performance impact.

## US-4.6.7.10 Design Levels with Heavy Destruction
**As a** level designer (P-6), **I want** debris pooling and LOD to handle heavy destruction scenes,
**so that** I can design intense battles without performance issues.

## US-4.6.7.11 Support Pool Size 2048 on High-End PC
**As an** engine developer (P-26), **I want** high-end PC to support pool size 2048 with extended
LOD distances, **so that** massive destruction sequences work.

## US-4.6.7.12 Profile Pooling vs Fresh Allocation
**As an** engine tester (P-27), **I want to** profile pooled entity reuse vs fresh allocation, **so
that** I can verify pooling eliminates allocation overhead.
