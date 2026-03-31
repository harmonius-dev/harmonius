# User Stories -- 7.8 Tactical Combat AI

## Cover and Positioning

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.8.1.1  | game designer (P-5)     |
| US-7.8.1.2  | game designer (P-5)     |
| US-7.8.1.3  | engine developer (P-26) |
| US-7.8.1.4  | engine developer (P-26) |
| US-7.8.2.1  | game designer (P-5)     |
| US-7.8.2.2  | game designer (P-5)     |
| US-7.8.2.3  | engine developer (P-26) |
| US-7.8.2.4  | engine developer (P-26) |

1. **US-7.8.1.1** -- **As a** game designer (P-5), **I want** to configure cover scoring weights per
   AI archetype, **so that** cautious AI prioritizes protection while aggressive AI prioritizes
   sight lines.

2. **US-7.8.1.2** -- **As a** game designer (P-5), **I want** to see pre-computed cover positions
   with quality scores as debug overlays, **so that** I can verify cover placement in my level.

3. **US-7.8.1.3** -- **As an** engine developer (P-26), **I want** to implement cover scoring that
   evaluates protection angle, sight lines, flanking exposure, target distance, and objective
   proximity, **so that** cover selection is context-aware.

4. **US-7.8.1.4** -- **As an** engine developer (P-26), **I want** to pre-compute cover positions
   from world geometry and cache them in the shared spatial index, **so that** runtime queries are
   fast.

5. **US-7.8.2.1** -- **As a** game designer (P-5), **I want** to configure the minimum squad size
   that triggers flanking behavior, **so that** solo enemies attack directly while groups
   coordinate.

6. **US-7.8.2.2** -- **As a** game designer (P-5), **I want** flankers to wait at staging positions
   until all are ready before attacking simultaneously, **so that** coordinated assaults feel
   planned.

7. **US-7.8.2.3** -- **As an** engine developer (P-26), **I want** the squad leader to designate
   flanking assignments with frontal pressure and flank roles, **so that** coordinated movement
   emerges from role assignment.

8. **US-7.8.2.4** -- **As an** engine developer (P-26), **I want** flanking paths to avoid the
   target's line of sight using geometry occlusion checks, **so that** flankers approach undetected.

## Squad Coordination

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.8.3.1  | game designer (P-5)     |
| US-7.8.3.2  | game designer (P-5)     |
| US-7.8.3.3  | engine developer (P-26) |
| US-7.8.3.4  | engine developer (P-26) |
| US-7.8.4.1  | game designer (P-5)     |
| US-7.8.4.2  | game designer (P-5)     |
| US-7.8.4.3  | engine developer (P-26) |

1. **US-7.8.3.1** -- **As a** game designer (P-5), **I want** to configure which formation shape a
   squad uses per context, **so that** squads use wedge in open terrain and column in corridors.

2. **US-7.8.3.2** -- **As a** game designer (P-5), **I want** to configure squad communication barks
   like target callouts and retreat orders, **so that** squad chatter is audible and informative to
   the player.

3. **US-7.8.3.3** -- **As an** engine developer (P-26), **I want** formation selection based on
   environmental context, **so that** squads automatically adapt their shape to the terrain.

4. **US-7.8.3.4** -- **As an** engine developer (P-26), **I want** a squad communication system that
   sends functional messages and plays bark audio, **so that** communication is both functional for
   AI decisions and presentational for the player.

5. **US-7.8.4.1** -- **As a** game designer (P-5), **I want** to configure suppression zone radius,
   duration, and accuracy penalty per weapon type, **so that** suppression intensity matches the
   weapon.

6. **US-7.8.4.2** -- **As a** game designer (P-5), **I want** AI to use suppressive fire tactically
   while flankers move, **so that** combined fire-and-maneuver forces the player to reposition.

7. **US-7.8.4.3** -- **As an** engine developer (P-26), **I want** suppressive fire to target a zone
   rather than an entity and apply a suppressed debuff to all entities in the zone, **so that** area
   denial is distinct from aimed fire.

## Search and Retreat

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.8.5.1  | game designer (P-5)     |
| US-7.8.5.2  | game designer (P-5)     |
| US-7.8.5.3  | engine developer (P-26) |
| US-7.8.6.1  | game designer (P-5)     |
| US-7.8.6.2  | game designer (P-5)     |
| US-7.8.6.3  | engine developer (P-26) |
| US-7.8.6.4  | engine developer (P-26) |

1. **US-7.8.5.1** -- **As a** game designer (P-5), **I want** to configure search pattern types and
   timeout durations per AI archetype, **so that** search behavior matches the level design and
   enemy type.

2. **US-7.8.5.2** -- **As a** game designer (P-5), **I want** squad members to divide the search
   area among themselves, **so that** coordinated search feels efficient and systematic.

3. **US-7.8.5.3** -- **As an** engine developer (P-26), **I want** search patterns implemented as
   configurable behaviors driven by spatial queries, **so that** search is systematic and covers
   known hiding spots.

4. **US-7.8.6.1** -- **As a** game designer (P-5), **I want** to configure retreat triggers like
   health threshold, cover destroyed, and outnumbered ratio, **so that** retreat behavior matches
   the combat scenario.

5. **US-7.8.6.2** -- **As a** game designer (P-5), **I want** to set a casualty threshold that
   triggers squad-wide retreat, **so that** morale-based fallback makes combat feel dynamic.

6. **US-7.8.6.3** -- **As an** engine developer (P-26), **I want** retreat destinations selected
   using the cover evaluation system, **so that** fallback positions are further from threats and
   tactically sound.

7. **US-7.8.6.4** -- **As an** engine developer (P-26), **I want** retreating agents to use smoke or
   suppressive fire to cover withdrawal when available, **so that** retreat is tactical rather than
   a simple flee.
