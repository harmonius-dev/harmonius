# Animation Features

Animation systems for MMO-scale character rendering, covering skeletal deformation, facial
animation, procedural motion, state-driven blending, and physics-based cloth and hair
simulation.

## Files

| File | Category | Features |
|------|----------|----------|
| [skeletal.md](skeletal.md) | 9.1 Skeletal Animation | GPU skinning, keyframe evaluation, blending, layers, instancing, root motion, compression |
| [morph.md](morph.md) | 9.2 Morph Targets | Blend shapes, corrective shapes, facial animation, vertex animation textures, streaming |
| [procedural.md](procedural.md) | 9.3 Procedural Animation | Two-bone/CCD/FABRIK IK, ragdoll, look-at/aim, motion matching, foot placement |
| [state-machine.md](state-machine.md) | 9.4 Animation State Machine | State graph, transitions, sub-state machines, layers, variables, sync groups, montages |
| [cloth-hair.md](cloth-hair.md) | 9.5 Cloth & Hair Simulation | GPU cloth, strand hair, card hair, hair LOD, cloth-body interaction, wind response |

## Feature Index

| ID | Title | File |
|----|-------|------|
| F-9.1.1 | GPU Compute Skinning | [skeletal.md](skeletal.md) |
| F-9.1.2 | GPU Keyframe Evaluation | [skeletal.md](skeletal.md) |
| F-9.1.3 | Animation Blending (Linear and Cubic) | [skeletal.md](skeletal.md) |
| F-9.1.4 | Animation Layers and Additive Blending | [skeletal.md](skeletal.md) |
| F-9.1.5 | Instanced Skeletal Animation | [skeletal.md](skeletal.md) |
| F-9.1.6 | Root Motion Extraction | [skeletal.md](skeletal.md) |
| F-9.1.7 | Animation Compression | [skeletal.md](skeletal.md) |
| F-9.2.1 | GPU Blend Shape Accumulation | [morph.md](morph.md) |
| F-9.2.2 | Corrective Blend Shapes | [morph.md](morph.md) |
| F-9.2.3 | Facial Animation System | [morph.md](morph.md) |
| F-9.2.4 | Per-Vertex Animation Textures | [morph.md](morph.md) |
| F-9.2.5 | Morph Target Streaming | [morph.md](morph.md) |
| F-9.3.1 | Two-Bone IK Solver | [procedural.md](procedural.md) |
| F-9.3.2 | CCD IK Solver | [procedural.md](procedural.md) |
| F-9.3.3 | FABRIK IK Solver | [procedural.md](procedural.md) |
| F-9.3.4 | Ragdoll Physics (Partial and Full) | [procedural.md](procedural.md) |
| F-9.3.5 | Look-At and Aim Constraints | [procedural.md](procedural.md) |
| F-9.3.6 | Motion Matching | [procedural.md](procedural.md) |
| F-9.3.7 | Foot Placement and Procedural Locomotion | [procedural.md](procedural.md) |
| F-9.4.1 | Animation State Graph | [state-machine.md](state-machine.md) |
| F-9.4.2 | Transitions with Blend Profiles and Sync Markers | [state-machine.md](state-machine.md) |
| F-9.4.3 | Sub-State Machines | [state-machine.md](state-machine.md) |
| F-9.4.4 | State Machine Animation Layers | [state-machine.md](state-machine.md) |
| F-9.4.5 | State Variables and Conditions | [state-machine.md](state-machine.md) |
| F-9.4.6 | Sync Groups | [state-machine.md](state-machine.md) |
| F-9.4.7 | Animation Montages | [state-machine.md](state-machine.md) |
| F-9.5.1 | GPU Cloth Simulation | [cloth-hair.md](cloth-hair.md) |
| F-9.5.2 | Strand-Based Hair Simulation | [cloth-hair.md](cloth-hair.md) |
| F-9.5.3 | Card-Based Hair Rendering | [cloth-hair.md](cloth-hair.md) |
| F-9.5.4 | Hair LOD System | [cloth-hair.md](cloth-hair.md) |
| F-9.5.5 | Cloth-Body Interaction | [cloth-hair.md](cloth-hair.md) |
| F-9.5.6 | Hair Wind Response | [cloth-hair.md](cloth-hair.md) |
