# User Stories — 9.2 Morph Targets

## US-9.2.1 Express Unique Facial Identities Across Hundreds of NPCs
**As an** artist, **I want** GPU-driven blend shape accumulation with facial action units,
corrective shapes, and on-demand morph streaming, **so that** every NPC in a crowded city
displays a distinct face without exhausting GPU memory.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Arbitrary active morph targets per mesh via GPU compute | F-9.2.1 | R-9.2.1 |
| Corrective shapes auto-activate from joint angles | F-9.2.2 | R-9.2.2 |
| Standardized face action units for performance capture | F-9.2.3 | R-9.2.3 |
| Vertex animation textures for zero-CPU playback | F-9.2.4 | R-9.2.4 |
| LRU morph streaming keeps GPU memory bounded | F-9.2.5 | R-9.2.5 |
| No NaN or exploding vertices under any morph weights | F-9.2.1 | R-X.11.2 |
