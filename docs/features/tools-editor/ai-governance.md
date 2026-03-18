# 15.7 — AI Governance & Provenance

## Content Tagging

| ID       | Feature                       | Requirements |
|----------|-------------------------------|--------------|
| F-15.7.1 | AI Content Provenance Tagging | R-15.7.1     |
| F-15.7.2 | Human Modification Tracking   | R-15.7.2     |

1. **F-15.7.1** — Every asset produced or modified by a generative AI or LLM system receives a
   persistent provenance tag recording the AI system identifier, model version, timestamp, and a
   hash of the prompt or generation parameters used. The tag is embedded in the asset metadata and
   survives all pipeline stages — import, processing, cook, and packaging. A provenance tag is only
   removed when every AI-generated region in the asset has been fully replaced by human-authored
   content, as determined by the human modification tracking system (F-15.7.2). asset headers on all
   platforms. Packaged builds retain a read-only copy of the provenance data.
   - **Deps:** F-12.3.1
   - **Platform:** Provenance metadata is stored in the asset registry and serialized alongside
2. **F-15.7.2** — Tracks which regions of an AI-generated asset have been subsequently modified by a
   human author at a coarse granularity optimized for performance rather than per-element precision.
   For meshes, modification status is tracked per vertex group. For textures, per tile or per layer.
   For data tables, per row or column. For logic graphs, per node. Modification metadata is stored
   as compact bitmasks to minimize memory and serialization overhead. The provenance tag (F-15.7.1)
   consults this tracking data to determine whether all AI-generated content has been replaced. for
   typical production content.
   - **Deps:** F-15.7.1, F-12.3.1
   - **Platform:** Bitmask storage scales with asset complexity but remains under 1 KB per asset

## Feature Toggles

| ID       | Feature                      | Requirements |
|----------|------------------------------|--------------|
| F-15.7.3 | Generative AI Feature Toggle | R-15.7.3     |
| F-15.7.4 | AI Assistance Toggle         | R-15.7.4     |

1. **F-15.7.3** — Provides a global engine setting that disables all generative AI and LLM-based
   features. When disabled, AI content search (F-12.3.8), AI categorization (F-12.3.7), AI-driven
   procedural content generation (F-3.6.29), and any LLM integrations become completely unavailable
   — their UI entry points are hidden and their APIs return errors. Deterministic AI systems such as
   pathfinding (F-7.1), navmesh generation (F-7.1.4), behavior trees (F-7.3), utility AI, and GOAP
   planners are entirely unaffected by this toggle. editor restart to ensure all generative AI
   services are fully torn down or brought up.
   - **Deps:** F-12.3.7, F-12.3.8, F-3.6.29, F-7.1.1
   - **Platform:** The toggle is evaluated at subsystem initialization. Changing it requires an
2. **F-15.7.4** — An independent toggle that controls AI-based editor assistance features such as
   voice control, agent-driven editing, intelligent recommendations, and contextual suggestions.
   This toggle operates separately from the generative AI content toggle (F-15.7.3): disabling AI
   assistance does not affect AI content generation, and disabling AI content generation does not
   affect AI assistance. Both toggles can be enabled, both disabled, or either one independently,
   giving studios fine-grained control over which AI capabilities are active. or console runtime.
   - **Deps:** F-15.7.3
   - **Platform:** Desktop only. AI assistance features are editor-only and not present in mobile

## Enterprise Administration

| ID       | Feature                          | Requirements |
|----------|----------------------------------|--------------|
| F-15.7.5 | Enterprise Remote Administration | R-15.7.5     |

1. **F-15.7.5** — Enables remote configuration of AI feature toggles and provenance policies via an
   authenticated admin API. Enterprise administrators can push policy updates to all editor
   instances in an organization — disabling AI content generation globally, restricting AI
   assistance to specific teams, or enforcing mandatory provenance tagging and review. Policy
   documents are cryptographically signed using Ed25519 to prevent tampering, and editors verify the
   signature before applying any policy change. supports both push (WebSocket) and pull (HTTPS
   polling) models to accommodate diverse network topologies.
   - **Deps:** F-15.7.3, F-15.7.4
   - **Platform:** The admin API uses TLS 1.3 for transport security. Policy distribution

## Audit and Compliance

| ID       | Feature                                | Requirements |
|----------|----------------------------------------|--------------|
| F-15.7.6 | AI Content Audit Trail                 | R-15.7.6     |
| F-15.7.7 | AI Content Review Workflow             | R-15.7.7     |
| F-15.7.8 | Provenance Metadata in Packaged Builds | R-15.7.8     |

1. **F-15.7.6** — Maintains an append-only audit log that records every AI content generation event:
   the asset produced, the AI system and model version used, generation parameters and prompt hash,
   the user who triggered the operation, and a precise timestamp. Log entries are hash-chained so
   that any tampering or deletion is detectable. The audit trail supports compliance reviews,
   intellectual property auditing, and integration with external governance and legal workflows.
   enterprise configurations. Log rotation preserves the hash chain across segments.
   - **Deps:** F-15.7.1
   - **Platform:** Audit logs are stored locally and can be replicated to a central server in
2. **F-15.7.7** — Assets carrying AI provenance tags can be routed through a mandatory review
   workflow before entering the production asset database. Reviewers see the full provenance
   metadata, human modification status, and a visual diff highlighting AI-generated versus
   human-authored regions. Reviewers can approve, reject, or request changes. The workflow is
   configurable per project: studios can require review for all AI content, limit mandatory review
   to specific asset types, or allow auto-approval for assets where human modification exceeds a
   configurable threshold. console runtime.
   - **Deps:** F-15.7.1, F-15.7.2, F-15.7.6
   - **Platform:** Desktop only. Review workflow runs in the editor. Not available on mobile or
3. **F-15.7.8** — AI provenance tags survive asset cooking and packaging and remain queryable in
   shipped builds. Runtime APIs allow the game to read provenance metadata for any loaded asset,
   enabling studios to display "AI-generated content" labels in credits, settings menus, or in-game
   UI as required by platform holder policies or regional regulations. The metadata footprint in
   packaged builds is minimal — only provenance flags and AI system identifiers are retained, not
   full prompt data. runtime API provides raw data that the game's UI layer formats per platform
   requirements.
   - **Deps:** F-15.7.1, F-12.4.1
   - **Platform:** Console platform certification may require specific disclosure formats; the
