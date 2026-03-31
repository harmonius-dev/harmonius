# R-5.4 -- Music System Requirements

## Adaptive Layers

1. **R-5.4.1** — The engine **SHALL** play multiple synchronized stems for a single music cue and
   crossfade individual stem volumes in response to gameplay state. All stems **SHALL** maintain
   sample-level alignment (zero-sample drift). Stem count **SHALL** scale per tier: 4-6 on mobile, 8
   on Switch, 12+ on desktop.
   - **Rationale:** Vertical re-mixing lets a single composed piece scale from quiet exploration to
     full combat without jarring track switches.
   - **Verification:** Integration test: play a 4-stem cue for 60 seconds and verify all stems
     remain sample-aligned (cross-correlate, peak offset = 0). Fade a stem and verify the curve is
     smooth with no clicks.

2. **R-5.4.2** — The engine **SHALL** arrange music segments in a directed graph and select the next
   segment at runtime based on gameplay conditions. Branch points **SHALL** be quantized to bar or
   beat boundaries.
   - **Rationale:** Horizontal re-sequencing enables seamless zone-crossing music reflecting biome,
     threat, and narrative.
   - **Verification:** Integration test: trigger a transition mid-bar and verify the switch occurs
     at the next bar boundary. Verify segment selection matches the gameplay condition on each edge.

## Transitions

3. **R-5.4.3** — The engine **SHALL** support per-edge transition behaviors: immediate cut, timed
   crossfade, beat-synced crossfade, next-bar switch, and custom fade curves. Custom curves
   **SHALL** match the authored curve within 1% tolerance.
   - **Rationale:** Musically coherent transitions require per-edge control since different musical
     contexts demand different transition types.
   - **Verification:** Integration test: verify timed crossfade overlaps both segments for the
     configured duration. Verify beat-synced crossfades start on a beat boundary. Verify custom
     curves match within 1%.

## Tempo and Beat Tracking

4. **R-5.4.4** — The engine **SHALL** maintain a global music clock tracking BPM, time signature,
   and beat/bar position from segment metadata. Beat and bar events **SHALL** be published and
   consumable by the transition system, stinger scheduler, and gameplay systems. Tempo changes
   **SHALL** take effect within 1 ms.
   - **Rationale:** Beat-synced transitions, stingers, and rhythm-synchronized gameplay all require
     a centralized, accurate beat clock.
   - **Verification:** Unit test: start a 120 BPM 4/4 segment, count beats over 10 seconds, and
     verify exactly 20 beats and 5 bars. Change to 140 BPM mid-playback and verify subsequent
     intervals match within 1 ms.

## Stingers

5. **R-5.4.5** — The engine **SHALL** trigger short musical stingers on gameplay events, layering on
   or ducking the score. Stingers **SHALL** support beat-quantized or immediate triggering.
   Per-stinger cooldown timers and priority levels **SHALL** prevent pile-up. Higher-priority
   stingers **SHALL** duck lower-priority ones.
   - **Rationale:** Musical stingers punctuate key gameplay moments. Cooldowns and priority prevent
     pile-up during rapid event sequences.
   - **Verification:** Unit test: trigger a beat-quantized stinger and verify playback begins on the
     next beat. Trigger two within cooldown and verify the second is suppressed. Trigger
     high-priority during low-priority and verify ducking.

## Playlists

6. **R-5.4.6** — The engine **SHALL** organize music cues into playlists with sequential, shuffle,
   and weighted-random modes. The same track **SHALL NOT** play twice in immediate succession.
   Weighted random **SHALL** produce proportional distribution within 10% over 1,000 selections.
   - **Rationale:** Music variety reduces listener fatigue. Non-repeat constraints prevent
     immediately repeated tracks.
   - **Verification:** Unit test: 20 shuffle selections with no back-to-back repeats. Weight one
     track 10x and verify proportional selection within 10% over 1,000 picks.

## Dynamic Intensity

7. **R-5.4.7** — The engine **SHALL** expose a normalized intensity parameter (0.0-1.0)
   simultaneously driving vertical stem mixing, horizontal segment selection, and stinger
   likelihood. The parameter **SHALL** clamp to [0.0, 1.0]. At 0.0 only the base stem plays; at 1.0
   all stems are active.
   - **Rationale:** A single intensity curve per cue simplifies designer workflow while producing
     rich adaptive scores.
   - **Verification:** Integration test: set intensity to 0.0 and verify only the base stem plays.
     Ramp to 1.0 and verify all stems fade in and high-intensity segments are selected. Verify
     clamping for out-of-range inputs.

## Non-Functional Requirements

8. **R-5.4.NF1** — The engine **SHALL** execute a music transition within one beat period of the
   requested point, with no audible gap, click, or phase discontinuity.
   - **Rationale:** Late transitions produce musically incoherent results; the system must resolve
     the next valid point and execute within one beat.
   - **Verification:** Integration test: trigger a bar-quantized transition at a random point within
     a bar. Assert execution at the next bar boundary within +/- 1 sample. Assert no clicks, pops,
     or silence gaps.
