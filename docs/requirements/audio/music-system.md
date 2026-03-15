# R-5.4 — Music System Requirements

## Adaptive Layers

### R-5.4.1 Vertical Re-Mixing (Layered Stems)

The engine **SHALL** play multiple synchronized stems for a single music cue and crossfade
individual stem volumes in response to gameplay state changes, maintaining sample-level
synchronization between all stems throughout playback.

- **Derived from:** [F-5.4.1](../../features/audio/music-system.md)
- **Rationale:** Vertical re-mixing lets a single composed piece scale from quiet exploration
  to full combat intensity by adding or removing layers without jarring track switches.
- **Verification:** Unit test: load a cue with 4 stems, start playback, and verify all stems
  remain sample-aligned after 60 seconds (cross-correlate outputs, peak offset = 0 samples).
  Fade one stem from 1.0 to 0.0 over 500 ms and verify the fade curve is smooth with no
  clicks. Verify remaining stems are unaffected during the fade.

### R-5.4.2 Horizontal Re-Sequencing

The engine **SHALL** arrange music segments in a directed graph and select the next segment
at runtime based on gameplay conditions, quantizing branch points to bar or beat boundaries
so that transitions always land on a musically valid position.

- **Derived from:** [F-5.4.2](../../features/audio/music-system.md)
- **Rationale:** Horizontal re-sequencing supports seamless zone-crossing music where the
  score reflects biome, threat level, and narrative state without audible cuts.
- **Verification:** Integration test: construct a 4-segment graph (intro, loop-A, loop-B,
  outro) with bar-quantized branch points. Trigger a transition mid-bar from loop-A to
  loop-B and verify the switch occurs at the next bar boundary. Verify the transition
  point aligns with a beat clock bar event. Trigger a transition at a beat boundary and
  verify immediate switching.

## Transitions

### R-5.4.3 Transition Rules (Crossfade and Beat-Sync)

The engine **SHALL** support per-edge transition behaviors in the music graph including
immediate cut, timed crossfade, beat-synced crossfade, next-bar switch, and custom fade
curves, each selectable per graph edge.

- **Derived from:** [F-5.4.3](../../features/audio/music-system.md)
- **Rationale:** Transition rules ensure musically coherent shifts between themes even when
  gameplay state changes arrive at unpredictable times.
- **Verification:** Integration test: define edges with each transition type between two
  segments. For timed crossfade, verify both segments overlap for the configured duration.
  For beat-synced crossfade, verify the crossfade starts on a beat boundary. For next-bar
  switch, verify the old segment plays to the end of the current bar. For custom curve,
  verify the fade envelope matches the authored curve within 1% tolerance.

## Tempo and Beat Tracking

### R-5.4.4 Tempo and Beat Clock

The engine **SHALL** maintain a global music clock tracking tempo (BPM), time signature, and
current beat/bar position, publishing beat and bar events consumable by the transition
system, stinger scheduler, and gameplay code.

- **Derived from:** [F-5.4.4](../../features/audio/music-system.md)
- **Rationale:** A centralized beat clock enables beat-synchronized transitions, stinger
  quantization, and rhythm-driven gameplay mechanics from a single timing source.
- **Verification:** Unit test: start a music segment at 120 BPM in 4/4 time. Count beat
  events over 10 seconds and verify exactly 20 beats are received. Count bar events and
  verify exactly 5 bars. Change tempo to 140 BPM mid-playback and verify subsequent beat
  intervals match the new tempo within 1 ms tolerance. Verify beat/bar positions reset
  correctly on segment transitions.

## Stingers and One-Shots

### R-5.4.5 Stinger Playback

The engine **SHALL** trigger short musical phrases (stingers) on gameplay events that layer
on top of or duck the current score, supporting beat-quantized and immediate triggering with
per-stinger cooldown timers and priority rules to prevent pile-up.

- **Derived from:** [F-5.4.5](../../features/audio/music-system.md)
- **Rationale:** Stingers provide immediate musical feedback for gameplay events without
  interrupting the score, with pile-up prevention for high-frequency event scenarios.
- **Verification:** Unit test: trigger a beat-quantized stinger and verify playback begins
  on the next beat boundary. Trigger two stingers within the cooldown window and verify
  the second is suppressed. Trigger a high-priority stinger while a low-priority stinger
  is playing and verify the low-priority stinger is ducked. Verify stinger audio mixes
  with the current score output.

## Playlists and Randomization

### R-5.4.6 Playlists and Weighted Randomization

The engine **SHALL** organize music cues into playlists with sequential, shuffle, and
weighted-random playback modes, enforcing a non-repeating constraint so that the same track
is never played twice in immediate succession.

- **Derived from:** [F-5.4.6](../../features/audio/music-system.md)
- **Rationale:** Weighted randomization with non-repeat constraints ensures variety across
  long play sessions and reduces listener fatigue.
- **Verification:** Unit test: create a playlist of 5 tracks in shuffle mode. Play through
  20 selections and verify no track appears twice in a row. Create a weighted-random
  playlist with one track weighted 10x higher than the others. Play 1,000 selections and
  verify the heavily weighted track is selected proportionally (within 10% of expected
  frequency). Verify sequential mode plays tracks in order and loops.

### R-5.4.7 Dynamic Intensity Parameter

The engine **SHALL** expose a normalized intensity parameter (0.0 to 1.0) that simultaneously
controls vertical stem mixing, horizontal segment selection, and stinger likelihood, driven
by gameplay systems via a single authored intensity curve per music cue.

- **Derived from:** [F-5.4.7](../../features/audio/music-system.md)
- **Rationale:** A unified intensity parameter simplifies designer workflow while producing
  rich adaptive scores across exploration, social, and combat contexts.
- **Verification:** Integration test: set intensity to 0.0 and verify only the base stem is
  audible and the low-intensity segment is playing. Ramp intensity to 1.0 over 5 seconds
  and verify all stems fade in according to the authored curve, the segment graph selects
  the high-intensity branch, and stinger trigger probability increases. Verify the intensity
  parameter clamps to [0.0, 1.0].

---

## Non-Functional Requirements

### R-5.4.NF1 Music Transition Latency

The engine **SHALL** execute a music transition (segment switch, stem crossfade, or stinger
trigger) within one beat period of the requested transition point, with no audible gap,
click, or phase discontinuity in the output.

- **Derived from:** [F-5.4.2](../../features/audio/music-system.md),
  [F-5.4.3](../../features/audio/music-system.md)
- **Rationale:** Late transitions produce musically incoherent results. The system must
  resolve the next valid transition point and execute within one beat to maintain musical
  flow.
- **Verification:** Integration test: trigger a bar-quantized transition at a random point
  within a bar. Assert the transition executes at the next bar boundary (within +/- 1 sample).
  Capture the output waveform across the transition and assert no clicks, pops, or silence
  gaps exceeding 0 samples.
