# User Stories — 9.1 Skeletal Animation

## US-9.1.1 Animate Thousands of Characters Simultaneously
**As a** developer, **I want** GPU compute skinning to evaluate skeletal animation for
thousands of characters in a single dispatch, **so that** MMO city hubs and battlegrounds
render with full animation quality without CPU bottlenecks.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| 1000+ skeletons evaluated in one GPU dispatch | F-9.1.1, F-9.1.2, F-9.1.5 | R-9.1.1, R-9.1.2, R-9.1.5 |
| Linear blend and dual-quaternion skinning modes | F-9.1.1 | R-9.1.1 |
| Blend up to 8 clips per skeleton smoothly | F-9.1.3 | R-9.1.3 |
| Layered animation with per-bone masks | F-9.1.4 | R-9.1.4 |
| Root motion drives character world transform | F-9.1.6 | R-9.1.6 |
| 10:1+ compression ratio on humanoid clips | F-9.1.7 | R-9.1.7 |
| Retarget clips across different skeletons | F-9.1.8 | R-9.1.8 |
| Animation events fire through ECS observers | F-9.1.9 | R-9.1.9 |
| LOD tiers reduce cost for distant characters | F-9.1.10 | R-9.1.10 |
| Budget: 1.5ms CPU + 1.0ms GPU for 500 entities | F-9.1.10 | R-X.11.1 |
