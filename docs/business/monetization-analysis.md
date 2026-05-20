# Monetization and Open-Source Strategy Analysis

This document analyzes the business model for the Harmonius game engine. The entire engine is open
source under Apache 2.0, with revenue from proprietary console SDK addon licenses, enterprise
support contracts, and optional managed hosting.

## 0. Licensing Model

### Core Principles

| Principle                  |
|----------------------------|
| Engine license             |
| Royalties                  |
| Per-seat fees              |
| Open source asset store    |
| External store integration |
| Proprietary addons         |
| AI content generation      |

1. **Engine license** — 100% free, Apache 2.0, forever
2. **Royalties** — None -- developers keep 100% of game revenue
3. **Per-seat fees** — None -- team size does not affect cost
4. **Open source asset store** — Free, community-run, Git-based repository
5. **External store integration** — FAB, Synty, TurboSquid integration in-editor -- engine takes no
   commission
6. **Proprietary addons** — ONLY for console SDKs (PlayStation, Xbox, Nintendo) that cannot be
   open-sourced due to platform holder NDAs; sold with 24/7 support by the engine creator
7. **AI content generation** — Local inference only, no cloud fees, included with the engine

### Revenue Sources

| Source                       | Recurring |
|------------------------------|-----------|
| Console SDK addon licenses   | Yes       |
| Enterprise support contracts | Yes       |
| Managed hosting (optional)   | Yes       |

1. **Console SDK addon licenses** — Annual per-platform license for proprietary console backend
   integrations (PS5, Xbox, Switch)
2. **Enterprise support contracts** — Priority bug fixes, dedicated engineer, SLA guarantees for
   studios with shipping deadlines
3. **Managed hosting (optional)** — Convenience service for teams who prefer not to operate the
   collaboration, build cache, and mod infrastructure themselves on Kubernetes

### What We Do NOT Charge For

| Item                     |
|--------------------------|
| Engine (all components)  |
| Open source asset store  |
| External store purchases |
| AI content generation    |
| Self-hosting (Kubernetes)|
| All server services      |
| Marketplace commission   |

1. **Engine (all components)** — Apache 2.0 -- open source builds trust and maximizes adoption
2. **Open source asset store** — Community-run repository; free to browse, download, and publish
3. **External store purchases** — We take no commission on FAB, Synty, TurboSquid, or any
   third-party store purchases
4. **AI content generation** — Runs locally on user hardware; no cloud dependency, no per-generation
   fee
5. **Self-hosting (Kubernetes)** — Helm charts provided; user pays only their own Kubernetes
   infrastructure costs (any provider or bare metal)
6. **All server services** — Build cache, collaboration, matchmaking, asset store -- all open source
   and free to self-host
7. **Marketplace commission** — No built-in paid marketplace; the open source store is free and
   external stores handle their own payments

## 1. Component Classification

All engine components are open source under Apache 2.0. Developers can self-host every service using
provided deployment templates. The only proprietary components are console SDKs (due to platform NDA
requirements) and the optional managed hosting service.

### Classification Definitions

| Tier                            |
|---------------------------------|
| **Open source**                 |
| **Proprietary (NDA-required)**  |
| **Proprietary SaaS (optional)** |

1. ****Open source**** — Apache 2.0
   - **Who Sees Source:** Everyone
   - **Revenue Model:** Adoption driver; no direct revenue
2. ****Proprietary (NDA-required)**** — Proprietary binary
   - **Who Sees Source:** Console licensees
   - **Revenue Model:** Annual platform license
3. ****Proprietary SaaS (optional)**** — Closed; hosted service
   - **Who Sees Source:** N/A (managed service)
   - **Revenue Model:** Subscription for convenience

### Component Classification Matrix

| Classification              |
|-----------------------------|
| Open Source (Apache 2.0)    |
| Open Source (Apache 2.0)    |
| Open Source (Apache 2.0)    |
| Open Source (Apache 2.0)    |
| Open Source (Apache 2.0)    |
| Open Source (Apache 2.0)    |
| Open Source (Apache 2.0)    |
| Open Source (Apache 2.0)    |
| Open Source (Apache 2.0)    |
| Open Source (Apache 2.0)    |
| Open Source (Apache 2.0)    |
| Open Source (Apache 2.0)    |
| Open Source (Apache 2.0)    |
| Open Source (Apache 2.0)    |
| Open Source (Apache 2.0)    |
| Open Source (Apache 2.0)    |
| Open Source (Apache 2.0)    |
| Open Source (Apache 2.0)    |
| Open Source (Apache 2.0)    |
| Open Source (Apache 2.0)    |
| Open Source (Apache 2.0)    |
| Open Source (Apache 2.0)    |
| Open Source (Apache 2.0)    |
| Open Source (Apache 2.0)    |
| Proprietary (NDA-required)  |
| Proprietary SaaS (optional) |

1. **Core Runtime (ECS, serialization, events)** — Core Runtime (ECS, serialization, events)
   - **Rationale:** Foundation of every game; open source builds trust and enables modding.
2. **Rendering (GPU abstraction, render graph)** — Rendering (GPU abstraction, render graph)
   - **Rationale:** Developers need to inspect, debug, and extend rendering for custom shaders.
3. **Physics** — Physics
   - **Rationale:** Gameplay-critical; studios need to tune and extend for deterministic replay.
4. **Audio** — Audio
   - **Rationale:** Basic audio is table-stakes; open source allows custom DSP pipelines.
5. **Input** — Input
   - **Rationale:** Device support must be extensible for VR, accessibility, and custom hardware.
6. **AI (navigation, behavior trees)** — AI (navigation, behavior trees)
   - **Rationale:** Gameplay AI must be inspectable. Community contributes steering, GOAP, utility
     AI.
7. **Networking (transport, replication)** — Networking (transport, replication)
   - **Rationale:** Multiplayer is a core differentiator; open source enables community trust and
     auditing.
8. **Animation** — Animation
   - **Rationale:** Animation blending, IK, and state machines need studio customization.
9. **UI framework** — UI framework
   - **Rationale:** Every game has custom UI; open source enables deep theming and accessibility.
10. **VFX (particles, effects)** — VFX (particles, effects)
    - **Rationale:** Artists and tech artists need to extend particle systems for community effect
      libraries.
11. **Content Pipeline (import, processing)** — Content Pipeline (import, processing)
    - **Rationale:** Import pipeline depends on open formats. Community contributes importers.
12. **Game Framework (gameplay systems)** — Game Framework (gameplay systems)
    - **Rationale:** Combat, progression, and camera systems must be customizable per genre.
13. **Platform (windowing, OS)** — Platform (windowing, OS)
    - **Rationale:** Platform abstraction must be portable across all supported operating systems.
14. **Editor (scene, material editors)** — Editor (scene, material editors)
    - **Rationale:** Full editor source enables deep customization and community-driven extensions.
15. **Logic Graph system** — Logic Graph system
    - **Rationale:** Core no-code promise; open source enables custom node authoring by the
      community.
16. **Collaboration service (CRDT, cloud)** — Collaboration service (CRDT, cloud)
    - **Rationale:** Self-hosted on Kubernetes (TiKV + Garage + Pingora). Managed hosting available
      for convenience.
17. **Shared build cache** — Shared build cache
    - **Rationale:** Self-hosted on Kubernetes (Garage for blobs, TiKV for index). Managed hosting
      available for convenience.
18. **Matchmaking service** — Matchmaking service
    - **Rationale:** Self-hosted on Kubernetes via the matchmaker microservice
      ([network-infrastructure.md](../design/networking/network-infrastructure.md)). Managed
      hosting available for convenience.
19. **Asset marketplace** — Asset marketplace
    - **Rationale:** Self-hosted on Kubernetes (TiKV + Garage). Managed hosting available for
      convenience.
20. **Universe generation server** — Universe generation server
    - **Rationale:** Self-hosted on Kubernetes. Managed hosting available for convenience.
21. **Mod hosting and moderation** — Mod hosting and moderation
    - **Rationale:** Self-hosted on Kubernetes (Garage for blobs). Managed hosting available for
      convenience.
22. **AI assistant (LLM-based)** — AI assistant (LLM-based)
    - **Rationale:** Local inference shipped with the engine. Cloud LLM access uses
      customer-owned API keys (no Harmonius proxy). See PDR-0002.
23. **Deployment configuration** — Deployment configuration
    - **Rationale:** Helm chart bundling TiKV, Garage, Pingora, Vector, Prometheus, Grafana, Loki,
      cert-manager, Sealed Secrets. Runs on EKS, GKE, AKS, k3s, or bare metal.
24. **Monitoring stack** — Monitoring stack
    - **Rationale:** Vector + Prometheus + Grafana + Loki. Self-hosted on Kubernetes. No
      proprietary cloud monitoring required.
25. **Console backends (PS5, Xbox, Switch)** — Console backends (PS5, Xbox, Switch)
    - **Rationale:** Platform holder NDAs prohibit open-sourcing console SDKs.
26. **Managed hosting service** — Managed hosting service
    - **Rationale:** Convenience service for teams who prefer not to operate Kubernetes themselves.

### Open-Source Service Dependencies

All self-hosted services use exclusively open-source software. The canonical OSS stack lives in
[design/networking/network-infrastructure.md](../design/networking/network-infrastructure.md)
RF-6 and is referenced normatively by ADR-0009. There is no AWS-specific dependency in the
default stack — any Kubernetes substrate works.

| Service Need        | Open-Source Choice                  | Proprietary Avoided                |
|---------------------|-------------------------------------|------------------------------------|
| Database            | TiKV                                | RDS, Aurora, DynamoDB              |
| Cache / sessions    | TiKV (TTL keys)                     | Redis on ElastiCache               |
| Auth                | Keycloak (Helm)                     | Cognito                            |
| Message queue       | NATS (Helm)                         | SQS, SNS                           |
| Object storage      | Garage (S3-compatible)              | S3, MinIO Enterprise               |
| Search              | App-layer indexes on TiKV           | CloudSearch, OpenSearch            |
| CDN / reverse proxy | Pingora (Rust)                      | CloudFront, CloudFlare             |
| Logs                | Vector → Loki                       | CloudWatch Logs                    |
| Metrics             | Prometheus + Grafana                | CloudWatch Metrics                 |
| Git hosting         | Forgejo                             | CodeCommit, GitHub Enterprise      |
| CI/CD               | Forgejo Actions                     | CodePipeline, CodeBuild            |
| Orchestration       | Kubernetes + custom kube-rs operator| ArgoCD, Spinnaker                  |
| Secrets             | Sealed Secrets / cert-manager       | KMS, Secrets Manager               |

### Classification Summary

| Tier | Count | Components |
|------|-------|------------|
| Open Source (Apache 2.0) | 24 | All engine, editor, server, monitoring, and tooling components |
| Proprietary (NDA-required) | 1 | Console backends (PS5, Xbox, Switch SDKs) |
| Proprietary SaaS (optional) | 1 | Managed hosting service |

Open-sourcing 24 of 26 components (92%) under Apache 2.0 maximizes adoption and community trust. The
only proprietary components exist due to legal constraints (console NDAs) or as an optional
convenience service (managed hosting).

## 2. Pricing Model

### Pricing Structure

| Aspect                       | Price                |
|------------------------------|----------------------|
| Engine (all components)      | Free                 |
| Self-hosting (Kubernetes)    | Free (user pays infra) |
| Managed hosting              | $29/user/month       |
| Enterprise support           | $10,000-$50,000/year |
| Console SDK license (PS5)    | $10,000/year         |
| Console SDK license (Xbox)   | $10,000/year         |
| Console SDK license (Switch) | $10,000/year         |
| Open source asset store      | Free                 |
| External store integration   | Free                 |
| Education and non-commercial | Free                 |

1. **Engine (all components)** — Apache 2.0, no royalties, no per-seat fees
2. **Self-hosting (Kubernetes)** — Helm chart provided for all services (any K8s substrate)
3. **Managed hosting** — Optional; for teams who prefer not to self-host
4. **Enterprise support** — Priority bug fixes, dedicated engineer, SLA
5. **Console SDK license (PS5)** — Covers NDA compliance, 24/7 support, and platform updates
6. **Console SDK license (Xbox)** — Covers NDA compliance, 24/7 support, and platform updates
7. **Console SDK license (Switch)** — Covers NDA compliance, 24/7 support, and platform updates
8. **Open source asset store** — Community-run; no fees to browse, download, or publish
9. **External store integration** — FAB, Synty, TurboSquid; engine takes no commission
10. **Education and non-commercial** — All components, including managed hosting trial

### Key Pricing Principles

1. **No royalties** -- Developers keep 100% of game revenue regardless of sales volume.
2. **No per-seat fees** -- Team size does not affect engine cost.
3. **Self-hosting is always free** -- Managed hosting is a convenience, not a requirement.
4. **Console SDKs are the only proprietary addons** -- Required only for console shipping; includes
   24/7 support.
5. **No marketplace commission** -- The open source asset store is free; external store integrations
   take no cut.
6. **AI content generation is free** -- Local inference runs on user hardware with no cloud fees.

### Self-Hosted Scaling Profiles (Kubernetes Footprint)

All services are free and open source. Users pay only their own Kubernetes infrastructure
costs (any provider, any region, or bare metal). Four profiles match team size and budget.

| Profile    | Target       | K8s footprint (rough)                               | HA AZs |
|------------|--------------|------------------------------------------------------|--------|
| Solo       | 1 user       | k3s on a single small VM (1–2 vCPU, 4 GiB RAM)      | 1      |
| Team       | 2–10 users   | 3-node general-purpose cluster                       | 1      |
| Studio     | 10–50 users  | 3+ node cluster, dedicated TiKV PD/store node pool   | 2      |
| Production | 50+ users    | Multi-zone cluster with horizontal scaling           | 3      |

The dollar cost depends on the chosen provider (cloud Kubernetes, managed Kubernetes, on-prem,
bare metal) and is no longer modeled here. The Helm chart and operator are identical across
substrates per
[network-infrastructure.md](../design/networking/network-infrastructure.md) RF-6.

### Comparison: Self-Hosted vs. Managed Hosting

| Aspect                        | Self-host                              | Managed                              |
|-------------------------------|----------------------------------------|--------------------------------------|
| Setup effort                  | `helm install harmonius-server`       | Zero; provisioned per organization   |
| Recurring fee                 | Substrate cost only                    | $29/user/month                        |
| Dependencies                  | TiKV, Garage, Pingora, Vector, Loki   | Fully managed                        |
| Maintenance                   | Operator-driven; Harmonius community  | Fully managed                        |
| Data sovereignty              | Full control                           | US/EU region selection                |
| Customization                 | Full (modify charts, swap services)    | Standard configuration                |
| Portability                   | Any K8s substrate                      | Locked to Harmonius managed service  |
| SLA                           | Self-defined                           | 99.9% uptime guarantee                |

For solo developers, the Solo profile (k3s on a small VM) typically costs less than managed
hosting. For larger teams, self-hosting scales more cost-effectively. Managed hosting remains
attractive for teams that want zero operational overhead.

## 3. Per-User Server Costs (Managed Hosting)

The previous version of this section modeled per-user managed-hosting costs using AWS US-East
list prices (RDS, ECS, S3, CloudFront, Lambda, DynamoDB). That model conflicts with the current
OSS stack. A new per-user cost model rebased on Kubernetes node sizing plus the OSS bundle
(TiKV, Garage, Pingora, Vector, Prometheus, Grafana, Loki) is tracked under
[BL-0044](../backlog/issues/BL-0044-thin-requirement-files.md) and the
[2026-Q3 OKRs](../okrs/2026-q3.md). Until then, the published managed-hosting price ($29 / user /
month) is the only canonical figure; underlying cost composition is treated as commercially
sensitive.

The shape of the model still applies:

| Service                          | Cost driver                                            |
|----------------------------------|--------------------------------------------------------|
| Collaboration (CRDT)             | TiKV writes + WebSocket pods                           |
| Shared build cache               | Garage egress + TiKV index                             |
| Asset marketplace                | Garage storage + TiKV reads + Pingora egress          |
| Universe generation              | GPU node-hour bucket                                   |
| Mod hosting                      | Garage egress + content-mod inference                  |
| AI assistant                     | Customer-owned API keys (no engine proxy, see PDR-0002)|
| Crash reporting                  | Garage uploads + Vector ingestion                      |

## 4. Revenue Projections

### Adoption Model Assumptions

Most users self-host for free. A minority opt for managed hosting for convenience.

| Year   |
|--------|
| Year 1 |
| Year 2 |
| Year 3 |
| Year 5 |

1. **Year 1** — 5,000
   - **Active monthly devs:** 2,000
   - **Managed hosting users (% of active):** 200 (10%)
   - **Console licensees:** 5
2. **Year 2** — 20,000
   - **Active monthly devs:** 8,000
   - **Managed hosting users (% of active):** 1,200 (15%)
   - **Console licensees:** 20
3. **Year 3** — 50,000
   - **Active monthly devs:** 20,000
   - **Managed hosting users (% of active):** 3,000 (15%)
   - **Console licensees:** 50
4. **Year 5** — 150,000
   - **Active monthly devs:** 60,000
   - **Managed hosting users (% of active):** 12,000 (20%)
   - **Console licensees:** 150

### Revenue by Stream (Annual)

#### Year 1 (Launch)

| Revenue stream | Calculation | Annual revenue |
|----------------|-------------|----------------|
| Managed hosting | 200 users x $29/mo x 12 | $69,600 |
| Enterprise support | 2 contracts x $15K avg | $30,000 |
| Console SDK licenses | 5 studios x 1.5 platforms avg x $10K | $75,000 |
| **Total revenue** | | **$174,600** |

#### Year 2 (Growth)

| Revenue stream | Calculation | Annual revenue |
|----------------|-------------|----------------|
| Managed hosting | 1,200 users x $29/mo x 12 | $417,600 |
| Enterprise support | 8 contracts x $20K avg | $160,000 |
| Console SDK licenses | 20 studios x 1.8 platforms avg x $10K | $360,000 |
| **Total revenue** | | **$937,600** |

#### Year 3 (Traction)

| Revenue stream | Calculation | Annual revenue |
|----------------|-------------|----------------|
| Managed hosting | 3,000 users x $29/mo x 12 | $1,044,000 |
| Enterprise support | 20 contracts x $25K avg | $500,000 |
| Console SDK licenses | 50 studios x 2 platforms avg x $10K | $1,000,000 |
| **Total revenue** | | **$2,544,000** |

#### Year 5 (Maturity)

| Revenue stream | Calculation | Annual revenue |
|----------------|-------------|----------------|
| Managed hosting | 12,000 users x $29/mo x 12 | $4,176,000 |
| Enterprise support | 50 contracts x $30K avg | $1,500,000 |
| Console SDK licenses | 150 studios x 2.2 platforms avg x $10K | $3,300,000 |
| **Total revenue** | | **$8,976,000** |

### Revenue Mix at Maturity (Year 5)

| Revenue stream | Annual revenue | % of total |
|----------------|---------------|------------|
| Managed hosting | $4,176,000 | 46.5% |
| Console SDK licenses | $3,300,000 | 36.8% |
| Enterprise support | $1,500,000 | 16.7% |
| **Total** | **$8,976,000** | **100%** |

### Cost Model (Annual, at Maturity Year 5)

| Cost item | Calculation | Annual cost |
|-----------|-------------|-------------|
| Managed hosting infrastructure | 12,000 users x $21.02/mo x 12 | $3,026,880 |
| Engineering team (25 engineers) | 25 x $200K fully loaded | $5,000,000 |
| Community and DevRel (5 staff) | 5 x $150K fully loaded | $750,000 |
| Operations and overhead | Estimate | $500,000 |
| **Total cost** | | **$9,276,880** |
| **Net income** | | **-$300,880** |

At maturity with 25 engineers, the model is roughly break-even. Profitability requires either higher
managed hosting adoption, more console licensees, or a smaller engineering team during the growth
phase.

### Break-Even Analysis

| Team size    | Annual cost | Additional revenue needed         |
|--------------|-------------|-----------------------------------|
| 10 engineers | $2,500,000  | $1,308,560 from support + console |
| 25 engineers | $6,250,000  | $4,058,560 from support + console |
| 50 engineers | $12,500,000 | $8,558,560 from support + console |

1. **10 engineers** — 2,000
2. **25 engineers** — 5,000
3. **50 engineers** — 10,000

Managed hosting alone cannot sustain a large team. Console SDK licenses and enterprise support are
the critical secondary revenue streams. Without a paid marketplace commission, the model depends
more heavily on console SDK adoption and enterprise support contracts.

## 5. Competitive Comparison

| Aspect           | Harmonius                                                                 |
|------------------|---------------------------------------------------------------------------|
| Source access    | Fully open source (Apache 2.0)                                            |
| Royalties        | None                                                                      |
| Engine cost      | Free forever                                                              |
| Cloud services   | TiKV + Garage + Pingora + Vector + Prometheus + Grafana + Loki on K8s     |
| Multiplayer      | Built-in QUIC transport + replication                                     |
| Collaboration    | Built-in CRDT sync (open source)                                          |
| Build cache      | Built-in shared cache (open source)                                       |
| Marketplace      | None (no commission); third-party stores keep their own fees             |
| Console support  | $10K/yr per platform                                                      |
| No-code tools    | Logic Graph (open source)                                                 |
| AI assistant     | Local inference + customer-owned cloud LLM keys (PDR-0002)                |
| Mod support      | Built-in hosting (open source)                                            |
| Deployment infra | Kubernetes Helm chart (any K8s substrate)                                 |
| Matchmaking      | Built-in OSS matchmaker microservice (TiKV + custom kube-rs operator)     |

| Aspect           |
|------------------|
| Source access    |
| Royalties        |
| Engine cost      |
| Cloud services   |
| Multiplayer      |
| Collaboration    |
| Build cache      |
| Marketplace      |
| Console support  |
| No-code tools    |
| AI assistant     |
| Mod support      |
| Deployment infra |
| Matchmaking      |

1. **Source access** — Source-available (custom EULA)
   - **Unity:** Closed runtime
   - **Godot:** Fully open source (MIT)
   - **O3DE:** Fully open source (Apache 2.0)
2. **Royalties** — 5% above $1M (3.5% with EGS)
   - **Unity:** None (subscription model)
3. **Engine cost** — Free under $1M revenue
   - **Unity:** $2,200/seat/yr (Pro)
   - **Godot:** Free forever
   - **O3DE:** Free forever
4. **Cloud services** — None built-in
   - **Unity:** Unity Cloud (proprietary)
5. **Multiplayer** — Basic; use third-party
   - **Unity:** Netcode package
   - **Godot:** High-level only
   - **O3DE:** Basic
6. **Collaboration**
   - **Unity:** Unity DevOps (proprietary)
7. **Build cache**
   - **Unity:** Unity Accelerator (proprietary)
8. **Marketplace** — Fab: 12% commission
   - **Unity:** Asset Store: 30% commission
   - **Godot:** Community library (free)
9. **Console support** — Free with license
   - **Unity:** Enterprise tier
   - **Godot:** Community ports (limited)
   - **O3DE:** Experimental
10. **No-code tools** — Blueprints (source-available)
    - **Unity:** Visual Scripting (deprecated)
    - **Godot:** VisualScript
    - **O3DE:** Script Canvas
11. **AI assistant** — None built-in
    - **Unity:** Muse AI (limited)
12. **Mod support** — Limited (game-specific)
    - **Unity:** Limited (game-specific)
    - **Godot:** None built-in
    - **O3DE:** None built-in
13. **Deployment infra**
    - **Unity:** Unity Cloud (proprietary)
14. **Matchmaking** — None built-in
    - **Unity:** None built-in

### Key Differentiators

1. **Fully open source with no royalties** -- Unlike Unreal (5% royalty, source-available only) or
   Unity (closed source, per-seat fees), Harmonius is Apache 2.0 with zero royalties. Developers
   keep 100% of game revenue.
2. **Built-in multiplayer and collaboration infrastructure** -- No other open-source engine ships
   with production-ready networking, CRDT collaboration, shared build cache, matchmaking, and
   deployment infrastructure. Godot lacks these entirely; Unreal requires third-party solutions.
3. **Self-hostable everything with open-source deps** -- Every server component uses open-source
   dependencies (TiKV, Garage, Pingora, Vector, Prometheus, Grafana, Loki, Keycloak, Forgejo).
   Teams can migrate to any cloud or on-premises Kubernetes deployment. Unlike Unity Cloud, there
   is zero vendor lock-in.
4. **One-command deployment** -- `helm install harmonius-server` plus a custom kube-rs operator
   handles game-aware deployment, canary validation, and graceful player drain. Solo developers
   can run the full stack on a single small VM via k3s. No DevOps expertise required.
5. **AAA-grade features in an open-source engine** -- Harmonius bridges the gap between Godot (open
   source but limited feature set) and Unreal (AAA features but proprietary with royalties).
   Features like GPU-driven rendering, ray tracing, advanced physics, and universe generation exceed
   what Godot offers.
6. **No vendor lock-in** -- Apache 2.0 license guarantees perpetual access. No pricing changes, no
   retroactive terms, no trust erosion. The Unity pricing crisis of 2023 cannot happen. All server
   dependencies are portable open-source software.

### Competitive Positioning Matrix

| Need                         | Best choice             |
|------------------------------|-------------------------|
| AAA graphics, zero royalties | **Harmonius**           |
| Full source access + modify  | **Harmonius** or Godot  |
| Built-in multiplayer infra   | **Harmonius**           |
| No-code development          | **Harmonius** or Unreal |
| Indie with no budget         | Godot or **Harmonius**  |
| Enterprise with SLA needs    | **Harmonius** or Unreal |

1. **AAA graphics, zero royalties** — Unreal charges 5%; Godot lacks AAA features
2. **Full source access + modify** — Unreal is source-available only (cannot redistribute)
3. **Built-in multiplayer infra** — Only engine with open-source transport, replication, and
   deployment
4. **No-code development** — Harmonius Logic Graph is open source; Blueprints is source-available
5. **Indie with no budget** — Both free; Harmonius adds cloud infra if self-hosted
6. **Enterprise with SLA needs** — Harmonius offers dedicated engineer support contracts

## 6. Risks and Mitigations

### Risk 1: Insufficient Revenue from Optional Services

| Aspect           |
|------------------|
| **Risk**         |
| **Severity**     |
| **Trigger**      |
| **Mitigation 1** |
| **Mitigation 2** |
| **Mitigation 3** |
| **Mitigation 4** |

1. ****Risk**** — With no royalties and a fully open-source engine, revenue depends entirely on
   optional services that most users may skip.
2. ****Severity**** — High -- primary business risk of the open-source model.
3. ****Trigger**** — Managed hosting adoption below 10%; enterprise support demand lower than
   projected.
4. ****Mitigation 1**** — Console SDK licenses provide baseline revenue independent of hosting
   adoption.
5. ****Mitigation 2**** — Managed hosting is priced competitively vs. self-hosting for small teams,
   driving organic adoption.
6. ****Mitigation 3**** — Enterprise support includes priority bug fixes and SLA, creating clear
   value for studios with shipping deadlines.
7. ****Mitigation 4**** — Asset marketplace acts as ecosystem flywheel for managed hosting and
   enterprise support adoption rather than a direct revenue line; the engine takes no commission
   (per PDR-0001).

### Risk 2: Open-Source Fork Competing

| Aspect           |
|------------------|
| **Risk**         |
| **Severity**     |
| **Trigger**      |
| **Mitigation 1** |
| **Mitigation 2** |
| **Mitigation 3** |
| **Mitigation 4** |

1. ****Risk**** — A well-funded competitor forks the entire engine (including server components) and
   offers competing managed hosting.
2. ****Severity**** — High -- all components are open source, so forking is legally permitted.
3. ****Trigger**** — Engine reaches critical mass; a major company (Amazon, Google) forks and builds
   competing SaaS.
4. ****Mitigation 1**** — Maintain development velocity: ship features faster than any fork.
   Harmonius team has full context advantage.
5. ****Mitigation 2**** — Build marketplace network effects: asset sellers, buyers, and mod
   communities create ecosystem lock-in that a fork cannot replicate overnight.
6. ****Mitigation 3**** — Cultivate strong community relationships. Forks succeed when upstream
   maintainers alienate contributors. Invest heavily in community governance and contributor
   recognition.
7. ****Mitigation 4**** — Console SDK licenses are non-forkable (NDA-bound), providing a revenue
   floor that forks cannot undercut.

### Risk 3: Cloud Service Costs Exceeding Revenue

| Aspect           |
|------------------|
| **Risk**         |
| **Severity**     |
| **Trigger**      |
| **Mitigation 1** |
| **Mitigation 2** |
| **Mitigation 3** |
| **Mitigation 4** |

1. ****Risk**** — Managed hosting margin depends on the per-user K8s footprint plus egress
   from Garage and Pingora. Unexpected usage spikes or substrate price increases could compress
   margin.
2. ****Severity**** — Medium -- manageable with monitoring and committed-use discounts where
   available.
3. ****Trigger**** — Viral adoption spike; underestimated egress costs; GPU compute price
   increases.
4. ****Mitigation 1**** — Use committed-use discounts on managed-Kubernetes substrates;
   alternatively run dedicated bare-metal nodes for predictable cost.
5. ****Mitigation 2**** — Usage-based overages for bandwidth-heavy users rather than flat-rate
   pricing.
6. ****Mitigation 3**** — Monitor cost-per-user weekly; adjust pricing quarterly with 90 days
   notice.
7. ****Mitigation 4**** — Build cache (Garage) and CDN (Pingora) costs scale sub-linearly with
   deduplication and intelligent caching.

### Risk 4: Marketplace Not Achieving Critical Mass

| Aspect           |
|------------------|
| **Risk**         |
| **Severity**     |
| **Trigger**      |
| **Mitigation 1** |
| **Mitigation 2** |
| **Mitigation 3** |
| **Mitigation 4** |

1. ****Risk**** — Without enough sellers, buyers don't visit. Without buyers, sellers don't publish.
2. ****Severity**** — Medium -- marketplace is a growing but not primary revenue source.
3. ****Trigger**** — Launch with fewer than 500 assets; competing marketplaces (Fab, Sketchfab)
   dominate.
4. ****Mitigation 1**** — Seed the marketplace with 200+ first-party assets (materials, models,
   templates) at launch.
5. ****Mitigation 2**** — Offer 95/5 revenue split for first 12 months to attract early sellers (vs.
   standard 88/12).
6. ****Mitigation 3**** — Integrate marketplace deeply into editor: one-click import, live preview,
   dependency resolution.
7. ****Mitigation 4**** — Support cross-engine formats so sellers from Unity/Unreal can publish with
   minimal friction.

### Risk 5: Console SDK Revenue Concentration

| Aspect           |
|------------------|
| **Risk**         |
| **Severity**     |
| **Trigger**      |
| **Mitigation 1** |
| **Mitigation 2** |
| **Mitigation 3** |

1. ****Risk**** — Console SDK licenses represent 29% of mature revenue but depend on platform holder
   relationships.
2. ****Severity**** — Medium -- console holders could change NDA terms or build competing
   integrations.
3. ****Trigger**** — Platform holder builds first-party Harmonius support; NDA terms become more
   restrictive.
4. ****Mitigation 1**** — Diversify revenue so no single stream exceeds 35% of total.
5. ****Mitigation 2**** — Build strong relationships with platform holder developer relations teams.
6. ****Mitigation 3**** — Ensure console SDK quality is best-in-class to justify the license fee.

### Risk Summary Matrix

| Risk | Severity | Likelihood | Impact | Primary mitigation |
|------|----------|------------|--------|--------------------|
| Insufficient revenue | High | Medium | Cash flow crisis | Diversified revenue streams |
| Fork competition | High | Medium | Revenue erosion | Velocity + community + console lock-in |
| Cost overrun | Medium | Medium | Margin compression | Reserved instances + monitoring |
| Marketplace cold start | Medium | High | Slow revenue growth | Seed content + seller incentives |
| Console revenue concentration | Medium | Low | Revenue loss | Revenue diversification |

## Sources

- [Epic Games reduces royalty to 3.5%](https://www.cgchannel.com/2024/10/epic-games-to-cut-royalty-rate-on-unreal-engine-games/)
- [Unreal Engine licensing](https://www.unrealengine.com/en-US/license)
- [Unity cancels Runtime Fee](https://www.gamedeveloper.com/business/unity-is-killing-its-controversial-runtime-fee)
- [Unity pricing updates](https://unity.com/products/pricing-updates)
- [W4 Games raises $15M for Godot](https://www.gamedeveloper.com/production/w4-games-nets-15-million-to-help-godot-scale-exponentially)
- [Godot Foundation](https://godot.foundation/)
- [CryEngine licensing](https://www.cryengine.com/support/view/licensing)
- [O3DE open-source announcement](https://www.theregister.com/2021/07/07/open_3d_engine/)
- [Epic Marketplace 88/12 split](https://www.unrealengine.com/en-US/blog/epic-announces-unreal-engine-marketplace-88-12-revenue-share)
- [TiKV documentation](https://tikv.org/docs/)
- [Garage S3-compatible object storage](https://garagehq.deuxfleurs.fr/)
- [Pingora reverse proxy](https://github.com/cloudflare/pingora)
- [Vector observability pipeline](https://vector.dev/)
- [Loki log aggregation](https://grafana.com/oss/loki/)
- [Helm package manager](https://helm.sh/)
- [LLM API pricing comparison 2025](https://intuitionlabs.ai/articles/llm-api-pricing-comparison-2025)
