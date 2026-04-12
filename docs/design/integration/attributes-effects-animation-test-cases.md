# Attributes/Effects ↔ Animation Integration Test Cases

All tests below are pure, deterministic, and CI-runnable: they use real `AnimationPlayer`,
`AttributeSet`, `ActiveEffects`, and `TableRegistry` instances with no mocks. Fakes are not used --
the real Phase 3 schedule is driven with a fixed tick harness.

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.5.1.1 | Slow effect halves speed | 0.5x slow mod | `speed = 0.5` | IR-2.5.1 |
| TC-IR-2.5.1.2 | Haste effect doubles speed | 2.0x haste mod | `speed = 2.0` | IR-2.5.1 |
| TC-IR-2.5.1.3 | Stacked speed effects | 0.5x + 1.5x mods | `speed = 0.75` | IR-2.5.1 |
| TC-IR-2.5.1.4 | Speed effect expires | Slow removed | `speed = 1.0` | IR-2.5.1 |
| TC-IR-2.5.1.N1 | Missing speed attr (FM-1) | No attr | `speed = 1.0` | IR-2.5.1 |
| TC-IR-2.5.1.N2 | Zero base value (FM-3) | `base = 0.0` | `speed = 0.0`, warn | IR-2.5.1 |
| TC-IR-2.5.1.N3 | Negative aggregate (FM-2) | Mods sum to -0.3 | `speed = 0.0` | IR-2.5.1 |
| TC-IR-2.5.2.1 | Low health triggers anim | Health drops 20% | Wounded transition | IR-2.5.2 |
| TC-IR-2.5.2.2 | Health recovers above | Health rises 50% | Normal transition | IR-2.5.2 |
| TC-IR-2.5.3.1 | Fatigue drives blend | Fatigue = 0.7 | Tired weight = 0.7 | IR-2.5.3 |
| TC-IR-2.5.3.2 | Zero fatigue normal | Fatigue = 0.0 | Normal weight = 1.0 | IR-2.5.3 |
| TC-IR-2.5.3.3 | Multi-attribute blend | Fatigue 0.4, wet 0.6 | Two layers set | IR-2.5.3 |
| TC-IR-2.5.3.4 | Over-range clamp (FM-7) | Fatigue = 1.8 | Weight = 1.0 | IR-2.5.3 |
| TC-IR-2.5.4.1 | Freeze triggers anim | Freeze applied | Freeze clip plays | IR-2.5.4 |
| TC-IR-2.5.4.2 | Stun triggers anim | Stun applied | Stun clip once | IR-2.5.4 |
| TC-IR-2.5.4.N1 | Missing clip (FM-5) | No clip in def | Skip, warn | IR-2.5.4 |
| TC-IR-2.5.5.1 | Hit frame applies damage | Attack at t=0.4 | Target damaged | IR-2.5.5 |
| TC-IR-2.5.5.2 | Hit on despawned (FM-4) | Target gone | No crash, warn | IR-2.5.5 |
| TC-IR-2.5.5.N1 | Missing ActiveEffects (FM-6) | Env target | Skip, warn | IR-2.5.5 |
| TC-IR-2.5.5.N2 | Unresolved RowRef (FM-5) | Bad `RowRef` | Skip, warn | IR-2.5.5 |
| TC-IR-2.5.5.S1 | Same-tick eval (no delay) | Hit at tick N | Evaluated tick N | IR-2.5.5 |

Detail notes:

1. **TC-IR-2.5.1.N2** -- Sets `AttributeValue::base = 0.0` and asserts `sync_speed_modifiers` writes
   `AnimationPlayer::speed = 0.0` and emits the documented warning via the runtime-toggleable debug
   log sink. Validates the zero-base guard (FM-3).
2. **TC-IR-2.5.3.3** -- Entity carries two attribute values (`fatigue`, `wetness`) and
   `AnimationBlendDescriptor` has two layer entries. After `sync_blend_weights`, both layer weights
   equal their source attribute value. Validates multi-attribute blend coverage.
3. **TC-IR-2.5.3.4** -- Attribute value deliberately set to 1.8 (out of the 0.0..1.0 blend range).
   After `sync_blend_weights`, `ClipEntry::weight` equals exactly 1.0 (clamped, FM-7).
4. **TC-IR-2.5.5.N1** -- Target entity has no `ActiveEffects` component. `anim_event_apply_effects`
   short-circuits, emits the FM-6 warning, and leaves other entities untouched.
5. **TC-IR-2.5.5.S1** -- Asserts the no-delay guarantee from the Timing and Ordering section:
   running a single Phase 3 tick causes `anim_fixed_advance`, `sample_anim_events`,
   `anim_event_apply_effects`, and `effect_eval` to process the hit frame, apply the effect, and
   evaluate the resulting attribute change in that same tick. No state is observable only on the
   next tick.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-2.5.1.B1 | 1000 speed syncs | < 0.1 ms | IR-2.5.1 |
| TC-IR-2.5.2.B1 | 500 threshold checks | < 0.05 ms | IR-2.5.2 |
| TC-IR-2.5.3.B1 | 1000 blend-weight syncs | < 0.15 ms | IR-2.5.3 |
| TC-IR-2.5.4.B1 | 500 one-shot anim dispatches | < 0.15 ms | IR-2.5.4 |
| TC-IR-2.5.5.B1 | 200 anim event effects | < 0.2 ms | IR-2.5.5 |

Detail notes:

1. **TC-IR-2.5.3.B1** -- 1000 entities, each with two attribute-driven blend layers. Measures
   `sync_blend_weights` over a single Phase 3 tick, including clamp branches. Runs under the CI
   benchmark harness with a fixed seed.
2. **TC-IR-2.5.4.B1** -- 500 entities receive `EffectEvent::Applied` in one tick, each triggering a
   one-shot clip dispatch on their `AnimationPlayer`. Measures only the `effect_event_trigger_anim`
   system, not subsequent clip sampling.
