# User Stories -- 9.5 Cloth and Hair Simulation

## F-9.5.1

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.5.1.1 | character artist (P-9) | I want GPU cloth simulation with distance, bending, and self-collision constraints driven by wind, skeletal animation, and bone collision capsules | cloaks, banners, and tabards move dynamically on characters | F-9.5.1 | R-9.5.1 |
| US-9.5.1.2 | technical artist (P-13) | I want a panel-based authoring model to define constraint regions for garments without destructive mesh edits | I can iterate on cloth behavior per garment quickly | F-9.5.1 | R-9.5.1 |
| US-9.5.1.3 | engine tester (P-27) | I want verify that cloth simulation is disabled on mobile and uses baked animation fallback, Switch uses distance-only constraints, and desktop runs full PBD cloth | cloth cost scales per platform | F-9.5.1 | R-9.5.1 |

## F-9.5.2

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.5.2.1 | character artist (P-9) | I want physics-based strand hair simulation using guide curves with stretch, bend, and collision constraints | hero character hair responds to movement, wind, and gravity with high visual fidelity | F-9.5.2 | R-9.5.2 |
| US-9.5.2.2 | engine developer (P-26) | I want benchmark strand simulation at 64, 128, and 256 guide strands per character on desktop | I can set per-character strand budgets that fit within the animation frame allocation | F-9.5.2 | R-9.5.2 |
| US-9.5.2.3 | engine tester (P-27) | I want confirm that strand-based hair simulation is available only on desktop, with Switch falling back to card-based and mobile to static shell | the feature is correctly platform-gated | F-9.5.2 | R-9.5.2 |

## F-9.5.3

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.5.3.1 | character artist (P-9) | I want card-based hair rendering with alpha blending, anisotropic specular, and simple spring physics | NPCs at medium distance have convincing hair without strand simulation cost | F-9.5.3 | R-9.5.3 |
| US-9.5.3.2 | engine tester (P-27) | I want verify that mobile uses 8-16 cards per character, Switch 16-32, and desktop 32-64 | card-based hair overdraw stays within per-platform budgets | F-9.5.3 | R-9.5.3 |

## F-9.5.4

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.5.4.1 | player (P-23) | I want hair LOD to transition from strands to simplified clusters to cards to shell using temporal blending | hair quality changes are imperceptible as characters move away | F-9.5.4 | R-9.5.4 |
| US-9.5.4.2 | engine tester (P-27) | I want verify that mobile starts at card/shell LOD tier while desktop starts at full strand simulation for hero characters | LOD tier selection matches platform capability | F-9.5.4 | R-9.5.4 |

## F-9.5.5

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.5.5.1 | character animator (P-11) | I want cloth-body collision with capsule and convex hull proxies that support friction and sticking contacts | cloth does not pass through arms and legs during fast combat animations | F-9.5.5 | R-9.5.5 |
| US-9.5.5.2 | engine tester (P-27) | I want verify that mobile has 0 collision proxies (cloth disabled), Switch uses 4-6 capsules, and desktop uses 8-12 capsules plus convex hulls | cloth collision complexity matches platform budget | F-9.5.5 | R-9.5.5 |

## F-9.5.6

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.5.6.1 | character artist (P-9) | I want hair wind response sampled from the shared wind field texture | hair, cloth, foliage, and particles all respond to the same wind direction and intensity | F-9.5.6 | R-9.5.6 |
| US-9.5.6.2 | engine tester (P-27) | I want verify that strand-based hair uses per-particle aerodynamic drag (desktop) and card-based hair uses simplified spring displacement (mobile/Switch) | wind response is consistent across LOD levels | F-9.5.6 | R-9.5.6 |
| US-9.5.6.3 | character animator (P-11) | I want preview hair wind response in the animation editor viewport with adjustable wind direction and intensity | I can tune hair dynamics without entering play mode | F-9.5.6 | R-9.5.6 |
