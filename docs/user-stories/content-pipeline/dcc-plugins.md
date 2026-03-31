# User Stories -- 12.6 DCC Tool Plugins

## Plugin SDK

| ID         | Persona                    |
|------------|----------------------------|
| US-12.6.1  | engine developer (P-26)    |
| US-12.6.2  | technical artist (P-13)    |
| US-12.6.3  | engine developer (P-26)    |

1. **US-12.6.1** — **As an** engine developer (P-26), **I want** a C API plugin SDK with Python and
   C++ bindings for exporting meshes, skeletons, animations, materials, and scenes, **so that**
   every DCC tool can export directly to the native binary format.
2. **US-12.6.2** — **As a** technical artist (P-13), **I want** the plugin SDK shipped as a shared
   library loadable by any DCC host, **so that** I can integrate it into any supported tool without
   engine modifications.
3. **US-12.6.3** — **As an** engine developer (P-26), **I want** the SDK to handle versioning,
   compression, and transport over local socket or shared memory, **so that** third-party
   integrations work without custom transport code.

## DCC Bridge

| ID         | Persona                    |
|------------|----------------------------|
| US-12.6.4  | environment artist (P-8)   |
| US-12.6.5  | environment artist (P-8)   |

1. **US-12.6.4** — **As an** environment artist (P-8), **I want** a real-time bidirectional DCC
   bridge that pushes mesh, material, and animation changes to the engine viewport within 2 seconds,
   **so that** I iterate without full re-exports.
2. **US-12.6.5** — **As an** environment artist (P-8), **I want** the engine to send camera position
   and selection state back to my DCC tool via the bridge, **so that** both viewports stay
   synchronized.

## Houdini

| ID         | Persona                    |
|------------|----------------------------|
| US-12.6.6  | technical artist (P-13)    |
| US-12.6.7  | environment artist (P-8)   |
| US-12.6.8  | environment artist (P-8)   |

1. **US-12.6.6** — **As a** technical artist (P-13), **I want** Houdini Digital Assets to run in the
   engine editor with exposed parameters and cook-on-parameter-change, **so that** I use procedural
   workflows interactively.
2. **US-12.6.7** — **As an** environment artist (P-8), **I want** to export meshes, skeletons, morph
   targets, packed primitives, curves, and volumes from Houdini to the engine, **so that** all
   Houdini output types transfer without data loss.
3. **US-12.6.8** — **As an** environment artist (P-8), **I want** Houdini scatter nodes to output
   point clouds that the engine converts to ECS entities via the PCG pipeline, **so that** I can
   author art-directed placement procedurally.

## Maya

| ID         | Persona                    |
|------------|----------------------------|
| US-12.6.9  | character artist (P-9)     |
| US-12.6.10 | character animator (P-11)  |

1. **US-12.6.9** — **As a** character artist (P-9), **I want** a Maya plugin that exports meshes,
   skeletons, blend shapes, materials, cameras, and lights with incremental export, **so that** only
   changed DAG nodes are re-exported.
2. **US-12.6.10** — **As a** character animator (P-11), **I want** animation exports with full curve
   data, skin weights, IK metadata, and character rig bundles, **so that** animation quality is
   preserved without baked per-frame samples.

## Blender

| ID         | Persona                    |
|------------|----------------------------|
| US-12.6.11 | environment artist (P-8)   |
| US-12.6.12 | technical artist (P-13)    |

1. **US-12.6.11** — **As an** environment artist (P-8), **I want** a Blender addon that exports
   meshes, armatures, shape keys, animations, and materials with collections mapped to entity
   template hierarchies, **so that** Blender users have a first-class export path.
2. **US-12.6.12** — **As a** technical artist (P-13), **I want** Blender Principled BSDF nodes
   mapped to the engine PBR material model with warnings for non-mappable nodes, **so that**
   material conversion is automatic and predictable.

## Marvelous Designer

| ID         | Persona                    |
|------------|----------------------------|
| US-12.6.13 | character artist (P-9)     |
| US-12.6.14 | character artist (P-9)     |

1. **US-12.6.13** — **As a** character artist (P-9), **I want** garment meshes, UV layouts, seam
   data, and fabric properties exported from Marvelous Designer with automatic skeleton binding,
   **so that** cloth assets are simulation-ready.
2. **US-12.6.14** — **As a** character artist (P-9), **I want** automatic garment-to-character
   fitting that computes cloth constraints and skin weight transfers across body variations,
   **so that** garments deform correctly on different characters.

## Substance

| ID         | Persona                    |
|------------|----------------------------|
| US-12.6.15 | environment artist (P-8)   |
| US-12.6.16 | environment artist (P-8)   |
| US-12.6.17 | technical artist (P-13)    |

1. **US-12.6.15** — **As an** environment artist (P-8), **I want** to import .sbsar Substance
   materials and adjust exposed parameters in the engine editor without re-exporting, **so that** I
   tune procedural materials in-engine.
2. **US-12.6.16** — **As an** environment artist (P-8), **I want** Substance Painter projects
   imported with channel-packed textures matching engine conventions and UDIM support, **so that**
   painted texture sets transfer correctly.
3. **US-12.6.17** — **As a** technical artist (P-13), **I want** optional runtime .sbsar evaluation
   for dynamic material variation with a configurable budget, **so that** procedural materials adapt
   to gameplay without frame spikes.

## Photoshop

| ID         | Persona                    |
|------------|----------------------------|
| US-12.6.18 | environment artist (P-8)   |
| US-12.6.19 | game designer (P-5)        |

1. **US-12.6.18** — **As an** environment artist (P-8), **I want** a Photoshop plugin that exports
   textures with channel packing, PSD round-trip, and DCC bridge push, **so that** I iterate on
   textures with instant engine feedback.
2. **US-12.6.19** — **As a** game designer (P-5), **I want** Photoshop layer groups exported as
   engine UI widget hierarchies with text, shape, and visibility mapping, **so that** UI mockups
   transfer without manual recreation.

## Illustrator

| ID         | Persona                    |
|------------|----------------------------|
| US-12.6.20 | game designer (P-5)        |

1. **US-12.6.20** — **As a** game designer (P-5), **I want** Illustrator vector artwork exported as
   resolution-independent vector assets or rasterized atlases with artboard-to-asset mapping,
   **so that** icons and HUD elements render crisply at any scale.

## ZBrush

| ID         | Persona                    |
|------------|----------------------------|
| US-12.6.21 | character artist (P-9)     |
| US-12.6.22 | character artist (P-9)     |

1. **US-12.6.21** — **As a** character artist (P-9), **I want** ZBrush sculpts exported with baked
   normal, displacement, and cavity maps projected onto a low-poly mesh, **so that** high-poly
   detail transfers to engine-ready assets.
2. **US-12.6.22** — **As a** character artist (P-9), **I want** automatic decimation and per-LOD
   normal map bakes from a single sculpt, **so that** a complete LOD chain is produced without
   manual work.

## MotionBuilder

| ID         | Persona                    |
|------------|----------------------------|
| US-12.6.23 | character animator (P-11)  |
| US-12.6.24 | character animator (P-11)  |

1. **US-12.6.23** — **As a** character animator (P-11), **I want** mocap data and cleaned animations
   exported from MotionBuilder with full curve data and batch export, **so that** capture sessions
   transfer efficiently.
2. **US-12.6.24** — **As a** character animator (P-11), **I want** real-time mocap streaming from
   MotionBuilder applied to engine characters with full materials and lighting, **so that**
   directors see capture results on final characters.

## Git Integration

| ID         | Persona                    |
|------------|----------------------------|
| US-12.6.25 | environment artist (P-8)   |
| US-12.6.26 | environment artist (P-8)   |
| US-12.6.27 | environment artist (P-8)   |

1. **US-12.6.25** — **As an** environment artist (P-8), **I want** DCC plugins to auto-acquire Git
   LFS locks when I open source files and release on save, **so that** my team avoids merge
   conflicts on binary source files.
2. **US-12.6.26** — **As an** environment artist (P-8), **I want** review comments from the engine
   editor displayed in a panel within my DCC tool with reply and resolve actions, **so that** I
   respond to feedback without switching applications.
3. **US-12.6.27** — **As an** environment artist (P-8), **I want** a consistent version control
   dashboard across all DCC plugins showing branch, lock status, and one-click commit, push, and
   pull, **so that** I have the same experience in every tool.

## Shared Features

| ID         | Persona                    |
|------------|----------------------------|
| US-12.6.28 | engine developer (P-26)    |
| US-12.6.29 | build engineer (P-16)      |

1. **US-12.6.28** — **As an** engine developer (P-26), **I want** a configurable material
   translation layer that maps each DCC tool's material system to the engine PBR model, **so that**
   material import is consistent regardless of source tool.
2. **US-12.6.29** — **As a** build engineer (P-16), **I want** all DCC plugins to support headless
   batch export via command-line invocation, **so that** CI pipelines re-export modified source
   files automatically.
