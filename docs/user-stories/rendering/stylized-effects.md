# User Stories -- 2.11 Stylized and Gameplay Effects

## Stories

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-2.11.1.1 | game designer (P-5) | I want per-entity outlines with configurable color, width, and style (solid, dashed, animated) driven by ECS components | interactive objects are visually highlighted when hovered or selected without custom rendering code |  |  |
| US-2.11.1.2 | game designer (P-5) | I want an entity to have a selection outline, a team-color outline, and a quest-indicator outline simultaneously composited additively | multiple gameplay states are communicated visually without conflict |  |  |
| US-2.11.1.3 | engine tester (P-27) | I want to disable compute shaders and verify that the jump-flood outline algorithm falls back to Sobel edge detection | outlines render correctly on devices without compute support |  |  |
| US-2.11.2.1 | game designer (P-5) | I want emissive glow highlights driven by a per-entity HighlightState ECS component with configurable color and intensity per rarity | players can identify loot quality at a glance without reading UI labels |  |  |
| US-2.11.2.2 | effects artist (P-12) | I want fresnel-based rim glow with sinusoidal intensity pulsing on enemies that flash on damage | combat feedback is immediate and visible even in chaotic multi-character fights |  |  |
| US-2.11.2.3 | engine tester (P-27) | I want to verify that mobile limits highlighted entities to 4 with flat color inner glow only, Switch allows 8 with Gaussian blur, and desktop has no limit with full glow | highlight rendering scales per platform |  |  |
| US-2.11.3.1 | technical artist (P-13) | I want to configure cel-shading with per-material ramp textures controlling band count, thresholds, and colors including hatching patterns for shadow regions | I can achieve a specific toon art style per character or environment |  |  |
| US-2.11.3.2 | effects artist (P-12) | I want specular highlights rendered as hard-edged shapes (circular, star, cross) with artist-controlled size and threshold | toon characters have stylized specular that matches the cel-shaded aesthetic |  |  |
| US-2.11.3.3 | engine tester (P-27) | I want to verify that mobile uses 2-3 bands with no hatching/stipple patterns and simplified rim lighting | toon shading stays within mobile shader complexity limits |  |  |
| US-2.11.4.1 | game designer (P-5) | I want automatic roof fading with configurable modes (volume- based, ray-based, layer-based) when the camera or player moves beneath occluding geometry | interiors are visible in isometric and top-down views without manual trigger setup |  |  |
| US-2.11.4.2 | environment artist (P-8) | I want fading to use dithered transparency, cross-hatch dissolve, or smooth alpha with configurable fade speed | roof removal looks polished and matches the game's art style |  |  |
| US-2.11.4.3 | engine tester (P-27) | I want to verify that mobile supports only layer-based and volume-based modes with dither-only dissolve, and desktop supports all modes with smooth alpha | cut-through visibility scales per platform |  |  |
| US-2.11.5.1 | player (P-23) | I want ally characters rendered as colored silhouettes visible through walls when they have an XRayVisible component | I can track teammate positions in team-based shooters |  |  |
| US-2.11.5.2 | game designer (P-5) | I want to set priority levels for x-ray visibility (player always, allies on toggle, enemies within detection range) | only gameplay-relevant entities show through walls |  |  |
| US-2.11.5.3 | engine tester (P-27) | I want to verify that mobile caps x-ray entities at 8 with flat color only, Switch at 16 with team tint, and desktop has no limit with fresnel outlines | x-ray rendering respects per-platform budgets |  |  |
