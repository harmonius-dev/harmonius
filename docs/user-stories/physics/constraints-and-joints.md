# User Stories -- 4.3 Constraints & Joints

## Joint Types

| ID         | Persona                 |
|------------|-------------------------|
| US-4.3.1.1 | game developer (P-15)   |
| US-4.3.1.2 | game designer (P-5)     |
| US-4.3.2.1 | game developer (P-15)   |
| US-4.3.2.2 | engine developer (P-26) |

1. **US-4.3.1.1** -- **As a** game developer (P-15), **I want** revolute, prismatic, fixed, and
   distance joint types as ECS entities, **so that** I connect bodies with standard mechanical
   constraints.
2. **US-4.3.1.2** -- **As a** game designer (P-5), **I want** to configure joint type and connected
   bodies in the visual editor, **so that** joint setup requires no code.
3. **US-4.3.2.1** -- **As a** game developer (P-15), **I want** spring, cone-twist, and generic
   6-DOF joint variants, **so that** ragdolls, turrets, and complex mechanisms are possible.
4. **US-4.3.2.2** -- **As a** engine developer (P-26), **I want** per-axis locking, limiting, and
   freeing via a Dof6Config component, **so that** each degree of freedom is individually
   controllable.

## Motors and Limits

| ID         | Persona                 |
|------------|-------------------------|
| US-4.3.3.1 | game developer (P-15)   |
| US-4.3.3.2 | game designer (P-5)     |
| US-4.3.4.1 | game developer (P-15)   |
| US-4.3.4.2 | engine tester (P-27)    |

1. **US-4.3.3.1** -- **As a** game developer (P-15), **I want** optional JointMotor and JointLimits
   components on joint entities, **so that** I add driven motion and angular bounds to any joint.
2. **US-4.3.3.2** -- **As a** game designer (P-5), **I want** motor target velocity, max force, and
   limit softness exposed as tunable parameters, **so that** I adjust joint behavior without code.
3. **US-4.3.4.1** -- **As a** game developer (P-15), **I want** breakable joints with configurable
   force and torque thresholds, **so that** joints fracture when stress exceeds limits.
4. **US-4.3.4.2** -- **As a** engine tester (P-27), **I want** to verify JointBroken events fire
   with correct force magnitude, **so that** break detection is accurate.

## Ragdolls and Chains

| ID         | Persona                 |
|------------|-------------------------|
| US-4.3.5.1 | game developer (P-15)   |
| US-4.3.5.2 | game designer (P-5)     |
| US-4.3.6.1 | game developer (P-15)   |
| US-4.3.6.2 | engine tester (P-27)    |

1. **US-4.3.5.1** -- **As a** game developer (P-15), **I want** ragdoll configurations from skeleton
   assets with anatomical joint limits, **so that** characters transition to physics-driven ragdoll
   on death.
2. **US-4.3.5.2** -- **As a** game designer (P-5), **I want** configurable ragdoll bone count and
   limits per platform tier, **so that** mobile uses simplified ragdolls.
3. **US-4.3.6.1** -- **As a** game developer (P-15), **I want** ropes and chains as sequences of
   linked joint entities, **so that** cables and hanging objects simulate physically.
4. **US-4.3.6.2** -- **As a** engine tester (P-27), **I want** to verify chain segment count and
   stiffness scale per platform tier, **so that** performance budgets are respected.

## Solvers

| ID         | Persona                 |
|------------|-------------------------|
| US-4.3.7.1 | engine developer (P-26) |
| US-4.3.7.2 | game designer (P-5)     |
| US-4.3.7.3 | engine tester (P-27)    |

1. **US-4.3.7.1** -- **As a** engine developer (P-26), **I want** sequential impulse and TGS solvers
   selectable via SolverConfig, **so that** the solver matches fidelity requirements per project.
2. **US-4.3.7.2** -- **As a** game designer (P-5), **I want** iteration count configurable per
   platform tier, **so that** mobile uses fewer iterations while desktop achieves high-fidelity
   stacking.
3. **US-4.3.7.3** -- **As a** engine tester (P-27), **I want** to verify solver determinism given
   identical entity ordering, **so that** server-authoritative prediction is consistent.

## Limb Severance

| ID         | Persona                 |
|------------|-------------------------|
| US-4.3.8.1 | game developer (P-15)   |
| US-4.3.8.2 | game designer (P-5)     |
| US-4.3.9.1 | game developer (P-15)   |
| US-4.3.9.2 | engine tester (P-27)    |

1. **US-4.3.8.1** -- **As a** game developer (P-15), **I want** joint connections destroyed when
   accumulated damage exceeds a severance threshold, **so that** limbs detach as independent ragdoll
   entities.
2. **US-4.3.8.2** -- **As a** game designer (P-5), **I want** per-joint HP tracked in a LimbHealth
   component with progressive damage stages, **so that** limb damage provides gameplay feedback
   before severance.
3. **US-4.3.9.1** -- **As a** game developer (P-15), **I want** replacement limbs attachable to
   severed sockets at runtime, **so that** prosthetics and regrown appendages re-establish physics
   constraints.
4. **US-4.3.9.2** -- **As a** engine tester (P-27), **I want** to verify attachment re-establishes
   joint constraints and updates the skeleton hierarchy, **so that** replaced limbs function
   correctly.

## Warm Starting and LOD

| ID          | Persona                 |
|-------------|-------------------------|
| US-4.3.10.1 | engine developer (P-26) |
| US-4.3.11.1 | game developer (P-15)   |
| US-4.3.12.1 | game developer (P-15)   |

1. **US-4.3.10.1** -- **As an** engine developer (P-26), **I want** the warm-start factor tunable
   via `SolverConfig` per platform tier, **so that** I can trade convergence speed against
   oscillation risk on constrained devices.
2. **US-4.3.11.1** -- **As a** game developer (P-15), **I want** ragdoll simulation to drop to
   animation blend at distance via RagdollLod tiers, **so that** battle scenes with many downed
   enemies stay within the physics frame budget.
3. **US-4.3.12.1** -- **As a** game developer (P-15), **I want** distant chains and ropes to use a
   cheap Verlet fallback without collision, **so that** decorative environmental chains do not
   consume solver time.
