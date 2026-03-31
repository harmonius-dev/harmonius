# User Stories -- 15.7 AI Governance and Provenance

## Stories

| ID          | Persona                 |
|-------------|-------------------------|
| US-15.7.1.1 | engine developer (P-26) |
| US-15.7.1.2 | technical artist (P-13) |
| US-15.7.2.1 | technical artist (P-13) |
| US-15.7.2.2 | engine developer (P-26) |
| US-15.7.3.1 | game designer (P-5)     |
| US-15.7.3.2 | engine developer (P-26) |
| US-15.7.4.1 | game designer (P-5)     |
| US-15.7.4.2 | engine developer (P-26) |
| US-15.7.5.1 | build engineer (P-16)   |
| US-15.7.5.2 | engine developer (P-26) |
| US-15.7.6.1 | engine developer (P-26) |
| US-15.7.6.2 | build engineer (P-16)   |
| US-15.7.7.1 | technical artist (P-13) |
| US-15.7.7.2 | game designer (P-5)     |
| US-15.7.8.1 | engine developer (P-26) |
| US-15.7.8.2 | game designer (P-5)     |

1. **US-15.7.1.1** — **As a** engine developer (P-26), **I want** every AI-generated asset to
   receive a persistent provenance tag with model version and prompt hash, **so that** origin is
   always traceable.

2. **US-15.7.1.2** — **As a** technical artist (P-13), **I want** provenance tags to survive all
   pipeline stages from import through packaging, **so that** governance metadata is never lost.

3. **US-15.7.2.1** — **As a** technical artist (P-13), **I want** human modification tracking at
   coarse granularity per vertex group, tile, or node, **so that** the system knows when all AI
   content has been replaced.

4. **US-15.7.2.2** — **As a** engine developer (P-26), **I want** modification metadata stored as
   compact bitmasks, **so that** tracking adds minimal serialization overhead.

5. **US-15.7.3.1** — **As a** game designer (P-5), **I want** a global toggle to disable all
   generative AI features while leaving deterministic AI unaffected, **so that** my studio can
   comply with AI-free policies.

6. **US-15.7.3.2** — **As a** engine developer (P-26), **I want** the AI toggle evaluated at
   subsystem initialization, **so that** disabled features are fully torn down.

7. **US-15.7.4.1** — **As a** game designer (P-5), **I want** an independent toggle for AI editor
   assistance separate from generative AI content, **so that** I can use voice control without
   enabling content generation.

8. **US-15.7.4.2** — **As a** engine developer (P-26), **I want** both AI toggles to operate
   independently, **so that** studios have fine-grained control over AI capabilities.

9. **US-15.7.5.1** — **As a** build engineer (P-16), **I want** remote admin API to push AI policy
   updates to all editors organization-wide, **so that** compliance changes deploy instantly.

10. **US-15.7.5.2** — **As a** engine developer (P-26), **I want** policy documents
    cryptographically signed with Ed25519, **so that** tampered policies are rejected.

11. **US-15.7.6.1** — **As a** engine developer (P-26), **I want** an append-only, hash-chained
    audit log recording every AI generation event, **so that** tampering is detectable.

12. **US-15.7.6.2** — **As a** build engineer (P-16), **I want** audit logs replicable to a central
    server, **so that** compliance reviews cover the entire organization.

13. **US-15.7.7.1** — **As a** technical artist (P-13), **I want** a mandatory review workflow for
    AI-generated assets with visual diff of AI versus human regions, **so that** AI content is
    approved before production use.

14. **US-15.7.7.2** — **As a** game designer (P-5), **I want** configurable auto-approval thresholds
    based on human modification percentage, **so that** heavily edited assets skip manual review.

15. **US-15.7.8.1** — **As a** engine developer (P-26), **I want** provenance metadata to survive
    packaging and be queryable in shipped builds, **so that** runtime can display AI content labels.

16. **US-15.7.8.2** — **As a** game designer (P-5), **I want** the packaged metadata footprint
    minimized to flags and identifiers only, **so that** prompt data is not shipped.
