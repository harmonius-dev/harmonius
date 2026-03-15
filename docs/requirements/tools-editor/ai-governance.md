# R-15.7 — AI Governance & Provenance Requirements

## Content Tagging

### R-15.7.1 AI Content Provenance Tagging

The engine **SHALL** attach a persistent provenance tag to every asset produced or modified by a
generative AI system, recording the AI system identifier, model version, timestamp, and prompt
hash, surviving all pipeline stages (import, processing, cook, packaging), with removal only
when all AI-generated regions are fully replaced by human-authored content as determined by
the human modification tracking system.

- **Derived from:** [F-15.7.1](../../features/tools-editor/ai-governance.md)
- **Rationale:** Provenance tagging enables compliance with platform holder policies and regional
  regulations that require disclosure of AI-generated content.
- **Verification:** Generate an asset with an AI system; verify the provenance tag persists
  through import, processing, cook, and packaging stages. Manually replace all AI-generated
  content and confirm the tag is removed. Partially replace content and confirm the tag remains.

### R-15.7.2 Human Modification Tracking

The engine **SHALL** track human modifications to AI-generated assets at coarse granularity (per
vertex group for meshes, per tile/layer for textures, per row/column for data tables, per node
for logic graphs) using compact bitmasks under 1 KB per typical production asset, and report
whether all AI-generated content has been replaced.

- **Derived from:** [F-15.7.2](../../features/tools-editor/ai-governance.md)
- **Rationale:** Tracking human modifications enables automated determination of when provenance
  tags can be removed and supports review workflows that distinguish AI from human content.
- **Verification:** Generate a mesh with 4 vertex groups via AI; modify 2 groups manually and
  verify the bitmask reports exactly 2 groups as human-modified. Modify all 4 and confirm the
  system reports full human replacement. Assert bitmask size remains under 1 KB.

## Feature Toggles

### R-15.7.3 Generative AI Feature Toggle

The engine **SHALL** provide a global setting that disables all generative AI and LLM-based
features (AI content search, AI categorization, AI procedural generation, LLM integrations) by
hiding their UI entry points and returning errors from their APIs, while leaving deterministic AI
systems (pathfinding, navmesh generation, behavior trees, utility AI, GOAP) unaffected.

- **Derived from:** [F-15.7.3](../../features/tools-editor/ai-governance.md)
- **Rationale:** Studios must be able to disable generative AI entirely for legal, ethical, or
  policy reasons without losing deterministic AI gameplay systems.
- **Verification:** Disable the generative AI toggle; verify all generative AI UI elements are
  hidden and API calls return errors. Confirm pathfinding, navmesh generation, and behavior
  trees continue to function. Re-enable and verify generative AI features are restored.

### R-15.7.4 AI Assistance Toggle

The engine **SHALL** provide an independent toggle controlling AI-based editor assistance features
(voice control, agent-driven editing, recommendations, contextual suggestions) that operates
separately from the generative AI content toggle, allowing all four combinations of the two
toggles.

- **Derived from:** [F-15.7.4](../../features/tools-editor/ai-governance.md)
- **Rationale:** Studios need fine-grained control to enable AI assistance for productivity
  while disabling generative content creation, or vice versa.
- **Verification:** Test all four toggle combinations (both on, both off, each independently);
  verify AI assistance features appear/disappear independently of generative AI content features
  in each combination.

## Enterprise Administration

### R-15.7.5 Enterprise Remote Administration

The engine **SHALL** enable remote configuration of AI feature toggles and provenance policies via
an authenticated admin API, supporting organization-wide policy distribution with Ed25519
cryptographic signature verification on all policy documents before application.

- **Derived from:** [F-15.7.5](../../features/tools-editor/ai-governance.md)
- **Rationale:** Enterprise administrators need centralized control over AI policies across all
  editor instances to enforce organizational compliance requirements.
- **Verification:** Push a signed policy disabling generative AI to a running editor; verify the
  toggle changes and generative AI features become unavailable. Push a policy with an invalid
  signature and verify the editor rejects it without applying changes.

## Audit and Compliance

### R-15.7.6 AI Content Audit Trail

The engine **SHALL** maintain an append-only, hash-chained audit log recording every AI content
generation event with the asset produced, AI system and model version, generation parameters,
prompt hash, triggering user, and timestamp, such that any tampering or deletion is detectable
via hash chain verification.

- **Derived from:** [F-15.7.6](../../features/tools-editor/ai-governance.md)
- **Rationale:** An immutable audit trail supports intellectual property auditing, compliance
  reviews, and integration with legal governance workflows.
- **Verification:** Generate 10 assets via AI; verify the audit log contains exactly 10 entries
  with all required fields. Tamper with one entry and verify hash chain validation detects the
  corruption. Delete one entry and verify the gap is detected.

### R-15.7.7 AI Content Review Workflow

The engine **SHALL** route assets with AI provenance tags through a configurable review workflow
before they enter the production asset database, showing reviewers the full provenance metadata,
human modification status, and a visual diff of AI versus human regions, with approve, reject,
and request-changes actions, configurable per project to require review for all AI content,
specific asset types, or auto-approve above a human modification threshold.

- **Derived from:** [F-15.7.7](../../features/tools-editor/ai-governance.md)
- **Rationale:** Mandatory review of AI-generated content ensures quality control and compliance
  before assets enter the production pipeline.
- **Verification:** Generate an AI asset and submit for review; verify the reviewer sees
  provenance metadata, modification status, and visual diff. Approve, reject, and request
  changes on separate assets; verify each action produces the correct workflow state transition.
  Set auto-approve threshold to 80% and verify assets with 90% human modification bypass review.

### R-15.7.8 Provenance Metadata in Packaged Builds

The engine **SHALL** retain AI provenance tags through asset cooking and packaging into shipped
builds, exposing runtime APIs to query provenance metadata (provenance flags and AI system
identifiers) for any loaded asset, with minimal metadata footprint excluding full prompt data.

- **Derived from:** [F-15.7.8](../../features/tools-editor/ai-governance.md)
- **Rationale:** Runtime access to provenance data enables studios to display AI content
  disclosures as required by platform certification and regional regulations.
- **Verification:** Cook and package a build containing AI-generated assets; query the runtime
  API for each asset and verify provenance flags and AI system identifiers are returned. Verify
  full prompt data is not present in the packaged metadata.
