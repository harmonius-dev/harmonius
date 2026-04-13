# 15.18 — Self-Hosted Server Infrastructure

## Self-Hosted AWS Deployment

| ID | Feature |
| ------------ | -------------------------------------- |
| F-15.18.1 | AWS Deployment Stacks |
| F-15.18.2 | Live Collaboration Server |
| F-15.18.3 | Git and Git LFS Hosting with Locking |
| F-15.18.4 | Asset and Shader Compilation Server |
| F-15.18.5 | Signing and Distribution Server |
| F-15.18.6 | Continuous Deployment Pipeline |
| F-15.18.7 | Test Runner Infrastructure |
| F-15.18.8 | Shared Cache and Database Services |
| F-15.18.9 | Backup and Disaster Recovery |
| F-15.18.10 | Enterprise Security Configuration |

1. **F-15.18.1** — Open-source deployment configuration in Rust that deploys all Harmonius server
   components to a developer's own AWS account. Stacks are modular — deploy only what you need. Each
   stack configures VPC networking, IAM roles, security groups, encryption (at rest and in transit),
   CloudWatch monitoring, and auto-scaling. Two deployment profiles: **Free Tier** (single-AZ,
   t3.micro/t4g.micro instances, minimal storage, suitable for solo developers and prototyping) and
   **Enterprise** (multi-AZ, auto-scaling groups, RDS Multi-AZ, ElastiCache, CloudFront CDN, WAF,
   suitable for production studios). A single deploy command provisions the entire infrastructure.
   Stack outputs include endpoint URLs, connection strings, and API keys. All stacks are
   version-pinned to the engine release for compatibility. limits (750 hours t3.micro, 5GB S3, 20GB
   RDS).
   - **Deps:** None (standalone infrastructure)
   - **Platform:** AWS only. Free tier stays within AWS Free Tier
2. **F-15.18.2** — A self-hosted CRDT-based real-time collaboration server (the backend for
   F-15.12.3-14). Deployed as an ECS Fargate container or EC2 instance. Uses PostgreSQL (RDS) for
   persistent document state, S3 for large binary assets, and WebSocket connections for real-time
   sync. The server handles: multi-user editing sessions, presence tracking, access control,
   voice/text chat relay, and AI agent collaboration channels. Deploys: RDS PostgreSQL (db.t3.micro
   for free tier, db.r6g.large for enterprise), S3 bucket with lifecycle policies, Application Load
   Balancer with WebSocket support, and ECS task definitions. Fargate, Multi-AZ RDS.
   - **Deps:** F-15.18.1, F-15.12.7 (Cloud Collaboration Service)
   - **Platform:** Free tier: single Fargate task, db.t3.micro RDS. Enterprise: auto-scaling
3. **F-15.18.3** — A self-hosted Git server with Git LFS support and file locking. Deployed as a
   Gitea or Forgejo instance on ECS/EC2 with S3-backed LFS storage. Supports the engine's version
   control features (F-15.10.1-8): push, pull, branch, merge, LFS lock/unlock, and multi-provider
   API compatibility (GitHub-compatible REST API so the editor's Git integration works without
   modification). LFS objects are stored in S3 with configurable lifecycle policies (move to Glacier
   after 90 days). Deploys: EC2/Fargate for the Git server, S3 for LFS storage, EBS for Git
   repository data, and Route 53 for DNS. CloudFront for LFS downloads.
   - **Deps:** F-15.18.1, F-15.10.1 (Git Integration), F-15.10.2 (Git LFS)
   - **Platform:** Free tier: t3.micro EC2, 5GB S3. Enterprise: m6i.large, unlimited S3,
4. **F-15.18.4** — A self-hosted build farm for asset cooking and shader compilation. Deployed as an
   auto-scaling group of GPU-enabled instances (for shader compilation) or CPU instances (for asset
   cooking). The compilation server processes jobs from a Redis/SQS queue: texture compression, LOD
   generation, meshlet building, HLSL-to-DXIL/SPIR-V/MSL compilation, and logic graph bytecode
   generation. Results are stored in the shared build cache (S3-backed, F-15.11.1). Provisions: SQS
   job queue, auto-scaling EC2 (c6i for CPU, g5 for GPU), S3 for build cache, and CloudWatch alarms
   for queue depth. c6i.2xlarge + g5.xlarge spot instances.
   - **Deps:** F-15.18.1, F-15.11.1 (Shared Build Cache), F-12.2.9 (Shader Pipeline)
   - **Platform:** Free tier: single t3.micro (CPU-only, slow). Enterprise: auto-scaling
5. **F-15.18.5** — A self-hosted code signing and distribution packaging server. Stores signing
   credentials in AWS Secrets Manager (never on disk). Automates: iOS signing with provisioning
   profiles, macOS notarization submission, Android keystore signing, Windows Authenticode signing,
   and package generation (.dmg, .msi, .deb, AppImage). Integrates with the store distribution
   pipeline (F-15.14.8) for automated submissions. Deploys: EC2 build agent, Secrets Manager for
   credentials, S3 for build artifacts, and CodePipeline for orchestration.
   - **Deps:** F-15.18.1, F-15.14.4 (Code Signing), F-15.14.5 (Installers)
   - **Platform:** iOS signing requires a macOS build agent (Mac EC2 or external).
6. **F-15.18.6** — A self-hosted CI/CD pipeline for building, testing, and deploying game builds.
   Uses AWS CodePipeline with CodeBuild for Rust compilation, asset cooking, test execution, and
   deployment to staging/production environments. The pipeline triggers on Git push events (via
   webhook from the Git server F-15.18.3). Pipeline stages: source checkout, Rust build
   (debug/release), asset cook (F-15.18.4), automated tests (unit, integration, screenshot),
   packaging (F-15.18.5), deployment to S3/CloudFront for distribution. The deployment configuration
   provisions the entire pipeline with configurable stages. environments.
   - **Deps:** F-15.18.1, F-15.18.3 (Git Server), F-15.18.4 (Build Farm), F-15.18.5 (Signing)
   - **Platform:** Free tier: single CodeBuild project. Enterprise: parallel builds, multiple
7. **F-15.18.7** — A self-hosted automated testing infrastructure for continuous quality assurance.
   Deploys test runner instances that execute: unit tests (Rust cargo test), integration tests
   (multi-system scenarios), screenshot comparison tests (render a scene, compare against golden
   images), performance benchmark tests (measure frame time, draw calls, memory), and stress tests
   (spawn N entities, simulate M seconds). Test results are stored in DynamoDB with CloudWatch
   dashboards for trend analysis. Failed tests block the deployment pipeline (F-15.18.6).
   Provisions: CodeBuild projects per test type, DynamoDB for results, S3 for golden images, and SNS
   for failure notifications.
   - **Deps:** F-15.18.1, F-15.18.6 (CI/CD Pipeline)
   - **Platform:** GPU tests require g5 instances. Free tier uses CPU-only tests.
8. **F-15.18.8** — Self-hosted shared cache (S3-backed) and database services (PostgreSQL, DynamoDB)
   for all engine server components. The shared cache stores: compiled assets, shader bytecode,
   logic graph bytecode, terrain chunks, and universe generation output. Databases store: project
   metadata, user accounts, collaboration state, crash reports, telemetry, and matchmaking data.
   Provisions: S3 with intelligent tiering, RDS PostgreSQL, DynamoDB tables, ElastiCache Redis for
   hot data, and backup schedules. Data encryption at rest (KMS) and in transit (TLS) is enforced.
   Intelligent-Tiering, db.r6g.large Multi-AZ, DynamoDB provisioned.
   - **Deps:** F-15.18.1, F-15.11.1 (Shared Build Cache), F-8.7.5 (Persistent World State)
   - **Platform:** Free tier: S3 standard, db.t3.micro RDS, DynamoDB on-demand. Enterprise: S3
9. **F-15.18.9** — Automated backup and restore for all self-hosted server data. RDS automated
   backups with configurable retention (7-35 days). S3 versioning for asset storage with lifecycle
   policies. DynamoDB point-in-time recovery. Cross-region replication for enterprise disaster
   recovery. A restore CLI tool recovers from any backup point to a new or existing stack. Backup
   status is monitored via CloudWatch alarms. The deployment configuration provisions all backup
   policies and the cross-region replication topology.
   - **Deps:** F-15.18.1, F-15.18.8 (Databases)
   - **Platform:** Free tier: 7-day RDS backups, S3 versioning. Enterprise: 35-day, cross-region.
10. **F-15.18.10** — Enterprise-grade security for all self-hosted components. VPC with private
    subnets for databases and internal services, public subnets only for load balancers. IAM roles
    with least-privilege policies per service. AWS WAF on Application Load Balancers to block common
    attacks (SQL injection, XSS, rate limiting). KMS encryption for all data at rest. TLS 1.3 for
    all data in transit. CloudTrail audit logging for all API calls. GuardDuty for threat detection.
    Secrets Manager rotation for database credentials and API keys. The deployment configuration
    enforces these security controls by default in enterprise profile; the free tier profile relaxes
    some controls (single AZ, no WAF, no GuardDuty) to stay within cost limits. VPC.
    - **Deps:** F-15.18.1
    - **Platform:** Enterprise security adds ~$50-100/month in AWS costs. Free tier uses default

## Open-Source Service Deployment

All service stacks use open-source dependencies exclusively. Proprietary AWS services are replaced
with open-source equivalents deployed on AWS infrastructure. S3 is retained because it implements
the standard object storage API (MinIO-compatible for local dev).

**Open-Source Dependency Map:**

| Service Need | Open-Source Choice | AWS Managed Option | Notes |
|--------------|--------------------|--------------------|-------|
| Database | PostgreSQL | RDS for PostgreSQL | No Aurora, no DynamoDB |
| Cache | Redis / Valkey | ElastiCache for Redis | No proprietary ElastiCache protocol |
| Auth | Keycloak | Keycloak on ECS | No Cognito |
| Message queue | NATS | Amazon MQ for NATS | No SQS, no SNS |
| Object storage | S3-compatible API | S3 | MinIO for local dev and on-prem |
| Search | OpenSearch | Amazon OpenSearch Service | No proprietary CloudSearch |
| Monitoring | Grafana + Prometheus | Self-hosted on ECS | No CloudWatch dashboards |
| Git hosting | Forgejo | Self-hosted on ECS/EC2 | No CodeCommit |
| CI/CD | Forgejo Actions | Self-hosted runners | No CodePipeline, no CodeBuild |

| ID | Feature |
| ------------ | ------------------------------------ |
| F-15.18.11 | AWS Open-Source Service Stacks |
| F-15.18.12 | 1-Click AWS Marketplace Deployment |
| F-15.18.13 | Service Admin Dashboard |
| F-15.18.14 | Scaling Profiles |
| F-15.18.15 | Self-Hosted Build Cache Service |
| F-15.18.16 | Self-Hosted Collaboration Service |
| F-15.18.17 | Self-Hosted Matchmaking Service |
| F-15.18.18 | Self-Hosted Asset Store Service |
| F-15.18.19 | Health Monitoring Stack |
| F-15.18.20 | Multi-Region Deployment |

1. **F-15.18.11** — Deployment templates provisioning all Harmonius services using exclusively
   open-source dependencies. Each stack provisions the open-source service (PostgreSQL,
   Redis/Valkey, Keycloak, NATS, Forgejo, Grafana, Prometheus, OpenSearch) on AWS compute (ECS
   Fargate or EC2). Stacks use RDS for PostgreSQL and ElastiCache for Redis as managed hosting of
   the open-source software. All services expose standard open-source APIs and protocols, enabling
   migration to any cloud or on-premises deployment. Templates are parameterized for instance size,
   storage, and replica count.
   - **Deps:** F-15.18.1
   - **Platform:** AWS. All services run their official open-source container images.
2. **F-15.18.12** — A free AWS Marketplace product listing that launches a guided CloudFormation
   wizard. The wizard collects: scaling profile (solo/team/studio/production), region, domain name,
   and optional services. On submit, it deploys the infrastructure with the selected profile. No
   Marketplace fees -- the product is listed as free with the user paying only AWS infrastructure
   costs. The listing includes a cost calculator showing estimated monthly spend per profile.
   - **Deps:** F-15.18.11
   - **Platform:** AWS Marketplace free-tier listing. Requires AWS Marketplace seller account.
3. **F-15.18.13** — A web-based admin panel (Grafana + custom dashboards) for monitoring all
   deployed services. Displays: service health (up/down/degraded), resource utilization (CPU,
   memory, disk, network), active users, build queue depth, collaboration session count, and cost
   estimates. Alerts via NATS pub/sub to email, Slack, or PagerDuty webhooks. The dashboard is
   deployed as an ECS Fargate task with Keycloak SSO authentication.
   - **Deps:** F-15.18.11, F-15.18.19
   - **Platform:** Grafana OSS. Prometheus for metrics collection.
4. **F-15.18.14** — Four predefined scaling profiles controlling instance sizes, replica counts, and
   storage allocations across all service stacks. **Solo** (~~20/mo): t4g.micro instances, minimal
   storage, single-AZ, 1 user. **Team**
   ~~$100/mo): t4g.small instances, moderate storage, single-AZ, 2-10 users. **Studio** (~$500/mo):
   t4g.medium/m6g.large instances, generous storage, multi-AZ, 10-50 users. **Production**
   (~$2000+/mo): m6g.xlarge+ instances, auto-scaling, multi-AZ, 50+ users, read replicas, CDN. Each
   profile is a deployment configuration parameter.
   - **Deps:** F-15.18.11
   - **Platform:** Cost estimates are for US-East-1. Actual costs vary by region and usage.
5. **F-15.18.15** — A self-hosted build cache backed by S3-compatible storage with a Redis/Valkey
   lookup index. Stores compiled assets, shader bytecode, and logic graph bytecode keyed by content
   hash (BLAKE3). The cache service exposes a REST API for put/get/query operations. Cache eviction
   uses LRU with configurable TTL and storage quota. NATS publishes cache hit/miss metrics to
   Prometheus. Integrates with the build farm (F-15.18.4) and editor (F-15.11.1). Authentication via
   Keycloak JWT tokens.
   - **Deps:** F-15.18.11, F-15.11.1 (Shared Build Cache)
   - **Platform:** Solo profile: 10 GB S3. Production profile: unlimited S3 with CloudFront CDN.
6. **F-15.18.16** — A self-hosted CRDT collaboration backend using PostgreSQL for document state,
   Redis/Valkey for session state and pub/sub, NATS for inter-service messaging, and S3-compatible
   storage for binary assets. Authentication and authorization via Keycloak with project-level RBAC.
   WebSocket connections terminate at an ALB with sticky sessions backed by Redis. Supports the same
   protocol as the managed collaboration service (F-15.18.2).
   - **Deps:** F-15.18.11, F-15.12.7 (Cloud Collaboration Service)
   - **Platform:** Solo: 1 Fargate task, db.t4g.micro. Production: auto-scaling, Multi-AZ.
7. **F-15.18.17** — A self-hosted matchmaking and game server orchestration service. Uses PostgreSQL
   for player profiles, skill ratings, and match history. Redis/Valkey for real-time matchmaking
   queues and lobby state. NATS for match events and server allocation requests. Supports:
   ELO/Glicko-2 skill rating, region-based matching, party queuing, custom match rules, and
   dedicated server provisioning on EC2 spot instances. Authentication via Keycloak. REST +
   WebSocket API for game clients.
   - **Deps:** F-15.18.11, F-8.7.1 (Server Architecture)
   - **Platform:** Solo: single-instance matchmaker. Production: multi-region with latency-based
     routing.
8. **F-15.18.18** — A self-hosted asset marketplace backend using PostgreSQL for asset metadata,
   OpenSearch for full-text and tag-based search, S3-compatible storage for asset packages, and
   Redis/Valkey for download counts and trending rankings. Supports: asset upload with automated
   validation, version management, dependency resolution, license tagging (Apache 2.0, CC-BY, etc.),
   and community ratings. Authentication via Keycloak. REST API for the editor's asset browser
   (F-15.17). NATS publishes asset events for webhook integrations.
   - **Deps:** F-15.18.11, F-15.17.1 (Asset Store Browsing)
   - **Platform:** Solo: local SQLite fallback. Production: OpenSearch cluster, CloudFront CDN.
9. **F-15.18.19** — A self-hosted monitoring stack using Prometheus for metrics collection, Grafana
   for dashboards and alerting, and Loki for log aggregation. Prometheus scrapes all service
   endpoints via `/metrics`. Grafana dashboards are pre-configured for each service (PostgreSQL,
   Redis, Keycloak, NATS, Forgejo, build farm, collaboration). Alert rules fire to NATS topics,
   which route to email, Slack, PagerDuty, or custom webhooks. CloudWatch is used only for AWS
   infrastructure metrics (EC2, RDS, ECS) which are exported to Prometheus via the CloudWatch
   exporter.
   - **Deps:** F-15.18.11
   - **Platform:** Prometheus + Grafana on ECS Fargate. Loki for structured log search.
10. **F-15.18.20** — Deployment templates support multi-region deployment for production workloads.
    Primary region hosts all read-write services. Secondary regions host read replicas (PostgreSQL),
    Redis replicas, S3 cross-region replication, and CDN edge caches. NATS clustering spans regions
    for event replication. Failover is manual via configuration parameter switch (primary region
    designation). DNS-based routing (Route 53 health checks) directs clients to the nearest healthy
    region. Multi-region adds ~50% cost overhead.
    - **Deps:** F-15.18.11, F-15.18.14
    - **Platform:** Production profile only. Minimum 2 regions. NATS super-cluster for cross-region.
