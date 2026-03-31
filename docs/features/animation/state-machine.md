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
