# 9.4 — Animation State Machine

## State Graph

| ID      | Feature               |
|---------|-----------------------|
| F-9.4.1 | Animation State Graph |

1. **F-9.4.1** — CPU-side declarative state graph where each node represents a pose source (clip,
   blend tree, sub-state machine, or montage). Evaluated each frame to produce blend descriptors
   uploaded to the GPU. Per-character graph instances share graph definitions to minimize memory
   overhead.
   - **Deps:** F-9.1.2, F-9.1.3
   - **Platform:** State graph complexity uniform across platforms. Active graph instances capped by
     per-tier animation instance budget (see F-9.1.5).

## Transitions

| ID      | Feature                                          |
|---------|--------------------------------------------------|
| F-9.4.2 | Transitions with Blend Profiles and Sync Markers |

1. **F-9.4.2** — Transitions between states with configurable blend duration, blend curve (linear,
   ease-in/out, cubic), and per-bone blend profiles. Sync markers embedded in animation clips align
   footfalls and other cyclic events across source and destination states.
   - **Deps:** F-9.4.1
   - **Platform:** Per-bone blend profiles supported on all tiers. Sync marker alignment is
     lightweight. No platform-specific scaling required.

## Sub-State Machines

| ID      | Feature            |
|---------|--------------------|
| F-9.4.3 | Sub-State Machines |

1. **F-9.4.3** — Encapsulates related states into a reusable sub-graph with defined entry and exit
   points. Enables modular authoring of complex behaviors. Sub-state machines can be shared across
   character archetypes and composed hierarchically.
   - **Deps:** F-9.4.1
   - **Platform:** Nesting depth uniform across platforms. CPU-side evaluation is lightweight. No
     platform-specific scaling required.

## Animation Layers and Blending

| ID      | Feature                        |
|---------|--------------------------------|
| F-9.4.4 | State Machine Animation Layers |

1. **F-9.4.4** — Runs multiple state machine instances in parallel on separate layers with per-bone
   masks and blend modes (override, additive). Enables independent upper-body and lower-body state
   machines, facial expression layers, and additive hit-reaction overlays. Layer weights are
   dynamically adjustable.
   - **Deps:** F-9.4.1, F-9.1.4
   - **Platform:** Parallel layer count inherits per-tier limits from F-9.1.4: mobile 2, Switch 3,
     desktop 4+.

## State Variables and Conditions

| ID      | Feature                        |
|---------|--------------------------------|
| F-9.4.5 | State Variables and Conditions |

1. **F-9.4.5** — Exposes named parameters (booleans, floats, integers, triggers) that gameplay code
   sets to drive state transitions via boolean expressions. Trigger parameters auto-reset after
   consumption, ensuring one-shot events fire exactly once.
   - **Deps:** F-9.4.1
   - **Platform:** CPU-side and lightweight on all platforms. No platform-specific scaling required.

## Sync Groups

| ID      | Feature     |
|---------|-------------|
| F-9.4.6 | Sync Groups |

1. **F-9.4.6** — Groups multiple animation clips that must stay phase-synchronized regardless of
   playback rates. All clips in a sync group advance by normalized time, keeping sync markers
   aligned. Essential for locomotion blending where walk and run cycles must maintain consistent
   foot contact timing.
   - **Deps:** F-9.4.2
   - **Platform:** Sync group clip count inherits per-tier blend limits from F-9.1.3. Phase
     synchronization is lightweight on all platforms.

## Animation Montages

| ID      | Feature            |
|---------|--------------------|
| F-9.4.7 | Animation Montages |

1. **F-9.4.7** — One-shot or looping animation sequences that temporarily override state machine
   output on specific bone groups. Montages support branching sections, notify events, and
   blend-in/blend-out curves. Used for emotes, combat abilities, interaction animations, and
   cutscene-driven poses.
   - **Deps:** F-9.4.1, F-9.1.4
   - **Platform:** Concurrent montage count inherits per-tier animation instance budget (see
     F-9.1.5). Branching complexity uniform across platforms.

## Blend Spaces

| ID      | Feature                |
|---------|------------------------|
| F-9.4.8 | 1D and 2D Blend Spaces |

1. **F-9.4.8** — Parameterized blend nodes that interpolate between animation clips based on one or
   two continuous variables. Sample points are triangulated; the runtime evaluates barycentric
   weights for the active triangle. Blend spaces are authored in the visual animation editor and
   connected as pose sources in the state graph.
   - **Deps:** F-9.4.1, F-9.1.3 (Animation Blending)
   - **Platform:** Sample point count scales per tier: mobile 6-8, Switch 12, desktop 16+.
     Triangulation is pre-computed on all platforms.

## Aim Offsets

| ID      | Feature                            |
|---------|------------------------------------|
| F-9.4.9 | Aim Offset and Additive Aim Layers |

1. **F-9.4.9** — Additive animation layers parameterized by pitch and yaw that produce additive bone
   rotations on top of locomotion. Supports smooth interpolation between aim poses, per-bone
   masking, and IK system integration (F-9.3.1) for precise weapon alignment.
   - **Deps:** F-9.4.8, F-9.3.1 (IK Solvers), F-9.1.4 (Animation Layers)
   - **Platform:** Aim offset sample count inherits per-tier blend space limits from F-9.4.8. IK
     precision reduced on mobile.

## AI Animation Integration

| ID       | Feature                  |
|----------|--------------------------|
| F-9.4.10 | AI Animation Integration |

1. **F-9.4.10** — Behavior trees (F-7.3.1) and GOAP planners (F-7.5.1) trigger animation state
   transitions through the logic graph system (F-15.8.4). AI agents set blackboard variables that
   drive blend space parameters and trigger state transitions. Query APIs: current state, remaining
   clip time, root motion displacement, montage notify status.
   - **Deps:** F-9.4.1, F-7.3.1, F-7.5.1, F-15.8.4
   - **Platform:** CPU-side and lightweight. Active AI-animated agent count inherits per-tier
     instance budget (see F-9.1.5).

## Transition Modes and Pose Sources

| ID       | Feature |
|----------|----------------------------------- |
| F-9.4.11 | Inertialization Transition Blend Mode |
| F-9.4.12 | Multiply Animation Layer Blend Mode |
| F-9.4.13 | Sprite-Sheet Pose Source in State Machine |
| F-9.4.14 | Motion Matching Pose Source in State Machine |

1. **F-9.4.11** — An inertialization transition mode samples only the target pose at each step and
   decays the source-to-target offset over the transition duration. Costs roughly half of a
   crossfade because only one clip is sampled. Integrates with sync markers and per-bone blend
   profiles the same as crossfade.
   - **Deps:** F-9.4.2 (Transitions), F-9.1.3 (Animation Blending)
2. **F-9.4.12** — A Multiply layer blend mode scales bone transforms by the layer weight times a
   bone factor, enabling size pulsing, muscle flex, and magnification effects that cannot be
   expressed as override or additive layers.
   - **Deps:** F-9.4.4 (State Machine Animation Layers), F-9.1.4 (Animation Layers)
3. **F-9.4.13** — A `SpriteSheet` pose source reads a texture atlas plus frame list, frame FPS, and
   playback mode (once, loop, ping-pong) to drive 2D sprite-sheet animation through the same state
   machine transitions, conditions, layers, and sync groups used for 3D skeletal animation.
   - **Deps:** F-9.4.1 (State Graph), F-9.4.2 (Transitions)
4. **F-9.4.14** — A `MotionMatching` pose source in the state machine references a pose database and
   search schema. The state machine can enter and exit motion matching regions via inertialization
   transitions (F-9.4.11), preserving transition blend fidelity.
   - **Deps:** F-9.4.1, F-9.3.6 (Motion Matching), F-9.4.11

## Montages, Parameters, and Feedback

| ID       | Feature |
|----------|----------------------------------- |
| F-9.4.15 | Montage Priority Resolution |
| F-9.4.16 | AnimationParams ECS Component Bridge |
| F-9.4.17 | AnimationQuery Read-Only State Export |

1. **F-9.4.15** — `MontageDef` carries a `u8` priority field. When multiple montages target the same
   bone groups, the highest priority wins. Equal priorities resolve by last-started-wins.
   Deterministic priority resolution prevents visual artifacts when gameplay and AI systems
   simultaneously request montages.
   - **Deps:** F-9.4.7 (Animation Montages)
2. **F-9.4.16** — An `AnimationParams` ECS component holds structured parameter values (speed,
   direction, grounded, crouching, jumping, falling, aim pitch, aim yaw, typed triggers) that
   gameplay and AI systems write. The state machine reads this component for transition evaluation,
   decoupling parameter sources from animation consumers so the same state graph drives player and
   AI instances of a character.
   - **Deps:** F-9.4.5 (State Variables and Conditions), F-1.1.2
3. **F-9.4.17** — An `AnimationQuery` read-only ECS component exposes the current state name,
   elapsed time, remaining time, transitioning flag, active montage name, and root motion delta. AI
   and gameplay systems query this component to sequence actions relative to animation progress
   without polling state machine internals.
   - **Deps:** F-9.4.1, F-9.4.7, F-9.1.6 (Root Motion)
