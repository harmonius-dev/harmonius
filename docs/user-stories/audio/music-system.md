# User Stories — 5.4 Music System

## F-5.4.1

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.4.1.1  | audio designer (P-14)   | F-5.4.1  | R-5.4.1      |
| US-5.4.1.2  | audio designer (P-14)   | F-5.4.1  | R-5.4.1      |
| US-5.4.1.3  | engine developer (P-26) | F-5.4.1  | R-5.4.1      |
| US-5.4.1.4  | player (P-23)           | F-5.4.1  | R-5.4.1      |
| US-5.4.1.5  | engine tester (P-27)    | F-5.4.1  | R-5.4.1      |
| US-5.4.1.6  | audio designer (P-14)   | F-5.4.1  | R-5.4.1      |
| US-5.4.1.7  | designer (P-5)          | F-5.4.1  | R-5.4.1      |
| US-5.4.1.8  | engine tester (P-27)    | F-5.4.1  | R-5.4.1      |
| US-5.4.1.9  | player (P-23)           | F-5.4.1  | R-5.4.1      |
| US-5.4.1.10 | engine developer (P-26) | F-5.4.1  | R-5.4.1      |
| US-5.4.1.11 | audio designer (P-14)   | F-5.4.1  | R-5.4.1      |
| US-5.4.1.12 | engine tester (P-27)    | F-5.4.1  | R-5.4.1      |

1. **US-5.4.1.1** — I want to play multiple synchronized stems (percussion, strings, brass, choir)
   for a single cue
   - **Acceptance:** vertical re-mixing is possible
2. **US-5.4.1.2** — I want individual stem volumes to crossfade in response to gameplay intensity
   - **Acceptance:** music scales from quiet to intense
3. **US-5.4.1.3** — I want stems synchronized using sample-accurate scheduling
   - **Acceptance:** layers are perfectly aligned
4. **US-5.4.1.4** — I want music to intensify during combat and calm during exploration
   - **Acceptance:** the score matches the mood
5. **US-5.4.1.5** — I want to confirm mobile supports 4-6 stems, Switch 8, desktop 12+
   - **Acceptance:** budget scales per tier
6. **US-5.4.1.6** — I want layers added and removed with smooth crossfades
   - **Acceptance:** stem changes are inaudible
7. **US-5.4.1.7** — I want to configure stem layout and crossfade curves in the editor
   - **Acceptance:** vertical mixing is visual
8. **US-5.4.1.8** — I want to test stem synchronization with maximum stem count
   - **Acceptance:** sync holds under load
9. **US-5.4.1.9** — I want layer changes to be seamless
   - **Acceptance:** the score never sounds jarring
10. **US-5.4.1.10** — I want mobile to use lower-bitrate stem encoding
    - **Acceptance:** bandwidth is conserved
11. **US-5.4.1.11** — I want stem volume maps driven by intensity
    - **Acceptance:** one parameter controls the full vertical mix
12. **US-5.4.1.12** — I want to verify crossfade timing between stems matches configured curves
    - **Acceptance:** transitions are predictable

## F-5.4.2

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.4.2.1  | audio designer (P-14)   | F-5.4.2  | R-5.4.2      |
| US-5.4.2.2  | engine developer (P-26) | F-5.4.2  | R-5.4.2      |
| US-5.4.2.3  | engine developer (P-26) | F-5.4.2  | R-5.4.2      |
| US-5.4.2.4  | player (P-23)           | F-5.4.2  | R-5.4.2      |
| US-5.4.2.5  | audio designer (P-14)   | F-5.4.2  | R-5.4.2      |
| US-5.4.2.6  | engine tester (P-27)    | F-5.4.2  | R-5.4.2      |
| US-5.4.2.7  | engine developer (P-26) | F-5.4.2  | R-5.4.2      |
| US-5.4.2.8  | designer (P-5)          | F-5.4.2  | R-5.4.2      |
| US-5.4.2.9  | engine tester (P-27)    | F-5.4.2  | R-5.4.2      |
| US-5.4.2.10 | player (P-23)           | F-5.4.2  | R-5.4.2      |
| US-5.4.2.11 | engine developer (P-26) | F-5.4.2  | R-5.4.2      |
| US-5.4.2.12 | audio designer (P-14)   | F-5.4.2  | R-5.4.2      |

1. **US-5.4.2.1** — I want music segments (intro, loop-A, loop-B, transition, outro) in a directed
   graph
   - **Acceptance:** horizontal re-sequencing is possible
2. **US-5.4.2.2** — I want the next segment selected at runtime based on gameplay conditions
   - **Acceptance:** music responds to game state
3. **US-5.4.2.3** — I want branch points quantized to bar or beat boundaries
   - **Acceptance:** transitions land musically
4. **US-5.4.2.4** — I want music to transition seamlessly when crossing zones
   - **Acceptance:** exploration music never cuts awkwardly
5. **US-5.4.2.5** — I want to author the segment graph visually
   - **Acceptance:** music flow is easy to design
6. **US-5.4.2.6** — I want to verify segment graph complexity is uniform across platforms
   - **Acceptance:** music logic is consistent
7. **US-5.4.2.7** — I want prefetch buffer size for the next segment reduced on mobile
   - **Acceptance:** memory is conserved
8. **US-5.4.2.8** — I want to configure gameplay conditions on segment edges
   - **Acceptance:** music branching is data-driven
9. **US-5.4.2.9** — I want to test that branches always land on bar/beat boundaries
   - **Acceptance:** musical integrity is maintained
10. **US-5.4.2.10** — I want music to change when I enter a new biome
    - **Acceptance:** the score reflects the environment
11. **US-5.4.2.11** — I want loopback edges in the segment graph
    - **Acceptance:** repeating sections loop correctly
12. **US-5.4.2.12** — I want distinct segment graphs per biome
    - **Acceptance:** each area has unique musical flow

## F-5.4.3

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.4.3.1  | audio designer (P-14)   | F-5.4.3  | R-5.4.3      |
| US-5.4.3.2  | engine developer (P-26) | F-5.4.3  | R-5.4.3      |
| US-5.4.3.3  | player (P-23)           | F-5.4.3  | R-5.4.3      |
| US-5.4.3.4  | designer (P-5)          | F-5.4.3  | R-5.4.3      |
| US-5.4.3.5  | engine tester (P-27)    | F-5.4.3  | R-5.4.3      |
| US-5.4.3.6  | audio designer (P-14)   | F-5.4.3  | R-5.4.3      |
| US-5.4.3.7  | engine tester (P-27)    | F-5.4.3  | R-5.4.3      |
| US-5.4.3.8  | player (P-23)           | F-5.4.3  | R-5.4.3      |
| US-5.4.3.9  | engine tester (P-27)    | F-5.4.3  | R-5.4.3      |
| US-5.4.3.10 | engine developer (P-26) | F-5.4.3  | R-5.4.3      |
| US-5.4.3.11 | audio designer (P-14)   | F-5.4.3  | R-5.4.3      |
| US-5.4.3.12 | engine tester (P-27)    | F-5.4.3  | R-5.4.3      |

1. **US-5.4.3.1** — I want per-edge transition rules (cut, crossfade, beat-sync, next-bar, custom
   curve)
   - **Acceptance:** each transition sounds right
2. **US-5.4.3.2** — I want beat-synced crossfade transitions
   - **Acceptance:** music shifts land on musical beats
3. **US-5.4.3.3** — I want music transitions to sound musically coherent
   - **Acceptance:** shifts between themes are smooth
4. **US-5.4.3.4** — I want to configure transition rules per graph edge in the editor
   - **Acceptance:** transition behavior is visual
5. **US-5.4.3.5** — I want to trigger transitions at random gameplay moments and verify musical
   coherence
   - **Acceptance:** transitions handle timing variance
6. **US-5.4.3.6** — I want custom fade curves per transition
   - **Acceptance:** unique transition shapes are possible
7. **US-5.4.3.7** — I want to verify mobile uses shortened crossfades
   - **Acceptance:** concurrent stream overlap is reduced
8. **US-5.4.3.8** — I want the shift from combat to exploration music to be smooth
   - **Acceptance:** disengagement feels natural
9. **US-5.4.3.9** — I want to test all transition types (cut, crossfade, beat-sync, next-bar)
   - **Acceptance:** each works correctly
10. **US-5.4.3.10** — I want immediate cut transitions for dramatic moments
    - **Acceptance:** sudden music changes are supported
11. **US-5.4.3.11** — I want crossfade duration configurable per edge
    - **Acceptance:** each transition has appropriate timing
12. **US-5.4.3.12** — I want to verify transition rules are lightweight on all platforms
    - **Acceptance:** music logic has negligible CPU cost

## F-5.4.4

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.4.4.1  | engine developer (P-26) | F-5.4.4  | R-5.4.4      |
| US-5.4.4.2  | engine developer (P-26) | F-5.4.4  | R-5.4.4      |
| US-5.4.4.3  | player (P-23)           | F-5.4.4  | R-5.4.4      |
| US-5.4.4.4  | engine developer (P-26) | F-5.4.4  | R-5.4.4      |
| US-5.4.4.5  | engine tester (P-27)    | F-5.4.4  | R-5.4.4      |
| US-5.4.4.6  | audio designer (P-14)   | F-5.4.4  | R-5.4.4      |
| US-5.4.4.7  | designer (P-5)          | F-5.4.4  | R-5.4.4      |
| US-5.4.4.8  | engine tester (P-27)    | F-5.4.4  | R-5.4.4      |
| US-5.4.4.9  | player (P-23)           | F-5.4.4  | R-5.4.4      |
| US-5.4.4.10 | engine tester (P-27)    | F-5.4.4  | R-5.4.4      |
| US-5.4.4.11 | engine developer (P-26) | F-5.4.4  | R-5.4.4      |
| US-5.4.4.12 | audio designer (P-14)   | F-5.4.4  | R-5.4.4      |

1. **US-5.4.4.1** — I want a global music clock tracking BPM, time signature, and current beat/bar
   - **Acceptance:** musical timing is available
2. **US-5.4.4.2** — I want beat and bar events published from the music clock
   - **Acceptance:** transitions, stingers, and gameplay consume them
3. **US-5.4.4.3** — I want rhythm-synced ability cooldowns and UI pulses
   - **Acceptance:** gameplay feels connected to the music
4. **US-5.4.4.4** — I want beat position derived from the playing segment's metadata
   - **Acceptance:** the clock is accurate
5. **US-5.4.4.5** — I want to verify beat and bar positions are sample-accurate
   - **Acceptance:** musical timing is precise
6. **US-5.4.4.6** — I want BPM and time signature configurable per segment
   - **Acceptance:** tempo changes are supported
7. **US-5.4.4.7** — I want beat events consumable by gameplay systems
   - **Acceptance:** rhythm mechanics are possible
8. **US-5.4.4.8** — I want to test beat clock accuracy across tempo changes
   - **Acceptance:** BPM shifts are handled correctly
9. **US-5.4.4.9** — I want UI effects to pulse with the music beat
   - **Acceptance:** the interface feels connected to the score
10. **US-5.4.4.10** — I want to verify beat clock tracking is lightweight
    - **Acceptance:** no platform scaling is needed
11. **US-5.4.4.11** — I want segments with variable tempo to update the clock
    - **Acceptance:** accelerandos and ritardandos work
12. **US-5.4.4.12** — I want the stinger scheduler to consume beat events
    - **Acceptance:** stingers can be beat-quantized

## F-5.4.5

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.4.5.1  | audio designer (P-14)   | F-5.4.5  | R-5.4.5      |
| US-5.4.5.2  | engine developer (P-26) | F-5.4.5  | R-5.4.5      |
| US-5.4.5.3  | player (P-23)           | F-5.4.5  | R-5.4.5      |
| US-5.4.5.4  | engine developer (P-26) | F-5.4.5  | R-5.4.5      |
| US-5.4.5.5  | engine developer (P-26) | F-5.4.5  | R-5.4.5      |
| US-5.4.5.6  | engine tester (P-27)    | F-5.4.5  | R-5.4.5      |
| US-5.4.5.7  | audio designer (P-14)   | F-5.4.5  | R-5.4.5      |
| US-5.4.5.8  | engine tester (P-27)    | F-5.4.5  | R-5.4.5      |
| US-5.4.5.9  | player (P-23)           | F-5.4.5  | R-5.4.5      |
| US-5.4.5.10 | designer (P-5)          | F-5.4.5  | R-5.4.5      |
| US-5.4.5.11 | engine developer (P-26) | F-5.4.5  | R-5.4.5      |
| US-5.4.5.12 | engine tester (P-27)    | F-5.4.5  | R-5.4.5      |

1. **US-5.4.5.1** — I want short musical phrases triggered on events (level-up, boss aggro, loot
   drops)
   - **Acceptance:** key moments have musical punctuation
2. **US-5.4.5.2** — I want stingers to layer on top of or momentarily duck the score
   - **Acceptance:** stingers enhance without disrupting
3. **US-5.4.5.3** — I want a fanfare when I level up
   - **Acceptance:** progression feels rewarding
4. **US-5.4.5.4** — I want stingers optionally beat-quantized
   - **Acceptance:** musical timing is maintained
5. **US-5.4.5.5** — I want cooldowns and priority rules to prevent stinger pile-up
   - **Acceptance:** rapid triggers do not stack
6. **US-5.4.5.6** — I want to verify stingers draw from the voice pool
   - **Acceptance:** voice budget is respected
7. **US-5.4.5.7** — I want stinger priority configurable
   - **Acceptance:** boss aggro stingers override loot drop stingers
8. **US-5.4.5.8** — I want to confirm mobile uses longer stinger cooldowns
   - **Acceptance:** pile-up prevention is stricter
9. **US-5.4.5.9** — I want a dramatic musical hit when a boss notices me
   - **Acceptance:** the encounter feels epic
10. **US-5.4.5.10** — I want to author stinger triggers and priorities in the editor
    - **Acceptance:** musical events are data-driven
11. **US-5.4.5.11** — I want both immediate and beat-quantized stinger modes
    - **Acceptance:** timing matches the use case
12. **US-5.4.5.12** — I want to trigger many stingers rapidly and verify cooldowns prevent pile-up
    - **Acceptance:** priority logic works

## F-5.4.6

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.4.6.1  | audio designer (P-14)   | F-5.4.6  | R-5.4.6      |
| US-5.4.6.2  | engine developer (P-26) | F-5.4.6  | R-5.4.6      |
| US-5.4.6.3  | player (P-23)           | F-5.4.6  | R-5.4.6      |
| US-5.4.6.4  | engine developer (P-26) | F-5.4.6  | R-5.4.6      |
| US-5.4.6.5  | audio designer (P-14)   | F-5.4.6  | R-5.4.6      |
| US-5.4.6.6  | engine tester (P-27)    | F-5.4.6  | R-5.4.6      |
| US-5.4.6.7  | designer (P-5)          | F-5.4.6  | R-5.4.6      |
| US-5.4.6.8  | engine tester (P-27)    | F-5.4.6  | R-5.4.6      |
| US-5.4.6.9  | player (P-23)           | F-5.4.6  | R-5.4.6      |
| US-5.4.6.10 | engine developer (P-26) | F-5.4.6  | R-5.4.6      |
| US-5.4.6.11 | audio designer (P-14)   | F-5.4.6  | R-5.4.6      |
| US-5.4.6.12 | engine tester (P-27)    | F-5.4.6  | R-5.4.6      |

1. **US-5.4.6.1** — I want music cues organized into playlists with sequential, shuffle, and
   weighted-random modes
   - **Acceptance:** variety is managed
2. **US-5.4.6.2** — I want weighting by time-since-last-play, player location, and gameplay tags
   - **Acceptance:** variety is maximized
3. **US-5.4.6.3** — I want varied music during long play sessions
   - **Acceptance:** the score does not feel repetitive
4. **US-5.4.6.4** — I want non-repeating constraints
   - **Acceptance:** the same track is not heard twice in succession
5. **US-5.4.6.5** — I want to author playlists and weights in the editor
   - **Acceptance:** music scheduling is visual
6. **US-5.4.6.6** — I want to verify playlist logic is lightweight on all platforms
   - **Acceptance:** no scaling is needed
7. **US-5.4.6.7** — I want playlist weights configurable per location
   - **Acceptance:** different areas favor different tracks
8. **US-5.4.6.8** — I want to test weighted random over many plays and verify distribution matches
   weights
   - **Acceptance:** randomization is fair
9. **US-5.4.6.9** — I want music variety each time I play
   - **Acceptance:** returning to the game feels fresh
10. **US-5.4.6.10** — I want sequential playlist mode
    - **Acceptance:** story sequences play in authored order
11. **US-5.4.6.11** — I want tracks tagged by gameplay context
    - **Acceptance:** playlists filter by activity
12. **US-5.4.6.12** — I want to verify the same track never plays twice in a row
    - **Acceptance:** non-repeat logic works

## F-5.4.7

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.4.7.1  | engine developer (P-26) | F-5.4.7  | R-5.4.7      |
| US-5.4.7.2  | designer (P-5)          | F-5.4.7  | R-5.4.7      |
| US-5.4.7.3  | player (P-23)           | F-5.4.7  | R-5.4.7      |
| US-5.4.7.4  | audio designer (P-14)   | F-5.4.7  | R-5.4.7      |
| US-5.4.7.5  | engine tester (P-27)    | F-5.4.7  | R-5.4.7      |
| US-5.4.7.6  | designer (P-5)          | F-5.4.7  | R-5.4.7      |
| US-5.4.7.7  | engine developer (P-26) | F-5.4.7  | R-5.4.7      |
| US-5.4.7.8  | engine developer (P-26) | F-5.4.7  | R-5.4.7      |
| US-5.4.7.9  | player (P-23)           | F-5.4.7  | R-5.4.7      |
| US-5.4.7.10 | engine tester (P-27)    | F-5.4.7  | R-5.4.7      |
| US-5.4.7.11 | engine tester (P-27)    | F-5.4.7  | R-5.4.7      |
| US-5.4.7.12 | audio designer (P-14)   | F-5.4.7  | R-5.4.7      |

1. **US-5.4.7.1** — I want a normalized intensity parameter (0.0-1.0) driving stem mixing, segment
   selection, and stinger likelihood
   - **Acceptance:** one parameter controls the adaptive score
2. **US-5.4.7.2** — I want intensity driven by combat threat, POI proximity, and narrative tension
   - **Acceptance:** music responds to game state
3. **US-5.4.7.3** — I want music to escalate as danger increases
   - **Acceptance:** tension is communicated through the score
4. **US-5.4.7.4** — I want a single authored intensity curve per music cue controlling stems,
   segments, and stingers
   - **Acceptance:** workflow is simplified
5. **US-5.4.7.5** — I want to verify intensity is a single float on all platforms
   - **Acceptance:** overhead is negligible
6. **US-5.4.7.6** — I want to configure intensity source parameters in the editor
   - **Acceptance:** music adaptation is visual
7. **US-5.4.7.7** — I want intensity mapped to stem volumes via authored curves
   - **Acceptance:** vertical mixing responds automatically
8. **US-5.4.7.8** — I want intensity influencing segment selection
   - **Acceptance:** horizontal sequencing adapts to game state
9. **US-5.4.7.9** — I want music to be calm during peaceful exploration
   - **Acceptance:** the score matches the relaxed mood
10. **US-5.4.7.10** — I want to test intensity at 0.0 and 1.0 and verify stem and segment behavior
    - **Acceptance:** edge cases work correctly
11. **US-5.4.7.11** — I want to verify intensity-driven stem responses inherit per-tier stem limits
    - **Acceptance:** budget is respected
12. **US-5.4.7.12** — I want the intensity parameter to control all adaptive aspects
    - **Acceptance:** one curve creates a rich, responsive score
