# Decals and Screen

Screen-space effects, overlay rendering, and impact decals.

## What it covers

- Decal rendering: projected and mesh-based decals.
- Decal lifetime: fading decals over time.
- Blood and scorch marks: impact-triggered decals.
- Screen-space effects: blood splatter on screen, vignette, scanlines.
- Damage feedback: visual and audio cues on taking damage.
- Hit markers: confirming shots on target.
- Status indicators: health bars, icons, labels.
- Screen shake: camera vibration on impact or explosion.
- Motion lines: speed lines suggesting motion.
- UI overlays: ammo counters, minimap, health HUD.

## Concepts

### Decal Projection

Projected decals render a texture onto nearby surfaces. A decal volume (cuboid) defines projection
bounds; geometry inside projects the decal. Projection direction (typically downward or toward
surface) determines decal orientation. Decals blend with base surface: blood decals appear on walls
and ground; burn marks fade over time.

### Screen-Space Effects

Screen-space effects render to a full-screen overlay layer. Blood splatter effect positions a blood
texture at screen corners, fading out after a few seconds. Vignette darkens screen edges. Scanlines
add a video-feedback aesthetic. Screen shake (camera vibration) applies per-frame camera position
offset that decays over time. These effects apply uniformly across screen without depth awareness.

### Damage Feedback and Indicators

Damage feedback combines visual and audio cues: screen tint flashes red, camera shakes, sound plays.
Hit markers confirm shots on target; different marker types distinguish headshots or critical hits.
UI overlays show health bars (enemies, bosses), ammo counters, and ability cooldowns. Status
indicators show buff/debuff icons.

### Transient Effects

Transient effects are short-lived visual flourishes: motion lines suggest speed; kill feed updates as
enemies die; floating damage numbers rise and fade. These enhance feedback without blocking view.
Carefully positioned and timed, they reinforce gameplay events.

## How it fits

- See [particles-and-effects.md](./particles-and-effects.md) for particle systems underlying
  many effects.
- See [../rendering/effects-and-styles.md](../rendering/effects-and-styles.md) for post-processing
  and color grading.
- See [weather-and-destruction.md](./weather-and-destruction.md) for destruction-triggered
  effects.
