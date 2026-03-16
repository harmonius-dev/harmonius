# R-15.12 -- Remote Editing & Collaboration Requirements

## Remote Rendering

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.12.1 | The editor **SHALL** stream the viewport as H.264/H.265 video with adaptive bitrate, input event forwarding with prediction, and GPU-accelerated encoding under 2 ms per frame using platform-native encoders (VideoToolbox, NVENC, VA-API). [F-15.12.1](../../features/tools-editor/remote-editing.md) |  | Remote editing without a local GPU requires efficient viewport streaming. | Benchmark: measure encoding overhead and verify it stays below 2 ms per frame. |
| R-15.12.2 | The editor **SHALL** use a custom protocol for editor UI streaming with high-quality viewport frames, change-detection driven UI panel updates, and QUIC transport with TCP+TLS 1.3 fallback, achieving at least 60% bandwidth reduction versus generic RDP/VNC. [F-15.12.2](../../features/tools-editor/remote-editing.md) generic solutions. 60% reduction. |  | Editor-specific protocol semantics enable far better bandwidth efficiency than | Benchmark: compare bandwidth with VNC for the same session and verify at least |

## Multi-User Collaboration

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.12.3 | The editor **SHALL** support simultaneous multi-user editing with CRDT-based synchronization, per-user undo stacks, presence indicators showing other users' cursors, integrated chat and voice channels, and peer-to-peer mode on LAN with mDNS discovery. [F-15.12.3](../../features/tools-editor/remote-editing.md) cross-user interference. states converge. |  | Real-time collaboration without conflicts requires CRDTs; per-user undo prevents | Integration test: connect three users, make concurrent edits, and verify all |

## Server Infrastructure

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.12.4 | The editor **SHALL** run headless on a remote GPU server with sub-frame latency targeting under 16 ms round-trip on LAN, multi-GPU support with per-session GPU assignment, and headless GPU context creation on all platforms (EGL, headless Metal, WDDM). [F-15.12.4](../../features/tools-editor/remote-editing.md) 16 ms. |  | Centralized GPU servers reduce hardware costs for distributed teams. | Benchmark: measure input-to-display round-trip latency on LAN and verify under |

## Session Management

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.12.5 | The editor **SHALL** support suspending and resuming remote sessions with full state preservation (panels, cameras, selection, undo history, unsaved modifications), including resume on a different client device. [F-15.12.5](../../features/tools-editor/remote-editing.md) exact visual and functional state restoration. |  | Developers must move between office and home without losing state. | Integration test: suspend a session, resume on a different client, and verify |
| R-15.12.6 | The editor **SHALL** automatically adjust stream quality based on network speed (high >100 Mbps, medium 10-100, low <10), with manual quality override to pin a specific tier. [F-15.12.6](../../features/tools-editor/remote-editing.md) medium, and low respectively. |  | Automatic quality adaptation ensures usability across varying network conditions. | Unit test: simulate 150, 50, and 5 Mbps and verify tier selection matches high, |

## Cloud Service

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.12.7 | The editor **SHALL** use a Rust-based cloud service with PostgreSQL for session/user/permission data, S3-compatible storage for CRDT snapshots, horizontal scaling behind a load balancer, and container deployment via Docker/Kubernetes. [F-15.12.7](../../features/tools-editor/remote-editing.md) |  | Scalable cloud infrastructure is required for enterprise real-time collaboration. | Load test: verify 100 concurrent sessions sync correctly. |
| R-15.12.8 | The engine **SHALL** provide tree CRDT for scene hierarchies, operation log CRDT for logic graphs, map CRDT for data tables, and last-writer-wins register per tile for terrain. [F-15.12.8](../../features/tools-editor/remote-editing.md) concurrent editing. valid hierarchy. |  | Each asset type requires a CRDT model matched to its semantics for correct | Unit test: perform concurrent scene reparenting and verify tree CRDT produces a |
| R-15.12.9 | The engine **SHALL** provide role-based permissions (viewer, editor, admin), asset-level exclusive locks, and OAuth2/OIDC authentication for enterprise SSO. [F-15.12.9](../../features/tools-editor/remote-editing.md) |  | Enterprise collaboration requires fine-grained access control and SSO integration. | Unit test: verify viewer role prevents edits. |
| R-15.12.10 | The editor **SHALL** provide voice chat (Opus over QUIC with echo cancellation), text chat with threaded replies and mentions, inline asset references, and searchable history persisted in PostgreSQL. [F-15.12.10](../../features/tools-editor/remote-editing.md) notification. |  | In-editor communication eliminates context switching to external tools. | Integration test: send a chat message and verify delivery and mention |
| R-15.12.11 | The editor **SHALL** support named work groups per team with isolated workspace layers, explicit sharing of changes between groups, and dynamic group membership. [F-15.12.11](../../features/tools-editor/remote-editing.md) shared. |  | Teams need isolation for unfinished work with controlled sharing. | Unit test: verify isolated workspace edits are invisible to other groups until |
| R-15.12.12 | The editor **SHALL** support AI agents as virtual users in collaborative sessions, instructable via text chat, with visible cursors, selections, and edits, and provenance metadata on all agent actions. [F-15.12.12](../../features/tools-editor/remote-editing.md) tags. |  | AI agents must integrate into the collaboration model with full traceability. | Unit test: verify AI agent edits are visible to other users and carry provenance |
| R-15.12.13 | The editor **SHALL** support spatial comments attached to entities or locations with threaded replies, @mention notifications, viewport pins, and real-time sync across clients with session persistence. [F-15.12.13](../../features/tools-editor/remote-editing.md) across sessions. |  | Spatial feedback is essential for art direction and design review. | Integration test: add a comment, verify it syncs across clients and persists |
| R-15.12.14 | The editor **SHALL** support viewing PR changed assets with structural diffs, adding review comments on specific nodes or properties, approve/request-changes actions, and status pushed to GitHub, GitLab, and Bitbucket APIs. [F-15.12.14](../../features/tools-editor/remote-editing.md) service API. |  | In-editor review eliminates context switching for binary asset PRs. | Integration test: approve a PR and verify status is pushed to the hosting |

---

## US-15.12.1 Remote Desktop Optimized Rendering

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.12.1.1 | developer | I want the viewport streamed as H.264/H.265 video so that I can work remotely without a local GPU. |  |  |
| US-15.12.1.2 | developer | I want adaptive bitrate based on network bandwidth so that quality adjusts automatically without manual configuration. |  |  |
| US-15.12.1.3 | developer | I want input event forwarding with prediction so that remote editing feels responsive despite network latency. |  |  |
| US-15.12.1.4 | engine dev | I want GPU-accelerated encoding under 2ms per frame so that encoding overhead does not reduce the editor frame rate. |  |  |
| US-15.12.1.5 | engine dev | I want platform-native encoders (VideoToolbox, NVENC, VA-API) so that encoding leverages hardware acceleration on each platform. |  |  |
| US-15.12.1.6 | engine tester | I want to verify encoding overhead stays below 2ms so that the encoding budget is regression-tested. |  |  |

## US-15.12.2 Remote Editor Protocol

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.12.2.1 | developer | I want viewport frames at high quality and full frame rate so that the 3D viewport is crisp during remote editing. |  |  |
| US-15.12.2.2 | developer | I want UI panels encoded with change-detection driven updates so that idle panels do not consume bandwidth. |  |  |
| US-15.12.2.3 | engine dev | I want QUIC transport with TCP+TLS 1.3 fallback so that the protocol works even when UDP is blocked by firewalls. |  |  |
| US-15.12.2.4 | engine dev | I want 60-80% bandwidth reduction vs generic RDP/VNC so that the protocol is efficient for editor semantics. |  |  |
| US-15.12.2.5 | engine tester | I want to verify bandwidth reduction of at least 60% compared to VNC for the same session so that protocol efficiency is regression-tested. |  |  |

## US-15.12.3 CRDT-Based Real-Time Collaborative Editing

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.12.3.1 | designer | I want to edit the same world simultaneously with teammates so that we can collaborate on level design in real time. |  |  |
| US-15.12.3.2 | designer | I want per-user undo stacks during collaboration so that I can undo my own changes without affecting others. |  |  |
| US-15.12.3.3 | artist | I want presence indicators showing other users' cursors so that I can see what my collaborators are doing. |  |  |
| US-15.12.3.4 | developer | I want CRDT-based synchronization that merges concurrent edits without conflicts so that no one's work is lost during simultaneous editing. |  |  |
| US-15.12.3.5 | developer | I want integrated chat and voice channels so that I can coordinate with collaborators without external tools. |  |  |
| US-15.12.3.6 | engine dev | I want peer-to-peer mode on LAN with mDNS discovery so that collaboration works without a cloud server on local networks. |  |  |
| US-15.12.3.7 | engine tester | I want to verify three concurrent users' edits converge to the same state so that CRDT convergence is regression-tested. |  |  |

## US-15.12.4 Remote GPU Server Support

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.12.4.1 | developer | I want the editor running headless on a remote GPU server so that I can use a thin client without a dedicated GPU. |  |  |
| US-15.12.4.2 | developer | I want sub-frame latency targeting under 16ms on LAN so that remote editing feels local. |  |  |
| US-15.12.4.3 | server admin | I want multi-GPU support with per-session GPU assignment so that a single server hosts multiple concurrent editing sessions. |  |  |
| US-15.12.4.4 | server admin | I want an orchestrator service for session scheduling so that GPU assignment is managed automatically. |  |  |
| US-15.12.4.5 | engine dev | I want headless GPU context creation on all platforms (EGL, headless Metal, WDDM) so that the server runs without a display. |  |  |
| US-15.12.4.6 | engine tester | I want to verify input-to-display round-trip latency under 16ms on LAN so that remote session responsiveness is regression-tested. |  |  |

## US-15.12.5 Session Handoff and Persistence

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.12.5.1 | developer | I want to suspend a remote session and resume later so that I can move between locations without losing state. |  |  |
| US-15.12.5.2 | developer | I want session resume on a different client device so that I can start at the office and continue from home. |  |  |
| US-15.12.5.3 | developer | I want undo history preserved across suspend/resume so that I can undo actions from before the suspension. |  |  |
| US-15.12.5.4 | engine tester | I want to verify session resume restores exact visual and functional state so that session persistence is regression-tested. |  |  |

## US-15.12.6 Bandwidth Adaptation and Quality Tiers

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.12.6.1 | developer | I want automatic quality adjustment based on network speed so that the stream remains usable across varying connections. |  |  |
| US-15.12.6.2 | developer | I want manual quality override to pin a specific tier so that I can force high quality when I know my bandwidth is stable. |  |  |
| US-15.12.6.3 | engine tester | I want to verify tier selection at 150, 50, and 5 Mbps matches high, medium, and low respectively so that auto-tier selection is regression-tested. |  |  |

## US-15.12.7 Collaboration Cloud Service

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.12.7.1 | server admin | I want a Rust-based cloud service for collaboration so that real-time sync is managed by a dedicated, scalable backend. |  |  |
| US-15.12.7.2 | server admin | I want PostgreSQL for session/user/permission data so that relational data is stored reliably. |  |  |
| US-15.12.7.3 | server admin | I want S3-compatible storage for CRDT snapshots so that document state persists durably. |  |  |
| US-15.12.7.4 | server admin | I want horizontal scaling behind a load balancer so that the service handles hundreds of concurrent sessions. |  |  |
| US-15.12.7.5 | DevOps | I want container deployment via Docker/Kubernetes so that the service integrates with standard infrastructure. |  |  |
| US-15.12.7.6 | engine tester | I want to verify 100 concurrent sessions sync correctly so that service scalability is regression-tested. |  |  |

## US-15.12.8 CRDT Document Model for Engine Assets

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.12.8.1 | engine dev | I want tree CRDT for scene hierarchies so that entity add/remove/reparent merges correctly. |  |  |
| US-15.12.8.2 | engine dev | I want operation log CRDT for logic graphs so that node add/remove/connect operations merge correctly. |  |  |
| US-15.12.8.3 | engine dev | I want map CRDT for data tables (per-row, per-cell) so that concurrent table edits merge without data loss. |  |  |
| US-15.12.8.4 | engine dev | I want last-writer-wins register per tile for terrain so that heightmap edits resolve deterministically. |  |  |
| US-15.12.8.5 | engine tester | I want to verify concurrent scene reparenting produces a valid hierarchy via tree CRDT so that CRDT correctness is regression-tested per asset type. |  |  |

## US-15.12.9 Collaboration Access Control and Permissions

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.12.9.1 | server admin | I want role-based permissions (viewer, editor, admin) so that access is controlled per-project and per-asset. |  |  |
| US-15.12.9.2 | server admin | I want asset-level exclusive locks within sessions so that specific assets can be edited by only one user at a time. |  |  |
| US-15.12.9.3 | server admin | I want OAuth2/OIDC authentication for enterprise SSO so that the service integrates with corporate identity providers. |  |  |
| US-15.12.9.4 | engine tester | I want to verify viewer role prevents edits so that access control enforcement is regression-tested. |  |  |

## US-15.12.10 Integrated Voice and Text Chat

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.12.10.1 | designer | I want voice chat within collaborative sessions so that I can coordinate with teammates without alt-tabbing. |  |  |
| US-15.12.10.2 | designer | I want text chat with threaded replies and mentions so that I can have structured conversations about specific topics. |  |  |
| US-15.12.10.3 | designer | I want inline asset references in chat messages so that clicking a reference navigates to that asset in the editor. |  |  |
| US-15.12.10.4 | developer | I want searchable chat history persisted in PostgreSQL so that I can find past discussions about project decisions. |  |  |
| US-15.12.10.5 | engine dev | I want Opus codec over QUIC with echo cancellation so that voice quality is high with minimal bandwidth. |  |  |
| US-15.12.10.6 | engine tester | I want to verify chat message delivery and mention notifications so that communication features are regression-tested. |  |  |

## US-15.12.11 Work Groups and Isolated Workspaces

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.12.11.1 | creative director | I want named work groups per team so that level designers and shader artists work in separate contexts. |  |  |
| US-15.12.11.2 | designer | I want isolated workspace layers per group so that my unfinished edits are invisible to other groups. |  |  |
| US-15.12.11.3 | designer | I want to explicitly share workspace changes so that I control when other groups see my work. |  |  |
| US-15.12.11.4 | developer | I want dynamic group membership so that I can join and leave groups without disrupting others. |  |  |
| US-15.12.11.5 | engine tester | I want to verify isolated workspace edits are invisible to other groups until shared so that workspace isolation is regression-tested. |  |  |

## US-15.12.12 AI Agent Collaboration

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.12.12.1 | designer | I want AI agents as virtual users in collaborative sessions so that I can delegate content generation tasks to agents. |  |  |
| US-15.12.12.2 | designer | I want to instruct AI agents via text chat so that I can assign tasks using natural language. |  |  |
| US-15.12.12.3 | developer | I want to see AI agent cursors, selections, and edits so that I can monitor what the agent is doing in real time. |  |  |
| US-15.12.12.4 | developer | I want all AI agent actions tagged with provenance metadata so that AI-generated content is traceable. |  |  |
| US-15.12.12.5 | engine tester | I want to verify AI agent edits are visible to other users and carry provenance tags so that agent collaboration is regression-tested. |  |  |

## US-15.12.13 Asset and Scene Comments

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.12.13.1 | creative director | I want to attach comments to entities or locations so that I can provide spatial feedback to the team. |  |  |
| US-15.12.13.2 | designer | I want threaded comment replies on assets so that feedback conversations are structured and traceable. |  |  |
| US-15.12.13.3 | artist | I want @mention notifications in comments so that I am alerted when someone needs my input on an asset. |  |  |
| US-15.12.13.4 | artist | I want viewport pins showing comment locations so that I can click a pin to read feedback in spatial context. |  |  |
| US-15.12.13.5 | developer | I want comments synced in real time across clients so that all team members see the latest feedback. |  |  |
| US-15.12.13.6 | engine tester | I want to verify comments sync across connected clients and persist across sessions so that comment persistence is regression-tested. |  |  |

## US-15.12.14 Pull Request Review in Editor

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.12.14.1 | developer | I want to view PR changed assets with structural diffs so that I can review binary asset changes in native editors. |  |  |
| US-15.12.14.2 | developer | I want to add review comments on specific nodes or properties so that my feedback is anchored to exact change locations. |  |  |
| US-15.12.14.3 | developer | I want approve/request-changes actions from the editor so that I can complete the review cycle without a web browser. |  |  |
| US-15.12.14.4 | DevOps | I want review status pushed to GitHub, GitLab, and Bitbucket APIs so that in-editor reviews integrate with the hosting provider's workflow. |  |  |
| US-15.12.14.5 | engine tester | I want to verify approval status is pushed to the Git hosting service API so that PR integration is regression-tested. |  |  |
