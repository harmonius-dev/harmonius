# User Stories — 12.6 DCC Tool Plugins

## US-12.6.1 Export From Any DCC Tool Directly to the Engine
**As an** artist, **I want** native plugins for Houdini, Maya, Blender, Marvelous Designer,
Substance, Photoshop, Illustrator, ZBrush, and MotionBuilder with live link, Git LFS
locking, review comments, and batch CI export, **so that** I can push assets from my
preferred tool to the engine without intermediate formats or manual import steps.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| C API plugin SDK with Python and C++ bindings | F-12.6.1 | R-12.6.1 |
| Live link with <2s viewport update latency | F-12.6.2 | R-12.6.2 |
| Houdini Engine HDA hosting and procedural placement | F-12.6.3, F-12.6.4, F-12.6.5 | R-12.6.3, R-12.6.4, R-12.6.5 |
| Maya mesh, animation, and rigging export | F-12.6.6, F-12.6.7 | R-12.6.6, R-12.6.7 |
| Blender export with material conversion | F-12.6.8, F-12.6.9 | R-12.6.8, R-12.6.9 |
| Marvelous Designer garment export and auto-fitting | F-12.6.10, F-12.6.11 | R-12.6.10, R-12.6.11 |
| Substance material import and runtime evaluation | F-12.6.12, F-12.6.13, F-12.6.14 | R-12.6.12, R-12.6.13, R-12.6.14 |
| Photoshop export with layer-to-UI conversion | F-12.6.15, F-12.6.16 | R-12.6.15, R-12.6.16 |
| Illustrator vector asset export | F-12.6.17 | R-12.6.17 |
| ZBrush high-poly pipeline with LOD generation | F-12.6.18, F-12.6.19 | R-12.6.18, R-12.6.19 |
| MotionBuilder animation export and live mocap | F-12.6.20, F-12.6.21 | R-12.6.20, R-12.6.21 |
| Git LFS lock-on-open, review comments, status dashboard | F-12.6.22, F-12.6.23, F-12.6.24 | R-12.6.22, R-12.6.23, R-12.6.24 |
| DCC-agnostic material mapping table | F-12.6.25 | R-12.6.25 |
| Headless batch export for CI pipelines | F-12.6.26 | R-12.6.26 |
