# User Stories — 13.8 Character Customization

## F-13.8.1 Parametric Facial Feature System

## US-13.8.1.1 Author Morph Target Regions for Face Parts

**As a** character artist (P-9), **I want to** define per-region morph target controls for eyes,
nose, mouth, jaw, cheeks, ears, and forehead with 3D coordinate sliders, **so that** facial
features are adjustable at a granular level.

## US-13.8.1.2 Sculpt Unique Facial Features

**As a** player (P-23), **I want to** adjust facial feature sliders (height, width, depth,
rotation) per region to create a unique face, **so that** my character looks distinct from
others.

## US-13.8.1.3 Verify Morph Targets Blend Additively

**As an** engine tester (P-27), **I want to** adjust multiple facial regions simultaneously and
verify morph targets compose additively without interference, **so that** combined adjustments
produce expected results.

## US-13.8.1.4 Verify Symmetry Enforcement and Override

**As an** engine tester (P-27), **I want to** adjust a facial feature with symmetry enabled and
verify both sides match, then break symmetry and verify independent control, **so that**
symmetry behavior is correct.

## F-13.8.2 Preset Blending and Templates

## US-13.8.2.1 Provide Base Template Library

**As a** character artist (P-9), **I want to** provide a library of base face templates by
ethnicity, age range, and body type as starting points, **so that** players have diverse
presets to begin customization.

## US-13.8.2.2 Blend Between Multiple Face Presets

**As a** player (P-23), **I want to** select multiple preset faces and interpolate between them
with per-preset weight sliders, **so that** I can create a face that combines features from
different presets.

## US-13.8.2.3 Save, Load, and Share Custom Presets

**As a** player (P-23), **I want to** save my custom face configuration and share it with other
players as a serialized data file, **so that** my creations are portable.

## US-13.8.2.4 Verify Preset Blending Produces Weighted Average

**As an** engine tester (P-27), **I want to** blend two presets at 50/50 weight and verify the
resulting morph target weights are the mathematical average, **so that** blending is accurate.

## F-13.8.3 Parametric Body Shape System

## US-13.8.3.1 Define Body Shape Morph Targets and Scale Bones

**As a** character artist (P-9), **I want to** create archetypal body morph targets (slim,
heavy, muscular) with matching skeleton scale bones, **so that** body proportions change
through both mesh deformation and bone scaling.

## US-13.8.3.2 Adjust Body Proportions With Continuous Sliders

**As a** player (P-23), **I want to** adjust height, chest, waist, hips, shoulder width, arm
length, leg length, musculature, and body fat with continuous sliders, **so that** my
character's body matches my vision.

## US-13.8.3.3 Verify Measurement Constraints Ensure Plausible Proportions

**As an** engine tester (P-27), **I want to** set extreme slider values and verify measurement
constraints prevent implausible proportions, **so that** body shapes remain within valid
ranges.

## F-13.8.4 Body Morph Propagation to Equipment

## US-13.8.4.1 Author Equipment With Matching Morph Targets

**As a** character artist (P-9), **I want to** author equipment meshes with morph targets
matching the body shape system or use the runtime conform system, **so that** equipment adapts
to all body types without clipping.

## US-13.8.4.2 Wear Equipment Without Clipping on Any Body Type

**As a** player (P-23), **I want to** equipment to conform to my character's body shape without
clipping, **so that** all body types look correct in all armor.

## US-13.8.4.3 Verify Rigid Armor Attaches to Bones Without Deformation

**As an** engine tester (P-27), **I want to** equip plate armor on varied body shapes and verify
rigid pieces attach to bones correctly without deformation artifacts, **so that** the rigid/
deformable distinction is handled properly.

## F-13.8.5 Skin Material System

## US-13.8.5.1 Author Layered Skin Detail Maps

**As a** character artist (P-9), **I want to** create layered detail maps for wrinkles, freckles,
stubble, pores, and vascular detail that compose additively, **so that** skin appearance is
built from reusable parameter sets rather than unique textures.

## US-13.8.5.2 Select Skin Tone With Subsurface Scattering

**As a** player (P-23), **I want to** select from a full spectrum of skin tones with
physically-based subsurface scattering adjusted per tone, **so that** my character's skin
looks realistic.

## US-13.8.5.3 Verify Skin Detail Layers Compose Without Artifacts

**As an** engine tester (P-27), **I want to** enable all skin detail layers simultaneously and
verify they compose additively without blending artifacts, **so that** layered composition is
correct.

## F-13.8.6 Makeup and Face Paint Layer System

## US-13.8.6.1 Author Decal-Based Appearance Layers

**As a** character artist (P-9), **I want to** create makeup, face paint, tattoo, and scar
layers as decals with color, opacity, blend mode, and mask texture, **so that** appearance
customization uses a compositing workflow.

## US-13.8.6.2 Apply Makeup and Tattoos During Character Creation

**As a** player (P-23), **I want to** apply foundation, blush, lipstick, eyeliner, tattoos, and
scars with per-layer color and opacity controls, **so that** my character has a personalized
appearance.

## US-13.8.6.3 Verify Layers Composite Without Modifying Base Textures

**As an** engine tester (P-27), **I want to** apply multiple appearance layers and verify they
composite at runtime into the character's material without altering base textures, **so that**
layer application is non-destructive.

## F-13.8.7 Eye Customization

## US-13.8.7.1 Define Layered Eye Material System

**As a** character artist (P-9), **I want to** define a layered eye material (sclera, iris,
cornea) with per-layer controls for color, pattern, and refraction, **so that** eyes have
depth and realism.

## US-13.8.7.2 Customize Eye Appearance Including Heterochromia

**As a** player (P-23), **I want to** set per-eye iris color (supporting heterochromia), iris
pattern, pupil size, and sclera details, **so that** my character's eyes are unique.

## US-13.8.7.3 Verify Each Eye Layer Renders Independently

**As an** engine tester (P-27), **I want to** toggle each eye material layer and verify it
renders independently with correct compositing order, **so that** the layered eye system
stacks correctly.

## F-13.8.8 Hair Customization System

## US-13.8.8.1 Author Swappable Hair Assets With LOD Transitions

**As a** character artist (P-9), **I want to** create hairstyles as swappable groom or mesh
assets with strand-based rendering at high LOD and card-based at lower LODs, **so that**
hair quality scales with viewing distance.

## US-13.8.8.2 Customize Hair Color, Length, and Curl Pattern

**As a** player (P-23), **I want to** adjust hair color (primary, highlight, ombre), length,
curl pattern, and density, **so that** my character's hairstyle is personalized.

## US-13.8.8.3 Verify Hair Color Propagation to Eyebrows

**As an** engine tester (P-27), **I want to** set hair color and verify it optionally propagates
to eyebrows and eyelashes with per-component override, **so that** color propagation follows
configured rules.

## F-13.8.9 Modular Mesh Part System

## US-13.8.9.1 Define Interchangeable Mesh Part Slots

**As a** character artist (P-9), **I want to** define interchangeable mesh part slots (head,
torso, arms, legs, feet, hands, sub-parts) with a shared skeleton, **so that** characters
are assembled from modular components.

## US-13.8.9.2 Customize Character Appearance Per Mesh Part

**As a** player (P-23), **I want to** mix and match mesh parts and customize materials and
colors per part, **so that** my character's equipment appearance is unique.

## US-13.8.9.3 Verify Mesh Parts Share Skeleton Correctly

**As an** engine tester (P-27), **I want to** assemble a character from parts authored by
different artists and verify all parts animate correctly on the shared skeleton, **so that**
the master-pose component sharing works across all part combinations.

## F-13.8.10 Equipment Attachment and Socket System

## US-13.8.10.1 Define Named Attachment Points on Skeleton

**As a** character artist (P-9), **I want to** define named socket attachment points (head,
back, shoulders, hips, hands) bound to skeleton bones, **so that** equipment attaches at
authored positions.

## US-13.8.10.2 See Equipment Attached Correctly on Character

**As a** player (P-23), **I want to** equipped weapons, shields, and armor to attach to the
correct body positions and follow bone transforms, **so that** equipment looks properly worn.

## US-13.8.10.3 Verify Hidden Mesh Regions Under Opaque Equipment

**As an** engine tester (P-27), **I want to** equip opaque armor and verify underlying body mesh
regions are hidden, **so that** clipping is prevented and GPU cost is saved.

## F-13.8.11 Transmog and Appearance Override

## US-13.8.11.1 Build Account-Wide Wardrobe Collection

**As a** character artist (P-9), **I want to** define which equipment appearances are unlockable
and how they are cataloged in the wardrobe, **so that** the appearance collection grows
meaningfully with play.

## US-13.8.11.2 Override Equipment Appearance With Transmog

**As a** player (P-23), **I want to** equip stat-optimal gear while displaying a different
appearance from my unlocked wardrobe, **so that** I look how I want without sacrificing stats.

## US-13.8.11.3 Save and Swap Outfit Loadouts

**As a** player (P-23), **I want to** save complete appearance loadouts to outfit slots and swap
between them, **so that** I can switch looks quickly for different activities.

## US-13.8.11.4 Verify Dye Colors Persist Across Outfit Changes

**As an** engine tester (P-27), **I want to** apply per-slot dye colors, change outfits, then
change back and verify dye colors are preserved, **so that** dye customization survives outfit
swaps.

## F-13.8.12 Multi-Race Base Mesh Support

## US-13.8.12.1 Author Race-Specific Skeleton and Morph Targets

**As a** character artist (P-9), **I want to** create distinct base mesh sets per race (humanoid,
elven, bestial, mechanical) with race-specific morph sliders and equipment catalogs, **so
that** each race has unique customization options.

## US-13.8.12.2 Choose Race and See Race-Specific Customization

**As a** player (P-23), **I want to** select a race and see race-specific morph sliders (ear
length for elves, tail shape for beast races) in addition to base customization, **so that**
my character reflects my chosen race.

## US-13.8.12.3 Verify Animation Retargeting Across Races

**As an** engine tester (P-27), **I want to** play shared animations on different race
skeletons and verify retargeting produces correct results, **so that** animations work across
all races.

## F-13.8.13 Character LOD and Crowd Optimization

## US-13.8.13.1 Configure LOD Distances Per Character Detail Level

**As a** character artist (P-9), **I want to** configure LOD transition distances for morph
targets, strand hair, card hair, and impostor billboards, **so that** visual quality scales
with viewing distance.

## US-13.8.13.2 See Hundreds of Unique Characters Without Frame Drops

**As a** player (P-23), **I want to** see hundreds of unique customized characters on screen
simultaneously without significant frame rate drops, **so that** crowded areas remain
playable.

## US-13.8.13.3 Verify LOD Transitions Are Not Visually Jarring

**As an** engine tester (P-27), **I want to** walk toward a distant character and verify LOD
transitions (impostor to mesh, card hair to strand hair) are smooth, **so that** LOD pops
are not distracting.

## F-13.8.14 Mesh Merging and Draw Call Reduction

## US-13.8.14.1 Configure Mesh Merging Distance Thresholds

**As a** character artist (P-9), **I want to** configure the distance at which modular mesh
parts are merged into a single draw call, **so that** distant characters use merged meshes
while nearby characters keep hot-swappable parts.

## US-13.8.14.2 Experience Seamless Equipment Swaps on Nearby Characters

**As a** player (P-23), **I want to** nearby characters to show equipment changes immediately
without visible mesh rebuilds, **so that** equipment swaps look instant.

## US-13.8.14.3 Verify Merged Mesh Cache Correctness

**As an** engine tester (P-27), **I want to** equip different part combinations and verify the
merged mesh cache produces correct results for each unique combination, **so that** caching
does not display incorrect meshes.

## F-13.8.15 Character Appearance Serialization

## US-13.8.15.1 Author Version-Tagged Serialization Format

**As a** character artist (P-9), **I want to** the appearance serialization format to be version-
tagged with migration logic for added parameters, **so that** old saves load in new builds.

## US-13.8.15.2 Save and Load Character Appearance Across Sessions

**As a** player (P-23), **I want to** my character's complete appearance (morphs, colors, parts,
layers, equipment) to save and load correctly, **so that** customization persists across play
sessions.

## US-13.8.15.3 Share Appearance Data With Other Players

**As a** player (P-23), **I want to** export and import character appearance data files, **so
that** I can share my character's look with friends.

## US-13.8.15.4 Verify Appearance Migration From Old Versions

**As an** engine tester (P-27), **I want to** load an appearance save from a previous format
version and verify migration adds new parameters with correct defaults, **so that** old saves
are forward-compatible.
