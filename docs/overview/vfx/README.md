# VFX

Particles, effects, decals, and weather.

## Topics

- [particles-and-effects](./particles-and-effects.md) — particle systems, effect graphs, and
  spawning rules.
- [decals-and-screen](./decals-and-screen.md) — decals, screen-space effects, and screen
  distortion.
- [weather-and-destruction](./weather-and-destruction.md) — rain, snow, wind, and
  destruction effects.

## Key takeaways

- Particle systems with lifetime curves (size, color, velocity) animating per-particle enable
  diverse effects (fire, smoke, sparks) from few emitter variants.
- GPU-driven batch compaction reduces per-effect CPU overhead; individual particle updates shift to
  GPU compute.
- Decal projection volumes avoid dense geometry modification, preserving base meshes while adding
  surface detail (bullet holes, blood, dirt).
- Destruction prefabs blueprint breakage outcomes, spawning debris with physics; destruction state
  persists showing environmental history.
- Particle sorting (depth) ensures transparency compositing correctness; back-to-front rendering
  produces correct blending.

## Integration risks

- Emitter spawn rates and lifetime must be tuned together; too-long particle lifetime combined with
  high spawn rate causes GPU memory pressure. See [particles-and-effects.md](./particles-and-effects.md)
  for budget allocation.
- Decal lifetime management (fading over time) requires coordination with frame rate; frame drops
  cause decals to fade inconsistently. See [decals-and-screen.md](./decals-and-screen.md) for
  age-based fading vs frame-count.
- Destruction debris initial velocity must match impact direction; misaligned vectors cause debris
  flying wrong direction. See [weather-and-destruction.md](./weather-and-destruction.md) for
  impulse direction calculation.
- Weather particle wind forces must synchronize with environment wind zones; mismatched wind causes
  particles and foliage to animate differently. See [weather-and-destruction.md](./weather-and-destruction.md)
  for wind zone integration.
- Screen-space effect intensity scales with resolution; full-screen blood splatter looks different
  on 1080p vs 4K. See [decals-and-screen.md](./decals-and-screen.md) for resolution-aware
  scaling.
