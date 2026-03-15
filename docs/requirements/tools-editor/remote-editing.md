# R-15.12 -- Remote Editing & Collaboration Requirements

## R-15.12.1 Remote Desktop Optimized Rendering

The engine **SHALL** render the editor viewport to a compressed video stream (H.264 or H.265) with
adaptive bitrate based on measured network bandwidth, GPU-accelerated encoding with overhead below
2ms per frame, and input event forwarding with prediction to mask network latency.

- **Derived from:** [F-15.12.1](../../features/tools-editor/remote-editing.md)
- **Rationale:** Remote rendering enables developers to work from any location using a remote GPU
  server, eliminating the need to ship dedicated GPU hardware to each team member.
- **Verification:** Connect to a remote editor session over a 50 Mbps link. Verify the viewport
  streams at the configured resolution and frame rate. Measure encoding time per frame and verify
  it is below 2ms. Verify input prediction masks latency up to 50ms without visible artifacts.
  Verify platform-appropriate hardware encoders are used (VideoToolbox, NVENC, VA-API).

## R-15.12.2 Remote Editor Protocol

The engine **SHALL** implement a custom protocol over QUIC (with TCP+TLS 1.3 fallback) that encodes
viewport frames at high quality and full frame rate, UI panels at lower quality with change-
detection driven updates, and skips idle regions, achieving 60-80% bandwidth reduction compared to
generic RDP or VNC solutions.

- **Derived from:** [F-15.12.2](../../features/tools-editor/remote-editing.md)
- **Rationale:** A purpose-built protocol optimized for editor semantics delivers better visual
  fidelity and lower bandwidth than generic remote desktop solutions.
- **Verification:** Measure total bandwidth during an active editing session and compare against
  a generic VNC connection of the same session. Verify bandwidth reduction of at least 60%.
  Verify viewport frames render at full frame rate while idle UI panels do not retransmit.
  Verify fallback to TCP+TLS 1.3 when UDP is blocked.

## R-15.12.3 CRDT-Based Real-Time Collaborative Editing

The engine **SHALL** enable multiple users to edit the same shared world simultaneously with
independent viewports and selection states, synchronizing all edits in real time via CRDTs that
merge concurrent edits without conflicts, with per-user undo stacks, presence indicators, and
integrated chat and voice channels, backed by a Rust collaboration service using PostgreSQL for
persistent data and S3-compatible storage for CRDT snapshots.

- **Derived from:** [F-15.12.3](../../features/tools-editor/remote-editing.md)
- **Rationale:** CRDT-based collaboration eliminates merge conflicts during concurrent editing,
  enabling real-time multi-user workflows that traditional lock-based systems cannot support.
- **Verification:** Connect three users to the same shared world. Have all three edit different
  entities concurrently and verify all edits converge to the same state on all clients. Have two
  users edit the same entity concurrently and verify the CRDT merges both edits without conflict.
  Verify per-user undo only reverts the issuing user's changes. Verify presence indicators show
  all connected users' cursor positions. Verify the collaboration service persists session data
  to PostgreSQL and CRDT snapshots to S3.

## R-15.12.4 Remote GPU Server Support

The engine **SHALL** run headless on a remote GPU server, streaming viewport and UI to thin clients
via the remote editor protocol with sub-frame latency targeting under 16ms round-trip on LAN, with
multi-GPU support assigning each concurrent session to a dedicated GPU managed by an orchestrator
service.

- **Derived from:** [F-15.12.4](../../features/tools-editor/remote-editing.md)
- **Rationale:** Centralized GPU servers reduce hardware costs and simplify management compared to
  distributing dedicated GPUs to each developer workstation.
- **Verification:** Launch the editor in headless mode on a multi-GPU server. Connect two thin
  clients and verify each session is assigned to a separate GPU. Measure input-to-display
  round-trip latency on LAN and verify it is under 16ms. Verify headless GPU context creation
  works on all platforms (EGL on Linux, headless Metal on macOS, WDDM on Windows).

## R-15.12.5 Session Handoff and Persistence

The engine **SHALL** serialize full session state (open panels, viewport cameras, selection, undo
history, unsaved modifications) to server-local storage on suspend and restore the editor to the
exact visual and functional state on resume, enabling session handoff across client devices.

- **Derived from:** [F-15.12.5](../../features/tools-editor/remote-editing.md)
- **Rationale:** Session persistence enables developers to move between locations and devices
  without losing context, supporting hybrid office/remote workflows.
- **Verification:** Open several panels, position the viewport camera, select entities, and make
  unsaved modifications. Suspend the session. Resume on a different client device and verify all
  panels, camera positions, selections, undo history, and unsaved modifications are restored
  exactly. Verify session state serialization uses the same format as crash recovery.

## R-15.12.6 Bandwidth Adaptation and Quality Tiers

The engine **SHALL** automatically adjust stream quality based on continuously measured network
conditions across three tiers -- high (over 100 Mbps, near-lossless), medium (10-100 Mbps, lossy
60fps), low (under 10 Mbps, 30fps with aggressive compression) -- with manual override to pin a
specific quality level.

- **Derived from:** [F-15.12.6](../../features/tools-editor/remote-editing.md)
- **Rationale:** Automatic quality adaptation ensures a usable editing experience across varying
  network conditions without manual configuration.
- **Verification:** Simulate bandwidth at 150 Mbps, 50 Mbps, and 5 Mbps. Verify the stream
  automatically selects the high, medium, and low tiers respectively. Verify frame rate and
  compression level match the tier specification. Pin a specific quality level and verify the
  stream does not change tiers regardless of bandwidth variation.

## R-15.12.7 Collaboration Cloud Service

The engine **SHALL** provide a centralized cloud service written in Rust (tokio + axum) that manages
real-time CRDT synchronization, session state, presence, and collaboration metadata, using
PostgreSQL for persistent relational data and S3-compatible object storage for CRDT snapshots and
asset deltas, supporting hundreds of concurrent sessions with horizontal scaling.

- **Derived from:** [F-15.12.7](../../features/tools-editor/remote-editing.md)
- **Rationale:** A dedicated collaboration service decouples real-time synchronization from the
  editor process, enabling horizontal scaling and persistent state across client reconnections.
- **Verification:** Deploy the service behind a load balancer with two replicas. Connect 100
  concurrent sessions and verify CRDT operations synchronize correctly across all participants.
  Verify session and user data persists in PostgreSQL. Verify CRDT snapshots are stored in S3.
  Verify WebSocket transport for real-time sync and REST API for administration. Verify
  container deployment via Docker/Kubernetes.

## R-15.12.8 CRDT Document Model for Engine Assets

The engine **SHALL** define a CRDT document schema for each asset type -- tree CRDT for scene
hierarchies (entity add/remove/reparent), operation log CRDT for logic graphs (node
add/remove/connect/disconnect), map CRDT for data tables (per-row, per-cell), and last-writer-wins
register per tile for terrain heightmaps -- with editors reading and writing through CRDT accessors.

- **Derived from:** [F-15.12.8](../../features/tools-editor/remote-editing.md)
- **Rationale:** Asset-specific CRDT schemas preserve semantic intent during concurrent edits,
  producing correct merges that generic text-based CRDTs cannot achieve.
- **Verification:** Concurrently add and reparent entities in a scene hierarchy from two clients
  and verify the tree CRDT produces a valid hierarchy. Concurrently add and connect nodes in a
  logic graph and verify the operation log CRDT preserves both additions. Concurrently edit
  different cells in a data table and verify the map CRDT merges without data loss. Edit
  adjacent terrain tiles concurrently and verify last-writer-wins resolves per tile.

## R-15.12.9 Collaboration Access Control and Permissions

The engine **SHALL** enforce server-side per-project and per-asset access control with role-based
permissions (viewer, editor, admin) stored in PostgreSQL, asset-level exclusive locks within
collaborative sessions, and authentication via OAuth2/OIDC for enterprise SSO.

- **Derived from:** [F-15.12.9](../../features/tools-editor/remote-editing.md)
- **Rationale:** Server-side access control prevents unauthorized edits and ensures asset
  integrity in multi-user environments where client-side enforcement is insufficient.
- **Verification:** Assign a user the viewer role and verify they can observe but not edit.
  Assign the editor role and verify edit access. Lock an asset and verify other editors cannot
  modify it until unlocked. Verify permissions are enforced server-side by attempting to bypass
  via direct API calls. Verify OAuth2/OIDC authentication and API key access for CI/CD.

## R-15.12.10 Integrated Voice and Text Chat

The engine **SHALL** provide built-in voice chat (Opus codec over QUIC with echo cancellation and
noise suppression) and text chat (threaded, with mentions, reactions, and inline asset references)
within collaborative sessions, with chat history persisted in PostgreSQL and searchable.

- **Derived from:** [F-15.12.10](../../features/tools-editor/remote-editing.md)
- **Rationale:** Integrated communication eliminates the need for external tools, and inline asset
  references enable contextual coordination directly tied to the work being discussed.
- **Verification:** Join a voice channel and verify audio quality with echo cancellation and noise
  suppression active. Send a text message with an @mention and verify the mentioned user
  receives a notification. Insert an inline asset reference in chat, click it, and verify the
  editor navigates to that asset. Search chat history and verify results include messages from
  prior sessions.

## R-15.12.11 Work Groups and Isolated Workspaces

The engine **SHALL** support named work groups with dedicated voice channels, text threads, and
optional isolated workspace layers where edits are invisible to other groups until explicitly
shared, with dynamic membership and traceability to work items.

- **Derived from:** [F-15.12.11](../../features/tools-editor/remote-editing.md)
- **Rationale:** Work groups enable parallel team workflows on separate areas of a project without
  interference, while isolated workspaces prevent unfinished work from affecting other teams.
- **Verification:** Create two work groups. Verify each group has independent voice and text
  channels. Enable isolated workspaces and make edits in group A. Verify group B does not see
  group A's edits until explicitly shared. Verify dynamic membership by joining and leaving
  groups without disrupting other members. Verify groups map to work items for traceability.

## R-15.12.12 AI Agent Collaboration

The engine **SHALL** enable AI agents to participate in collaborative sessions as virtual users,
receiving instructions via text chat, executing editor operations visible to the group with real-
time cursor and selection indicators, and tagging all actions with AI provenance metadata.

- **Derived from:** [F-15.12.12](../../features/tools-editor/remote-editing.md)
- **Rationale:** AI agents as collaborative participants enable automated content generation
  tasks to run alongside human editing without separate tooling or workflows.
- **Verification:** Add an AI agent to a collaborative session. Send a task instruction via text
  chat and verify the agent executes the requested editor operations. Verify other users see
  the agent's cursor, selections, and edits in real time. Verify all agent actions carry AI
  provenance metadata. Verify the agent can work independently while humans edit other areas.

## Non-Functional Requirements

### R-15.12.NF1 Collaboration Latency

CRDT operations **SHALL** propagate between connected clients within 100ms on a LAN (under 1ms
round-trip) and within 500ms over a WAN connection with up to 100ms round-trip latency. Presence
indicator updates (cursor position, selection changes) **SHALL** propagate within 200ms under the
same WAN conditions. The collaboration cloud service **SHALL** process and rebroadcast a CRDT
operation within 10ms of receipt at the server. End-to-end latency from one client's edit to
another client's screen update **SHALL NOT** exceed 1 second under any supported network condition.

- **Derived from:** F-15.12.3 (CRDT Collaborative Editing), F-15.12.7 (Collaboration Cloud
  Service).
- **Rationale:** Real-time collaboration requires low-latency synchronization so that users
  perceive concurrent edits as instantaneous. High latency causes confusion about the current state
  and increases the risk of conflicting spatial edits.
- **Verification:** Connect two clients over a simulated 100ms RTT WAN link. Have client A edit an
  entity and measure the time until client B's viewport reflects the change; assert it is under
  500ms. Repeat with 50 concurrent users on the same session and assert the 500ms threshold holds.
  Measure server-side CRDT processing time and assert it is under 10ms per operation.

### R-15.12.NF2 Remote Editing Input Latency

For remote editing sessions (F-15.12.1, F-15.12.2), the end-to-end latency from client input event
to visible viewport update **SHALL** be under 50ms on LAN (under 1ms RTT) and under 150ms on a
broadband connection (30ms RTT). Input prediction **SHALL** mask latency up to 100ms without visible
correction artifacts for translation and rotation gizmo operations.

- **Derived from:** F-15.12.1 (Remote Desktop Rendering), F-15.12.2 (Remote Editor Protocol),
  F-15.12.4 (Remote GPU Server).
- **Rationale:** Remote editing must feel responsive enough for precision spatial work. Visible
  input lag or prediction correction jitter makes gizmo manipulation imprecise and frustrating.
- **Verification:** Measure input-to-photon latency on a LAN remote session by timing from key
  press to viewport frame containing the result; assert under 50ms. Simulate 30ms RTT and repeat;
  assert under 150ms. Perform a continuous gizmo drag over 100ms simulated latency and assert no
  visible correction snaps in the recorded frame sequence.

## R-15.12.13 Asset and Scene Comments

The engine **SHALL** support threaded comments attached to any asset, entity, node, property, or
spatial location, with @mention notifications, viewport pins and editor margin markers, storage in
the collaboration cloud service, and real-time sync across connected clients.

- **Derived from:** [F-15.12.13](../../features/tools-editor/remote-editing.md)
- **Rationale:** Contextual comments anchored to specific assets and locations enable precise
  feedback that is difficult to convey through external communication tools.
- **Verification:** Attach a comment to a scene entity. Verify the comment appears as a pin in
  the viewport. Reply to the comment to create a thread. Add an @mention and verify the
  mentioned user receives a notification. Resolve the comment and verify it collapses but
  remains accessible. Verify comments sync to all connected clients in real time.

## R-15.12.14 Pull Request Review in Editor

The engine **SHALL** display pull request changed assets with structural diffs rendered in their
native visual editors, support adding review comments on specific nodes, properties, or regions via
the comment system, and push approve/request-changes status back to the Git hosting service API.

- **Derived from:** [F-15.12.14](../../features/tools-editor/remote-editing.md)
- **Rationale:** Visual structural diffs in native editors provide far more useful review context
  than text diffs for binary and structured assets, enabling effective code review without
  leaving the editor.
- **Verification:** Open a pull request containing modified logic graph and material assets.
  Verify the PR review panel shows structural diffs in the graph and material editors. Add a
  review comment on a specific node and verify it appears via the comment system. Approve the
  PR from the editor and verify the approval status is pushed to the Git hosting service API.
  Verify support for GitHub, GitLab, and Bitbucket.
