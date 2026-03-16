# R-15.7 -- AI Governance & Provenance Requirements

## Provenance

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.7.1 | The engine **SHALL** tag every AI-generated or AI-modified asset with a persistent provenance record containing the AI system identifier, model version, timestamp, and prompt hash, surviving all pipeline stages (import, cook, packaging). | [F-15.7.1](../../features/tools-editor/ai-governance.md) | AI content origin must be traceable for compliance, licensing, and disclosure requirements. | Integration test: generate an asset via AI, import, cook, and package it. Verify the provenance tag is present and correct at each stage. |
| R-15.7.2 | The engine **SHALL** track human modifications to AI-generated assets at coarse granularity (per vertex group, per tile, per node) using compact bitmasks under 1 KB per asset, removing the provenance tag only when all AI-generated regions have been fully replaced. | [F-15.7.2](../../features/tools-editor/ai-governance.md) | Distinguishing AI from human content is required for review workflows and compliance. | Unit test: modify specific regions, verify bitmask reports them as human-modified. Replace all regions and verify provenance tag is removed. |

## Feature Toggles

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.7.3 | The engine **SHALL** provide a global toggle disabling all generative AI features, hiding their UI and returning errors from their APIs, while leaving deterministic AI (pathfinding, behavior trees, GOAP) unaffected. | [F-15.7.3](../../features/tools-editor/ai-governance.md) | Studios must be able to comply with AI-content policies while retaining gameplay AI. | Unit test: disable the toggle, verify all generative AI UI is hidden and APIs return errors. Verify pathfinding and behavior trees still function. |
| R-15.7.4 | The engine **SHALL** provide an independent toggle for AI editor assistance (voice control, agent editing, recommendations) operating separately from the generative AI content toggle, with all four combinations functional. | [F-15.7.4](../../features/tools-editor/ai-governance.md) | Fine-grained control lets studios enable assistance without content generation or vice versa. | Unit test: verify all four toggle combinations activate exactly the correct feature sets. |

## Enterprise Administration

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.7.5 | The engine **SHALL** support remote AI policy configuration via an authenticated admin API with Ed25519-signed policy documents transmitted over TLS 1.3, enabling per-team AI feature restrictions pushed to all editor instances. | [F-15.7.5](../../features/tools-editor/ai-governance.md) | Enterprise environments require centralized AI policy enforcement across all seats. | Unit test: push a policy with an invalid signature and verify the editor rejects it. |

## Audit

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.7.6 | The engine **SHALL** maintain an append-only, hash-chained audit log of all AI generation events, replicable to a central server, where tampering with any entry is detectable via hash chain validation. | [F-15.7.6](../../features/tools-editor/ai-governance.md) | Tamper-evident audit logs are required for enterprise compliance and legal review. | Unit test: modify an audit entry and verify the hash chain validation detects tampering. |

## Review Workflow

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.7.7 | The engine **SHALL** route AI-tagged assets through a configurable review workflow with approve, reject, and request-changes actions, visual diff showing AI vs human regions, and auto-approval above a configurable human modification threshold. | [F-15.7.7](../../features/tools-editor/ai-governance.md) | Quality control of AI content requires formal review before production use. | Unit test: verify auto-approval triggers at the configured threshold. |

## Packaged Builds

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.7.8 | The engine **SHALL** include provenance tags in shipped builds queryable at runtime via a public API, with minimal metadata footprint (flags and IDs only). | [F-15.7.8](../../features/tools-editor/ai-governance.md) | Runtime AI disclosure labels require provenance data in shipped builds. | Integration test: package a build and verify the runtime API returns correct provenance for AI-generated assets. |

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/ai-governance.md](../../user-stories/tools-editor/ai-governance.md).
Requirements in this document are derived from those user stories.
