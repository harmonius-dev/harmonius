# User Stories — 9.3 Procedural Animation

## US-9.3.1 Animate Any Creature on Any Terrain Without Authored Clips
**As a** designer, **I want** procedural locomotion that adapts to arbitrary skeleton types
and terrain, with IK, ragdoll blending, and physics-driven movement, **so that** I can place
bipeds, quadrupeds, and fantasy creatures in the world without authoring per-species
animation sets for every surface.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Two-bone, CCD, and FABRIK IK solvers on GPU | F-9.3.1, F-9.3.2, F-9.3.3 | R-9.3.1, R-9.3.2, R-9.3.3 |
| Partial and full ragdoll with per-bone blend | F-9.3.4 | R-9.3.4 |
| Look-at and aim constraints with angle limits | F-9.3.5 | R-9.3.5 |
| Motion matching from pose database | F-9.3.6 | R-9.3.6 |
| Foot placement on uneven terrain via raycast | F-9.3.7 | R-9.3.7 |
| Multi-skeleton gaits (biped, quadruped, hexapod) | F-9.3.8 | R-9.3.8 |
| Physics-based locomotion with balance recovery | F-9.3.9 | R-9.3.9 |
| Runtime attachment and dismemberment via ECS | F-9.3.10 | R-9.3.10 |
| Debug visualization for foot placement and IK | F-9.3.11 | R-9.3.11 |
| Simulation never produces NaN or infinite positions | F-9.3.4 | R-X.11.2 |
