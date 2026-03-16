# 9.5 — Cloth & Hair Simulation

## Cloth Simulation (GPU)

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-9.5.1 | GPU Cloth Simulation | Position-based dynamics cloth simulation running entirely on GPU compute with distance, bending, and self-collision constraints. Cloth meshes are driven by wind forces, character skeletal animation, and collision capsules attached to bones. The cloth simulation substrate is owned by the physics domain (F-4.7.1 XPBD solver). This feature defines the character-specific integration layer: garment authoring, panel-based collision proxy setup, LOD tier management, and skinned mesh binding. | R-9.5.1 | F-9.1.1, F-4.7.1 (Soft-Body and Cloth Physics) | Cloth simulation disabled on mobile by default; uses baked animation fallback. Switch uses simplified constraints (distance only, no bending/self-collision). Desktop runs full PBD cloth. Active cloth panels: mobile 0, Switch 4, desktop 16+. |

## Strand-Based Hair

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-9.5.2 | Strand-Based Hair Simulation | Physics-based strand simulation using guide curves that drive interpolated render strands via skinning weights. Each guide strand is simulated as a chain of particles with stretch, bend, and collision constraints. Supports wind, gravity, and collision with skeletal mesh capsules. Designed for player characters and key NPCs where visual fidelity justifies the simulation cost. | R-9.5.2 | F-9.1.1 | Strand-based hair available on desktop only. Switch uses card-based fallback (F-9.5.3). Mobile uses static shell or baked animation. Guide strand count: desktop 64-256 per character. |

## Card-Based Hair

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-9.5.3 | Card-Based Hair Rendering | Renders hair as textured polygon strips (cards) with alpha-tested or alpha-blended transparency and anisotropic specular shading. Cards are attached to the skeleton and driven by simple spring physics or baked animation. Provides a performant alternative to strand-based hair for NPCs at medium distance and crowd characters. | R-9.5.3 | F-9.1.1 | Card-based hair is the primary hair method on mobile and Switch. Card count per character: mobile 8-16, Switch 16-32, desktop 32-64. |

## Hair LOD System

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-9.5.4 | Hair LOD System | Transitions hair representation between strand-based simulation, simplified strand clusters, and card-based rendering based on camera distance and screen coverage. LOD transitions use temporal blending to avoid popping artifacts. At extreme distances, hair collapses to a single textured shell. Critical for MMO scenes with hundreds of visible characters. | R-9.5.4 | F-9.5.2, F-9.5.3 | LOD tier selection more aggressive on lower platforms. Mobile starts at card/shell LOD. Desktop starts at full strand simulation for hero characters. |

## Cloth-Body Interaction

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-9.5.5 | Cloth-Body Interaction | Resolves collisions between simulated cloth panels and the character's body using capsule and convex hull proxies attached to skeleton bones. Supports friction and sticking contacts to prevent cloth from passing through limbs during fast movement. Collision proxies update each frame from the skinned skeleton. | R-9.5.5 | F-9.5.1, F-9.1.1, F-4.7.4 (Two-Way Rigid Body Coupling) | Collision proxy count per character scales per tier: mobile 0 (cloth disabled), Switch 4-6 capsules, desktop 8-12 capsules + convex hulls. |

## Hair Wind Response

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-9.5.6 | Hair Wind Response | Applies directional and turbulent wind forces to both strand-based and card-based hair systems. Wind is sampled from the shared wind field texture generated from `WindSource` ECS entities (F-4.7.5), ensuring visual coherence across all simulated elements. Hair strands apply per-strand aerodynamic drag proportional to the sampled wind velocity. | R-9.5.6 | F-9.5.2, F-9.5.3, F-4.7.5 (Wind Field Texture) | Wind response available on card-based hair (mobile/Switch) as simplified spring displacement. Full per-particle aerodynamic drag on desktop strand simulation only. |
