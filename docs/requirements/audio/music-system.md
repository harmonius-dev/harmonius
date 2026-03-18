# R-5.4 -- Music System Requirements

## Adaptive Layers

| ID      | Derived From                                    |
|---------|-------------------------------------------------|
| R-5.4.1 | [F-5.4.1](../../features/audio/music-system.md) |
| R-5.4.2 | [F-5.4.2](../../features/audio/music-system.md) |

1. **R-5.4.1** — The engine **SHALL** play multiple synchronized stems for a single music cue and
   crossfade individual stem volumes in response to gameplay state. All stems within a cue **SHALL**
   maintain sample-level alignment (zero-sample drift) for the duration of playback. Concurrent stem
   count **SHALL** scale per tier: 4-6 on mobile, 8 on Switch, and 12+ on desktop.
   - **Rationale:** Vertical re-mixing lets a single composed piece scale from quiet exploration to
     full combat intensity by adding or removing layers without jarring track switches.
   - **Verification:** Integration test: play a 4-stem cue for 60 seconds and verify all stems
     remain sample-aligned (cross-correlate, peak offset = 0). Fade a stem from 1.0 to 0.0 over
     500ms and verify the curve is smooth with no clicks.
2. **R-5.4.2** — The engine **SHALL** arrange music segments in a directed graph and select the next
   segment at runtime based on gameplay conditions. Branch points **SHALL** be quantized to bar or
   beat boundaries so transitions always land musically.
   - **Rationale:** Horizontal re-sequencing enables seamless zone-crossing music where the score
     reflects biome, threat level, and narrative state without audible cuts.
   - **Verification:** Integration test: trigger a transition mid-bar and verify the switch occurs
     at the next bar boundary. Verify segment selection matches the gameplay condition on each
     transition edge.

## Transitions

| ID      | Derived From                                    |
|---------|-------------------------------------------------|
| R-5.4.3 | [F-5.4.3](../../features/audio/music-system.md) |

1. **R-5.4.3** — The engine **SHALL** support per-edge transition behaviors: immediate cut, timed
   crossfade, beat-synced crossfade, next-bar switch, and custom fade curves. Custom fade curves
   **SHALL** match the authored curve within 1% tolerance.
   - **Rationale:** Musically coherent transitions between exploration, combat, victory, and social
     themes require per-edge control, since different musical contexts demand different transition
     types.
   - **Verification:** Integration test: verify timed crossfade overlaps both segments for the
     configured duration. Verify beat-synced crossfades start on a beat boundary. Verify custom fade
     curves match the authored curve within 1% tolerance.

## Tempo and Beat Tracking

| ID      | Derived From                                    |
|---------|-------------------------------------------------|
| R-5.4.4 | [F-5.4.4](../../features/audio/music-system.md) |

1. **R-5.4.4** — The engine **SHALL** maintain a global music clock tracking BPM, time signature,
   and current beat/bar position from music segment metadata. The clock **SHALL** publish beat and
   bar events consumable by the transition system, stinger scheduler, and gameplay systems. Tempo
   changes **SHALL** take effect within 1 ms of the change point.
   - **Rationale:** Beat-synced transitions, stingers, and rhythm-synchronized gameplay (UI pulses,
     ability cooldowns) all require a centralized, accurate beat clock.
   - **Verification:** Unit test: start a 120 BPM 4/4 segment, count beats over 10 seconds, and
     verify exactly 20 beats and 5 bars. Change tempo to 140 BPM mid-playback and verify subsequent
     beat intervals match the new tempo within 1 ms.

## Stingers

| ID      | Derived From                                    |
|---------|-------------------------------------------------|
| R-5.4.5 | [F-5.4.5](../../features/audio/music-system.md) |

1. **R-5.4.5** — The engine **SHALL** trigger short musical stingers on gameplay events, layering on
   or ducking the current score. Stingers **SHALL** support beat-quantized or immediate triggering.
   The engine **SHALL** enforce per-stinger cooldown timers and priority levels to prevent pile-up.
   A higher-priority stinger **SHALL** duck lower-priority stingers.
   - **Rationale:** Musical stingers (fanfares, aggro hits) punctuate key gameplay moments.
     Cooldowns and priority prevent pile-up during rapid event sequences in large encounters.
   - **Verification:** Unit test: trigger a beat-quantized stinger and verify playback begins on the
     next beat boundary. Trigger two stingers within the cooldown window and verify the second is
     suppressed. Trigger a high-priority stinger while a low-priority one plays and verify the
     low-priority is ducked.

## Playlists

| ID      | Derived From                                    |
|---------|-------------------------------------------------|
| R-5.4.6 | [F-5.4.6](../../features/audio/music-system.md) |

1. **R-5.4.6** — The engine **SHALL** organize music cues into playlists with sequential, shuffle,
   and weighted-random playback modes. The same track **SHALL NOT** play twice in immediate
   succession (non-repeat constraint). Weighted random selection **SHALL** produce proportional
   distribution within 10% over 1,000 selections.
   - **Rationale:** Music variety across long play sessions reduces listener fatigue. Non-repeat
     constraints prevent immediately repeated tracks.
   - **Verification:** Unit test: play through 20 shuffle selections and verify no track appears
     twice in a row. Weight one track 10x and play 1,000 selections; verify proportional selection
     within 10%.

## Dynamic Intensity

| ID      | Derived From                                    |
|---------|-------------------------------------------------|
| R-5.4.7 | [F-5.4.7](../../features/audio/music-system.md) |

1. **R-5.4.7** — The engine **SHALL** expose a normalized intensity parameter (0.0 to 1.0) that
   simultaneously drives vertical stem mixing, horizontal segment selection, and stinger likelihood.
   The parameter **SHALL** clamp to [0.0, 1.0] for out-of-range inputs. At intensity 0.0, only the
   base stem **SHALL** play; at 1.0, all stems **SHALL** be active and high-intensity segments
   selected.
   - **Rationale:** A single authored intensity curve per music cue simplifies designer workflow
     while producing rich adaptive scores across exploration, social, and combat contexts.
   - **Verification:** Integration test: set intensity to 0.0 and verify only the base stem plays.
     Ramp to 1.0 and verify all stems fade in and high-intensity segments are selected. Verify the
     parameter clamps to [0.0, 1.0] for out-of-range inputs.

## Non-Functional Requirements

| ID        | Derived From     |
|-----------|------------------|
| R-5.4.NF1 | F-5.4.2, F-5.4.3 |

1. **R-5.4.NF1** — The engine **SHALL** execute a music transition within one beat period of the
   requested transition point, with no audible gap, click, or phase discontinuity in the output.
   - **Rationale:** Late transitions produce musically incoherent results. The system must resolve
     the next valid transition point and execute within one beat.
   - **Verification:** Integration test: trigger a bar-quantized transition at a random point within
     a bar. Assert the transition executes at the next bar boundary within +/- 1 sample. Assert no
     clicks, pops, or silence gaps.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/audio/music-system.md](../../user-stories/audio/music-system.md).
