# User Stories: AI Governance and Provenance

## F-15.7.1 AI Content Provenance Tagging

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.7.1.1 | developer (P-15)        |          |              |
| US-15.7.1.2 | DevOps engineer (P-16)  |          |              |
| US-15.7.1.3 | creative director (P-2) |          |              |
| US-15.7.1.4 | engine tester (P-27)    |          |              |

1. **US-15.7.1.1** — every asset produced or modified by a generative AI to receive a persistent
   provenance tag recording the AI system identifier, model version, timestamp, and prompt hash
   - **Acceptance:** the studio can trace the origin of all AI content
2. **US-15.7.1.2** — provenance tags to survive all pipeline stages (import, processing, cook,
   packaging)
   - **Acceptance:** shipped builds retain provenance metadata for compliance verification
3. **US-15.7.1.3** — to filter assets by AI provenance tag in the asset browser
   - **Acceptance:** I can review all AI-generated content and ensure it meets the project's
     artistic standards
4. **US-15.7.1.4** — to verify that provenance tags survive the complete asset pipeline from import
   through packaging
   - **Acceptance:** metadata is never silently stripped

## F-15.7.2 Human Modification Tracking

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-15.7.2.1 | artist (P-8)         |          |              |
| US-15.7.2.2 | tech artist (P-13)   |          |              |
| US-15.7.2.3 | developer (P-15)     |          |              |
| US-15.7.2.4 | engine tester (P-27) |          |              |

1. **US-15.7.2.1** — the editor to track which regions of an AI-generated asset I have modified (per
   vertex group, per tile, per node)
   - **Acceptance:** the provenance tag is automatically removed once all AI content is replaced
2. **US-15.7.2.2** — modification status stored as compact bitmasks under 1 KB per asset
   - **Acceptance:** tracking overhead does not impact editor performance or asset size
3. **US-15.7.2.3** — to query the percentage of human-modified content in an asset via API
   - **Acceptance:** automated workflows can gate on modification thresholds
4. **US-15.7.2.4** — to verify that modification tracking operates at the documented granularity
   (vertex group, tile, node) for each asset type
   - **Acceptance:** the system is neither too coarse nor too fine

## F-15.7.3 Generative AI Feature Toggle

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.7.3.1 | server admin (P-22)     |          |              |
| US-15.7.3.2 | designer (P-5)          |          |              |
| US-15.7.3.3 | engine developer (P-26) |          |              |
| US-15.7.3.4 | engine tester (P-27)    |          |              |

1. **US-15.7.3.1** — a global toggle that disables all generative AI and LLM-based features (hiding
   UI entry points and returning API errors)
   - **Acceptance:** studios with AI content policies can enforce them engine-wide
2. **US-15.7.3.2** — the generative AI toggle to leave deterministic AI systems (pathfinding,
   behavior trees, utility AI, GOAP) entirely unaffected
   - **Acceptance:** disabling generative AI does not break any gameplay logic
3. **US-15.7.3.3** — the toggle evaluated at subsystem initialization requiring an editor restart
   - **Acceptance:** all generative AI services are fully torn down or brought up cleanly
4. **US-15.7.3.4** — to verify that disabling generative AI completely removes all LLM-related UI,
   API endpoints, and background services
   - **Acceptance:** no generative AI functionality is accessible when the toggle is off

## F-15.7.4 AI Assistance Toggle

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-15.7.4.1 | designer (P-5)       |          |              |
| US-15.7.4.2 | server admin (P-22)  |          |              |
| US-15.7.4.3 | developer (P-15)     |          |              |
| US-15.7.4.4 | engine tester (P-27) |          |              |

1. **US-15.7.4.1** — an independent toggle for AI editor assistance (voice control, agent editing,
   recommendations) separate from generative AI content
   - **Acceptance:** I can use productivity tools without enabling content generation
2. **US-15.7.4.2** — both toggles configurable independently (both on, both off, or either one)
   - **Acceptance:** the studio has fine-grained control over AI capabilities
3. **US-15.7.4.3** — to query the AI assistance toggle state via API
   - **Acceptance:** my plugins can adapt their behavior based on the studio's AI policy
4. **US-15.7.4.4** — to verify that disabling AI assistance does not affect generative AI features
   and vice versa
   - **Acceptance:** the two toggles are truly independent

## F-15.7.5 Enterprise Remote Administration

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.7.5.1 | server admin (P-22)     |          |              |
| US-15.7.5.2 | DevOps engineer (P-16)  |          |              |
| US-15.7.5.3 | engine developer (P-26) |          |              |
| US-15.7.5.4 | engine tester (P-27)    |          |              |

1. **US-15.7.5.1** — to push AI feature toggle and provenance policy updates to all editor instances
   via an authenticated admin API
   - **Acceptance:** policy changes take effect organization-wide without manual per-machine
     configuration
2. **US-15.7.5.2** — policy documents cryptographically signed with Ed25519 and verified by editors
   before application
   - **Acceptance:** tampered policies are rejected automatically
3. **US-15.7.5.3** — policy distribution to support both push (WebSocket) and pull (HTTPS polling)
   models
   - **Acceptance:** diverse network topologies are accommodated
4. **US-15.7.5.4** — to verify that editors reject policies with invalid signatures
   - **Acceptance:** only authentic administrator-issued policies are applied

## F-15.7.6 AI Content Audit Trail

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.7.6.1 | server admin (P-22)     |          |              |
| US-15.7.6.2 | DevOps engineer (P-16)  |          |              |
| US-15.7.6.3 | creative director (P-2) |          |              |
| US-15.7.6.4 | engine tester (P-27)    |          |              |

1. **US-15.7.6.1** — an append-only audit log recording every AI content generation event (asset,
   model version, user, timestamp) with hash-chained entries
   - **Acceptance:** any tampering or deletion is detectable
2. **US-15.7.6.2** — audit logs replicable to a central server in enterprise configurations
   - **Acceptance:** all AI activity across the organization is collected for compliance review
3. **US-15.7.6.3** — to query the audit trail to see how much AI content generation the team is
   using
   - **Acceptance:** I can make informed decisions about AI adoption and review priorities
4. **US-15.7.6.4** — to verify that the hash chain across audit log segments is intact after log
   rotation
   - **Acceptance:** the tamper-detection guarantee holds across all log files

## F-15.7.7 AI Content Review Workflow

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.7.7.1 | creative director (P-2) |          |              |
| US-15.7.7.2 | artist (P-8)            |          |              |
| US-15.7.7.3 | designer (P-5)          |          |              |
| US-15.7.7.4 | engine tester (P-27)    |          |              |

1. **US-15.7.7.1** — assets with AI provenance tags routed through a mandatory review workflow
   before entering the production database
   - **Acceptance:** no AI content ships without human approval
2. **US-15.7.7.2** — to see the full provenance metadata and a visual diff highlighting AI-generated
   versus human-authored regions
   - **Acceptance:** I can evaluate exactly what was AI-generated during review
3. **US-15.7.7.3** — to configure the review workflow per project (all AI content, specific asset
   types, or auto-approve above a modification threshold)
   - **Acceptance:** review overhead matches the studio's risk tolerance
4. **US-15.7.7.4** — to verify that the mandatory review workflow blocks AI-tagged assets from the
   production database until approved
   - **Acceptance:** the review gate cannot be bypassed

## F-15.7.8 Provenance Metadata in Packaged Builds

| ID          | Persona                | Features | Requirements |
|-------------|------------------------|----------|--------------|
| US-15.7.8.1 | developer (P-15)       |          |              |
| US-15.7.8.2 | server admin (P-22)    |          |              |
| US-15.7.8.3 | DevOps engineer (P-16) |          |              |
| US-15.7.8.4 | engine tester (P-27)   |          |              |

1. **US-15.7.8.1** — runtime APIs to read AI provenance metadata for any loaded asset
   - **Acceptance:** the game can display "AI-generated content" labels in credits, settings, or
     in-game UI
2. **US-15.7.8.2** — only provenance flags and AI system identifiers retained in packaged builds
   (not full prompt data)
   - **Acceptance:** the metadata footprint is minimal in shipped games
3. **US-15.7.8.3** — the runtime API to provide raw provenance data that the game's UI layer formats
   per platform requirements
   - **Acceptance:** disclosure formats comply with each platform holder's certification rules
4. **US-15.7.8.4** — to verify that provenance metadata is queryable in packaged builds on all
   platforms
   - **Acceptance:** runtime disclosure features work correctly after cooking and packaging
