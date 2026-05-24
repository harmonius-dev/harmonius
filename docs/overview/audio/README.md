# Audio

How the engine plays, places, shapes, and adapts sound.

## Topics

- [engine-and-mixing](./engine-and-mixing.md) — playback, voices, buses, and the mixer.
- [spatial-audio](./spatial-audio.md) — 3D positioning, attenuation, occlusion, and reverb zones.
- [effects-and-dsp](./effects-and-dsp.md) — filters, reverbs, dynamics, and other signal effects.
- [music-and-voice](./music-and-voice.md) — adaptive music, dialogue, lip sync, and speech.

## Key takeaways

- Audio sources route through hierarchical buses (master, dialogue, music, effects), enabling
  ducking and mix control without per-source logic.
- Spatial audio uses HRTF (head-related transfer functions) for binaural rendering on headphones and
  panning for speaker arrays, making 3D sound perception natural.
- Amortized spatial queries (obstruction, distance attenuation) spread over frames reduce per-frame
  cost while maintaining responsiveness via priority queuing (nearby sources update frequently).
- Adaptive music layers stems (percussion, bass, harmony) enabling dynamic transitions synced to
  beat boundaries for seamless mood shifts during gameplay.
- Sidechain compression ducks music when dialogue plays without manual volume automation, keeping
  dialogue intelligible.

## Integration risks

- Impulse reverb convolution (convolver effects) are expensive; misplaced reverbs on every audio
  source cause CPU spike. See [effects-and-dsp.md](./effects-and-dsp.md) for convolution budgeting.
- Amortized spatial queries with low update frequency cause occlusion lag: distant gunshots sound
  unoccluded until next update. See [spatial-audio.md](./spatial-audio.md) for update cadence
  tuning.
- Music layer stem stems must be balanced identically across all music tracks; unbalanced stems
  cause jarring level changes during transitions. See [music-and-voice.md](./music-and-voice.md) for
  normalization guidance.
- Lip-sync timing extracted from dialogue audio drifts if audio is compressed or resampled; frame
  drops cause desync. See [music-and-voice.md](./music-and-voice.md) for robust lip-sync
  integration.
- Sidechain compression threshold tuning affects mix balance: too high threshold means music doesn't
  duck; too low causes constant ducking during speech. See
  [engine-and-mixing.md](./engine-and-mixing.md) for threshold per dialogue type.
