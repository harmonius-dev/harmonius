# R-15.12 -- Remote Editing and Collaboration Requirements

## Requirements

1. **R-15.12.1** — The engine **SHALL** stream the editor viewport as compressed video with
   GPU-accelerated encoding below 2 ms per frame and adaptive bitrate.
   - **Rationale:** Low-latency streaming enables remote editing on thin clients.
   - **Verification:** Measure encoding latency and verify it stays below 2 ms at 1080p.

2. **R-15.12.2** — The engine **SHALL** use a custom protocol over QUIC with TCP/TLS fallback,
   reducing bandwidth 60-80% versus generic desktop mirroring.
   - **Rationale:** Editor-specific protocol avoids retransmitting idle UI regions.
   - **Verification:** Compare bandwidth to VNC for the same editing session and verify at least 60%
     reduction.

3. **R-15.12.3** — The engine **SHALL** support CRDT-based real-time collaborative editing with
   per-user undo stacks and cursor presence indicators.
   - **Rationale:** CRDTs enable conflict-free concurrent editing without locking.
   - **Verification:** Two users edit the same scene concurrently and verify changes merge without
     conflicts.

4. **R-15.12.4** — The engine **SHALL** run headless on remote GPU servers with multiple concurrent
   sessions, each assigned a dedicated GPU, targeting sub-16 ms round-trip on LAN.
   - **Rationale:** Shared GPU servers reduce per-developer hardware costs.
   - **Verification:** Launch two sessions on the same server and verify independent GPU assignment.

5. **R-15.12.5** — The engine **SHALL** support session suspend and resume with full state
   serialization, allowing session handoff between devices.
   - **Rationale:** Persistent sessions enable work-from-anywhere workflows.
   - **Verification:** Suspend a session, resume on a different client, and verify all state
     restores.

6. **R-15.12.6** — The engine **SHALL** automatically adjust stream quality based on measured
   bandwidth with user-overridable quality tiers (high, medium, low).
   - **Rationale:** Adaptive quality ensures usability across network conditions.
   - **Verification:** Throttle bandwidth to 10 Mbps and verify the system downgrades to medium
     tier.

7. **R-15.12.7** — The engine **SHALL** provide a centralized collaboration service in Rust with
   PostgreSQL and S3 for CRDT sync, session management, and presence.
   - **Rationale:** A single backend simplifies deployment and scales horizontally.
   - **Verification:** Deploy the service and verify 10 concurrent editing sessions synchronize
     correctly.

8. **R-15.12.8** — The engine **SHALL** define CRDT document schemas per asset type (tree, op-log,
   map, LWW register) with transparent accessor integration.
   - **Rationale:** Type-appropriate CRDTs ensure correct merge semantics for each asset format.
   - **Verification:** Concurrently edit a scene hierarchy and a logic graph; verify both merge
     correctly.

9. **R-15.12.9** — The engine **SHALL** enforce server-side role-based access control (viewer,
   editor, admin) with asset-level exclusive locks.
   - **Rationale:** Access control prevents unauthorized edits in enterprise environments.
   - **Verification:** Assign viewer role and verify edit attempts are rejected server-side.

10. **R-15.12.10** — The engine **SHALL** provide built-in voice and text chat with spatial audio,
    threads, mentions, and inline asset references.
    - **Rationale:** Integrated communication reduces context switching during collaboration.
    - **Verification:** Send a text message with an asset reference and verify clicking it navigates
      to that asset.

11. **R-15.12.11** — The engine **SHALL** support named work groups with isolated workspace layers,
    voice channels, and work-item traceability.
    - **Rationale:** Isolation prevents unfinished edits from affecting other teams.
    - **Verification:** Create a work group, make edits, and verify they are invisible to other
      groups until shared.

12. **R-15.12.12** — The engine **SHALL** support AI agents as virtual collaborators with provenance
    tagging on all actions.
    - **Rationale:** Automated editing tasks benefit from collaborative infrastructure.
    - **Verification:** Assign an AI agent to place vegetation and verify its edits carry provenance
      tags.

13. **R-15.12.13** — The engine **SHALL** support threaded comments on entities, nodes, properties,
    and spatial locations with viewport pins and graph annotations.
    - **Rationale:** Contextual feedback eliminates ambiguity in review workflows.
    - **Verification:** Attach a comment to a node and verify it appears as an annotation in the
      graph editor.

14. **R-15.12.14** — The engine **SHALL** support in-editor pull request review with structural
    diffs for all asset types and per-node comment integration.
    - **Rationale:** Structural PR review is more effective than text diffs for visual assets.
    - **Verification:** Open a PR diff for a logic graph and verify added/removed nodes are visually
      highlighted.
