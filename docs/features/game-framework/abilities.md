# 13.10 — Abilities and Combat

## Ability System

### F-13.10.1 Ability Definition and Composition

A data-driven ability system where each ability is composed from modular building blocks: activation
conditions (input trigger, AI decision, item use), resource costs (mana, stamina, cooldown timers),
targeting (self, single target, AoE cone/sphere/line), gameplay effects (damage, heal, buff, debuff,
knockback, stun), animation playback (montage reference with sync points), VFX (particle effect
references with attachment points), audio (sound cue references for cast, impact, loop), and AI
behavior hints (threat generation, interrupt priority). Abilities are authored as visual assets in
the editor, not code. Each building block is a reusable component that can be shared across
abilities — a "Fire Damage" effect component is used by both "Fireball" and "Flame Sword."

- **Requirements:** R-13.10.1
- **Dependencies:** F-15.8.4 (Gameplay Logic Graphs), F-9.4.7 (Animation Montages), F-11.6.1 (Effect
  Graph)
- **Platform notes:** None

### F-13.10.2 Ability Activation and Input Integration

Abilities bind to input actions (F-6.2.1) through configurable activation modes: press (instant
cast), hold (channeled — ability active while held), charge (hold to charge, release to fire with
charge-dependent power), combo (sequential presses within a timing window execute a combo chain),
and toggle (press to activate, press again to deactivate). Activation respects ability state:
cooldown remaining, resource availability, cast conditions (grounded, airborne, moving), and global
cooldown groups. The input system communicates ability requests to the ability manager, which
validates and executes through the logic graph. AI agents activate abilities through the same API
with synthetic input events from behavior trees (F-7.3.1).

- **Requirements:** R-13.10.2
- **Dependencies:** F-13.10.1, F-6.2.1 (Input Actions), F-7.3.1 (Behavior Trees)
- **Platform notes:** None

### F-13.10.3 Gameplay Effect System

A composable effect system that modifies entity attributes: instant effects (deal 50 damage),
duration effects (reduce speed by 30% for 5 seconds), periodic effects (heal 10 HP every 2 seconds
for 10 seconds), and permanent effects (learn a new ability). Effects stack according to
configurable rules: additive (multiple damage bonuses sum), multiplicative (multiple speed modifiers
multiply), highest-wins (only the strongest buff applies), or non-stacking (refresh duration on
reapply). Effects carry metadata: source entity, damage type (physical, fire, ice, poison), and
gameplay tags for conditional interactions (fire damage removes frozen debuff). The effect system
integrates with gameplay databases (F-13.7.9) for stat tables and damage formulas.

- **Requirements:** R-13.10.3
- **Dependencies:** F-13.10.1, F-13.7.9 (Stat Tables), F-13.7.5 (Formula DSL)
- **Platform notes:** None

## Melee and Ranged Combat

### F-13.10.4 Melee Combat System

Close-range combat with hit detection driven by animation events (F-9.1.9) and physics shape casts
(F-4.4.2). Weapon hitboxes are defined as collision shapes attached to skeleton bones, activated
during attack animation windows. The combat system resolves hits against target hurtboxes (per-bone
damage regions with multipliers — headshot, limb, body), applies directional damage (front, back,
flank bonuses), and triggers hit reactions (stagger, knockback, death) through the animation state
machine (F-9.4.1). Supports weapon trails (F-11.1.7), impact VFX (F-11.6.4), and hit-stop (brief
time dilation on impact for game feel). Blocking, parrying, and dodge mechanics are implemented as
ability states with invulnerability frames.

- **Requirements:** R-13.10.4
- **Dependencies:** F-13.10.1, F-9.1.9 (Animation Events), F-4.4.2 (Shape Casts), F-9.4.1 (State
  Machine)
- **Platform notes:** None

### F-13.10.5 Ranged Combat and Projectile System

Projectile-based ranged combat with configurable projectile behaviors: linear (bullets), arced
(grenades, arrows), homing (missiles), beam (continuous ray), and spread (shotgun pattern).
Projectiles are ECS entities with `Projectile`, `Velocity`, and `Collider` components, simulated by
the physics system with optional gravity and wind. Hit detection uses continuous collision detection
(F-4.1.5) for fast projectiles or ray casts for hitscan weapons. Projectiles carry effect payloads
(F-13.10.3) applied on impact. The aiming system integrates with aim offsets (F-9.4.9) for accurate
weapon pointing and supports aim assist on gamepad (magnetism, friction, snap-to-target configurable
per weapon type).

- **Requirements:** R-13.10.5
- **Dependencies:** F-13.10.1, F-13.10.3, F-4.1.5 (CCD), F-9.4.9 (Aim Offsets)
- **Platform notes:** None

### F-13.10.6 Hitbox and Hurtbox System

Per-bone hit detection regions defined as collision shapes (capsules, boxes) attached to skeleton
bones via socket transforms. Hurtboxes define damageable regions with per-region damage multipliers
(head x2, torso x1, limbs x0.75), armor penetration modifiers, and critical hit zones. Hitboxes
define damage-dealing regions activated during attack animations. The system supports: overlapping
hitboxes (a sword swing hits multiple targets), multi-hit prevention (each swing hits each target at
most once), and hitbox visualization in the editor and runtime debug overlay. Hitbox data is
authored visually on the skeleton in the animation editor. For networked games, hitbox state is
lag-compensated using the server's historical entity snapshots (F-8.4.5).

- **Requirements:** R-13.10.6
- **Dependencies:** F-9.1.1 (GPU Skinning), F-4.2.1 (Broadphase), F-8.4.5 (Lag Compensation)
- **Platform notes:** None
