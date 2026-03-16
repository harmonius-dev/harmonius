# R-15.7 -- AI Governance & Provenance Requirements

## Provenance

### R-15.7.1 AI Content Provenance Tagging

The engine **SHALL** tag every AI-generated or AI-modified asset with a persistent provenance record
containing the AI system identifier, model version, timestamp, and prompt hash, surviving all
pipeline stages (import, cook, packaging).

- **Derived from:**
  [F-15.7.1](../../features/tools-editor/ai-governance.md)
- **Rationale:** AI content origin must be traceable for compliance, licensing, and disclosure
  requirements.
- **Verification:** Integration test: generate an asset via AI, import, cook, and package it. Verify
  the provenance tag is present and correct at each stage.

### R-15.7.2 Human Modification Tracking

The engine **SHALL** track human modifications to AI-generated assets at coarse granularity (per
vertex group, per tile, per node) using compact bitmasks under 1 KB per asset, removing the
provenance tag only when all AI-generated regions have been fully replaced.

- **Derived from:**
  [F-15.7.2](../../features/tools-editor/ai-governance.md)
- **Rationale:** Distinguishing AI from human content is required for review workflows and
  compliance.
- **Verification:** Unit test: modify specific regions, verify bitmask reports them as
  human-modified. Replace all regions and verify provenance tag is removed.

## Feature Toggles

### R-15.7.3 Generative AI Feature Toggle

The engine **SHALL** provide a global toggle disabling all generative AI features, hiding their UI
and returning errors from their APIs, while leaving deterministic AI (pathfinding, behavior trees,
GOAP) unaffected.

- **Derived from:**
  [F-15.7.3](../../features/tools-editor/ai-governance.md)
- **Rationale:** Studios must be able to comply with AI-content policies while retaining gameplay
  AI.
- **Verification:** Unit test: disable the toggle, verify all generative AI UI is hidden and APIs
  return errors. Verify pathfinding and behavior trees still function.

### R-15.7.4 AI Assistance Toggle

The engine **SHALL** provide an independent toggle for AI editor assistance (voice control, agent
editing, recommendations) operating separately from the generative AI content toggle, with all four
combinations functional.

- **Derived from:**
  [F-15.7.4](../../features/tools-editor/ai-governance.md)
- **Rationale:** Fine-grained control lets studios enable assistance without content generation or
  vice versa.
- **Verification:** Unit test: verify all four toggle combinations activate exactly the correct
  feature sets.

## Enterprise Administration

### R-15.7.5 Enterprise Remote Administration

The engine **SHALL** support remote AI policy configuration via an authenticated admin API with
Ed25519-signed policy documents transmitted over TLS 1.3, enabling per-team AI feature restrictions
pushed to all editor instances.

- **Derived from:**
  [F-15.7.5](../../features/tools-editor/ai-governance.md)
- **Rationale:** Enterprise environments require centralized AI policy enforcement across all seats.
- **Verification:** Unit test: push a policy with an invalid signature and verify the editor rejects
  it.

## Audit

### R-15.7.6 AI Content Audit Trail

The engine **SHALL** maintain an append-only, hash-chained audit log of all AI generation events,
replicable to a central server, where tampering with any entry is detectable via hash chain
validation.

- **Derived from:**
  [F-15.7.6](../../features/tools-editor/ai-governance.md)
- **Rationale:** Tamper-evident audit logs are required for enterprise compliance and legal review.
- **Verification:** Unit test: modify an audit entry and verify the hash chain validation detects
  tampering.

## Review Workflow

### R-15.7.7 AI Content Review Workflow

The engine **SHALL** route AI-tagged assets through a configurable review workflow with approve,
reject, and request-changes actions, visual diff showing AI vs human regions, and auto-approval
above a configurable human modification threshold.

- **Derived from:**
  [F-15.7.7](../../features/tools-editor/ai-governance.md)
- **Rationale:** Quality control of AI content requires formal review before production use.
- **Verification:** Unit test: verify auto-approval triggers at the configured threshold.

## Packaged Builds

### R-15.7.8 Provenance in Packaged Builds

The engine **SHALL** include provenance tags in shipped builds queryable at runtime via a public
API, with minimal metadata footprint (flags and IDs only).

- **Derived from:**
  [F-15.7.8](../../features/tools-editor/ai-governance.md)
- **Rationale:** Runtime AI disclosure labels require provenance data in shipped builds.
- **Verification:** Integration test: package a build and verify the runtime API returns correct
  provenance for AI-generated assets.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/ai-governance.md](../../user-stories/tools-editor/ai-governance.md).
Requirements in this document are derived from those
user stories.
