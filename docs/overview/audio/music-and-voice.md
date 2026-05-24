# Music and Voice

Music playback, layering, transition synchronization, and voice interaction.

## What it covers

- Music composition: multitrack layering (stems) for dynamic mixing.
- Adaptive music: layers activate based on gameplay state (combat, exploration, calm).
- Transition synchronization: music changes synced to beat boundaries.
- Interactive loops: seamless looping with intro, main, fill, and outro segments.
- Vocalization playback: dialogue, barks, grunts, lip-sync timing.
- Voice processing: pitch variation and formant shifting for character voices.
- Ducking music when dialogue plays.
- Music priority: higher-priority tracks override lower-priority tracks.
- Real-time music scheduling: queuing tracks to play after current.
- Regional music zones: background music per location.

## Concepts

### Adaptive Music Architecture

Music organizes into stems: percussion layer, bass layer, harmony layer, and effects layer. Each
stem plays independently; the engine enables or disables stems based on gameplay state. Combat mode
enables all stems; calm exploration disables aggressive percussion. Transitions between states sync
to the next beat or measure, avoiding jarring cuts.

### Sequencing and Loops

Interactive music uses looping segments: intro (non-repeating opening), main (repeating core
section), fill (fill bars or transitions), and outro (ending). The engine loops the main section
indefinitely, inserting fills at specified intervals, and plays outro when exiting. Beat-synced
playback ensures all transitions occur at musical boundaries.

### Voice and Dialogue

Voice playback loads compressed audio files and decodes on-demand. Dialogue scripts organize lines
by speaker, emotion, and context; selection picks an appropriate take. Voice processing applies
pitch variation for distinct character voices (high squeak, low growl) or emotional delivery
(stressed, calm). Lip-sync extracts phoneme timings from voice, driving character mouth animations.

### Ducking and Prioritization

Music reduces volume when dialogue plays (ducking). Dialogue has higher priority; the audio engine
pauses lower-priority music until dialogue finishes. Music priority queuing allows layered soundtracks:
ambience (lowest priority) plays always; music tracks (medium priority) override ambience; stingers
(high priority) interrupt music for dramatic moments.

### Regional Music Zones

Locations define background music zones. Entering a zone starts the zone's music; exiting fades it
out. Nested zones support: a tavern zone contains a tavern-zone sub-region with tavern-specific
music. Smooth crossfades transition between zones.

## How it fits

- See [engine-and-mixing.md](./engine-and-mixing.md) for playback control and mixing.
- See [spatial-audio.md](./spatial-audio.md) for voice positioning in 3D.
- Integrates with [../animation/character-and-first-person.md](../animation/character-and-first-person.md)
  for lip-sync animation timing.
