# 9.5 — Cloth & Hair Simulation

## Cloth Simulation (GPU)

### F-9.5.1 GPU Cloth Simulation

Position-based dynamics cloth simulation running entirely on GPU compute with distance, bending,
and self-collision constraints. Cloth meshes are driven by wind forces, character skeletal
animation, and collision capsules attached to bones. A panel-based authoring model allows
designers to define constraint regions non-destructively for cloaks, banners, and tabards.

- **Requirements:** R-9.5.1
- **Dependencies:** F-9.1.1
- **Platform notes:** None

## Strand-Based Hair

### F-9.5.2 Strand-Based Hair Simulation

Physics-based strand simulation using guide curves that drive interpolated render strands via
skinning weights. Each guide strand is simulated as a chain of particles with stretch, bend, and
collision constraints. Supports wind, gravity, and collision with skeletal mesh capsules.
Designed for player characters and key NPCs where visual fidelity justifies the simulation cost.

- **Requirements:** R-9.5.2
- **Dependencies:** F-9.1.1
- **Platform notes:** None

## Card-Based Hair

### F-9.5.3 Card-Based Hair Rendering

Renders hair as textured polygon strips (cards) with alpha-tested or alpha-blended transparency
and anisotropic specular shading. Cards are attached to the skeleton and driven by simple spring
physics or baked animation. Provides a performant alternative to strand-based hair for NPCs at
medium distance and crowd characters, maintaining visual quality at lower simulation cost.

- **Requirements:** R-9.5.3
- **Dependencies:** F-9.1.1
- **Platform notes:** None

## Hair LOD System

### F-9.5.4 Hair LOD System

Transitions hair representation between strand-based simulation, simplified strand clusters, and
card-based rendering based on camera distance and screen coverage. LOD transitions use temporal
blending to avoid popping artifacts. At extreme distances, hair collapses to a single textured
shell. Critical for MMO scenes with hundreds of visible characters where full strand simulation
is prohibitively expensive.

- **Requirements:** R-9.5.4
- **Dependencies:** F-9.5.2, F-9.5.3
- **Platform notes:** None

## Cloth-Body Interaction

### F-9.5.5 Cloth-Body Interaction

Resolves collisions between simulated cloth panels and the character's body using capsule and
convex hull proxies attached to skeleton bones. Supports friction and sticking contacts to
prevent cloth from passing through limbs during fast movement. Collision proxies update each
frame from the skinned skeleton, enabling cloth to respond correctly to combat animations,
mounts, and emote poses.

- **Requirements:** R-9.5.5
- **Dependencies:** F-9.5.1, F-9.1.1
- **Platform notes:** None

## Hair Wind Response

### F-9.5.6 Hair Wind Response

Applies directional and turbulent wind forces to both strand-based and card-based hair systems.
Wind is sampled from the global wind field shared with foliage and particle systems, ensuring
visual coherence across all simulated elements. Strand-based hair receives per-particle forces
while card-based hair uses simplified spring displacements, maintaining consistent wind behavior
across LOD levels.

- **Requirements:** R-9.5.6
- **Dependencies:** F-9.5.2, F-9.5.3
- **Platform notes:** None
