# 9.4 — Animation State Machine

## State Graph

### F-9.4.1 Animation State Graph

CPU-side declarative state graph where each node represents a pose source (clip, blend tree,
sub-state machine, or montage). The graph is evaluated each frame to produce blend descriptors
(clip references, weights, and playback times) that are uploaded to the GPU for evaluation.
Supports per-character graph instances with shared graph definitions to minimize memory overhead
across thousands of MMO characters.

- **Requirements:** R-9.4.1
- **Dependencies:** F-9.1.2, F-9.1.3
- **Platform notes:** None

## Transitions

### F-9.4.2 Transitions with Blend Profiles and Sync Markers

Defines transitions between states with configurable blend duration, blend curve (linear,
ease-in/out, cubic), and per-bone blend profiles that allow different body parts to transition
at different rates. Sync markers embedded in animation clips align footfalls and other cyclic
events across source and destination states, preventing foot sliding during locomotion
transitions.

- **Requirements:** R-9.4.2
- **Dependencies:** F-9.4.1
- **Platform notes:** None

## Sub-State Machines

### F-9.4.3 Sub-State Machines

Encapsulates a group of related states into a reusable sub-graph with defined entry and exit
points. Enables modular authoring of complex behaviors (e.g., a "combat" sub-state machine
nested inside a top-level locomotion graph). Sub-state machines can be shared across character
archetypes and composed hierarchically to manage complexity in MMO character animation.

- **Requirements:** R-9.4.3
- **Dependencies:** F-9.4.1
- **Platform notes:** None

## Animation Layers and Blending

### F-9.4.4 State Machine Animation Layers

Runs multiple state machine instances in parallel on separate layers with per-bone masks and
blend modes (override, additive). Enables independent upper-body and lower-body state machines,
facial expression layers, and additive hit-reaction overlays. Layer weights are dynamically
adjustable at runtime to smoothly activate and deactivate overlay behaviors.

- **Requirements:** R-9.4.4
- **Dependencies:** F-9.4.1, F-9.1.4
- **Platform notes:** None

## State Variables and Conditions

### F-9.4.5 State Variables and Conditions

Exposes named parameters (booleans, floats, integers, triggers) that gameplay code sets to
drive state transitions. Transition rules are authored as boolean expressions over these
parameters (e.g., `speed > 0.1 && is_grounded`). Trigger parameters auto-reset after
consumption, ensuring one-shot events like jump or attack fire exactly once.

- **Requirements:** R-9.4.5
- **Dependencies:** F-9.4.1
- **Platform notes:** None

## Sync Groups

### F-9.4.6 Sync Groups

Groups multiple animation clips that must stay phase-synchronized regardless of their playback
rates. All clips in a sync group advance by normalized time, keeping their sync markers aligned.
Essential for locomotion blending where walk and run cycles must maintain consistent foot
contact timing to eliminate sliding artifacts when blending between movement speeds.

- **Requirements:** R-9.4.6
- **Dependencies:** F-9.4.2
- **Platform notes:** None

## Animation Montages

### F-9.4.7 Animation Montages

One-shot or looping animation sequences that temporarily override state machine output on
specific bone groups. Montages support branching sections, notify events, and blend-in/blend-out
curves. Used for emotes, combat abilities, interaction animations, and cutscene-driven poses in
MMO gameplay where hundreds of players may trigger unique montages simultaneously.

- **Requirements:** R-9.4.7
- **Dependencies:** F-9.4.1, F-9.1.4
- **Platform notes:** None
