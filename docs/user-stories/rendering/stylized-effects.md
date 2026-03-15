# User Stories -- 2.11 Stylized and Gameplay Effects

## US-2.11.1.1 Add Selection Outlines to Interactive Objects

**As a** game designer (P-5), **I want** per-entity outlines with configurable color, width,
and style (solid, dashed, animated) driven by ECS components, **so that** interactive objects
are visually highlighted when hovered or selected without custom rendering code.

## US-2.11.1.2 Layer Multiple Outline Types on a Single Entity

**As a** game designer (P-5), **I want** an entity to have a selection outline, a team-color
outline, and a quest-indicator outline simultaneously composited additively, **so that**
multiple gameplay states are communicated visually without conflict.

## US-2.11.1.3 Validate Jump-Flood Outline Fallback to Sobel on Compute-Less Devices

**As an** engine tester (P-27), **I want** to disable compute shaders and verify that the
jump-flood outline algorithm falls back to Sobel edge detection, **so that** outlines render
correctly on devices without compute support.

## US-2.11.2.1 Make Loot Glow by Rarity Tier

**As a** game designer (P-5), **I want** emissive glow highlights driven by a per-entity
HighlightState ECS component with configurable color and intensity per rarity, **so that**
players can identify loot quality at a glance without reading UI labels.

## US-2.11.2.2 Add Pulsing Fresnel Rim Glow to Damageable Enemies

**As a** effects artist (P-12), **I want** fresnel-based rim glow with sinusoidal intensity
pulsing on enemies that flash on damage, **so that** combat feedback is immediate and visible
even in chaotic multi-character fights.

## US-2.11.2.3 Validate Highlight Entity Count Limits Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile limits highlighted entities to
4 with flat color inner glow only, Switch allows 8 with Gaussian blur, and desktop has no limit
with full glow, **so that** highlight rendering scales per platform.

## US-2.11.3.1 Set Up Cel-Shading With Custom Band Ramp Textures

**As a** technical artist (P-13), **I want** to configure cel-shading with per-material ramp
textures controlling band count, thresholds, and colors including hatching patterns for shadow
regions, **so that** I can achieve a specific toon art style per character or environment.

## US-2.11.3.2 Author Toon Specular Highlights as Hard-Edged Shapes

**As a** effects artist (P-12), **I want** specular highlights rendered as hard-edged shapes
(circular, star, cross) with artist-controlled size and threshold, **so that** toon characters
have stylized specular that matches the cel-shaded aesthetic.

## US-2.11.3.3 Validate Toon Shading Band Count on Mobile

**As an** engine tester (P-27), **I want** to verify that mobile uses 2-3 bands with no
hatching/stipple patterns and simplified rim lighting, **so that** toon shading stays within
mobile shader complexity limits.

## US-2.11.4.1 Fade Roofs When Camera Enters an Interior in an Isometric RPG

**As a** game designer (P-5), **I want** automatic roof fading with configurable modes (volume-
based, ray-based, layer-based) when the camera or player moves beneath occluding geometry,
**so that** interiors are visible in isometric and top-down views without manual trigger setup.

## US-2.11.4.2 Dissolve Occluding Geometry With Dithered Transparency

**As a** environment artist (P-8), **I want** fading to use dithered transparency, cross-hatch
dissolve, or smooth alpha with configurable fade speed, **so that** roof removal looks polished
and matches the game's art style.

## US-2.11.4.3 Validate Cut-Through Modes Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile supports only layer-based and
volume-based modes with dither-only dissolve, and desktop supports all modes with smooth alpha,
**so that** cut-through visibility scales per platform.

## US-2.11.5.1 See Allies Through Walls as Colored Silhouettes

**As a** player (P-23), **I want** ally characters rendered as colored silhouettes visible
through walls when they have an XRayVisible component, **so that** I can track teammate
positions in team-based shooters.

## US-2.11.5.2 Configure X-Ray Priority Levels for Different Entity Types

**As a** game designer (P-5), **I want** to set priority levels for x-ray visibility (player
always, allies on toggle, enemies within detection range), **so that** only gameplay-relevant
entities show through walls.

## US-2.11.5.3 Validate X-Ray Entity Count Limits Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile caps x-ray entities at 8 with
flat color only, Switch at 16 with team tint, and desktop has no limit with fresnel outlines,
**so that** x-ray rendering respects per-platform budgets.
