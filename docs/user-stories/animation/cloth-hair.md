# User Stories — 9.5 Cloth & Hair Simulation

## US-9.5.1 Dress Characters in Flowing Cloth and Realistic Hair at Scale
**As an** artist, **I want** GPU cloth simulation, strand-based hair with card LOD fallback,
and wind-driven dynamics, **so that** player characters and key NPCs wear visually rich
garments and hair that respond to movement and weather without manual per-pose authoring.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| GPU cloth sim with distance, bending, self-collision | F-9.5.1 | R-9.5.1 |
| Strand-based hair with guide curves and skinning | F-9.5.2 | R-9.5.2 |
| Card-based hair fallback for medium/far distances | F-9.5.3 | R-9.5.3 |
| Hair LOD transitions from strands to cards to shell | F-9.5.4 | R-9.5.4 |
| Cloth-body collision with friction contacts | F-9.5.5 | R-9.5.5 |
| Hair responds to shared wind field texture | F-9.5.6 | R-9.5.6 |
| No NaN/infinite vertices under extreme conditions | F-9.5.1, F-9.5.2 | R-X.11.2 |
| Wind coherent across cloth, hair, foliage, particles | F-9.5.6 | R-X.9.1 |
