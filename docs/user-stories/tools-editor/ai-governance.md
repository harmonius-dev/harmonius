# User Stories: AI Governance and Provenance

## F-15.7.1 AI Content Provenance Tagging

## US-15.7.1.1 Developer Tags AI-Generated Assets
**As a** developer (P-15), **I want** every asset produced or modified by a generative AI to
receive a persistent provenance tag recording the AI system identifier, model version, timestamp,
and prompt hash, **so that** the studio can trace the origin of all AI content.

## US-15.7.1.2 DevOps Verifies Tag Persistence Through Pipeline
**As a** DevOps engineer (P-16), **I want** provenance tags to survive all pipeline stages
(import, processing, cook, packaging), **so that** shipped builds retain provenance metadata for
compliance verification.

## US-15.7.1.3 Creative Director Audits AI Content
**As a** creative director (P-2), **I want** to filter assets by AI provenance tag in the asset
browser, **so that** I can review all AI-generated content and ensure it meets the project's
artistic standards.

## US-15.7.1.4 Engine Tester Validates Tag Survival
**As an** engine tester (P-27), **I want** to verify that provenance tags survive the complete
asset pipeline from import through packaging, **so that** metadata is never silently stripped.

## F-15.7.2 Human Modification Tracking

## US-15.7.2.1 Artist Sees Modification Status
**As an** artist (P-8), **I want** the editor to track which regions of an AI-generated asset
I have modified (per vertex group, per tile, per node), **so that** the provenance tag is
automatically removed once all AI content is replaced.

## US-15.7.2.2 Tech Artist Inspects Modification Bitmask
**As a** tech artist (P-13), **I want** modification status stored as compact bitmasks under
1 KB per asset, **so that** tracking overhead does not impact editor performance or asset size.

## US-15.7.2.3 Developer Queries Modification Percentage
**As a** developer (P-15), **I want** to query the percentage of human-modified content in an
asset via API, **so that** automated workflows can gate on modification thresholds.

## US-15.7.2.4 Engine Tester Validates Tracking Granularity
**As an** engine tester (P-27), **I want** to verify that modification tracking operates at
the documented granularity (vertex group, tile, node) for each asset type, **so that** the
system is neither too coarse nor too fine.

## F-15.7.3 Generative AI Feature Toggle

## US-15.7.3.1 Server Admin Disables Generative AI
**As a** server admin (P-22), **I want** a global toggle that disables all generative AI and
LLM-based features (hiding UI entry points and returning API errors), **so that** studios
with AI content policies can enforce them engine-wide.

## US-15.7.3.2 Designer Keeps Deterministic AI
**As a** designer (P-5), **I want** the generative AI toggle to leave deterministic AI systems
(pathfinding, behavior trees, utility AI, GOAP) entirely unaffected, **so that** disabling
generative AI does not break any gameplay logic.

## US-15.7.3.3 Engine Dev Enforces Restart Requirement
**As an** engine developer (P-26), **I want** the toggle evaluated at subsystem initialization
requiring an editor restart, **so that** all generative AI services are fully torn down or
brought up cleanly.

## US-15.7.3.4 Engine Tester Validates Toggle Isolation
**As an** engine tester (P-27), **I want** to verify that disabling generative AI completely
removes all LLM-related UI, API endpoints, and background services, **so that** no generative
AI functionality is accessible when the toggle is off.

## F-15.7.4 AI Assistance Toggle

## US-15.7.4.1 Designer Toggles AI Assistance Independently
**As a** designer (P-5), **I want** an independent toggle for AI editor assistance (voice
control, agent editing, recommendations) separate from generative AI content, **so that** I can
use productivity tools without enabling content generation.

## US-15.7.4.2 Server Admin Controls Both Toggles
**As a** server admin (P-22), **I want** both toggles configurable independently (both on, both
off, or either one), **so that** the studio has fine-grained control over AI capabilities.

## US-15.7.4.3 Developer Queries Toggle State
**As a** developer (P-15), **I want** to query the AI assistance toggle state via API, **so
that** my plugins can adapt their behavior based on the studio's AI policy.

## US-15.7.4.4 Engine Tester Validates Independence
**As an** engine tester (P-27), **I want** to verify that disabling AI assistance does not
affect generative AI features and vice versa, **so that** the two toggles are truly
independent.

## F-15.7.5 Enterprise Remote Administration

## US-15.7.5.1 Server Admin Pushes Policy Updates
**As a** server admin (P-22), **I want** to push AI feature toggle and provenance policy updates
to all editor instances via an authenticated admin API, **so that** policy changes take effect
organization-wide without manual per-machine configuration.

## US-15.7.5.2 DevOps Verifies Policy Signatures
**As a** DevOps engineer (P-16), **I want** policy documents cryptographically signed with
Ed25519 and verified by editors before application, **so that** tampered policies are rejected
automatically.

## US-15.7.5.3 Engine Dev Supports Push and Pull
**As an** engine developer (P-26), **I want** policy distribution to support both push
(WebSocket) and pull (HTTPS polling) models, **so that** diverse network topologies are
accommodated.

## US-15.7.5.4 Engine Tester Validates Tamper Detection
**As an** engine tester (P-27), **I want** to verify that editors reject policies with invalid
signatures, **so that** only authentic administrator-issued policies are applied.

## F-15.7.6 AI Content Audit Trail

## US-15.7.6.1 Server Admin Reviews Audit Log
**As a** server admin (P-22), **I want** an append-only audit log recording every AI content
generation event (asset, model version, user, timestamp) with hash-chained entries, **so that**
any tampering or deletion is detectable.

## US-15.7.6.2 DevOps Replicates Logs to Central Server
**As a** DevOps engineer (P-16), **I want** audit logs replicable to a central server in
enterprise configurations, **so that** all AI activity across the organization is collected
for compliance review.

## US-15.7.6.3 Creative Director Tracks AI Usage
**As a** creative director (P-2), **I want** to query the audit trail to see how much AI
content generation the team is using, **so that** I can make informed decisions about AI
adoption and review priorities.

## US-15.7.6.4 Engine Tester Validates Hash Chain Integrity
**As an** engine tester (P-27), **I want** to verify that the hash chain across audit log
segments is intact after log rotation, **so that** the tamper-detection guarantee holds across
all log files.

## F-15.7.7 AI Content Review Workflow

## US-15.7.7.1 Creative Director Routes AI Content for Review
**As a** creative director (P-2), **I want** assets with AI provenance tags routed through a
mandatory review workflow before entering the production database, **so that** no AI content
ships without human approval.

## US-15.7.7.2 Artist Reviews AI-Generated Regions
**As an** artist (P-8), **I want** to see the full provenance metadata and a visual diff
highlighting AI-generated versus human-authored regions, **so that** I can evaluate exactly
what was AI-generated during review.

## US-15.7.7.3 Designer Configures Review Thresholds
**As a** designer (P-5), **I want** to configure the review workflow per project (all AI
content, specific asset types, or auto-approve above a modification threshold), **so that**
review overhead matches the studio's risk tolerance.

## US-15.7.7.4 Engine Tester Validates Workflow Enforcement
**As an** engine tester (P-27), **I want** to verify that the mandatory review workflow blocks
AI-tagged assets from the production database until approved, **so that** the review gate
cannot be bypassed.

## F-15.7.8 Provenance Metadata in Packaged Builds

## US-15.7.8.1 Developer Queries Provenance at Runtime
**As a** developer (P-15), **I want** runtime APIs to read AI provenance metadata for any
loaded asset, **so that** the game can display "AI-generated content" labels in credits,
settings, or in-game UI.

## US-15.7.8.2 Server Admin Ensures Minimal Footprint
**As a** server admin (P-22), **I want** only provenance flags and AI system identifiers
retained in packaged builds (not full prompt data), **so that** the metadata footprint is
minimal in shipped games.

## US-15.7.8.3 DevOps Formats Disclosures Per Platform
**As a** DevOps engineer (P-16), **I want** the runtime API to provide raw provenance data that
the game's UI layer formats per platform requirements, **so that** disclosure formats comply
with each platform holder's certification rules.

## US-15.7.8.4 Engine Tester Validates Runtime Query
**As an** engine tester (P-27), **I want** to verify that provenance metadata is queryable in
packaged builds on all platforms, **so that** runtime disclosure features work correctly after
cooking and packaging.
