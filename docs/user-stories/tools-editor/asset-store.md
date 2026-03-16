# User Stories: Asset Marketplace

## F-15.17.1 Integrated Asset Store Browser

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.1.1 | designer (P-5) | to browse and search the asset marketplace directly inside the editor with category filters, tags, ratings, and semantic search | I can find gameplay templates and tools without leaving my workspace |  |  |
| US-15.17.1.2 | artist (P-8) | 3D model viewers (rotate, zoom), material previews on standard meshes, audio playback, and screenshot galleries in asset listings | I can evaluate visual quality before purchasing |  |  |
| US-15.17.1.3 | developer (P-15) | semantic search beyond keyword matching | I can find assets by describing what I need rather than guessing exact tag names |  |  |
| US-15.17.1.4 | modder (P-24) | the store accessible from both the editor main menu and the launcher home screen | I can discover assets before opening a project |  |  |

## F-15.17.2 One-Click Asset Import and Project Integration

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.2.1 | developer (P-15) | purchased assets to import into my project with a single click, automatically resolving dependencies (required plugins, material libraries) | I do not manually track transitive dependencies |  |  |
| US-15.17.2.2 | designer (P-5) | version conflicts (asset requires plugin v2.0 but project has v1.8) flagged with specific upgrade steps | I can resolve incompatibilities before they cause runtime errors |  |  |
| US-15.17.2.3 | artist (P-8) | imported assets to appear in the asset browser immediately after download | I can start using new content without restarting the editor |  |  |
| US-15.17.2.4 | engine tester (P-27) | to verify that the import pipeline validates engine version compatibility, required plugins, and platform support before importing | incompatible assets are caught at import time |  |  |

## F-15.17.3 Asset Ratings, Reviews, and Curation

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.3.1 | artist (P-8) | to rate assets 1-5 stars with text reviews | other artists can make informed purchasing decisions based on community feedback |  |  |
| US-15.17.3.2 | designer (P-5) | staff-curated and community-curated collections organized by theme (e.g., "Medieval Bundle", "Sci-Fi Starter Kit") | I can bootstrap projects with visually consistent asset sets |  |  |
| US-15.17.3.3 | creative director (P-2) | featured assets, staff picks, and themed collections highlighted in the store | high-quality content is discoverable |  |  |
| US-15.17.3.4 | engine tester (P-27) | to verify that reviews are moderated for spam and abuse | the review system provides trustworthy feedback |  |  |

## F-15.17.4 Publisher Account and Dashboard

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.4.1 | developer (P-15) | to register a verified publisher account with identity verification and access a dashboard showing published assets, revenue analytics, and user reviews | I can manage my asset business |  |  |
| US-15.17.4.2 | tech artist (P-13) | to set prices in multiple currencies with regional pricing overrides | my assets are competitively priced in each market |  |  |
| US-15.17.4.3 | DevOps engineer (P-16) | compatibility test results per engine version displayed in the publisher dashboard | I can track which engine versions my assets support |  |  |
| US-15.17.4.4 | engine tester (P-27) | to verify that revenue analytics, download statistics, and review data are accurate and up-to-date | publishers can trust their dashboard metrics |  |  |

## F-15.17.5 Automated Compatibility Testing

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.5.1 | DevOps engineer (P-16) | the marketplace to automatically test all published assets against each new engine version and award compatibility badges | buyers have confidence assets work without publisher intervention |  |  |
| US-15.17.5.2 | developer (P-15) | to receive notifications with specific error details when my asset fails compatibility testing | I can remediate quickly and maintain my badge |  |  |
| US-15.17.5.3 | designer (P-5) | to filter store listings by compatibility badge for the current engine version | I only see assets confirmed to work with my project |  |  |
| US-15.17.5.4 | engine tester (P-27) | to verify that the compatibility testing pipeline imports each asset, verifies loading, and runs included tests correctly | badges are meaningful |  |  |

## F-15.17.6 Revenue Sharing and Payout

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.6.1 | developer (P-15) | monthly revenue reports showing per-asset sales, regional breakdown, commission deductions (12% default), and refund reversals | I can track income accurately |  |  |
| US-15.17.6.2 | tech artist (P-13) | free assets to cost nothing to publish | I can share community tools and resources without financial barriers |  |  |
| US-15.17.6.3 | server admin (P-22) | tax documentation (W-9, W-8BEN) collected from publishers for compliance | the marketplace meets tax reporting requirements |  |  |
| US-15.17.6.4 | engine tester (P-27) | to verify that refunds within the 14-day window automatically reverse publisher revenue | financial data stays accurate |  |  |

## F-15.17.7 Asset Type Support

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.7.1 | artist (P-8) | to publish all engine asset types (3D meshes, materials, VFX graphs, audio, terrain brushes, procedural generation graphs) | the marketplace covers my full content workflow |  |  |
| US-15.17.7.2 | developer (P-15) | to publish compiled Rust plugins with per-platform binaries | I can distribute engine extensions without requiring source compilation |  |  |
| US-15.17.7.3 | designer (P-5) | to download pre-built logic graph templates for common gameplay systems (inventory, dialogue, AI behavior) | I start with proven implementations |  |  |
| US-15.17.7.4 | engine tester (P-27) | to verify that each asset type has appropriate preview rendering and metadata in store listings | all types are presented meaningfully |  |  |

## F-15.17.8 License Management and DRM

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.8.1 | artist (P-8) | clear license metadata (personal, commercial, redistribution rights, attribution, seat count) on every asset listing and in the asset browser after import | I know exactly what I am allowed to do |  |  |
| US-15.17.8.2 | developer (P-15) | imported assets to work fully offline with no runtime license checks or DRM | shipped builds never depend on the marketplace being reachable |  |  |
| US-15.17.8.3 | modder (P-24) | to see whether an asset's license permits redistribution in mods | I know which marketplace assets I can include in my mod packages |  |  |
| US-15.17.8.4 | engine tester (P-27) | to verify that license terms are validated at import time and displayed in the asset browser | users are informed of restrictions before using purchased content |  |  |

---

## F-15.17.9 Open Source Asset Browser

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.9.1 | environment artist (P-8) | to browse and search the open source asset store filtered by license type (CC0, CC-BY), category, and engine version | I can find free, legally clear assets for my environment scenes without cost |  |  |
| US-15.17.9.2 | modder (P-24) | to browse the community asset repository with sorting by popularity, rating, and recency | I can discover the best community-contributed content for my mods |  |  |
| US-15.17.9.3 | modder (P-24) | to clone the entire open source asset repository for fully offline access | I can browse and use community assets without network connectivity |  |  |

## F-15.17.10 Asset Upload and Publishing

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.10.1 | technical artist (P-13) | to publish an asset to the open source store with license tagging (CC0, Apache 2.0), metadata, thumbnails, and preview media via a pull request workflow | I can contribute back to the community with proper license attribution |  |  |
| US-15.17.10.2 | technical artist (P-13) | automated CI to validate my submitted asset (load test, lint, license file check) before maintainers review it | I get fast feedback on any issues with my contribution |  |  |
| US-15.17.10.3 | environment artist (P-8) | to publish updated versions of my open source assets with changelogs | users of my assets can see what changed and upgrade safely |  |  |

## F-15.17.11 Asset Rating and Reviews

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.11.1 | environment artist (P-8) | to rate and review open source assets with 1-5 stars and text feedback | the community can identify the highest quality free assets |  |  |

## F-15.17.12 Asset Versioning

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.12.1 | technical artist (P-13) | semantic versioning for my published open source assets with engine version compatibility declarations | users know which engine versions my asset supports |  |  |

## F-15.17.13 One-Click Import

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.13.1 | environment artist (P-8) | to import open source assets into my project with one click, auto-configuring materials, prefabs, and animations | I can use community assets immediately without manual setup |  |  |

---

## F-15.17.14 FAB (Epic) Integration

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.14.1 | level designer (P-6) | to browse and purchase assets from Epic's FAB marketplace within the editor, with OAuth login and auto-import including format conversion | I can access FAB's large asset catalog without leaving my workspace |  |  |
| US-15.17.14.2 | level designer (P-6) | to preview FAB asset listings with prices, ratings, and 3D previews inside the editor | I can evaluate assets before purchasing them |  |  |
| US-15.17.14.3 | level designer (P-6) | FAB assets to be auto-imported with FBX-to-engine format conversion and material remapping | imported assets render correctly without manual material fixes |  |  |

## F-15.17.15 Synty Store Integration

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.15.1 | level designer (P-6) | to browse Synty asset packs within the editor and purchase them through Synty's storefront | I can quickly acquire low-poly art packs for prototyping |  |  |

## F-15.17.16 TurboSquid Integration

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.16.1 | environment artist (P-8) | to search and import 3D models from TurboSquid with automatic format conversion (FBX, OBJ, glTF), LOD generation, and UV validation | I can use high-quality third-party models without manual pipeline steps |  |  |

## F-15.17.17 Generic Store API

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.17.1 | technical artist (P-13) | a generic store plugin API that allows integrating additional third-party asset stores as new tabs in the store browser | our team can access assets from niche stores specific to our art style |  |  |

## F-15.17.18 License Compliance Tracking

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.18.1 | DevOps engineer (P-16) | to generate a license compliance report for all imported assets in my project listing each asset's license type, attribution requirements, and redistribution rights | I can verify legal compliance before distributing the game |  |  |
| US-15.17.18.2 | DevOps engineer (P-16) | warnings when imported assets have incompatible license combinations (e.g., GPL in a proprietary project) | license conflicts are caught before distribution |  |  |
| US-15.17.18.3 | DevOps engineer (P-16) | to export the license compliance report as Markdown, JSON, or PDF | I can share it with legal review and include it in release documentation |  |  |

---

## F-15.17.19 AI Texture Generation

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.19.1 | character artist (P-9) | to generate PBR textures from text prompts with seamless tiling and automatic albedo, normal, roughness, and metallic channel output | I can rapidly create material textures for character models without painting from scratch |  |  |
| US-15.17.19.2 | character artist (P-9) | to batch-generate texture variations (weathered, clean, damaged) from a single prompt | I have multiple options to choose from without running each variation manually |  |  |
| US-15.17.19.3 | character artist (P-9) | to choose texture resolution from 256x256 to 4096x4096 | I can balance quality and memory budget for different use cases |  |  |

## F-15.17.20 AI Mesh Generation

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.20.1 | environment artist (P-8) | to generate 3D meshes from text prompts with clean topology, auto UV unwrapping, LOD chain, and collision geometry | I can quickly populate scenes with varied props during level design |  |  |
| US-15.17.20.2 | environment artist (P-8) | to set a target poly count for AI-generated meshes | the output fits my project's performance budget |  |  |

## F-15.17.21 AI Animation Generation

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.21.1 | character animator (P-11) | to generate character animations from text descriptions (e.g., "tired walk cycle") that are retargetable to any skeleton | I can quickly create animation drafts for iteration |  |  |
| US-15.17.21.2 | character animator (P-11) | to blend AI-generated animation clips with hand-authored animations via motion matching | AI output can supplement my library without replacing hand-crafted quality |  |  |
| US-15.17.21.3 | character animator (P-11) | AI-generated animations to include root motion data and animation events | they integrate with gameplay systems without manual event tagging |  |  |

## F-15.17.22 AI Audio Generation

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.22.1 | audio designer (P-14) | to generate sound effects from text prompts (e.g., "metallic clang", "forest ambience") in the engine's native audio format | I can rapidly fill out the audio palette during prototyping |  |  |
| US-15.17.22.2 | audio designer (P-14) | to batch-generate sound effect variations from a single prompt | I have auditory variety for repeated gameplay events (footsteps, impacts) |  |  |
| US-15.17.22.3 | audio designer (P-14) | to generate music variations from a seed track or style description | I can create adaptive music layers without composing each variation |  |  |

## F-15.17.23 AI Level Layout

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.23.1 | game designer (P-5) | to generate level layouts by specifying constraints (genre, size, difficulty, theme) via a visual interface | I get a playable starting point for level design without manually placing every prop |  |  |
| US-15.17.23.2 | game designer (P-5) | to regenerate specific regions of an AI-generated level while keeping the rest intact | I can iteratively refine layouts without starting over |  |  |
| US-15.17.23.3 | game designer (P-5) | AI-generated levels to be fully editable in the level editor with all entities selectable and modifiable | AI output is a starting point, not a locked result |  |  |

## F-15.17.24 AI Material Generation

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.24.1 | environment artist (P-8) | to generate PBR materials from text descriptions or photo references as editable material graphs | I can create and tweak materials faster than building graphs from scratch |  |  |

## F-15.17.25 AI VFX Generation

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.25.1 | VFX artist (P-12) | to generate particle effects from text descriptions (e.g., "campfire with sparks", "magical portal swirl") as editable VFX effect graphs | I can rapidly prototype visual effects and refine them |  |  |
| US-15.17.25.2 | VFX artist (P-12) | to apply style transfer from a reference VFX to a new text description | I can maintain visual consistency across generated effects |  |  |
| US-15.17.25.3 | VFX artist (P-12) | AI-generated VFX graphs to be fully editable in the VFX editor with all emitters, forces, and parameters exposed | I can hand-tune the output |  |  |

## F-15.17.26 AI Content Iteration

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.26.1 | character artist (P-9) | to refine AI-generated content with follow-up prompts (e.g., "make the texture more weathered") and undo/redo each AI operation | I can iteratively improve output without losing previous states |  |  |
| US-15.17.26.2 | character animator (P-11) | to blend AI-generated content with hand-authored content using masking and layering | I can combine AI efficiency with manual precision |  |  |

## F-15.17.27 Local AI Inference

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.27.1 | environment artist (P-8) | all AI generation to run locally on my GPU without cloud connectivity | I can use AI features on air-gapped workstations and protect project IP |  |  |
| US-15.17.27.2 | technical artist (P-13) | AI models to be downloaded and cached via the shared cache system | the team shares one download and I do not re-download models on every workstation |  |  |

## F-15.17.28 AI Content Governance

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.17.28.1 | DevOps engineer (P-16) | every AI-generated asset to carry provenance metadata (model, timestamp, prompt hash) tracked by the governance system | I can audit which assets in a build are AI-generated for regulatory compliance |  |  |
| US-15.17.28.2 | character artist (P-9) | an opt-out list that prevents my published artwork from being used in AI training data | my creative rights are respected |  |  |
| US-15.17.28.3 | DevOps engineer (P-16) | license-safe training data verification that audits model training sources against known license databases | AI-generated assets in our project are defensible |  |  |

---
