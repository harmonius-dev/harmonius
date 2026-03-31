# User Stories -- Fog of War (13.20)

## Visibility Grid

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.20.1.1  | game designer (P-5)     |
| US-13.20.1.2  | game developer (P-15)   |
| US-13.20.1.3  | player (P-23)           |
| US-13.20.1.4  | player (P-23)           |
| US-13.20.2.1  | game designer (P-5)     |
| US-13.20.2.2  | game developer (P-15)   |
| US-13.20.2.3  | player (P-23)           |
| US-13.20.2.4  | player (P-23)           |

1. **US-13.20.1.1** -- **As a** game designer (P-5), **I want** fog grid resolution configurable
   independently of terrain grid, **so that** I balance visual quality against compute cost.

2. **US-13.20.1.2** -- **As a** game developer (P-15), **I want** fog data compact at 2 bits per
   cell per faction with GPU compute generation and CPU fallback, **so that** fog replicates cheaply
   in multiplayer.

3. **US-13.20.1.3** -- [game-specific] **As a** player (P-23), **I want** unexplored areas fully
   hidden, previously seen areas to show terrain but not entities, and visible areas rendered in
   real time, **so that** scouting is strategic.

4. **US-13.20.1.4** -- [game-specific] **As a** player (P-23), **I want** fog state reflected on the
   minimap for progressive world reveal, **so that** I track my exploration progress.

5. **US-13.20.2.1** -- **As a** game designer (P-5), **I want** per-entity sight radius, shape
   (circle or cone), and height advantage bonus, **so that** I differentiate unit types visually and
   mechanically.

6. **US-13.20.2.2** -- **As a** game developer (P-15), **I want** line-of- sight blocking via the
   shared spatial index so walls and terrain occlude vision, **so that** vision respects world
   geometry.

7. **US-13.20.2.3** -- [game-specific] **As a** player (P-23), **I want** each of my units to reveal
   fog within its sight radius, **so that** I scout the map by moving units.

8. **US-13.20.2.4** -- [game-specific] **As a** player (P-23), **I want** units on high ground to
   have increased vision radius, **so that** elevation provides a strategic scouting advantage.

## Vision Modifiers and Memory

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.20.3.1  | game designer (P-5)     |
| US-13.20.3.2  | game developer (P-15)   |
| US-13.20.3.3  | player (P-23)           |
| US-13.20.3.4  | player (P-23)           |
| US-13.20.4.1  | game designer (P-5)     |
| US-13.20.4.2  | player (P-23)           |
| US-13.20.4.3  | player (P-23)           |

1. **US-13.20.3.1** -- **As a** game designer (P-5), **I want** to place vision modifier volumes
   with configurable types (grass, smoke, high ground, darkness), **so that** maps have tactically
   interesting terrain.

2. **US-13.20.3.2** -- **As a** game developer (P-15), **I want** dynamic modifiers like smoke
   grenades to create temporary vision-blocking volumes that dissipate, **so that** abilities can
   create tactical cover.

3. **US-13.20.3.3** -- [game-specific] **As a** player (P-23), **I want** tall grass patches to hide
   me from enemies outside the grass, **so that** bush positioning is a tactical option.

4. **US-13.20.3.4** -- [game-specific] **As a** player (P-23), **I want** smoke clouds to block all
   vision through them, **so that** smoke creates temporary cover.

5. **US-13.20.4.1** -- **As a** game designer (P-5), **I want** fog memory to be per-faction and
   persisted through saves, **so that** exploration progress carries across sessions.

6. **US-13.20.4.2** -- [game-specific] **As a** player (P-23), **I want** shrouded areas to display
   the last-seen state of buildings and resources as ghost images, **so that** I have context about
   previously explored areas.

7. **US-13.20.4.3** -- [game-specific] **As a** player (P-23), **I want** ghost images to update
   when I re-reveal the area, **so that** destroyed buildings disappear from memory on re-scouting.
