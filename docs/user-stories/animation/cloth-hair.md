# User Stories -- 9.5 Cloth and Hair Simulation

## US-9.5.1.1 Simulate Cloaks and Banners With GPU Cloth Physics

**As a** character artist (P-9), **I want** GPU cloth simulation with distance, bending, and
self-collision constraints driven by wind, skeletal animation, and bone collision capsules, **so
that** cloaks, banners, and tabards move dynamically on characters.

## US-9.5.1.2 Author Cloth Constraint Regions Non-Destructively

**As a** technical artist (P-13), **I want** a panel-based authoring model to define constraint
regions for garments without destructive mesh edits, **so that** I can iterate on cloth behavior per
garment quickly.

## US-9.5.1.3 Validate Cloth Disabled on Mobile With Baked Fallback

**As an** engine tester (P-27), **I want** to verify that cloth simulation is disabled on mobile and
uses baked animation fallback, Switch uses distance-only constraints, and desktop runs full PBD
cloth, **so that** cloth cost scales per platform.

## US-9.5.2.1 Simulate Hero Character Hair With Strand Physics

**As a** character artist (P-9), **I want** physics-based strand hair simulation using guide curves
with stretch, bend, and collision constraints, **so that** hero character hair responds to movement,
wind, and gravity with high visual fidelity.

## US-9.5.2.2 Profile Guide Strand Count Impact on Simulation Cost

**As an** engine developer (P-26), **I want** to benchmark strand simulation at 64, 128, and 256
guide strands per character on desktop, **so that** I can set per-character strand budgets that fit
within the animation frame allocation.

## US-9.5.2.3 Verify Strand Hair Available on Desktop Only

**As an** engine tester (P-27), **I want** to confirm that strand-based hair simulation is available
only on desktop, with Switch falling back to card-based and mobile to static shell, **so that** the
feature is correctly platform-gated.

## US-9.5.3.1 Render NPC Hair With Textured Polygon Cards

**As a** character artist (P-9), **I want** card-based hair rendering with alpha blending,
anisotropic specular, and simple spring physics, **so that** NPCs at medium distance have convincing
hair without strand simulation cost.

## US-9.5.3.2 Validate Card Count Per Character Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile uses 8-16 cards per character,
Switch 16-32, and desktop 32-64, **so that** card-based hair overdraw stays within per-platform
budgets.

## US-9.5.4.1 Transition Hair LOD Without Visible Popping

**As a** player (P-23), **I want** hair LOD to transition from strands to simplified clusters to
cards to shell using temporal blending, **so that** hair quality changes are imperceptible as
characters move away.

## US-9.5.4.2 Validate Aggressive LOD Selection on Mobile

**As an** engine tester (P-27), **I want** to verify that mobile starts at card/shell LOD tier while
desktop starts at full strand simulation for hero characters, **so that** LOD tier selection matches
platform capability.

## US-9.5.5.1 Prevent Cloth Penetrating Character Limbs During Combat

**As a** character animator (P-11), **I want** cloth-body collision with capsule and convex hull
proxies that support friction and sticking contacts, **so that** cloth does not pass through arms
and legs during fast combat animations.

## US-9.5.5.2 Validate Collision Proxy Count Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile has 0 collision proxies (cloth
disabled), Switch uses 4-6 capsules, and desktop uses 8-12 capsules plus convex hulls, **so that**
cloth collision complexity matches platform budget.

## US-9.5.6.1 Make Hair Blow in the Wind Coherently With Foliage and Cloth

**As a** character artist (P-9), **I want** hair wind response sampled from the shared wind field
texture, **so that** hair, cloth, foliage, and particles all respond to the same wind direction and
intensity.

## US-9.5.6.2 Validate Wind Response Across LOD Levels

**As an** engine tester (P-27), **I want** to verify that strand-based hair uses per-particle
aerodynamic drag (desktop) and card-based hair uses simplified spring displacement (mobile/Switch),
**so that** wind response is consistent across LOD levels.

## US-9.5.6.3 Preview Hair Wind Behavior in the Animation Editor

**As a** character animator (P-11), **I want** to preview hair wind response in the animation editor
viewport with adjustable wind direction and intensity, **so that** I can tune hair dynamics without
entering play mode.
