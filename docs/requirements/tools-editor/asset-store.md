# R-15.17 -- Asset Marketplace Requirements

## Browsing and Discovery

| ID        | Derived From                                            |
|-----------|---------------------------------------------------------|
| R-15.17.1 | [F-15.17.1](../../features/tools-editor/asset-store.md) |
| R-15.17.2 | [F-15.17.2](../../features/tools-editor/asset-store.md) |

1. **R-15.17.1** — The editor **SHALL** provide an in-editor marketplace browser with searchable,
   filterable catalog, interactive 3D model previews, material previews, audio playback,
   engine-version compatibility badges, ratings, reviews, and download counts.
   - **Rationale:** In-editor asset discovery eliminates context switching and accelerates content
     acquisition.
   - **Verification:** Unit test: search by keyword and verify results match query terms.
2. **R-15.17.2** — The editor **SHALL** support one-click import of marketplace assets with
   automatic dependency resolution, engine-version compatibility validation at import, and
   per-project license tracking.
   - **Rationale:** Frictionless import with dependency resolution ensures purchased assets work
     immediately.
   - **Verification:** Unit test: import an asset with transitive dependencies and verify all are
     downloaded automatically.

## Ratings and Curation

| ID        | Derived From                                            |
|-----------|---------------------------------------------------------|
| R-15.17.3 | [F-15.17.3](../../features/tools-editor/asset-store.md) |

1. **R-15.17.3** — The marketplace **SHALL** support 1-5 star ratings with text reviews, editorial
   curation with featured assets and staff picks, community-curated collections, and review
   moderation for spam and abuse.
   - **Rationale:** Quality signals help buyers evaluate assets before purchase.
   - **Verification:** Unit test: flag content for moderation and verify it is intercepted.

## Publisher Tools

| ID        | Derived From                                            |
|-----------|---------------------------------------------------------|
| R-15.17.4 | [F-15.17.4](../../features/tools-editor/asset-store.md) |
| R-15.17.5 | [F-15.17.5](../../features/tools-editor/asset-store.md) |
| R-15.17.6 | [F-15.17.6](../../features/tools-editor/asset-store.md) |

1. **R-15.17.4** — The marketplace **SHALL** provide publisher accounts with identity verification,
   revenue analytics, multi-currency pricing with regional pricing, review response capability, and
   time-limited sales and discount codes.
   - **Rationale:** Publishers need business tools to manage and promote their assets.
   - **Verification:** Unit test: apply a discount code at checkout and verify the discount is
     applied.
2. **R-15.17.5** — The marketplace **SHALL** automatically test all published assets against new
   engine versions, awarding compatibility badges to passing assets and notifying publishers of
   failures with specific error details.
   - **Rationale:** Automated testing keeps compatibility badges current without manual publisher
     effort.
   - **Verification:** Unit test: verify passing assets receive the new version badge.
3. **R-15.17.6** — The marketplace **SHALL** take a configurable commission (default 12%) with
   monthly payouts via bank transfer or PayPal, per-asset revenue reports with regional breakdown,
   and automatic refund revenue reversal within 14 days.
   - **Rationale:** Fair, transparent revenue sharing incentivizes publisher participation.
   - **Verification:** Unit test: process a refund within 14 days and verify publisher revenue is
     reversed.

## Asset Types and Licensing

| ID        | Derived From                                            |
|-----------|---------------------------------------------------------|
| R-15.17.7 | [F-15.17.7](../../features/tools-editor/asset-store.md) |
| R-15.17.8 | [F-15.17.8](../../features/tools-editor/asset-store.md) |

1. **R-15.17.7** — The marketplace **SHALL** support all engine-producible asset types (3D meshes,
   materials, VFX, audio, logic graph templates, full project templates, Rust plugins) with
   per-platform compiled binaries for plugins.
   - **Rationale:** Complete asset type coverage maximizes marketplace utility.
   - **Verification:** Unit test: verify each supported asset type renders a correct preview in the
     store listing.
2. **R-15.17.8** — The marketplace **SHALL** support per-asset usage rights (personal, commercial),
   license terms displayed at import time, and no runtime DRM on imported assets.
   - **Rationale:** Clear licensing and DRM-free imports ensure assets work offline without
     restrictions.
   - **Verification:** Unit test: import an asset and verify it functions offline without network
     connectivity.

## Open Source Asset Store

| ID         | Derived From                                             |
|------------|----------------------------------------------------------|
| R-15.17.9  | [F-15.17.9](../../features/tools-editor/asset-store.md)  |
| R-15.17.10 | [F-15.17.10](../../features/tools-editor/asset-store.md) |
| R-15.17.11 | [F-15.17.11](../../features/tools-editor/asset-store.md) |
| R-15.17.12 | [F-15.17.12](../../features/tools-editor/asset-store.md) |
| R-15.17.13 | [F-15.17.13](../../features/tools-editor/asset-store.md) |

1. **R-15.17.9** — The editor **SHALL** provide an open source asset browser integrated into the
   store UI with a dedicated tab, filtering by license type (CC0, CC-BY, Apache 2.0, MIT), category,
   tags, and engine version compatibility, backed by a community Git repository that is cloneable
   for offline access.
   - **Rationale:** A free, community-driven asset repository lowers the barrier to entry for all
     developers and supports the open source mission.
   - **Verification:** Unit test: filter by CC0 license and verify only CC0 assets are returned.
2. **R-15.17.10** — The editor **SHALL** support publishing assets to the open source store with
   mandatory license tagging, metadata, thumbnails, and preview media via pull requests, with
   automated CI validation (load test, lint, license file check) before merge.
   - **Rationale:** Automated validation ensures published open source assets meet quality and
     licensing standards.
   - **Verification:** Integration test: submit an asset via PR and verify CI validates license file
     presence.
3. **R-15.17.11** — The open source store **SHALL** support community ratings (1-5 stars), text
   reviews, download counts, and popularity sorting, with review moderation by community
   maintainers.
   - **Rationale:** Quality signals help users discover the best community-contributed assets.
   - **Verification:** Unit test: submit a rating and verify the average is recalculated.
4. **R-15.17.12** — The open source store **SHALL** use semantic versioning with tagged Git commits,
   engine version compatibility validation via semver ranges, and dependency resolution following
   the same rules as the paid marketplace (R-15.17.2).
   - **Rationale:** Versioning ensures reproducible builds and safe upgrades.
   - **Verification:** Unit test: resolve dependencies for an asset with a semver range and verify
     the correct version is selected.
5. **R-15.17.13** — The editor **SHALL** support one-click import of open source assets with
   auto-configuration of materials, prefabs, and animations, transitive dependency resolution from
   the community repository, and immediate visibility in the asset browser.
   - **Rationale:** Frictionless import removes barriers to adopting community assets.
   - **Verification:** Integration test: import an open source asset with dependencies and verify
     all appear in the asset browser.

## External Store Integration

| ID         | Derived From                                             |
|------------|----------------------------------------------------------|
| R-15.17.14 | [F-15.17.14](../../features/tools-editor/asset-store.md) |
| R-15.17.15 | [F-15.17.15](../../features/tools-editor/asset-store.md) |
| R-15.17.16 | [F-15.17.16](../../features/tools-editor/asset-store.md) |
| R-15.17.17 | [F-15.17.17](../../features/tools-editor/asset-store.md) |
| R-15.17.18 | [F-15.17.18](../../features/tools-editor/asset-store.md) |

1. **R-15.17.14** — The editor **SHALL** integrate with Epic's FAB marketplace via OAuth 2.0,
   displaying FAB listings with prices, ratings, and previews, with auto-import including format
   conversion (FBX, material remapping) and per-project license tracking, taking no commission on
   FAB purchases.
   - **Rationale:** FAB integration gives users access to the largest third-party asset catalog
     without leaving the editor.
   - **Verification:** Integration test: authenticate with FAB, browse listings, and import a free
     asset with format conversion.
2. **R-15.17.15** — The editor **SHALL** integrate with the Synty store, supporting browsing,
   purchasing via embedded browser or OAuth, and auto-import with PBR material channel remapping
   from Unity and Unreal formats, taking no commission.
   - **Rationale:** Synty is the most popular low-poly asset provider; direct integration
     accelerates prototyping.
   - **Verification:** Integration test: import a Synty pack and verify material channels are
     correctly remapped.
3. **R-15.17.16** — The editor **SHALL** integrate with TurboSquid for searching and importing 3D
   models with format conversion (FBX, OBJ, glTF), auto-LOD generation, and UV validation/repair,
   taking no commission.
   - **Rationale:** TurboSquid hosts high-quality 3D models; direct integration reduces manual
     conversion steps.
   - **Verification:** Integration test: import a TurboSquid model in FBX format and verify LODs are
     generated.
4. **R-15.17.17** — The editor **SHALL** provide a generic store plugin API with standardized
   interfaces for catalog browsing, user authentication, purchase processing, asset download, and
   format-converting import, allowing third-party stores to register as additional tabs in the asset
   store browser.
   - **Rationale:** An extensible API future-proofs the editor for new asset stores without engine
     changes.
   - **Verification:** Unit test: register a mock store plugin and verify it appears as a tab in the
     store browser.
5. **R-15.17.18** — The editor **SHALL** track licenses of all imported assets per-project across
   all sources, generate a compliance report (Markdown, JSON, PDF) listing each asset's license
   type, attribution requirements, and redistribution rights, and warn on incompatible license
   combinations.
   - **Rationale:** License compliance tracking prevents legal issues at distribution time.
   - **Verification:** Unit test: import assets with conflicting licenses and verify a warning is
     generated.

## AI Content Generation

| ID         | Derived From                                             |
|------------|----------------------------------------------------------|
| R-15.17.19 | [F-15.17.19](../../features/tools-editor/asset-store.md) |
| R-15.17.20 | [F-15.17.20](../../features/tools-editor/asset-store.md) |
| R-15.17.21 | [F-15.17.21](../../features/tools-editor/asset-store.md) |
| R-15.17.22 | [F-15.17.22](../../features/tools-editor/asset-store.md) |
| R-15.17.23 | [F-15.17.23](../../features/tools-editor/asset-store.md) |
| R-15.17.24 | [F-15.17.24](../../features/tools-editor/asset-store.md) |
| R-15.17.25 | [F-15.17.25](../../features/tools-editor/asset-store.md) |
| R-15.17.26 | [F-15.17.26](../../features/tools-editor/asset-store.md) |
| R-15.17.27 | [F-15.17.27](../../features/tools-editor/asset-store.md) |
| R-15.17.28 | [F-15.17.28](../../features/tools-editor/asset-store.md) |

1. **R-15.17.19** — The editor **SHALL** generate textures from text prompts via diffusion-based
   models with seamless tiling, PBR channel output (albedo, normal, roughness, metallic, AO),
   resolutions from 256x256 to 4096x4096, batch variation support, and AI provenance tagging
   (R-15.7.1).
   - **Rationale:** AI texture generation accelerates material creation and reduces dependency on
     external texture libraries.
   - **Verification:** Unit test: generate a texture from a prompt and verify all PBR channels are
     present and seamlessly tileable.
2. **R-15.17.20** — The editor **SHALL** generate 3D meshes from text prompts or reference images
   with clean quad-dominant topology, automatic UV unwrapping, auto-generated LOD chain, collision
   geometry, controllable poly budget, and AI provenance tagging.
   - **Rationale:** AI mesh generation enables rapid prototyping without waiting for artist-created
     assets.
   - **Verification:** Unit test: generate a mesh from a prompt and verify LOD chain and collision
     geometry are present.
3. **R-15.17.21** — The editor **SHALL** generate character animations from text descriptions with
   retargetable output via the retargeting system (F-9.1.8), motion matching integration, root
   motion data, and animation events.
   - **Rationale:** AI animation generation fills the gap for teams without dedicated animators.
   - **Verification:** Unit test: generate a walk cycle and verify it retargets to a test skeleton
     without foot sliding.
4. **R-15.17.22** — The editor **SHALL** generate sound effects and music variations from text
   prompts in the engine's native audio format, with spatial audio metadata, batch variation
   support, and AI provenance tagging.
   - **Rationale:** AI audio generation provides placeholder and production-quality audio without
     recording sessions.
   - **Verification:** Unit test: generate a sound effect from a prompt and verify it plays through
     the audio system.
5. **R-15.17.23** — The editor **SHALL** generate level layouts from visual constraint
   specifications (genre, size, difficulty, theme, flow) with prop placement, lighting, terrain
   sculpting, and material painting, outputting a fully editable level with support for iterative
   region regeneration.
   - **Rationale:** AI level layout accelerates level design iteration and provides starting points
     for hand-refinement.
   - **Verification:** Integration test: specify constraints and verify the generated level loads in
     the editor with all placed entities.
6. **R-15.17.24** — The editor **SHALL** generate PBR materials from text descriptions or photo
   references as editable material graphs with nodes for albedo, normal, roughness, metallic, and
   height channels, supporting variations and AI provenance tagging.
   - **Rationale:** AI material generation produces editable material graphs, not opaque textures.
   - **Verification:** Unit test: generate a material from a text prompt and verify the output is an
     editable material graph.
7. **R-15.17.25** — The editor **SHALL** generate VFX effect graphs from text descriptions with
   configured emitters, forces, and rendering parameters, outputting editable VFX graphs with style
   transfer support and AI provenance tagging.
   - **Rationale:** AI VFX generation enables rapid creation of particle effects without manual
     emitter configuration.
   - **Verification:** Unit test: generate a fire effect from a prompt and verify the output is an
     editable VFX graph.
8. **R-15.17.26** — The editor **SHALL** support iterative refinement of AI-generated content via
   follow-up prompts, undo/redo integration with the editor command history, blending of AI output
   with hand-authored content via masking and layering, and iteration history in the AI provenance
   trail.
   - **Rationale:** Iterative refinement turns AI output from rough drafts into production-quality
     assets.
   - **Verification:** Unit test: generate content, apply a follow-up prompt, undo, and verify the
     original state is restored.
9. **R-15.17.27** — The editor **SHALL** run AI models locally via GPU-accelerated inference (Metal,
   Vulkan, D3D12), with model download and caching via the shared cache, ONNX and safetensors format
   support, VRAM budget management with CPU fallback, and no telemetry or data collection.
   - **Rationale:** Local inference ensures AI features work offline, protects IP, and eliminates
     cloud costs.
   - **Verification:** Unit test: run inference on a test model and verify output matches reference
     within tolerance.
10. **R-15.17.28** — The editor **SHALL** track AI content provenance via the governance system
    (F-15.7.1), support artist opt-out lists for training data, verify license-safe training data
    sources against known databases, and integrate with the AI content review workflow (F-15.7.7)
    for mandatory review.
    - **Rationale:** Governance protects artists' rights and ensures compliance with evolving AI
      content regulations.
    - **Verification:** Unit test: generate an AI asset and verify provenance tag, opt-out list
      check, and review workflow trigger.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/asset-store.md](../../user-stories/tools-editor/asset-store.md).
Requirements in this document are derived from those user stories.
