# User Stories: AI Governance and Provenance

## F-15.7.1 AI Content Provenance Tagging

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.7.1.1 | developer (P-15) | every asset produced or modified by a generative AI to receive a persistent provenance tag recording the AI system identifier, model version, timestamp, and prompt hash | the studio can trace the origin of all AI content |  |  |
| US-15.7.1.2 | DevOps engineer (P-16) | provenance tags to survive all pipeline stages (import, processing, cook, packaging) | shipped builds retain provenance metadata for compliance verification |  |  |
| US-15.7.1.3 | creative director (P-2) | to filter assets by AI provenance tag in the asset browser | I can review all AI-generated content and ensure it meets the project's artistic standards |  |  |
| US-15.7.1.4 | engine tester (P-27) | to verify that provenance tags survive the complete asset pipeline from import through packaging | metadata is never silently stripped |  |  |

## F-15.7.2 Human Modification Tracking

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.7.2.1 | artist (P-8) | the editor to track which regions of an AI-generated asset I have modified (per vertex group, per tile, per node) | the provenance tag is automatically removed once all AI content is replaced |  |  |
| US-15.7.2.2 | tech artist (P-13) | modification status stored as compact bitmasks under 1 KB per asset | tracking overhead does not impact editor performance or asset size |  |  |
| US-15.7.2.3 | developer (P-15) | to query the percentage of human-modified content in an asset via API | automated workflows can gate on modification thresholds |  |  |
| US-15.7.2.4 | engine tester (P-27) | to verify that modification tracking operates at the documented granularity (vertex group, tile, node) for each asset type | the system is neither too coarse nor too fine |  |  |

## F-15.7.3 Generative AI Feature Toggle

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.7.3.1 | server admin (P-22) | a global toggle that disables all generative AI and LLM-based features (hiding UI entry points and returning API errors) | studios with AI content policies can enforce them engine-wide |  |  |
| US-15.7.3.2 | designer (P-5) | the generative AI toggle to leave deterministic AI systems (pathfinding, behavior trees, utility AI, GOAP) entirely unaffected | disabling generative AI does not break any gameplay logic |  |  |
| US-15.7.3.3 | engine developer (P-26) | the toggle evaluated at subsystem initialization requiring an editor restart | all generative AI services are fully torn down or brought up cleanly |  |  |
| US-15.7.3.4 | engine tester (P-27) | to verify that disabling generative AI completely removes all LLM-related UI, API endpoints, and background services | no generative AI functionality is accessible when the toggle is off |  |  |

## F-15.7.4 AI Assistance Toggle

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.7.4.1 | designer (P-5) | an independent toggle for AI editor assistance (voice control, agent editing, recommendations) separate from generative AI content | I can use productivity tools without enabling content generation |  |  |
| US-15.7.4.2 | server admin (P-22) | both toggles configurable independently (both on, both off, or either one) | the studio has fine-grained control over AI capabilities |  |  |
| US-15.7.4.3 | developer (P-15) | to query the AI assistance toggle state via API | my plugins can adapt their behavior based on the studio's AI policy |  |  |
| US-15.7.4.4 | engine tester (P-27) | to verify that disabling AI assistance does not affect generative AI features and vice versa | the two toggles are truly independent |  |  |

## F-15.7.5 Enterprise Remote Administration

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.7.5.1 | server admin (P-22) | to push AI feature toggle and provenance policy updates to all editor instances via an authenticated admin API | policy changes take effect organization-wide without manual per-machine configuration |  |  |
| US-15.7.5.2 | DevOps engineer (P-16) | policy documents cryptographically signed with Ed25519 and verified by editors before application | tampered policies are rejected automatically |  |  |
| US-15.7.5.3 | engine developer (P-26) | policy distribution to support both push (WebSocket) and pull (HTTPS polling) models | diverse network topologies are accommodated |  |  |
| US-15.7.5.4 | engine tester (P-27) | to verify that editors reject policies with invalid signatures | only authentic administrator-issued policies are applied |  |  |

## F-15.7.6 AI Content Audit Trail

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.7.6.1 | server admin (P-22) | an append-only audit log recording every AI content generation event (asset, model version, user, timestamp) with hash-chained entries | any tampering or deletion is detectable |  |  |
| US-15.7.6.2 | DevOps engineer (P-16) | audit logs replicable to a central server in enterprise configurations | all AI activity across the organization is collected for compliance review |  |  |
| US-15.7.6.3 | creative director (P-2) | to query the audit trail to see how much AI content generation the team is using | I can make informed decisions about AI adoption and review priorities |  |  |
| US-15.7.6.4 | engine tester (P-27) | to verify that the hash chain across audit log segments is intact after log rotation | the tamper-detection guarantee holds across all log files |  |  |

## F-15.7.7 AI Content Review Workflow

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.7.7.1 | creative director (P-2) | assets with AI provenance tags routed through a mandatory review workflow before entering the production database | no AI content ships without human approval |  |  |
| US-15.7.7.2 | artist (P-8) | to see the full provenance metadata and a visual diff highlighting AI-generated versus human-authored regions | I can evaluate exactly what was AI-generated during review |  |  |
| US-15.7.7.3 | designer (P-5) | to configure the review workflow per project (all AI content, specific asset types, or auto-approve above a modification threshold) | review overhead matches the studio's risk tolerance |  |  |
| US-15.7.7.4 | engine tester (P-27) | to verify that the mandatory review workflow blocks AI-tagged assets from the production database until approved | the review gate cannot be bypassed |  |  |

## F-15.7.8 Provenance Metadata in Packaged Builds

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.7.8.1 | developer (P-15) | runtime APIs to read AI provenance metadata for any loaded asset | the game can display "AI-generated content" labels in credits, settings, or in-game UI |  |  |
| US-15.7.8.2 | server admin (P-22) | only provenance flags and AI system identifiers retained in packaged builds (not full prompt data) | the metadata footprint is minimal in shipped games |  |  |
| US-15.7.8.3 | DevOps engineer (P-16) | the runtime API to provide raw provenance data that the game's UI layer formats per platform requirements | disclosure formats comply with each platform holder's certification rules |  |  |
| US-15.7.8.4 | engine tester (P-27) | to verify that provenance metadata is queryable in packaged builds on all platforms | runtime disclosure features work correctly after cooking and packaging |  |  |
