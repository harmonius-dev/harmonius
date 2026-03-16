# R-5.4 — Music System Requirements

## Adaptive Layers

### R-5.4.1 Vertical Re-Mixing (Layered Stems)

The engine **SHALL** play multiple synchronized stems for a single music cue and crossfade
individual stem volumes in response to gameplay state. All stems within a cue **SHALL** maintain
sample-level alignment (zero-sample drift) for the duration of playback. Concurrent stem count
**SHALL** scale per tier: 4-6 on mobile, 8 on Switch, and 12+ on desktop.

- **Derived from:**
  [F-5.4.1](../../features/audio/music-system.md)
- **Rationale:** Vertical re-mixing lets a single composed piece scale from quiet exploration to
  full combat intensity by adding or removing layers without jarring track switches.
- **Verification:** Integration test: play a 4-stem cue for 60 seconds and verify all stems remain
  sample- aligned (cross-correlate, peak offset = 0). Fade a stem from 1.0 to 0.0 over 500ms and
  verify the curve is smooth with no clicks.

### R-5.4.2 Horizontal Re-Sequencing

The engine **SHALL** arrange music segments in a directed graph and select the next segment at
runtime based on gameplay conditions. Branch points **SHALL** be quantized to bar or beat boundaries
so transitions always land musically.

- **Derived from:**
  [F-5.4.2](../../features/audio/music-system.md)
- **Rationale:** Horizontal re-sequencing enables seamless zone-crossing music where the score
  reflects biome, threat level, and narrative state without audible cuts.
- **Verification:** Integration test: trigger a transition mid-bar and verify the switch occurs at
  the next bar boundary. Verify segment selection matches the gameplay condition on each transition
  edge.

## Transitions

### R-5.4.3 Transition Rules

The engine **SHALL** support per-edge transition behaviors: immediate cut, timed crossfade,
beat-synced crossfade, next-bar switch, and custom fade curves. Custom fade curves **SHALL** match
the authored curve within 1% tolerance.

- **Derived from:**
  [F-5.4.3](../../features/audio/music-system.md)
- **Rationale:** Musically coherent transitions between exploration, combat, victory, and social
  themes require per-edge control, since different musical contexts demand different transition
  types.
- **Verification:** Integration test: verify timed crossfade overlaps both segments for the
  configured duration. Verify beat-synced crossfades start on a beat boundary. Verify custom fade
  curves match the authored curve within 1% tolerance.

## Tempo and Beat Tracking

### R-5.4.4 Tempo and Beat Clock

The engine **SHALL** maintain a global music clock tracking BPM, time signature, and current
beat/bar position from music segment metadata. The clock **SHALL** publish beat and bar events
consumable by the transition system, stinger scheduler, and gameplay systems. Tempo changes
**SHALL** take effect within 1 ms of the change point.

- **Derived from:**
  [F-5.4.4](../../features/audio/music-system.md)
- **Rationale:** Beat-synced transitions, stingers, and rhythm-synchronized gameplay (UI pulses,
  ability cooldowns) all require a centralized, accurate beat clock.
- **Verification:** Unit test: start a 120 BPM 4/4 segment, count beats over 10 seconds, and verify
  exactly 20 beats and 5 bars. Change tempo to 140 BPM mid-playback and verify subsequent beat
  intervals match the new tempo within 1 ms.

## Stingers

### R-5.4.5 Stinger Playback

The engine **SHALL** trigger short musical stingers on gameplay events, layering on or ducking the
current score. Stingers **SHALL** support beat-quantized or immediate triggering. The engine
**SHALL** enforce per-stinger cooldown timers and priority levels to prevent pile-up. A
higher-priority stinger **SHALL** duck lower-priority stingers.

- **Derived from:**
  [F-5.4.5](../../features/audio/music-system.md)
- **Rationale:** Musical stingers (fanfares, aggro hits) punctuate key gameplay moments. Cooldowns
  and priority prevent pile-up during rapid event sequences in large encounters.
- **Verification:** Unit test: trigger a beat-quantized stinger and verify playback begins on the
  next beat boundary. Trigger two stingers within the cooldown window and verify the second is
  suppressed. Trigger a high-priority stinger while a low-priority one plays and verify the
  low-priority is ducked.

## Playlists

### R-5.4.6 Playlists and Weighted Randomization

The engine **SHALL** organize music cues into playlists with sequential, shuffle, and
weighted-random playback modes. The same track **SHALL NOT** play twice in immediate succession
(non-repeat constraint). Weighted random selection **SHALL** produce proportional distribution
within 10% over 1,000 selections.

- **Derived from:**
  [F-5.4.6](../../features/audio/music-system.md)
- **Rationale:** Music variety across long play sessions reduces listener fatigue. Non-repeat
  constraints prevent immediately repeated tracks.
- **Verification:** Unit test: play through 20 shuffle selections and verify no track appears twice
  in a row. Weight one track 10x and play 1,000 selections; verify proportional selection within
  10%.

## Dynamic Intensity

### R-5.4.7 Dynamic Intensity Parameter

The engine **SHALL** expose a normalized intensity parameter (0.0 to 1.0) that simultaneously drives
vertical stem mixing, horizontal segment selection, and stinger likelihood. The parameter **SHALL**
clamp to [0.0, 1.0] for out-of-range inputs. At intensity 0.0, only the base stem **SHALL** play; at
1.0, all stems **SHALL** be active and high-intensity segments selected.

- **Derived from:**
  [F-5.4.7](../../features/audio/music-system.md)
- **Rationale:** A single authored intensity curve per music cue simplifies designer workflow while
  producing rich adaptive scores across exploration, social, and combat contexts.
- **Verification:** Integration test: set intensity to 0.0 and verify only the base stem plays. Ramp
  to 1.0 and verify all stems fade in and high-intensity segments are selected. Verify the parameter
  clamps to [0.0, 1.0] for out-of-range inputs.

---

## User Stories

## F-5.4.1 Vertical Re-Mixing (Layered Stems)

## US-5.4.1.1 Author Layered Music Cues

**As an** audio designer (P-14), **I want to** compose music cues with multiple synchronized stems
(percussion, strings, brass, choir), **so that** layers can be mixed dynamically.

## US-5.4.1.2 Crossfade Stems Based on Gameplay State

**As an** audio designer (P-14), **I want to** crossfade individual stem volumes in response to
gameplay state changes, **so that** the score scales from exploration to combat intensity.

## US-5.4.1.3 Configure Stem Layers in Editor

**As a** designer (P-5), **I want to** set up stem layers and their gameplay triggers in the visual
editor, **so that** adaptive music is authored without code.

## US-5.4.1.4 Verify Sample-Level Stem Synchronization

**As an** engine tester (P-27), **I want to** play a 4-stem cue for 60 seconds and verify all stems
remain sample-aligned (cross-correlate, peak offset = 0), **so that** synchronization is confirmed.

## US-5.4.1.5 Verify Smooth Stem Fades

**As an** engine tester (P-27), **I want to** fade a stem from 1.0 to 0.0 over 500ms and verify the
curve is smooth with no clicks, **so that** fading is artifact-free.

## US-5.4.1.6 Implement Vertical Re-Mixing System

**As an** engine developer (P-26), **I want to** implement synchronized stem playback with per-stem
volume control, maintaining sample-level alignment, **so that** vertical re-mixing is available.

## US-5.4.1.7 Hear Music Intensity Match Gameplay

**As a** player (P-23), **I want** the music to grow more intense as combat escalates and quieter
during exploration, **so that** the soundtrack dynamically matches the experience.

---

## F-5.4.2 Horizontal Re-Sequencing

## US-5.4.2.1 Author Music Segment Graphs

**As an** audio designer (P-14), **I want to** arrange music segments in a directed graph with
gameplay-driven transitions, **so that** the score follows the player's journey.

## US-5.4.2.2 Configure Branch Points and Conditions

**As an** audio designer (P-14), **I want to** set branch points quantized to bar or beat boundaries
with gameplay conditions on each edge, **so that** transitions are musically valid.

## US-5.4.2.3 Set Up Music Graphs in Editor

**As a** designer (P-5), **I want to** build music segment graphs visually in the editor,
**so that** adaptive music structure is authored without code.

## US-5.4.2.4 Verify Beat-Quantized Branch Points

**As an** engine tester (P-27), **I want to** trigger a transition mid-bar and verify the switch
occurs at the next bar boundary, **so that** beat quantization is correct.

## US-5.4.2.5 Implement Horizontal Re-Sequencing

**As an** engine developer (P-26), **I want to** implement the segment graph player that selects the
next segment based on gameplay conditions and transitions at quantized boundaries, **so that**
horizontal re-sequencing is available.

## US-5.4.2.6 Hear Seamless Music Transitions Between Areas

**As a** player (P-23), **I want** music to transition seamlessly when I move between biomes or
zones, **so that** the score never has jarring cuts.

---

## F-5.4.3 Transition Rules (Crossfade and Beat-Sync)

## US-5.4.3.1 Define Per-Edge Transition Types

**As an** audio designer (P-14), **I want to** assign transition types (immediate cut, timed
crossfade, beat-synced crossfade, next-bar switch, custom curve) per graph edge, **so that** each
transition is musically appropriate.

## US-5.4.3.2 Configure Transition Rules in Editor

**As a** designer (P-5), **I want to** select and configure transition rules per edge in the music
graph editor, **so that** transitions are authored visually.

## US-5.4.3.3 Verify Timed Crossfade Duration

**As an** engine tester (P-27), **I want to** verify timed crossfade overlaps both segments for the
configured duration, **so that** crossfade timing is correct.

## US-5.4.3.4 Verify Beat-Synced Crossfade Alignment

**As an** engine tester (P-27), **I want to** verify beat-synced crossfades start on a beat
boundary, **so that** musical alignment is maintained.

## US-5.4.3.5 Verify Custom Fade Curve Accuracy

**As an** engine tester (P-27), **I want to** verify custom fade curves match the authored curve
within 1% tolerance, **so that** designer intent is preserved.

## US-5.4.3.6 Implement Transition Rule System

**As an** engine developer (P-26), **I want to** implement per-edge transition behaviors with
immediate cut, crossfade, beat-sync, and custom curves, **so that** musically coherent transitions
are available.

## US-5.4.3.7 Hear Smooth Music Transitions

**As a** player (P-23), **I want** music to transition smoothly between themes without jarring cuts
or overlaps, **so that** the score feels professionally produced.

---

## F-5.4.4 Tempo and Beat Clock

## US-5.4.4.1 Configure Tempo Metadata Per Segment

**As an** audio designer (P-14), **I want to** embed BPM and time signature metadata in music
segments, **so that** the beat clock tracks accurately.

## US-5.4.4.2 Use Beat Events in Gameplay

**As a** designer (P-5), **I want to** bind gameplay effects (UI pulses, ability cooldowns) to beat
and bar events, **so that** rhythm-synchronized gameplay is possible.

## US-5.4.4.3 Verify Beat Clock Accuracy

**As an** engine tester (P-27), **I want to** start a 120 BPM 4/4 segment, count beats over 10
seconds, and verify exactly 20 beats and 5 bars, **so that** clock accuracy is confirmed.

## US-5.4.4.4 Verify Tempo Change Tracking

**As an** engine tester (P-27), **I want to** change tempo to 140 BPM mid-playback and verify
subsequent beat intervals match the new tempo within 1ms, **so that** tempo changes are tracked.

## US-5.4.4.5 Implement Beat Clock System

**As an** engine developer (P-26), **I want to** implement the global music clock that tracks BPM,
time signature, and beat/bar position, publishing events for the transition system and gameplay
code, **so that** beat tracking is centralized.

## US-5.4.4.6 Experience Rhythm-Synchronized Gameplay

**As a** player (P-23), **I want** some gameplay elements to pulse or activate in sync with the
music beat, **so that** music and gameplay feel connected.

---

## F-5.4.5 Stinger Playback

## US-5.4.5.1 Trigger Stingers on Gameplay Events

**As an** audio designer (P-14), **I want to** trigger short musical stingers (fanfares, aggro hits)
on gameplay events that layer on or duck the score, **so that** key moments have musical
punctuation.

## US-5.4.5.2 Configure Stinger Cooldowns and Priorities

**As an** audio designer (P-14), **I want to** set cooldown timers and priority levels per stinger,
**so that** rapid events do not pile up stingers.

## US-5.4.5.3 Set Up Stinger Triggers in Editor

**As a** designer (P-5), **I want to** bind stingers to gameplay events in the editor, **so that**
musical accents are data-driven.

## US-5.4.5.4 Verify Beat-Quantized Stinger Triggering

**As an** engine tester (P-27), **I want to** trigger a beat-quantized stinger and verify playback
begins on the next beat boundary, **so that** stinger timing is correct.

## US-5.4.5.5 Verify Cooldown Suppression

**As an** engine tester (P-27), **I want to** trigger two stingers within the cooldown window and
verify the second is suppressed, **so that** pile-up prevention works.

## US-5.4.5.6 Verify Priority Ducking

**As an** engine tester (P-27), **I want to** trigger a high-priority stinger while a low- priority
one plays and verify the low-priority is ducked, **so that** priority ordering works.

## US-5.4.5.7 Implement Stinger Playback System

**As an** engine developer (P-26), **I want to** implement stinger scheduling with beat
quantization, cooldown timers, and priority-based ducking, **so that** musical stingers are
available.

## US-5.4.5.8 Hear Fanfares on Level-Ups and Boss Kills

**As a** player (P-23), **I want** triumphant fanfares to play when I level up or defeat a boss,
**so that** achievements feel musically rewarding.

---

## F-5.4.6 Playlists and Weighted Randomization

## US-5.4.6.1 Create Music Playlists

**As an** audio designer (P-14), **I want to** organize music cues into playlists with sequential,
shuffle, and weighted-random modes, **so that** variety is managed per zone.

## US-5.4.6.2 Configure Non-Repeat Constraints

**As an** audio designer (P-14), **I want** the same track to never play twice in immediate
succession, **so that** listener fatigue is reduced.

## US-5.4.6.3 Set Up Zone Playlists in Editor

**As a** designer (P-5), **I want to** assign playlists to zones in the editor with weighting
parameters, **so that** music variety is authored per area.

## US-5.4.6.4 Verify Non-Repeat Constraint

**As an** engine tester (P-27), **I want to** play through 20 shuffle selections and verify no track
appears twice in a row, **so that** the non-repeat constraint works.

## US-5.4.6.5 Verify Weighted Random Distribution

**As an** engine tester (P-27), **I want to** weight one track 10x and play 1000 selections,
verifying proportional selection within 10%, **so that** weighting is statistically correct.

## US-5.4.6.6 Implement Playlist System

**As an** engine developer (P-26), **I want to** implement playlist playback with sequential,
shuffle, and weighted-random modes with non-repeat constraints, **so that** music variety is
managed.

## US-5.4.6.7 Hear Varied Music During Long Play Sessions

**As a** player (P-23), **I want** to hear different music tracks during long play sessions without
repetition, **so that** the soundtrack stays fresh.

---

## F-5.4.7 Dynamic Intensity Parameter

## US-5.4.7.1 Map Intensity to Stem Mixing

**As an** audio designer (P-14), **I want to** author a single intensity curve per music cue that
drives stem mixing, segment selection, and stinger likelihood, **so that** one parameter controls
the entire adaptive score.

## US-5.4.7.2 Drive Intensity from Gameplay Systems

**As a** designer (P-5), **I want to** connect the intensity parameter to combat threat, proximity,
and narrative tension in the editor, **so that** music responds to gameplay automatically.

## US-5.4.7.3 Verify Intensity Controls All Systems

**As an** engine tester (P-27), **I want to** set intensity to 0.0 and verify only the base stem
plays, ramp to 1.0 and verify all stems fade in and high-intensity segments are selected,
**so that** the parameter drives the full adaptive system.

## US-5.4.7.4 Verify Intensity Clamping

**As an** engine tester (P-27), **I want to** verify the intensity parameter clamps to [0.0, 1.0],
**so that** out-of-range values are handled safely.

## US-5.4.7.5 Implement Dynamic Intensity System

**As an** engine developer (P-26), **I want to** implement the normalized intensity parameter that
simultaneously drives vertical mixing, horizontal sequencing, and stinger probability, **so that**
adaptive scoring is unified.

## US-5.4.7.6 Experience Music Matching Emotional Intensity

**As a** player (P-23), **I want** the music to match the emotional intensity of what is happening
in the game, **so that** the soundtrack enhances the experience.

---

## Non-Functional Requirements

### R-5.4.NF1 Music Transition Latency

The engine **SHALL** execute a music transition within one beat period of the requested transition
point, with no audible gap, click, or phase discontinuity in the output.

- **Derived from:** F-5.4.2, F-5.4.3
- **Rationale:** Late transitions produce musically incoherent results. The system must resolve the
  next valid transition point and execute within one beat.
- **Verification:** Integration test: trigger a bar-quantized transition at a random point within a
  bar. Assert the transition executes at the next bar boundary within +/- 1 sample. Assert no
  clicks, pops, or silence gaps.
