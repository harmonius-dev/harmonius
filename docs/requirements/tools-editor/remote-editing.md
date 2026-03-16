# R-15.12 -- Remote Editing & Collaboration Requirements

## Remote Rendering

### R-15.12.1 Remote Desktop Optimized Rendering

The editor **SHALL** stream the viewport as H.264/H.265
video with adaptive bitrate, input event forwarding with
prediction, and GPU-accelerated encoding under 2 ms per
frame using platform-native encoders (VideoToolbox, NVENC,
VA-API).

- **Derived from:**
  [F-15.12.1](../../features/tools-editor/remote-editing.md)
- **Rationale:** Remote editing without a local GPU
  requires efficient viewport streaming.
- **Verification:** Benchmark: measure encoding overhead
  and verify it stays below 2 ms per frame.

### R-15.12.2 Remote Editor Protocol

The editor **SHALL** use a custom protocol for editor UI
streaming with high-quality viewport frames,
change-detection driven UI panel updates, and QUIC
transport with TCP+TLS 1.3 fallback, achieving at least
60% bandwidth reduction versus generic RDP/VNC.

- **Derived from:**
  [F-15.12.2](../../features/tools-editor/remote-editing.md)
- **Rationale:** Editor-specific protocol semantics enable
  far better bandwidth efficiency than generic solutions.
- **Verification:** Benchmark: compare bandwidth with VNC
  for the same session and verify at least 60% reduction.

## Multi-User Collaboration

### R-15.12.3 CRDT-Based Real-Time Collaborative Editing

The editor **SHALL** support simultaneous multi-user
editing with CRDT-based synchronization, per-user undo
stacks, presence indicators showing other users' cursors,
integrated chat and voice channels, and peer-to-peer mode
on LAN with mDNS discovery.

- **Derived from:**
  [F-15.12.3](../../features/tools-editor/remote-editing.md)
- **Rationale:** Real-time collaboration without conflicts
  requires CRDTs; per-user undo prevents cross-user
  interference.
- **Verification:** Integration test: connect three users,
  make concurrent edits, and verify all states converge.

## Server Infrastructure

### R-15.12.4 Remote GPU Server Support

The editor **SHALL** run headless on a remote GPU server
with sub-frame latency targeting under 16 ms round-trip on
LAN, multi-GPU support with per-session GPU assignment, and
headless GPU context creation on all platforms (EGL,
headless Metal, WDDM).

- **Derived from:**
  [F-15.12.4](../../features/tools-editor/remote-editing.md)
- **Rationale:** Centralized GPU servers reduce hardware
  costs for distributed teams.
- **Verification:** Benchmark: measure input-to-display
  round-trip latency on LAN and verify under 16 ms.

## Session Management

### R-15.12.5 Session Handoff and Persistence

The editor **SHALL** support suspending and resuming remote
sessions with full state preservation (panels, cameras,
selection, undo history, unsaved modifications), including
resume on a different client device.

- **Derived from:**
  [F-15.12.5](../../features/tools-editor/remote-editing.md)
- **Rationale:** Developers must move between office and
  home without losing state.
- **Verification:** Integration test: suspend a session,
  resume on a different client, and verify exact visual
  and functional state restoration.

### R-15.12.6 Bandwidth Adaptation and Quality Tiers

The editor **SHALL** automatically adjust stream quality
based on network speed (high >100 Mbps, medium 10-100,
low <10), with manual quality override to pin a specific
tier.

- **Derived from:**
  [F-15.12.6](../../features/tools-editor/remote-editing.md)
- **Rationale:** Automatic quality adaptation ensures
  usability across varying network conditions.
- **Verification:** Unit test: simulate 150, 50, and 5
  Mbps and verify tier selection matches high, medium,
  and low respectively.

## Cloud Service

### R-15.12.7 Collaboration Cloud Service

The editor **SHALL** use a Rust-based cloud service with
PostgreSQL for session/user/permission data, S3-compatible
storage for CRDT snapshots, horizontal scaling behind a
load balancer, and container deployment via
Docker/Kubernetes.

- **Derived from:**
  [F-15.12.7](../../features/tools-editor/remote-editing.md)
- **Rationale:** Scalable cloud infrastructure is required
  for enterprise real-time collaboration.
- **Verification:** Load test: verify 100 concurrent
  sessions sync correctly.

### R-15.12.8 CRDT Document Model for Engine Assets

The engine **SHALL** provide tree CRDT for scene
hierarchies, operation log CRDT for logic graphs, map CRDT
for data tables, and last-writer-wins register per tile
for terrain.

- **Derived from:**
  [F-15.12.8](../../features/tools-editor/remote-editing.md)
- **Rationale:** Each asset type requires a CRDT model
  matched to its semantics for correct concurrent editing.
- **Verification:** Unit test: perform concurrent scene
  reparenting and verify tree CRDT produces a valid
  hierarchy.

### R-15.12.9 Collaboration Access Control

The engine **SHALL** provide role-based permissions
(viewer, editor, admin), asset-level exclusive locks,
and OAuth2/OIDC authentication for enterprise SSO.

- **Derived from:**
  [F-15.12.9](../../features/tools-editor/remote-editing.md)
- **Rationale:** Enterprise collaboration requires
  fine-grained access control and SSO integration.
- **Verification:** Unit test: verify viewer role prevents
  edits.

### R-15.12.10 Integrated Voice and Text Chat

The editor **SHALL** provide voice chat (Opus over QUIC
with echo cancellation), text chat with threaded replies
and mentions, inline asset references, and searchable
history persisted in PostgreSQL.

- **Derived from:**
  [F-15.12.10](../../features/tools-editor/remote-editing.md)
- **Rationale:** In-editor communication eliminates
  context switching to external tools.
- **Verification:** Integration test: send a chat message
  and verify delivery and mention notification.

### R-15.12.11 Work Groups and Isolated Workspaces

The editor **SHALL** support named work groups per team
with isolated workspace layers, explicit sharing of
changes between groups, and dynamic group membership.

- **Derived from:**
  [F-15.12.11](../../features/tools-editor/remote-editing.md)
- **Rationale:** Teams need isolation for unfinished work
  with controlled sharing.
- **Verification:** Unit test: verify isolated workspace
  edits are invisible to other groups until shared.

### R-15.12.12 AI Agent Collaboration

The editor **SHALL** support AI agents as virtual users in
collaborative sessions, instructable via text chat, with
visible cursors, selections, and edits, and provenance
metadata on all agent actions.

- **Derived from:**
  [F-15.12.12](../../features/tools-editor/remote-editing.md)
- **Rationale:** AI agents must integrate into the
  collaboration model with full traceability.
- **Verification:** Unit test: verify AI agent edits are
  visible to other users and carry provenance tags.

### R-15.12.13 Asset and Scene Comments

The editor **SHALL** support spatial comments attached to
entities or locations with threaded replies, @mention
notifications, viewport pins, and real-time sync across
clients with session persistence.

- **Derived from:**
  [F-15.12.13](../../features/tools-editor/remote-editing.md)
- **Rationale:** Spatial feedback is essential for art
  direction and design review.
- **Verification:** Integration test: add a comment, verify
  it syncs across clients and persists across sessions.

### R-15.12.14 Pull Request Review in Editor

The editor **SHALL** support viewing PR changed assets with
structural diffs, adding review comments on specific nodes
or properties, approve/request-changes actions, and status
pushed to GitHub, GitLab, and Bitbucket APIs.

- **Derived from:**
  [F-15.12.14](../../features/tools-editor/remote-editing.md)
- **Rationale:** In-editor review eliminates context
  switching for binary asset PRs.
- **Verification:** Integration test: approve a PR and
  verify status is pushed to the hosting service API.

---

## User Stories

## US-15.12.1 Remote Desktop Optimized Rendering

### US-15.12.1.1
As a **developer (P-15)**, I want the viewport streamed as H.264/H.265 video so that I can work
remotely without a local GPU.

### US-15.12.1.2
As a **developer (P-15)**, I want adaptive bitrate based on network bandwidth so that quality
adjusts automatically without manual configuration.

### US-15.12.1.3
As a **developer (P-15)**, I want input event forwarding with prediction so that remote editing
feels responsive despite network latency.

### US-15.12.1.4
As an **engine dev (P-26)**, I want GPU-accelerated encoding under 2ms per frame so that encoding
overhead does not reduce the editor frame rate.

### US-15.12.1.5
As an **engine dev (P-26)**, I want platform-native encoders (VideoToolbox, NVENC, VA-API) so that
encoding leverages hardware acceleration on each platform.

### US-15.12.1.6
As an **engine tester (P-27)**, I want to verify encoding overhead stays below 2ms so that the
encoding budget is regression-tested.

---

## US-15.12.2 Remote Editor Protocol

### US-15.12.2.1
As a **developer (P-15)**, I want viewport frames at high quality and full frame rate so that the 3D
viewport is crisp during remote editing.

### US-15.12.2.2
As a **developer (P-15)**, I want UI panels encoded with change-detection driven updates so that
idle panels do not consume bandwidth.

### US-15.12.2.3
As an **engine dev (P-26)**, I want QUIC transport with TCP+TLS 1.3 fallback so that the protocol
works even when UDP is blocked by firewalls.

### US-15.12.2.4
As an **engine dev (P-26)**, I want 60-80% bandwidth reduction vs generic RDP/VNC so that the
protocol is efficient for editor semantics.

### US-15.12.2.5
As an **engine tester (P-27)**, I want to verify bandwidth reduction of at least 60% compared to VNC
for the same session so that protocol efficiency is regression-tested.

---

## US-15.12.3 CRDT-Based Real-Time Collaborative Editing

### US-15.12.3.1
As a **designer (P-5)**, I want to edit the same world simultaneously with teammates so that we can
collaborate on level design in real time.

### US-15.12.3.2
As a **designer (P-5)**, I want per-user undo stacks during collaboration so that I can undo my own
changes without affecting others.

### US-15.12.3.3
As a **artist (P-8)**, I want presence indicators showing other users' cursors so that I can see
what my collaborators are doing.

### US-15.12.3.4
As a **developer (P-15)**, I want CRDT-based synchronization that merges concurrent edits without
conflicts so that no one's work is lost during simultaneous editing.

### US-15.12.3.5
As a **developer (P-15)**, I want integrated chat and voice channels so that I can coordinate with
collaborators without external tools.

### US-15.12.3.6
As an **engine dev (P-26)**, I want peer-to-peer mode on LAN with mDNS discovery so that
collaboration works without a cloud server on local networks.

### US-15.12.3.7
As an **engine tester (P-27)**, I want to verify three concurrent users' edits converge to the same
state so that CRDT convergence is regression-tested.

---

## US-15.12.4 Remote GPU Server Support

### US-15.12.4.1
As a **developer (P-15)**, I want the editor running headless on a remote GPU server so that I can
use a thin client without a dedicated GPU.

### US-15.12.4.2
As a **developer (P-15)**, I want sub-frame latency targeting under 16ms on LAN so that remote
editing feels local.

### US-15.12.4.3
As a **server admin (P-22)**, I want multi-GPU support with per-session GPU assignment so that a
single server hosts multiple concurrent editing sessions.

### US-15.12.4.4
As a **server admin (P-22)**, I want an orchestrator service for session scheduling so that GPU
assignment is managed automatically.

### US-15.12.4.5
As an **engine dev (P-26)**, I want headless GPU context creation on all platforms (EGL, headless
Metal, WDDM) so that the server runs without a display.

### US-15.12.4.6
As an **engine tester (P-27)**, I want to verify input-to-display round-trip latency under 16ms on
LAN so that remote session responsiveness is regression-tested.

---

## US-15.12.5 Session Handoff and Persistence

### US-15.12.5.1
As a **developer (P-15)**, I want to suspend a remote session and resume later so that I can move
between locations without losing state.

### US-15.12.5.2
As a **developer (P-15)**, I want session resume on a different client device so that I can start at
the office and continue from home.

### US-15.12.5.3
As a **developer (P-15)**, I want undo history preserved across suspend/resume so that I can undo
actions from before the suspension.

### US-15.12.5.4
As an **engine tester (P-27)**, I want to verify session resume restores exact visual and functional
state so that session persistence is regression-tested.

---

## US-15.12.6 Bandwidth Adaptation and Quality Tiers

### US-15.12.6.1
As a **developer (P-15)**, I want automatic quality adjustment based on network speed so that the
stream remains usable across varying connections.

### US-15.12.6.2
As a **developer (P-15)**, I want manual quality override to pin a specific tier so that I can force
high quality when I know my bandwidth is stable.

### US-15.12.6.3
As an **engine tester (P-27)**, I want to verify tier selection at 150, 50, and 5 Mbps matches high,
medium, and low respectively so that auto-tier selection is regression-tested.

---

## US-15.12.7 Collaboration Cloud Service

### US-15.12.7.1
As a **server admin (P-22)**, I want a Rust-based cloud service for collaboration so that real-time
sync is managed by a dedicated, scalable backend.

### US-15.12.7.2
As a **server admin (P-22)**, I want PostgreSQL for session/user/permission data so that relational
data is stored reliably.

### US-15.12.7.3
As a **server admin (P-22)**, I want S3-compatible storage for CRDT snapshots so that document state
persists durably.

### US-15.12.7.4
As a **server admin (P-22)**, I want horizontal scaling behind a load balancer so that the service
handles hundreds of concurrent sessions.

### US-15.12.7.5
As a **DevOps (P-16)**, I want container deployment via Docker/Kubernetes so that the service
integrates with standard infrastructure.

### US-15.12.7.6
As an **engine tester (P-27)**, I want to verify 100 concurrent sessions sync correctly so that
service scalability is regression-tested.

---

## US-15.12.8 CRDT Document Model for Engine Assets

### US-15.12.8.1
As an **engine dev (P-26)**, I want tree CRDT for scene hierarchies so that entity
add/remove/reparent merges correctly.

### US-15.12.8.2
As an **engine dev (P-26)**, I want operation log CRDT for logic graphs so that node
add/remove/connect operations merge correctly.

### US-15.12.8.3
As an **engine dev (P-26)**, I want map CRDT for data tables (per-row, per-cell) so that concurrent
table edits merge without data loss.

### US-15.12.8.4
As an **engine dev (P-26)**, I want last-writer-wins register per tile for terrain so that heightmap
edits resolve deterministically.

### US-15.12.8.5
As an **engine tester (P-27)**, I want to verify concurrent scene reparenting produces a valid
hierarchy via tree CRDT so that CRDT correctness is regression-tested per asset type.

---

## US-15.12.9 Collaboration Access Control and Permissions

### US-15.12.9.1
As a **server admin (P-22)**, I want role-based permissions (viewer, editor, admin) so that access
is controlled per-project and per-asset.

### US-15.12.9.2
As a **server admin (P-22)**, I want asset-level exclusive locks within sessions so that specific
assets can be edited by only one user at a time.

### US-15.12.9.3
As a **server admin (P-22)**, I want OAuth2/OIDC authentication for enterprise SSO so that the
service integrates with corporate identity providers.

### US-15.12.9.4
As an **engine tester (P-27)**, I want to verify viewer role prevents edits so that access control
enforcement is regression-tested.

---

## US-15.12.10 Integrated Voice and Text Chat

### US-15.12.10.1
As a **designer (P-5)**, I want voice chat within collaborative sessions so that I can coordinate
with teammates without alt-tabbing.

### US-15.12.10.2
As a **designer (P-5)**, I want text chat with threaded replies and mentions so that I can have
structured conversations about specific topics.

### US-15.12.10.3
As a **designer (P-5)**, I want inline asset references in chat messages so that clicking a
reference navigates to that asset in the editor.

### US-15.12.10.4
As a **developer (P-15)**, I want searchable chat history persisted in PostgreSQL so that I can find
past discussions about project decisions.

### US-15.12.10.5
As an **engine dev (P-26)**, I want Opus codec over QUIC with echo cancellation so that voice
quality is high with minimal bandwidth.

### US-15.12.10.6
As an **engine tester (P-27)**, I want to verify chat message delivery and mention notifications so
that communication features are regression-tested.

---

## US-15.12.11 Work Groups and Isolated Workspaces

### US-15.12.11.1
As a **creative director (P-2)**, I want named work groups per team so that level designers and
shader artists work in separate contexts.

### US-15.12.11.2
As a **designer (P-5)**, I want isolated workspace layers per group so that my unfinished edits are
invisible to other groups.

### US-15.12.11.3
As a **designer (P-5)**, I want to explicitly share workspace changes so that I control when other
groups see my work.

### US-15.12.11.4
As a **developer (P-15)**, I want dynamic group membership so that I can join and leave groups
without disrupting others.

### US-15.12.11.5
As an **engine tester (P-27)**, I want to verify isolated workspace edits are invisible to other
groups until shared so that workspace isolation is regression-tested.

---

## US-15.12.12 AI Agent Collaboration

### US-15.12.12.1
As a **designer (P-5)**, I want AI agents as virtual users in collaborative sessions so that I can
delegate content generation tasks to agents.

### US-15.12.12.2
As a **designer (P-5)**, I want to instruct AI agents via text chat so that I can assign tasks using
natural language.

### US-15.12.12.3
As a **developer (P-15)**, I want to see AI agent cursors, selections, and edits so that I can
monitor what the agent is doing in real time.

### US-15.12.12.4
As a **developer (P-15)**, I want all AI agent actions tagged with provenance metadata so that
AI-generated content is traceable.

### US-15.12.12.5
As an **engine tester (P-27)**, I want to verify AI agent edits are visible to other users and carry
provenance tags so that agent collaboration is regression-tested.

---

## US-15.12.13 Asset and Scene Comments

### US-15.12.13.1
As a **creative director (P-2)**, I want to attach comments to entities or locations so that I can
provide spatial feedback to the team.

### US-15.12.13.2
As a **designer (P-5)**, I want threaded comment replies on assets so that feedback conversations
are structured and traceable.

### US-15.12.13.3
As a **artist (P-8)**, I want @mention notifications in comments so that I am alerted when someone
needs my input on an asset.

### US-15.12.13.4
As a **artist (P-8)**, I want viewport pins showing comment locations so that I can click a pin to
read feedback in spatial context.

### US-15.12.13.5
As a **developer (P-15)**, I want comments synced in real time across clients so that all team
members see the latest feedback.

### US-15.12.13.6
As an **engine tester (P-27)**, I want to verify comments sync across connected clients and persist
across sessions so that comment persistence is regression-tested.

---

## US-15.12.14 Pull Request Review in Editor

### US-15.12.14.1
As a **developer (P-15)**, I want to view PR changed assets with structural diffs so that I can
review binary asset changes in native editors.

### US-15.12.14.2
As a **developer (P-15)**, I want to add review comments on specific nodes or properties so that my
feedback is anchored to exact change locations.

### US-15.12.14.3
As a **developer (P-15)**, I want approve/request-changes actions from the editor so that I can
complete the review cycle without a web browser.

### US-15.12.14.4
As a **DevOps (P-16)**, I want review status pushed to GitHub, GitLab, and Bitbucket APIs so that
in-editor reviews integrate with the hosting provider's workflow.

### US-15.12.14.5
As an **engine tester (P-27)**, I want to verify approval status is pushed to the Git hosting
service API so that PR integration is regression-tested.
