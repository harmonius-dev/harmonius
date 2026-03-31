# R-13.10 — Abilities and Combat Requirements

## Ability System

1. **R-13.10.1** — The engine **SHALL** define abilities as data-driven assets composed from
   modular, reusable building blocks (activation conditions, resource costs, targeting modes,
   gameplay effects, animation references, VFX/audio cues, AI hints) authored entirely in the visual
   editor.
   - **Rationale:** Modular composition lets designers build abilities by combining shared
     components without programmer involvement, aligning with the no-code philosophy.
   - **Verification:** Author a "Fireball" ability using a "Fire Damage" effect, AoE targeting, and
     a cast animation. Author a "Flame Sword" reusing the same effect with melee targeting. Verify
     both resolve correctly and share the effect component.

2. **R-13.10.2** — The engine **SHALL** support configurable ability activation modes (press, hold,
   charge, combo, toggle) bound to input actions, validating cooldowns, resource costs, and cast
   conditions before execution, with AI agents activating through the same API via synthetic input
   events.
   - **Rationale:** Unified activation ensures player and AI share identical execution paths,
     preventing behavioral divergence.
   - **Verification:** Bind a press ability and verify instant cast. Bind a hold ability and verify
     active only while held. Trigger the same ability from AI and verify identical execution.
     Attempt activation during cooldown and verify rejection.

3. **R-13.10.3** — The engine **SHALL** provide a composable effect system supporting instant,
   duration, periodic, and permanent effects with configurable stacking rules (additive,
   multiplicative, highest-wins, non-stacking), source metadata, damage type tags, and conditional
   tag interactions.
   - **Rationale:** A tag-aware composable effect system enables complex interactions (fire removes
     frozen) without hardcoded logic.
   - **Verification:** Apply two additive bonuses (+10, +20) and verify +30. Apply highest-wins
     buffs (+15%, +25%) and verify only +25%. Apply a periodic heal and verify correct tick count.
     Apply fire damage to a frozen entity and verify tag removal.

## Melee and Ranged Combat

4. **R-13.10.4** — The engine **SHALL** resolve melee hit detection using animation-event-activated
   weapon hitboxes against per-bone hurtboxes with damage multipliers, directional damage, and hit
   reaction triggers.
   - **Rationale:** Tying detection to animation events ensures attacks connect where the weapon
     appears visually.
   - **Verification:** Play a swing; verify hitbox activates only during the attack window. Strike
     head and verify x2 multiplier. Strike from behind and verify flank bonus. Verify hit-stop
     pauses for the configured duration.

5. **R-13.10.5** — The engine **SHALL** simulate projectiles as ECS entities with configurable
   trajectories (linear, arced, homing, beam, spread), physics-driven CCD for fast projectiles,
   effect payloads on impact, and aim-assist for gamepad.
   - **Rationale:** ECS-based projectiles integrate with physics and rendering; configurable
     trajectories cover all ranged archetypes.
   - **Verification:** Spawn linear, arced, and homing projectiles and verify trajectories. Fire a
     fast bullet through thin geometry and verify CCD prevents tunneling. Enable aim assist and
     verify snap-to-target.

6. **R-13.10.6** — The engine **SHALL** provide per-bone hitbox and hurtbox collision shapes with
   per-region damage multipliers, multi-hit prevention per swing, editor visualization, and network
   lag compensation using historical entity snapshots.
   - **Rationale:** Per-bone regions with lag compensation are essential for fair combat in
     networked games.
   - **Verification:** Define head (x2), torso (x1), limb (x0.75) hurtboxes. Strike each and verify
     multipliers. Wide swing same target twice and verify one hit. With 100ms latency, verify
     lag-compensated hits match the attacker's visual frame.

## Advanced Ability Systems

7. **R-13.10.7** — The engine **SHALL** provide ground-targeted AoE placement with a projected
   reticle preview, configurable shapes (cone, sphere, line, ring), maximum range, and a per-ability
   friendly fire toggle.
   - **Rationale:** AoE is fundamental to ability-based games; preview and friendly fire control
     enable both cooperative and competitive design.
   - **Verification:** Place a sphere AoE and verify reticle renders. Place a cone and verify only
     entities within it receive the effect. Toggle friendly fire and verify ally
     inclusion/exclusion.

8. **R-13.10.8** — The engine **SHALL** support periodic effects with configurable tick rate,
   stacking rules (refresh, stack count, pandemic), and caster stat snapshotting at application
   time.
   - **Rationale:** Snapshotting ensures predictable values after buffs expire; stacking rules
     enable depth for healer and DoT gameplay.
   - **Verification:** Apply a periodic effect and verify correct tick count. Test refresh, stack
     count, and pandemic stacking. Buff caster, apply effect, remove buff, verify ticks use
     snapshotted values.

9. **R-13.10.9** — The engine **SHALL** implement named status ailments (stun, slow, root, silence,
   blind, fear, charm, sleep, knockback, pull) with configurable duration, post-immunity window, and
   diminishing returns reducing successive durations.
   - **Rationale:** Diminishing returns prevent frustrating chain-stun without removing crowd
     control entirely.
   - **Verification:** Apply stun and verify inability to act. Apply again immediately and verify
     immunity window blocks it. After immunity, verify reduced duration from DR. Apply sleep and
     deal damage, verify sleep breaks.

10. **R-13.10.10** — The engine **SHALL** support consumable item activation from typed containers
    with configurable charge count, per-item and shared cooldown groups, and item effects
    implemented as gameplay effects.
    - **Rationale:** Consumables bridge inventory and ability systems using the same effect
      pipeline.
    - **Verification:** Use a 3-charge item and verify decrement. Verify shared cooldown prevents
      using another item. Exhaust charges and verify item consumed.

11. **R-13.10.11** — The engine **SHALL** evaluate composable ability conditions (weapon type,
    stance, resource threshold, status, talent, combo state) with AND/OR/NOT logic, reporting the
    specific failing condition to the UI.
    - **Rationale:** Specific failure feedback prevents player confusion when an ability is locked.
    - **Verification:** Require a sword, equip staff, verify "Requires a sword" message. Require 30%
      mana at 20%, verify rejection. Configure AND condition and verify both must be met.

12. **R-13.10.12** — The engine **SHALL** support sequential ability combo chains with per-step
    timing windows, branching paths, combo point generation, and finisher abilities consuming points
    with scaled effects.
    - **Rationale:** Combos reward skilled play with escalating damage in action and fighting games.
    - **Verification:** Define 3-step combo with 1-second windows. Execute all steps and verify
      completion. Let window expire mid-combo and verify reset. Verify branching paths produce
      different finishers.

## Non-Functional Requirements

13. **R-13.10.NF1** — The engine **SHALL** support at least 64 concurrent gameplay effects per
    entity without evaluation exceeding 0.1 ms per entity per frame.
    - **Rationale:** Boss encounters apply dozens of simultaneous effects; the system must not
      bottleneck frame budget.
    - **Verification:** Apply 64 effects to one entity and measure evaluation time. Apply 64 effects
      to 40 entities and verify total under 4 ms.

14. **R-13.10.NF2** — The engine **SHALL** begin ability execution within 1 frame (16.67 ms at 60
    fps) of activation input, excluding network round-trip.
    - **Rationale:** Perceptible delay breaks responsive combat feel.
    - **Verification:** Measure input-to-animation-start across 100 activations and verify all
      within one frame.

15. **R-13.10.NF3** — The engine **SHALL** maintain periodic effect tick timing accuracy within 1 ms
    of the configured interval across all concurrent effects.
    - **Rationale:** Tick drift causes inconsistent total values, breaking balance expectations.
    - **Verification:** Apply a 2-second tick effect and measure intervals over 30 seconds. Verify
      each tick within 1 ms of target under load.
