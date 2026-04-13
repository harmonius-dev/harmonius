# R-15.7 -- AI Governance and Provenance Requirements

## Requirements

1. **R-15.7.1** — The engine **SHALL** attach persistent provenance tags (AI system ID, model
   version, timestamp, prompt hash) to every AI-generated or AI-modified asset, surviving all
   pipeline stages.
   - **Rationale:** Provenance tracking is required for IP auditing and regulatory compliance.
   - **Verification:** Generate an asset via AI, cook and package it, and verify the provenance tag
     is present in the packaged output.

2. **R-15.7.2** — The engine **SHALL** track human modification status at coarse granularity (vertex
   group, tile, node) using compact bitmasks under 1 KB per asset.
   - **Rationale:** Modification tracking determines when AI content has been fully replaced by
     human work.
   - **Verification:** Modify 50% of an AI-generated mesh, verify the bitmask reports the correct
     modified percentage.

3. **R-15.7.3** — The engine **SHALL** provide a global toggle that disables all generative AI
   features while leaving deterministic AI systems unaffected.
   - **Rationale:** Studios may need to operate without generative AI for policy or legal reasons.
   - **Verification:** Disable the toggle, verify AI content generation UI is hidden and APIs return
     errors, while pathfinding and behavior trees still function.

4. **R-15.7.4** — The engine **SHALL** provide an independent toggle for AI editor assistance that
   operates separately from the generative AI toggle.
   - **Rationale:** Fine-grained control lets studios enable voice control without enabling content
     generation.
   - **Verification:** Enable assistance, disable generation, and verify voice commands work but
     content generation is unavailable.

5. **R-15.7.5** — The engine **SHALL** support remote administration of AI toggles and provenance
   policies via an authenticated API with Ed25519-signed policy documents.
   - **Rationale:** Enterprise environments require centralized policy management.
   - **Verification:** Push a signed policy update and verify all connected editors apply it. Push
     an unsigned policy and verify it is rejected.

6. **R-15.7.6** — The engine **SHALL** maintain an append-only, hash-chained audit log for all AI
   generation events, replicable to a central server.
   - **Rationale:** Hash-chained logs ensure tamper-evident compliance records.
   - **Verification:** Generate 10 assets, verify the audit log contains 10 entries with valid hash
     chains.

7. **R-15.7.7** — The engine **SHALL** provide a configurable review workflow for AI-generated
   assets with visual diff and auto-approval thresholds.
   - **Rationale:** Mandatory review ensures AI content quality; auto-approval reduces overhead for
     heavily edited assets.
   - **Verification:** Submit an AI asset with 90% human modification and verify auto-approval when
     threshold is set to 80%.

8. **R-15.7.8** — The engine **SHALL** preserve provenance metadata in packaged builds with a
   minimal footprint (flags and identifiers only), queryable via runtime APIs.
   - **Rationale:** Shipped builds may need to display AI content labels per platform or regulatory
     requirements.
   - **Verification:** Package a build, query provenance at runtime, and verify the correct AI flag
     is returned.
