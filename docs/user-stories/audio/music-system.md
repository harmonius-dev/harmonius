# User Stories — 5.4 Music System

## US-5.4.1.1 Play Synchronized Stems

**As an** audio designer (P-14), **I want to** play multiple synchronized stems (percussion,
strings, brass, choir) for a single cue, **so that** vertical re-mixing is possible.

## US-5.4.1.2 Crossfade Stem Volumes by Gameplay State

**As an** audio designer (P-14), **I want** individual stem volumes to crossfade in response to
gameplay intensity, **so that** music scales from quiet to intense.

## US-5.4.1.3 Implement Stem Synchronization

**As an** engine developer (P-26), **I want** stems synchronized using sample-accurate scheduling,
**so that** layers are perfectly aligned.

## US-5.4.1.4 Hear Music Intensity Match Combat

**As a** player (P-23), **I want** music to intensify during combat and calm during exploration,
**so that** the score matches the mood.

## US-5.4.1.5 Verify Stem Count Per Platform

**As an** engine tester (P-27), **I want to** confirm mobile supports 4-6 stems, Switch 8, desktop
12+, **so that** budget scales per tier.

## US-5.4.1.6 Add and Remove Layers Smoothly

**As an** audio designer (P-14), **I want** layers added and removed with smooth crossfades,
**so that** stem changes are inaudible.

## US-5.4.1.7 Configure Stem Layout in Editor

**As a** designer (P-5), **I want to** configure stem layout and crossfade curves in the editor,
**so that** vertical mixing is visual.

## US-5.4.1.8 Test Stem Sync Under Load

**As an** engine tester (P-27), **I want to** test stem synchronization with maximum stem count,
**so that** sync holds under load.

## US-5.4.1.9 Hear Seamless Layer Transitions

**As a** player (P-23), **I want** layer changes to be seamless, **so that** the score never sounds
jarring.

## US-5.4.1.10 Use Lower Bitrate Stems on Mobile

**As an** engine developer (P-26), **I want** mobile to use lower-bitrate stem encoding, **so that**
bandwidth is conserved.

## US-5.4.1.11 Author Intensity-Driven Stem Maps

**As an** audio designer (P-14), **I want** stem volume maps driven by intensity, **so that** one
parameter controls the full vertical mix.

## US-5.4.1.12 Verify Layer Crossfade Timing

**As an** engine tester (P-27), **I want to** verify crossfade timing between stems matches
configured curves, **so that** transitions are predictable.

## US-5.4.2.1 Arrange Music Segments in Directed Graph

**As an** audio designer (P-14), **I want** music segments (intro, loop-A, loop-B, transition,
outro) in a directed graph, **so that** horizontal re-sequencing is possible.

## US-5.4.2.2 Select Next Segment by Gameplay Conditions

**As an** engine developer (P-26), **I want** the next segment selected at runtime based on gameplay
conditions, **so that** music responds to game state.

## US-5.4.2.3 Quantize Branch Points to Bar Boundaries

**As an** engine developer (P-26), **I want** branch points quantized to bar or beat boundaries,
**so that** transitions land musically.

## US-5.4.2.4 Hear Seamless Music During Zone Crossing

**As a** player (P-23), **I want** music to transition seamlessly when crossing zones, **so that**
exploration music never cuts awkwardly.

## US-5.4.2.5 Author Segment Graph in Visual Editor

**As an** audio designer (P-14), **I want to** author the segment graph visually, **so that** music
flow is easy to design.

## US-5.4.2.6 Verify Uniform Graph Complexity

**As an** engine tester (P-27), **I want to** verify segment graph complexity is uniform across
platforms, **so that** music logic is consistent.

## US-5.4.2.7 Reduce Prefetch Buffer on Mobile

**As an** engine developer (P-26), **I want** prefetch buffer size for the next segment reduced on
mobile, **so that** memory is conserved.

## US-5.4.2.8 Configure Segment Conditions

**As a** designer (P-5), **I want to** configure gameplay conditions on segment edges, **so that**
music branching is data-driven.

## US-5.4.2.9 Test Branch Points at Musical Boundaries

**As an** engine tester (P-27), **I want to** test that branches always land on bar/beat boundaries,
**so that** musical integrity is maintained.

## US-5.4.2.10 Hear Music Reflect Biome Changes

**As a** player (P-23), **I want** music to change when I enter a new biome, **so that** the score
reflects the environment.

## US-5.4.2.11 Support Loopback Edges in Graph

**As an** engine developer (P-26), **I want** loopback edges in the segment graph, **so that**
repeating sections loop correctly.

## US-5.4.2.12 Author Music for Each Biome

**As an** audio designer (P-14), **I want** distinct segment graphs per biome, **so that** each area
has unique musical flow.

## US-5.4.3.1 Define Per-Edge Transition Rules

**As an** audio designer (P-14), **I want** per-edge transition rules (cut, crossfade, beat-sync,
next-bar, custom curve), **so that** each transition sounds right.

## US-5.4.3.2 Implement Beat-Synced Crossfade

**As an** engine developer (P-26), **I want** beat-synced crossfade transitions, **so that** music
shifts land on musical beats.

## US-5.4.3.3 Hear Musically Coherent Transitions

**As a** player (P-23), **I want** music transitions to sound musically coherent, **so that** shifts
between themes are smooth.

## US-5.4.3.4 Configure Transition Rules in Editor

**As a** designer (P-5), **I want to** configure transition rules per graph edge in the editor,
**so that** transition behavior is visual.

## US-5.4.3.5 Verify Transition at Unpredictable Times

**As an** engine tester (P-27), **I want to** trigger transitions at random gameplay moments and
verify musical coherence, **so that** transitions handle timing variance.

## US-5.4.3.6 Author Custom Fade Curves

**As an** audio designer (P-14), **I want** custom fade curves per transition, **so that** unique
transition shapes are possible.

## US-5.4.3.7 Shorten Crossfade on Mobile

**As an** engine tester (P-27), **I want to** verify mobile uses shortened crossfades, **so that**
concurrent stream overlap is reduced.

## US-5.4.3.8 Hear Smooth Combat to Exploration Shift

**As a** player (P-23), **I want** the shift from combat to exploration music to be smooth,
**so that** disengagement feels natural.

## US-5.4.3.9 Test All Transition Types

**As an** engine tester (P-27), **I want to** test all transition types (cut, crossfade, beat-sync,
next-bar), **so that** each works correctly.

## US-5.4.3.10 Implement Immediate Cut Transitions

**As an** engine developer (P-26), **I want** immediate cut transitions for dramatic moments,
**so that** sudden music changes are supported.

## US-5.4.3.11 Configure Crossfade Duration

**As an** audio designer (P-14), **I want** crossfade duration configurable per edge, **so that**
each transition has appropriate timing.

## US-5.4.3.12 Verify Lightweight Transition Logic

**As an** engine tester (P-27), **I want to** verify transition rules are lightweight on all
platforms, **so that** music logic has negligible CPU cost.

## US-5.4.4.1 Maintain Global Music Beat Clock

**As an** engine developer (P-26), **I want** a global music clock tracking BPM, time signature, and
current beat/bar, **so that** musical timing is available.

## US-5.4.4.2 Publish Beat and Bar Events

**As an** engine developer (P-26), **I want** beat and bar events published from the music clock,
**so that** transitions, stingers, and gameplay consume them.

## US-5.4.4.3 Hear Ability Cooldowns Sync to Beat

**As a** player (P-23), **I want** rhythm-synced ability cooldowns and UI pulses, **so that**
gameplay feels connected to the music.

## US-5.4.4.4 Derive Beat From Playing Segment Metadata

**As an** engine developer (P-26), **I want** beat position derived from the playing segment's
metadata, **so that** the clock is accurate.

## US-5.4.4.5 Verify Beat Clock Accuracy

**As an** engine tester (P-27), **I want to** verify beat and bar positions are sample-accurate,
**so that** musical timing is precise.

## US-5.4.4.6 Configure BPM and Time Signature

**As an** audio designer (P-14), **I want** BPM and time signature configurable per segment,
**so that** tempo changes are supported.

## US-5.4.4.7 Use Beat Events in Gameplay

**As a** designer (P-5), **I want** beat events consumable by gameplay systems, **so that** rhythm
mechanics are possible.

## US-5.4.4.8 Test Beat Clock Across Tempo Changes

**As an** engine tester (P-27), **I want to** test beat clock accuracy across tempo changes,
**so that** BPM shifts are handled correctly.

## US-5.4.4.9 Feel Music Pulse in Gameplay

**As a** player (P-23), **I want** UI effects to pulse with the music beat, **so that** the
interface feels connected to the score.

## US-5.4.4.10 Verify Lightweight CPU Cost

**As an** engine tester (P-27), **I want to** verify beat clock tracking is lightweight, **so that**
no platform scaling is needed.

## US-5.4.4.11 Support Variable Tempo Segments

**As an** engine developer (P-26), **I want** segments with variable tempo to update the clock,
**so that** accelerandos and ritardandos work.

## US-5.4.4.12 Subscribe Stinger Scheduler to Beat Events

**As an** audio designer (P-14), **I want** the stinger scheduler to consume beat events,
**so that** stingers can be beat-quantized.

## US-5.4.5.1 Trigger Stingers on Gameplay Events

**As an** audio designer (P-14), **I want** short musical phrases triggered on events (level-up,
boss aggro, loot drops), **so that** key moments have musical punctuation.

## US-5.4.5.2 Layer or Duck Current Score

**As an** engine developer (P-26), **I want** stingers to layer on top of or momentarily duck the
score, **so that** stingers enhance without disrupting.

## US-5.4.5.3 Hear Level-Up Fanfare

**As a** player (P-23), **I want** a fanfare when I level up, **so that** progression feels
rewarding.

## US-5.4.5.4 Quantize Stingers to Beat

**As an** engine developer (P-26), **I want** stingers optionally beat-quantized, **so that**
musical timing is maintained.

## US-5.4.5.5 Prevent Stinger Pile-Up

**As an** engine developer (P-26), **I want** cooldowns and priority rules to prevent stinger
pile-up, **so that** rapid triggers do not stack.

## US-5.4.5.6 Verify Stingers Draw from Voice Pool

**As an** engine tester (P-27), **I want to** verify stingers draw from the voice pool, **so that**
voice budget is respected.

## US-5.4.5.7 Configure Stinger Priority

**As an** audio designer (P-14), **I want** stinger priority configurable, **so that** boss aggro
stingers override loot drop stingers.

## US-5.4.5.8 More Aggressive Pile-Up Prevention on Mobile

**As an** engine tester (P-27), **I want to** confirm mobile uses longer stinger cooldowns,
**so that** pile-up prevention is stricter.

## US-5.4.5.9 Hear Boss Aggro Musical Hit

**As a** player (P-23), **I want** a dramatic musical hit when a boss notices me, **so that** the
encounter feels epic.

## US-5.4.5.10 Author Stingers in Visual Editor

**As a** designer (P-5), **I want to** author stinger triggers and priorities in the editor,
**so that** musical events are data-driven.

## US-5.4.5.11 Support Immediate and Quantized Stingers

**As an** engine developer (P-26), **I want** both immediate and beat-quantized stinger modes,
**so that** timing matches the use case.

## US-5.4.5.12 Test Rapid Stinger Triggers

**As an** engine tester (P-27), **I want to** trigger many stingers rapidly and verify cooldowns
prevent pile-up, **so that** priority logic works.

## US-5.4.6.1 Organize Cues into Playlists

**As an** audio designer (P-14), **I want** music cues organized into playlists with sequential,
shuffle, and weighted-random modes, **so that** variety is managed.

## US-5.4.6.2 Weight by Time-Since-Last-Play

**As an** engine developer (P-26), **I want** weighting by time-since-last-play, player location,
and gameplay tags, **so that** variety is maximized.

## US-5.4.6.3 Hear Varied Music Over Long Sessions

**As a** player (P-23), **I want** varied music during long play sessions, **so that** the score
does not feel repetitive.

## US-5.4.6.4 Prevent Same Track Twice in a Row

**As an** engine developer (P-26), **I want** non-repeating constraints, **so that** the same track
is not heard twice in succession.

## US-5.4.6.5 Author Playlists in Visual Editor

**As an** audio designer (P-14), **I want to** author playlists and weights in the editor,
**so that** music scheduling is visual.

## US-5.4.6.6 Verify Lightweight CPU Cost

**As an** engine tester (P-27), **I want to** verify playlist logic is lightweight on all platforms,
**so that** no scaling is needed.

## US-5.4.6.7 Configure Weights Per Location

**As a** designer (P-5), **I want** playlist weights configurable per location, **so that**
different areas favor different tracks.

## US-5.4.6.8 Test Weighted Random Distribution

**As an** engine tester (P-27), **I want to** test weighted random over many plays and verify
distribution matches weights, **so that** randomization is fair.

## US-5.4.6.9 Hear Fresh Music Each Session

**As a** player (P-23), **I want** music variety each time I play, **so that** returning to the game
feels fresh.

## US-5.4.6.10 Support Sequential Mode for Story

**As an** engine developer (P-26), **I want** sequential playlist mode, **so that** story sequences
play in authored order.

## US-5.4.6.11 Tag Tracks by Gameplay Context

**As an** audio designer (P-14), **I want** tracks tagged by gameplay context, **so that** playlists
filter by activity.

## US-5.4.6.12 Verify Non-Repeating Constraints

**As an** engine tester (P-27), **I want to** verify the same track never plays twice in a row,
**so that** non-repeat logic works.

## US-5.4.7.1 Expose Normalized Intensity Parameter

**As an** engine developer (P-26), **I want** a normalized intensity parameter (0.0-1.0) driving
stem mixing, segment selection, and stinger likelihood, **so that** one parameter controls the
adaptive score.

## US-5.4.7.2 Drive Intensity from Gameplay Systems

**As a** designer (P-5), **I want** intensity driven by combat threat, POI proximity, and narrative
tension, **so that** music responds to game state.

## US-5.4.7.3 Hear Music Escalate with Danger

**As a** player (P-23), **I want** music to escalate as danger increases, **so that** tension is
communicated through the score.

## US-5.4.7.4 Author Intensity Curves Per Cue

**As an** audio designer (P-14), **I want** a single authored intensity curve per music cue
controlling stems, segments, and stingers, **so that** workflow is simplified.

## US-5.4.7.5 Verify Intensity is a Single Float

**As an** engine tester (P-27), **I want to** verify intensity is a single float on all platforms,
**so that** overhead is negligible.

## US-5.4.7.6 Configure Intensity Sources in Editor

**As a** designer (P-5), **I want to** configure intensity source parameters in the editor,
**so that** music adaptation is visual.

## US-5.4.7.7 Map Intensity to Stem Volumes

**As an** engine developer (P-26), **I want** intensity mapped to stem volumes via authored curves,
**so that** vertical mixing responds automatically.

## US-5.4.7.8 Map Intensity to Segment Selection

**As an** engine developer (P-26), **I want** intensity influencing segment selection, **so that**
horizontal sequencing adapts to game state.

## US-5.4.7.9 Hear Calm Music During Exploration

**As a** player (P-23), **I want** music to be calm during peaceful exploration, **so that** the
score matches the relaxed mood.

## US-5.4.7.10 Test Intensity at Extreme Values

**As an** engine tester (P-27), **I want to** test intensity at 0.0 and 1.0 and verify stem and
segment behavior, **so that** edge cases work correctly.

## US-5.4.7.11 Inherit Stem Limits from Platform Tier

**As an** engine tester (P-27), **I want to** verify intensity-driven stem responses inherit
per-tier stem limits, **so that** budget is respected.

## US-5.4.7.12 Author Rich Adaptive Scores

**As an** audio designer (P-14), **I want** the intensity parameter to control all adaptive aspects,
**so that** one curve creates a rich, responsive score.
