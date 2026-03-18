# Harmonius Game Engine -- Features

Complete feature specifications for the Harmonius game engine, a general-purpose, all-genre engine
supporting MMO, RPG, FPS, RTS, 2D, VR, co-op, and local multiplayer games. Each feature has a unique
identifier (e.g., `F-2.3.1` is the first feature in category 2.3). Features reference requirements
from [docs/requirements/](../requirements/).

## Summary

| # | Module | Files | Features |
|---|--------|-------|----------|
| 1 | [Core Runtime](core-runtime/) | 9 | 100 |
| 2 | [Rendering](rendering/) | 12 | 142 |
| 3 | [Geometry & World](geometry-world/) | 6 | 106 |
| 4 | [Physics](physics/) | 8 | 62 |
| 5 | [Audio](audio/) | 5 | 38 |
| 6 | [Input](input/) | 5 | 31 |
| 7 | [AI](ai/) | 8 | 67 |
| 8 | [Networking](networking/) | 8 | 52 |
| 9 | [Animation](animation/) | 6 | 46 |
| 10 | [UI & 2D](ui-2d/) | 6 | 75 |
| 11 | [VFX](vfx/) | 6 | 38 |
| 12 | [Content Pipeline](content-pipeline/) | 7 | 75 |
| 13 | [Game Framework](game-framework/) | 28 | 347 |
| 14 | [Platform](platform/) | 6 | 42 |
| 15 | [Tools & Editor](tools-editor/) | 19 | 160 |
| | **Total** | **139** | **1381** |

## Complete Feature Index

| Module           | Area                       | ID          |
|------------------|----------------------------|-------------|
| core-runtime     | entity-component-system    | F-1.1.1     |
| core-runtime     | entity-component-system    | F-1.1.2     |
| core-runtime     | entity-component-system    | F-1.1.3     |
| core-runtime     | entity-component-system    | F-1.1.4     |
| core-runtime     | entity-component-system    | F-1.1.5     |
| core-runtime     | entity-component-system    | F-1.1.6     |
| core-runtime     | entity-component-system    | F-1.1.7     |
| core-runtime     | entity-component-system    | F-1.1.8     |
| core-runtime     | entity-component-system    | F-1.1.9     |
| core-runtime     | entity-component-system    | F-1.1.10    |
| core-runtime     | entity-component-system    | F-1.1.11    |
| core-runtime     | entity-component-system    | F-1.1.12    |
| core-runtime     | entity-component-system    | F-1.1.13    |
| core-runtime     | entity-component-system    | F-1.1.14    |
| core-runtime     | entity-component-system    | F-1.1.15    |
| core-runtime     | entity-component-system    | F-1.1.16    |
| core-runtime     | entity-component-system    | F-1.1.17    |
| core-runtime     | entity-component-system    | F-1.1.18    |
| core-runtime     | entity-component-system    | F-1.1.19    |
| core-runtime     | entity-component-system    | F-1.1.20    |
| core-runtime     | entity-component-system    | F-1.1.21    |
| core-runtime     | entity-component-system    | F-1.1.22    |
| core-runtime     | entity-component-system    | F-1.1.23    |
| core-runtime     | entity-component-system    | F-1.1.24    |
| core-runtime     | entity-component-system    | F-1.1.25    |
| core-runtime     | entity-component-system    | F-1.1.26    |
| core-runtime     | entity-component-system    | F-1.1.27    |
| core-runtime     | entity-component-system    | F-1.1.28    |
| core-runtime     | entity-component-system    | F-1.1.29    |
| core-runtime     | entity-component-system    | F-1.1.30    |
| core-runtime     | entity-component-system    | F-1.1.31    |
| core-runtime     | entity-component-system    | F-1.1.32    |
| core-runtime     | entity-component-system    | F-1.1.33    |
| core-runtime     | entity-component-system    | F-1.1.34    |
| core-runtime     | entity-component-system    | F-1.1.35    |
| core-runtime     | entity-component-system    | F-1.1.36    |
| core-runtime     | entity-component-system    | F-1.1.37    |
| core-runtime     | entity-component-system    | F-1.1.38    |
| core-runtime     | scene-and-transforms       | F-1.2.1     |
| core-runtime     | scene-and-transforms       | F-1.2.2     |
| core-runtime     | scene-and-transforms       | F-1.2.3     |
| core-runtime     | scene-and-transforms       | F-1.2.4     |
| core-runtime     | scene-and-transforms       | F-1.2.5     |
| core-runtime     | scene-and-transforms       | F-1.2.6     |
| core-runtime     | scene-and-transforms       | F-1.2.7     |
| core-runtime     | reflection-and-type-system | F-1.3.1     |
| core-runtime     | reflection-and-type-system | F-1.3.2     |
| core-runtime     | reflection-and-type-system | F-1.3.3     |
| core-runtime     | reflection-and-type-system | F-1.3.4     |
| core-runtime     | reflection-and-type-system | F-1.3.5     |
| core-runtime     | reflection-and-type-system | F-1.3.6     |
| core-runtime     | reflection-and-type-system | F-1.3.7     |
| core-runtime     | serialization              | F-1.4.1     |
| core-runtime     | serialization              | F-1.4.2     |
| core-runtime     | serialization              | F-1.4.3     |
| core-runtime     | serialization              | F-1.4.4     |
| core-runtime     | serialization              | F-1.4.5     |
| core-runtime     | serialization              | F-1.4.6     |
| core-runtime     | serialization              | F-1.4.7     |
| core-runtime     | events-and-messaging       | F-1.5.1     |
| core-runtime     | events-and-messaging       | F-1.5.2     |
| core-runtime     | events-and-messaging       | F-1.5.3     |
| core-runtime     | events-and-messaging       | F-1.5.4     |
| core-runtime     | events-and-messaging       | F-1.5.5     |
| core-runtime     | events-and-messaging       | F-1.5.6     |
| core-runtime     | events-and-messaging       | F-1.5.7     |
| core-runtime     | plugin-system              | F-1.6.1     |
| core-runtime     | plugin-system              | F-1.6.2     |
| core-runtime     | plugin-system              | F-1.6.3     |
| core-runtime     | plugin-system              | F-1.6.4     |
| core-runtime     | plugin-system              | F-1.6.5     |
| core-runtime     | plugin-system              | F-1.6.6     |
| core-runtime     | plugin-system              | F-1.6.7     |
| core-runtime     | memory-management          | F-1.7.1     |
| core-runtime     | memory-management          | F-1.7.2     |
| core-runtime     | memory-management          | F-1.7.3     |
| core-runtime     | memory-management          | F-1.7.4     |
| core-runtime     | memory-management          | F-1.7.5     |
| core-runtime     | memory-management          | F-1.7.6     |
| core-runtime     | memory-management          | F-1.7.7     |
| core-runtime     | memory-management          | F-1.7.8     |
| core-runtime     | memory-management          | F-1.7.9     |
| core-runtime     | async-io                   | F-1.8.1     |
| core-runtime     | async-io                   | F-1.8.2     |
| core-runtime     | async-io                   | F-1.8.3     |
| core-runtime     | async-io                   | F-1.8.4     |
| core-runtime     | async-io                   | F-1.8.5     |
| core-runtime     | async-io                   | F-1.8.6     |
| core-runtime     | async-io                   | F-1.8.7     |
| core-runtime     | async-io                   | F-1.8.8     |
| core-runtime     | async-io                   | F-1.8.9     |
| core-runtime     | spatial-indexing           | F-1.9.1     |
| core-runtime     | spatial-indexing           | F-1.9.2     |
| core-runtime     | spatial-indexing           | F-1.9.3     |
| core-runtime     | spatial-indexing           | F-1.9.4     |
| core-runtime     | spatial-indexing           | F-1.9.5     |
| core-runtime     | spatial-indexing           | F-1.9.6     |
| core-runtime     | spatial-indexing           | F-1.9.7     |
| core-runtime     | spatial-indexing           | F-1.9.8     |
| core-runtime     | spatial-indexing           | F-1.9.9     |
| rendering        | gpu-abstraction-layer      | F-2.1.1     |
| rendering        | gpu-abstraction-layer      | F-2.1.2     |
| rendering        | gpu-abstraction-layer      | F-2.1.3     |
| rendering        | gpu-abstraction-layer      | F-2.1.4     |
| rendering        | gpu-abstraction-layer      | F-2.1.5     |
| rendering        | gpu-abstraction-layer      | F-2.1.6     |
| rendering        | gpu-abstraction-layer      | F-2.1.7     |
| rendering        | gpu-abstraction-layer      | F-2.1.8     |
| rendering        | gpu-abstraction-layer      | F-2.1.9     |
| rendering        | gpu-abstraction-layer      | F-2.1.10    |
| rendering        | gpu-abstraction-layer      | F-2.1.11    |
| rendering        | gpu-abstraction-layer      | F-2.1.12    |
| rendering        | render-graph               | F-2.2.1     |
| rendering        | render-graph               | F-2.2.2     |
| rendering        | render-graph               | F-2.2.3     |
| rendering        | render-graph               | F-2.2.4     |
| rendering        | render-graph               | F-2.2.5     |
| rendering        | render-graph               | F-2.2.6     |
| rendering        | render-graph               | F-2.2.7     |
| rendering        | render-graph               | F-2.2.8     |
| rendering        | render-graph               | F-2.2.9     |
| rendering        | render-graph               | F-2.2.10    |
| rendering        | render-graph               | F-2.2.11    |
| rendering        | render-graph               | F-2.2.12    |
| rendering        | render-graph               | F-2.2.13    |
| rendering        | core-rendering             | F-2.3.1     |
| rendering        | core-rendering             | F-2.3.2     |
| rendering        | core-rendering             | F-2.3.3     |
| rendering        | core-rendering             | F-2.3.4     |
| rendering        | core-rendering             | F-2.3.5     |
| rendering        | core-rendering             | F-2.3.6     |
| rendering        | core-rendering             | F-2.3.7     |
| rendering        | core-rendering             | F-2.3.8     |
| rendering        | core-rendering             | F-2.3.9     |
| rendering        | core-rendering             | F-2.3.10    |
| rendering        | core-rendering             | F-2.3.11    |
| rendering        | core-rendering             | F-2.3.12    |
| rendering        | core-rendering             | F-2.3.13    |
| rendering        | lighting                   | F-2.4.1     |
| rendering        | lighting                   | F-2.4.2     |
| rendering        | lighting                   | F-2.4.3     |
| rendering        | lighting                   | F-2.4.4     |
| rendering        | lighting                   | F-2.4.5     |
| rendering        | lighting                   | F-2.4.6     |
| rendering        | lighting                   | F-2.4.7     |
| rendering        | lighting                   | F-2.4.8     |
| rendering        | lighting                   | F-2.4.9     |
| rendering        | lighting                   | F-2.4.10    |
| rendering        | lighting                   | F-2.4.11    |
| rendering        | lighting                   | F-2.4.12    |
| rendering        | lighting                   | F-2.4.13    |
| rendering        | lighting                   | F-2.4.14    |
| rendering        | lighting                   | F-2.4.15    |
| rendering        | lighting                   | F-2.4.16    |
| rendering        | lighting                   | F-2.4.17    |
| rendering        | lighting                   | F-2.4.18    |
| rendering        | lighting                   | F-2.4.19    |
| rendering        | lighting                   | F-2.4.20    |
| rendering        | lighting                   | F-2.4.21    |
| rendering        | lighting                   | F-2.4.22    |
| rendering        | lighting                   | F-2.4.23    |
| rendering        | advanced-rendering         | F-2.5.1     |
| rendering        | advanced-rendering         | F-2.5.2     |
| rendering        | advanced-rendering         | F-2.5.3     |
| rendering        | advanced-rendering         | F-2.5.4     |
| rendering        | advanced-rendering         | F-2.5.5     |
| rendering        | advanced-rendering         | F-2.5.6     |
| rendering        | advanced-rendering         | F-2.5.7     |
| rendering        | advanced-rendering         | F-2.5.8     |
| rendering        | advanced-rendering         | F-2.5.9     |
| rendering        | advanced-rendering         | F-2.5.10    |
| rendering        | advanced-rendering         | F-2.5.11    |
| rendering        | advanced-rendering         | F-2.5.12    |
| rendering        | advanced-rendering         | F-2.5.13    |
| rendering        | advanced-rendering         | F-2.5.14    |
| rendering        | advanced-rendering         | F-2.5.15    |
| rendering        | advanced-rendering         | F-2.5.16    |
| rendering        | anti-aliasing-upscaling    | F-2.6.1     |
| rendering        | anti-aliasing-upscaling    | F-2.6.2     |
| rendering        | anti-aliasing-upscaling    | F-2.6.3     |
| rendering        | anti-aliasing-upscaling    | F-2.6.4     |
| rendering        | anti-aliasing-upscaling    | F-2.6.5     |
| rendering        | anti-aliasing-upscaling    | F-2.6.6     |
| rendering        | anti-aliasing-upscaling    | F-2.6.7     |
| rendering        | anti-aliasing-upscaling    | F-2.6.8     |
| rendering        | environment                | F-2.7.1     |
| rendering        | environment                | F-2.7.2     |
| rendering        | environment                | F-2.7.3     |
| rendering        | environment                | F-2.7.4     |
| rendering        | environment                | F-2.7.5     |
| rendering        | environment                | F-2.7.6     |
| rendering        | environment                | F-2.7.7     |
| rendering        | environment                | F-2.7.8     |
| rendering        | environment                | F-2.7.9     |
| rendering        | environment                | F-2.7.10    |
| rendering        | character-rendering        | F-2.8.1     |
| rendering        | character-rendering        | F-2.8.2     |
| rendering        | character-rendering        | F-2.8.3     |
| rendering        | character-rendering        | F-2.8.4     |
| rendering        | character-rendering        | F-2.8.5     |
| rendering        | character-rendering        | F-2.8.6     |
| rendering        | character-rendering        | F-2.8.7     |
| rendering        | character-rendering        | F-2.8.8     |
| rendering        | character-rendering        | F-2.8.9     |
| rendering        | post-processing            | F-2.9.1     |
| rendering        | post-processing            | F-2.9.2     |
| rendering        | post-processing            | F-2.9.3     |
| rendering        | post-processing            | F-2.9.4     |
| rendering        | post-processing            | F-2.9.5     |
| rendering        | post-processing            | F-2.9.6     |
| rendering        | post-processing            | F-2.9.7     |
| rendering        | post-processing            | F-2.9.8     |
| rendering        | post-processing            | F-2.9.9     |
| rendering        | post-processing            | F-2.9.10    |
| rendering        | post-processing            | F-2.9.11    |
| rendering        | post-processing            | F-2.9.12    |
| rendering        | post-processing            | F-2.9.13    |
| rendering        | post-processing            | F-2.9.14    |
| rendering        | scene-rendering-pipeline   | F-2.10.1    |
| rendering        | scene-rendering-pipeline   | F-2.10.2    |
| rendering        | scene-rendering-pipeline   | F-2.10.3    |
| rendering        | scene-rendering-pipeline   | F-2.10.4    |
| rendering        | scene-rendering-pipeline   | F-2.10.5    |
| rendering        | scene-rendering-pipeline   | F-2.10.6    |
| rendering        | scene-rendering-pipeline   | F-2.10.7    |
| rendering        | scene-rendering-pipeline   | F-2.10.8    |
| rendering        | scene-rendering-pipeline   | F-2.10.9    |
| rendering        | scene-rendering-pipeline   | F-2.10.10   |
| rendering        | stylized-effects           | F-2.11.1    |
| rendering        | stylized-effects           | F-2.11.2    |
| rendering        | stylized-effects           | F-2.11.3    |
| rendering        | stylized-effects           | F-2.11.4    |
| rendering        | stylized-effects           | F-2.11.5    |
| rendering        | advanced-materials         | F-2.12.1    |
| rendering        | advanced-materials         | F-2.12.2    |
| rendering        | advanced-materials         | F-2.12.3    |
| rendering        | advanced-materials         | F-2.12.4    |
| rendering        | advanced-materials         | F-2.12.5    |
| rendering        | advanced-materials         | F-2.12.6    |
| rendering        | advanced-materials         | F-2.12.7    |
| rendering        | advanced-materials         | F-2.12.8    |
| rendering        | advanced-materials         | F-2.12.9    |
| geometry-world   | meshlet-pipeline           | F-3.1.1     |
| geometry-world   | meshlet-pipeline           | F-3.1.2     |
| geometry-world   | meshlet-pipeline           | F-3.1.3     |
| geometry-world   | meshlet-pipeline           | F-3.1.4     |
| geometry-world   | meshlet-pipeline           | F-3.1.5     |
| geometry-world   | meshlet-pipeline           | F-3.1.6     |
| geometry-world   | meshlet-pipeline           | F-3.1.7     |
| geometry-world   | terrain                    | F-3.2.1     |
| geometry-world   | terrain                    | F-3.2.2     |
| geometry-world   | terrain                    | F-3.2.3     |
| geometry-world   | terrain                    | F-3.2.4     |
| geometry-world   | terrain                    | F-3.2.5     |
| geometry-world   | terrain                    | F-3.2.6     |
| geometry-world   | terrain                    | F-3.2.7     |
| geometry-world   | terrain                    | F-3.2.8     |
| geometry-world   | terrain                    | F-3.2.9     |
| geometry-world   | terrain                    | F-3.2.10    |
| geometry-world   | terrain                    | F-3.2.11    |
| geometry-world   | terrain                    | F-3.2.12    |
| geometry-world   | terrain                    | F-3.2.13    |
| geometry-world   | terrain                    | F-3.2.14    |
| geometry-world   | foliage                    | F-3.3.1     |
| geometry-world   | foliage                    | F-3.3.2     |
| geometry-world   | foliage                    | F-3.3.3     |
| geometry-world   | foliage                    | F-3.3.4     |
| geometry-world   | foliage                    | F-3.3.5     |
| geometry-world   | foliage                    | F-3.3.6     |
| geometry-world   | foliage                    | F-3.3.7     |
| geometry-world   | water                      | F-3.4.1     |
| geometry-world   | water                      | F-3.4.2     |
| geometry-world   | water                      | F-3.4.3     |
| geometry-world   | water                      | F-3.4.4     |
| geometry-world   | water                      | F-3.4.5     |
| geometry-world   | water                      | F-3.4.6     |
| geometry-world   | water                      | F-3.4.7     |
| geometry-world   | sky-atmosphere             | F-3.5.1     |
| geometry-world   | sky-atmosphere             | F-3.5.2     |
| geometry-world   | sky-atmosphere             | F-3.5.3     |
| geometry-world   | sky-atmosphere             | F-3.5.4     |
| geometry-world   | sky-atmosphere             | F-3.5.5     |
| geometry-world   | sky-atmosphere             | F-3.5.6     |
| geometry-world   | sky-atmosphere             | F-3.5.7     |
| geometry-world   | procedural-generation      | F-3.6.1     |
| geometry-world   | procedural-generation      | F-3.6.2     |
| geometry-world   | procedural-generation      | F-3.6.3     |
| geometry-world   | procedural-generation      | F-3.6.4     |
| geometry-world   | procedural-generation      | F-3.6.5     |
| geometry-world   | procedural-generation      | F-3.6.6     |
| geometry-world   | procedural-generation      | F-3.6.7     |
| geometry-world   | procedural-generation      | F-3.6.8     |
| geometry-world   | procedural-generation      | F-3.6.9     |
| geometry-world   | procedural-generation      | F-3.6.10    |
| geometry-world   | procedural-generation      | F-3.6.11    |
| geometry-world   | procedural-generation      | F-3.6.12    |
| geometry-world   | procedural-generation      | F-3.6.13    |
| geometry-world   | procedural-generation      | F-3.6.14    |
| geometry-world   | procedural-generation      | F-3.6.15    |
| geometry-world   | procedural-generation      | F-3.6.16    |
| geometry-world   | procedural-generation      | F-3.6.17    |
| geometry-world   | procedural-generation      | F-3.6.18    |
| geometry-world   | procedural-generation      | F-3.6.19    |
| geometry-world   | procedural-generation      | F-3.6.20    |
| geometry-world   | procedural-generation      | F-3.6.21    |
| geometry-world   | procedural-generation      | F-3.6.22    |
| geometry-world   | procedural-generation      | F-3.6.23    |
| geometry-world   | procedural-generation      | F-3.6.24    |
| geometry-world   | procedural-generation      | F-3.6.25    |
| geometry-world   | procedural-generation      | F-3.6.26    |
| geometry-world   | procedural-generation      | F-3.6.27    |
| geometry-world   | procedural-generation      | F-3.6.28    |
| geometry-world   | procedural-generation      | F-3.6.29    |
| geometry-world   | procedural-generation      | F-3.6.30    |
| geometry-world   | procedural-generation      | F-3.6.31    |
| geometry-world   | procedural-generation      | F-3.6.32    |
| geometry-world   | procedural-generation      | F-3.6.33    |
| geometry-world   | procedural-generation      | F-3.6.34    |
| geometry-world   | procedural-generation      | F-3.6.35    |
| geometry-world   | procedural-generation      | F-3.6.36    |
| geometry-world   | procedural-generation      | F-3.6.37    |
| geometry-world   | procedural-generation      | F-3.6.38    |
| geometry-world   | procedural-generation      | F-3.6.39    |
| geometry-world   | procedural-generation      | F-3.6.40    |
| geometry-world   | procedural-generation      | F-3.6.41    |
| geometry-world   | procedural-generation      | F-3.6.42    |
| geometry-world   | procedural-generation      | F-3.6.43    |
| geometry-world   | procedural-generation      | F-3.6.44    |
| geometry-world   | procedural-generation      | F-3.6.45    |
| geometry-world   | procedural-generation      | F-3.6.46    |
| geometry-world   | procedural-generation      | F-3.6.47    |
| geometry-world   | procedural-generation      | F-3.6.48    |
| geometry-world   | procedural-generation      | F-3.6.49    |
| geometry-world   | procedural-generation      | F-3.6.50    |
| geometry-world   | procedural-generation      | F-3.6.51    |
| geometry-world   | procedural-generation      | F-3.6.52    |
| geometry-world   | procedural-generation      | F-3.6.53    |
| geometry-world   | procedural-generation      | F-3.6.54    |
| geometry-world   | procedural-generation      | F-3.6.55    |
| geometry-world   | procedural-generation      | F-3.6.56    |
| geometry-world   | procedural-generation      | F-3.6.57    |
| geometry-world   | procedural-generation      | F-3.6.58    |
| geometry-world   | procedural-generation      | F-3.6.59    |
| geometry-world   | procedural-generation      | F-3.6.60    |
| geometry-world   | procedural-generation      | F-3.6.61    |
| geometry-world   | procedural-generation      | F-3.6.62    |
| geometry-world   | procedural-generation      | F-3.6.63    |
| geometry-world   | procedural-generation      | F-3.6.64    |
| physics          | rigid-body-dynamics        | F-4.1.1     |
| physics          | rigid-body-dynamics        | F-4.1.2     |
| physics          | rigid-body-dynamics        | F-4.1.3     |
| physics          | rigid-body-dynamics        | F-4.1.4     |
| physics          | rigid-body-dynamics        | F-4.1.5     |
| physics          | rigid-body-dynamics        | F-4.1.6     |
| physics          | rigid-body-dynamics        | F-4.1.7     |
| physics          | rigid-body-dynamics        | F-4.1.8     |
| physics          | rigid-body-dynamics        | F-4.1.9     |
| physics          | rigid-body-dynamics        | F-4.1.10    |
| physics          | collision-detection        | F-4.2.1     |
| physics          | collision-detection        | F-4.2.2     |
| physics          | collision-detection        | F-4.2.3     |
| physics          | collision-detection        | F-4.2.4     |
| physics          | collision-detection        | F-4.2.5     |
| physics          | collision-detection        | F-4.2.6     |
| physics          | collision-detection        | F-4.2.7     |
| physics          | collision-detection        | F-4.2.8     |
| physics          | collision-detection        | F-4.2.9     |
| physics          | constraints-and-joints     | F-4.3.1     |
| physics          | constraints-and-joints     | F-4.3.2     |
| physics          | constraints-and-joints     | F-4.3.3     |
| physics          | constraints-and-joints     | F-4.3.4     |
| physics          | constraints-and-joints     | F-4.3.5     |
| physics          | constraints-and-joints     | F-4.3.6     |
| physics          | constraints-and-joints     | F-4.3.7     |
| physics          | constraints-and-joints     | F-4.3.8     |
| physics          | constraints-and-joints     | F-4.3.9     |
| physics          | spatial-queries            | F-4.4.1     |
| physics          | spatial-queries            | F-4.4.2     |
| physics          | spatial-queries            | F-4.4.3     |
| physics          | spatial-queries            | F-4.4.4     |
| physics          | spatial-queries            | F-4.4.5     |
| physics          | spatial-queries            | F-4.4.6     |
| physics          | vehicle-physics            | F-4.5.1     |
| physics          | vehicle-physics            | F-4.5.2     |
| physics          | vehicle-physics            | F-4.5.3     |
| physics          | vehicle-physics            | F-4.5.4     |
| physics          | vehicle-physics            | F-4.5.5     |
| physics          | vehicle-physics            | F-4.5.6     |
| physics          | vehicle-physics            | F-4.5.7     |
| physics          | destruction-and-fracture   | F-4.6.1     |
| physics          | destruction-and-fracture   | F-4.6.2     |
| physics          | destruction-and-fracture   | F-4.6.3     |
| physics          | destruction-and-fracture   | F-4.6.4     |
| physics          | destruction-and-fracture   | F-4.6.5     |
| physics          | destruction-and-fracture   | F-4.6.6     |
| physics          | destruction-and-fracture   | F-4.6.7     |
| physics          | soft-body-and-cloth        | F-4.7.1     |
| physics          | soft-body-and-cloth        | F-4.7.2     |
| physics          | soft-body-and-cloth        | F-4.7.3     |
| physics          | soft-body-and-cloth        | F-4.7.4     |
| physics          | soft-body-and-cloth        | F-4.7.5     |
| physics          | soft-body-and-cloth        | F-4.7.6     |
| physics          | soft-body-and-cloth        | F-4.7.7     |
| physics          | fluid-simulation           | F-4.8.1     |
| physics          | fluid-simulation           | F-4.8.2     |
| physics          | fluid-simulation           | F-4.8.3     |
| physics          | fluid-simulation           | F-4.8.4     |
| physics          | fluid-simulation           | F-4.8.5     |
| physics          | fluid-simulation           | F-4.8.6     |
| physics          | fluid-simulation           | F-4.8.7     |
| audio            | audio-engine               | F-5.1.1     |
| audio            | audio-engine               | F-5.1.2     |
| audio            | audio-engine               | F-5.1.3     |
| audio            | audio-engine               | F-5.1.4     |
| audio            | audio-engine               | F-5.1.5     |
| audio            | audio-engine               | F-5.1.6     |
| audio            | audio-engine               | F-5.1.7     |
| audio            | spatial-audio              | F-5.2.1     |
| audio            | spatial-audio              | F-5.2.2     |
| audio            | spatial-audio              | F-5.2.3     |
| audio            | spatial-audio              | F-5.2.4     |
| audio            | spatial-audio              | F-5.2.5     |
| audio            | spatial-audio              | F-5.2.6     |
| audio            | spatial-audio              | F-5.2.7     |
| audio            | dsp-and-effects            | F-5.3.1     |
| audio            | dsp-and-effects            | F-5.3.2     |
| audio            | dsp-and-effects            | F-5.3.3     |
| audio            | dsp-and-effects            | F-5.3.4     |
| audio            | dsp-and-effects            | F-5.3.5     |
| audio            | dsp-and-effects            | F-5.3.6     |
| audio            | dsp-and-effects            | F-5.3.7     |
| audio            | dsp-and-effects            | F-5.3.8     |
| audio            | music-system               | F-5.4.1     |
| audio            | music-system               | F-5.4.2     |
| audio            | music-system               | F-5.4.3     |
| audio            | music-system               | F-5.4.4     |
| audio            | music-system               | F-5.4.5     |
| audio            | music-system               | F-5.4.6     |
| audio            | music-system               | F-5.4.7     |
| audio            | voice-and-speech           | F-5.5.1     |
| audio            | voice-and-speech           | F-5.5.2     |
| audio            | voice-and-speech           | F-5.5.3     |
| audio            | voice-and-speech           | F-5.5.4     |
| audio            | voice-and-speech           | F-5.5.5     |
| audio            | voice-and-speech           | F-5.5.6     |
| audio            | voice-and-speech           | F-5.5.7     |
| audio            | voice-and-speech           | F-5.5.8     |
| audio            | voice-and-speech           | F-5.5.9     |
| input            | device-abstraction         | F-6.1.1     |
| input            | device-abstraction         | F-6.1.2     |
| input            | device-abstraction         | F-6.1.3     |
| input            | device-abstraction         | F-6.1.4     |
| input            | device-abstraction         | F-6.1.5     |
| input            | input-actions-and-mapping  | F-6.2.1     |
| input            | input-actions-and-mapping  | F-6.2.2     |
| input            | input-actions-and-mapping  | F-6.2.3     |
| input            | input-actions-and-mapping  | F-6.2.4     |
| input            | input-actions-and-mapping  | F-6.2.5     |
| input            | input-actions-and-mapping  | F-6.2.6     |
| input            | input-actions-and-mapping  | F-6.2.7     |
| input            | input-actions-and-mapping  | F-6.2.8     |
| input            | input-actions-and-mapping  | F-6.2.9     |
| input            | input-actions-and-mapping  | F-6.2.10    |
| input            | input-actions-and-mapping  | F-6.2.11    |
| input            | gestures                   | F-6.3.1     |
| input            | gestures                   | F-6.3.2     |
| input            | gestures                   | F-6.3.3     |
| input            | gestures                   | F-6.3.4     |
| input            | gestures                   | F-6.3.5     |
| input            | haptics-and-feedback       | F-6.4.1     |
| input            | haptics-and-feedback       | F-6.4.2     |
| input            | haptics-and-feedback       | F-6.4.3     |
| input            | haptics-and-feedback       | F-6.4.4     |
| input            | haptics-and-feedback       | F-6.4.5     |
| input            | vr-input                   | F-6.5.1     |
| input            | vr-input                   | F-6.5.2     |
| input            | vr-input                   | F-6.5.3     |
| input            | vr-input                   | F-6.5.4     |
| input            | vr-input                   | F-6.5.5     |
| ai               | navigation                 | F-7.1.1     |
| ai               | navigation                 | F-7.1.2     |
| ai               | navigation                 | F-7.1.3     |
| ai               | navigation                 | F-7.1.4     |
| ai               | navigation                 | F-7.1.5     |
| ai               | navigation                 | F-7.1.6     |
| ai               | navigation                 | F-7.1.7     |
| ai               | navigation                 | F-7.1.8     |
| ai               | navigation                 | F-7.1.9     |
| ai               | navigation                 | F-7.1.10    |
| ai               | navigation                 | F-7.1.11    |
| ai               | navigation                 | F-7.1.12    |
| ai               | navigation                 | F-7.1.13    |
| ai               | navigation                 | F-7.1.14    |
| ai               | navigation                 | F-7.1.15    |
| ai               | steering-avoidance         | F-7.2.1     |
| ai               | steering-avoidance         | F-7.2.2     |
| ai               | steering-avoidance         | F-7.2.3     |
| ai               | steering-avoidance         | F-7.2.4     |
| ai               | steering-avoidance         | F-7.2.5     |
| ai               | steering-avoidance         | F-7.2.6     |
| ai               | behavior-trees             | F-7.3.1     |
| ai               | behavior-trees             | F-7.3.2     |
| ai               | behavior-trees             | F-7.3.3     |
| ai               | behavior-trees             | F-7.3.4     |
| ai               | behavior-trees             | F-7.3.5     |
| ai               | behavior-trees             | F-7.3.6     |
| ai               | behavior-trees             | F-7.3.7     |
| ai               | utility-ai                 | F-7.4.1     |
| ai               | utility-ai                 | F-7.4.2     |
| ai               | utility-ai                 | F-7.4.3     |
| ai               | utility-ai                 | F-7.4.4     |
| ai               | utility-ai                 | F-7.4.5     |
| ai               | goap                       | F-7.5.1     |
| ai               | goap                       | F-7.5.2     |
| ai               | goap                       | F-7.5.3     |
| ai               | goap                       | F-7.5.4     |
| ai               | goap                       | F-7.5.5     |
| ai               | goap                       | F-7.5.6     |
| ai               | perception                 | F-7.6.1     |
| ai               | perception                 | F-7.6.2     |
| ai               | perception                 | F-7.6.3     |
| ai               | perception                 | F-7.6.4     |
| ai               | perception                 | F-7.6.5     |
| ai               | perception                 | F-7.6.6     |
| ai               | perception                 | F-7.6.7     |
| ai               | perception                 | F-7.6.8     |
| ai               | perception                 | F-7.6.9     |
| ai               | perception                 | F-7.6.10    |
| ai               | perception                 | F-7.6.11    |
| ai               | crowd-simulation           | F-7.7.1     |
| ai               | crowd-simulation           | F-7.7.2     |
| ai               | crowd-simulation           | F-7.7.3     |
| ai               | crowd-simulation           | F-7.7.4     |
| ai               | crowd-simulation           | F-7.7.5     |
| ai               | crowd-simulation           | F-7.7.6     |
| ai               | crowd-simulation           | F-7.7.7     |
| ai               | crowd-simulation           | F-7.7.8     |
| ai               | crowd-simulation           | F-7.7.9     |
| ai               | crowd-simulation           | F-7.7.10    |
| ai               | crowd-simulation           | F-7.7.11    |
| ai               | tactical-combat            | F-7.8.1     |
| ai               | tactical-combat            | F-7.8.2     |
| ai               | tactical-combat            | F-7.8.3     |
| ai               | tactical-combat            | F-7.8.4     |
| ai               | tactical-combat            | F-7.8.5     |
| ai               | tactical-combat            | F-7.8.6     |
| networking       | transport-layer            | F-8.1.1     |
| networking       | transport-layer            | F-8.1.2     |
| networking       | transport-layer            | F-8.1.3     |
| networking       | transport-layer            | F-8.1.4     |
| networking       | transport-layer            | F-8.1.5     |
| networking       | transport-layer            | F-8.1.6     |
| networking       | transport-layer            | F-8.1.7     |
| networking       | transport-layer            | F-8.1.8     |
| networking       | state-replication          | F-8.2.1     |
| networking       | state-replication          | F-8.2.2     |
| networking       | state-replication          | F-8.2.3     |
| networking       | state-replication          | F-8.2.4     |
| networking       | state-replication          | F-8.2.5     |
| networking       | state-replication          | F-8.2.6     |
| networking       | remote-procedure-calls     | F-8.3.1     |
| networking       | remote-procedure-calls     | F-8.3.2     |
| networking       | remote-procedure-calls     | F-8.3.3     |
| networking       | remote-procedure-calls     | F-8.3.4     |
| networking       | remote-procedure-calls     | F-8.3.5     |
| networking       | prediction-and-rollback    | F-8.4.1     |
| networking       | prediction-and-rollback    | F-8.4.2     |
| networking       | prediction-and-rollback    | F-8.4.3     |
| networking       | prediction-and-rollback    | F-8.4.4     |
| networking       | prediction-and-rollback    | F-8.4.5     |
| networking       | prediction-and-rollback    | F-8.4.6     |
| networking       | session-management         | F-8.5.1     |
| networking       | session-management         | F-8.5.2     |
| networking       | session-management         | F-8.5.3     |
| networking       | session-management         | F-8.5.4     |
| networking       | session-management         | F-8.5.5     |
| networking       | session-management         | F-8.5.6     |
| networking       | session-management         | F-8.5.7     |
| networking       | session-management         | F-8.5.8     |
| networking       | session-management         | F-8.5.9     |
| networking       | replay-system              | F-8.6.1     |
| networking       | replay-system              | F-8.6.2     |
| networking       | replay-system              | F-8.6.3     |
| networking       | replay-system              | F-8.6.4     |
| networking       | replay-system              | F-8.6.5     |
| networking       | mmo-infrastructure         | F-8.7.1     |
| networking       | mmo-infrastructure         | F-8.7.2     |
| networking       | mmo-infrastructure         | F-8.7.3     |
| networking       | mmo-infrastructure         | F-8.7.4     |
| networking       | mmo-infrastructure         | F-8.7.5     |
| networking       | mmo-infrastructure         | F-8.7.6     |
| networking       | mmo-infrastructure         | F-8.7.7     |
| networking       | mmo-infrastructure         | F-8.7.8     |
| networking       | anti-cheat                 | F-8.8.1     |
| networking       | anti-cheat                 | F-8.8.2     |
| networking       | anti-cheat                 | F-8.8.3     |
| networking       | anti-cheat                 | F-8.8.4     |
| networking       | anti-cheat                 | F-8.8.5     |
| animation        | skeletal                   | F-9.1.1     |
| animation        | skeletal                   | F-9.1.2     |
| animation        | skeletal                   | F-9.1.3     |
| animation        | skeletal                   | F-9.1.4     |
| animation        | skeletal                   | F-9.1.5     |
| animation        | skeletal                   | F-9.1.6     |
| animation        | skeletal                   | F-9.1.7     |
| animation        | skeletal                   | F-9.1.8     |
| animation        | skeletal                   | F-9.1.9     |
| animation        | skeletal                   | F-9.1.10    |
| animation        | morph                      | F-9.2.1     |
| animation        | morph                      | F-9.2.2     |
| animation        | morph                      | F-9.2.3     |
| animation        | morph                      | F-9.2.4     |
| animation        | morph                      | F-9.2.5     |
| animation        | procedural                 | F-9.3.1     |
| animation        | procedural                 | F-9.3.2     |
| animation        | procedural                 | F-9.3.3     |
| animation        | procedural                 | F-9.3.4     |
| animation        | procedural                 | F-9.3.5     |
| animation        | procedural                 | F-9.3.6     |
| animation        | procedural                 | F-9.3.7     |
| animation        | procedural                 | F-9.3.8     |
| animation        | procedural                 | F-9.3.9     |
| animation        | procedural                 | F-9.3.10    |
| animation        | procedural                 | F-9.3.11    |
| animation        | state-machine              | F-9.4.1     |
| animation        | state-machine              | F-9.4.2     |
| animation        | state-machine              | F-9.4.3     |
| animation        | state-machine              | F-9.4.4     |
| animation        | state-machine              | F-9.4.5     |
| animation        | state-machine              | F-9.4.6     |
| animation        | state-machine              | F-9.4.7     |
| animation        | state-machine              | F-9.4.8     |
| animation        | state-machine              | F-9.4.9     |
| animation        | state-machine              | F-9.4.10    |
| animation        | cloth-hair                 | F-9.5.1     |
| animation        | cloth-hair                 | F-9.5.2     |
| animation        | cloth-hair                 | F-9.5.3     |
| animation        | cloth-hair                 | F-9.5.4     |
| animation        | cloth-hair                 | F-9.5.5     |
| animation        | cloth-hair                 | F-9.5.6     |
| animation        | first-person               | F-9.6.1     |
| animation        | first-person               | F-9.6.2     |
| animation        | first-person               | F-9.6.3     |
| animation        | first-person               | F-9.6.4     |
| ui-2d            | widget-framework           | F-10.1.1    |
| ui-2d            | widget-framework           | F-10.1.2    |
| ui-2d            | widget-framework           | F-10.1.3    |
| ui-2d            | widget-framework           | F-10.1.4    |
| ui-2d            | widget-framework           | F-10.1.5    |
| ui-2d            | widget-framework           | F-10.1.6    |
| ui-2d            | widget-framework           | F-10.1.7    |
| ui-2d            | widget-framework           | F-10.1.8    |
| ui-2d            | widget-framework           | F-10.1.9    |
| ui-2d            | widget-framework           | F-10.1.10   |
| ui-2d            | widget-framework           | F-10.1.11   |
| ui-2d            | widget-framework           | F-10.1.12   |
| ui-2d            | widget-framework           | F-10.1.13   |
| ui-2d            | widget-framework           | F-10.1.14   |
| ui-2d            | common-widgets             | F-10.2.1    |
| ui-2d            | common-widgets             | F-10.2.2    |
| ui-2d            | common-widgets             | F-10.2.3    |
| ui-2d            | common-widgets             | F-10.2.4    |
| ui-2d            | common-widgets             | F-10.2.5    |
| ui-2d            | common-widgets             | F-10.2.6    |
| ui-2d            | common-widgets             | F-10.2.7    |
| ui-2d            | common-widgets             | F-10.2.8    |
| ui-2d            | hud-and-game-ui            | F-10.3.1    |
| ui-2d            | hud-and-game-ui            | F-10.3.2    |
| ui-2d            | hud-and-game-ui            | F-10.3.3    |
| ui-2d            | hud-and-game-ui            | F-10.3.4    |
| ui-2d            | hud-and-game-ui            | F-10.3.5    |
| ui-2d            | hud-and-game-ui            | F-10.3.6    |
| ui-2d            | hud-and-game-ui            | F-10.3.7    |
| ui-2d            | hud-and-game-ui            | F-10.3.8    |
| ui-2d            | hud-and-game-ui            | F-10.3.9    |
| ui-2d            | hud-and-game-ui            | F-10.3.10   |
| ui-2d            | hud-and-game-ui            | F-10.3.11   |
| ui-2d            | hud-and-game-ui            | F-10.3.12   |
| ui-2d            | hud-and-game-ui            | F-10.3.13   |
| ui-2d            | hud-and-game-ui            | F-10.3.14   |
| ui-2d            | hud-and-game-ui            | F-10.3.15   |
| ui-2d            | ui-rendering               | F-10.4.1    |
| ui-2d            | ui-rendering               | F-10.4.2    |
| ui-2d            | ui-rendering               | F-10.4.3    |
| ui-2d            | ui-rendering               | F-10.4.4    |
| ui-2d            | ui-rendering               | F-10.4.5    |
| ui-2d            | ui-rendering               | F-10.4.6    |
| ui-2d            | ui-rendering               | F-10.4.7    |
| ui-2d            | 2d-games                   | F-10.5.1    |
| ui-2d            | 2d-games                   | F-10.5.2    |
| ui-2d            | 2d-games                   | F-10.5.3    |
| ui-2d            | 2d-games                   | F-10.5.4    |
| ui-2d            | 2d-games                   | F-10.5.5    |
| ui-2d            | 2d-games                   | F-10.5.6    |
| ui-2d            | 2d-games                   | F-10.5.7    |
| ui-2d            | 2d-games                   | F-10.5.8    |
| ui-2d            | 2d-games                   | F-10.5.9    |
| ui-2d            | 2d-games                   | F-10.5.10   |
| ui-2d            | 2d-games                   | F-10.5.11   |
| ui-2d            | 2d-games                   | F-10.5.12   |
| ui-2d            | 2d-games                   | F-10.5.13   |
| ui-2d            | 2d-games                   | F-10.5.14   |
| ui-2d            | 2d-games                   | F-10.5.15   |
| ui-2d            | 2d-games                   | F-10.5.16   |
| ui-2d            | 2d-games                   | F-10.5.17   |
| ui-2d            | 2d-games                   | F-10.5.18   |
| ui-2d            | 2d-games                   | F-10.5.19   |
| ui-2d            | 2d-games                   | F-10.5.20   |
| ui-2d            | 2d-games                   | F-10.5.21   |
| ui-2d            | 2d-games                   | F-10.5.22   |
| ui-2d            | 2d-games                   | F-10.5.23   |
| ui-2d            | 2d-games                   | F-10.5.24   |
| ui-2d            | accessibility              | F-10.6.1    |
| ui-2d            | accessibility              | F-10.6.2    |
| ui-2d            | accessibility              | F-10.6.3    |
| ui-2d            | accessibility              | F-10.6.4    |
| ui-2d            | accessibility              | F-10.6.5    |
| ui-2d            | accessibility              | F-10.6.6    |
| ui-2d            | accessibility              | F-10.6.7    |
| vfx              | particles                  | F-11.1.1    |
| vfx              | particles                  | F-11.1.2    |
| vfx              | particles                  | F-11.1.3    |
| vfx              | particles                  | F-11.1.4    |
| vfx              | particles                  | F-11.1.5    |
| vfx              | particles                  | F-11.1.6    |
| vfx              | particles                  | F-11.1.7    |
| vfx              | decals                     | F-11.2.1    |
| vfx              | decals                     | F-11.2.2    |
| vfx              | decals                     | F-11.2.3    |
| vfx              | decals                     | F-11.2.4    |
| vfx              | decals                     | F-11.2.5    |
| vfx              | decals                     | F-11.2.6    |
| vfx              | screen-effects             | F-11.3.1    |
| vfx              | screen-effects             | F-11.3.2    |
| vfx              | screen-effects             | F-11.3.3    |
| vfx              | screen-effects             | F-11.3.4    |
| vfx              | screen-effects             | F-11.3.5    |
| vfx              | screen-effects             | F-11.3.6    |
| vfx              | weather-environmental      | F-11.4.1    |
| vfx              | weather-environmental      | F-11.4.2    |
| vfx              | weather-environmental      | F-11.4.3    |
| vfx              | weather-environmental      | F-11.4.4    |
| vfx              | weather-environmental      | F-11.4.5    |
| vfx              | weather-environmental      | F-11.4.6    |
| vfx              | weather-environmental      | F-11.4.7    |
| vfx              | destruction                | F-11.5.1    |
| vfx              | destruction                | F-11.5.2    |
| vfx              | destruction                | F-11.5.3    |
| vfx              | destruction                | F-11.5.4    |
| vfx              | destruction                | F-11.5.5    |
| vfx              | destruction                | F-11.5.6    |
| vfx              | destruction                | F-11.5.7    |
| vfx              | effect-graph               | F-11.6.1    |
| vfx              | effect-graph               | F-11.6.2    |
| vfx              | effect-graph               | F-11.6.3    |
| vfx              | effect-graph               | F-11.6.4    |
| vfx              | effect-graph               | F-11.6.5    |
| content-pipeline | asset-import               | F-12.1.1    |
| content-pipeline | asset-import               | F-12.1.2    |
| content-pipeline | asset-import               | F-12.1.3    |
| content-pipeline | asset-import               | F-12.1.4    |
| content-pipeline | asset-import               | F-12.1.5    |
| content-pipeline | asset-processing           | F-12.2.1    |
| content-pipeline | asset-processing           | F-12.2.2    |
| content-pipeline | asset-processing           | F-12.2.3    |
| content-pipeline | asset-processing           | F-12.2.4    |
| content-pipeline | asset-processing           | F-12.2.5    |
| content-pipeline | asset-processing           | F-12.2.6    |
| content-pipeline | asset-processing           | F-12.2.7    |
| content-pipeline | asset-processing           | F-12.2.8    |
| content-pipeline | asset-processing           | F-12.2.9    |
| content-pipeline | asset-database             | F-12.3.1    |
| content-pipeline | asset-database             | F-12.3.2    |
| content-pipeline | asset-database             | F-12.3.3    |
| content-pipeline | asset-database             | F-12.3.4    |
| content-pipeline | asset-database             | F-12.3.5    |
| content-pipeline | asset-database             | F-12.3.6    |
| content-pipeline | asset-database             | F-12.3.7    |
| content-pipeline | asset-database             | F-12.3.8    |
| content-pipeline | asset-database             | F-12.3.9    |
| content-pipeline | asset-database             | F-12.3.10   |
| content-pipeline | hot-reload                 | F-12.4.1    |
| content-pipeline | hot-reload                 | F-12.4.2    |
| content-pipeline | hot-reload                 | F-12.4.3    |
| content-pipeline | hot-reload                 | F-12.4.4    |
| content-pipeline | hot-reload                 | F-12.4.5    |
| content-pipeline | hot-reload                 | F-12.4.6    |
| content-pipeline | hot-reload                 | F-12.4.7    |
| content-pipeline | streaming-io               | F-12.5.1    |
| content-pipeline | streaming-io               | F-12.5.2    |
| content-pipeline | streaming-io               | F-12.5.3    |
| content-pipeline | streaming-io               | F-12.5.4    |
| content-pipeline | streaming-io               | F-12.5.5    |
| content-pipeline | streaming-io               | F-12.5.6    |
| content-pipeline | streaming-io               | F-12.5.7    |
| content-pipeline | streaming-io               | F-12.5.8    |
| content-pipeline | streaming-io               | F-12.5.9    |
| content-pipeline | streaming-io               | F-12.5.10   |
| content-pipeline | dcc-plugins                | F-12.6.1    |
| content-pipeline | dcc-plugins                | F-12.6.2    |
| content-pipeline | dcc-plugins                | F-12.6.3    |
| content-pipeline | dcc-plugins                | F-12.6.4    |
| content-pipeline | dcc-plugins                | F-12.6.5    |
| content-pipeline | dcc-plugins                | F-12.6.6    |
| content-pipeline | dcc-plugins                | F-12.6.7    |
| content-pipeline | dcc-plugins                | F-12.6.8    |
| content-pipeline | dcc-plugins                | F-12.6.9    |
| content-pipeline | dcc-plugins                | F-12.6.10   |
| content-pipeline | dcc-plugins                | F-12.6.11   |
| content-pipeline | dcc-plugins                | F-12.6.12   |
| content-pipeline | dcc-plugins                | F-12.6.13   |
| content-pipeline | dcc-plugins                | F-12.6.14   |
| content-pipeline | dcc-plugins                | F-12.6.15   |
| content-pipeline | dcc-plugins                | F-12.6.16   |
| content-pipeline | dcc-plugins                | F-12.6.17   |
| content-pipeline | dcc-plugins                | F-12.6.18   |
| content-pipeline | dcc-plugins                | F-12.6.19   |
| content-pipeline | dcc-plugins                | F-12.6.20   |
| content-pipeline | dcc-plugins                | F-12.6.21   |
| content-pipeline | dcc-plugins                | F-12.6.22   |
| content-pipeline | dcc-plugins                | F-12.6.23   |
| content-pipeline | dcc-plugins                | F-12.6.24   |
| content-pipeline | dcc-plugins                | F-12.6.25   |
| content-pipeline | dcc-plugins                | F-12.6.26   |
| content-pipeline | asset-versioning           | F-12.7.1    |
| content-pipeline | asset-versioning           | F-12.7.2    |
| content-pipeline | asset-versioning           | F-12.7.3    |
| content-pipeline | asset-versioning           | F-12.7.4    |
| content-pipeline | asset-versioning           | F-12.7.5    |
| content-pipeline | asset-versioning           | F-12.7.6    |
| content-pipeline | asset-versioning           | F-12.7.7    |
| content-pipeline | asset-versioning           | F-12.7.8    |
| game-framework   | gameplay-primitives        | F-13.1.1    |
| game-framework   | gameplay-primitives        | F-13.1.2    |
| game-framework   | gameplay-primitives        | F-13.1.3    |
| game-framework   | gameplay-primitives        | F-13.1.4    |
| game-framework   | gameplay-primitives        | F-13.1.5    |
| game-framework   | gameplay-primitives        | F-13.1.6    |
| game-framework   | gameplay-primitives        | F-13.1.7    |
| game-framework   | gameplay-primitives        | F-13.1.8    |
| game-framework   | gameplay-primitives        | F-13.1.9    |
| game-framework   | gameplay-primitives        | F-13.1.10   |
| game-framework   | world-management           | F-13.2.1    |
| game-framework   | world-management           | F-13.2.2    |
| game-framework   | world-management           | F-13.2.3    |
| game-framework   | world-management           | F-13.2.4    |
| game-framework   | world-management           | F-13.2.5    |
| game-framework   | world-management           | F-13.2.6    |
| game-framework   | world-management           | F-13.2.7    |
| game-framework   | save-system                | F-13.3.1    |
| game-framework   | save-system                | F-13.3.2    |
| game-framework   | save-system                | F-13.3.3    |
| game-framework   | save-system                | F-13.3.4    |
| game-framework   | save-system                | F-13.3.5    |
| game-framework   | save-system                | F-13.3.6    |
| game-framework   | scripting                  | F-13.4.1    |
| game-framework   | scripting                  | F-13.4.2    |
| game-framework   | scripting                  | F-13.4.3    |
| game-framework   | cinematics                 | F-13.5.1    |
| game-framework   | cinematics                 | F-13.5.2    |
| game-framework   | cinematics                 | F-13.5.3    |
| game-framework   | cinematics                 | F-13.5.4    |
| game-framework   | cinematics                 | F-13.5.5    |
| game-framework   | cinematics                 | F-13.5.6a   |
| game-framework   | cinematics                 | F-13.5.6b   |
| game-framework   | cinematics                 | F-13.5.6c   |
| game-framework   | cinematics                 | F-13.5.7    |
| game-framework   | quest-dialogue             | F-13.6.1    |
| game-framework   | quest-dialogue             | F-13.6.2    |
| game-framework   | quest-dialogue             | F-13.6.3    |
| game-framework   | quest-dialogue             | F-13.6.4    |
| game-framework   | quest-dialogue             | F-13.6.5a   |
| game-framework   | quest-dialogue             | F-13.6.5b   |
| game-framework   | quest-dialogue             | F-13.6.5c   |
| game-framework   | quest-dialogue             | F-13.6.6    |
| game-framework   | quest-dialogue             | F-13.6.7a   |
| game-framework   | quest-dialogue             | F-13.6.7b   |
| game-framework   | gameplay-databases         | F-13.7.1    |
| game-framework   | gameplay-databases         | F-13.7.2    |
| game-framework   | gameplay-databases         | F-13.7.3    |
| game-framework   | gameplay-databases         | F-13.7.4    |
| game-framework   | gameplay-databases         | F-13.7.5    |
| game-framework   | gameplay-databases         | F-13.7.6    |
| game-framework   | gameplay-databases         | F-13.7.7    |
| game-framework   | gameplay-databases         | F-13.7.8    |
| game-framework   | gameplay-databases         | F-13.7.9    |
| game-framework   | gameplay-databases         | F-13.7.10   |
| game-framework   | gameplay-databases         | F-13.7.11   |
| game-framework   | gameplay-databases         | F-13.7.12   |
| game-framework   | gameplay-databases         | F-13.7.13   |
| game-framework   | gameplay-databases         | F-13.7.14   |
| game-framework   | character-customization    | F-13.8.1    |
| game-framework   | character-customization    | F-13.8.2    |
| game-framework   | character-customization    | F-13.8.3    |
| game-framework   | character-customization    | F-13.8.4    |
| game-framework   | character-customization    | F-13.8.5    |
| game-framework   | character-customization    | F-13.8.6    |
| game-framework   | character-customization    | F-13.8.7    |
| game-framework   | character-customization    | F-13.8.8    |
| game-framework   | character-customization    | F-13.8.9    |
| game-framework   | character-customization    | F-13.8.10   |
| game-framework   | character-customization    | F-13.8.11   |
| game-framework   | character-customization    | F-13.8.12   |
| game-framework   | character-customization    | F-13.8.13   |
| game-framework   | character-customization    | F-13.8.14   |
| game-framework   | character-customization    | F-13.8.15   |
| game-framework   | inventory                  | F-13.9.1    |
| game-framework   | inventory                  | F-13.9.2    |
| game-framework   | inventory                  | F-13.9.3    |
| game-framework   | inventory                  | F-13.9.4    |
| game-framework   | inventory                  | F-13.9.5    |
| game-framework   | inventory                  | F-13.9.6    |
| game-framework   | inventory                  | F-13.9.7    |
| game-framework   | inventory                  | F-13.9.8    |
| game-framework   | inventory                  | F-13.9.9    |
| game-framework   | inventory                  | F-13.9.10   |
| game-framework   | abilities                  | F-13.10.1   |
| game-framework   | abilities                  | F-13.10.2   |
| game-framework   | abilities                  | F-13.10.3   |
| game-framework   | abilities                  | F-13.10.4   |
| game-framework   | abilities                  | F-13.10.5   |
| game-framework   | abilities                  | F-13.10.6   |
| game-framework   | selection-system           | F-13.11.1   |
| game-framework   | selection-system           | F-13.11.2   |
| game-framework   | selection-system           | F-13.11.3   |
| game-framework   | selection-system           | F-13.11.4a  |
| game-framework   | selection-system           | F-13.11.4b  |
| game-framework   | selection-system           | F-13.11.4c  |
| game-framework   | selection-system           | F-13.11.4d  |
| game-framework   | selection-system           | F-13.11.5   |
| game-framework   | selection-system           | F-13.11.6a  |
| game-framework   | selection-system           | F-13.11.6b  |
| game-framework   | selection-system           | F-13.11.6c  |
| game-framework   | selection-system           | F-13.11.6d  |
| game-framework   | selection-system           | F-13.11.7   |
| game-framework   | selection-system           | F-13.11.8   |
| game-framework   | progression                | F-13.12.1a  |
| game-framework   | progression                | F-13.12.1b  |
| game-framework   | progression                | F-13.12.1c  |
| game-framework   | progression                | F-13.12.1d  |
| game-framework   | progression                | F-13.12.2a  |
| game-framework   | progression                | F-13.12.2b  |
| game-framework   | progression                | F-13.12.2c  |
| game-framework   | progression                | F-13.12.3a  |
| game-framework   | progression                | F-13.12.3b  |
| game-framework   | progression                | F-13.12.3c  |
| game-framework   | progression                | F-13.12.4   |
| game-framework   | progression                | F-13.12.5   |
| game-framework   | progression                | F-13.12.6a  |
| game-framework   | progression                | F-13.12.6b  |
| game-framework   | progression                | F-13.12.6c  |
| game-framework   | progression                | F-13.12.7   |
| game-framework   | progression                | F-13.12.8a  |
| game-framework   | progression                | F-13.12.8b  |
| game-framework   | progression                | F-13.12.8c  |
| game-framework   | progression                | F-13.12.9   |
| game-framework   | progression                | F-13.12.10  |
| game-framework   | social                     | F-13.13.1a  |
| game-framework   | social                     | F-13.13.1b  |
| game-framework   | social                     | F-13.13.1c  |
| game-framework   | social                     | F-13.13.1d  |
| game-framework   | social                     | F-13.13.2   |
| game-framework   | social                     | F-13.13.3a  |
| game-framework   | social                     | F-13.13.3b  |
| game-framework   | social                     | F-13.13.3c  |
| game-framework   | social                     | F-13.13.3d  |
| game-framework   | social                     | F-13.13.4   |
| game-framework   | social                     | F-13.13.5a  |
| game-framework   | social                     | F-13.13.5b  |
| game-framework   | social                     | F-13.13.5c  |
| game-framework   | social                     | F-13.13.6a  |
| game-framework   | social                     | F-13.13.6b  |
| game-framework   | social                     | F-13.13.6c  |
| game-framework   | social                     | F-13.13.7   |
| game-framework   | social                     | F-13.13.8   |
| game-framework   | social                     | F-13.13.9   |
| game-framework   | social                     | F-13.13.10a |
| game-framework   | social                     | F-13.13.10b |
| game-framework   | social                     | F-13.13.10c |
| game-framework   | social                     | F-13.13.10d |
| game-framework   | building-survival          | F-13.14.1   |
| game-framework   | building-survival          | F-13.14.2   |
| game-framework   | building-survival          | F-13.14.3   |
| game-framework   | building-survival          | F-13.14.4   |
| game-framework   | building-survival          | F-13.14.5a  |
| game-framework   | building-survival          | F-13.14.5b  |
| game-framework   | building-survival          | F-13.14.5c  |
| game-framework   | building-survival          | F-13.14.6a  |
| game-framework   | building-survival          | F-13.14.6b  |
| game-framework   | building-survival          | F-13.14.6c  |
| game-framework   | building-survival          | F-13.14.6d  |
| game-framework   | building-survival          | F-13.14.7a  |
| game-framework   | building-survival          | F-13.14.7b  |
| game-framework   | building-survival          | F-13.14.7c  |
| game-framework   | building-survival          | F-13.14.8   |
| game-framework   | building-survival          | F-13.14.9a  |
| game-framework   | building-survival          | F-13.14.9b  |
| game-framework   | building-survival          | F-13.14.9c  |
| game-framework   | pets-mounts                | F-13.15.1   |
| game-framework   | pets-mounts                | F-13.15.2   |
| game-framework   | pets-mounts                | F-13.15.3a  |
| game-framework   | pets-mounts                | F-13.15.3b  |
| game-framework   | pets-mounts                | F-13.15.3c  |
| game-framework   | pets-mounts                | F-13.15.3d  |
| game-framework   | pets-mounts                | F-13.15.4   |
| game-framework   | pets-mounts                | F-13.15.5a  |
| game-framework   | pets-mounts                | F-13.15.5b  |
| game-framework   | pets-mounts                | F-13.15.5c  |
| game-framework   | weapon-system              | F-13.16.1   |
| game-framework   | weapon-system              | F-13.16.2a  |
| game-framework   | weapon-system              | F-13.16.2b  |
| game-framework   | weapon-system              | F-13.16.2c  |
| game-framework   | weapon-system              | F-13.16.3   |
| game-framework   | weapon-system              | F-13.16.4a  |
| game-framework   | weapon-system              | F-13.16.4b  |
| game-framework   | weapon-system              | F-13.16.4c  |
| game-framework   | weapon-system              | F-13.16.4d  |
| game-framework   | weapon-system              | F-13.16.5a  |
| game-framework   | weapon-system              | F-13.16.5b  |
| game-framework   | weapon-system              | F-13.16.5c  |
| game-framework   | weapon-system              | F-13.16.6a  |
| game-framework   | weapon-system              | F-13.16.6b  |
| game-framework   | weapon-system              | F-13.16.6c  |
| game-framework   | weapon-system              | F-13.16.6d  |
| game-framework   | traversal-interaction      | F-13.17.1   |
| game-framework   | traversal-interaction      | F-13.17.2   |
| game-framework   | traversal-interaction      | F-13.17.3   |
| game-framework   | traversal-interaction      | F-13.17.4a  |
| game-framework   | traversal-interaction      | F-13.17.4b  |
| game-framework   | traversal-interaction      | F-13.17.4c  |
| game-framework   | traversal-interaction      | F-13.17.4d  |
| game-framework   | traversal-interaction      | F-13.17.4e  |
| game-framework   | traversal-interaction      | F-13.17.5a  |
| game-framework   | traversal-interaction      | F-13.17.5b  |
| game-framework   | traversal-interaction      | F-13.17.5c  |
| game-framework   | traversal-interaction      | F-13.17.6   |
| game-framework   | traversal-interaction      | F-13.17.7   |
| game-framework   | stealth-cover              | F-13.18.1   |
| game-framework   | stealth-cover              | F-13.18.2   |
| game-framework   | stealth-cover              | F-13.18.3   |
| game-framework   | stealth-cover              | F-13.18.4   |
| game-framework   | stealth-cover              | F-13.18.5   |
| game-framework   | npc-simulation             | F-13.19.1   |
| game-framework   | npc-simulation             | F-13.19.2   |
| game-framework   | npc-simulation             | F-13.19.3a  |
| game-framework   | npc-simulation             | F-13.19.3b  |
| game-framework   | npc-simulation             | F-13.19.3c  |
| game-framework   | npc-simulation             | F-13.19.4a  |
| game-framework   | npc-simulation             | F-13.19.4b  |
| game-framework   | npc-simulation             | F-13.19.4c  |
| game-framework   | npc-simulation             | F-13.19.5   |
| game-framework   | npc-simulation             | F-13.19.6   |
| game-framework   | npc-simulation             | F-13.19.7   |
| game-framework   | npc-simulation             | F-13.19.8   |
| game-framework   | npc-simulation             | F-13.19.9   |
| game-framework   | npc-simulation             | F-13.19.10  |
| game-framework   | npc-simulation             | F-13.19.11  |
| game-framework   | npc-simulation             | F-13.19.12  |
| game-framework   | fog-of-war                 | F-13.20.1   |
| game-framework   | fog-of-war                 | F-13.20.2   |
| game-framework   | fog-of-war                 | F-13.20.3   |
| game-framework   | fog-of-war                 | F-13.20.4   |
| game-framework   | turn-based                 | F-13.21.1   |
| game-framework   | turn-based                 | F-13.21.2   |
| game-framework   | turn-based                 | F-13.21.3   |
| game-framework   | turn-based                 | F-13.21.4   |
| game-framework   | turn-based                 | F-13.21.5   |
| game-framework   | racing                     | F-13.22.1   |
| game-framework   | racing                     | F-13.22.2   |
| game-framework   | racing                     | F-13.22.3a  |
| game-framework   | racing                     | F-13.22.3b  |
| game-framework   | racing                     | F-13.22.3c  |
| game-framework   | racing                     | F-13.22.4   |
| game-framework   | racing                     | F-13.22.5   |
| game-framework   | monetization               | F-13.23.1   |
| game-framework   | monetization               | F-13.23.2   |
| game-framework   | monetization               | F-13.23.3a  |
| game-framework   | monetization               | F-13.23.3b  |
| game-framework   | monetization               | F-13.23.3c  |
| game-framework   | monetization               | F-13.23.3d  |
| game-framework   | monetization               | F-13.23.4   |
| game-framework   | monetization               | F-13.23.5a  |
| game-framework   | monetization               | F-13.23.5b  |
| game-framework   | monetization               | F-13.23.5c  |
| game-framework   | monetization               | F-13.23.5d  |
| game-framework   | monetization               | F-13.23.6a  |
| game-framework   | monetization               | F-13.23.6b  |
| game-framework   | monetization               | F-13.23.6c  |
| game-framework   | monetization               | F-13.23.7   |
| game-framework   | monetization               | F-13.23.8   |
| game-framework   | monetization               | F-13.23.9a  |
| game-framework   | monetization               | F-13.23.9b  |
| game-framework   | monetization               | F-13.23.9c  |
| game-framework   | monetization               | F-13.23.9d  |
| game-framework   | game-modes-misc            | F-13.24.1   |
| game-framework   | game-modes-misc            | F-13.24.2   |
| game-framework   | game-modes-misc            | F-13.24.3   |
| game-framework   | game-modes-misc            | F-13.24.4a  |
| game-framework   | game-modes-misc            | F-13.24.4b  |
| game-framework   | game-modes-misc            | F-13.24.4c  |
| game-framework   | game-modes-misc            | F-13.24.5   |
| game-framework   | game-modes-misc            | F-13.24.6   |
| game-framework   | game-modes-misc            | F-13.24.7a  |
| game-framework   | game-modes-misc            | F-13.24.7b  |
| game-framework   | game-modes-misc            | F-13.24.7c  |
| game-framework   | game-modes-misc            | F-13.24.8a  |
| game-framework   | game-modes-misc            | F-13.24.8b  |
| game-framework   | game-modes-misc            | F-13.24.8c  |
| game-framework   | camera-system              | F-13.25.1   |
| game-framework   | camera-system              | F-13.25.2   |
| game-framework   | camera-system              | F-13.25.3   |
| game-framework   | camera-system              | F-13.25.4   |
| game-framework   | camera-system              | F-13.25.5   |
| game-framework   | camera-system              | F-13.25.6   |
| game-framework   | camera-system              | F-13.25.7   |
| game-framework   | camera-system              | F-13.25.8   |
| game-framework   | camera-system              | F-13.25.9   |
| game-framework   | camera-system              | F-13.25.10  |
| game-framework   | camera-system              | F-13.25.11  |
| game-framework   | camera-system              | F-13.25.12  |
| game-framework   | camera-system              | F-13.25.13  |
| game-framework   | camera-system              | F-13.25.14  |
| game-framework   | camera-system              | F-13.25.15  |
| game-framework   | camera-system              | F-13.25.16  |
| game-framework   | camera-system              | F-13.25.17  |
| game-framework   | camera-system              | F-13.25.18  |
| game-framework   | camera-system              | F-13.25.19  |
| game-framework   | camera-system              | F-13.25.20  |
| game-framework   | camera-system              | F-13.25.21  |
| game-framework   | camera-system              | F-13.25.22  |
| game-framework   | camera-system              | F-13.25.23  |
| game-framework   | camera-system              | F-13.25.24  |
| game-framework   | camera-system              | F-13.25.25  |
| game-framework   | camera-system              | F-13.25.26  |
| game-framework   | camera-system              | F-13.25.27  |
| game-framework   | camera-system              | F-13.25.28  |
| game-framework   | camera-system              | F-13.25.29  |
| game-framework   | camera-system              | F-13.25.30  |
| game-framework   | camera-system              | F-13.25.31  |
| game-framework   | camera-system              | F-13.25.32  |
| game-framework   | camera-system              | F-13.25.33  |
| game-framework   | camera-system              | F-13.25.34  |
| game-framework   | camera-system              | F-13.25.35  |
| game-framework   | camera-system              | F-13.25.36  |
| game-framework   | camera-system              | F-13.25.37  |
| game-framework   | camera-system              | F-13.25.38  |
| game-framework   | camera-system              | F-13.25.39  |
| game-framework   | camera-system              | F-13.25.40  |
| game-framework   | minigames                  | F-13.26.1   |
| game-framework   | minigames                  | F-13.26.2   |
| game-framework   | minigames                  | F-13.26.3   |
| game-framework   | minigames                  | F-13.26.4   |
| game-framework   | minigames                  | F-13.26.5a  |
| game-framework   | minigames                  | F-13.26.5b  |
| game-framework   | minigames                  | F-13.26.5c  |
| game-framework   | minigames                  | F-13.26.5d  |
| game-framework   | minigames                  | F-13.26.6   |
| game-framework   | minigames                  | F-13.26.7   |
| game-framework   | minigames                  | F-13.26.8   |
| game-framework   | block-voxel                | F-13.27.1   |
| game-framework   | block-voxel                | F-13.27.2   |
| game-framework   | block-voxel                | F-13.27.3   |
| game-framework   | block-voxel                | F-13.27.4   |
| game-framework   | block-voxel                | F-13.27.5   |
| game-framework   | block-voxel                | F-13.27.6a  |
| game-framework   | block-voxel                | F-13.27.6b  |
| game-framework   | block-voxel                | F-13.27.6c  |
| game-framework   | block-voxel                | F-13.27.7a  |
| game-framework   | block-voxel                | F-13.27.7b  |
| game-framework   | block-voxel                | F-13.27.7c  |
| game-framework   | block-voxel                | F-13.27.7d  |
| game-framework   | block-voxel                | F-13.27.8a  |
| game-framework   | block-voxel                | F-13.27.8b  |
| game-framework   | block-voxel                | F-13.27.8c  |
| game-framework   | block-voxel                | F-13.27.8d  |
| game-framework   | advertising                | F-13.28.1   |
| game-framework   | advertising                | F-13.28.2   |
| game-framework   | advertising                | F-13.28.3   |
| game-framework   | advertising                | F-13.28.4   |
| platform         | window-display             | F-14.1.1    |
| platform         | window-display             | F-14.1.2    |
| platform         | window-display             | F-14.1.3    |
| platform         | window-display             | F-14.1.4    |
| platform         | window-display             | F-14.1.5    |
| platform         | window-display             | F-14.1.6    |
| platform         | os-integration             | F-14.2.1    |
| platform         | os-integration             | F-14.2.2    |
| platform         | os-integration             | F-14.2.3    |
| platform         | os-integration             | F-14.2.4    |
| platform         | os-integration             | F-14.2.5    |
| platform         | os-integration             | F-14.2.6    |
| platform         | threading-async            | F-14.3.1    |
| platform         | threading-async            | F-14.3.2    |
| platform         | threading-async            | F-14.3.3    |
| platform         | threading-async            | F-14.3.4    |
| platform         | threading-async            | F-14.3.5    |
| platform         | crash-reporting            | F-14.4.1    |
| platform         | crash-reporting            | F-14.4.2    |
| platform         | crash-reporting            | F-14.4.3    |
| platform         | crash-reporting            | F-14.4.4    |
| platform         | crash-reporting            | F-14.4.5    |
| platform         | crash-reporting            | F-14.4.6    |
| platform         | platform-services          | F-14.5.1    |
| platform         | platform-services          | F-14.5.2    |
| platform         | platform-services          | F-14.5.3    |
| platform         | platform-services          | F-14.5.4    |
| platform         | platform-services          | F-14.5.5    |
| platform         | platform-services          | F-14.5.6    |
| platform         | platform-services          | F-14.5.7    |
| platform         | filesystem                 | F-14.6.1    |
| platform         | filesystem                 | F-14.6.2    |
| platform         | filesystem                 | F-14.6.3    |
| platform         | filesystem                 | F-14.6.4    |
| platform         | filesystem                 | F-14.6.5    |
| platform         | filesystem                 | F-14.6.6    |
| platform         | filesystem                 | F-14.6.7    |
| tools-editor     | editor-framework           | F-15.1.1    |
| tools-editor     | editor-framework           | F-15.1.2    |
| tools-editor     | editor-framework           | F-15.1.3    |
| tools-editor     | editor-framework           | F-15.1.4    |
| tools-editor     | editor-framework           | F-15.1.5    |
| tools-editor     | editor-framework           | F-15.1.6    |
| tools-editor     | editor-framework           | F-15.1.7    |
| tools-editor     | editor-framework           | F-15.1.8    |
| tools-editor     | editor-framework           | F-15.1.9    |
| tools-editor     | level-editor               | F-15.2.1    |
| tools-editor     | level-editor               | F-15.2.2    |
| tools-editor     | level-editor               | F-15.2.3    |
| tools-editor     | level-editor               | F-15.2.4    |
| tools-editor     | level-editor               | F-15.2.5    |
| tools-editor     | level-editor               | F-15.2.6    |
| tools-editor     | level-editor               | F-15.2.7    |
| tools-editor     | material-editor            | F-15.3.1    |
| tools-editor     | material-editor            | F-15.3.2    |
| tools-editor     | material-editor            | F-15.3.3    |
| tools-editor     | material-editor            | F-15.3.4    |
| tools-editor     | material-editor            | F-15.3.5    |
| tools-editor     | material-editor            | F-15.3.6    |
| tools-editor     | animation-editor           | F-15.4.1    |
| tools-editor     | animation-editor           | F-15.4.2    |
| tools-editor     | animation-editor           | F-15.4.3    |
| tools-editor     | animation-editor           | F-15.4.4    |
| tools-editor     | animation-editor           | F-15.4.5    |
| tools-editor     | animation-editor           | F-15.4.6    |
| tools-editor     | animation-editor           | F-15.4.7    |
| tools-editor     | profiling-tools            | F-15.5.1    |
| tools-editor     | profiling-tools            | F-15.5.2    |
| tools-editor     | profiling-tools            | F-15.5.3    |
| tools-editor     | profiling-tools            | F-15.5.4    |
| tools-editor     | profiling-tools            | F-15.5.5    |
| tools-editor     | profiling-tools            | F-15.5.6    |
| tools-editor     | profiling-tools            | F-15.5.7    |
| tools-editor     | world-building             | F-15.6.1    |
| tools-editor     | world-building             | F-15.6.2    |
| tools-editor     | world-building             | F-15.6.3    |
| tools-editor     | world-building             | F-15.6.4    |
| tools-editor     | world-building             | F-15.6.5    |
| tools-editor     | world-building             | F-15.6.6    |
| tools-editor     | world-building             | F-15.6.7    |
| tools-editor     | world-building             | F-15.6.8    |
| tools-editor     | ai-governance              | F-15.7.1    |
| tools-editor     | ai-governance              | F-15.7.2    |
| tools-editor     | ai-governance              | F-15.7.3    |
| tools-editor     | ai-governance              | F-15.7.4    |
| tools-editor     | ai-governance              | F-15.7.5    |
| tools-editor     | ai-governance              | F-15.7.6    |
| tools-editor     | ai-governance              | F-15.7.7    |
| tools-editor     | ai-governance              | F-15.7.8    |
| tools-editor     | logic-graph                | F-15.8.1    |
| tools-editor     | logic-graph                | F-15.8.2    |
| tools-editor     | logic-graph                | F-15.8.3    |
| tools-editor     | logic-graph                | F-15.8.4    |
| tools-editor     | logic-graph                | F-15.8.5a   |
| tools-editor     | logic-graph                | F-15.8.5b   |
| tools-editor     | logic-graph                | F-15.8.5c   |
| tools-editor     | logic-graph                | F-15.8.6    |
| tools-editor     | logic-graph                | F-15.8.7    |
| tools-editor     | logic-graph                | F-15.8.8    |
| tools-editor     | logic-graph                | F-15.8.9    |
| tools-editor     | logic-graph                | F-15.8.10   |
| tools-editor     | logic-graph                | F-15.8.11   |
| tools-editor     | logic-graph                | F-15.8.12   |
| tools-editor     | logic-graph                | F-15.8.13   |
| tools-editor     | logic-graph                | F-15.8.14   |
| tools-editor     | ai-assistant               | F-15.9.1a   |
| tools-editor     | ai-assistant               | F-15.9.1b   |
| tools-editor     | ai-assistant               | F-15.9.1c   |
| tools-editor     | ai-assistant               | F-15.9.2    |
| tools-editor     | ai-assistant               | F-15.9.3    |
| tools-editor     | ai-assistant               | F-15.9.4    |
| tools-editor     | ai-assistant               | F-15.9.5    |
| tools-editor     | ai-assistant               | F-15.9.6a   |
| tools-editor     | ai-assistant               | F-15.9.6b   |
| tools-editor     | ai-assistant               | F-15.9.6c   |
| tools-editor     | ai-assistant               | F-15.9.7    |
| tools-editor     | ai-assistant               | F-15.9.8    |
| tools-editor     | ai-assistant               | F-15.9.9    |
| tools-editor     | ai-assistant               | F-15.9.10   |
| tools-editor     | version-control            | F-15.10.1   |
| tools-editor     | version-control            | F-15.10.2   |
| tools-editor     | version-control            | F-15.10.3   |
| tools-editor     | version-control            | F-15.10.4   |
| tools-editor     | version-control            | F-15.10.5   |
| tools-editor     | version-control            | F-15.10.6   |
| tools-editor     | version-control            | F-15.10.7   |
| tools-editor     | version-control            | F-15.10.8   |
| tools-editor     | shared-cache               | F-15.11.1   |
| tools-editor     | shared-cache               | F-15.11.2   |
| tools-editor     | shared-cache               | F-15.11.3   |
| tools-editor     | shared-cache               | F-15.11.4   |
| tools-editor     | shared-cache               | F-15.11.5   |
| tools-editor     | shared-cache               | F-15.11.6   |
| tools-editor     | shared-cache               | F-15.11.7   |
| tools-editor     | shared-cache               | F-15.11.8   |
| tools-editor     | remote-editing             | F-15.12.1   |
| tools-editor     | remote-editing             | F-15.12.2   |
| tools-editor     | remote-editing             | F-15.12.3   |
| tools-editor     | remote-editing             | F-15.12.4   |
| tools-editor     | remote-editing             | F-15.12.5   |
| tools-editor     | remote-editing             | F-15.12.6   |
| tools-editor     | remote-editing             | F-15.12.7   |
| tools-editor     | remote-editing             | F-15.12.8   |
| tools-editor     | remote-editing             | F-15.12.9   |
| tools-editor     | remote-editing             | F-15.12.10  |
| tools-editor     | remote-editing             | F-15.12.11  |
| tools-editor     | remote-editing             | F-15.12.12  |
| tools-editor     | remote-editing             | F-15.12.13  |
| tools-editor     | remote-editing             | F-15.12.14  |
| tools-editor     | localization-editor        | F-15.13.1   |
| tools-editor     | localization-editor        | F-15.13.2   |
| tools-editor     | localization-editor        | F-15.13.3   |
| tools-editor     | deployment                 | F-15.14.1   |
| tools-editor     | deployment                 | F-15.14.2   |
| tools-editor     | deployment                 | F-15.14.3   |
| tools-editor     | deployment                 | F-15.14.4   |
| tools-editor     | deployment                 | F-15.14.5   |
| tools-editor     | deployment                 | F-15.14.6   |
| tools-editor     | deployment                 | F-15.14.7   |
| tools-editor     | deployment                 | F-15.14.8   |
| tools-editor     | launcher                   | F-15.15.1   |
| tools-editor     | launcher                   | F-15.15.2   |
| tools-editor     | launcher                   | F-15.15.3   |
| tools-editor     | launcher                   | F-15.15.4   |
| tools-editor     | launcher                   | F-15.15.5   |
| tools-editor     | launcher                   | F-15.15.6   |
| tools-editor     | mod-support                | F-15.16.1   |
| tools-editor     | mod-support                | F-15.16.2   |
| tools-editor     | mod-support                | F-15.16.3   |
| tools-editor     | mod-support                | F-15.16.4   |
| tools-editor     | mod-support                | F-15.16.5   |
| tools-editor     | mod-support                | F-15.16.6   |
| tools-editor     | asset-store                | F-15.17.1   |
| tools-editor     | asset-store                | F-15.17.2   |
| tools-editor     | asset-store                | F-15.17.3   |
| tools-editor     | asset-store                | F-15.17.4   |
| tools-editor     | asset-store                | F-15.17.5   |
| tools-editor     | asset-store                | F-15.17.6   |
| tools-editor     | asset-store                | F-15.17.7   |
| tools-editor     | asset-store                | F-15.17.8   |
| tools-editor     | server-infrastructure      | F-15.18.1   |
| tools-editor     | server-infrastructure      | F-15.18.2   |
| tools-editor     | server-infrastructure      | F-15.18.3   |
| tools-editor     | server-infrastructure      | F-15.18.4   |
| tools-editor     | server-infrastructure      | F-15.18.5   |
| tools-editor     | server-infrastructure      | F-15.18.6   |
| tools-editor     | server-infrastructure      | F-15.18.7   |
| tools-editor     | server-infrastructure      | F-15.18.8   |
| tools-editor     | server-infrastructure      | F-15.18.9   |
| tools-editor     | server-infrastructure      | F-15.18.10  |
| tools-editor     | documentation              | F-15.19.1   |
| tools-editor     | documentation              | F-15.19.2   |
| tools-editor     | documentation              | F-15.19.3   |
| tools-editor     | documentation              | F-15.19.4   |
| tools-editor     | documentation              | F-15.19.5   |
| tools-editor     | documentation              | F-15.19.6   |
| tools-editor     | documentation              | F-15.19.7   |
| tools-editor     | cloud-build                | F-15.24.1   |
| tools-editor     | cloud-build                | F-15.24.2   |
| tools-editor     | cloud-build                | F-15.24.3   |
| tools-editor     | cloud-build                | F-15.24.4   |
| tools-editor     | cloud-build                | F-15.24.5   |
| tools-editor     | cloud-build                | F-15.24.6   |
| tools-editor     | cloud-build                | F-15.24.7   |

1. **core-runtime** — Archetype-Based Dense Storage
2. **core-runtime** — Sparse Component Storage
3. **core-runtime** — Archetype Graph and Edge Caching
4. **core-runtime** — Static and Dynamic Component Registration
5. **core-runtime** — Tag Components (Zero-Size)
6. **core-runtime** — Shared Components
7. **core-runtime** — Buffer Components (Dynamic Arrays)
8. **core-runtime** — Enableable Components
9. **core-runtime** — Component Hooks
10. **core-runtime** — Component Bundles and Required Components
11. **core-runtime** — Entity Lifecycle with Generational Indices
12. **core-runtime** — Cleanup Components and Deferred Destruction
13. **core-runtime** — Entity Names and Path Lookup
14. **core-runtime** — Entity Relationships (Pairs)
15. **core-runtime** — Relationship Properties
16. **core-runtime** — Built-In Parent-Child Hierarchy
17. **core-runtime** — Composable Archetype Queries
18. **core-runtime** — Query Sorting and Grouping
19. **core-runtime** — Query Variables and Pattern Matching
20. **core-runtime** — Automatic Parallel Iteration
21. **core-runtime** — Component Aspects
22. **core-runtime** — Tick-Based Change Detection
23. **core-runtime** — World Resources
24. **core-runtime** — Non-Send Resources
25. **core-runtime** — Dependency Resolution and Topological Ordering
26. **core-runtime** — System Groups and Phases
27. **core-runtime** — System Run Criteria and Conditions
28. **core-runtime** — Ambiguity Detection
29. **core-runtime** — Exclusive Systems
30. **core-runtime** — Event-Triggered Observers
31. **core-runtime** — Custom Entity Events
32. **core-runtime** — Deferred Structural Changes via Command Buffers
33. **core-runtime** — Parallel Command Recording
34. **core-runtime** — Multiple World Support
35. **core-runtime** — Entity Migration Between Worlds
36. **core-runtime** — Prefab Entities with Inheritance
37. **core-runtime** — Prefab Children and Nested Prefabs
38. **core-runtime** — ECS-Integrated State Machine
39. **core-runtime** — Entity-Based Scene Hierarchy
40. **core-runtime** — Hierarchy Traversal Iterators
41. **core-runtime** — Cascading Lifecycle Propagation
42. **core-runtime** — Hierarchical Transform Propagation
43. **core-runtime** — Transform Dirty Tracking
44. **core-runtime** — Spatial Partitioning Index
45. **core-runtime** — Spatial Scene Queries
46. **core-runtime** — Global Type Registry
47. **core-runtime** — Structured Type Descriptors
48. **core-runtime** — Reflective Property Access
49. **core-runtime** — Collection Reflection
50. **core-runtime** — Type-Erased Dynamic Values
51. **core-runtime** — Custom Type and Field Attributes
52. **core-runtime** — Trait Object Registration and Dispatch
53. **core-runtime** — Compact Binary Serialization Format
54. **core-runtime** — Zero-Copy Deserialization for Read-Only Data
55. **core-runtime** — Human-Readable Text Serialization
56. **core-runtime** — Schema Versioning with Semantic Version Tags
57. **core-runtime** — Data Migration Pipeline
58. **core-runtime** — Asset-Aware Serialization with Handle Resolution
59. **core-runtime** — Full Scene Serialization and Deserialization
60. **core-runtime** — Typed Double-Buffered Event Channels
61. **core-runtime** — Persistent Event Streams with Cursor-Based Reading
62. **core-runtime** — Component Lifecycle Observers
63. **core-runtime** — Deferred Command Buffers
64. **core-runtime** — Reactive Query Notifications
65. **core-runtime** — Typed Singleton Resources for Shared State
66. **core-runtime** — Cross-World Event Bridges
67. **core-runtime** — Declarative Plugin Registration
68. **core-runtime** — Plugin Groups and Presets
69. **core-runtime** — Explicit Plugin Dependency Declaration
70. **core-runtime** — Plugin Load Order Resolution
71. **core-runtime** — Hot Reload of Gameplay Plugins
72. **core-runtime** — Semantic Versioning and ABI Stability Metadata
73. **core-runtime** — Capability Negotiation for Optional Features
74. **core-runtime** — Per-Frame Arena Allocator
75. **core-runtime** — Scoped Arena Allocator with Nested Lifetimes
76. **core-runtime** — Typed Pool Allocator
77. **core-runtime** — Generational Index Handles
78. **core-runtime** — Slot Map Container
79. **core-runtime** — Per-Subsystem Memory Budgets
80. **core-runtime** — Memory Profiling and Telemetry Hooks
81. **core-runtime** — Allocation Tagging and Categorization
82. **core-runtime** — Arbitrary Precision Numeric Types
83. **core-runtime** — Platform I/O Backend Abstraction
84. **core-runtime** — Completion-Based Async Execution Model
85. **core-runtime** — Async File I/O Operations
86. **core-runtime** — Async Network Socket I/O
87. **core-runtime** — Async Audio Stream I/O
88. **core-runtime** — Scatter-Gather and Vectored I/O
89. **core-runtime** — I/O Priority and Deadline Scheduling
90. **core-runtime** — Cooperative I/O Cancellation
91. **core-runtime** — Registered Buffer Pools and Zero-Copy Transfers
92. **core-runtime** — Shared BVH Spatial Index
93. **core-runtime** — Incremental BVH Updates
94. **core-runtime** — Hierarchical Grid / Octree Index
95. **core-runtime** — Unified Spatial Query API
96. **core-runtime** — Batch and Parallel Spatial Queries
97. **core-runtime** — Physics Broadphase Integration
98. **core-runtime** — Rendering Culling Integration
99. **core-runtime** — Network Interest Management Integration
100. **core-runtime** — AI Perception and Gameplay Integration
101. **rendering** — GPU Backend Trait
102. **rendering** — Command Buffer Abstraction
103. **rendering** — Pipeline State Abstraction
104. **rendering** — Metal Backend (Swift-to-C-to-Bindgen)
105. **rendering** — D3D12 Backend (COM-to-Bindgen)
106. **rendering** — Vulkan Backend (C-to-Bindgen)
107. **rendering** — Memory Sub-Allocation
108. **rendering** — GPU State Tracking
109. **rendering** — Barrier Optimization
110. **rendering** — Work Graph Support
111. **rendering** — Cross-Backend Feature Emulation
112. **rendering** — GPU Performance Queries and Profiling
113. **rendering** — Declarative Pass Registration
114. **rendering** — Capability Gating
115. **rendering** — Transient Resource Declaration
116. **rendering** — Resource Aliasing
117. **rendering** — Automatic Barrier Insertion
118. **rendering** — Multi-Queue Scheduling
119. **rendering** — Pass Ordering and Topological Sort
120. **rendering** — Budget Culling
121. **rendering** — Multi-View Execution
122. **rendering** — Parallel Command Encoding
123. **rendering** — Streaming Integration
124. **rendering** — Graph Compilation and Caching
125. **rendering** — Render Graph Diagnostics
126. **rendering** — Direct Lighting
127. **rendering** — GPU Frustum Culling
128. **rendering** — Backface Culling
129. **rendering** — Occlusion Culling (HZB Two-Phase)
130. **rendering** — Orthographic Projection
131. **rendering** — Perspective Projection (Reverse-Z)
132. **rendering** — GPU-Driven Instancing
133. **rendering** — Render-to-Texture
134. **rendering** — Cubemaps
135. **rendering** — Scene Capture
136. **rendering** — Dynamic Resolution
137. **rendering** — Subsurface Scattering
138. **rendering** — Alpha Mask Cutouts
139. **rendering** — Forward+ Lighting (Tiled/Clustered)
140. **rendering** — Deferred Lighting (G-Buffer)
141. **rendering** — PBR Materials (Cook-Torrance BRDF)
142. **rendering** — Extended BSDF Materials
143. **rendering** — Transparent Objects
144. **rendering** — Material Instances
145. **rendering** — Material Layers and Blending
146. **rendering** — Decal Rendering
147. **rendering** — Shading Model Variants
148. **rendering** — Stochastic Many-Light Sampling
149. **rendering** — Cascaded Shadow Maps
150. **rendering** — Soft Shadows (PCF / PCSS / RT)
151. **rendering** — Ambient Occlusion (SSAO / GTAO / RT AO)
152. **rendering** — Virtual Shadow Maps
153. **rendering** — Contact Shadows
154. **rendering** — Distance Field Shadows
155. **rendering** — Capsule Shadows
156. **rendering** — Order-Independent Transparency (OIT)
157. **rendering** — Volumetric Shadow Maps
158. **rendering** — Area Lights (Rect/Sphere)
159. **rendering** — Sky Light (IBL)
160. **rendering** — IES Light Profiles
161. **rendering** — Light Functions
162. **rendering** — Acceleration Structure Management (BLAS/TLAS)
163. **rendering** — Ray Traced Reflections (Hybrid SSR + RT)
164. **rendering** — Ray Traced Indirect Lighting
165. **rendering** — Real-Time Global Illumination (DDGI)
166. **rendering** — Path Tracing (Reference Renderer)
167. **rendering** — Ray Traced Subsurface Transmission
168. **rendering** — Surfel-Based Global Illumination
169. **rendering** — ReSTIR Sampling Framework
170. **rendering** — Real-Time Production Path Tracing
171. **rendering** — Opacity Micromaps
172. **rendering** — Shader Execution Reordering
173. **rendering** — Neural Denoising (Ray Reconstruction)
174. **rendering** — RT Lens Flare
175. **rendering** — Voxel-Based Global Illumination
176. **rendering** — Neural Radiance Cache
177. **rendering** — Stochastic Screen-Space Reflections
178. **rendering** — Temporal Anti-Aliasing (TAA)
179. **rendering** — Temporal Super Resolution (TSR)
180. **rendering** — FXAA (Fast Approximate Anti-Aliasing)
181. **rendering** — MSAA (Multi-Sample Anti-Aliasing)
182. **rendering** — Checkerboard Rendering
183. **rendering** — Third-Party Upscaler Integration
184. **rendering** — Frame Generation
185. **rendering** — Latency Reduction
186. **rendering** — Procedural Sky (Bruneton/Hillaire Atmosphere)
187. **rendering** — Ray-Marched Volumetric Fog (Froxels)
188. **rendering** — Procedural Volumetric Clouds
189. **rendering** — God Rays
190. **rendering** — Distance and Height Fog
191. **rendering** — Water Simulation (FFT Ocean)
192. **rendering** — Heterogeneous Volumes (OpenVDB)
193. **rendering** — Voxel-Based Volumetric Clouds
194. **rendering** — Art-Directable Breaking Waves
195. **rendering** — Weather System
196. **rendering** — Strand-Based Hair Rendering
197. **rendering** — Card-Based Hair Rendering
198. **rendering** — Hair LOD System
199. **rendering** — Eye Rendering
200. **rendering** — Cloth Rendering
201. **rendering** — Skin Rendering (Subsurface Profiles)
202. **rendering** — Compute Software Rasterized Hair
203. **rendering** — Peach Fuzz (Vellus Hair)
204. **rendering** — Biometric Skin Model
205. **rendering** — Bloom
206. **rendering** — Depth of Field (Cinematic)
207. **rendering** — Motion Blur
208. **rendering** — Auto Exposure / Eye Adaptation
209. **rendering** — Tonemapping and Color Grading
210. **rendering** — Film Grain
211. **rendering** — Chromatic Aberration
212. **rendering** — Lens Flare
213. **rendering** — Vignette
214. **rendering** — Post-Process Materials
215. **rendering** — Local Exposure
216. **rendering** — Panini Projection
217. **rendering** — Screen-Space Cavity and Curvature
218. **rendering** — Post-Process Graph Editor
219. **rendering** — Render Proxy Extraction
220. **rendering** — Render Component System
221. **rendering** — Change Detection and Incremental Update
222. **rendering** — View and Camera Setup
223. **rendering** — Multi-View Rendering
224. **rendering** — Draw List Construction
225. **rendering** — GPU-Driven Batch Compaction
226. **rendering** — Material Parameter Binding
227. **rendering** — Debug Visualization and Gizmos
228. **rendering** — Buffer Visualization Modes
229. **rendering** — 2D and 3D Outline Rendering
230. **rendering** — Highlight and Glow Effect
231. **rendering** — Advanced Toon Shading
232. **rendering** — Cut-Through Visibility and Roof Fading
233. **rendering** — X-Ray and See-Through Silhouette Rendering
234. **rendering** — Transparent Glass and Crystal Rendering
235. **rendering** — Ocean Reflection and Refraction
236. **rendering** — Emission Maps and Emissive Materials
237. **rendering** — Heightmap Tessellation and Parallax
238. **rendering** — Fabric and Cloth Materials
239. **rendering** — Metal, Wood, Stone, and Natural Materials
240. **rendering** — Rubber, Wax, and Soft Surface Materials
241. **rendering** — Clearcoat and Multi-Layer Materials
242. **rendering** — Fully Custom Material Graphs
243. **geometry-world** — Meshlet Decomposition and Hierarchy
244. **geometry-world** — Two-Phase Occlusion Culling
245. **geometry-world** — Cluster and Triangle Culling
246. **geometry-world** — Mesh Shader Pipeline with Indirect Draw Fallback
247. **geometry-world** — Screen-Space Error LOD Selection
248. **geometry-world** — On-Demand Meshlet Page Streaming
249. **geometry-world** — Visibility Buffer Rendering
250. **geometry-world** — Heightfield Terrain with Tile-Based Streaming
251. **geometry-world** — Virtual Texture Clipmap
252. **geometry-world** — CDLOD / Geometry Clipmap LOD
253. **geometry-world** — Terrain Hole Masking
254. **geometry-world** — Splatmap Material Blending
255. **geometry-world** — Terrain Physics Collision
256. **geometry-world** — Large World Coordinate Support
257. **geometry-world** — Indoor Environments and Portal Visibility
258. **geometry-world** — Voxel Volume Representation
259. **geometry-world** — Hybrid Heightmap-Voxel Terrain
260. **geometry-world** — Planetary-Scale Voxel Sphere
261. **geometry-world** — Voxel Meshing Pipeline
262. **geometry-world** — Runtime Voxel Editing and Deformation
263. **geometry-world** — Voxel LOD and Streaming
264. **geometry-world** — GPU-Driven Instanced Foliage
265. **geometry-world** — Density Map and Rule-Based Procedural Placement
266. **geometry-world** — Billboard and Impostor LOD
267. **geometry-world** — GPU Vertex Shader Wind Animation
268. **geometry-world** — Character-Vegetation Interaction
269. **geometry-world** — Procedural Grass Blade Rendering
270. **geometry-world** — Tree Rendering System
271. **geometry-world** — FFT Ocean Wave Simulation
272. **geometry-world** — Shoreline and Depth-Based Blending
273. **geometry-world** — Underwater Rendering and Volume Effects
274. **geometry-world** — Water Caustics Projection
275. **geometry-world** — Water Reflection and Refraction
276. **geometry-world** — Flow Map River Simulation
277. **geometry-world** — Dynamic Foam Generation
278. **geometry-world** — Procedural Sky Model
279. **geometry-world** — Multi-Scattering Atmosphere with Aerial Perspective
280. **geometry-world** — Ray-Marched Volumetric Clouds
281. **geometry-world** — Cloud Shadow Map
282. **geometry-world** — Dynamic Time-of-Day System
283. **geometry-world** — Celestial Body Rendering
284. **geometry-world** — Environment Cubemap Capture
285. **geometry-world** — Node-Based Procedural Content Graph
286. **geometry-world** — Point Generation Nodes
287. **geometry-world** — Point Filtering and Transformation
288. **geometry-world** — Mesh and Instance Spawning from Points
289. **geometry-world** — Deterministic Seeding
290. **geometry-world** — Point Attributes and Metadata
291. **geometry-world** — Point Set Operations
292. **geometry-world** — Graph Control Flow (Loops, Branches, Subgraphs)
293. **geometry-world** — Non-Destructive Terrain Stamp System
294. **geometry-world** — Terrain Texture Stamps
295. **geometry-world** — Biome Distribution System
296. **geometry-world** — Rule-Based Vegetation Placement
297. **geometry-world** — Vegetation Clearing Along Splines
298. **geometry-world** — Spline-Based Road Generation
299. **geometry-world** — Road Network Generation
300. **geometry-world** — Spline SDF Optimization
301. **geometry-world** — Road Intersections and Junctions
302. **geometry-world** — Shape Grammar Building Generator
303. **geometry-world** — Modular Building Assembly
304. **geometry-world** — 2D Tile-Based WFC
305. **geometry-world** — 3D Voxel WFC
306. **geometry-world** — WFC Constraint Painting
307. **geometry-world** — Socket-Based Modular Assembly Engine
308. **geometry-world** — Procedural Object Generation Rules
309. **geometry-world** — Houdini Engine Procedural Object Pipeline
310. **geometry-world** — Hierarchical Modular Composition
311. **geometry-world** — Interactive PCG Authoring Tools
312. **geometry-world** — Artist-Guided Constraint Authoring
313. **geometry-world** — AI-Driven Content Generation
314. **geometry-world** — Constraint Satisfaction Solver
315. **geometry-world** — Runtime Chunk-Based Procedural Generation
316. **geometry-world** — GPU Compute Procedural Generation
317. **geometry-world** — Noise Function Library
318. **geometry-world** — Planetary Terrain Generation
319. **geometry-world** — City and Settlement Generation
320. **geometry-world** — Faction and Civilization Generation
321. **geometry-world** — Procedural Quest Generation
322. **geometry-world** — Dynamic Ecosystem Simulation
323. **geometry-world** — Civilization Time-Scale Simulation
324. **geometry-world** — Procedural Enemy and Creature Placement
325. **geometry-world** — Procedural Loot and Economy Distribution
326. **geometry-world** — Plate Tectonics and Geological Simulation
327. **geometry-world** — Climate and Atmospheric Simulation
328. **geometry-world** — Biome Classification and Distribution
329. **geometry-world** — Hydrological System and Water Body Generation
330. **geometry-world** — Geological Landform Generation
331. **geometry-world** — Earth Import and GIS Data Integration
332. **geometry-world** — Configurable Planet Parameters
333. **geometry-world** — Star System Generation and Stellar Lifecycle
334. **geometry-world** — Protoplanetary Disk and Accretion Simulation
335. **geometry-world** — Planetary Collision and Giant Impact Simulation
336. **geometry-world** — Gas Giant and Non-Terrestrial Planet Generation
337. **geometry-world** — Moon and Ring System Generation
338. **geometry-world** — Automatic Planet Type Classification
339. **geometry-world** — Galaxy Structure Generation
340. **geometry-world** — Supermassive Black Hole and Galactic Core
341. **geometry-world** — Dark Matter Halo and Large-Scale Structure
342. **geometry-world** — Stellar Collision and Merger Simulation
343. **geometry-world** — Black Hole Formation and Collision
344. **geometry-world** — Universe Generation Pipeline
345. **geometry-world** — Planetary Mineralogy and Resource Distribution
346. **geometry-world** — Server-Side Universe Generation and Sharding
347. **geometry-world** — Sparse Cosmic Data Storage
348. **geometry-world** — On-Demand Hierarchical Detail Resolution
349. **physics** — Deterministic Fixed-Timestep Integration
350. **physics** — Simulation Substeps
351. **physics** — Contact Resolution with Restitution and Friction
352. **physics** — Continuous Collision Detection
353. **physics** — Simulation Islands
354. **physics** — Body Sleeping and Wake-Up
355. **physics** — Cross-Zone Physics Continuity
356. **physics** — Character Controller
357. **physics** — Moving Platform System
358. **physics** — Surface Smoothing and Ground Conformance
359. **physics** — Broadphase Acceleration Structures
360. **physics** — Narrowphase Contact Generation
361. **physics** — Primitive and Convex Collision Shapes
362. **physics** — Triangle Mesh and Heightfield Shapes
363. **physics** — Compound Shapes
364. **physics** — Collision Filtering and Layers
365. **physics** — Collision Events
366. **physics** — Trigger Volumes
367. **physics** — Physics Material Assets
368. **physics** — Core Joint Types
369. **physics** — Advanced Joint Types
370. **physics** — Joint Motors and Limits
371. **physics** — Breakable Joints
372. **physics** — Ragdoll Configuration
373. **physics** — Joint Chains and Ropes
374. **physics** — Constraint Solvers
375. **physics** — Limb Severance and Joint Destruction
376. **physics** — Prosthetic and Limb Replacement
377. **physics** — Ray Casting
378. **physics** — Shape Casting (Sweep Tests)
379. **physics** — Overlap Tests
380. **physics** — Closest Point Queries
381. **physics** — Batch Query Execution
382. **physics** — Query Filtering and Custom Predicates
383. **physics** — Wheel Suspension Model
384. **physics** — Tire Friction Model
385. **physics** — Drivetrain Simulation
386. **physics** — Anti-Roll Bars and Stability Control
387. **physics** — Tracked Vehicle Simulation
388. **physics** — Hover Vehicle Simulation
389. **physics** — Server-Authoritative Vehicle Replication
390. **physics** — Voronoi Fracture Generation
391. **physics** — Pre-Fractured Mesh Authoring
392. **physics** — Runtime Fracture and Activation
393. **physics** — Progressive Damage Model
394. **physics** — Stress Propagation and Structural Collapse
395. **physics** — Debris Simulation and Lifecycle
396. **physics** — Debris Pooling and LOD
397. **physics** — Position-Based Dynamics Solver
398. **physics** — Cloth Simulation
399. **physics** — Cloth Self-Collision
400. **physics** — Two-Way Rigid Body Coupling
401. **physics** — Wind Interaction
402. **physics** — Cloth Tearing
403. **physics** — Cloth Level of Detail
404. **physics** — SPH Fluid Simulation
405. **physics** — FLIP/PIC Hybrid Simulation
406. **physics** — Eulerian Grid Fluid Solver
407. **physics** — Fluid Surface Reconstruction
408. **physics** — Water Surface Simulation
409. **physics** — Buoyancy and Drag Forces
410. **physics** — Two-Way Fluid-Rigid Body Coupling
411. **audio** — Sound Source Component
412. **audio** — Listener Component
413. **audio** — Hierarchical Mixer Bus Graph
414. **audio** — Voice Management and Priority System
415. **audio** — Streaming Playback
416. **audio** — Sample-Accurate Scheduling
417. **audio** — Audio Format and Codec Support
418. **audio** — 3D Sound Positioning and Doppler
419. **audio** — Distance Attenuation Curves
420. **audio** — HRTF Binaural Rendering
421. **audio** — Ambisonics Encoding and Decoding
422. **audio** — Occlusion and Obstruction Filtering
423. **audio** — Sound Propagation via Portals and Rays
424. **audio** — Reverb Zones and Early Reflections
425. **audio** — Parametric Filters
426. **audio** — Parametric Equalizer
427. **audio** — Algorithmic Reverb
428. **audio** — Convolution Reverb
429. **audio** — Compressor, Limiter, and Dynamics Processing
430. **audio** — Delay, Chorus, and Flanger
431. **audio** — Pitch Shifting
432. **audio** — Custom DSP Node Chains
433. **audio** — Vertical Re-Mixing (Layered Stems)
434. **audio** — Horizontal Re-Sequencing
435. **audio** — Transition Rules (Crossfade and Beat-Sync)
436. **audio** — Tempo and Beat Clock
437. **audio** — Stinger Playback
438. **audio** — Playlists and Weighted Randomization
439. **audio** — Dynamic Intensity Parameter
440. **audio** — Voice Chat Codec and Transport
441. **audio** — Jitter Buffer and Packet Loss Concealment
442. **audio** — Voice Activity Detection and Noise Suppression
443. **audio** — Text-to-Speech Integration
444. **audio** — Viseme Generation for Lip Sync
445. **audio** — Dialogue Playback and Queuing
446. **audio** — Branching Dialogue Graph
447. **audio** — Voice Chat Channel Management
448. **audio** — Acoustic Echo Cancellation
449. **input** — Keyboard Input Capture
450. **input** — Mouse Button, Motion, and Scroll Input
451. **input** — Unified Gamepad Abstraction
452. **input** — Multi-Touch and Pen Input
453. **input** — Device Hot-Plug Detection and Enumeration
454. **input** — Typed Input Actions
455. **input** — Input Mapping Contexts with Priority Stacking
456. **input** — Input Value Modifiers
457. **input** — Input Trigger Conditions
458. **input** — Runtime Key Rebinding with Conflict Detection
459. **input** — Platform-Aware Button Glyph Resolution
460. **input** — Input Recording and Playback
461. **input** — Combo Input Trees and Directional Sequences
462. **input** — Input Buffering and Ability Cancel Windows
463. **input** — Input Smoothing, Acceleration, and Aim Assist
464. **input** — Controller-Driven UI Interaction
465. **input** — Tap, Multi-Tap, and Long Press Recognition
466. **input** — Swipe Direction Recognition
467. **input** — Pinch, Rotate, and Pan Gestures
468. **input** — Gesture State Machines with Conflict Resolution
469. **input** — Custom Gesture Definition
470. **input** — Dual-Motor Rumble with Pattern Sequencing
471. **input** — DualSense Adaptive Trigger Effects
472. **input** — High-Definition Haptic Playback
473. **input** — Audio-Driven Haptic Generation
474. **input** — Custom Force Feedback Profiles
475. **input** — Head-Mounted Display Tracking
476. **input** — Motion Controller Input
477. **input** — Hand Tracking and Skeletal Input
478. **input** — Eye Tracking and Gaze Input
479. **input** — VR Controller Haptics
480. **ai** — Recast-Style NavMesh Generation
481. **ai** — NavMesh Tiling & Streaming
482. **ai** — A* Pathfinding on NavMesh
483. **ai** — String Pulling (Funnel Algorithm)
484. **ai** — Path Smoothing
485. **ai** — Dynamic Obstacle Carving
486. **ai** — NavMesh Links (Off-Mesh Connections)
487. **ai** — Incremental Tile Rebuild
488. **ai** — Background NavMesh Generation
489. **ai** — Destruction-Triggered NavMesh Updates
490. **ai** — Player-Built Structure Integration
491. **ai** — Multi-Size Agent NavMeshes
492. **ai** — Dynamic Area Cost Modification
493. **ai** — Hierarchical Pathfinding (HPA*)
494. **ai** — NavMesh Runtime Visualization
495. **ai** — RVO/ORCA Local Avoidance
496. **ai** — Obstacle Avoidance (Static Geometry)
497. **ai** — Core Steering Behaviors
498. **ai** — Steering Behavior Blending & Priority
499. **ai** — Formation Movement
500. **ai** — Group Steering & Cohesion
501. **ai** — Core Composite & Leaf Nodes
502. **ai** — Decorator Nodes
503. **ai** — Conditional Aborts
504. **ai** — Blackboard System
505. **ai** — Behavior Tree Assets & Serialization
506. **ai** — Subtree References & Reuse
507. **ai** — Runtime Debugging & Visualization
508. **ai** — Scoring Functions & Response Curves
509. **ai** — Action Selection & Compensation
510. **ai** — Considerations & Input Axes
511. **ai** — Dual Utility System
512. **ai** — Context-Based Reasoning
513. **ai** — World State Representation
514. **ai** — GOAP Forward-Search Planner
515. **ai** — Action Preconditions & Effects
516. **ai** — Plan Caching & Reuse
517. **ai** — Replanning Triggers
518. **ai** — Goal Prioritization
519. **ai** — Sight Sense (Cone + Line of Sight)
520. **ai** — Hearing Sense (Radius + Attenuation)
521. **ai** — Damage Sense
522. **ai** — Team & Faction Awareness
523. **ai** — Stimuli Registration & Expiration
524. **ai** — Sense Aging & Memory Decay
525. **ai** — Custom Senses & Perception Priority
526. **ai** — Smell Sense and Scent Trails
527. **ai** — Environmental Evidence and Footprint Detection
528. **ai** — AI Investigation Behavior
529. **ai** — Multi-Sense Tracking and Pursuit
530. **ai** — Flocking Behaviors (Separation, Alignment, Cohesion)
531. **ai** — Flow Field Navigation
532. **ai** — Flow Field Streaming & Caching
533. **ai** — Mass Entity Simulation
534. **ai** — AI Level of Detail (Processing Budget)
535. **ai** — Density Management
536. **ai** — Knowledge Sharing and Event Propagation
537. **ai** — Shared Awareness and Synchronized Group Reactions
538. **ai** — Faction-Based Behavioral Relationships
539. **ai** — Threat Table and Aggro Targeting
540. **ai** — Animal Tracking and Hunting Behaviors
541. **ai** — Cover Evaluation and Scoring
542. **ai** — Flanking and Pincer Behavior
543. **ai** — Squad Formation and Communication
544. **ai** — Suppressive Fire and Pinning
545. **ai** — Search and Investigation Patterns
546. **ai** — Retreat and Fallback Behavior
547. **networking** — Connection Handshake and Authentication
548. **networking** — Connection Lifecycle Management
549. **networking** — Reliable Ordered Channel
550. **networking** — Unreliable and Unordered Channels
551. **networking** — DTLS Encryption
552. **networking** — Packet Fragmentation, Reassembly, and MTU Discovery
553. **networking** — Bandwidth Estimation and Congestion Control
554. **networking** — Network Diagnostics and Quality Indicators
555. **networking** — Delta-Compressed Property Replication
556. **networking** — Component Replication with Schema Versioning
557. **networking** — Area-of-Interest Filtering
558. **networking** — Conditional and Tiered Replication
559. **networking** — Priority Scheduling and Bandwidth Budgeting
560. **networking** — Entity Dormancy
561. **networking** — Server RPC (Client-to-Server)
562. **networking** — Client RPC (Server-to-Client)
563. **networking** — Multicast RPC (Server-to-Group)
564. **networking** — RPC Reliability Modes
565. **networking** — Parameter Serialization and Validation
566. **networking** — Input Prediction and Server Reconciliation
567. **networking** — Input Buffering and Redundant Transmission
568. **networking** — Snapshot Interpolation
569. **networking** — Entity Extrapolation with Error Correction
570. **networking** — Server-Side Lag Compensation (Hitbox Rewinding)
571. **networking** — Jitter Buffer and Adaptive Tick Alignment
572. **networking** — Login and Authentication Services
573. **networking** — Skill-Based and Region-Based Matchmaking
574. **networking** — Lobby and Party System
575. **networking** — Dedicated Server Cluster Management
576. **networking** — Session Discovery and Reconnection
577. **networking** — Cross-Play Matchmaking and Account Linking
578. **networking** — Login Queue and Capacity Management
579. **networking** — Headless Dedicated Game Server
580. **networking** — Skill-Based Matchmaking Service
581. **networking** — State Recording with Snapshots and Deltas
582. **networking** — Deterministic Playback
583. **networking** — Seek, Fast-Forward, and Slow Motion
584. **networking** — Live Spectator Mode
585. **networking** — Kill Cam and Highlight Extraction
586. **networking** — World Sharding and Instancing
587. **networking** — Seamless Zone Transitions
588. **networking** — Dynamic Server Mesh
589. **networking** — Player Migration Between Servers
590. **networking** — Persistent World State and Database Integration
591. **networking** — Load Balancing and Auto-Scaling
592. **networking** — Cross-Shard Services
593. **networking** — Inter-Server Communication Bus
594. **networking** — Server-Side Cheat Detection
595. **networking** — Client Integrity Verification
596. **networking** — Behavioral Analysis and Anomaly Detection
597. **networking** — Economy Exploit Prevention
598. **networking** — Rate Limiting and Abuse Prevention
599. **animation** — GPU Compute Skinning
600. **animation** — GPU Keyframe Evaluation
601. **animation** — Animation Blending (Linear and Cubic)
602. **animation** — Animation Layers and Additive Blending
603. **animation** — Instanced Skeletal Animation
604. **animation** — Root Motion Extraction
605. **animation** — Animation Compression
606. **animation** — Animation Retargeting
607. **animation** — Animation Events and Notifies
608. **animation** — Animation Level of Detail
609. **animation** — GPU Blend Shape Accumulation
610. **animation** — Corrective Blend Shapes
611. **animation** — Facial Animation System
612. **animation** — Per-Vertex Animation Textures
613. **animation** — Morph Target Streaming
614. **animation** — Two-Bone IK Solver
615. **animation** — CCD IK Solver
616. **animation** — FABRIK IK Solver
617. **animation** — Ragdoll Physics (Partial and Full)
618. **animation** — Look-At and Aim Constraints
619. **animation** — Motion Matching
620. **animation** — Foot Placement and Procedural Locomotion
621. **animation** — Multi-Skeleton Procedural Locomotion
622. **animation** — Physics-Based Locomotion
623. **animation** — Procedural Attachment and Dismemberment
624. **animation** — Locomotion Diagnostics and Visualization
625. **animation** — Animation State Graph
626. **animation** — Transitions with Blend Profiles and Sync Markers
627. **animation** — Sub-State Machines
628. **animation** — State Machine Animation Layers
629. **animation** — State Variables and Conditions
630. **animation** — Sync Groups
631. **animation** — Animation Montages
632. **animation** — 1D and 2D Blend Spaces
633. **animation** — Aim Offset and Additive Aim Layers
634. **animation** — AI Animation Integration
635. **animation** — GPU Cloth Simulation
636. **animation** — Strand-Based Hair Simulation
637. **animation** — Card-Based Hair Rendering
638. **animation** — Hair LOD System
639. **animation** — Cloth-Body Interaction
640. **animation** — Hair Wind Response
641. **animation** — First-Person Camera Controller
642. **animation** — Procedural Weapon Sway and Bob
643. **animation** — Procedural Recoil and ADS Animation
644. **animation** — Weapon Equip, Inspect, and Dual Wield
645. **ui-2d** — Declarative Retained Widget Tree
646. **ui-2d** — Declarative UI Asset Format
647. **ui-2d** — Widget Pooling and Recycling
648. **ui-2d** — Flexbox and Grid Layout
649. **ui-2d** — Anchor and Constraint Layout
650. **ui-2d** — CSS-like Styling and Themes
651. **ui-2d** — Reactive Data Binding
652. **ui-2d** — Focus and Navigation System
653. **ui-2d** — Localization Hooks
654. **ui-2d** — World-Space UI Panels
655. **ui-2d** — VR-Optimized UI Interaction
656. **ui-2d** — Automatic Minimal Tree Diffing
657. **ui-2d** — Widget Animation System
658. **ui-2d** — UI Audio Feedback
659. **ui-2d** — Rich Text and Text Shaping
660. **ui-2d** — Text Input and Editing
661. **ui-2d** — Buttons, Sliders, and Toggle Controls
662. **ui-2d** — Combo Boxes and Dropdown Menus
663. **ui-2d** — Scroll Views and Virtualized List Views
664. **ui-2d** — Tooltips, Context Menus, and Modal Dialogs
665. **ui-2d** — Drag and Drop
666. **ui-2d** — Progress Bars and Loading Indicators
667. **ui-2d** — Health, Resource, and Cast Bars
668. **ui-2d** — Buff and Debuff Icons
669. **ui-2d** — Action Bars and Cooldown Indicators
670. **ui-2d** — Nameplates and World-Space Labels
671. **ui-2d** — Floating Combat Text and Damage Numbers
672. **ui-2d** — Minimap and World Map Overlays
673. **ui-2d** — Quest Tracker and Objective HUD
674. **ui-2d** — Chat System
675. **ui-2d** — Inventory Grids and Container Management
676. **ui-2d** — Compass Bar HUD
677. **ui-2d** — Off-Screen Objective Indicators
678. **ui-2d** — Procedural Minimap Generation
679. **ui-2d** — World Map Generation and Rendering
680. **ui-2d** — Artist-Authored Map Overlays and Post-Processing
681. **ui-2d** — Dynamic Map Markers and Quest Integration
682. **ui-2d** — Batched Quad Rendering
683. **ui-2d** — SDF Text Rendering
684. **ui-2d** — Vector Graphics Rendering
685. **ui-2d** — UI Atlas and Nine-Slice Rendering
686. **ui-2d** — Render-to-Texture for 3D-in-UI
687. **ui-2d** — World-Space and Diegetic UI
688. **ui-2d** — Anti-Aliased Edges and Clipping
689. **ui-2d** — Sprite Rendering and Sprite Sheets
690. **ui-2d** — Frame-Based Sprite Animation
691. **ui-2d** — 2D Skeletal Animation
692. **ui-2d** — Vector-Based 2D Rendering
693. **ui-2d** — Vector Skeletal Animation
694. **ui-2d** — Tilemap Rendering
695. **ui-2d** — Isometric and Hex Tilemap Support
696. **ui-2d** — Procedural 2D Tilemap Generation
697. **ui-2d** — 2D Camera System
698. **ui-2d** — 2D Rigid Body Physics
699. **ui-2d** — 2D Collision Shapes and Tilemap Colliders
700. **ui-2d** — 2D Joints and Constraints
701. **ui-2d** — 2D Spatial Queries
702. **ui-2d** — 2D Dynamic Lighting
703. **ui-2d** — 2D Particle Effects
704. **ui-2d** — On-Screen Virtual Controls
705. **ui-2d** — Mobile Gesture Integration for 2D Games
706. **ui-2d** — 2D State Replication
707. **ui-2d** — 2D Client Prediction and Rollback
708. **ui-2d** — Procedural 2D World Generation
709. **ui-2d** — 2D Room and Dungeon Generation
710. **ui-2d** — 2.5D Layer Composition
711. **ui-2d** — Perspective 3D with 2D Physics (Octopath-Style)
712. **ui-2d** — Dynamic 2D/3D Asset Layering
713. **ui-2d** — Screen Reader Support
714. **ui-2d** — Subtitle and Caption System
715. **ui-2d** — Colorblind Modes
716. **ui-2d** — High Contrast and Scalable UI
717. **ui-2d** — Input Remapping for Accessibility
718. **ui-2d** — Text-to-Speech for Chat
719. **ui-2d** — WCAG Compliance Targets
720. **vfx** — Compute Shader Particle Simulation
721. **vfx** — Particle Simulation Modules
722. **vfx** — Particle Rendering Modes
723. **vfx** — Particle LOD, Sorting, and Budget Culling
724. **vfx** — Sub-Emitters and Events
725. **vfx** — Particle Light Emission
726. **vfx** — GPU Fluid Simulation
727. **vfx** — Deferred and Projected Decals
728. **vfx** — Mesh Decals
729. **vfx** — Decal Atlasing and Batching
730. **vfx** — Decal Priority, Layering, and Lifecycle
731. **vfx** — Blood and Damage Decals
732. **vfx** — Footprints and Tire Tracks
733. **vfx** — Screen Shake
734. **vfx** — Motion Blur
735. **vfx** — Lens Flare
736. **vfx** — Chromatic Aberration and Film Grain
737. **vfx** — Heat Haze and Refraction
738. **vfx** — Damage Overlays and Screen Flash
739. **vfx** — Rain Particle System and Screen Droplets
740. **vfx** — Rain Puddles and Wet Surfaces
741. **vfx** — Snow Accumulation and Interaction
742. **vfx** — Fog Volumes
743. **vfx** — Lightning
744. **vfx** — Wind Visualization and Dust Storms
745. **vfx** — Underwater Caustics and Depth Fog
746. **vfx** — Debris Spawning
747. **vfx** — Dust Clouds and Smoke Plumes
748. **vfx** — Sparks and Embers
749. **vfx** — Structural Cracking Overlays
750. **vfx** — Persistent Scorch Marks
751. **vfx** — Explosion Shockwaves
752. **vfx** — Fire Propagation Visuals
753. **vfx** — Node-Based Effect Graph Editor
754. **vfx** — Custom Effect Graph Nodes
755. **vfx** — Effect Graph Parameter System
756. **vfx** — Event-Driven Effect Spawning
757. **vfx** — VFX LOD and Performance Budget
758. **content-pipeline** — Native Asset Ingestion
759. **content-pipeline** — Texture Source Import
760. **content-pipeline** — Audio Source Import
761. **content-pipeline** — Import Validation and Error Reporting
762. **content-pipeline** — Batch Import with Progress Tracking
763. **content-pipeline** — Texture Compression (BC, ASTC, ETC)
764. **content-pipeline** — LOD Generation
765. **content-pipeline** — Meshlet Building
766. **content-pipeline** — Vertex Cache and Overdraw Optimization
767. **content-pipeline** — Lightmap UV Generation
768. **content-pipeline** — Audio Encoding
769. **content-pipeline** — Shader Graph to HLSL Code Generation
770. **content-pipeline** — Asset Dependency Graphs
771. **content-pipeline** — DXC and Metal Shader Converter Pipeline
772. **content-pipeline** — Content-Addressable Storage
773. **content-pipeline** — Asset Metadata Store
774. **content-pipeline** — Hash-Based Import Caching
775. **content-pipeline** — Incremental Builds
776. **content-pipeline** — Asset Search and Tagging
777. **content-pipeline** — Asset Thumbnail Generation
778. **content-pipeline** — AI-Driven Auto-Categorization
779. **content-pipeline** — LLM-Based Semantic Asset Search
780. **content-pipeline** — Smart Collections and Recommendations
781. **content-pipeline** — Asset Versioning
782. **content-pipeline** — File Watcher
783. **content-pipeline** — Asset Hot Reload
784. **content-pipeline** — Shader Hot Reload
785. **content-pipeline** — Logic Graph Hot Reload
786. **content-pipeline** — UI Hot Reload
787. **content-pipeline** — Partial Re-Import
788. **content-pipeline** — Editor-Runtime Synchronization
789. **content-pipeline** — Virtual File System
790. **content-pipeline** — Platform-Native Async I/O
791. **content-pipeline** — GPU Direct Storage
792. **content-pipeline** — Texture Streaming
793. **content-pipeline** — Mesh Streaming
794. **content-pipeline** — Streaming Priority Queues
795. **content-pipeline** — Memory Pressure Response
796. **content-pipeline** — Pak / Archive Files
797. **content-pipeline** — Compression (LZ4, Zstd)
798. **content-pipeline** — Download-on-Demand Patching
799. **content-pipeline** — Asset Pipeline Plugin SDK
800. **content-pipeline** — Live Link Connection
801. **content-pipeline** — Houdini Engine Integration
802. **content-pipeline** — Houdini Mesh and Scene Export
803. **content-pipeline** — Houdini Procedural Placement Pipeline
804. **content-pipeline** — Maya Direct Export Plugin
805. **content-pipeline** — Maya Animation and Rigging Export
806. **content-pipeline** — Blender Direct Export Addon
807. **content-pipeline** — Blender Material to Engine Material Conversion
808. **content-pipeline** — Marvelous Designer Clothing Export
809. **content-pipeline** — Garment-to-Character Fitting
810. **content-pipeline** — Substance Material Import
811. **content-pipeline** — Substance Painter Project Import
812. **content-pipeline** — Runtime Substance Material Evaluation
813. **content-pipeline** — Photoshop Direct Export Plugin
814. **content-pipeline** — Photoshop Layer-to-UI Asset Pipeline
815. **content-pipeline** — Illustrator Vector Asset Export
816. **content-pipeline** — ZBrush High-Poly to Engine Pipeline
817. **content-pipeline** — ZBrush Decimation and LOD Generation
818. **content-pipeline** — MotionBuilder Animation Export
819. **content-pipeline** — MotionBuilder Live Mocap Streaming
820. **content-pipeline** — DCC Plugin Git LFS Lock Workflow
821. **content-pipeline** — DCC Plugin Review Comment Viewer
822. **content-pipeline** — DCC Plugin Asset Status Dashboard
823. **content-pipeline** — DCC-Agnostic Material Mapping
824. **content-pipeline** — Batch Export and CI Integration
825. **content-pipeline** — Universal Binary Asset Format
826. **content-pipeline** — Compressed Asset Bundles
827. **content-pipeline** — Structural Asset Diffing
828. **content-pipeline** — Three-Way Asset Merge
829. **content-pipeline** — Automatic Merge Conflict Resolution
830. **content-pipeline** — Spreadsheet-Style Data Table Editor
831. **content-pipeline** — Universal Asset Inspector
832. **content-pipeline** — Git LFS and Custom Merge Driver Integration
833. **game-framework** — Game Mode State Machine
834. **game-framework** — Game State Manager
835. **game-framework** — Player Controller
836. **game-framework** — Pawn and Character System
837. **game-framework** — Gameplay Ability System
838. **game-framework** — Gameplay Effect System
839. **game-framework** — Damage Model
840. **game-framework** — Death, Respawn, and Encounter Reset
841. **game-framework** — Modular System Registration
842. **game-framework** — Rust Plugin API for Developers
843. **game-framework** — Level Streaming
844. **game-framework** — Grid-Based World Partitioning
845. **game-framework** — Hierarchical Spatial Partitioning
846. **game-framework** — Sub-Level Composition
847. **game-framework** — Persistent and Transient Actors
848. **game-framework** — Day/Night Cycle
849. **game-framework** — Weather System Integration
850. **game-framework** — Save Game Serialization
851. **game-framework** — Save Data Migration and Versioning
852. **game-framework** — Checkpoint and Autosave
853. **game-framework** — Save Slots and Management
854. **game-framework** — Cloud Save with Platform APIs
855. **game-framework** — Async Save I/O Pipeline
856. **game-framework** — Gameplay Logic Graph Integration
857. **game-framework** — Logic Graph Debugging for Gameplay
858. **game-framework** — Logic Graph Hot Reload
859. **game-framework** — Sequencer and Timeline
860. **game-framework** — Cutscene Camera System
861. **game-framework** — Camera Rails and Splines
862. **game-framework** — Actor Animation Blending in Cinematics
863. **game-framework** — Dialogue Triggers and Subtitles
864. **game-framework** — Cutscene Skip System
865. **game-framework** — Cutscene Playback Speed
866. **game-framework** — Cutscene Pause
867. **game-framework** — Letterboxing and Cinematic Framing
868. **game-framework** — Quest Graph System
869. **game-framework** — Quest Prerequisites and Gating
870. **game-framework** — Quest Tracking and Journal
871. **game-framework** — Dialogue Tree System
872. **game-framework** — Conversation Camera and Framing
873. **game-framework** — Conversation Gameplay State
874. **game-framework** — Conversation Interruption and Resumption
875. **game-framework** — Quest Rewards and Economy Hooks
876. **game-framework** — Server-Driven World Events
877. **game-framework** — Quest Phasing System
878. **game-framework** — Typed Table Schema Definition
879. **game-framework** — Row-Based Data Tables
880. **game-framework** — Curve and Formula Definitions
881. **game-framework** — Visual Formula Nodes
882. **game-framework** — Row Inheritance and Prototype Chains
883. **game-framework** — Currency and Resource Definitions
884. **game-framework** — Crafting Recipe Tables
885. **game-framework** — Loot Tables with Weighted Random
886. **game-framework** — Stat and Attribute Tables
887. **game-framework** — Asset List Tables
888. **game-framework** — Indexed Lookup and Filtering
889. **game-framework** — ECS Component Binding
890. **game-framework** — Hot Reload and Versioned Patching
891. **game-framework** — Data Validation and Constraint Checking
892. **game-framework** — Parametric Facial Feature System
893. **game-framework** — Preset Blending and Templates
894. **game-framework** — Parametric Body Shape System
895. **game-framework** — Body Morph Propagation to Equipment
896. **game-framework** — Skin Material System
897. **game-framework** — Makeup and Face Paint Layer System
898. **game-framework** — Eye Customization
899. **game-framework** — Hair Customization System
900. **game-framework** — Modular Mesh Part System
901. **game-framework** — Equipment Attachment and Socket System
902. **game-framework** — Transmog and Appearance Override
903. **game-framework** — Multi-Race Base Mesh Support
904. **game-framework** — Character LOD and Crowd Optimization
905. **game-framework** — Mesh Merging and Draw Call Reduction
906. **game-framework** — Character Appearance Serialization
907. **game-framework** — ECS-Based Inventory Containers
908. **game-framework** — Grid-Based Inventory Layout
909. **game-framework** — Item Stacking and Splitting
910. **game-framework** — Per-Instance Item Properties
911. **game-framework** — Item Socket and Augmentation System
912. **game-framework** — Inventory Transfer and Drag-Drop
913. **game-framework** — Loot Distribution
914. **game-framework** — Merchant and Trading
915. **game-framework** — Equipment Slot Binding
916. **game-framework** — Inventory Serialization and Persistence
917. **game-framework** — Ability Definition and Composition
918. **game-framework** — Ability Activation and Input Integration
919. **game-framework** — Gameplay Effect System
920. **game-framework** — Melee Combat System
921. **game-framework** — Ranged Combat and Projectile System
922. **game-framework** — Hitbox and Hurtbox System
923. **game-framework** — 3D World Picking
924. **game-framework** — 2D Screen-Space Picking
925. **game-framework** — Selection State Management
926. **game-framework** — RTS Selection Preset
927. **game-framework** — RPG Selection Preset
928. **game-framework** — Action Selection Preset
929. **game-framework** — Builder/Sandbox Selection Preset
930. **game-framework** — Runtime Selection Groups
931. **game-framework** — Basic Command Dispatch
932. **game-framework** — Formation Movement
933. **game-framework** — Split and Subgroup Commands
934. **game-framework** — Command History and Undo
935. **game-framework** — 2D Selection Box (Marquee Select)
936. **game-framework** — Selection Indicators and Visual Feedback
937. **game-framework** — Race Definition
938. **game-framework** — Class Definition
939. **game-framework** — Multi-Class and Job Change
940. **game-framework** — Prestige and Rebirth System
941. **game-framework** — Talent Tree Data Model
942. **game-framework** — Talent Allocation and Respec
943. **game-framework** — Talent Tree Visual Editor
944. **game-framework** — Profession Data Model
945. **game-framework** — Gathering Profession Integration
946. **game-framework** — Crafting Profession Integration
947. **game-framework** — Crafting Station Interaction
948. **game-framework** — Reputation and Faction Standing
949. **game-framework** — Achievement Definition and Tracking
950. **game-framework** — Achievement Rewards and Display
951. **game-framework** — Platform Achievement Sync
952. **game-framework** — Item Enhancement and Enchanting
953. **game-framework** — Item Rarity Tier System
954. **game-framework** — Affix System
955. **game-framework** — Stat Re-Rolling Mechanics
956. **game-framework** — Item Set Bonuses
957. **game-framework** — Item Durability and Repair
958. **game-framework** — Guild CRUD and Membership
959. **game-framework** — Guild Rank and Permission System
960. **game-framework** — Guild Leveling and Perks
961. **game-framework** — Guild Roster UI
962. **game-framework** — Guild Storage and Bank
963. **game-framework** — Territory Claim System
964. **game-framework** — Guild War Declaration and PvP Rules
965. **game-framework** — Siege Mechanics
966. **game-framework** — Guild Rankings and Leaderboards
967. **game-framework** — Friends List and Social Graph
968. **game-framework** — Core Mail Send/Receive
969. **game-framework** — Mail Attachments
970. **game-framework** — System Mail
971. **game-framework** — Core Chat Infrastructure
972. **game-framework** — Chat Content Features
973. **game-framework** — Custom Player-Created Channels
974. **game-framework** — Emote and Social Action System
975. **game-framework** — Player Inspection
976. **game-framework** — Dungeon and Group Finder
977. **game-framework** — Arena System
978. **game-framework** — Battleground System
979. **game-framework** — PvP Rating and Seasonal Rewards
980. **game-framework** — PvP Stat Normalization
981. **game-framework** — Modular Building Placement System
982. **game-framework** — Construction Phase and Progress
983. **game-framework** — Structural Integrity
984. **game-framework** — Building Upgrade and Repair
985. **game-framework** — Housing Plot and Instance System
986. **game-framework** — Furniture Placement
987. **game-framework** — Functional Furniture Effects
988. **game-framework** — Hunger and Thirst System
989. **game-framework** — Temperature and Warmth System
990. **game-framework** — Stamina and Fatigue System
991. **game-framework** — Vital Debuff System
992. **game-framework** — Resource Node Definition
993. **game-framework** — Gathering Interaction Loop
994. **game-framework** — Resource Node Procedural Distribution
995. **game-framework** — Farming and Crop System
996. **game-framework** — Animal Needs and Happiness
997. **game-framework** — Animal Housing
998. **game-framework** — Animal Breeding
999. **game-framework** — Companion AI Framework
1000. **game-framework** — Pet Needs and Mood
1001. **game-framework** — Mount Summoning and Dismissal
1002. **game-framework** — Mounted Locomotion
1003. **game-framework** — Mounted Combat
1004. **game-framework** — Mount Type Variants
1005. **game-framework** — Creature Taming
1006. **game-framework** — Pet Life Stages
1007. **game-framework** — Pet Evolution Branching
1008. **game-framework** — Pet Breeding System
1009. **game-framework** — Weapon Fire Mode System
1010. **game-framework** — Magazine and Ammo Management
1011. **game-framework** — Reload Mechanics
1012. **game-framework** — Ammo Type System
1013. **game-framework** — Recoil Pattern and Weapon Spread
1014. **game-framework** — Projectile Drop and Travel Time
1015. **game-framework** — Wind Deflection
1016. **game-framework** — Surface Penetration and Ricochet
1017. **game-framework** — Weapon Zeroing
1018. **game-framework** — Attachment Slot Model
1019. **game-framework** — Attachment Visual Integration
1020. **game-framework** — Attachment Customization UI
1021. **game-framework** — Surface Type Tag System
1022. **game-framework** — Impact VFX Response
1023. **game-framework** — Impact Audio Response
1024. **game-framework** — Impact Decal Response
1025. **game-framework** — World Object Interaction System
1026. **game-framework** — Door and Lock System
1027. **game-framework** — Physics Object Pickup and Throw
1028. **game-framework** — Traversal Detection System
1029. **game-framework** — Vault and Mantle Actions
1030. **game-framework** — Wall Run
1031. **game-framework** — Crouch Slide
1032. **game-framework** — Balance Beam
1033. **game-framework** — Free-Climb System
1034. **game-framework** — Ladder System
1035. **game-framework** — Ledge Grab and Shimmy
1036. **game-framework** — Swimming and Diving
1037. **game-framework** — Grappling Hook and Zipline
1038. **game-framework** — Player Visibility and Stealth System
1039. **game-framework** — AI Alert State Machine
1040. **game-framework** — Noise Generation and Distraction
1041. **game-framework** — Stealth Takedown System
1042. **game-framework** — Cover Point Detection and Usage
1043. **game-framework** — NPC Relationship and Affinity System
1044. **game-framework** — NPC Personality and Emotion Model
1045. **game-framework** — NPC Deed Memory
1046. **game-framework** — Gossip Propagation Network
1047. **game-framework** — Emergent Reputation Aggregation
1048. **game-framework** — Schedule Data Model
1049. **game-framework** — Schedule Execution
1050. **game-framework** — Schedule-Gated Interactions
1051. **game-framework** — Ambient Bark System
1052. **game-framework** — Threat and Aggro Table System
1053. **game-framework** — NPC-to-NPC Conversation System
1054. **game-framework** — NPC Independent Memory System
1055. **game-framework** — NPC Environmental Interaction
1056. **game-framework** — Social-Cue Player Search
1057. **game-framework** — Quest and Story State NPC Awareness
1058. **game-framework** — Player-Witnessed NPC Social Behaviors
1059. **game-framework** — Fog of War Grid System
1060. **game-framework** — Vision Source and Sight Radius
1061. **game-framework** — Vision Modifier Volumes
1062. **game-framework** — Fog of War Memory
1063. **game-framework** — Tactical Grid System
1064. **game-framework** — Turn Manager and Initiative
1065. **game-framework** — Action Point Movement and Abilities
1066. **game-framework** — Grid Cover and Overwatch
1067. **game-framework** — Hit Probability and Combat Resolution
1068. **game-framework** — Track and Checkpoint System
1069. **game-framework** — Race Mode Framework
1070. **game-framework** — Racing AI Navigation
1071. **game-framework** — Rubber-Banding Difficulty
1072. **game-framework** — AI Racing Behavior
1073. **game-framework** — Drift Scoring and Boost System
1074. **game-framework** — Ghost Replay and Leaderboards
1075. **game-framework** — Battle Pass and Season System
1076. **game-framework** — Daily and Weekly Challenge System
1077. **game-framework** — Platform Purchase Abstraction
1078. **game-framework** — Server-Side Receipt Validation
1079. **game-framework** — Premium Currency System
1080. **game-framework** — Purchase History and Refund Tracking
1081. **game-framework** — Daily Login Reward Calendar
1082. **game-framework** — Subscription State Verification
1083. **game-framework** — Subscription Benefit Application
1084. **game-framework** — Subscription Management UI
1085. **game-framework** — Subscription Gifting
1086. **game-framework** — Timed Game Trial
1087. **game-framework** — Free Weekend Events
1088. **game-framework** — Content Trial
1089. **game-framework** — DLC and Expansion Purchasing
1090. **game-framework** — Cosmetic Store and Virtual Currency
1091. **game-framework** — Deceptive UI Prevention
1092. **game-framework** — Minor-Targeted Ad Blocking
1093. **game-framework** — Dark Pattern Prevention
1094. **game-framework** — Frequency Cap Enforcement
1095. **game-framework** — Wave Spawning System
1096. **game-framework** — Tower Targeting and Upgrade System
1097. **game-framework** — Score and Combo System
1098. **game-framework** — Feedback Stack Asset and Triggering
1099. **game-framework** — Hit-Stop and Time Scale Effects
1100. **game-framework** — Feedback Entry Types Library
1101. **game-framework** — Fast Travel Network
1102. **game-framework** — Respawn and Graveyard System
1103. **game-framework** — Tutorial Step System
1104. **game-framework** — Tutorial Visual Overlays
1105. **game-framework** — Toast Notification System
1106. **game-framework** — Free Camera Controller
1107. **game-framework** — Photo Mode Visual Controls
1108. **game-framework** — Photo Capture and Gallery
1109. **game-framework** — Virtual Camera Entity and Priority System
1110. **game-framework** — Camera Brain and Output Controller
1111. **game-framework** — Follow (Fixed Offset)
1112. **game-framework** — Orbital Follow
1113. **game-framework** — Third-Person Follow with Shoulder Offset
1114. **game-framework** — Hard Lock to Target
1115. **game-framework** — Position Composer (Adaptive Framing)
1116. **game-framework** — Spline Dolly
1117. **game-framework** — Rotation Composer (Adaptive Aim)
1118. **game-framework** — Hard Look At
1119. **game-framework** — Pan and Tilt (Input-Driven Rotation)
1120. **game-framework** — Rotate with Follow Target
1121. **game-framework** — Spring Arm Component
1122. **game-framework** — Camera Deoccluder (Line-of-Sight Preservation)
1123. **game-framework** — Camera Decollider (Geometry Penetration Prevention)
1124. **game-framework** — Camera Blend System
1125. **game-framework** — Camera Mixing (Weighted Multi-Camera Blend)
1126. **game-framework** — Multi-Channel Perlin Noise Profiles
1127. **game-framework** — Camera Impulse System
1128. **game-framework** — Wave Oscillation Shake
1129. **game-framework** — Composite Shake Patterns
1130. **game-framework** — Sequencer-Driven Camera Shake
1131. **game-framework** — State-Driven Camera Switching
1132. **game-framework** — Clear Shot (Automatic Best-Camera Selection)
1133. **game-framework** — Shot Quality Evaluator
1134. **game-framework** — Camera Sequencer (Timed Camera Playlist)
1135. **game-framework** — Target Group (Multi-Target Aggregation)
1136. **game-framework** — Group Framing Extension
1137. **game-framework** — Camera Confiner 2D
1138. **game-framework** — Camera Confiner 3D
1139. **game-framework** — Follow Zoom (Constant Screen-Size Framing)
1140. **game-framework** — Auto Focus
1141. **game-framework** — Third-Person Aim Extension
1142. **game-framework** — FreeLook Modifier
1143. **game-framework** — Recomposer (Timeline Composition Override)
1144. **game-framework** — Camera Modifier Stack
1145. **game-framework** — Camera Input Axis Controller
1146. **game-framework** — Cine Camera Properties
1147. **game-framework** — Camera Rig Presets (Dolly, Crane, Jib)
1148. **game-framework** — Picture-in-Picture
1149. **game-framework** — Minigame Session and Sandbox Context
1150. **game-framework** — Minigame World Presentation
1151. **game-framework** — Minigame Lifecycle and Result Contract
1152. **game-framework** — Timing and Rhythm Minigames
1153. **game-framework** — Grid/Board Engine
1154. **game-framework** — Match Detection Algorithms
1155. **game-framework** — Board Minigame AI
1156. **game-framework** — Board Piece Animation and Cascading
1157. **game-framework** — Physics Toy Minigames
1158. **game-framework** — Multiplayer Minigame Sessions
1159. **game-framework** — Minigame Library and Discovery
1160. **game-framework** — Block Type Registry and Properties
1161. **game-framework** — Block Placement and Destruction
1162. **game-framework** — Chunk-Based Block Storage
1163. **game-framework** — Block Chunk Meshing
1164. **game-framework** — Block Light Propagation
1165. **game-framework** — Gravity Block Physics
1166. **game-framework** — Fluid Flow Simulation
1167. **game-framework** — Fluid-Block Interactions
1168. **game-framework** — Signal Source and Wire Blocks
1169. **game-framework** — Logic Gate Blocks
1170. **game-framework** — Mechanism Blocks
1171. **game-framework** — Circuit Evaluation and Budget
1172. **game-framework** — Block Terrain Generation
1173. **game-framework** — Block Biome System
1174. **game-framework** — Block Ore Placement
1175. **game-framework** — Block Structure Generation
1176. **game-framework** — Rewarded Video Ads
1177. **game-framework** — Interstitial Ads
1178. **game-framework** — Banner Ads
1179. **game-framework** — Ad Mediation and Network Abstraction
1180. **platform** — Window Creation and Lifecycle
1181. **platform** — Fullscreen, Borderless, and Windowed Modes
1182. **platform** — Display Enumeration and Multi-Monitor Support
1183. **platform** — DPI Awareness and Display Scaling
1184. **platform** — VSync and Refresh Rate Management
1185. **platform** — HDR Output and Tone Mapping Handoff
1186. **platform** — Clipboard Access
1187. **platform** — Native File Dialogs
1188. **platform** — System Notifications and Tray Icons
1189. **platform** — Drag and Drop
1190. **platform** — Platform Keyboard Layouts and Dead Keys
1191. **platform** — Input Method Editor (IME) for CJK
1192. **platform** — Work-Stealing Thread Pool
1193. **platform** — Thread Affinity and Priority
1194. **platform** — Task Graph Job System
1195. **platform** — Fiber and Stackful Coroutine Support
1196. **platform** — Platform Async I/O Integration
1197. **platform** — Crash Handler and Minidump Generation
1198. **platform** — Symbol Upload and Server-Side Symbolication
1199. **platform** — Crash Aggregation and Alerting
1200. **platform** — Structured Logging with Severity and Channels
1201. **platform** — Performance Counters and Telemetry Hooks
1202. **platform** — GPU Crash Breadcrumbs
1203. **platform** — Cross-Platform Achievement System
1204. **platform** — Leaderboards
1205. **platform** — Rich Presence
1206. **platform** — Platform Voice and Party Integration
1207. **platform** — Platform Cloud Storage
1208. **platform** — Entitlements, DLC, and Subscription Verification
1209. **platform** — Console Certification Compliance
1210. **platform** — Async File Open, Read, and Write
1211. **platform** — Async File Create and Delete
1212. **platform** — Async File Metadata and Stat
1213. **platform** — Async Directory Enumeration
1214. **platform** — Directory Change Notifications
1215. **platform** — File Content Change Detection
1216. **platform** — Canonical Path Resolution
1217. **tools-editor** — Dockable Panel Layout
1218. **tools-editor** — Multi-Viewport
1219. **tools-editor** — Undo/Redo System (Command Pattern)
1220. **tools-editor** — Selection System
1221. **tools-editor** — Transform Gizmos
1222. **tools-editor** — Bounds and Measurement Gizmos
1223. **tools-editor** — Editor Preferences and Configuration
1224. **tools-editor** — Editor Extensions and Plugin API
1225. **tools-editor** — VR Editor Mode
1226. **tools-editor** — Entity Placement and Snapping
1227. **tools-editor** — Prefab System with Nested Prefabs
1228. **tools-editor** — Prefab Instance Overrides
1229. **tools-editor** — Brush and CSG Tools
1230. **tools-editor** — Spline Editing
1231. **tools-editor** — Landscape Painting
1232. **tools-editor** — Foliage Painting
1233. **tools-editor** — Node-Based Material Graph
1234. **tools-editor** — Material Functions and Subgraphs
1235. **tools-editor** — Live Material Preview
1236. **tools-editor** — Shader Parameter Tweaking
1237. **tools-editor** — Material Instances
1238. **tools-editor** — Material Library and Browser
1239. **tools-editor** — Animation Timeline
1240. **tools-editor** — Curve Editor
1241. **tools-editor** — Skeleton Viewer
1242. **tools-editor** — Animation Preview
1243. **tools-editor** — Blend Space Editor
1244. **tools-editor** — State Machine Editor
1245. **tools-editor** — Retargeting Setup
1246. **tools-editor** — Frame Profiler (CPU Timeline)
1247. **tools-editor** — GPU Profiler (Pass Timing and Occupancy)
1248. **tools-editor** — Memory Profiler (Allocation Tracking)
1249. **tools-editor** — Leak Detection
1250. **tools-editor** — Network Profiler (Bandwidth and Packet Inspector)
1251. **tools-editor** — Stat Overlays
1252. **tools-editor** — Remote Profiling
1253. **tools-editor** — Terrain Sculpting Brushes
1254. **tools-editor** — Terrain Erosion
1255. **tools-editor** — Terrain Material Painting
1256. **tools-editor** — Water Body Placement
1257. **tools-editor** — Vegetation Painting with Density Rules
1258. **tools-editor** — Lighting Setup (Light Probes and Reflection Probes)
1259. **tools-editor** — Navmesh Preview
1260. **tools-editor** — World Partition Visualization
1261. **tools-editor** — AI Content Provenance Tagging
1262. **tools-editor** — Human Modification Tracking
1263. **tools-editor** — Generative AI Feature Toggle
1264. **tools-editor** — AI Assistance Toggle
1265. **tools-editor** — Enterprise Remote Administration
1266. **tools-editor** — AI Content Audit Trail
1267. **tools-editor** — AI Content Review Workflow
1268. **tools-editor** — Provenance Metadata in Packaged Builds
1269. **tools-editor** — Universal Logic Graph Runtime
1270. **tools-editor** — Static Type System
1271. **tools-editor** — Strict Validation Before Use
1272. **tools-editor** — Gameplay Logic Graphs
1273. **tools-editor** — Shader Graph Core
1274. **tools-editor** — Shader Graph to HLSL Compilation
1275. **tools-editor** — Material Graph Variant
1276. **tools-editor** — Render Graph Configuration
1277. **tools-editor** — Animation Logic Graphs
1278. **tools-editor** — Audio Logic Graphs
1279. **tools-editor** — Custom Tool Graphs
1280. **tools-editor** — Graph Node Library
1281. **tools-editor** — Graph Debugging
1282. **tools-editor** — Graph Compilation and Optimization
1283. **tools-editor** — Graph Diffing and Merge
1284. **tools-editor** — Graph Search and Refactoring
1285. **tools-editor** — Speech-to-Text Pipeline
1286. **tools-editor** — Voice Command Interpretation
1287. **tools-editor** — Voice Activation Modes
1288. **tools-editor** — AI Assistant Tool Interface
1289. **tools-editor** — Visual and Graphical Tool Access
1290. **tools-editor** — Keyboard Shortcut Recommendations
1291. **tools-editor** — Contextual Action Reminders
1292. **tools-editor** — Headless Editor API Layer
1293. **tools-editor** — Agent Orchestration
1294. **tools-editor** — CI/CD Agent Integration
1295. **tools-editor** — Screenshot and Screen Recording Capture
1296. **tools-editor** — UI Accessibility Tree Analysis
1297. **tools-editor** — Multi-Modal Understanding
1298. **tools-editor** — AI Assistance Administration
1299. **tools-editor** — Native Git Integration
1300. **tools-editor** — Git LFS Management
1301. **tools-editor** — Asset-Aware Merge Driver
1302. **tools-editor** — Branch-Per-Feature Workflow
1303. **tools-editor** — Collaborative Presence
1304. **tools-editor** — Partial Clone and Sparse Checkout
1305. **tools-editor** — Shelving and Local Stash
1306. **tools-editor** — Multi-Provider Git Hosting Support
1307. **tools-editor** — Centralized Compiled Asset Cache
1308. **tools-editor** — Shader Compilation Cache
1309. **tools-editor** — Logic Graph Compilation Cache
1310. **tools-editor** — New Developer Onboarding Acceleration
1311. **tools-editor** — Cache Invalidation and Garbage Collection
1312. **tools-editor** — Cache Transport and Storage
1313. **tools-editor** — CI/CD Cache Population
1314. **tools-editor** — Cache Hit Metrics and Monitoring
1315. **tools-editor** — Remote Desktop Optimized Rendering
1316. **tools-editor** — Remote Editor Protocol
1317. **tools-editor** — CRDT-Based Real-Time Collaborative Editing
1318. **tools-editor** — Remote GPU Server Support
1319. **tools-editor** — Session Handoff and Persistence
1320. **tools-editor** — Bandwidth Adaptation and Quality Tiers
1321. **tools-editor** — Collaboration Cloud Service
1322. **tools-editor** — CRDT Document Model for Engine Assets
1323. **tools-editor** — Collaboration Access Control and Permissions
1324. **tools-editor** — Integrated Voice and Text Chat
1325. **tools-editor** — Work Groups and Isolated Workspaces
1326. **tools-editor** — AI Agent Collaboration
1327. **tools-editor** — Asset and Scene Comments
1328. **tools-editor** — Pull Request Review in Editor
1329. **tools-editor** — String Table Editor
1330. **tools-editor** — Localization Preview and Validation
1331. **tools-editor** — Translation Workflow Integration
1332. **tools-editor** — Platform Build Packaging
1333. **tools-editor** — Deploy-to-Device Workflow
1334. **tools-editor** — Certification Compliance Checker
1335. **tools-editor** — Code Signing Pipeline
1336. **tools-editor** — Platform-Specific Installers and Distributions
1337. **tools-editor** — Asset Bundle and DLC Packaging
1338. **tools-editor** — Delta Patching System
1339. **tools-editor** — Store Distribution Pipeline
1340. **tools-editor** — Engine Version Management
1341. **tools-editor** — Automatic Project Upgrades
1342. **tools-editor** — Project Browser and Creation Wizard
1343. **tools-editor** — Project File Format and Association
1344. **tools-editor** — Cross-Game Preferences and Account Management
1345. **tools-editor** — Collaboration Setup Wizard
1346. **tools-editor** — Mod SDK and Authoring Tools
1347. **tools-editor** — Developer-Defined Mod Constraints
1348. **tools-editor** — Mod Packaging and Distribution Format
1349. **tools-editor** — Mod Loading and Sandboxing
1350. **tools-editor** — Mod Workshop Integration
1351. **tools-editor** — Mod Moderation and Review
1352. **tools-editor** — Integrated Asset Store Browser
1353. **tools-editor** — One-Click Asset Import and Project Integration
1354. **tools-editor** — Asset Ratings, Reviews, and Curation
1355. **tools-editor** — Publisher Account and Dashboard
1356. **tools-editor** — Automated Compatibility Testing
1357. **tools-editor** — Revenue Sharing and Payout
1358. **tools-editor** — Asset Type Support
1359. **tools-editor** — License Management and DRM
1360. **tools-editor** — AWS CDK Deployment Stacks
1361. **tools-editor** — Live Collaboration Server
1362. **tools-editor** — Git and Git LFS Hosting with Locking
1363. **tools-editor** — Asset and Shader Compilation Server
1364. **tools-editor** — Signing and Distribution Server
1365. **tools-editor** — Continuous Deployment Pipeline
1366. **tools-editor** — Test Runner Infrastructure
1367. **tools-editor** — Shared Cache and Database Services
1368. **tools-editor** — Backup and Disaster Recovery
1369. **tools-editor** — Enterprise Security Configuration
1370. **tools-editor** — Auto-Generated API Reference
1371. **tools-editor** — Logic Graph Node Documentation
1372. **tools-editor** — Interactive In-Editor Tutorials
1373. **tools-editor** — Video Tutorial Integration
1374. **tools-editor** — Contextual Help and Tooltip System
1375. **tools-editor** — Sample Projects and Template Library
1376. **tools-editor** — Inline Code Examples in Documentation
1377. **tools-editor** — Cloud Build Service
1378. **tools-editor** — Platform Toolchain Containers
1379. **tools-editor** — Cross-Platform Shader Compilation
1380. **tools-editor** — Remote Code Signing
1381. **tools-editor** — Build Artifact Distribution
1382. **tools-editor** — Local Development Mode
1383. **tools-editor** — Build Pipeline Visualization
