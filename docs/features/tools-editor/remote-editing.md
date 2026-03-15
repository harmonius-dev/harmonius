# 15.12 — Remote Editing & Collaboration

## Remote Rendering

### F-15.12.1 Remote Desktop Optimized Rendering

The editor renders its viewport to a compressed video stream (H.264 or H.265) for remote desktop
scenarios, with adaptive bitrate based on measured network bandwidth. Input events are forwarded
from the client with prediction to mask network latency. GPU-accelerated encoding on the server
side keeps encoding overhead below 2ms per frame. Enables developers to work from home on a
remote GPU server without shipping dedicated GPU hardware to each team member.

- **Requirements:** R-15.12.1
- **Dependencies:** F-15.1.1, F-15.1.2, F-2.1.1 (GPU Abstraction)
- **Platform notes:** On macOS, uses VideoToolbox for H.264/H.265 encoding. On Windows, uses
  NVENC, AMF, or Intel Quick Sync depending on GPU vendor. On Linux, uses VA-API or NVENC.

### F-15.12.2 Remote Editor Protocol

A custom protocol for streaming the editor UI and viewport over the network, designed specifically
for editor semantics rather than generic desktop mirroring. Viewport frames are encoded at high
quality and full frame rate; UI panels are encoded at lower quality with change-detection driven
updates; idle regions are not retransmitted. Reduces bandwidth by 60-80% compared to generic
RDP or VNC solutions while maintaining visual fidelity where it matters most.

- **Requirements:** R-15.12.2
- **Dependencies:** F-15.12.1, F-15.1.1
- **Platform notes:** The protocol runs over QUIC for low-latency, multiplexed transport. Falls
  back to TCP with TLS 1.3 when UDP is blocked by firewalls.

## Multi-User Collaboration

### F-15.12.3 CRDT-Based Real-Time Collaborative Editing

Multiple users connect to the same shared world simultaneously, each with their own viewport and
selection state. All edits synchronize in real time via Conflict-free Replicated Data Types
(CRDTs) — every asset type (scenes, logic graphs, data tables, terrain) has a CRDT
representation that merges concurrent edits without conflicts. Per-user undo stacks allow each
participant to undo their own changes without affecting others. Presence indicators show cursor
positions and selections of all connected users. A chat sidebar and voice channel are integrated
for coordination.

- **Requirements:** R-15.12.3
- **Dependencies:** F-15.12.7 (Collaboration Cloud Service), F-15.1.3 (Undo/Redo)
- **Platform notes:** CRDT operations are serialized as compact binary deltas. Direct peer-to-peer
  mode is supported on LAN with mDNS discovery; cloud relay for remote collaboration.

## Server Infrastructure

### F-15.12.4 Remote GPU Server Support

The editor runs headless on a remote GPU server (Linux or Windows), streaming the viewport and UI
to thin clients via the remote editor protocol (F-15.12.2). Input events are forwarded from the
client to the server with sub-frame latency targeting under 16ms round-trip on LAN. A single
multi-GPU server supports multiple concurrent sessions, with each session assigned to a dedicated
GPU. Session scheduling and GPU assignment are managed by an orchestrator service.

- **Requirements:** R-15.12.4
- **Dependencies:** F-15.12.1, F-15.12.2
- **Platform notes:** Headless mode requires EGL on Linux or headless Metal on macOS for GPU
  context creation without a display. On Windows, uses WDDM headless mode. Multi-GPU assignment
  uses Vulkan device groups on Linux and DXGI adapter enumeration on Windows.

## Session Management

### F-15.12.5 Session Handoff and Persistence

A remote editing session can be suspended and resumed later without losing state. Session state —
open panels, viewport camera positions, selection, undo history, and unsaved modifications — is
serialized to the server's local storage on suspend. A developer can start a session at the
office, suspend it, and resume from home on a different client device. Session resume restores
the editor to the exact visual and functional state at the time of suspension.

- **Requirements:** R-15.12.5
- **Dependencies:** F-15.12.4, F-15.1.1, F-15.1.3 (Undo/Redo)
- **Platform notes:** Session state files are stored in the server's project workspace directory.
  State serialization uses the same binary format as the editor's crash recovery system.

### F-15.12.6 Bandwidth Adaptation and Quality Tiers

Stream quality adjusts automatically based on continuously measured network conditions. High
bandwidth (office LAN, over 100 Mbps): near-lossless viewport encoding, full UI refresh rate.
Medium bandwidth (home broadband, 10-100 Mbps): lossy viewport at 60fps with moderate
compression, reduced UI refresh rate. Low bandwidth (mobile tethering, under 10 Mbps): viewport
at 30fps with aggressive compression, UI updates only on change. Users can override the automatic
tier selection and pin a specific quality level.

- **Requirements:** R-15.12.6
- **Dependencies:** F-15.12.1, F-15.12.2
- **Platform notes:** Bandwidth measurement uses QUIC transport metrics when available. Falls back
  to application-level throughput sampling over TCP.

## Cloud Service

### F-15.12.7 Collaboration Cloud Service

A centralized cloud service written in Rust that manages real-time CRDT synchronization,
session state, presence, and collaboration metadata. The service uses PostgreSQL for persistent
session/project/user data and S3-compatible object storage for CRDT document snapshots and
asset deltas. All CRDT operations flow through this service — clients push local operations,
the service merges and rebroadcasts to other participants. The service handles hundreds of
concurrent sessions with horizontal scaling behind a load balancer.

- **Requirements:** R-15.12.7
- **Dependencies:** F-15.12.3, F-1.8.4 (Async Network I/O)
- **Platform notes:** Service stack: Rust (tokio + axum), PostgreSQL for relational data
  (sessions, users, permissions, audit log), S3 for CRDT snapshots and binary deltas.
  Deployed as containers (Docker/Kubernetes). WebSocket transport for real-time sync;
  REST API for session management and administration.

### F-15.12.8 CRDT Document Model for Engine Assets

Each asset type defines a CRDT document schema mapping its structure to mergeable CRDT types.
Scene hierarchies use a tree CRDT (entity add/remove/reparent). Logic graphs use an operation
log CRDT (node add/remove/connect/disconnect). Data tables use a map CRDT (per-row, per-cell).
Terrain heightmaps use a last-writer-wins register per tile. The CRDT layer sits between the
asset editor and the persistence layer — editors read/write through CRDT accessors, which
handle merge automatically.

- **Requirements:** R-15.12.8
- **Dependencies:** F-15.12.3, F-12.7.1 (Binary Asset Format)
- **Platform notes:** Desktop only. CRDT document schemas are used exclusively by the editor
  collaboration system.

### F-15.12.9 Collaboration Access Control and Permissions

Per-project and per-asset access control managed through the collaboration cloud service.
Roles (viewer, editor, admin) determine who can edit, who can only observe, and who can
manage sessions. Asset-level locks allow exclusive editing of specific assets within a
collaborative session. Permissions are stored in PostgreSQL and enforced server-side — the
client cannot bypass access controls.

- **Requirements:** R-15.12.9
- **Dependencies:** F-15.12.7
- **Platform notes:** Authentication via OAuth2/OIDC (enterprise SSO). API keys for CI/CD
  integration.

## Communication

### F-15.12.10 Integrated Voice and Text Chat

Built-in voice chat (spatial and non-spatial modes) and text chat within collaborative sessions.
Users join voice channels per work group. Text chat supports threads, mentions, emoji reactions,
and inline asset/node references (click a reference to navigate to that asset in the editor).
Chat history is persisted in the collaboration cloud service (PostgreSQL) and searchable. Voice
uses Opus codec over the QUIC transport with echo cancellation and noise suppression.

- **Requirements:** R-15.12.10
- **Dependencies:** F-15.12.7, F-5.5.1 (Voice Chat Codec)
- **Platform notes:** Voice capture via platform audio APIs (CoreAudio on macOS, WASAPI on
  Windows, PipeWire on Linux).

### F-15.12.11 Work Groups and Isolated Workspaces

Collaborators organize into named work groups (e.g., "Level Design — Zone 3", "Shader Team",
"Boss AI"). Each group has its own voice channel, text thread, and optionally an isolated
workspace layer where edits are invisible to other groups until explicitly shared. Work groups
map to work items (tasks, tickets, branches) for traceability. Group membership is dynamic —
users join and leave without disrupting others.

- **Requirements:** R-15.12.11
- **Dependencies:** F-15.12.7, F-15.12.10
- **Platform notes:** Desktop only. Not available on mobile or console runtime.

### F-15.12.12 AI Agent Collaboration

AI agents (F-15.9.6) participate in collaborative sessions as virtual users. An AI agent can
be assigned to a work group, receive instructions via text chat, and execute editor operations
visible to the group. Other users see the AI's cursor, selections, and edits in real time.
AI agents can be instructed to work independently on a task ("populate this zone with
vegetation") while the human team works on other areas. Agent actions are tagged with AI
provenance (F-15.7.1).

- **Requirements:** R-15.12.12
- **Dependencies:** F-15.9.6 (Agent Automation), F-15.12.7, F-15.7.1 (Provenance)
- **Platform notes:** Desktop only. Not available on mobile or console runtime.

## Review and Comments

### F-15.12.13 Asset and Scene Comments

Attach comments to any asset, entity, node, property, or spatial location in the world.
Comments are threaded (reply chains), support mentions (@user), and display as non-intrusive
pins in the viewport or margin markers in editors. Comments are stored in the collaboration
cloud service and synced across all connected clients. Resolved comments are collapsed but
not deleted. Comments on logic graph nodes appear as annotations in the graph editor.

- **Requirements:** R-15.12.13
- **Dependencies:** F-15.12.7
- **Platform notes:** Desktop only. Not available on mobile or console runtime.

### F-15.12.14 Pull Request Review in Editor

View and review GitHub/GitLab pull requests entirely within the editor. The PR review panel
shows the list of changed assets with structural diffs (F-12.7.3) rendered in their native
visual editors — not text diffs. Reviewers add comments on specific nodes, properties, or
regions using the comment system (F-15.12.13). Approve/request-changes actions push status
back to the Git hosting service via API. Users never need to view diffs in a web browser.

- **Requirements:** R-15.12.14
- **Dependencies:** F-15.12.13, F-12.7.3 (Structural Diff), F-15.10.1 (Git Integration)
- **Platform notes:** Supports GitHub, GitLab, and Bitbucket APIs via configurable provider.
