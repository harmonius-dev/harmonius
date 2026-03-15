# User Stories -- 5.1 Audio Engine

## US-5.1.1 Place Sounds in the World
**As an** audio designer, **I want** to attach point, line, and area sound emitters to any
entity via a lightweight ECS component, **so that** I can populate large open worlds with
hundreds of ambient sources (campfires, waterfalls, NPC chatter) without per-entity overhead
concerns.

## US-5.1.2 Hear from the Right Perspective
**As a** player, **I want** audio to always play from my character's auditory perspective
even in split-screen or spectator mode, **so that** I hear sounds correctly positioned
relative to my camera rather than another player's viewpoint.

## US-5.1.3 Mix Audio by Category
**As an** audio designer, **I want** a hierarchical mixer bus graph with independent volume,
mute, and effect chains for music, SFX, ambient, voice, and UI, **so that** I can control
each category independently and create dynamic mix states like underwater ducking or
pause-menu attenuation.

## US-5.1.4 Never Lose Critical Sounds
**As a** player, **I want** important sounds (boss ability cues, critical alerts) to always
play even when many other sounds are active, **so that** I never miss gameplay-critical audio
feedback during large-scale encounters.

## US-5.1.5 Stream Long Audio Without Memory Spikes
**As an** audio designer, **I want** music and dialogue to stream from disk in small chunks
with prefetch hinting, **so that** long assets play without loading the entire file into
memory and without audible startup latency.

## US-5.1.6 Synchronize Layered Loops Perfectly
**As an** audio designer, **I want** to schedule sound starts, stops, and parameter changes
with sample-accurate timing, **so that** layered loops and musical cues stay perfectly in
sync with each other and with gameplay events.

## US-5.1.7 Support Multiple Audio Formats
**As an** audio designer, **I want** the engine to decode WAV, Vorbis, Opus, and FLAC with
an extensible codec registry, **so that** I can choose the right format for each use case
and add custom codecs via plugins without engine modifications.
