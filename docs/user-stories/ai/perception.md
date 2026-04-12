# User Stories -- 7.6 Perception

## Core Senses

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.6.1.1  | game designer (P-5)     |
| US-7.6.1.2  | game designer (P-5)     |
| US-7.6.1.3  | engine developer (P-26) |
| US-7.6.2.1  | game designer (P-5)     |
| US-7.6.2.2  | engine developer (P-26) |
| US-7.6.3.1  | game designer (P-5)     |
| US-7.6.3.2  | engine developer (P-26) |
| US-7.6.4.1  | game designer (P-5)     |
| US-7.6.4.2  | game developer (P-15)   |

1. **US-7.6.1.1** -- **As a** game designer (P-5), **I want** to configure vision cone range,
   half-angle, and peripheral falloff per NPC archetype, **so that** each enemy type has distinct
   visual awareness.

2. **US-7.6.1.2** -- **As a** game designer (P-5), **I want** line-of-sight raycasts to respect
   configurable trace channels, **so that** glass blocks some NPCs but not others.

3. **US-7.6.1.3** -- **As an** engine developer (P-26), **I want** sight sense to detect stimuli
   within a configurable vision cone with LOS raycast confirmation, **so that** visibility checks
   are physically grounded.

4. **US-7.6.2.1** -- **As a** game designer (P-5), **I want** auditory stimuli to attenuate by
   distance and occlusion, **so that** NPCs distinguish nearby whispers from distant explosions.

5. **US-7.6.2.2** -- **As an** engine developer (P-26), **I want** hearing sense to use spherical
   radius with distance and occlusion attenuation, **so that** sound perception is spatially
   accurate.

6. **US-7.6.3.1** -- **As a** game designer (P-5), **I want** NPCs to always react to incoming
   damage regardless of sight or hearing range, **so that** being attacked produces an immediate
   response.

7. **US-7.6.3.2** -- **As an** engine developer (P-26), **I want** damage sense to register as a
   high-priority perception event bypassing range and LOS checks, **so that** hit reactions are
   never missed.

8. **US-7.6.4.1** -- **As a** game designer (P-5), **I want** perception filters to classify stimuli
   by faction and relationship, **so that** NPCs ignore friendly stimuli and escalate hostile ones.

9. **US-7.6.4.2** -- **As a** game developer (P-15), **I want** faction relationships to be
   modifiable at runtime, **so that** reputation changes and betrayals alter NPC perception
   behavior.

## Stimuli Management

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.6.5.1  | game designer (P-5)     |
| US-7.6.5.2  | engine developer (P-26) |
| US-7.6.6.1  | game designer (P-5)     |
| US-7.6.6.2  | engine developer (P-26) |
| US-7.6.7.1  | game developer (P-15)   |
| US-7.6.7.2  | engine developer (P-26) |

1. **US-7.6.5.1** -- **As a** game designer (P-5), **I want** gameplay systems to broadcast
   perception events with type, intensity, location, radius, and TTL, **so that** the perception
   system queries stimuli spatially.

2. **US-7.6.5.2** -- **As an** engine developer (P-26), **I want** a global stimulus registry that
   expires stale entries automatically, **so that** the active stimulus count stays bounded.

3. **US-7.6.6.1** -- **As a** game designer (P-5), **I want** perception confidence to decay over
   time since last confirmation, **so that** a recently seen enemy has high certainty while old
   stimuli fade.

4. **US-7.6.6.2** -- **As an** engine developer (P-26), **I want** per-archetype memory retention
   tuning, **so that** NPCs with better memory retain stimuli longer than forgetful ones.

5. **US-7.6.7.1** -- **As a** game developer (P-15), **I want** to register project-specific senses
   through a trait interface, **so that** custom senses like tremor or magic detection extend the
   perception system.

6. **US-7.6.7.2** -- **As an** engine developer (P-26), **I want** a per-tick perception budget that
   evaluates high-priority senses first and defers low-priority ones, **so that** perception cost
   stays within platform limits.

## Environmental Awareness

| ID           | Persona                 |
|--------------|-------------------------|
| US-7.6.8.1   | game designer (P-5)     |
| US-7.6.8.2   | game designer (P-5)     |
| US-7.6.8.3   | engine developer (P-26) |
| US-7.6.9.1   | game designer (P-5)     |
| US-7.6.9.2   | level designer (P-6)    |
| US-7.6.9.3   | engine developer (P-26) |
| US-7.6.10.1  | game designer (P-5)     |
| US-7.6.10.2  | game designer (P-5)     |
| US-7.6.10.3  | engine developer (P-26) |
| US-7.6.11.1  | game designer (P-5)     |
| US-7.6.11.2  | engine developer (P-26) |

1. **US-7.6.8.1** -- **As a** game designer (P-5), **I want** scent stimuli that linger and drift
   with wind, **so that** tracking dogs and predators can follow scent trails.

2. **US-7.6.8.2** -- **As a** game designer (P-5), **I want** scent to be blocked by sealed doors
   and diluted by rain, **so that** environmental factors affect tracking difficulty.

3. **US-7.6.8.3** -- **As an** engine developer (P-26), **I want** scent data stored in a spatial
   grid with wind-driven propagation, **so that** scent intensity queries are efficient.

4. **US-7.6.9.1** -- **As a** game designer (P-5), **I want** AI to detect footprints, broken
   vegetation, blood drops, and opened containers, **so that** NPCs can track entities through
   environmental evidence.

5. **US-7.6.9.2** -- **As a** level designer (P-6), **I want** footprints to appear on deformable
   surfaces like snow and mud, **so that** they serve as both visual decals and AI tracking data.

6. **US-7.6.9.3** -- **As an** engine developer (P-26), **I want** evidence entities with type,
   timestamp, source, and decay timer, **so that** AI infers travel direction, recency, and entity
   type from evidence.

7. **US-7.6.10.1** -- **As a** game designer (P-5), **I want** AI to enter investigation behavior
   when perceiving a sub-threshold stimulus, **so that** suspicious sounds or faint scents trigger a
   search response.

8. **US-7.6.10.2** -- **As a** game designer (P-5), **I want** investigation behavior to select a
   pattern based on stimulus type, **so that** sound triggers move-to-origin, scent triggers
   trail-following, and footprints trigger chain-following.

9. **US-7.6.10.3** -- **As an** engine developer (P-26), **I want** investigation to integrate with
   the alert state machine, **so that** unaware agents transition to suspicious during investigation
   and to alerted if evidence confirms a threat.

10. **US-7.6.11.1** -- **As a** game designer (P-5), **I want** AI to track targets using combined
    sight, hearing, scent, and evidence inputs, **so that** the tracker seamlessly switches senses
    as conditions change.

11. **US-7.6.11.2** -- **As an** engine developer (P-26), **I want** tracking confidence to decay
    without stimulus refresh, **so that** the tracker transitions to search patterns when confidence
    reaches zero.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-7.6.1 | game designer (P-5) |
| US-7.6.10 | game designer (P-5) |
| US-7.6.11 | game designer (P-5) |
| US-7.6.2 | game designer (P-5) |
| US-7.6.3 | game designer (P-5) |
| US-7.6.4 | game designer (P-5) |
| US-7.6.5 | game designer (P-5) |
| US-7.6.6 | game designer (P-5) |
| US-7.6.7 | game developer (P-15) |
| US-7.6.8 | game designer (P-5) |
| US-7.6.9 | game designer (P-5) |

1. **US-7.6.1** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.6.1.1 through US-7.6.1.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

2. **US-7.6.10** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.6.10.1 through US-7.6.10.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

3. **US-7.6.11** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.6.11.1 through US-7.6.11.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

4. **US-7.6.2** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.6.2.1 through US-7.6.2.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

5. **US-7.6.3** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.6.3.1 through US-7.6.3.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

6. **US-7.6.4** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.6.4.1 through US-7.6.4.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

7. **US-7.6.5** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.6.5.1 through US-7.6.5.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

8. **US-7.6.6** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.6.6.1 through US-7.6.6.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

9. **US-7.6.7** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-7.6.7.1 through US-7.6.7.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

10. **US-7.6.8** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
US-7.6.8.1 through US-7.6.8.3 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

11. **US-7.6.9** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
US-7.6.9.1 through US-7.6.9.3 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.
