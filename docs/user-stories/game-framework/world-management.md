# User Stories -- World Management (13.2)

## Level Streaming and Partitioning

| ID           | Persona                 |
|--------------|-------------------------|
| US-13.2.1.1  | game designer (P-5)     |
| US-13.2.1.2  | game developer (P-15)   |
| US-13.2.1.3  | player (P-23)           |
| US-13.2.2.1  | game developer (P-15)   |
| US-13.2.2.2  | level designer (P-6)    |
| US-13.2.3.1  | game developer (P-15)   |

1. **US-13.2.1.1** -- **As a** game designer (P-5), **I want** to set per-zone load and unload radii
   with hysteresis margins and priority ordering, **so that** sub-levels stream smoothly.

2. **US-13.2.1.2** -- **As a** game developer (P-15), **I want** level streaming to use Tokio async
   I/O with no blocking calls, **so that** streaming never stalls the game thread.

3. **US-13.2.1.3** -- [game-specific] **As a** player (P-23), **I want** to move through the open
   world without visible loading screens or stalls, **so that** exploration feels continuous.

4. **US-13.2.2.1** -- **As a** game developer (P-15), **I want** the world divided into a
   configurable grid with O(1) cell lookup from world position, **so that** interest management
   scales to thousands of players.

5. **US-13.2.2.2** -- **As a** level designer (P-6), **I want** to set grid cell size per world and
   see cell boundaries overlaid on the viewport, **so that** I place content aware of streaming
   boundaries.

6. **US-13.2.3.1** -- **As a** game developer (P-15), **I want** octree or quadtree partitioning
   supplementing the grid for scenes with extreme vertical extent, **so that** spatial queries
   perform well in all layouts.

## Sub-Levels and Persistence

| ID           | Persona                 |
|--------------|-------------------------|
| US-13.2.4.1  | level designer (P-6)    |
| US-13.2.4.2  | player (P-23)           |
| US-13.2.5.1  | game developer (P-15)   |
| US-13.2.5.2  | level designer (P-6)    |

1. **US-13.2.4.1** -- **As a** level designer (P-6), **I want** to author terrain, buildings,
   interiors, and quest objects as independent sub-levels with conditional visibility, **so that**
   each content layer is edited independently.

2. **US-13.2.4.2** -- [game-specific] **As a** player (P-23), **I want** to see the world change
   based on my quest progress, **so that** my actions visibly affect the game world.

3. **US-13.2.5.1** -- **As a** game developer (P-15), **I want** entities classified as persistent
   (saved to database) or transient (spawned from templates at load), **so that** only entities
   requiring identity survive unload/reload.

4. **US-13.2.5.2** -- **As a** level designer (P-6), **I want** to tag entities as persistent or
   transient in the editor, **so that** I control which entities survive server restarts.

## Time of Day and Weather

| ID           | Persona                 |
|--------------|-------------------------|
| US-13.2.6.1  | game designer (P-5)     |
| US-13.2.6.2  | game developer (P-15)   |
| US-13.2.6.3  | player (P-23)           |
| US-13.2.7.1  | game designer (P-5)     |
| US-13.2.7.2  | game developer (P-15)   |
| US-13.2.7.3  | player (P-23)           |

1. **US-13.2.6.1** -- **As a** game designer (P-5), **I want** to configure real-to-game time ratio,
   sun/moon positions, sky color gradients, and ambient light intensity per zone, **so that** each
   zone has a distinct atmosphere.

2. **US-13.2.6.2** -- **As a** game developer (P-15), **I want** server- authoritative time
   synchronization so all players in a zone see the same sky, **so that** visual desynchronization
   is prevented.

3. **US-13.2.6.3** -- [game-specific] **As a** player (P-23), **I want** smooth transitions between
   dawn, day, dusk, and night with gradual lighting changes, **so that** the time-of-day cycle feels
   natural.

4. **US-13.2.7.1** -- **As a** game designer (P-5), **I want** to define weather states with blend
   transitions and regional overrides, **so that** each zone has weather matching its biome.

5. **US-13.2.7.2** -- **As a** game developer (P-15), **I want** weather state to feed into particle
   emitters, post-processing, audio, and gameplay modifiers, **so that** weather has cross-system
   impact.

6. **US-13.2.7.3** -- [game-specific] **As a** player (P-23), **I want** rain particles, thunder
   ambience, and wet surface reflections during storms, **so that** weather feels immersive and
   multi-sensory.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-13.2.1 | game designer (P-5) |
| US-13.2.2 | game developer (P-15) |
| US-13.2.3 | game developer (P-15) |
| US-13.2.4 | level designer (P-6) |
| US-13.2.5 | game developer (P-15) |
| US-13.2.6 | game designer (P-5) |
| US-13.2.7 | game designer (P-5) |

1. **US-13.2.1** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-13.2.1.1 through US-13.2.1.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

2. **US-13.2.2** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-13.2.2.1 through US-13.2.2.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

3. **US-13.2.3** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-13.2.3.1 through US-13.2.3.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

4. **US-13.2.4** -- **As a** level designer (P-6), **I want** the capabilities defined in
   sub-stories US-13.2.4.1 through US-13.2.4.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

5. **US-13.2.5** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-13.2.5.1 through US-13.2.5.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

6. **US-13.2.6** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-13.2.6.1 through US-13.2.6.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

7. **US-13.2.7** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-13.2.7.1 through US-13.2.7.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.
