# 9.4 — Animation State Machine

## State Graph

### F-9.4.1 Animation State Graph

CPU-side declarative state graph where each node represents a pose source (clip, blend tree,
sub-state machine, or montage). The graph is evaluated each frame to produce blend descriptors (clip
references, weights, and playback times) that are uploaded to the GPU for evaluation. Supports
per-character graph instances with shared graph definitions to minimize memory overhead across
thousands of MMO characters.

- **Requirements:** R-9.4.1
- **Dependencies:** F-9.1.2, F-9.1.3
- **Platform notes:** State graph complexity (node count) uniform across platforms. Active graph
  instances capped by per-tier animation instance budget (see F-9.1.5).

## Transitions

### F-9.4.2 Transitions with Blend Profiles and Sync Markers

Defines transitions between states with configurable blend duration, blend curve (linear,
ease-in/out, cubic), and per-bone blend profiles that allow different body parts to transition at
different rates. Sync markers embedded in animation clips align footfalls and other cyclic events
across source and destination states, preventing foot sliding during locomotion transitions.

- **Requirements:** R-9.4.2
- **Dependencies:** F-9.4.1
- **Platform notes:** Per-bone blend profiles supported on all tiers. Sync marker alignment is
  lightweight. No platform-specific scaling required.

## Sub-State Machines

### F-9.4.3 Sub-State Machines

Encapsulates a group of related states into a reusable sub-graph with defined entry and exit points.
Enables modular authoring of complex behaviors (e.g., a "combat" sub-state machine nested inside a
top-level locomotion graph). Sub-state machines can be shared across character archetypes and
composed hierarchically to manage complexity in MMO character animation.

- **Requirements:** R-9.4.3
- **Dependencies:** F-9.4.1
- **Platform notes:** Sub-state machine nesting depth uniform across platforms. CPU-side evaluation
  is lightweight. No platform-specific scaling required.

## Animation Layers and Blending

### F-9.4.4 State Machine Animation Layers

Runs multiple state machine instances in parallel on separate layers with per-bone masks and blend
modes (override, additive). Enables independent upper-body and lower-body state machines, facial
expression layers, and additive hit-reaction overlays. Layer weights are dynamically adjustable at
runtime to smoothly activate and deactivate overlay behaviors.

- **Requirements:** R-9.4.4
- **Dependencies:** F-9.4.1, F-9.1.4
- **Platform notes:** Parallel layer count inherits per-tier limits from F-9.1.4: mobile 2, Switch
  3, desktop 4+.

## State Variables and Conditions

### F-9.4.5 State Variables and Conditions

Exposes named parameters (booleans, floats, integers, triggers) that gameplay code sets to drive
state transitions. Transition rules are authored as boolean expressions over these parameters (e.g.,
`speed > 0.1 && is_grounded`). Trigger parameters auto-reset after consumption, ensuring one-shot
events like jump or attack fire exactly once.

- **Requirements:** R-9.4.5
- **Dependencies:** F-9.4.1
- **Platform notes:** State variable evaluation is CPU-side and lightweight on all platforms. No
  platform-specific scaling required.

## Sync Groups

### F-9.4.6 Sync Groups

Groups multiple animation clips that must stay phase-synchronized regardless of their playback
rates. All clips in a sync group advance by normalized time, keeping their sync markers aligned.
Essential for locomotion blending where walk and run cycles must maintain consistent foot contact
timing to eliminate sliding artifacts when blending between movement speeds.

- **Requirements:** R-9.4.6
- **Dependencies:** F-9.4.2
- **Platform notes:** Sync group clip count inherits per-tier blend limits from F-9.1.3. Phase
  synchronization is lightweight on all platforms.

## Animation Montages

### F-9.4.7 Animation Montages

One-shot or looping animation sequences that temporarily override state machine output on specific
bone groups. Montages support branching sections, notify events, and blend-in/blend-out curves. Used
for emotes, combat abilities, interaction animations, and cutscene-driven poses in MMO gameplay
where hundreds of players may trigger unique montages simultaneously.

- **Requirements:** R-9.4.7
- **Dependencies:** F-9.4.1, F-9.1.4
- **Platform notes:** Concurrent montage count inherits per-tier animation instance budget (see
  F-9.1.5). Montage branching complexity uniform across platforms.

## Blend Spaces

### F-9.4.8 1D and 2D Blend Spaces

Parameterized blend nodes that interpolate between animation clips based on one or two continuous
variables. A 1D blend space (e.g., speed) selects between walk, jog, and sprint clips. A 2D blend
space (e.g., speed x direction) handles full locomotion with strafing, forward, and backward motion.
Sample points are placed in the parameter space and triangulated; the runtime evaluates barycentric
weights for the active triangle. Blend spaces are authored in the visual animation editor and
connected as pose sources in the state graph.

- **Requirements:** R-9.4.8
- **Dependencies:** F-9.4.1, F-9.1.3 (Animation Blending)
- **Platform notes:** Blend space sample point count scales per tier: mobile 6-8 samples, Switch 12,
  desktop 16+. Triangulation is pre-computed on all platforms.

## Aim Offsets

### F-9.4.9 Aim Offset and Additive Aim Layers

Additive animation layers that modify a base locomotion pose to point a weapon or look direction
toward a target. Aim offsets are 2D blend spaces parameterized by pitch and yaw, producing additive
bone rotations applied on top of any locomotion state. Supports smooth interpolation between aim
poses, per-bone masking (only upper body aims while lower body locomotes), and integration with the
IK system (F-9.3.1) for precise weapon alignment. Used for third-person shooters, bow aiming, and
NPC look-at behavior.

- **Requirements:** R-9.4.9
- **Dependencies:** F-9.4.8, F-9.3.1 (IK Solvers), F-9.1.4 (Animation Layers)
- **Platform notes:** Aim offset sample count inherits per-tier blend space limits from F-9.4.8. IK
  precision for weapon alignment reduced on mobile.

## AI Animation Integration

### F-9.4.10 AI Animation Integration

Behavior trees (F-7.3.1) and GOAP planners (F-7.5.1) trigger animation state transitions and ability
playback through the logic graph system (F-15.8.4). AI agents set blackboard variables that drive
blend space parameters (movement speed, facing direction) and trigger state transitions (attack,
dodge, stagger, death). The animation system provides query APIs for AI: current state, remaining
clip time, root motion displacement, and whether a montage notify has fired — enabling AI to
synchronize decision-making with animation timing.

- **Requirements:** R-9.4.10
- **Dependencies:** F-9.4.1, F-7.3.1 (Behavior Trees), F-7.5.1 (GOAP), F-15.8.4 (Gameplay Logic
  Graphs)
- **Platform notes:** AI-driven animation integration is CPU-side and lightweight. Active
  AI-animated agent count inherits per-tier instance budget (see F-9.1.5).
