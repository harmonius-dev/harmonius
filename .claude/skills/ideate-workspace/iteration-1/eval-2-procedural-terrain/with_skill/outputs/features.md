# 3.6 — Procedural Generation (Biome-Driven Terrain)

New features for designer-painted biome regions that drive procedural terrain, vegetation, and
weather generation. These extend the existing biome system (F-3.6.11, F-3.6.44) with an interactive
authoring workflow and unified generation pipeline.

## Biome Painting and Authoring

| ID       | Feature                          | Requirements        |
|----------|----------------------------------|---------------------|
| F-3.6.65 | Biome Region Painting Tool       | R-3.6.65, R-3.6.65a |
| F-3.6.66 | Biome Template Library           | R-3.6.66            |
| F-3.6.67 | Biome Boundary Blending          | R-3.6.67, R-3.6.67a |

1. **F-3.6.65** — An editor brush tool for painting biome regions directly onto the terrain.
   Designers select a biome type from the template library (F-3.6.66) and paint regions using
   configurable brush shapes (circle, square, spline-lasso). Each painted stroke writes biome IDs
   into a per-tile biome map at the terrain tile resolution. The biome map is a lightweight texture
   layer independent of the heightfield, enabling rapid iteration. Painted regions override
   procedurally generated biome assignments (F-3.6.44) in the same tile, letting designers
   hand-place biomes while relying on procedural fill for the rest of the world. Real-time preview
   updates terrain materials, vegetation density, and weather as the designer paints.
   - **Deps:** F-3.6.11 (Biome Distribution), F-3.6.44 (Biome Classification), F-3.2.1 (Heightfield
     Terrain), F-15.1.1 (Editor Framework)
   - **Platform:** Editor-only feature. Runs on desktop editor platforms only.

2. **F-3.6.66** — A data-driven biome template system where each biome type is an asset defining:
   terrain heightmap stamp rules (ridge frequency, erosion amount, base elevation range), material
   splatmap weights (rock, dirt, sand, snow, grass ratios), vegetation species list with per-species
   density and placement constraints, weather state distribution (probability of rain, snow, fog,
   clear per season), ambient audio profile, and fog/atmosphere color overrides. Designers create
   custom biome templates by combining existing terrain stamps (F-3.6.9), vegetation rules
   (F-3.6.12), and weather states (F-2.7.10) into reusable bundles. Templates are versioned assets
   stored in the content pipeline.
   - **Deps:** F-3.6.9 (Terrain Stamps), F-3.6.10 (Texture Stamps), F-3.6.12 (Vegetation Placement),
     F-2.7.10 (Weather System)
   - **Platform:** Template authoring is editor-only. Runtime reads baked template data on all
     platforms.

3. **F-3.6.67** — Automatic blending at biome region boundaries using noise-perturbed gradient
   transitions. When two biome regions meet, the system interpolates terrain height stamps, material
   weights, vegetation density, and weather parameters across a configurable transition width. Noise
   perturbation breaks straight boundary lines into organic shapes. Transition zones support ecotone
   vegetation rules (species that appear only at biome boundaries, e.g., shrubland between forest
   and desert). Blending width is per-biome-pair configurable (forest-to-tundra may blend over 200m
   while desert-to-oasis blends over 20m).
   - **Deps:** F-3.6.65 (Biome Painting), F-3.6.66 (Biome Templates), F-3.6.33 (Noise Library)
   - **Platform:** Transition zone width reduced on mobile to lower blending cost. Ecotone
     vegetation density scaled per tier.

## Biome-Driven Terrain Generation

| ID       | Feature                            | Requirements        |
|----------|------------------------------------|---------------------|
| F-3.6.68 | Biome-Driven Heightmap Generation  | R-3.6.68, R-3.6.68a |
| F-3.6.69 | Biome-Driven Material Assignment   | R-3.6.69            |

1. **F-3.6.68** — Generate terrain heightmap detail from biome assignments. Each biome template
   (F-3.6.66) contributes terrain modification stamps that sculpt the heightfield: desert biomes
   produce dune ridges and wind erosion patterns, mountain biomes produce jagged peaks with scree
   slopes, swamp biomes flatten terrain and create shallow depressions, and tundra biomes produce
   gentle rolling hills with frost-heave patterning. Generation operates per terrain tile, reading
   the biome map and applying the appropriate stamp stack. The result composites with the base
   heightfield (F-3.2.1) non-destructively via the stamp system (F-3.6.9).
   - **Deps:** F-3.6.65 (Biome Painting), F-3.6.66 (Biome Templates), F-3.6.9 (Terrain Stamps),
     F-3.2.1 (Heightfield)
   - **Platform:** Stamp resolution matches terrain tile resolution per tier. Generation is
     asynchronous on all platforms.

2. **F-3.6.69** — Assign terrain material splatmap weights based on biome type and terrain analysis.
   Each biome template specifies preferred material layers and their weight curves as functions of
   slope, altitude within the biome, curvature, and moisture. The material assignment pass reads the
   biome map and terrain analysis data per tile and writes the splatmap (F-3.2.5). Biome boundary
   blending (F-3.6.67) interpolates material weights across transition zones. Designers can override
   specific material assignments with texture stamps (F-3.6.10) for hero areas.
   - **Deps:** F-3.6.65 (Biome Painting), F-3.6.66 (Biome Templates), F-3.2.5 (Splatmap Materials),
     F-3.6.10 (Texture Stamps)
   - **Platform:** Material layer count per biome inherits per-tier limits from F-3.2.5.

## Biome-Driven Vegetation and Weather

| ID       | Feature                             | Requirements        |
|----------|-------------------------------------|---------------------|
| F-3.6.70 | Biome-Driven Vegetation Scattering  | R-3.6.70, R-3.6.70a |
| F-3.6.71 | Biome-Driven Weather Zones          | R-3.6.71, R-3.6.71a |
| F-3.6.72 | Biome Seasonal Variation            | R-3.6.72            |

1. **F-3.6.70** — Scatter vegetation instances using biome-specific species lists and density rules
   from biome templates (F-3.6.66). Each biome defines which tree species, shrubs, grasses, flowers,
   and ground cover to place, with per-species density curves, clustering rules, and altitude/slope
   constraints. The scattering system reads the biome map per terrain tile and dispatches vegetation
   placement (F-3.6.12) with biome-appropriate parameters. Transition zones (F-3.6.67) blend species
   from adjacent biomes with ecotone rules. Supports biome-exclusive species (e.g., cacti only in
   desert) and ubiquitous species (e.g., grass everywhere except desert).
   - **Deps:** F-3.6.65 (Biome Painting), F-3.6.66 (Biome Templates), F-3.6.12 (Vegetation
     Placement), F-3.3.1 (Instanced Foliage)
   - **Platform:** Species variety per biome reduced on mobile. Vegetation density scaled per tier
     (inherits F-3.3.1 caps).

2. **F-3.6.71** — Drive the weather system (F-2.7.10) from biome regions so each biome has distinct
   weather patterns. The weather state machine receives per-region biome data and adjusts weather
   probabilities: tropical biomes have frequent rain and high humidity, desert biomes have rare rain
   with dust storms, tundra biomes have frequent snow and blizzards, and temperate biomes have
   varied seasonal weather. Weather transitions occur as the camera (or player) moves between biome
   regions, with interpolated blending across biome boundaries. Biome weather data also drives fog
   density, wind intensity, cloud coverage, and precipitation type.
   - **Deps:** F-3.6.65 (Biome Painting), F-3.6.66 (Biome Templates), F-2.7.10 (Weather System),
     F-3.5.3 (Volumetric Clouds)
   - **Platform:** Weather zone transitions simplified on mobile (instant switch vs gradient).
     Volumetric cloud integration disabled on mobile (uses fog fallback).

3. **F-3.6.72** — Biome appearance changes with in-game seasons. Each biome template defines
   per-season overrides: vegetation color shifts (autumn foliage, spring blooms), material weight
   adjustments (snow coverage in winter, mud in spring), weather probability shifts, and vegetation
   density changes (deciduous trees lose leaves in winter). A global season clock advances time and
   the biome system interpolates between seasonal states. Season duration is designer-configurable
   per game. Supports both realistic four-season cycles and fantasy seasonal models.
   - **Deps:** F-3.6.66 (Biome Templates), F-3.6.70 (Biome Vegetation), F-3.6.71 (Biome Weather)
   - **Platform:** Seasonal vegetation color shifts use tinted materials on mobile (no foliage mesh
     swap). Full seasonal variation on desktop.

## Additional Suggested Features

| ID       | Feature                              | Requirements |
|----------|--------------------------------------|--------------|
| F-3.6.73 | Biome-Driven Ambient Audio Zones     | R-3.6.73     |
| F-3.6.74 | Biome Minimap and Debug Visualization | R-3.6.74    |

1. **F-3.6.73** — Each biome template includes an ambient audio profile specifying background
   soundscapes (birdsong in forest, wind in tundra, insects in jungle, silence in desert). The audio
   system (F-5.1.3) reads the biome map at the listener position and crossfades between ambient
   profiles as the player moves between biomes. Transition zones blend audio from adjacent biomes.
   Seasonal overrides adjust ambient audio (fewer birds in winter, rain ambience in monsoon season).
   - **Deps:** F-3.6.66 (Biome Templates), F-3.6.71 (Biome Weather), F-5.1.3 (Mixer Bus)
   - **Platform:** Ambient audio crossfade on all platforms. Concurrent ambient source count reduced
     on mobile.

2. **F-3.6.74** — A debug overlay and minimap view showing biome region boundaries, biome IDs,
   transition zones, and per-tile generation status (pending, generating, complete). Designers use
   this to verify biome coverage, identify gaps, and debug generation issues. The minimap supports
   biome-colored terrain rendering at world scale. Available in editor and via a debug console
   command at runtime.
   - **Deps:** F-3.6.65 (Biome Painting), F-15.1.1 (Editor Framework)
   - **Platform:** Editor-only debug feature. No runtime platform scaling.
