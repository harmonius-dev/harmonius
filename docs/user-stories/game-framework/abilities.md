# User Stories — 13.10 Abilities and Combat

## F-13.10.1 Ability Definition and Composition

## US-13.10.1.1 Compose Abilities From Modular Building Blocks

**As a** gameplay director (P-3), **I want to** compose abilities from reusable building blocks
(activation conditions, costs, targeting, effects, animation, VFX, audio), **so that** common
components like "Fire Damage" are shared across abilities.

## US-13.10.1.2 Author Ability Assets in the Visual Editor

**As a** gameplay director (P-3), **I want to** author abilities as visual assets in the editor
without writing code, **so that** new abilities are created entirely through data.

## US-13.10.1.3 Preview Ability Execution in the Editor

**As a** player (P-23), **I want to** abilities to produce coordinated animation, VFX, and audio
when activated, **so that** ability use feels impactful and polished.

## US-13.10.1.4 Verify Reusable Components Are Shared Correctly

**As an** engine tester (P-27), **I want to** create two abilities sharing a "Fire Damage" effect
component and verify both produce identical damage behavior, **so that** component sharing works
without duplication bugs.

## US-13.10.1.5 Verify Ability Definition Validates Required Fields

**As an** engine tester (P-27), **I want to** create an ability asset missing a required field and
verify the editor reports a clear validation error, **so that** incomplete abilities are caught at
author time.

## F-13.10.2 Ability Activation and Input Integration

## US-13.10.2.1 Configure Activation Modes Per Ability

**As a** gameplay director (P-3), **I want to** configure activation modes (press, hold, charge,
combo, toggle) per ability, **so that** each ability responds to input in its intended way.

## US-13.10.2.2 Bind Abilities to Input Actions

**As a** gameplay director (P-3), **I want to** bind abilities to input actions with activation
respecting cooldowns, resource availability, and cast conditions, **so that** ability inputs are
validated before execution.

## US-13.10.2.3 Execute Combo Chains by Sequential Presses

**As a** player (P-23), **I want to** execute combo chains by pressing the ability button within
timing windows, **so that** skilled play is rewarded with extended combos.

## US-13.10.2.4 Verify AI Agents Activate Abilities Through the Same API

**As an** engine tester (P-27), **I want to** verify that AI agents activate abilities through the
same API as player input with synthetic events, **so that** NPC and player ability execution is
identical.

## US-13.10.2.5 Verify Activation Rejects Invalid State

**As an** engine tester (P-27), **I want to** attempt ability activation while on cooldown, without
resources, and while stunned, and verify each is rejected with the correct reason, **so that**
activation validation covers all conditions.

## F-13.10.3 Gameplay Effect System

## US-13.10.3.1 Define Effect Types and Stacking Rules

**As a** gameplay director (P-3), **I want to** define instant, duration, periodic, and permanent
effects with stacking rules (additive, multiplicative, highest-wins, non-stacking), **so that**
effect behavior matches intended balance design.

## US-13.10.3.2 Configure Effect Metadata for Conditional Interactions

**As a** gameplay director (P-3), **I want to** attach metadata (source entity, damage type,
gameplay tags) to effects so fire damage removes frozen debuffs, **so that** cross-effect
interactions are data-driven.

## US-13.10.3.3 See Buff and Debuff Indicators in the HUD

**As a** player (P-23), **I want to** see active effects displayed as icons with duration timers in
the HUD, **so that** I know what buffs and debuffs are affecting my character.

## US-13.10.3.4 Verify Periodic Effect Tick Accuracy

**As an** engine tester (P-27), **I want to** apply a periodic effect (heal 10 HP every 2 seconds
for 10 seconds) and verify exactly 5 ticks occur at the correct intervals, **so that** periodic
timing is accurate.

## US-13.10.3.5 Verify Effect Stacking Rules Produce Correct Values

**As an** engine tester (P-27), **I want to** apply multiple effects with each stacking mode and
verify final stat values match expected calculations, **so that** stacking math is correct.

## F-13.10.4 Melee Combat System

## US-13.10.4.1 Configure Melee Weapon Hitboxes on Skeleton

**As a** gameplay director (P-3), **I want to** define weapon hitboxes as collision shapes attached
to skeleton bones, activated during attack animation windows, **so that** melee hit detection
matches weapon motion.

## US-13.10.4.2 Configure Per-Bone Damage Multipliers

**As a** gameplay director (P-3), **I want to** define per-bone hurtbox regions with damage
multipliers (headshot x2, limb x0.75) and directional bonuses, **so that** melee combat rewards
precision and positioning.

## US-13.10.4.3 Experience Melee Combat With Hit Reactions

**As a** player (P-23), **I want to** melee attacks to produce weapon trails, impact VFX, hit- stop,
and target stagger animations, **so that** melee combat feels weighty and responsive.

## US-13.10.4.4 Verify Multi-Hit Prevention Per Swing

**As an** engine tester (P-27), **I want to** swing through overlapping targets and verify each
target is hit at most once per swing, **so that** multi-hit prevention works correctly.

## US-13.10.4.5 Verify Block/Parry/Dodge Invulnerability Frames

**As an** engine tester (P-27), **I want to** activate block, parry, and dodge states and verify
invulnerability frames prevent damage during their active windows, **so that** defensive mechanics
work reliably.

## F-13.10.5 Ranged Combat and Projectile System

## US-13.10.5.1 Configure Projectile Behaviors

**As a** gameplay director (P-3), **I want to** configure projectile types (linear, arced, homing,
beam, spread) with gravity, wind, and speed parameters, **so that** ranged combat offers diverse
weapon feel.

## US-13.10.5.2 Configure Aim Assist for Gamepad

**As a** gameplay director (P-3), **I want to** configure aim assist (magnetism, friction, snap-
to-target) per weapon type for gamepad play, **so that** ranged combat is accessible across input
devices.

## US-13.10.5.3 Fire Projectiles and See Impact Effects

**As a** player (P-23), **I want to** fire projectiles that travel with realistic physics and apply
damage effects on impact with VFX feedback, **so that** ranged attacks feel satisfying.

## US-13.10.5.4 Verify Continuous Collision Detection for Fast Projectiles

**As an** engine tester (P-27), **I want to** fire a fast projectile through a thin wall and verify
CCD prevents tunneling, **so that** fast projectiles collide reliably.

## US-13.10.5.5 Verify Projectile Effect Payloads Apply on Impact

**As an** engine tester (P-27), **I want to** fire a projectile carrying a gameplay effect and
verify the effect applies to the hit target, **so that** projectile damage integration works
correctly.

## F-13.10.6 Hitbox and Hurtbox System

## US-13.10.6.1 Author Hitbox Data on Skeleton in Animation Editor

**As a** gameplay director (P-3), **I want to** author hitbox and hurtbox shapes visually on the
skeleton in the animation editor, **so that** hit detection regions are defined without code.

## US-13.10.6.2 Visualize Hitboxes in Runtime Debug Overlay

**As a** gameplay director (P-3), **I want to** toggle hitbox and hurtbox visualization in the
runtime debug overlay, **so that** I can verify hit detection regions match animations.

## US-13.10.6.3 Experience Fair Hit Detection in Multiplayer

**As a** player (P-23), **I want to** hit detection to feel fair in multiplayer with lag
compensation, **so that** hits register where my crosshair was aimed regardless of latency.

## US-13.10.6.4 Verify Lag-Compensated Hitbox State

**As an** engine tester (P-27), **I want to** simulate 150ms latency and verify the server resolves
hits against historical entity snapshots at the client's fire time, **so that** lag compensation
produces fair results.

## US-13.10.6.5 Verify Hitbox Activation Matches Animation Windows

**As an** engine tester (P-27), **I want to** play an attack animation and verify hitboxes activate
and deactivate at the exact frames specified in the animation events, **so that** hitbox timing is
synchronized with animations.
