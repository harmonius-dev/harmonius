# User Stories — 4.1 Rigid Body Dynamics

## F-4.1.1 Deterministic Fixed-Timestep Integration

**US-4.1.1a** As a gameplay programmer, I want rigid body simulation to produce identical
results every run so that server-authoritative rollback and client prediction never diverge
due to floating-point non-determinism.

**US-4.1.1b** As a QA engineer, I want to record and replay a physics scenario and get
bit-identical results so that I can reproduce bugs deterministically from saved replays.

## F-4.1.2 Simulation Substeps

**US-4.1.2a** As a gameplay programmer, I want to configure substep count via a
`PhysicsConfig` resource so that I can trade CPU time for constraint stability on a
per-project basis without modifying engine code.

**US-4.1.2b** As a designer, I want to override substep count on individual entities so
that high-fidelity objects (e.g., a swinging wrecking ball) stay stable without increasing
the global substep count for every object.

## F-4.1.3 Contact Resolution with Restitution and Friction

**US-4.1.3a** As a designer, I want to assign physics materials with friction and
restitution values to surfaces so that ice feels slippery, rubber bounces, and metal
clangs — all without writing code.

**US-4.1.3b** As a player, I want a ball dropped on a bouncy surface to rebound to a
believable height so that physics interactions feel consistent and predictable.

## F-4.1.4 Continuous Collision Detection

**US-4.1.4a** As a gameplay programmer, I want to flag fast projectiles with a `CcdEnabled`
component so that bullets and arrows never tunnel through thin walls regardless of their
speed.

**US-4.1.4b** As a QA engineer, I want to verify that a high-speed sphere cannot pass
through a thin wall so that I can confirm CCD prevents tunneling under extreme conditions.

## F-4.1.5 Simulation Islands

**US-4.1.5a** As a gameplay programmer, I want independent groups of interacting bodies to
be automatically partitioned into islands so that the solver parallelizes across CPU cores
without manual grouping.

## F-4.1.6 Body Sleeping and Wake-Up

**US-4.1.6a** As a gameplay programmer, I want bodies at rest to sleep automatically and
wake when disturbed so that scenes with thousands of settled objects consume minimal CPU
without manual deactivation.

**US-4.1.6b** As a player, I want a stack of crates to remain still until something hits
it, then react immediately, so that the world feels stable yet responsive.

## F-4.1.7 Cross-Zone Physics Continuity

**US-4.1.7a** As a player, I want a rolling boulder to cross a streaming zone boundary
without stuttering, stopping, or changing direction so that the open world feels seamless.

**US-4.1.7b** As a gameplay programmer, I want physics entities to migrate between ECS
worlds while preserving momentum and contact state so that zone boundaries are invisible
to gameplay logic.

## F-4.1.8 Character Controller

**US-4.1.8a** As a designer, I want to configure slope limits and step heights on the
character controller so that characters reject steep surfaces and climb small stairs without
any scripting.

**US-4.1.8b** As a player, I want my character to walk up stairs, slide on steep slopes,
and land smoothly after jumps so that movement feels polished across all terrain types.

## F-4.1.9 Moving Platform System

**US-4.1.9a** As a designer, I want to place elevators, conveyor belts, and rotating
platforms that automatically carry characters and objects standing on them without drift
or jitter.

**US-4.1.9b** As a player, I want to stand on a moving platform and stay in place relative
to it so that riding elevators and conveyor belts feels natural without manual adjustment.

## F-4.1.10 Surface Smoothing and Ground Conformance

**US-4.1.10a** As a designer, I want character movement over jagged terrain to appear smooth
so that foot-placement IK looks correct and the camera does not vibrate on rough surfaces.

**US-4.1.10b** As a player, I want to run across terrain without micro-bouncing or
stuttering so that locomotion feels fluid even on uneven ground.
