# User Stories -- Fog of War (13.20)

## Fog of War Grid (F-13.20.1)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.20.1.1 | player (P-23) | **As a** player (P-23), **I want** unexplored areas to be fully hidden, previously seen areas to show terrain but not entities, and currently visible areas to render in real time, **so that** scouting and map control are strategic. |  | F-13.20.1 | R-13.20.1 |
| US-13.20.1.2 | player (P-23) | **As a** player (P-23), **I want** fog state reflected on the minimap for progressive world reveal, **so that** I can track my exploration progress. |  | F-13.20.1 | R-13.20.1 |
| US-13.20.1.3 | designer (P-5) | **As a** designer (P-5), **I want** to configure fog grid resolution independently of terrain grid, **so that** I can balance visual quality against compute cost. |  | F-13.20.1 | R-13.20.1 |
| US-13.20.1.4 | designer (P-5) | **As a** designer (P-5), **I want** fog data compact at 2 bits per cell per faction for efficient networking, **so that** fog state replicates cheaply in multiplayer. |  | F-13.20.1 | R-13.20.1 |
| US-13.20.1.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that entities in shrouded areas are not visible to the player, **so that** fog correctly hides unit positions. |  | F-13.20.1 | R-13.20.1 |
| US-13.20.1.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that fog texture generation runs on GPU compute with CPU fallback, **so that** the rendering path works on all devices. |  | F-13.20.1 | R-13.20.1 |

## Vision Source and Sight Radius (F-13.20.2)

| US-13.20.2.1 | player (P-23) | **As a** player (P-23), **I want** each of my units to reveal fog within its sight radius, **so that** I can scout the map by moving units. |  | F-13.20.2 | R-13.20.2 |
| US-13.20.2.2 | player (P-23) | **As a** player (P-23), **I want** units on high ground to have increased vision radius, **so that** elevation provides a strategic scouting advantage. |  | F-13.20.2 | R-13.20.2 |
| US-13.20.2.3 | designer (P-5) | **As a** designer (P-5), **I want** to configure per-entity sight radius, shape (circle or cone), and height advantage bonus, **so that** I can differentiate unit types. |  | F-13.20.2 | R-13.20.2 |
| US-13.20.2.4 | designer (P-5) | **As a** designer (P-5), **I want** walls, terrain, and tagged occluders to block line of sight for vision, **so that** vision respects world geometry. |  | F-13.20.2 | R-13.20.2 |
| US-13.20.2.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that multiple vision sources from the same faction union their revealed areas, **so that** overlapping vision is correctly merged. |  | F-13.20.2 | R-13.20.2 |
| US-13.20.2.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a wall between a unit and a cell blocks vision for that cell, **so that** line-of-sight occlusion is enforced. |  | F-13.20.2 | R-13.20.2 |

## Vision Modifier Volumes (F-13.20.3)

| US-13.20.3.1 | player (P-23) | **As a** player (P-23), **I want** tall grass patches to hide me from enemies outside the grass, **so that** bush positioning is a tactical option. |  | F-13.20.3 | R-13.20.3 |
| US-13.20.3.2 | player (P-23) | **As a** player (P-23), **I want** smoke clouds to block all vision through them, **so that** smoke creates temporary cover. |  | F-13.20.3 | R-13.20.3 |
| US-13.20.3.3 | designer (P-5) | **As a** designer (P-5), **I want** to place vision modifier volumes with configurable types (grass, smoke, high ground, darkness), **so that** maps have tactically interesting terrain. |  | F-13.20.3 | R-13.20.3 |
| US-13.20.3.4 | designer (P-5) | **As a** designer (P-5), **I want** dynamic modifiers like smoke grenades to create temporary vision-blocking volumes that dissipate, **so that** abilities can create tactical cover. |  | F-13.20.3 | R-13.20.3 |
| US-13.20.3.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a unit inside a tall grass volume is invisible to units outside it, **so that** the bush mechanic works correctly. |  | F-13.20.3 | R-13.20.3 |
| US-13.20.3.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that smoke volumes dissipate after the configured duration, **so that** temporary modifiers expire. |  | F-13.20.3 | R-13.20.3 |

## Fog of War Memory (F-13.20.4)

| US-13.20.4.1 | player (P-23) | **As a** player (P-23), **I want** shrouded areas to display the last-seen state of buildings and resources as ghost images, **so that** I have useful context about previously explored areas. |  | F-13.20.4 | R-13.20.4 |
| US-13.20.4.2 | player (P-23) | **As a** player (P-23), **I want** ghost images to update when I re-reveal the area, **so that** destroyed buildings disappear from memory on re-scouting. |  | F-13.20.4 | R-13.20.4 |
| US-13.20.4.3 | designer (P-5) | **As a** designer (P-5), **I want** fog memory to be per-faction and persisted through saves, **so that** exploration progress carries across sessions. |  | F-13.20.4 | R-13.20.4 |
| US-13.20.4.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a building destroyed while shrouded retains its ghost until re-revealed, **so that** memory does not leak real-time state. |  | F-13.20.4 | R-13.20.4 |
| US-13.20.4.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that fog memory survives save and load, **so that** persistence works correctly. |  | F-13.20.4 | R-13.20.4 |
