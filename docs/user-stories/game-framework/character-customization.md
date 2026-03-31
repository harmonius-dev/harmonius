# User Stories -- Character Customization (13.8)

## Facial and Body

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.8.1.1   | character artist (P-9)  |
| US-13.8.1.2   | player (P-23)           |
| US-13.8.1.3   | game developer (P-15)   |
| US-13.8.2.1   | character artist (P-9)  |
| US-13.8.2.2   | player (P-23)           |
| US-13.8.3.1   | character artist (P-9)  |
| US-13.8.3.2   | player (P-23)           |
| US-13.8.4.1   | game developer (P-15)   |
| US-13.8.4.2   | player (P-23)           |

1. **US-13.8.1.1** -- **As a** character artist (P-9), **I want** to define per-region morph target
   controls for eyes, nose, mouth, jaw, cheeks, ears, and forehead with 3D coordinate sliders,
   **so that** facial features are adjustable at a granular level.

2. **US-13.8.1.2** -- [game-specific] **As a** player (P-23), **I want** to adjust facial feature
   sliders per region to create a unique face, **so that** my character looks distinct from others.

3. **US-13.8.1.3** -- **As a** game developer (P-15), **I want** morph targets to compose additively
   so multiple adjustments combine without interference, **so that** the blending math is
   predictable.

4. **US-13.8.2.1** -- **As a** character artist (P-9), **I want** to provide a library of base face
   templates by ethnicity, age range, and body type, **so that** players have diverse starting
   points.

5. **US-13.8.2.2** -- [game-specific] **As a** player (P-23), **I want** to select multiple preset
   faces and interpolate between them with per-preset weight sliders, **so that** I combine features
   from different presets.

6. **US-13.8.3.1** -- **As a** character artist (P-9), **I want** to create archetypal body morph
   targets with matching skeleton scale bones, **so that** body proportions change through both mesh
   and bone scaling.

7. **US-13.8.3.2** -- [game-specific] **As a** player (P-23), **I want** continuous sliders for
   height, chest, waist, hips, shoulder width, and musculature, **so that** my character's body
   matches my vision.

8. **US-13.8.4.1** -- **As a** game developer (P-15), **I want** body shape changes to propagate
   automatically to equipped clothing and armor meshes, **so that** equipment conforms without
   clipping across all body types.

9. **US-13.8.4.2** -- [game-specific] **As a** player (P-23), **I want** equipment to conform to my
   character's body shape without clipping, **so that** all body types look correct in all armor.

## Skin and Appearance

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.8.5.1   | character artist (P-9)  |
| US-13.8.5.2   | player (P-23)           |
| US-13.8.6.1   | character artist (P-9)  |
| US-13.8.6.2   | player (P-23)           |
| US-13.8.7.1   | character artist (P-9)  |
| US-13.8.7.2   | player (P-23)           |
| US-13.8.8.1   | character artist (P-9)  |
| US-13.8.8.2   | player (P-23)           |

1. **US-13.8.5.1** -- **As a** character artist (P-9), **I want** layered detail maps for wrinkles,
   freckles, stubble, and pores that compose additively, **so that** skin appearance uses reusable
   parameter sets.

2. **US-13.8.5.2** -- [game-specific] **As a** player (P-23), **I want** to select from a full
   spectrum of skin tones with physically-based subsurface scattering, **so that** my character's
   skin looks realistic.

3. **US-13.8.6.1** -- **As a** character artist (P-9), **I want** decal- based appearance layers for
   makeup, tattoos, and scars with color, opacity, and blend mode, **so that** customization uses a
   compositing workflow.

4. **US-13.8.6.2** -- [game-specific] **As a** player (P-23), **I want** to apply foundation, blush,
   lipstick, tattoos, and scars with per-layer controls, **so that** my character has a personalized
   appearance.

5. **US-13.8.7.1** -- **As a** character artist (P-9), **I want** a layered eye material (sclera,
   iris, cornea) with per-layer controls for color, pattern, and refraction, **so that** eyes have
   depth and realism.

6. **US-13.8.7.2** -- [game-specific] **As a** player (P-23), **I want** per-eye iris color
   supporting heterochromia, iris pattern, and pupil size, **so that** my character's eyes are
   unique.

7. **US-13.8.8.1** -- **As a** character artist (P-9), **I want** hairstyles as swappable groom or
   mesh assets with strand-based rendering at high LOD and card-based at lower LODs, **so that**
   hair quality scales with distance.

8. **US-13.8.8.2** -- [game-specific] **As a** player (P-23), **I want** to adjust hair color,
   length, curl pattern, and density, **so that** my character's hairstyle is personalized.

## Modular Equipment

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.8.9.1   | character artist (P-9)  |
| US-13.8.9.2   | player (P-23)           |
| US-13.8.10.1  | game developer (P-15)   |
| US-13.8.10.2  | player (P-23)           |
| US-13.8.11.1  | game designer (P-5)     |
| US-13.8.11.2  | player (P-23)           |
| US-13.8.11.3  | player (P-23)           |

1. **US-13.8.9.1** -- **As a** character artist (P-9), **I want** to define interchangeable mesh
   part slots with a shared skeleton, **so that** characters are assembled from modular components.

2. **US-13.8.9.2** -- [game-specific] **As a** player (P-23), **I want** to mix and match mesh parts
   and customize materials and colors per part, **so that** my character's equipment appearance is
   unique.

3. **US-13.8.10.1** -- **As a** game developer (P-15), **I want** named socket attachment points
   bound to skeleton bones with body mesh hiding under opaque equipment, **so that** clipping is
   prevented and GPU cost is saved.

4. **US-13.8.10.2** -- [game-specific] **As a** player (P-23), **I want** equipped weapons and armor
   to attach to the correct body positions and follow bone transforms, **so that** equipment looks
   properly worn.

5. **US-13.8.11.1** -- **As a** game designer (P-5), **I want** an account-wide wardrobe collecting
   all unlocked appearances with outfit slots and a dye system, **so that** appearance collection is
   a progression system.

6. **US-13.8.11.2** -- [game-specific] **As a** player (P-23), **I want** to equip stat-optimal gear
   while displaying a different appearance from my unlocked wardrobe, **so that** I look how I want
   without sacrificing stats.

7. **US-13.8.11.3** -- [game-specific] **As a** player (P-23), **I want** to save complete
   appearance loadouts to outfit slots and swap between them, **so that** I can switch looks quickly
   for different activities.

## Race, Performance, and Serialization

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.8.12.1  | character artist (P-9)  |
| US-13.8.12.2  | player (P-23)           |
| US-13.8.13.1  | game developer (P-15)   |
| US-13.8.14.1  | game developer (P-15)   |
| US-13.8.15.1  | game developer (P-15)   |
| US-13.8.15.2  | player (P-23)           |

1. **US-13.8.12.1** -- **As a** character artist (P-9), **I want** distinct base mesh sets per race
   with race-specific morph sliders and equipment catalogs, **so that** each race has unique
   customization options.

2. **US-13.8.12.2** -- [game-specific] **As a** player (P-23), **I want** to select a race and see
   race-specific morph sliders in addition to base customization, **so that** my character reflects
   my chosen race.

3. **US-13.8.13.1** -- **As a** game developer (P-15), **I want** multi- level LOD for customized
   characters with impostor billboards at extreme range, **so that** hundreds of unique characters
   render simultaneously.

4. **US-13.8.14.1** -- **As a** game developer (P-15), **I want** modular mesh parts merged into a
   single draw call at distance, **so that** draw call count is reduced for crowds.

5. **US-13.8.15.1** -- **As a** game developer (P-15), **I want** appearance serialization to be
   version-tagged with migration logic for added parameters, **so that** old saves load in new
   builds.

6. **US-13.8.15.2** -- [game-specific] **As a** player (P-23), **I want** to export and import
   character appearance data files, **so that** I can share my character's look with friends.
