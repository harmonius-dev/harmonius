# Procedural Animation Test Cases

Companion test cases for [procedural.md](procedural.md).

## Unit Tests

### TC-9.3.1.1 Two-Bone IK Reach Target

| # | Requirement |
|---|-------------|
| 1 | R-9.3.1     |
| 2 | R-9.3.1     |

1. **#1** — Target within reach, chain lengths (1.0, 1.0), target at (1.5, 0, 0)
   - **Expected:** End-effector within 0.01 units of target
2. **#2** — Target at exact reach limit (2.0, 0, 0)
   - **Expected:** End-effector within 0.01 units of target

### TC-9.3.1.2 Two-Bone IK Pole Vector

| # | Requirement |
|---|-------------|
| 1 | R-9.3.1     |
| 2 | R-9.3.1     |

1. **#1** — Pole vector at (0, 0, 1), target at (1.5, 0, 0)
   - **Expected:** Elbow displaced toward +Z
2. **#2** — Pole vector rotated 90 degrees to (0, 0, -1)
   - **Expected:** Elbow displaced toward -Z

### TC-9.3.1.3 Two-Bone IK Unreachable

| # | Requirement |
|---|-------------|
| 1 | R-9.3.1     |
| 2 | R-9.3.1     |

1. **#1** — Chain lengths (1.0, 1.0), target at (5.0, 0, 0)
   - **Expected:** Chain fully extended toward target, no NaN in joints
2. **#2** — Chain lengths (1.0, 1.0), target at (0.01, 0, 0)
   - **Expected:** Chain folds inward, no joint explosion

### TC-9.3.1.4 Two-Bone IK Weight Zero

| # | Requirement |
|---|-------------|
| 1 | R-9.3.1     |
| 2 | R-9.3.1     |

1. **#1** — weight=0.0, any target position
   - **Expected:** Output pose identical to input pose (bitwise)
2. **#2** — weight=0.5, target at (1.5, 0, 0)
   - **Expected:** Output pose halfway between input and IK solution

### TC-9.3.2.1 CCD IK 6-Bone Convergence

| # | Requirement |
|---|-------------|
| 1 | R-9.3.2     |
| 2 | R-9.3.2     |

1. **#1** — 6-bone chain, reachable target, 10 iterations
   - **Expected:** End-effector within 0.05 units of target
2. **#2** — 6-bone chain, reachable target, 3 iterations
   - **Expected:** End-effector closer to target than initial pose

### TC-9.3.2.2 CCD IK Angular Limits

| # | Requirement |
|---|-------------|
| 1 | R-9.3.2     |
| 2 | R-9.3.2     |

1. **#1** — 6-bone chain, 30-degree per-joint limit, 10 iterations
   - **Expected:** No joint rotation exceeds 30 degrees
2. **#2** — 6-bone chain, 5-degree per-joint limit, target at edge of reach
   - **Expected:** All joints within 5-degree limit

### TC-9.3.2.3 CCD IK Unreachable

| # | Requirement |
|---|-------------|
| 1 | R-9.3.2     |

1. **#1** — 6-bone chain, target outside reach, 30-degree limits
   - **Expected:** Chain extends toward target, no joint exceeds limit

### TC-9.3.3.1 FABRIK 8-Bone

| # | Requirement |
|---|-------------|
| 1 | R-9.3.3     |
| 2 | R-9.3.3     |

1. **#1** — 8-bone chain, reachable target, 6 iterations
   - **Expected:** End-effector within 0.05 units of target
2. **#2** — 8-bone chain, target at half reach
   - **Expected:** End-effector within 0.05 units, chain naturally curved

### TC-9.3.3.2 FABRIK Multi-End-Effector

| # | Requirement |
|---|-------------|
| 1 | R-9.3.3     |
| 2 | R-9.3.3     |

1. **#1** — 8-leg spider skeleton, 8 ground targets
   - **Expected:** All 8 leg tips within 0.05 units of their targets
2. **#2** — 4-leg quadruped, 4 targets on uneven terrain
   - **Expected:** All 4 feet within 0.05 units of targets

### TC-9.3.3.3 FABRIK Priority

| # | Requirement |
|---|-------------|
| 1 | R-9.3.3     |

1. **#1** — Two effectors sharing joint, priority 1.0 vs 0.5
   - **Expected:** Higher-priority effector within 0.05, lower may exceed

### TC-9.3.4.1 Ragdoll Blend In

| # | Requirement |
|---|-------------|
| 1 | R-9.3.4     |
| 2 | R-9.3.4     |

1. **#1** — Activate ragdoll, blend_in_duration=0.5s, dt=1/60
   - **Expected:** blend_weight=1.0 after 30 frames
2. **#2** — Activate ragdoll, blend_in_duration=0.0s
   - **Expected:** blend_weight=1.0 immediately (frame 1)

### TC-9.3.4.2 Ragdoll Recovery

| # | Requirement |
|---|-------------|
| 1 | R-9.3.4     |
| 2 | R-9.3.4     |

1. **#1** — Trigger recovery, recovery_duration=1.0s, dt=1/60
   - **Expected:** blend_weight=0.0 after 60 frames
2. **#2** — Trigger recovery, recovery_duration=0.3s
   - **Expected:** blend_weight=0.0 after 18 frames

### TC-9.3.4.3 Ragdoll Partial Mask

| # | Requirement |
|---|-------------|
| 1 | R-9.3.4     |
| 2 | R-9.3.4     |

1. **#1** — Mask=upper_body only, ragdoll active
   - **Expected:** Lower body bones: output == animation pose (delta < 0.001)
2. **#2** — Mask=upper_body only, ragdoll active
   - **Expected:** Upper body bones: output == ragdoll pose

### TC-9.3.4.4 Ragdoll No Discontinuity

| # | Requirement |
|---|-------------|
| 1 | R-9.3.4     |

1. **#1** — Blend 0.0 to 1.0 over 0.5s at dt=1/60
   - **Expected:** Max frame-to-frame bone position delta < 0.1 units

### TC-9.3.5.1 Look-At 45 Degrees

| # | Requirement |
|---|-------------|
| 1 | R-9.3.5     |
| 2 | R-9.3.5     |

1. **#1** — Target 45 degrees right of forward
   - **Expected:** Head yaw within 1 degree of 45
2. **#2** — Target 45 degrees left of forward
   - **Expected:** Head yaw within 1 degree of -45

### TC-9.3.5.2 Look-At Clamp

| # | Requirement |
|---|-------------|
| 1 | R-9.3.5     |
| 2 | R-9.3.5     |

1. **#1** — Target 120 degrees right, max_yaw=90 degrees
   - **Expected:** Head yaw clamped at exactly 90 degrees
2. **#2** — Target 200 degrees right, max_yaw=90 degrees
   - **Expected:** Head yaw clamped at 90 degrees

### TC-9.3.5.3 Aim Alignment

| # | Requirement |
|---|-------------|
| 1 | R-9.3.5     |

1. **#1** — Aim target at (10, 5, 0), weapon forward=+X
   - **Expected:** Weapon direction within 2 degrees of target direction

### TC-9.3.5.4 Aim Preserves Lower Body

| # | Requirement |
|---|-------------|
| 1 | R-9.3.5     |

1. **#1** — Aim constraint active on upper body
   - **Expected:** Lower body bones differ from base pose by < 0.001 units

### TC-9.3.7.1 Foot Placement Stairs

| # | Requirement |
|---|-------------|
| 1 | R-9.3.7     |
| 2 | R-9.3.7     |

1. **#1** — Walk across 20 cm stair steps, 120 frames
   - **Expected:** No foot penetration > 1 cm, no foot float > 2 cm
2. **#2** — Walk across 10 cm stair steps
   - **Expected:** No penetration > 1 cm

### TC-9.3.7.2 Foot Placement Slope

| # | Requirement |
|---|-------------|
| 1 | R-9.3.7     |
| 2 | R-9.3.7     |

1. **#1** — Walk 30-degree slope uphill, 120 frames
   - **Expected:** Foot contacts match slope surface within 2 cm
2. **#2** — Walk 30-degree slope downhill
   - **Expected:** Stride length adapts (shorter than flat ground)

### TC-9.3.7.3 Foot Placement Disabled

| # | Requirement |
|---|-------------|
| 1 | R-9.3.7     |

1. **#1** — foot_placement_enabled=false, walk 60 frames
   - **Expected:** Zero raycasts issued to spatial index

### TC-9.3.8.1 Gait Biped Walk

| # | Requirement |
|---|-------------|
| 1 | R-9.3.8     |
| 2 | R-9.3.8     |

1. **#1** — Biped at walk speed (1.5 m/s)
   - **Expected:** Alternating foot phase pattern: L-R-L-R
2. **#2** — Biped at walk speed
   - **Expected:** Each foot phase duration within 10% of gait period/2

### TC-9.3.8.2 Gait Quadruped Trot-Gallop

| # | Requirement |
|---|-------------|
| 1 | R-9.3.8     |
| 2 | R-9.3.8     |

1. **#1** — Quadruped, speed increases from 2 to 8 m/s
   - **Expected:** Gait transitions from trot to gallop
2. **#2** — Quadruped at trot speed
   - **Expected:** Diagonal pairs move in sync

### TC-9.3.8.3 Gait Hexapod Tripod

| # | Requirement |
|---|-------------|
| 1 | R-9.3.8     |

1. **#1** — Hexapod at walk speed
   - **Expected:** Tripod gait: 3 legs down, 3 legs up, alternating

### TC-9.3.8.4 Gait All ECS

| # | Requirement |
|---|-------------|
| 1 | R-9.3.8     |
| 2 | R-9.3.8     |

1. **#1** — Query LocomotionProfile from ECS world
   - **Expected:** Component exists and is queryable
2. **#2** — Query GaitState and FootGroup from ECS world
   - **Expected:** Both components exist and are queryable

### TC-9.3.9.1 Physics Balance Upright

| # | Requirement |
|---|-------------|
| 1 | R-9.3.9     |
| 2 | R-9.3.9     |

1. **#1** — Level ground, PID active, 120 frames
   - **Expected:** Torso angle within 2 degrees of upright
2. **#2** — Level ground, PID gains=(100, 10, 5)
   - **Expected:** Stable balance, no oscillation

### TC-9.3.9.2 Physics Stumble Recovery

| # | Requirement |
|---|-------------|
| 1 | R-9.3.9     |

1. **#1** — Lateral impulse of 500 N*s applied
   - **Expected:** Stumble detected within 3 frames, recovery within 60 frames

### TC-9.3.9.3 Physics Slope Lean

| # | Requirement |
|---|-------------|
| 1 | R-9.3.9     |

1. **#1** — 20-degree slope, walking uphill
   - **Expected:** Forward lean angle > 5 degrees

### TC-9.3.10.1 Attach Socket

| # | Requirement |
|---|-------------|
| 1 | R-9.3.10    |
| 2 | R-9.3.10    |

1. **#1** — Attach entity to hand socket, 60 frames
   - **Expected:** Attached entity position within 0.01 units of socket each frame
2. **#2** — Detach entity from socket
   - **Expected:** Entity position no longer follows socket

### TC-9.3.10.2 Dismember Spawns Ragdoll

| # | Requirement |
|---|-------------|
| 1 | R-9.3.10    |
| 2 | R-9.3.10    |

1. **#1** — Sever quadruped front-right leg
   - **Expected:** Detached entity has RagdollBlend component
2. **#2** — Sever quadruped front-right leg
   - **Expected:** Parent entity missing front-right bone subtree

### TC-9.3.10.3 Dismember Gait Adapt

| # | Requirement |
|---|-------------|
| 1 | R-9.3.10    |
| 2 | R-9.3.10    |

1. **#1** — Sever quadruped front-right leg
   - **Expected:** Gait switches to three-legged pattern
2. **#2** — Sever hexapod leg
   - **Expected:** Gait adapts to 5-leg pattern

### TC-9.3.10.4 Dismember ECS Commands

| # | Requirement |
|---|-------------|
| 1 | R-9.3.10    |

1. **#1** — Sever limb, trace command buffer
   - **Expected:** All dismemberment operations use ECS commands (not direct mutation)

### TC-9.3.11.1 Debug Vis Foot Targets

| # | Requirement |
|---|-------------|
| 1 | R-9.3.11    |
| 2 | R-9.3.11    |

1. **#1** — foot_debug_vis=true, walk 30 frames
   - **Expected:** DebugDraw contains foot target markers each frame
2. **#2** — foot_debug_vis=false
   - **Expected:** Zero foot markers in DebugDraw

### TC-9.3.11.2 Debug Vis IK Chains

| # | Requirement |
|---|-------------|
| 1 | R-9.3.11    |

1. **#1** — ik_debug_vis=true, IK active
   - **Expected:** Bone axes rendered for each IK chain joint

### TC-9.3.11.3 Debug Vis Per Entity

| # | Requirement |
|---|-------------|
| 1 | R-9.3.11    |

1. **#1** — Toggle debug vis off for entity A, on for entity B
   - **Expected:** Entity A: zero overlays, Entity B: overlays present

### TC-9.3.6.1 Motion Matching Best Pose

| # | Requirement |
|---|-------------|
| 1 | R-9.3.6     |
| 2 | R-9.3.6     |

1. **#1** — Pose database with 1,000 poses, query trajectory (3 future samples), search top-1
   - **Expected:** Returned pose index minimizes the weighted trajectory + pose-vector distance
     across all 1,000 candidates
2. **#2** — Same database, weighted feature vector with velocity weight 0.5, position weight 1.0
   - **Expected:** Returned index matches an offline brute-force search within 1e-4 distance

### TC-9.5.1.1 GPU Cloth PBD Distance Constraint

| # | Requirement |
|---|-------------|
| 1 | R-9.5.1     |
| 2 | R-9.5.1     |

1. **#1** — 32×32 particle cloth pinned on top row, 60 substeps of gravity, distance constraints
   only
   - **Expected:** Vertical edge lengths stay within 1% of rest length after step; bottom row hangs
     under gravity
2. **#2** — Same cloth, 100 steps with bending constraints
   - **Expected:** No constraint residual > 1e-3 after last step; no NaN in particle positions

### TC-9.5.2.1 Strand Hair Guide Curve Sim

| # | Requirement |
|---|-------------|
| 1 | R-9.5.2     |
| 2 | R-9.5.2     |

1. **#1** — Guide curve of 16 points, stretch + bending constraints, 60 steps under gravity
   - **Expected:** Segment lengths stay within 2% of rest; tip deflection < 0.5 × length
2. **#2** — Same curve with wind vector (1, 0, 0) of 5 m/s
   - **Expected:** Steady-state tip displacement biased in +X direction

### TC-9.5.3.1 Card Hair Anisotropic Shading

| # | Requirement |
|---|-------------|
| 1 | R-9.5.3     |
| 2 | R-9.5.3     |

1. **#1** — Hair cards rendered with anisotropic specular (Kajiya-Kay), alpha-blended
   - **Expected:** Per-pixel specular direction perpendicular to tangent; no z-fight; alpha sorts
     correctly
2. **#2** — Hair cards rendered in alpha-test mode
   - **Expected:** No semi-transparent fringes; card edges exact

### TC-9.5.4.1 Hair LOD Transitions

| # | Requirement |
|---|-------------|
| 1 | R-9.5.4     |
| 2 | R-9.5.4     |
| 3 | R-9.5.4     |

1. **#1** — LOD 0 strand representation, camera 2 m
   - **Expected:** Renders strands, strand count equals source
2. **#2** — Camera 10 m — transition to cluster
   - **Expected:** Active LOD == `Cluster`; strand count reduced to cluster count
3. **#3** — Camera 50 m — transition to shell
   - **Expected:** Active LOD == `Shell`; shell mesh drawn

### TC-9.5.5.1 Cloth-Body Capsule Collision

| # | Requirement |
|---|-------------|
| 1 | R-9.5.5     |
| 2 | R-9.5.5     |

1. **#1** — Cloth patch falling onto a capsule proxy of the body
   - **Expected:** No particle penetrates capsule by more than 1% of capsule radius after solve
2. **#2** — Cloth patch draped over a convex hull proxy
   - **Expected:** Contact points lie on hull surface; zero inside-hull particles

### TC-9.5.6.1 Hair Wind Field Response

| # | Requirement |
|---|-------------|
| 1 | R-9.5.6     |
| 2 | R-9.5.6     |

1. **#1** — Shared wind field sample (2, 0, 0) m/s at hair root
   - **Expected:** Average tip displacement along +X; magnitude monotonic with wind strength
2. **#2** — Turbulent wind (sinusoidal in time)
   - **Expected:** Tip oscillates in phase with wind; no resonance blow-up

### TC-9.6.1.1 First-Person Head Bob Spring

| # | Requirement |
|---|-------------|
| 1 | R-9.6.1     |
| 2 | R-9.6.1     |

1. **#1** — Walk at 1.5 m/s, `head_bob.amplitude = 3 cm`, 60 frames
   - **Expected:** Camera vertical position oscillates with amplitude within ±5% of 3 cm
2. **#2** — Land from 3 m drop
   - **Expected:** Landing spring triggers; camera Y dips then recovers within 0.3 s

### TC-9.6.2.1 Weapon Sway and Sprint Tilt

| # | Requirement |
|---|-------------|
| 1 | R-9.6.2     |
| 2 | R-9.6.2     |

1. **#1** — Mouse delta (10, 0) applied, weapon sway stiffness = 30, damping = 4
   - **Expected:** Viewmodel rotates opposite to mouse delta; oscillation damps within 0.5 s
2. **#2** — Sprint state active, sprint tilt angle = 15 degrees
   - **Expected:** Viewmodel tilts 15 ± 1 degrees on the Z axis while sprinting

### TC-9.6.3.1 Recoil Pattern Spring

| # | Requirement |
|---|-------------|
| 1 | R-9.6.3     |
| 2 | R-9.6.3     |

1. **#1** — Fire 10 rounds with recoil pattern `[(0, 1), (0.5, 1.1), ...]`
   - **Expected:** Viewmodel kick matches pattern samples within 2%; spring returns toward rest
     between shots
2. **#2** — ADS active
   - **Expected:** Recoil magnitude scaled by `ads.recoil_scale`; camera sway minimized

### TC-9.6.4.1 Equip Inspect Dual Wield

| # | Requirement |
|---|-------------|
| 1 | R-9.6.4     |
| 2 | R-9.6.4     |

1. **#1** — Equip sequence triggered
   - **Expected:** Viewmodel animates from `stow_pose` to `idle_pose` over configured duration; no
     discontinuity
2. **#2** — Dual-wield, independent per-hand spring systems
   - **Expected:** Left-hand and right-hand springs integrate independently; no cross-talk in
     positions or forces

## Integration Tests

### TC-9.3.1.I1 Pipeline Order

| # | Requirement              |
|---|--------------------------|
| 1 | R-9.3.1 through R-9.3.11 |

1. **#1** — Full procedural pipeline, 1 frame
   - **Expected:** Systems execute: look-at, foot, IK, ragdoll, secondary (in order)

### TC-9.3.1.I2 500 Two-Bone GPU

| # | Requirement |
|---|-------------|
| 1 | R-9.3.1     |

1. **#1** — 500 two-bone IK chains in single GPU dispatch
   - **Expected:** All end-effectors within 0.01 units of targets

### TC-9.3.1.I3 IK After State Machine

| # | Requirement |
|---|-------------|
| 1 | R-9.3.1     |

1. **#1** — Walk animation + hand IK target
   - **Expected:** IK modifies the blended animation pose, not raw clip data

### TC-9.3.7.I1 Foot Placement Batch Raycast

| # | Requirement |
|---|-------------|
| 1 | R-9.3.7     |

1. **#1** — 100 characters with foot placement active
   - **Expected:** Single batch raycast to shared BVH (not 100 individual queries)

### TC-9.3.4.I1 Ragdoll Physics Integration

| # | Requirement |
|---|-------------|
| 1 | R-9.3.4     |

1. **#1** — Full ragdoll with physics step, 60 frames
   - **Expected:** All bones respond to gravity (Y decreasing)

### TC-9.3.8.I1 Locomotion All Topologies

| # | Requirement |
|---|-------------|
| 1 | R-9.3.8     |

1. **#1** — Biped, quadruped, hexapod on same terrain
   - **Expected:** Each uses correct gait pattern for its topology

### TC-9.3.9.I1 Physics to Animated Transition

| # | Requirement |
|---|-------------|
| 1 | R-9.3.9     |

1. **#1** — Transition physics->animated over 0.3s
   - **Expected:** Max frame-to-frame bone delta < 0.1 units (no visible pop)

### TC-9.3.10.I1 Dismember Runtime Full

| # | Requirement |
|---|-------------|
| 1 | R-9.3.10    |

1. **#1** — Sever wing at runtime
   - **Expected:** Ragdoll spawns for detached part, locomotion adapts

### TC-9.3.6.I1 Motion Matching Continuation

| # | Requirement |
|---|-------------|
| 1 | US-9.3.6.1  |
| 2 | US-9.3.6.2  |

1. **#1** — As an engine developer, play a character with a 1,000-pose motion-matching database;
   change desired trajectory mid-stride
   - **Expected:** Next selected clip is a pose whose outgoing trajectory best matches the new
     desired trajectory; feature-vector distance minimal across the DB
2. **#2** — As a character animator, tune weighted feature vector (velocity, bone positions); re-run
   match
   - **Expected:** Selected pose shifts in response to weights, no crashes, result stable across 5
     ticks

### TC-9.3.9.I2 Physics Locomotion Stumble

| # | Requirement |
|---|-------------|
| 1 | US-9.3.9.1  |
| 2 | US-9.3.9.2  |

1. **#1** — As an engine developer, apply PID balance to a biped on level ground for 120 frames
   - **Expected:** Torso upright within 2 degrees; no oscillation exceeding 1 degree
2. **#2** — Apply a 500 N·s lateral impulse to the torso
   - **Expected:** Stumble detected within 3 frames; PID drives recovery within 60 frames

### TC-9.3.10.I2 Dismember Locomotion Adapts

| # | Requirement |
|---|-------------|
| 1 | US-9.3.10.1 |
| 2 | US-9.3.10.3 |

1. **#1** — As a game developer, sever a quadruped's front-right leg at runtime
   - **Expected:** Detached segment becomes a ragdoll entity with physics; parent skeleton loses the
     subtree via ECS commands
2. **#2** — Continue locomotion after severance
   - **Expected:** Gait switches to three-legged pattern; character continues forward without
     falling

### TC-9.3.11.I1 Debug Vis Toggles

| # | Requirement |
|---|-------------|
| 1 | US-9.3.11.1 |
| 2 | US-9.3.11.3 |

1. **#1** — As a game developer, enable foot-target debug vis on entity A, disable on entity B
   - **Expected:** DebugDraw contains entity A markers; no markers for entity B
2. **#2** — Ship build with debug vis compiled out
   - **Expected:** `locomotion_diagnostics_system` and `LocomotionDebugVis` absent from binary;
     runtime toggle is a no-op

### TC-9.5.1.I1 Cloth Garment End to End

| # | Requirement |
|---|-------------|
| 1 | US-9.5.1.1  |
| 2 | US-9.5.1.2  |
| 3 | US-9.5.1.3  |

1. **#1** — As an engine developer, attach a GPU PBD cloth garment to a moving character for 300
   frames
   - **Expected:** Cloth follows character, constraint residual < 1e-3, no particle explosion
2. **#2** — Character strikes a pose introducing self-intersection
   - **Expected:** Self-collision constraints resolve intersections; no flip-through
3. **#3** — Configure per-tier iteration count in `PlatformTier::Low`
   - **Expected:** Lower iteration count applied; cloth still stable

### TC-9.5.2.I1 Strand Hair Simulation

| # | Requirement |
|---|-------------|
| 1 | US-9.5.2.1  |
| 2 | US-9.5.2.2  |
| 3 | US-9.5.2.3  |

1. **#1** — As an engine developer, simulate 50 strand-hair guide curves with stretch and bending
   for 120 frames
   - **Expected:** Strands remain within 2% of rest length; no NaN; tips hang naturally
2. **#2** — As a character animator, tune stiffness and damping
   - **Expected:** Higher stiffness reduces tip deflection; changes take effect without reload
3. **#3** — As a tools user, inspect per-strand debug overlay
   - **Expected:** Overlay draws guide curves; toggles off in ship builds

### TC-9.5.3.I1 Card Hair Rendering Pipeline

| # | Requirement |
|---|-------------|
| 1 | US-9.5.3.1  |
| 2 | US-9.5.3.2  |

1. **#1** — As a character artist, render a character with card-hair asset using anisotropic
   specular
   - **Expected:** Kajiya-Kay specular direction perpendicular to card tangent; no sorting artifacts
2. **#2** — Switch the same asset to alpha-test mode
   - **Expected:** No semi-transparent fringes; card edges sharp

### TC-9.5.4.I1 Hair LOD Cascade

| # | Requirement |
|---|-------------|
| 1 | US-9.5.4.1  |
| 2 | US-9.5.4.2  |

1. **#1** — As an engine developer, push the camera from 2 m to 80 m past a hair asset
   - **Expected:** LOD transitions strand → cluster → card → shell at configured distances; no
     visible pop
2. **#2** — As a character artist, author per-LOD overrides
   - **Expected:** Overrides honored; per-LOD shading parameters take effect at matching distances

### TC-9.5.5.I1 Cloth Body Collision End to End

| # | Requirement |
|---|-------------|
| 1 | US-9.5.5.1  |
| 2 | US-9.5.5.2  |

1. **#1** — As an engine developer, simulate cloth against body capsule and convex hull proxies for
   240 frames
   - **Expected:** Particles never penetrate proxies beyond 1% tolerance; contact points on proxy
     surfaces
2. **#2** — As a character animator, adjust collision friction
   - **Expected:** Cloth slides faster at low friction, grips at high friction; change takes effect
     without rebuild

### TC-9.5.6.I1 Hair Wind Field Integration

| # | Requirement |
|---|-------------|
| 1 | US-9.5.6.1  |
| 2 | US-9.5.6.2  |
| 3 | US-9.5.6.3  |

1. **#1** — As an engine developer, apply a shared wind field of 5 m/s along +X to strand + card
   hair
   - **Expected:** Both representations bend in +X; magnitude scales with wind strength
2. **#2** — As a VFX artist, toggle turbulent mode
   - **Expected:** Hair responds with non-uniform sub-oscillation; no blow-up
3. **#3** — As a tools user, inspect wind-field debug overlay
   - **Expected:** Overlay shows direction and magnitude at hair sample points

### TC-9.6.1.I1 First Person Camera Full

| # | Requirement |
|---|-------------|
| 1 | US-9.6.1.1  |
| 2 | US-9.6.1.2  |
| 3 | US-9.6.1.3  |

1. **#1** — As a player, move at 1.5 m/s for 5 s
   - **Expected:** Head bob oscillates at configured amplitude; camera FOV matches viewmodel FOV
     setting
2. **#2** — Land from 3 m drop
   - **Expected:** Landing spring triggered; lean/peek dampens within 0.5 s
3. **#3** — As an engine developer, toggle separate viewmodel FOV
   - **Expected:** Viewmodel FOV differs from world FOV by configured delta, without clipping

### TC-9.6.2.I1 Weapon Sway Bob Tilt

| # | Requirement |
|---|-------------|
| 1 | US-9.6.2.1  |
| 2 | US-9.6.2.2  |
| 3 | US-9.6.2.3  |

1. **#1** — As a player, pan camera while walking
   - **Expected:** Viewmodel sways opposite to pan, bobs with locomotion, damps within 0.5 s
2. **#2** — Sprint for 2 s
   - **Expected:** Sprint tilt angle applied; returns to neutral on sprint end
3. **#3** — As a weapon designer, change per-weapon spring stiffness
   - **Expected:** Sway profile shifts without relaunching

### TC-9.6.3.I1 Recoil and ADS

| # | Requirement |
|---|-------------|
| 1 | US-9.6.3.1  |
| 2 | US-9.6.3.3  |

1. **#1** — As a player, fire 30 rounds without ADS
   - **Expected:** Recoil follows weapon pattern data; spring recenters between bursts
2. **#2** — Enter ADS and fire same 30 rounds
   - **Expected:** Recoil magnitude scaled down by `ads.recoil_scale`; reticle stays closer to
     center

### TC-9.6.4.I1 Equip Holster Dual Wield

| # | Requirement |
|---|-------------|
| 1 | US-9.6.4.1  |
| 2 | US-9.6.4.4  |

1. **#1** — As a player, equip then holster a weapon
   - **Expected:** Equip/holster sequences play with smooth spring transitions; no clipping
2. **#2** — Dual wield two pistols, inspect one
   - **Expected:** Inspection plays on the active hand only; other hand spring remains undisturbed

## Benchmarks

### TC-9.3.1.B1 Two-Bone IK GPU

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 two-bone chains, single GPU dispatch | GPU time | < 0.5 ms | US-9.3.1.2 |

### TC-9.3.2.B1 CCD IK GPU

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100 chains, 8 iterations, GPU dispatch | GPU time | < 1.0 ms | US-9.3.2.1 |

### TC-9.3.3.B1 FABRIK

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 50 chains, 6 iterations | CPU time | < 0.8 ms | US-9.3.3.1 |

### TC-9.3.7.B1 Foot Placement Raycasts

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100 characters, batch raycast | CPU time | < 0.3 ms | US-9.3.7.2 |

### TC-9.3.4.B1 Ragdoll Blend

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 50 characters with ragdoll blend | CPU time | < 0.2 ms | US-9.3.4.1 |

### TC-9.3.5.B1 Look-At and Aim

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 200 look-at + aim constraints | CPU time | < 0.2 ms | US-9.3.5.1 |

### TC-9.3.8.B1 Locomotion System

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100 creatures with procedural locomotion | CPU time | < 0.5 ms | US-9.3.8.1 |

### TC-9.3.11.B1 Full Pipeline

| # | Metric         | Target   | Requirement              |
|---|----------------|----------|--------------------------|
| 1 | Total CPU time | < 2.0 ms | R-9.3.1 through R-9.3.11 |

1. **1** — 100 characters, all procedural features active

## Shipping Build Verification

### TC-9.3.11.S1 Debug Stripped

| # | Requirement |
|---|-------------|
| 1 | R-9.3.11    |
| 2 | R-9.3.11    |

1. **#1** — Compile shipping build, inspect binary
   - **Expected:** `locomotion_diagnostics_system` absent from binary
2. **#2** — Compile shipping build, inspect binary
   - **Expected:** `LocomotionDebugVis` and `LocomotionMetrics` types absent
