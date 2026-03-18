# User Stories -- 9.5 Cloth and Hair Simulation

## F-9.5.1

| ID         | Persona                 | Features | Requirements |
|------------|-------------------------|----------|--------------|
| US-9.5.1.1 | character artist (P-9)  | F-9.5.1  | R-9.5.1      |
| US-9.5.1.2 | technical artist (P-13) | F-9.5.1  | R-9.5.1      |
| US-9.5.1.3 | engine tester (P-27)    | F-9.5.1  | R-9.5.1      |

1. **US-9.5.1.1** — I want GPU cloth simulation with distance, bending, and self-collision
   constraints driven by wind, skeletal animation, and bone collision capsules
   - **Acceptance:** cloaks, banners, and tabards move dynamically on characters
2. **US-9.5.1.2** — I want a panel-based authoring model to define constraint regions for garments
   without destructive mesh edits
   - **Acceptance:** I can iterate on cloth behavior per garment quickly
3. **US-9.5.1.3** — I want verify that cloth simulation is disabled on mobile and uses baked
   animation fallback, Switch uses distance-only constraints, and desktop runs full PBD cloth
   - **Acceptance:** cloth cost scales per platform

## F-9.5.2

| ID         | Persona                 | Features | Requirements |
|------------|-------------------------|----------|--------------|
| US-9.5.2.1 | character artist (P-9)  | F-9.5.2  | R-9.5.2      |
| US-9.5.2.2 | engine developer (P-26) | F-9.5.2  | R-9.5.2      |
| US-9.5.2.3 | engine tester (P-27)    | F-9.5.2  | R-9.5.2      |

1. **US-9.5.2.1** — I want physics-based strand hair simulation using guide curves with stretch,
   bend, and collision constraints
   - **Acceptance:** hero character hair responds to movement, wind, and gravity with high visual
     fidelity
2. **US-9.5.2.2** — I want benchmark strand simulation at 64, 128, and 256 guide strands per
   character on desktop
   - **Acceptance:** I can set per-character strand budgets that fit within the animation frame
     allocation
3. **US-9.5.2.3** — I want confirm that strand-based hair simulation is available only on desktop,
   with Switch falling back to card-based and mobile to static shell
   - **Acceptance:** the feature is correctly platform-gated

## F-9.5.3

| ID         | Persona                | Features | Requirements |
|------------|------------------------|----------|--------------|
| US-9.5.3.1 | character artist (P-9) | F-9.5.3  | R-9.5.3      |
| US-9.5.3.2 | engine tester (P-27)   | F-9.5.3  | R-9.5.3      |

1. **US-9.5.3.1** — I want card-based hair rendering with alpha blending, anisotropic specular, and
   simple spring physics
   - **Acceptance:** NPCs at medium distance have convincing hair without strand simulation cost
2. **US-9.5.3.2** — I want verify that mobile uses 8-16 cards per character, Switch 16-32, and
   desktop 32-64
   - **Acceptance:** card-based hair overdraw stays within per-platform budgets

## F-9.5.4

| ID         | Persona              | Features | Requirements |
|------------|----------------------|----------|--------------|
| US-9.5.4.1 | player (P-23)        | F-9.5.4  | R-9.5.4      |
| US-9.5.4.2 | engine tester (P-27) | F-9.5.4  | R-9.5.4      |

1. **US-9.5.4.1** — I want hair LOD to transition from strands to simplified clusters to cards to
   shell using temporal blending
   - **Acceptance:** hair quality changes are imperceptible as characters move away
2. **US-9.5.4.2** — I want verify that mobile starts at card/shell LOD tier while desktop starts at
   full strand simulation for hero characters
   - **Acceptance:** LOD tier selection matches platform capability

## F-9.5.5

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.5.5.1 | character animator (P-11) | F-9.5.5  | R-9.5.5      |
| US-9.5.5.2 | engine tester (P-27)      | F-9.5.5  | R-9.5.5      |

1. **US-9.5.5.1** — I want cloth-body collision with capsule and convex hull proxies that support
   friction and sticking contacts
   - **Acceptance:** cloth does not pass through arms and legs during fast combat animations
2. **US-9.5.5.2** — I want verify that mobile has 0 collision proxies (cloth disabled), Switch uses
   4-6 capsules, and desktop uses 8-12 capsules plus convex hulls
   - **Acceptance:** cloth collision complexity matches platform budget

## F-9.5.6

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.5.6.1 | character artist (P-9)    | F-9.5.6  | R-9.5.6      |
| US-9.5.6.2 | engine tester (P-27)      | F-9.5.6  | R-9.5.6      |
| US-9.5.6.3 | character animator (P-11) | F-9.5.6  | R-9.5.6      |

1. **US-9.5.6.1** — I want hair wind response sampled from the shared wind field texture
   - **Acceptance:** hair, cloth, foliage, and particles all respond to the same wind direction and
     intensity
2. **US-9.5.6.2** — I want verify that strand-based hair uses per-particle aerodynamic drag
   (desktop) and card-based hair uses simplified spring displacement (mobile/Switch)
   - **Acceptance:** wind response is consistent across LOD levels
3. **US-9.5.6.3** — I want preview hair wind response in the animation editor viewport with
   adjustable wind direction and intensity
   - **Acceptance:** I can tune hair dynamics without entering play mode
