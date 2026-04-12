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
3. **Managed hosting (optional)** — Convenience service for teams who prefer not to self-host
   collaboration, build cache, and mod infrastructure on AWS

### What We Do NOT Charge For

| Item                     |
|--------------------------|
| Engine (all components)  |
| Open source asset store  |
| External store purchases |
| AI content generation    |
| Self-hosting (AWS)       |
| All server services      |
| Marketplace commission   |

1. **Engine (all components)** — Apache 2.0 -- open source builds trust and maximizes adoption
2. **Open source asset store** — Community-run repository; free to browse, download, and publish
3. **External store purchases** — We take no commission on FAB, Synty, TurboSquid, or any
   third-party store purchases
4. **AI content generation** — Runs locally on user hardware; no cloud dependency, no per-generation
   fee
5. **Self-hosting (AWS)** — Deployment templates provided; user pays only AWS infrastructure costs
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
    - **Rationale:** Self-hosted on AWS with open-source deps (PostgreSQL, Redis, NATS). Managed
      hosting available for convenience.
17. **Shared build cache** — Shared build cache
    - **Rationale:** Self-hosted on AWS with S3-compatible storage and Redis index. Managed hosting
      available for convenience.
18. **Matchmaking service** — Matchmaking service
    - **Rationale:** Self-hosted on AWS with PostgreSQL, Redis, and NATS. Managed hosting available
      for convenience.
19. **Asset marketplace** — Asset marketplace
    - **Rationale:** Self-hosted on AWS with PostgreSQL, OpenSearch, and S3. Managed hosting
      available for convenience.
20. **Universe generation server** — Universe generation server
    - **Rationale:** Self-hosted on AWS. Managed hosting available for convenience.
21. **Mod hosting and moderation** — Mod hosting and moderation
    - **Rationale:** Self-hosted on AWS. Managed hosting available for convenience.
22. **AI assistant (LLM-based)** — AI assistant (LLM-based)
    - **Rationale:** Self-hosted with user-provided LLM API keys. Managed hosting available.
23. **Deployment configuration** — Deployment configuration
    - **Rationale:** Infrastructure templates using exclusively open-source dependencies. Free on
      AWS Marketplace.
24. **Monitoring stack** — Monitoring stack
    - **Rationale:** Prometheus + Grafana + Loki. Self-hosted on AWS. No CloudWatch dependency.
25. **Console backends (PS5, Xbox, Switch)** — Console backends (PS5, Xbox, Switch)
    - **Rationale:** Platform holder NDAs prohibit open-sourcing console SDKs.
26. **Managed hosting service** — Managed hosting service
    - **Rationale:** Convenience service for teams who prefer not to self-host on AWS.

### Open-Source Service Dependencies

All self-hosted services use exclusively open-source software. No proprietary AWS services beyond
S3, EC2, ECS, and RDS for PostgreSQL.

| Service Need | Open-Source Choice | Proprietary Avoided |
|--------------|--------------------|--------------------|
| Database | PostgreSQL (RDS) | Aurora, DynamoDB |
| Cache | Redis / Valkey (ElastiCache) | Proprietary ElastiCache protocol |
| Auth | Keycloak (ECS) | Cognito |
| Message queue | NATS (ECS) | SQS, SNS |
| Object storage | S3-compatible API | (S3 retained -- standard API, MinIO-compatible) |
| Search | OpenSearch | CloudSearch |
| Monitoring | Grafana + Prometheus + Loki (ECS) | CloudWatch |
| Git hosting | Forgejo (ECS/EC2) | CodeCommit |
| CI/CD | Forgejo Actions | CodePipeline, CodeBuild |

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
| Self-hosting (AWS)           | Free (user pays AWS) |
| Managed hosting              | $29/user/month       |
| Enterprise support           | $10,000-$50,000/year |
| Console SDK license (PS5)    | $10,000/year         |
| Console SDK license (Xbox)   | $10,000/year         |
| Console SDK license (Switch) | $10,000/year         |
| Open source asset store      | Free                 |
| External store integration   | Free                 |
| Education and non-commercial | Free                 |

1. **Engine (all components)** — Apache 2.0, no royalties, no per-seat fees
2. **Self-hosting (AWS)** — Deployment templates provided for all services
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

### Self-Hosted Scaling Profiles (AWS Infrastructure Cost)

All services are free and open source. Users pay only AWS infrastructure costs. Four profiles match
team size and budget.

| Profile | Target | Est. Monthly Cost | Instance Sizes | AZs |
|---------|--------|-------------------|---------------|-----|
| Solo | 1 user | ~$20 | t4g.micro | 1 |
| Team | 2-10 users | ~$100 | t4g.small | 1 |
| Studio | 10-50 users | ~$500 | t4g.medium / m6g.large | 2 |
| Production | 50+ users | ~$2,000+ | m6g.xlarge+, auto-scaling | 3 |

### Comparison: Self-Hosted vs. Managed Hosting

| Aspect                        | Managed hosting ($29/user/mo)       |
|-------------------------------|-------------------------------------|
| Setup effort                  | Zero; instant provisioning          |
| Monthly cost (solo)           | $29                                 |
| Monthly cost (10-person team) | $290                                |
| Monthly cost (50-person team) | $1,450                              |
| Dependencies                  | Fully managed by Harmonius          |
| Maintenance                   | Fully managed by Harmonius          |
| Data sovereignty              | US/EU region selection              |
| Customization                 | Standard configuration only         |
| Portability                   | Locked to Harmonius managed service |
| SLA                           | 99.9% uptime guarantee              |

1. **Setup effort** — 1-click Marketplace or deployment CLI
2. **Monthly cost (solo)** — ~$20 (Solo profile)
3. **Monthly cost (10-person team)** — ~$100 (Team profile)
4. **Monthly cost (50-person team)** — ~$500 (Studio profile)
5. **Dependencies** — All open source (PostgreSQL, Redis, NATS, Keycloak)
6. **Maintenance** — Team manages updates, scaling
7. **Data sovereignty** — Full control over data location
8. **Customization** — Full (modify any service, swap deps)
9. **Portability** — Migrate to any cloud or on-prem (MinIO, bare metal)
10. **SLA** — Per AWS SLA

For solo developers, self-hosting on the Solo profile
(~$20/mo) is cheaper than managed hosting ($29/mo). For larger teams, self-hosting scales more
cost-effectively. Managed hosting remains attractive for teams that want zero operational overhead.

## 3. Per-User Server Costs (Managed Hosting)

### Cost Estimates per Cloud Service

All estimates use AWS US-East pricing as of 2025. Costs are per active user per month unless noted
otherwise. These costs apply only to the managed hosting service.

#### Collaboration Service (CRDT Sync)

| Resource | Unit cost | Usage per user/month | Monthly cost |
|----------|-----------|---------------------|--------------|
| PostgreSQL RDS (db.t4g.small, shared) | $23/mo per instance | 1/100th instance | $0.23 |
| S3 storage (project data) | $0.023/GB | 5 GB average | $0.12 |
| EC2 WebSocket server (t4g.medium, shared) | $48/mo per instance | 1/50th instance | $0.96 |
| Data transfer (CRDT sync) | $0.09/GB | 2 GB | $0.18 |
| **Subtotal** | | | **$1.49** |

#### Shared Build Cache

| Resource | Unit cost | Usage per user/month | Monthly cost |
|----------|-----------|---------------------|--------------|
| S3 storage (build artifacts) | $0.023/GB | 50 GB average | $1.15 |
| CloudFront bandwidth (cache hits) | $0.085/GB | 100 GB | $8.50 |
| S3 GET requests | $0.0004/1000 | 500,000 requests | $0.20 |
| **Subtotal** | | | **$9.85** |

#### Asset Marketplace

| Resource | Unit cost | Usage per user/month | Monthly cost |
|----------|-----------|---------------------|--------------|
| API server (shared Lambda) | $0.20/1M requests | 10,000 requests | $0.002 |
| S3 storage (asset hosting) | $0.023/GB | 1 GB (amortized) | $0.02 |
| CloudFront (asset downloads) | $0.085/GB | 5 GB | $0.43 |
| Search (OpenSearch, shared) | $150/mo per instance | 1/500th instance | $0.30 |
| **Subtotal** | | | **$0.75** |

#### Universe Generation Server

| Resource | Unit cost | Usage per user/month | Monthly cost |
|----------|-----------|---------------------|--------------|
| GPU compute (g5.xlarge) | $1.006/hr | 2 hours/month | $2.01 |
| Result storage (S3) | $0.023/GB | 10 GB | $0.23 |
| Database (DynamoDB) | $1.25/WCU | 5 WCU (shared) | $0.01 |
| **Subtotal** | | | **$2.25** |

#### Mod Hosting and Moderation

| Resource | Unit cost | Usage per user/month | Monthly cost |
|----------|-----------|---------------------|--------------|
| S3 storage (mod files) | $0.023/GB | 20 GB | $0.46 |
| CloudFront (mod downloads) | $0.085/GB | 50 GB | $4.25 |
| Moderation compute (Lambda + AI) | $0.20/1M + LLM | 1,000 scans | $0.15 |
| **Subtotal** | | | **$4.86** |

#### AI Assistant (LLM-Based)

| Resource | Unit cost | Usage per user/month | Monthly cost |
|----------|-----------|---------------------|--------------|
| LLM API (mid-tier model) | ~$3/M input, ~$15/M output | 50 queries, ~2K tokens each | $1.80 |
| Context retrieval (embeddings) | $0.10/M tokens | 100K tokens | $0.01 |
| **Subtotal** | | | **$1.81** |

#### Infrastructure Services

| Resource | Unit cost | Usage per user/month | Monthly cost |
|----------|-----------|---------------------|--------------|
| Crash reporting (S3 + Lambda) | $0.023/GB + Lambda | 100 MB + 100 invocations | $0.01 |
| **Subtotal** | | | **$0.01** |

### Total Per-User Monthly Cost (Managed Hosting)

| Service | Cost/user/month |
|---------|----------------|
| Collaboration (CRDT sync) | $1.49 |
| Shared build cache | $9.85 |
| Asset marketplace | $0.75 |
| Universe generation | $2.25 |
| Mod hosting | $4.86 |
| AI assistant | $1.81 |
| Infrastructure | $0.01 |
| **Total (all services)** | **$21.02** |

At $29/user/month managed hosting price, the gross margin per user is **$7.98 (27.5%)**. This margin
covers operational overhead, support, and profit.

### Per-User Cost at Different Team Sizes (Managed Hosting)

Economies of scale reduce per-user cost as shared infrastructure is amortized across more users.

| Service | 1 dev | 5 devs | 20 devs | 100 devs |
|---------|-------|--------|---------|----------|
| Collaboration | $23.00 | $5.60 | $2.10 | $1.49 |
| Build cache | $12.00 | $10.50 | $10.00 | $9.85 |
| Marketplace | $1.50 | $1.00 | $0.80 | $0.75 |
| Universe generation | $2.25 | $2.25 | $2.25 | $2.25 |
| Mod hosting | $5.00 | $4.90 | $4.86 | $4.86 |
| AI assistant | $1.81 | $1.81 | $1.81 | $1.81 |
| Infrastructure | $0.50 | $0.15 | $0.05 | $0.01 |
| **Total** | **$46.06** | **$26.21** | **$21.87** | **$21.02** |

Solo developers on managed hosting face higher per-user costs due to fixed infrastructure overhead.
At team sizes of 20+, per-user costs converge toward the marginal cost. Solo developers are
encouraged to self-host, where costs are significantly lower.

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
| Cloud services   | All open source (PostgreSQL, Redis, NATS, Keycloak); self-host or managed |
| Multiplayer      | Built-in transport + replication                                          |
| Collaboration    | Built-in CRDT sync (open source)                                          |
| Build cache      | Built-in shared cache (open source)                                       |
| Marketplace      | 12% commission                                                            |
| Console support  | $10K/yr per platform                                                      |
| No-code tools    | Logic Graph (open source)                                                 |
| AI assistant     | Open source (self-hosted)                                                 |
| Mod support      | Built-in hosting (open source)                                            |
| Deployment infra | AWS deployment (open-source deps: PostgreSQL, Redis, NATS, Keycloak)     |
| Matchmaking      | Built-in open-source matchmaker (PostgreSQL, Redis, NATS)                 |

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
   dependencies (PostgreSQL, Redis, NATS, Keycloak, Forgejo, Grafana). Teams can migrate to any
   cloud or on-premises deployment. Unlike Unity Cloud, there is zero vendor lock-in.
4. **1-click deployment** -- Free AWS Marketplace listing with guided wizard deploys all services.
   Solo developers pay ~$20/mo. No DevOps expertise required.
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
7. ****Mitigation 4**** — Marketplace commission scales with ecosystem growth; seed marketplace
   early to build GMV.

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

1. ****Risk**** — Managed hosting margins are thin ($7.98/user/month). Unexpected usage spikes or
   AWS price increases could eliminate margin.
2. ****Severity**** — Medium -- manageable with monitoring and reserved instances.
3. ****Trigger**** — Viral adoption spike; underestimated bandwidth costs; GPU compute price
   increases.
4. ****Mitigation 1**** — Use reserved instances and savings plans to reduce AWS costs 40-60%.
5. ****Mitigation 2**** — Usage-based overages for bandwidth-heavy users rather than flat-rate
   pricing.
6. ****Mitigation 3**** — Monitor cost-per-user weekly; adjust pricing quarterly with 90 days
   notice.
7. ****Mitigation 4**** — Build cache and CDN costs (largest line items) scale sub-linearly with
   deduplication.

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
- [AWS S3 pricing](https://aws.amazon.com/s3/pricing/)
- [AWS RDS pricing](https://aws.amazon.com/rds/postgresql/pricing/)
- [AWS CloudFront pricing](https://aws.amazon.com/cloudfront/pricing/)
- [LLM API pricing comparison 2025](https://intuitionlabs.ai/articles/llm-api-pricing-comparison-2025)
