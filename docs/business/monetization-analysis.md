# Monetization and Open-Source Strategy Analysis

This document analyzes the business model for the Harmonius game engine. The entire engine is open
source under Apache 2.0, with revenue from proprietary console SDK addon licenses, enterprise
support contracts, and optional managed hosting.

## 0. Licensing Model

### Core Principles

| Principle | Detail |
|-----------|--------|
| Engine license | 100% free, Apache 2.0, forever |
| Royalties | None -- developers keep 100% of game revenue |
| Per-seat fees | None -- team size does not affect cost |
| Open source asset store | Free, community-run, Git-based repository |
| External store integration | FAB, Synty, TurboSquid integration in-editor -- engine takes no commission |
| Proprietary addons | ONLY for console SDKs (PlayStation, Xbox, Nintendo) that cannot be open-sourced due to platform holder NDAs; sold with 24/7 support by the engine creator |
| AI content generation | Local inference only, no cloud fees, included with the engine |

### Revenue Sources

| Source | Description | Recurring |
|--------|-------------|-----------|
| Console SDK addon licenses | Annual per-platform license for proprietary console backend integrations (PS5, Xbox, Switch) | Yes |
| Enterprise support contracts | Priority bug fixes, dedicated engineer, SLA guarantees for studios with shipping deadlines | Yes |
| Managed hosting (optional) | Convenience service for teams who prefer not to self-host collaboration, build cache, and mod infrastructure on AWS | Yes |

### What We Do NOT Charge For

| Item | Why it is free |
|------|----------------|
| Engine (all components) | Apache 2.0 -- open source builds trust and maximizes adoption |
| Open source asset store | Community-run repository; free to browse, download, and publish |
| External store purchases | We take no commission on FAB, Synty, TurboSquid, or any third-party store purchases |
| AI content generation | Runs locally on user hardware; no cloud dependency, no per-generation fee |
| Self-hosting (AWS CDK) | Full CDK stacks provided; user pays only AWS infrastructure costs |
| Marketplace commission | No built-in paid marketplace; the open source store is free and external stores handle their own payments |

## 1. Component Classification

All engine components are open source under Apache 2.0. Developers can self-host every service using
provided AWS CDK stacks. The only proprietary components are console SDKs (due to platform NDA
requirements) and the optional managed hosting service.

### Classification Definitions

| Tier | License | Who Sees Source | Revenue Model |
|------|---------|-----------------|---------------|
| **Open source** | Apache 2.0 | Everyone | Adoption driver; no direct revenue |
| **Proprietary (NDA-required)** | Proprietary binary | Console licensees | Annual platform license |
| **Proprietary SaaS (optional)** | Closed; hosted service | N/A (managed service) | Subscription for convenience |

### Component Classification Matrix

| Component | Classification | Rationale |
|-----------|---------------|-----------|
| Core Runtime (ECS, serialization, events) | Open Source (Apache 2.0) | Foundation of every game; open source builds trust and enables modding. |
| Rendering (GPU abstraction, render graph) | Open Source (Apache 2.0) | Developers need to inspect, debug, and extend rendering for custom shaders. |
| Physics | Open Source (Apache 2.0) | Gameplay-critical; studios need to tune and extend for deterministic replay. |
| Audio | Open Source (Apache 2.0) | Basic audio is table-stakes; open source allows custom DSP pipelines. |
| Input | Open Source (Apache 2.0) | Device support must be extensible for VR, accessibility, and custom hardware. |
| AI (navigation, behavior trees) | Open Source (Apache 2.0) | Gameplay AI must be inspectable. Community contributes steering, GOAP, utility AI. |
| Networking (transport, replication) | Open Source (Apache 2.0) | Multiplayer is a core differentiator; open source enables community trust and auditing. |
| Animation | Open Source (Apache 2.0) | Animation blending, IK, and state machines need studio customization. |
| UI framework | Open Source (Apache 2.0) | Every game has custom UI; open source enables deep theming and accessibility. |
| VFX (particles, effects) | Open Source (Apache 2.0) | Artists and tech artists need to extend particle systems for community effect libraries. |
| Content Pipeline (import, processing) | Open Source (Apache 2.0) | DCC plugin ecosystem depends on open formats. Community contributes importers. |
| Game Framework (gameplay systems) | Open Source (Apache 2.0) | Combat, progression, and camera systems must be customizable per genre. |
| Platform (windowing, OS) | Open Source (Apache 2.0) | Platform abstraction must be portable across all supported operating systems. |
| Editor (scene, material editors) | Open Source (Apache 2.0) | Full editor source enables deep customization and community-driven extensions. |
| Logic Graph system | Open Source (Apache 2.0) | Core no-code promise; open source enables custom node authoring by the community. |
| Collaboration service (CRDT, cloud) | Open Source (Apache 2.0) | Self-hosted via AWS CDK. Managed hosting available for convenience. |
| Shared build cache | Open Source (Apache 2.0) | Self-hosted via AWS CDK. Managed hosting available for convenience. |
| Asset marketplace | Open Source (Apache 2.0) | Self-hosted via AWS CDK. Managed hosting available for convenience. |
| Universe generation server | Open Source (Apache 2.0) | Self-hosted via AWS CDK. Managed hosting available for convenience. |
| Mod hosting and moderation | Open Source (Apache 2.0) | Self-hosted via AWS CDK. Managed hosting available for convenience. |
| AI assistant (LLM-based) | Open Source (Apache 2.0) | Self-hosted with user-provided LLM API keys. Managed hosting available. |
| Console backends (PS5, Xbox, Switch) | Proprietary (NDA-required) | Platform holder NDAs prohibit open-sourcing console SDKs. |
| Managed hosting service | Proprietary SaaS (optional) | Convenience service for teams who prefer not to self-host on AWS. |

### Classification Summary

| Tier | Count | Components |
|------|-------|------------|
| Open Source (Apache 2.0) | 21 | All engine, editor, server, and tooling components |
| Proprietary (NDA-required) | 1 | Console backends (PS5, Xbox, Switch SDKs) |
| Proprietary SaaS (optional) | 1 | Managed hosting service |

Open-sourcing 21 of 23 components (91%) under Apache 2.0 maximizes adoption and community trust. The
only proprietary components exist due to legal constraints (console NDAs) or as an optional
convenience service (managed hosting).

## 2. Pricing Model

### Pricing Structure

| Aspect | Price | Notes |
|--------|-------|-------|
| Engine (all components) | Free | Apache 2.0, no royalties, no per-seat fees |
| Self-hosting (AWS CDK) | Free (user pays AWS) | Full AWS CDK stacks provided for all services |
| Managed hosting | $29/user/month | Optional; for teams who prefer not to self-host |
| Enterprise support | $10,000-$50,000/year | Priority bug fixes, dedicated engineer, SLA |
| Console SDK license (PS5) | $10,000/year | Covers NDA compliance, 24/7 support, and platform updates |
| Console SDK license (Xbox) | $10,000/year | Covers NDA compliance, 24/7 support, and platform updates |
| Console SDK license (Switch) | $10,000/year | Covers NDA compliance, 24/7 support, and platform updates |
| Open source asset store | Free | Community-run; no fees to browse, download, or publish |
| External store integration | Free | FAB, Synty, TurboSquid; engine takes no commission |
| Education and non-commercial | Free | All components, including managed hosting trial |

### Key Pricing Principles

1. **No royalties** -- Developers keep 100% of game revenue regardless of sales volume.
2. **No per-seat fees** -- Team size does not affect engine cost.
3. **Self-hosting is always free** -- Managed hosting is a convenience, not a requirement.
4. **Console SDKs are the only proprietary addons** -- Required only for console shipping; includes
   24/7 support.
5. **No marketplace commission** -- The open source asset store is free; external store integrations
   take no cut.
6. **AI content generation is free** -- Local inference runs on user hardware with no cloud fees.

### Comparison: Self-Hosted vs. Managed Hosting

| Aspect | Self-hosted (AWS CDK) | Managed hosting ($29/user/mo) |
|--------|----------------------|-------------------------------|
| Setup effort | Deploy CDK stacks (1-2 hours) | Zero; instant provisioning |
| Monthly cost (10-person team) | ~$500-$2,000 (AWS direct) | $290 |
| Monthly cost (50-person team) | ~$2,000-$5,000 (AWS direct) | $1,450 |
| Maintenance | Team manages updates, scaling | Fully managed by Harmonius |
| Data sovereignty | Full control over data location | US/EU region selection |
| Customization | Full (modify any service) | Standard configuration only |
| SLA | Per AWS SLA | 99.9% uptime guarantee |

For small teams (under 20), managed hosting is often cheaper than self-hosting due to shared
infrastructure amortization. Larger studios typically self-host for cost savings and control.

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

| Year | Total registered devs | Active monthly devs | Managed hosting users (% of active) | Console licensees |
|------|----------------------|--------------------|------------------------------------|-------------------|
| Year 1 | 5,000 | 2,000 | 200 (10%) | 5 |
| Year 2 | 20,000 | 8,000 | 1,200 (15%) | 20 |
| Year 3 | 50,000 | 20,000 | 3,000 (15%) | 50 |
| Year 5 | 150,000 | 60,000 | 12,000 (20%) | 150 |

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

| Team size | Annual cost | Required managed hosting users (at $7.98 margin) | Additional revenue needed |
|-----------|-------------|--------------------------------------------------|--------------------------|
| 10 engineers | $2,500,000 | 2,000 | $1,308,560 from support + console |
| 25 engineers | $6,250,000 | 5,000 | $4,058,560 from support + console |
| 50 engineers | $12,500,000 | 10,000 | $8,558,560 from support + console |

Managed hosting alone cannot sustain a large team. Console SDK licenses and enterprise support are
the critical secondary revenue streams. Without a paid marketplace commission, the model depends
more heavily on console SDK adoption and enterprise support contracts.

## 5. Competitive Comparison

| Aspect | Harmonius | Unreal Engine | Unity | Godot | O3DE |
|--------|-----------|---------------|-------|-------|------|
| Source access | Fully open source (Apache 2.0) | Source-available (custom EULA) | Closed runtime | Fully open source (MIT) | Fully open source (Apache 2.0) |
| Royalties | None | 5% above $1M (3.5% with EGS) | None (subscription model) | None | None |
| Engine cost | Free forever | Free under $1M revenue | $2,200/seat/yr (Pro) | Free forever | Free forever |
| Cloud services | All open source; self-host or managed | None built-in | Unity Cloud (proprietary) | None | None |
| Multiplayer | Built-in transport + replication | Basic; use third-party | Netcode package | High-level only | Basic |
| Collaboration | Built-in CRDT sync (open source) | None | Unity DevOps (proprietary) | None | None |
| Build cache | Built-in shared cache (open source) | None | Unity Accelerator (proprietary) | None | None |
| Marketplace | 12% commission | Fab: 12% commission | Asset Store: 30% commission | Community library (free) | None |
| Console support | $10K/yr per platform | Free with license | Enterprise tier | Community ports (limited) | Experimental |
| No-code tools | Logic Graph (open source) | Blueprints (source-available) | Visual Scripting (deprecated) | VisualScript | Script Canvas |
| AI assistant | Open source (self-hosted) | None built-in | Muse AI (limited) | None | None |
| Mod support | Built-in hosting (open source) | Limited (game-specific) | Limited (game-specific) | None built-in | None built-in |
| Deployment infra | AWS CDK stacks (open source) | None | Unity Cloud (proprietary) | None | None |

### Key Differentiators

1. **Fully open source with no royalties** -- Unlike Unreal (5% royalty, source-available only) or
   Unity (closed source, per-seat fees), Harmonius is Apache 2.0 with zero royalties. Developers
   keep 100% of game revenue.
2. **Built-in multiplayer and collaboration infrastructure** -- No other open-source engine ships
   with production-ready networking, CRDT collaboration, shared build cache, and deployment
   infrastructure. Godot lacks these entirely; Unreal requires third-party solutions.
3. **Self-hostable everything** -- Every server component ships with AWS CDK stacks. Teams are never
   locked into Harmonius-managed services, unlike Unity Cloud.
4. **AAA-grade features in an open-source engine** -- Harmonius bridges the gap between Godot (open
   source but limited feature set) and Unreal (AAA features but proprietary with royalties).
   Features like GPU-driven rendering, ray tracing, advanced physics, and universe generation exceed
   what Godot offers.
5. **No vendor lock-in** -- Apache 2.0 license guarantees perpetual access. No pricing changes, no
   retroactive terms, no trust erosion. The Unity pricing crisis of 2023 cannot happen.

### Competitive Positioning Matrix

| Need | Best choice | Why |
|------|-------------|-----|
| AAA graphics, zero royalties | **Harmonius** | Unreal charges 5%; Godot lacks AAA features |
| Full source access + modify | **Harmonius** or Godot | Unreal is source-available only (cannot redistribute) |
| Built-in multiplayer infra | **Harmonius** | Only engine with open-source transport, replication, and deployment |
| No-code development | **Harmonius** or Unreal | Harmonius Logic Graph is open source; Blueprints is source-available |
| Indie with no budget | Godot or **Harmonius** | Both free; Harmonius adds cloud infra if self-hosted |
| Enterprise with SLA needs | **Harmonius** or Unreal | Harmonius offers dedicated engineer support contracts |

## 6. Risks and Mitigations

### Risk 1: Insufficient Revenue from Optional Services

| Aspect | Detail |
|--------|--------|
| **Risk** | With no royalties and a fully open-source engine, revenue depends entirely on optional services that most users may skip. |
| **Severity** | High -- primary business risk of the open-source model. |
| **Trigger** | Managed hosting adoption below 10%; enterprise support demand lower than projected. |
| **Mitigation 1** | Console SDK licenses provide baseline revenue independent of hosting adoption. |
| **Mitigation 2** | Managed hosting is priced competitively vs. self-hosting for small teams, driving organic adoption. |
| **Mitigation 3** | Enterprise support includes priority bug fixes and SLA, creating clear value for studios with shipping deadlines. |
| **Mitigation 4** | Marketplace commission scales with ecosystem growth; seed marketplace early to build GMV. |

### Risk 2: Open-Source Fork Competing

| Aspect | Detail |
|--------|--------|
| **Risk** | A well-funded competitor forks the entire engine (including server components) and offers competing managed hosting. |
| **Severity** | High -- all components are open source, so forking is legally permitted. |
| **Trigger** | Engine reaches critical mass; a major company (Amazon, Google) forks and builds competing SaaS. |
| **Mitigation 1** | Maintain development velocity: ship features faster than any fork. Harmonius team has full context advantage. |
| **Mitigation 2** | Build marketplace network effects: asset sellers, buyers, and mod communities create ecosystem lock-in that a fork cannot replicate overnight. |
| **Mitigation 3** | Cultivate strong community relationships. Forks succeed when upstream maintainers alienate contributors. Invest heavily in community governance and contributor recognition. |
| **Mitigation 4** | Console SDK licenses are non-forkable (NDA-bound), providing a revenue floor that forks cannot undercut. |

### Risk 3: Cloud Service Costs Exceeding Revenue

| Aspect | Detail |
|--------|--------|
| **Risk** | Managed hosting margins are thin ($7.98/user/month). Unexpected usage spikes or AWS price increases could eliminate margin. |
| **Severity** | Medium -- manageable with monitoring and reserved instances. |
| **Trigger** | Viral adoption spike; underestimated bandwidth costs; GPU compute price increases. |
| **Mitigation 1** | Use reserved instances and savings plans to reduce AWS costs 40-60%. |
| **Mitigation 2** | Usage-based overages for bandwidth-heavy users rather than flat-rate pricing. |
| **Mitigation 3** | Monitor cost-per-user weekly; adjust pricing quarterly with 90 days notice. |
| **Mitigation 4** | Build cache and CDN costs (largest line items) scale sub-linearly with deduplication. |

### Risk 4: Marketplace Not Achieving Critical Mass

| Aspect | Detail |
|--------|--------|
| **Risk** | Without enough sellers, buyers don't visit. Without buyers, sellers don't publish. |
| **Severity** | Medium -- marketplace is a growing but not primary revenue source. |
| **Trigger** | Launch with fewer than 500 assets; competing marketplaces (Fab, Sketchfab) dominate. |
| **Mitigation 1** | Seed the marketplace with 200+ first-party assets (materials, models, templates) at launch. |
| **Mitigation 2** | Offer 95/5 revenue split for first 12 months to attract early sellers (vs. standard 88/12). |
| **Mitigation 3** | Integrate marketplace deeply into editor: one-click import, live preview, dependency resolution. |
| **Mitigation 4** | Support cross-engine formats so sellers from Unity/Unreal can publish with minimal friction. |

### Risk 5: Console SDK Revenue Concentration

| Aspect | Detail |
|--------|--------|
| **Risk** | Console SDK licenses represent 29% of mature revenue but depend on platform holder relationships. |
| **Severity** | Medium -- console holders could change NDA terms or build competing integrations. |
| **Trigger** | Platform holder builds first-party Harmonius support; NDA terms become more restrictive. |
| **Mitigation 1** | Diversify revenue so no single stream exceeds 35% of total. |
| **Mitigation 2** | Build strong relationships with platform holder developer relations teams. |
| **Mitigation 3** | Ensure console SDK quality is best-in-class to justify the license fee. |

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
