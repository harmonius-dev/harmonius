# User Stories — 13.10 Abilities and Combat

## F-13.10.1 Ability Definition and Composition

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.10.1.1 | gameplay director (P-3) | I want to compose abilities from reusable building blocks (activation conditions, costs, targeting, effects, animation, VFX, audio) so that common components like "Fire Damage" are shared across abilities |  | F-13.10.1 | R-13.10.1 |
| US-13.10.1.2 | gameplay director (P-3) | I want to author abilities as visual assets in the editor without writing code so that new abilities are created entirely through data |  | F-13.10.1 | R-13.10.1 |
| US-13.10.1.3 | player (P-23) | I want to abilities to produce coordinated animation, VFX, and audio when activated so that ability use feels impactful and polished |  | F-13.10.1 | R-13.10.1 |
| US-13.10.1.4 | engine tester (P-27) | I want to create two abilities sharing a "Fire Damage" effect component and verify both produce identical damage behavior so that component sharing works without duplication bugs |  | F-13.10.1 | R-13.10.1 |
| US-13.10.1.5 | engine tester (P-27) | I want to create an ability asset missing a required field and verify the editor reports a clear validation error so that incomplete abilities are caught at author time |  | F-13.10.1 | R-13.10.1 |

## F-13.10.2 Ability Activation and Input Integration

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.10.2.1 | gameplay director (P-3) | I want to configure activation modes (press, hold, charge, combo, toggle) per ability so that each ability responds to input in its intended way |  | F-13.10.2 | R-13.10.2 |
| US-13.10.2.2 | gameplay director (P-3) | I want to bind abilities to input actions with activation respecting cooldowns, resource availability, and cast conditions so that ability inputs are validated before execution |  | F-13.10.2 | R-13.10.2 |
| US-13.10.2.3 | player (P-23) | I want to execute combo chains by pressing the ability button within timing windows so that skilled play is rewarded with extended combos |  | F-13.10.2 | R-13.10.2 |
| US-13.10.2.4 | engine tester (P-27) | I want to verify that AI agents activate abilities through the same API as player input with synthetic events so that NPC and player ability execution is identical |  | F-13.10.2 | R-13.10.2 |
| US-13.10.2.5 | engine tester (P-27) | I want to attempt ability activation while on cooldown, without resources, and while stunned, and verify each is rejected with the correct reason so that activation validation covers all conditions |  | F-13.10.2 | R-13.10.2 |

## F-13.10.3 Gameplay Effect System

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.10.3.1 | gameplay director (P-3) | I want to define instant, duration, periodic, and permanent effects with stacking rules (additive, multiplicative, highest-wins, non-stacking) so that effect behavior matches intended balance design |  | F-13.10.3 | R-13.10.3 |
| US-13.10.3.2 | gameplay director (P-3) | I want to attach metadata (source entity, damage type, gameplay tags) to effects so fire damage removes frozen debuffs so that cross-effect interactions are data-driven |  | F-13.10.3 | R-13.10.3 |
| US-13.10.3.3 | player (P-23) | I want to see active effects displayed as icons with duration timers in the HUD so that I know what buffs and debuffs are affecting my character |  | F-13.10.3 | R-13.10.3 |
| US-13.10.3.4 | engine tester (P-27) | I want to apply a periodic effect (heal 10 HP every 2 seconds for 10 seconds) and verify exactly 5 ticks occur at the correct intervals so that periodic timing is accurate |  | F-13.10.3 | R-13.10.3 |
| US-13.10.3.5 | engine tester (P-27) | I want to apply multiple effects with each stacking mode and verify final stat values match expected calculations so that stacking math is correct |  | F-13.10.3 | R-13.10.3 |

## F-13.10.4 Melee Combat System

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.10.4.1 | gameplay director (P-3) | I want to define weapon hitboxes as collision shapes attached to skeleton bones, activated during attack animation windows so that melee hit detection matches weapon motion |  | F-13.10.4 | R-13.10.4 |
| US-13.10.4.2 | gameplay director (P-3) | I want to define per-bone hurtbox regions with damage multipliers (headshot x2, limb x0.75) and directional bonuses so that melee combat rewards precision and positioning |  | F-13.10.4 | R-13.10.4 |
| US-13.10.4.3 | player (P-23) | I want to melee attacks to produce weapon trails, impact VFX, hit- stop, and target stagger animations so that melee combat feels weighty and responsive |  | F-13.10.4 | R-13.10.4 |
| US-13.10.4.4 | engine tester (P-27) | I want to swing through overlapping targets and verify each target is hit at most once per swing so that multi-hit prevention works correctly |  | F-13.10.4 | R-13.10.4 |
| US-13.10.4.5 | engine tester (P-27) | I want to activate block, parry, and dodge states and verify invulnerability frames prevent damage during their active windows so that defensive mechanics work reliably |  | F-13.10.4 | R-13.10.4 |

## F-13.10.5 Ranged Combat and Projectile System

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.10.5.1 | gameplay director (P-3) | I want to configure projectile types (linear, arced, homing, beam, spread) with gravity, wind, and speed parameters so that ranged combat offers diverse weapon feel |  | F-13.10.5 | R-13.10.5 |
| US-13.10.5.2 | gameplay director (P-3) | I want to configure aim assist (magnetism, friction, snap- to-target) per weapon type for gamepad play so that ranged combat is accessible across input devices |  | F-13.10.5 | R-13.10.5 |
| US-13.10.5.3 | player (P-23) | I want to fire projectiles that travel with realistic physics and apply damage effects on impact with VFX feedback so that ranged attacks feel satisfying |  | F-13.10.5 | R-13.10.5 |
| US-13.10.5.4 | engine tester (P-27) | I want to fire a fast projectile through a thin wall and verify CCD prevents tunneling so that fast projectiles collide reliably |  | F-13.10.5 | R-13.10.5 |
| US-13.10.5.5 | engine tester (P-27) | I want to fire a projectile carrying a gameplay effect and verify the effect applies to the hit target so that projectile damage integration works correctly |  | F-13.10.5 | R-13.10.5 |

## F-13.10.6 Hitbox and Hurtbox System

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.10.6.1 | gameplay director (P-3) | I want to author hitbox and hurtbox shapes visually on the skeleton in the animation editor so that hit detection regions are defined without code |  | F-13.10.6 | R-13.10.6 |
| US-13.10.6.2 | gameplay director (P-3) | I want to toggle hitbox and hurtbox visualization in the runtime debug overlay so that I can verify hit detection regions match animations |  | F-13.10.6 | R-13.10.6 |
| US-13.10.6.3 | player (P-23) | I want to hit detection to feel fair in multiplayer with lag compensation so that hits register where my crosshair was aimed regardless of latency |  | F-13.10.6 | R-13.10.6 |
| US-13.10.6.4 | engine tester (P-27) | I want to simulate 150ms latency and verify the server resolves hits against historical entity snapshots at the client's fire time so that lag compensation produces fair results |  | F-13.10.6 | R-13.10.6 |
| US-13.10.6.5 | engine tester (P-27) | I want to play an attack animation and verify hitboxes activate and deactivate at the exact frames specified in the animation events so that hitbox timing is synchronized with animations |  | F-13.10.6 | R-13.10.6 |

## F-13.10.7 Area-of-Effect Targeting

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.10.7.1 | designer (P-5) | I want to place an AoE reticle on the ground that previews the affected area with valid/invalid color feedback so that I can author AoE abilities with clear placement rules |  | F-13.10.7 | R-13.10.7 |
| US-13.10.7.2 | designer (P-5) | I want to configure AoE shapes (cone, sphere, line, ring) with radius and length parameters so that each AoE ability has a distinct area profile |  | F-13.10.7 | R-13.10.7 |
| US-13.10.7.3 | gameplay director (P-3) | I want a per-ability friendly fire toggle that controls whether allies inside the AoE receive damage so that I can balance cooperative and competitive AoE independently |  | F-13.10.7 | R-13.10.7 |
| US-13.10.7.4 | player (P-23) | I want to see a ground reticle showing exactly where my AoE ability will land before I confirm placement so that I can aim area attacks precisely |  | F-13.10.7 | R-13.10.7 |
| US-13.10.7.5 | engine tester (P-27) | I want to place a sphere AoE and verify that only entities within the sphere radius receive the gameplay effect so that shape-based hit detection is accurate |  | F-13.10.7 | R-13.10.7 |

## F-13.10.8 Healing Over Time (HoT)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.10.8.1 | designer (P-5) | I want to configure HoT tick rate, duration, and stacking rules (refresh, stack count, pandemic) per ability so that each healing ability has distinct sustain behavior |  | F-13.10.8 | R-13.10.8 |
| US-13.10.8.2 | gameplay director (P-3) | I want HoT snapshotting to capture caster stats at application time so that heal values remain predictable even after temporary buffs expire |  | F-13.10.8 | R-13.10.8 |
| US-13.10.8.3 | player (P-23) | I want to see active HoT effects as buff icons with duration timers and stack counts in the HUD so that I know how much healing I am receiving over time |  | F-13.10.8 | R-13.10.8 |
| US-13.10.8.4 | engine tester (P-27) | I want to apply a HoT with pandemic stacking and verify that reapplication extends duration by the remaining time up to the configured cap so that pandemic math is correct |  | F-13.10.8 | R-13.10.8 |
| US-13.10.8.5 | engine tester (P-27) | I want to buff the caster, apply a HoT, then remove the buff, and verify all subsequent ticks use the snapshotted stat values so that snapshotting works correctly |  | F-13.10.8 | R-13.10.8 |

## F-13.10.9 Damage Over Time (DoT)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.10.9.1 | designer (P-5) | I want to configure DoT tick rate, duration, damage type tag, and stacking rules per ability so that each damage-over-time effect has distinct behavior |  | F-13.10.9 | R-13.10.9 |
| US-13.10.9.2 | gameplay director (P-3) | I want DoT snapshotting to capture caster stats at application time so that tick damage remains consistent regardless of later stat changes |  | F-13.10.9 | R-13.10.9 |
| US-13.10.9.3 | player (P-23) | I want to see active DoT debuffs with duration, stack count, and damage-per-tick in the HUD so that I know how much periodic damage I am taking |  | F-13.10.9 | R-13.10.9 |
| US-13.10.9.4 | player (P-23) | I want cleanse abilities to remove DoTs matching specific damage type tags so that I can counter poison or fire damage strategically |  | F-13.10.9 | R-13.10.9 |
| US-13.10.9.5 | engine tester (P-27) | I want to apply an ice cleanse to a fire DoT and verify the DoT is removed and the immunity window prevents immediate reapplication so that cleanse interaction works correctly |  | F-13.10.9 | R-13.10.9 |

## F-13.10.10 Status Ailments

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.10.10.1 | designer (P-5) | I want to configure stun duration, post-immunity window, and diminishing returns curve per status ailment so that crowd control is tunable per effect |  | F-13.10.10 | R-13.10.10 |
| US-13.10.10.2 | gameplay director (P-3) | I want diminishing returns to reduce successive status durations by a configurable percentage down to a minimum so that chain-stunning is prevented without removing crowd control entirely |  | F-13.10.10 | R-13.10.10 |
| US-13.10.10.3 | player (P-23) | I want to see status ailment debuff icons with remaining duration on my HUD so that I know when crowd control effects will expire |  | F-13.10.10 | R-13.10.10 |
| US-13.10.10.4 | player (P-23) | I want sleep to break immediately when I take damage so that allies can wake me from sleep by hitting me |  | F-13.10.10 | R-13.10.10 |
| US-13.10.10.5 | engine tester (P-27) | I want to apply stun three times consecutively and verify each application has a shorter duration due to diminishing returns so that DR scaling is correct |  | F-13.10.10 | R-13.10.10 |

## F-13.10.11 Usable Items

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.10.11.1 | player (P-23) | I want to use a healing potion from my inventory quick-slot and see my health restore so that consumable items provide immediate gameplay benefit |  | F-13.10.11 | R-13.10.11 |
| US-13.10.11.2 | player (P-23) | I want potions to share a cooldown group so that I cannot chain-consume multiple potions instantly |  | F-13.10.11 | R-13.10.11 |
| US-13.10.11.3 | designer (P-5) | I want to configure charge count, cooldown, shared cooldown group, use-while-moving flag, and cast time per usable item so that each consumable has distinct usage constraints |  | F-13.10.11 | R-13.10.11 |
| US-13.10.11.4 | gameplay director (P-3) | I want item effects implemented as gameplay effects so that consumables use the same effect pipeline as abilities for consistent balance tuning |  | F-13.10.11 | R-13.10.11 |
| US-13.10.11.5 | engine tester (P-27) | I want to exhaust all charges on a consumable item and verify it is removed from inventory so that charge tracking and consumption work correctly |  | F-13.10.11 | R-13.10.11 |

## F-13.10.12 Ability Conditions

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.10.12.1 | designer (P-5) | I want to configure ability prerequisites (requires weapon type, stance, resource threshold, buff, talent) in the ability editor so that activation gating is data-driven |  | F-13.10.12 | R-13.10.12 |
| US-13.10.12.2 | designer (P-5) | I want to compose conditions with AND/OR/NOT logic so that complex prerequisite rules can be expressed without code |  | F-13.10.12 | R-13.10.12 |
| US-13.10.12.3 | gameplay director (P-3) | I want the UI to display the specific failing condition when an ability cannot be activated so that players understand why an ability is locked |  | F-13.10.12 | R-13.10.12 |
| US-13.10.12.4 | player (P-23) | I want to see a clear message like "Requires a sword" when I try to use an ability without meeting its prerequisites so that I know what to change |  | F-13.10.12 | R-13.10.12 |
| US-13.10.12.5 | engine tester (P-27) | I want to configure an AND condition requiring both a sword and an enrage buff, then test with only one met, and verify the correct failing condition is reported so that condition evaluation and reporting are accurate |  | F-13.10.12 | R-13.10.12 |

## F-13.10.13 Combo System

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.10.13.1 | player (P-23) | I want to chain abilities in sequence within timing windows to execute combo attacks with escalating damage so that skilled play is rewarded |  | F-13.10.13 | R-13.10.13 |
| US-13.10.13.2 | player (P-23) | I want combo point finishers to scale damage based on accumulated combo points so that building up combo points before finishing is strategically rewarding |  | F-13.10.13 | R-13.10.13 |
| US-13.10.13.3 | designer (P-5) | I want to define branching combo paths (light-light-heavy vs. light-light-light) with different finisher abilities so that combos offer meaningful input choices |  | F-13.10.13 | R-13.10.13 |
| US-13.10.13.4 | gameplay director (P-3) | I want configurable timing windows per combo step and combo point generation per step so that I can tune combo difficulty and reward independently |  | F-13.10.13 | R-13.10.13 |
| US-13.10.13.5 | engine tester (P-27) | I want to let the timing window expire mid-combo and verify the combo resets to step 1 so that timing enforcement works correctly |  | F-13.10.13 | R-13.10.13 |
