# User Stories -- 15.3 Material Editor

## Stories

| ID          | Persona                    |
|-------------|----------------------------|
| US-15.3.1.1 | technical artist (P-13)    |
| US-15.3.1.2 | game designer (P-5)        |
| US-15.3.2.1 | technical artist (P-13)    |
| US-15.3.2.2 | extension developer (P-25) |
| US-15.3.3.1 | technical artist (P-13)    |
| US-15.3.3.2 | game designer (P-5)        |
| US-15.3.4.1 | game designer (P-5)        |
| US-15.3.4.2 | technical artist (P-13)    |
| US-15.3.5.1 | technical artist (P-13)    |
| US-15.3.5.2 | game designer (P-5)        |
| US-15.3.6.1 | game designer (P-5)        |
| US-15.3.6.2 | level designer (P-6)       |

1. **US-15.3.1.1** — **As a** technical artist (P-13), **I want** a node graph editor with type-safe
   pins for authoring materials, **so that** I can build complex shaders visually.

2. **US-15.3.1.2** — **As a** game designer (P-5), **I want** copy/paste, grouping, and minimap
   navigation in the material graph, **so that** I can manage large graphs efficiently.

3. **US-15.3.2.1** — **As a** technical artist (P-13), **I want** reusable material function
   subgraphs saved as assets, **so that** common patterns like triplanar mapping are authored once
   and shared.

4. **US-15.3.2.2** — **As a** extension developer (P-25), **I want** to publish material functions
   to the asset marketplace, **so that** other teams can reuse my work.

5. **US-15.3.3.1** — **As a** technical artist (P-13), **I want** a live 3D material preview on
   configurable meshes with adjustable lighting, **so that** I see changes instantly.

6. **US-15.3.3.2** — **As a** game designer (P-5), **I want** split-view comparison of two material
   variants side by side, **so that** I can evaluate options quickly.

7. **US-15.3.4.1** — **As a** game designer (P-5), **I want** material parameter sliders, color
   pickers, and curve editors in the inspector, **so that** I can tweak look without editing the
   node graph.

8. **US-15.3.4.2** — **As a** technical artist (P-13), **I want** parameter changes applied
   instantly without recompilation, **so that** iteration speed is not limited by shader builds.

9. **US-15.3.5.1** — **As a** technical artist (P-13), **I want** lightweight material instances
   that override parameters of a parent without duplicating the shader, **so that** thousands of
   visual variations share one compiled program.

10. **US-15.3.5.2** — **As a** game designer (P-5), **I want** to create material instances for
    color tints and weathering, **so that** I can diversify assets without artist involvement.

11. **US-15.3.6.1** — **As a** game designer (P-5), **I want** a searchable material library with
    thumbnails, tags, and usage tracking, **so that** I can find and reuse existing materials
    quickly.

12. **US-15.3.6.2** — **As a** level designer (P-6), **I want** drag-and-drop from the material
    library onto meshes in the viewport, **so that** I can assign materials without opening the
    inspector.
