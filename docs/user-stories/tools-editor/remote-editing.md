# User Stories: Remote Editing and Collaboration

## F-15.12.1 Remote Desktop Optimized Rendering

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.12.1.1 | developer (P-15)        |          |              |
| US-15.12.1.2 | artist (P-8)            |          |              |
| US-15.12.1.3 | engine developer (P-26) |          |              |
| US-15.12.1.4 | engine tester (P-27)    |          |              |

1. **US-15.12.1.1** — the editor viewport rendered to a compressed video stream (H.264/H.265) with
   adaptive bitrate
   - **Acceptance:** I can work from home on a remote GPU server without dedicated GPU hardware at
     my desk
2. **US-15.12.1.2** — input events forwarded from the client with prediction to mask network latency
   - **Acceptance:** gizmo manipulation and viewport navigation feel responsive over remote
     connections
3. **US-15.12.1.3** — GPU-accelerated encoding keeping overhead below 2ms per frame
   - **Acceptance:** remote rendering does not significantly reduce frame rate
4. **US-15.12.1.4** — to verify that encoding uses platform-native APIs (VideoToolbox on macOS,
   NVENC/AMF on Windows, VA-API on Linux)
   - **Acceptance:** hardware acceleration is utilized on each platform

## F-15.12.2 Remote Editor Protocol

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.12.2.1 | DevOps engineer (P-16) |          |              |
| US-15.12.2.2 | developer (P-15)       |          |              |
| US-15.12.2.3 | server admin (P-22)    |          |              |
| US-15.12.2.4 | engine tester (P-27)   |          |              |

1. **US-15.12.2.1** — a custom protocol that encodes viewport frames at high quality and UI panels
   with change-detection updates
   - **Acceptance:** bandwidth is reduced 60-80% compared to generic RDP or VNC
2. **US-15.12.2.2** — the protocol to run over QUIC for low-latency multiplexed transport with
   TCP+TLS 1.3 fallback
   - **Acceptance:** remote editing works reliably even when UDP is blocked
3. **US-15.12.2.3** — the remote editor protocol to support configurable quality and bandwidth
   settings
   - **Acceptance:** I can tune performance for different network environments
4. **US-15.12.2.4** — to verify that the protocol falls back gracefully from QUIC to TCP when UDP is
   blocked
   - **Acceptance:** corporate firewalls do not prevent remote editing

## F-15.12.3 CRDT-Based Real-Time Collaborative Editing

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.12.3.1 | designer (P-5)          |          |              |
| US-15.12.3.2 | artist (P-8)            |          |              |
| US-15.12.3.3 | creative director (P-2) |          |              |
| US-15.12.3.4 | engine tester (P-27)    |          |              |

1. **US-15.12.3.1** — multiple users to connect to the same shared world with independent viewports
   and selection states, syncing edits in real time via CRDTs
   - **Acceptance:** the team can work on the same level without file locking
2. **US-15.12.3.2** — per-user undo stacks so I can undo my changes without affecting other
   collaborators
   - **Acceptance:** parallel editing does not create undo conflicts
3. **US-15.12.3.3** — presence indicators showing cursor positions and selections of all connected
   users
   - **Acceptance:** I can see who is working where during collaborative sessions
4. **US-15.12.3.4** — to verify that concurrent edits merge without conflicts via CRDTs
   - **Acceptance:** collaborative editing never loses data or produces inconsistent states

## F-15.12.4 Remote GPU Server Support

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.12.4.1 | DevOps engineer (P-16) |          |              |
| US-15.12.4.2 | developer (P-15)       |          |              |
| US-15.12.4.3 | server admin (P-22)    |          |              |
| US-15.12.4.4 | engine tester (P-27)   |          |              |

1. **US-15.12.4.1** — the editor to run headless on a multi-GPU server with each session assigned to
   a dedicated GPU
   - **Acceptance:** multiple developers share a single powerful server
2. **US-15.12.4.2** — input events forwarded with sub-frame latency targeting under 16ms round-trip
   on LAN
   - **Acceptance:** editing feels like working locally
3. **US-15.12.4.3** — session scheduling and GPU assignment managed by an orchestrator service
   - **Acceptance:** GPU resources are allocated efficiently across users
4. **US-15.12.4.4** — to verify that headless mode works with EGL on Linux and headless Metal on
   macOS for GPU context creation without a display
   - **Acceptance:** server deployment does not require display hardware

## F-15.12.5 Session Handoff and Persistence

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.12.5.1 | developer (P-15)        |          |              |
| US-15.12.5.2 | designer (P-5)          |          |              |
| US-15.12.5.3 | engine developer (P-26) |          |              |
| US-15.12.5.4 | engine tester (P-27)    |          |              |

1. **US-15.12.5.1** — to suspend a remote editing session and resume it later from a different
   client with all panels, camera positions, selections, and unsaved work intact
   - **Acceptance:** I can move between locations without losing context
2. **US-15.12.5.2** — session state serialized to server storage on disconnect
   - **Acceptance:** accidental network drops do not lose my editing state
3. **US-15.12.5.3** — session state to use the same binary format as crash recovery
   - **Acceptance:** serialization is fast and complete
4. **US-15.12.5.4** — to verify that session resume restores the editor to the exact visual and
   functional state at suspension
   - **Acceptance:** handoff is seamless

## F-15.12.6 Bandwidth Adaptation and Quality Tiers

| ID           | Persona              | Features | Requirements |
|--------------|----------------------|----------|--------------|
| US-15.12.6.1 | developer (P-15)     |          |              |
| US-15.12.6.2 | artist (P-8)         |          |              |
| US-15.12.6.3 | server admin (P-22)  |          |              |
| US-15.12.6.4 | engine tester (P-27) |          |              |

1. **US-15.12.6.1** — stream quality to adjust automatically based on network conditions
   (high/medium/low bandwidth tiers)
   - **Acceptance:** the editing experience is optimized for my current connection
2. **US-15.12.6.2** — to override automatic tier selection and pin a specific quality level
   - **Acceptance:** I get consistent visual fidelity when evaluating materials and lighting
3. **US-15.12.6.3** — configurable bandwidth thresholds for each quality tier
   - **Acceptance:** I can tune the adaptation for my organization's network infrastructure
4. **US-15.12.6.4** — to verify that quality degrades smoothly as bandwidth drops without visual
   artifacts or input lag spikes
   - **Acceptance:** remote editing remains usable on poor connections

## F-15.12.7 Collaboration Cloud Service

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.12.7.1 | engine developer (P-26) |          |              |
| US-15.12.7.2 | server admin (P-22)     |          |              |
| US-15.12.7.3 | DevOps engineer (P-16)  |          |              |
| US-15.12.7.4 | engine tester (P-27)    |          |              |

1. **US-15.12.7.1** — a centralized cloud service in Rust (IoReactor) managing CRDT synchronization,
   sessions, presence, and collaboration metadata
   - **Acceptance:** the backend is performant and type-safe
2. **US-15.12.7.2** — the service to handle hundreds of concurrent sessions with horizontal scaling
   behind a load balancer
   - **Acceptance:** the collaboration infrastructure scales with team size
3. **US-15.12.7.3** — the service deployed as Docker/Kubernetes containers with PostgreSQL and S3
   - **Acceptance:** deployment follows standard cloud infrastructure patterns
4. **US-15.12.7.4** — to verify that WebSocket transport for real-time sync handles concurrent
   operations correctly
   - **Acceptance:** CRDT operations are never lost or reordered

## F-15.12.8 CRDT Document Model for Engine Assets

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.12.8.1 | engine developer (P-26) |          |              |
| US-15.12.8.2 | designer (P-5)          |          |              |
| US-15.12.8.3 | tech artist (P-13)      |          |              |
| US-15.12.8.4 | engine tester (P-27)    |          |              |

1. **US-15.12.8.1** — each asset type to define a CRDT document schema (tree CRDT for scenes,
   operation log for graphs, map CRDT for data tables, LWW register for heightmaps)
   - **Acceptance:** all asset types merge correctly during collaboration
2. **US-15.12.8.2** — editors to read/write through CRDT accessors that handle merge automatically
   - **Acceptance:** I do not need to think about synchronization during editing
3. **US-15.12.8.3** — terrain heightmaps to use last-writer-wins registers per tile for
   collaborative editing
   - **Acceptance:** terrain sculpting resolves conflicts sensibly
4. **US-15.12.8.4** — to verify that every editable asset type has a CRDT document schema
   - **Acceptance:** no asset type is excluded from collaborative editing

## F-15.12.9 Collaboration Access Control and Permissions

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.12.9.1 | server admin (P-22)    |          |              |
| US-15.12.9.2 | developer (P-15)       |          |              |
| US-15.12.9.3 | DevOps engineer (P-16) |          |              |
| US-15.12.9.4 | engine tester (P-27)   |          |              |

1. **US-15.12.9.1** — per-project and per-asset access control with roles (viewer, editor, admin)
   enforced server-side
   - **Acceptance:** only authorized users can edit sensitive assets
2. **US-15.12.9.2** — asset-level locks within collaborative sessions
   - **Acceptance:** I can claim exclusive editing rights on critical assets during sensitive
     changes
3. **US-15.12.9.3** — authentication via OAuth2/OIDC for enterprise SSO and API keys for CI/CD
   integration
   - **Acceptance:** access control integrates with our identity provider
4. **US-15.12.9.4** — to verify that permissions are enforced server-side and cannot be bypassed by
   the client
   - **Acceptance:** access controls are security-meaningful

## F-15.12.10 Integrated Voice and Text Chat

| ID            | Persona                 | Features | Requirements |
|---------------|-------------------------|----------|--------------|
| US-15.12.10.1 | designer (P-5)          |          |              |
| US-15.12.10.2 | artist (P-8)            |          |              |
| US-15.12.10.3 | creative director (P-2) |          |              |
| US-15.12.10.4 | engine tester (P-27)    |          |              |

1. **US-15.12.10.1** — built-in voice and text chat within collaborative sessions
   - **Acceptance:** I can coordinate with teammates without switching to a separate communication
     tool
2. **US-15.12.10.2** — text chat with inline asset and node references that navigate to the
   referenced item on click
   - **Acceptance:** team discussions are linked to specific content
3. **US-15.12.10.3** — to join voice channels per work group
   - **Acceptance:** I can provide real-time feedback to different teams during art reviews
4. **US-15.12.10.4** — to verify that voice chat using Opus codec over QUIC with echo cancellation
   and noise suppression produces clear audio
   - **Acceptance:** team communication is reliable

## F-15.12.11 Work Groups and Isolated Workspaces

| ID            | Persona                 | Features | Requirements |
|---------------|-------------------------|----------|--------------|
| US-15.12.11.1 | designer (P-5)          |          |              |
| US-15.12.11.2 | DevOps engineer (P-16)  |          |              |
| US-15.12.11.3 | creative director (P-2) |          |              |
| US-15.12.11.4 | engine tester (P-27)    |          |              |

1. **US-15.12.11.1** — to join named work groups (e.g., "Level Design - Zone 3") with their own
   voice channel, text thread, and optionally isolated workspace layer
   - **Acceptance:** my edits are invisible to other groups until explicitly shared
2. **US-15.12.11.2** — work groups mapped to tasks, tickets, and branches
   - **Acceptance:** collaboration is traceable to project management items
3. **US-15.12.11.3** — to view isolated workspace layers from each group
   - **Acceptance:** I can review work-in-progress from different teams before it merges
4. **US-15.12.11.4** — to verify that edits in an isolated workspace layer are invisible to other
   groups until shared
   - **Acceptance:** group isolation works correctly

## F-15.12.12 AI Agent Collaboration

| ID            | Persona              | Features | Requirements |
|---------------|----------------------|----------|--------------|
| US-15.12.12.1 | designer (P-5)       |          |              |
| US-15.12.12.2 | tech artist (P-13)   |          |              |
| US-15.12.12.3 | developer (P-15)     |          |              |
| US-15.12.12.4 | engine tester (P-27) |          |              |

1. **US-15.12.12.1** — AI agents to participate in collaborative sessions as virtual users assigned
   to work groups
   - **Acceptance:** I can instruct an AI to populate a zone with vegetation while I work on another
     area
2. **US-15.12.12.2** — to see the AI agent's cursor, selections, and edits in real time
   - **Acceptance:** I can supervise and correct AI-driven changes as they happen
3. **US-15.12.12.3** — agent actions tagged with AI provenance
   - **Acceptance:** AI-created content is distinguished from human-created content in the
     collaboration history
4. **US-15.12.12.4** — to verify that AI agents are visible to all group members as virtual users
   with real-time presence
   - **Acceptance:** human collaborators are always aware of AI activity

## F-15.12.13 Asset and Scene Comments

| ID            | Persona                 | Features | Requirements |
|---------------|-------------------------|----------|--------------|
| US-15.12.13.1 | designer (P-5)          |          |              |
| US-15.12.13.2 | artist (P-8)            |          |              |
| US-15.12.13.3 | creative director (P-2) |          |              |
| US-15.12.13.4 | engine tester (P-27)    |          |              |

1. **US-15.12.13.1** — to attach threaded comments to any asset, entity, node, or spatial location
   - **Acceptance:** I can leave feedback for teammates directly in context
2. **US-15.12.13.2** — comments displayed as non-intrusive pins in the viewport and margin markers
   in editors
   - **Acceptance:** I see feedback without it obscuring my workspace
3. **US-15.12.13.3** — @mentions in comments that notify the referenced user
   - **Acceptance:** I can direct feedback to specific team members during reviews
4. **US-15.12.13.4** — to verify that comments sync across all connected clients and resolved
   comments are collapsed but not deleted
   - **Acceptance:** the comment system is reliable and preserves history

## F-15.12.14 Pull Request Review in Editor

| ID            | Persona                | Features | Requirements |
|---------------|------------------------|----------|--------------|
| US-15.12.14.1 | developer (P-15)       |          |              |
| US-15.12.14.2 | designer (P-5)         |          |              |
| US-15.12.14.3 | DevOps engineer (P-16) |          |              |
| US-15.12.14.4 | engine tester (P-27)   |          |              |

1. **US-15.12.14.1** — to view and review pull requests within the editor with structural diffs
   rendered in native visual editors (not text diffs)
   - **Acceptance:** I can review asset changes in their proper visual context
2. **US-15.12.14.2** — to add review comments on specific nodes, properties, or regions of changed
   assets
   - **Acceptance:** my feedback is precise and contextual
3. **US-15.12.14.3** — approve and request-changes actions to push status back to the Git hosting
   service via API
   - **Acceptance:** in-editor reviews are reflected in the hosting provider's PR workflow
4. **US-15.12.14.4** — to verify that PR review works with GitHub, GitLab, and Bitbucket APIs via
   the configurable provider layer
   - **Acceptance:** in-editor reviews work regardless of hosting choice
