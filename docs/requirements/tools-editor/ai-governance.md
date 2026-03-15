# R-15.7 -- AI Governance & Provenance User Stories

## US-15.7.1 AI Content Provenance Tagging

### US-15.7.1.1
As a **creative director (P-2)**, I want AI-generated assets tagged with provenance
so that I can identify which content was AI-produced.

### US-15.7.1.2
As a **developer (P-15)**, I want provenance tags surviving import, cook, and packaging
so that AI origin information is never lost in the pipeline.

### US-15.7.1.3
As a **developer (P-15)**, I want provenance tags recording AI system, model version,
and timestamp
so that the exact generation context is traceable.

### US-15.7.1.4
As a **artist (P-8)**, I want provenance removal when all AI content is replaced
so that fully human-authored assets are not falsely tagged.

### US-15.7.1.5
As an **engine tester (P-27)**, I want to verify provenance survives all pipeline stages
so that tag persistence is regression-tested.

---

## US-15.7.2 Human Modification Tracking

### US-15.7.2.1
As a **artist (P-8)**, I want tracking of which regions I have manually modified
so that my contributions to AI-generated assets are recorded.

### US-15.7.2.2
As a **developer (P-15)**, I want coarse-granularity tracking (per vertex group,
per tile, per node)
so that tracking overhead is minimal.

### US-15.7.2.3
As a **developer (P-15)**, I want compact bitmask storage under 1 KB per asset
so that modification metadata does not bloat asset sizes.

### US-15.7.2.4
As an **engine tester (P-27)**, I want to verify bitmask correctly reports human-modified
regions
so that tracking accuracy is regression-tested.

---

## US-15.7.3 Generative AI Feature Toggle

### US-15.7.3.1
As a **creative director (P-2)**, I want a global toggle to disable all generative AI
so that the studio can comply with AI-content policies.

### US-15.7.3.2
As a **designer (P-5)**, I want deterministic AI (pathfinding, behavior trees) unaffected
by the toggle
so that gameplay AI continues to function normally.

### US-15.7.3.3
As a **developer (P-15)**, I want generative AI UI hidden and APIs returning errors
when disabled
so that the toggle completely removes generative AI from the interface.

### US-15.7.3.4
As an **engine tester (P-27)**, I want to verify disabling the toggle hides all
generative AI features
so that toggle completeness is regression-tested.

---

## US-15.7.4 AI Assistance Toggle

### US-15.7.4.1
As a **creative director (P-2)**, I want an independent toggle for AI editor assistance
so that voice control and recommendations are managed separately from
content generation.

### US-15.7.4.2
As a **designer (P-5)**, I want all four toggle combinations to work independently
so that I can enable assistance without content generation or vice versa.

### US-15.7.4.3
As an **engine tester (P-27)**, I want to verify all four toggle combinations activate
correct feature sets
so that toggle independence is regression-tested.

---

## US-15.7.5 Enterprise Remote Administration

### US-15.7.5.1
As a **server admin (P-22)**, I want to push AI policy updates to all editor instances
so that organization-wide AI settings change centrally.

### US-15.7.5.2
As a **server admin (P-22)**, I want Ed25519-signed policy documents
so that editors reject tampered or unauthorized policies.

### US-15.7.5.3
As a **server admin (P-22)**, I want to restrict AI features per team via policy
so that different teams have appropriate AI access levels.

### US-15.7.5.4
As an **engine dev (P-26)**, I want TLS 1.3 for admin API transport
so that policy distribution is encrypted in transit.

### US-15.7.5.5
As an **engine tester (P-27)**, I want to verify an invalid signature is rejected
so that policy verification is regression-tested.

---

## US-15.7.6 AI Content Audit Trail

### US-15.7.6.1
As a **creative director (P-2)**, I want an append-only audit log of AI generation events
so that I can review all AI content creation for compliance.

### US-15.7.6.2
As a **developer (P-15)**, I want hash-chained log entries for tamper detection
so that any deletion or modification of audit records is detectable.

### US-15.7.6.3
As a **server admin (P-22)**, I want audit logs replicable to a central server
so that enterprise environments have centralized compliance data.

### US-15.7.6.4
As an **engine tester (P-27)**, I want to verify tampering with an audit entry is
detected by hash chain validation
so that integrity detection is regression-tested.

---

## US-15.7.7 AI Content Review Workflow

### US-15.7.7.1
As a **creative director (P-2)**, I want AI-tagged assets routed through review
so that AI content is approved before entering production.

### US-15.7.7.2
As a **artist (P-8)**, I want to see provenance metadata and visual diff during review
so that I can distinguish AI-generated from human-authored regions.

### US-15.7.7.3
As a **artist (P-8)**, I want approve, reject, and request-changes actions
so that I can manage AI content quality through a formal workflow.

### US-15.7.7.4
As a **developer (P-15)**, I want configurable review policy per project
so that review can be required for all AI content or only specific types.

### US-15.7.7.5
As a **developer (P-15)**, I want auto-approval above a human modification threshold
so that heavily modified assets bypass review automatically.

### US-15.7.7.6
As an **engine tester (P-27)**, I want to verify auto-approval triggers at the
configured threshold
so that review policy enforcement is regression-tested.

---

## US-15.7.8 Provenance Metadata in Packaged Builds

### US-15.7.8.1
As a **developer (P-15)**, I want provenance tags in shipped builds queryable at runtime
so that the game can display AI content disclosures.

### US-15.7.8.2
As a **developer (P-15)**, I want minimal metadata footprint (flags and IDs only)
so that provenance does not bloat packaged builds.

### US-15.7.8.3
As a **DevOps (P-16)**, I want runtime APIs for provenance queries
so that UI teams can integrate AI disclosure labels.

### US-15.7.8.4
As an **engine tester (P-27)**, I want to verify runtime API returns correct provenance
for packaged assets
so that shipped provenance is regression-tested.
