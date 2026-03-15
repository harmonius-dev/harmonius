# User Stories -- 2.12 Advanced Materials

## US-2.12.1.1 Author Glass and Crystal Materials With Refraction

**As a** environment artist (P-8), **I want** to create transparent glass materials with
configurable IOR, chromatic dispersion, and Fresnel reflectance, **so that** stained glass
windows and crystal chandeliers refract light realistically without custom shader work.

## US-2.12.1.2 Preview Thin vs Thick Surface Refraction Modes

**As a** character artist (P-9), **I want** to toggle between thin-surface mode (flat glass) and
thick-surface mode (solid gemstones) per material, **so that** I can author jewelry and potion
bottles with correct internal refraction paths.

## US-2.12.1.3 Test Screen-Space Refraction Fallback When RT Is Unavailable

**As an** engine tester (P-27), **I want** to disable RT hardware and verify that transparent
glass falls back to screen-space refraction without visual corruption, **so that** glass
rendering works on all hardware configurations.

## US-2.12.2.1 See Accurate Reflections on Ocean Surfaces

**As a** player (P-23), **I want** ocean water to show Fresnel-weighted reflections of the sky
and shoreline with wave-normal distortion, **so that** the water surface looks believable from
both above and below.

## US-2.12.2.2 Validate Ocean Reflection Quality Scaling by Distance

**As an** engine tester (P-27), **I want** to verify that distant ocean uses low-resolution
reflection probes while near-camera water uses full-resolution planar or RT reflections,
**so that** reflection quality scales appropriately with screen coverage.

## US-2.12.3.1 Create Neon Signs That Trigger Bloom

**As a** environment artist (P-8), **I want** emissive materials with HDR intensity values that
interact with the bloom pass, **so that** neon signs, screens, and magic runes glow brightly
and bleed light into surrounding areas.

## US-2.12.3.2 Animate Emission for Pulsing Magic Effects

**As a** effects artist (P-12), **I want** to animate emission intensity with scrolling UV,
pulsing curves, and color cycling driven by material parameters, **so that** I can create living
surfaces like lava flows and enchanted weapons without per-frame texture swaps.

## US-2.12.3.3 Test Emissive GI Contribution on RT-Capable Hardware

**As an** engine developer (P-26), **I want** to verify that emissive surfaces contribute
indirect lighting onto nearby surfaces when RT GI is active, **so that** a glowing furnace
casts colored light on surrounding walls as expected.

## US-2.12.4.1 Add Geometric Detail to Cobblestone Using Tessellation

**As a** environment artist (P-8), **I want** GPU tessellation driven by heightmap textures to
add real geometric detail to flat cobblestone and brick surfaces, **so that** close-up views
show actual depth and parallax rather than a flat normal-mapped approximation.

## US-2.12.4.2 Fall Back to Parallax Occlusion Mapping on Budget-Limited Platforms

**As a** technical artist (P-13), **I want** to choose between tessellation and POM per material
based on platform budget, **so that** surfaces still show convincing depth on platforms where
tessellation is too expensive.

## US-2.12.4.3 Verify Adaptive Tessellation Factors by Camera Distance

**As an** engine tester (P-27), **I want** to confirm that tessellation density increases near
the camera and drops to minimal subdivision at distance, **so that** tessellation cost is
proportional to screen coverage.

## US-2.12.5.1 Author Velvet and Silk With Sheen BRDF

**As a** character artist (P-9), **I want** fabric materials with a sheen BRDF layer and thread
direction maps, **so that** velvet cloaks and silk gowns show the soft fuzzy highlight and weave
pattern that distinguishes them from hard surfaces.

## US-2.12.5.2 Preview Backlit Curtain Transmission

**As a** environment artist (P-8), **I want** thin-surface transmission on fabric materials so
that curtains and flags glow when backlit by sunlight, **so that** interior scenes have natural
light filtering through window drapery.

## US-2.12.5.3 Validate Fabric Shading Fallback on Mobile

**As an** engine tester (P-27), **I want** to verify that mobile uses basic diffuse wrap only
with no sheen or fuzz layers, **so that** fabric rendering remains performant on low-end
hardware without shader errors.

## US-2.12.6.1 Create Brushed Metal With Anisotropic Reflections

**As a** environment artist (P-8), **I want** metal materials with tangent-space anisotropic
reflections and per-pixel roughness variation, **so that** brushed steel, machined aluminum,
and weathered iron each show distinctive specular behavior.

## US-2.12.6.2 Apply Procedural Weathering to Stone and Wood

**As a** technical artist (P-13), **I want** a shared weathering system that drives procedural
rust, moss, cracks, and stains from age/exposure/damage parameters, **so that** I can weather
any natural material without authoring unique detail textures per variant.

## US-2.12.6.3 Verify Marble SSS on Desktop and Baked Fallback on Mobile

**As an** engine tester (P-27), **I want** to verify that stone SSS (marble, alabaster) renders
with porous subsurface scattering on desktop and falls back to baked textures on mobile,
**so that** quality scales correctly across platforms.

## US-2.12.7.1 Author Wax Materials With Thickness-Dependent Transmission

**As a** environment artist (P-8), **I want** wax and candle materials that glow when backlit
with thickness-dependent SSS transmission, **so that** thin wax edges appear translucent while
thick wax remains opaque.

## US-2.12.7.2 Test Deformation-Driven Roughness Modulation

**As an** engine developer (P-26), **I want** to verify that physics-driven vertex displacement
on rubber materials modulates roughness and scattering parameters in real time, **so that**
stretched rubber appears lighter and compressed rubber appears darker as specified.

## US-2.12.8.1 Paint a Car With Multi-Layer Clearcoat

**As a** technical artist (P-13), **I want** to apply a clearcoat layer with independent
roughness, IOR, and tint on top of a base metallic material, **so that** automotive paint shows
the characteristic deep gloss and orange-peel texture of real car finishes.

## US-2.12.8.2 Animate Rain Wetting a Clearcoat Surface

**As a** environment artist (P-8), **I want** to drive clearcoat wetness parameters at runtime
via weather state, **so that** surfaces gradually become wet and reflective during rain and dry
out afterward.

## US-2.12.8.3 Validate Multi-Layer Blending Limits Per Platform

**As an** engine tester (P-27), **I want** to verify max 2 baked layers on mobile, 2 runtime
layers on Switch, 4 on desktop, and unlimited on high-end, **so that** multi-layer materials
respect platform-specific layer budgets.

## US-2.12.9.1 Build Custom Materials Entirely in the Visual Graph Editor

**As a** technical artist (P-13), **I want** to author materials through the visual shader graph
editor with access to all rendering inputs and output targets, **so that** I can create
procedural wood grain, animated lava, and energy shields without engine source modification.

## US-2.12.9.2 Compose Complex Materials From Reusable Sub-Graphs

**As a** environment artist (P-8), **I want** to build material functions as reusable sub-graphs
and compose them into complex materials, **so that** I can share common operations (triplanar
mapping, noise blending) across many materials without duplication.

## US-2.12.9.3 Verify Custom Material Graph Complexity Caps on Mobile

**As an** engine tester (P-27), **I want** to confirm that mobile caps material graphs at 64
nodes and 4 texture reads, Switch at 128/8, and desktop has no limits, **so that** custom
materials compile within per-platform instruction budgets.
