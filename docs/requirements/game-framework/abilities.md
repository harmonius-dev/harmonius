# R-13.10 — Abilities and Combat Requirements

## Ability System

### R-13.10.1 Data-Driven Ability Composition

The engine **SHALL** define abilities as data-driven assets composed from modular, reusable building
blocks — activation conditions, resource costs, targeting modes (self, single, AoE), gameplay
effects, animation references, VFX/audio cues, and AI hints — authored entirely in the visual editor
without code.

- **Derived from:** [F-13.10.1](../../features/game-framework/abilities.md)
- **Rationale:** Modular composition lets designers build and iterate on abilities by combining
  shared effect, targeting, and animation components without programmer involvement, aligning with
  the no-code engine philosophy.
- **Verification:** In the visual editor, author a "Fireball" ability using a "Fire Damage" effect
  component, AoE sphere targeting, and a cast animation reference. Author a "Flame Sword" ability
  reusing the same "Fire Damage" component with melee single-target targeting. Verify both abilities
  resolve correctly at runtime and share the same effect component instance.

### R-13.10.2 Ability Activation Modes with Input Integration

The engine **SHALL** support configurable ability activation modes — press, hold (channeled),
charge, combo (sequential within timing window), and toggle — bound to input actions, validating
cooldowns, resource costs, and cast conditions before execution, with AI agents activating through
the same API via synthetic input events.

- **Derived from:** [F-13.10.2](../../features/game-framework/abilities.md)
- **Rationale:** Unified activation through input actions ensures player and AI agents share
  identical ability execution paths, preventing behavioral divergence between human and
  AI-controlled characters.
- **Verification:** Bind an ability to a press action and verify instant cast. Bind a hold ability
  and verify it remains active only while held. Bind a combo ability, press the input three times
  within the timing window, and verify the combo chain executes. Trigger the same ability from an AI
  behavior tree synthetic input and verify identical execution. Attempt activation during cooldown
  and verify rejection.

### R-13.10.3 Composable Gameplay Effect System

The engine **SHALL** provide a composable effect system supporting instant, duration, periodic, and
permanent effects with configurable stacking rules (additive, multiplicative, highest-wins,
non-stacking), source metadata, damage type tags, and conditional tag interactions.

- **Derived from:** [F-13.10.3](../../features/game-framework/abilities.md)
- **Rationale:** A tag-aware, composable effect system enables complex gameplay interactions (fire
  damage removes frozen debuff) without hardcoded logic, keeping all combat rules data-driven and
  designer-editable.
- **Verification:** Apply two additive damage bonus effects (+10, +20) and verify the combined bonus
  is +30. Apply two highest-wins buffs (+15%, +25%) and verify only +25% applies. Apply a periodic
  heal (10 HP every 2 seconds for 10 seconds) and verify 5 ticks. Apply a "frozen" debuff, then
  apply "fire" damage, and verify the frozen debuff is removed via tag interaction rules.

## Melee and Ranged Combat

### R-13.10.4 Animation-Driven Melee Combat

The engine **SHALL** resolve melee hit detection using animation-event-activated weapon hitboxes
against per-bone hurtboxes with damage multipliers, directional damage bonuses, and hit reaction
triggers, supporting weapon trails, impact VFX, and hit-stop time dilation.

- **Derived from:** [F-13.10.4](../../features/game-framework/abilities.md)
- **Rationale:** Tying hit detection to animation events ensures attacks connect visually where the
  weapon appears, and per-bone hurtbox multipliers enable skill-rewarding gameplay (headshots)
  without separate collision logic.
- **Verification:** Play a sword swing animation; verify the weapon hitbox activates only during the
  defined attack window. Strike a target's head hurtbox and verify the x2 multiplier is applied.
  Strike from behind and verify the flank bonus. Verify hit-stop pauses the game for the configured
  duration on impact.

### R-13.10.5 Projectile-Based Ranged Combat

The engine **SHALL** simulate projectiles as ECS entities with configurable trajectory behaviors
(linear, arced, homing, beam, spread), physics-driven collision using CCD for fast projectiles,
effect payloads applied on impact, and aim-assist support for gamepad input.

- **Derived from:** [F-13.10.5](../../features/game-framework/abilities.md)
- **Rationale:** ECS-based projectiles integrate naturally with the physics and rendering pipelines,
  and configurable trajectories cover the full spectrum of ranged weapon archetypes across game
  genres.
- **Verification:** Spawn a linear projectile and verify it travels in a straight line and applies
  its damage effect on collision. Spawn an arced projectile with gravity and verify the parabolic
  trajectory. Spawn a homing missile, move the target laterally, and verify the missile adjusts
  course. Fire a fast bullet and verify CCD prevents tunneling through thin geometry. Enable aim
  assist on gamepad and verify snap-to-target within the configured magnetism radius.

### R-13.10.6 Hitbox and Hurtbox System

The engine **SHALL** provide per-bone hitbox and hurtbox collision shapes with per-region damage
multipliers, multi-hit prevention per swing, editor visualization, and network lag compensation
using historical entity snapshots.

- **Derived from:** [F-13.10.6](../../features/game-framework/abilities.md)
- **Rationale:** Per-bone hit regions with multipliers and lag compensation are essential for fair,
  responsive combat in both single-player and networked games, and visual authoring on the skeleton
  reduces setup errors.
- **Verification:** Define head (x2), torso (x1), and limb (x0.75) hurtboxes on a skeleton. Strike
  each region and verify the correct multiplier is applied. Perform a wide swing hitting the same
  target twice and verify only one hit registers per swing. Enable the editor debug overlay and
  verify all hitbox/hurtbox shapes render correctly on the skeleton. In a networked scenario with
  100ms latency, verify lag-compensated hit detection matches the attacker's visual frame.

## Non-Functional Requirements

### R-13.10.NF1 Maximum Concurrent Gameplay Effects

The engine **SHALL** support at least 64 concurrent gameplay effects per entity (buffs, debuffs,
DoTs, HoTs) without the effect evaluation exceeding 0.1 ms per entity per frame.

- **Derived from:** F-13.10.3
- **Rationale:** Boss encounters and large-group content commonly apply dozens of simultaneous
  effects to each participant; the system must scale without becoming a frame budget bottleneck.
- **Verification:** Apply 64 concurrent effects (mix of duration, periodic, and permanent) to a
  single entity. Measure per-frame effect evaluation time and verify it stays under 0.1 ms. Apply 64
  effects to each of 40 entities (raid scenario) and verify total effect evaluation stays under 4
  ms.

### R-13.10.NF2 Ability Activation Response Time

The engine **SHALL** begin ability execution (animation start, resource deduction, visual feedback)
within 1 frame (16.67 ms at 60 fps) of the activation input, excluding network round-trip for server
validation.

- **Derived from:** F-13.10.1, F-13.10.2
- **Rationale:** Perceptible delay between input and ability response breaks the feel of responsive
  combat and disadvantages players in competitive scenarios.
- **Verification:** Bind an ability to a press input. Measure time from input event to animation
  playback start and resource deduction. Verify the response begins within the same frame across 100
  consecutive activations at 60 fps.

### R-13.10.NF3 Melee Hit Detection Accuracy

The engine **SHALL** resolve melee hit detection with at most 1 frame of latency between the
animation-event-defined hit window and the actual collision query, ensuring hits register when
visually expected.

- **Derived from:** F-13.10.4, F-13.10.6
- **Rationale:** Hit detection that lags behind the animation causes frustrating ghost swings where
  attacks appear to connect but deal no damage.
- **Verification:** Play a melee attack animation. Measure the time delta between the animation
  event firing and the collision query execution. Verify the delta is under 1 frame (16.67 ms at 60
  fps) across 100 attack animations.
