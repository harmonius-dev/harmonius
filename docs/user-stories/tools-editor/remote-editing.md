# User Stories: Remote Editing and Collaboration

## F-15.12.1 Remote Desktop Optimized Rendering

## US-15.12.1.1 Developer Works Remotely

**As a** developer (P-15), **I want** the editor viewport rendered to a compressed video stream
(H.264/H.265) with adaptive bitrate, **so that** I can work from home on a remote GPU server without
dedicated GPU hardware at my desk.

## US-15.12.1.2 Artist Gets Low-Latency Input

**As an** artist (P-8), **I want** input events forwarded from the client with prediction to mask
network latency, **so that** gizmo manipulation and viewport navigation feel responsive over remote
connections.

## US-15.12.1.3 Engine Dev Minimizes Encoding Overhead

**As an** engine developer (P-26), **I want** GPU-accelerated encoding keeping overhead below 2ms
per frame, **so that** remote rendering does not significantly reduce frame rate.

## US-15.12.1.4 Engine Tester Validates Platform Encoders

**As an** engine tester (P-27), **I want** to verify that encoding uses platform-native APIs
(VideoToolbox on macOS, NVENC/AMF on Windows, VA-API on Linux), **so that** hardware acceleration is
utilized on each platform.

## F-15.12.2 Remote Editor Protocol

## US-15.12.2.1 DevOps Reduces Bandwidth Usage

**As a** DevOps engineer (P-16), **I want** a custom protocol that encodes viewport frames at high
quality and UI panels with change-detection updates, **so that** bandwidth is reduced 60-80%
compared to generic RDP or VNC.

## US-15.12.2.2 Developer Uses QUIC Transport

**As a** developer (P-15), **I want** the protocol to run over QUIC for low-latency multiplexed
transport with TCP+TLS 1.3 fallback, **so that** remote editing works reliably even when UDP is
blocked.

## US-15.12.2.3 Server Admin Configures Protocol

**As a** server admin (P-22), **I want** the remote editor protocol to support configurable quality
and bandwidth settings, **so that** I can tune performance for different network environments.

## US-15.12.2.4 Engine Tester Validates Fallback

**As an** engine tester (P-27), **I want** to verify that the protocol falls back gracefully from
QUIC to TCP when UDP is blocked, **so that** corporate firewalls do not prevent remote editing.

## F-15.12.3 CRDT-Based Real-Time Collaborative Editing

## US-15.12.3.1 Designer Edits Simultaneously with Team

**As a** designer (P-5), **I want** multiple users to connect to the same shared world with
independent viewports and selection states, syncing edits in real time via CRDTs, **so that** the
team can work on the same level without file locking.

## US-15.12.3.2 Artist Undoes Own Changes Only

**As an** artist (P-8), **I want** per-user undo stacks so I can undo my changes without affecting
other collaborators, **so that** parallel editing does not create undo conflicts.

## US-15.12.3.3 Creative Director Sees All Collaborators

**As a** creative director (P-2), **I want** presence indicators showing cursor positions and
selections of all connected users, **so that** I can see who is working where during collaborative
sessions.

## US-15.12.3.4 Engine Tester Validates CRDT Merge

**As an** engine tester (P-27), **I want** to verify that concurrent edits merge without conflicts
via CRDTs, **so that** collaborative editing never loses data or produces inconsistent states.

## F-15.12.4 Remote GPU Server Support

## US-15.12.4.1 DevOps Provisions Multi-User Server

**As a** DevOps engineer (P-16), **I want** the editor to run headless on a multi-GPU server with
each session assigned to a dedicated GPU, **so that** multiple developers share a single powerful
server.

## US-15.12.4.2 Developer Gets Sub-Frame Latency

**As a** developer (P-15), **I want** input events forwarded with sub-frame latency targeting under
16ms round-trip on LAN, **so that** editing feels like working locally.

## US-15.12.4.3 Server Admin Manages Sessions

**As a** server admin (P-22), **I want** session scheduling and GPU assignment managed by an
orchestrator service, **so that** GPU resources are allocated efficiently across users.

## US-15.12.4.4 Engine Tester Validates Headless Mode

**As an** engine tester (P-27), **I want** to verify that headless mode works with EGL on Linux and
headless Metal on macOS for GPU context creation without a display, **so that** server deployment
does not require display hardware.

## F-15.12.5 Session Handoff and Persistence

## US-15.12.5.1 Developer Suspends and Resumes Session

**As a** developer (P-15), **I want** to suspend a remote editing session and resume it later from a
different client with all panels, camera positions, selections, and unsaved work intact, **so that**
I can move between locations without losing context.

## US-15.12.5.2 Designer Recovers from Disconnect

**As a** designer (P-5), **I want** session state serialized to server storage on disconnect,
**so that** accidental network drops do not lose my editing state.

## US-15.12.5.3 Engine Dev Uses Binary Serialization

**As an** engine developer (P-26), **I want** session state to use the same binary format as crash
recovery, **so that** serialization is fast and complete.

## US-15.12.5.4 Engine Tester Validates State Restore

**As an** engine tester (P-27), **I want** to verify that session resume restores the editor to the
exact visual and functional state at suspension, **so that** handoff is seamless.

## F-15.12.6 Bandwidth Adaptation and Quality Tiers

## US-15.12.6.1 Developer Gets Auto-Adapted Quality

**As a** developer (P-15), **I want** stream quality to adjust automatically based on network
conditions (high/medium/low bandwidth tiers), **so that** the editing experience is optimized for my
current connection.

## US-15.12.6.2 Artist Pins Quality Level

**As an** artist (P-8), **I want** to override automatic tier selection and pin a specific quality
level, **so that** I get consistent visual fidelity when evaluating materials and lighting.

## US-15.12.6.3 Server Admin Configures Tier Thresholds

**As a** server admin (P-22), **I want** configurable bandwidth thresholds for each quality tier,
**so that** I can tune the adaptation for my organization's network infrastructure.

## US-15.12.6.4 Engine Tester Validates Graceful Degradation

**As an** engine tester (P-27), **I want** to verify that quality degrades smoothly as bandwidth
drops without visual artifacts or input lag spikes, **so that** remote editing remains usable on
poor connections.

## F-15.12.7 Collaboration Cloud Service

## US-15.12.7.1 Engine Dev Builds Service in Rust

**As an** engine developer (P-26), **I want** a centralized cloud service in Rust (IoReactor)
managing CRDT synchronization, sessions, presence, and collaboration metadata, **so that** the
backend is performant and type-safe.

## US-15.12.7.2 Server Admin Scales Horizontally

**As a** server admin (P-22), **I want** the service to handle hundreds of concurrent sessions with
horizontal scaling behind a load balancer, **so that** the collaboration infrastructure scales with
team size.

## US-15.12.7.3 DevOps Deploys as Containers

**As a** DevOps engineer (P-16), **I want** the service deployed as Docker/Kubernetes containers
with PostgreSQL and S3, **so that** deployment follows standard cloud infrastructure patterns.

## US-15.12.7.4 Engine Tester Validates WebSocket Sync

**As an** engine tester (P-27), **I want** to verify that WebSocket transport for real-time sync
handles concurrent operations correctly, **so that** CRDT operations are never lost or reordered.

## F-15.12.8 CRDT Document Model for Engine Assets

## US-15.12.8.1 Engine Dev Defines CRDT Schemas

**As an** engine developer (P-26), **I want** each asset type to define a CRDT document schema (tree
CRDT for scenes, operation log for graphs, map CRDT for data tables, LWW register for heightmaps),
**so that** all asset types merge correctly during collaboration.

## US-15.12.8.2 Designer Edits Through CRDT Layer

**As a** designer (P-5), **I want** editors to read/write through CRDT accessors that handle merge
automatically, **so that** I do not need to think about synchronization during editing.

## US-15.12.8.3 Tech Artist Edits Terrain Collaboratively

**As a** tech artist (P-13), **I want** terrain heightmaps to use last-writer-wins registers per
tile for collaborative editing, **so that** terrain sculpting resolves conflicts sensibly.

## US-15.12.8.4 Engine Tester Validates Schema Coverage

**As an** engine tester (P-27), **I want** to verify that every editable asset type has a CRDT
document schema, **so that** no asset type is excluded from collaborative editing.

## F-15.12.9 Collaboration Access Control and Permissions

## US-15.12.9.1 Server Admin Assigns Roles

**As a** server admin (P-22), **I want** per-project and per-asset access control with roles
(viewer, editor, admin) enforced server-side, **so that** only authorized users can edit sensitive
assets.

## US-15.12.9.2 Developer Locks Assets for Exclusive Edit

**As a** developer (P-15), **I want** asset-level locks within collaborative sessions, **so that** I
can claim exclusive editing rights on critical assets during sensitive changes.

## US-15.12.9.3 DevOps Uses OAuth2 Authentication

**As a** DevOps engineer (P-16), **I want** authentication via OAuth2/OIDC for enterprise SSO and
API keys for CI/CD integration, **so that** access control integrates with our identity provider.

## US-15.12.9.4 Engine Tester Validates Server-Side Enforcement

**As an** engine tester (P-27), **I want** to verify that permissions are enforced server-side and
cannot be bypassed by the client, **so that** access controls are security-meaningful.

## F-15.12.10 Integrated Voice and Text Chat

## US-15.12.10.1 Designer Communicates During Collaboration

**As a** designer (P-5), **I want** built-in voice and text chat within collaborative sessions,
**so that** I can coordinate with teammates without switching to a separate communication tool.

## US-15.12.10.2 Artist References Assets in Chat

**As an** artist (P-8), **I want** text chat with inline asset and node references that navigate to
the referenced item on click, **so that** team discussions are linked to specific content.

## US-15.12.10.3 Creative Director Joins Voice Channel

**As a** creative director (P-2), **I want** to join voice channels per work group, **so that** I
can provide real-time feedback to different teams during art reviews.

## US-15.12.10.4 Engine Tester Validates Voice Quality

**As an** engine tester (P-27), **I want** to verify that voice chat using Opus codec over QUIC with
echo cancellation and noise suppression produces clear audio, **so that** team communication is
reliable.

## F-15.12.11 Work Groups and Isolated Workspaces

## US-15.12.11.1 Designer Joins Named Work Group

**As a** designer (P-5), **I want** to join named work groups (e.g., "Level Design - Zone 3") with
their own voice channel, text thread, and optionally isolated workspace layer, **so that** my edits
are invisible to other groups until explicitly shared.

## US-15.12.11.2 DevOps Maps Groups to Work Items

**As a** DevOps engineer (P-16), **I want** work groups mapped to tasks, tickets, and branches,
**so that** collaboration is traceable to project management items.

## US-15.12.11.3 Creative Director Reviews Group Work

**As a** creative director (P-2), **I want** to view isolated workspace layers from each group,
**so that** I can review work-in-progress from different teams before it merges.

## US-15.12.11.4 Engine Tester Validates Group Isolation

**As an** engine tester (P-27), **I want** to verify that edits in an isolated workspace layer are
invisible to other groups until shared, **so that** group isolation works correctly.

## F-15.12.12 AI Agent Collaboration

## US-15.12.12.1 Designer Assigns AI to Work Group

**As a** designer (P-5), **I want** AI agents to participate in collaborative sessions as virtual
users assigned to work groups, **so that** I can instruct an AI to populate a zone with vegetation
while I work on another area.

## US-15.12.12.2 Tech Artist Monitors AI Agent Work

**As a** tech artist (P-13), **I want** to see the AI agent's cursor, selections, and edits in real
time, **so that** I can supervise and correct AI-driven changes as they happen.

## US-15.12.12.3 Developer Tags AI Provenance

**As a** developer (P-15), **I want** agent actions tagged with AI provenance, **so that**
AI-created content is distinguished from human-created content in the collaboration history.

## US-15.12.12.4 Engine Tester Validates Agent Visibility

**As an** engine tester (P-27), **I want** to verify that AI agents are visible to all group members
as virtual users with real-time presence, **so that** human collaborators are always aware of AI
activity.

## F-15.12.13 Asset and Scene Comments

## US-15.12.13.1 Designer Attaches Comments to Entities

**As a** designer (P-5), **I want** to attach threaded comments to any asset, entity, node, or
spatial location, **so that** I can leave feedback for teammates directly in context.

## US-15.12.13.2 Artist Views Comments as Viewport Pins

**As an** artist (P-8), **I want** comments displayed as non-intrusive pins in the viewport and
margin markers in editors, **so that** I see feedback without it obscuring my workspace.

## US-15.12.13.3 Creative Director Mentions Team Members

**As a** creative director (P-2), **I want** @mentions in comments that notify the referenced user,
**so that** I can direct feedback to specific team members during reviews.

## US-15.12.13.4 Engine Tester Validates Comment Sync

**As an** engine tester (P-27), **I want** to verify that comments sync across all connected clients
and resolved comments are collapsed but not deleted, **so that** the comment system is reliable and
preserves history.

## F-15.12.14 Pull Request Review in Editor

## US-15.12.14.1 Developer Reviews PRs In-Editor

**As a** developer (P-15), **I want** to view and review pull requests within the editor with
structural diffs rendered in native visual editors (not text diffs), **so that** I can review asset
changes in their proper visual context.

## US-15.12.14.2 Designer Adds PR Comments on Nodes

**As a** designer (P-5), **I want** to add review comments on specific nodes, properties, or regions
of changed assets, **so that** my feedback is precise and contextual.

## US-15.12.14.3 DevOps Pushes Review Status to Provider

**As a** DevOps engineer (P-16), **I want** approve and request-changes actions to push status back
to the Git hosting service via API, **so that** in-editor reviews are reflected in the hosting
provider's PR workflow.

## US-15.12.14.4 Engine Tester Validates Multi-Provider

**As an** engine tester (P-27), **I want** to verify that PR review works with GitHub, GitLab, and
Bitbucket APIs via the configurable provider layer, **so that** in-editor reviews work regardless of
hosting choice.
