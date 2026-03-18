# User Stories: Self-Hosted Server Infrastructure

## F-15.18.1 AWS CDK Deployment Stacks

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.18.1.1 | developer (P-15)       |          |              |
| US-15.18.1.2 | DevOps engineer (P-16) |          |              |
| US-15.18.1.3 | server admin (P-22)    |          |              |
| US-15.18.1.4 | engine tester (P-27)   |          |              |

1. **US-15.18.1.1** — to deploy all Harmonius server components to my own AWS account with a single
   `cdk deploy --profile free-tier` command
   - **Acceptance:** I get collaboration server, Git hosting, and build cache without manually
     provisioning resources
2. **US-15.18.1.2** — to deploy only the CDK stacks my team needs (e.g., Git server and build farm
   but not collaboration)
   - **Acceptance:** I avoid paying for unused infrastructure
3. **US-15.18.1.3** — CDK stacks version-pinned to the engine release
   - **Acceptance:** infrastructure upgrades are coordinated with engine updates
4. **US-15.18.1.4** — to verify that the Free Tier profile stays within AWS Free Tier limits (750h
   t3.micro, 5GB S3, 20GB RDS)
   - **Acceptance:** solo developers incur near-zero cloud costs

## F-15.18.2 Live Collaboration Server

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.18.2.1 | server admin (P-22)    |          |              |
| US-15.18.2.2 | developer (P-15)       |          |              |
| US-15.18.2.3 | DevOps engineer (P-16) |          |              |
| US-15.18.2.4 | engine tester (P-27)   |          |              |

1. **US-15.18.2.1** — the collaboration server deployed in my own AWS account with all data in my
   RDS and S3 buckets
   - **Acceptance:** project data satisfies data residency requirements
2. **US-15.18.2.2** — a collaboration server enabling CRDT-based editing sessions over WebSocket
   - **Acceptance:** my team edits simultaneously without file locking
3. **US-15.18.2.3** — the enterprise server to auto-scale Fargate tasks based on active session
   count
   - **Acceptance:** the server handles peak usage without manual capacity planning
4. **US-15.18.2.4** — to verify that the collaboration server correctly handles multi-user editing
   sessions, presence tracking, access control, and chat relay
   - **Acceptance:** all collaboration features work on self-hosted infrastructure

## F-15.18.3 Git and Git LFS Hosting with Locking

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.18.3.1 | developer (P-15)       |          |              |
| US-15.18.3.2 | DevOps engineer (P-16) |          |              |
| US-15.18.3.3 | server admin (P-22)    |          |              |
| US-15.18.3.4 | engine tester (P-27)   |          |              |

1. **US-15.18.3.1** — a self-hosted Git server with LFS support backed by S3 storage
   - **Acceptance:** I host source and binary assets without per-seat hosting fees
2. **US-15.18.3.2** — the self-hosted Git server to expose a GitHub-compatible REST API
   - **Acceptance:** the editor's Git integration works without configuration changes
3. **US-15.18.3.3** — LFS objects automatically moved to Glacier after a configurable retention
   period
   - **Acceptance:** storage costs for old assets decrease over time
4. **US-15.18.3.4** — to verify that LFS lock and unlock operations work correctly through the
   self-hosted server
   - **Acceptance:** file locking prevents concurrent binary edits

## F-15.18.4 Asset and Shader Compilation Server

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.18.4.1 | developer (P-15)       |          |              |
| US-15.18.4.2 | DevOps engineer (P-16) |          |              |
| US-15.18.4.3 | server admin (P-22)    |          |              |
| US-15.18.4.4 | engine tester (P-27)   |          |              |

1. **US-15.18.4.1** — to submit shader compilation and asset cooking jobs to a remote build farm
   - **Acceptance:** my laptop stays responsive while builds run on dedicated cloud instances
2. **US-15.18.4.2** — the build farm to auto-scale EC2 instances based on SQS queue depth
   - **Acceptance:** compilation throughput increases automatically during content freezes
3. **US-15.18.4.3** — compiled results stored in the S3-backed shared build cache
   - **Acceptance:** each asset is compiled at most once across the team
4. **US-15.18.4.4** — to verify that GPU-enabled instances (g5) compile shaders correctly for all
   target platforms
   - **Acceptance:** remote compilation matches local output

## F-15.18.5 Signing and Distribution Server

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.18.5.1 | server admin (P-22)    |          |              |
| US-15.18.5.2 | DevOps engineer (P-16) |          |              |
| US-15.18.5.3 | developer (P-15)       |          |              |
| US-15.18.5.4 | engine tester (P-27)   |          |              |

1. **US-15.18.5.1** — signing credentials stored in AWS Secrets Manager with automatic rotation
   - **Acceptance:** credentials are never on disk or in version control
2. **US-15.18.5.2** — the signing server to sign builds for iOS, macOS, Android, and Windows as part
   of the packaging pipeline
   - **Acceptance:** manual signing is eliminated
3. **US-15.18.5.3** — a single command that signs and packages my game for all target platforms
   - **Acceptance:** I publish to multiple stores without learning each signing toolchain
4. **US-15.18.5.4** — to verify that all signing operations are logged in CloudTrail with
   non-repudiation
   - **Acceptance:** the studio has a complete audit trail

## F-15.18.6 Continuous Deployment Pipeline

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.18.6.1 | DevOps engineer (P-16) |          |              |
| US-15.18.6.2 | developer (P-15)       |          |              |
| US-15.18.6.3 | server admin (P-22)    |          |              |
| US-15.18.6.4 | engine tester (P-27)   |          |              |

1. **US-15.18.6.1** — a CI/CD pipeline triggering on every Git push that builds, cooks assets, runs
   tests, packages, and deploys to staging
   - **Acceptance:** every commit is validated without manual intervention
2. **US-15.18.6.2** — a single CodeBuild project on the free tier that builds, tests, and packages
   on every push
   - **Acceptance:** I get CI without configuring Jenkins or paying for hosted CI
3. **US-15.18.6.3** — to enable or disable pipeline stages (skip GPU tests, skip store submission)
   via CDK configuration
   - **Acceptance:** the pipeline adapts to the project's current phase
4. **US-15.18.6.4** — to verify that the full pipeline (source checkout, build, asset cook, test,
   package, deploy) completes successfully
   - **Acceptance:** automated releases are reliable

## F-15.18.7 Test Runner Infrastructure

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.18.7.1 | DevOps engineer (P-16) |          |              |
| US-15.18.7.2 | server admin (P-22)    |          |              |
| US-15.18.7.3 | developer (P-15)       |          |              |
| US-15.18.7.4 | engine tester (P-27)   |          |              |

1. **US-15.18.7.1** — screenshot comparison tests comparing rendered scenes against golden images
   - **Acceptance:** unintended rendering changes are caught before reaching players
2. **US-15.18.7.2** — test results stored in DynamoDB with CloudWatch dashboards showing frame time,
   draw call, and memory trends
   - **Acceptance:** gradual performance degradation is detected early
3. **US-15.18.7.3** — SNS notifications (email or Slack webhook) when tests fail
   - **Acceptance:** I can fix issues without polling the CI dashboard
4. **US-15.18.7.4** — to verify that failed tests block the deployment pipeline from advancing to
   packaging
   - **Acceptance:** no build ships without passing all quality checks

## F-15.18.8 Shared Cache and Database Services

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.18.8.1 | DevOps engineer (P-16) |          |              |
| US-15.18.8.2 | developer (P-15)       |          |              |
| US-15.18.8.3 | server admin (P-22)    |          |              |
| US-15.18.8.4 | engine tester (P-27)   |          |              |

1. **US-15.18.8.1** — all server components to share a single set of database and cache services
   (RDS, DynamoDB, ElastiCache, S3) configured by one CDK stack
   - **Acceptance:** data infrastructure is managed in one place
2. **US-15.18.8.2** — the shared S3 cache to return compiled assets by content hash on editor
   startup
   - **Acceptance:** I skip local compilation and start working in minutes
3. **US-15.18.8.3** — KMS encryption on all storage and TLS on all connections
   - **Acceptance:** project data is protected at rest and in transit
4. **US-15.18.8.4** — to verify that S3 Intelligent-Tiering moves infrequently accessed assets to
   cheaper storage automatically
   - **Acceptance:** costs scale with actual usage

## F-15.18.9 Backup and Disaster Recovery

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.18.9.1 | server admin (P-22)    |          |              |
| US-15.18.9.2 | DevOps engineer (P-16) |          |              |
| US-15.18.9.3 | developer (P-15)       |          |              |
| US-15.18.9.4 | engine tester (P-27)   |          |              |

1. **US-15.18.9.1** — RDS automated backups, S3 versioning, and DynamoDB point-in-time recovery
   enabled by default
   - **Acceptance:** data is recoverable without manual backup scripts
2. **US-15.18.9.2** — a restore CLI tool that recovers all server data from any backup point
   - **Acceptance:** full service is restored after catastrophic failure without manual database
     imports
3. **US-15.18.9.3** — CloudWatch alarms notifying me if a backup fails
   - **Acceptance:** I am aware of backup issues before I need to recover
4. **US-15.18.9.4** — to verify that enterprise cross-region replication works correctly
   - **Acceptance:** a regional outage does not cause permanent data loss

## F-15.18.10 Enterprise Security Configuration

| ID            | Persona                | Features | Requirements |
|---------------|------------------------|----------|--------------|
| US-15.18.10.1 | server admin (P-22)    |          |              |
| US-15.18.10.2 | DevOps engineer (P-16) |          |              |
| US-15.18.10.3 | server admin (P-22)    |          |              |
| US-15.18.10.4 | engine tester (P-27)   |          |              |

1. **US-15.18.10.1** — databases in private subnets accessible only via VPC with public subnets
   reserved for load balancers
   - **Acceptance:** sensitive data is never directly exposed to the internet
2. **US-15.18.10.2** — AWS WAF rules on all Application Load Balancers blocking SQL injection, XSS,
   and excessive request rates
   - **Acceptance:** services are protected without a separate firewall
3. **US-15.18.10.3** — Secrets Manager automatic rotation for database credentials and API keys
   - **Acceptance:** compromised credentials expire before exploitation
4. **US-15.18.10.4** — to verify that CloudTrail logging and GuardDuty threat detection are enabled
   in the enterprise profile
   - **Acceptance:** security controls are auditable

## F-15.18.11 AWS CDK Open-Source Service Stacks

| ID            | Persona                 | Features | Requirements |
|---------------|-------------------------|----------|--------------|
| US-15.18.11.1 | DevOps engineer (P-16)  |          |              |
| US-15.18.11.2 | developer (P-15)        |          |              |
| US-15.18.11.3 | executive (P-1)         |          |              |
| US-15.18.11.4 | engine developer (P-26) |          |              |

1. **US-15.18.11.1** — CDK stacks that deploy only open-source services (PostgreSQL, Redis,
   Keycloak, NATS, Forgejo)
   - **Acceptance:** my infrastructure is portable to any cloud or on-premises deployment
2. **US-15.18.11.2** — local Docker Compose configs using the same open-source services (MinIO,
   PostgreSQL, Redis, NATS)
   - **Acceptance:** I develop and test without an AWS account
3. **US-15.18.11.3** — all infrastructure dependencies to be open source
   - **Acceptance:** my studio can migrate to another cloud provider or self-host on bare metal
     without rewriting services
4. **US-15.18.11.4** — service stacks using standard protocols (S3 API, NATS, PostgreSQL wire
   protocol)
   - **Acceptance:** integration tests run against local containers in CI without AWS credentials

## F-15.18.12 1-Click AWS Marketplace Deployment

| ID            | Persona             | Features | Requirements |
|---------------|---------------------|----------|--------------|
| US-15.18.12.1 | developer (P-15)    |          |              |
| US-15.18.12.2 | executive (P-1)     |          |              |
| US-15.18.12.3 | server admin (P-22) |          |              |

1. **US-15.18.12.1** — to deploy all Harmonius services from the AWS Marketplace with a guided
   wizard
   - **Acceptance:** I get a working backend without learning AWS CDK or CloudFormation
2. **US-15.18.12.2** — a free Marketplace listing with no Marketplace fees
   - **Acceptance:** my team pays only the underlying AWS infrastructure costs
3. **US-15.18.12.3** — the wizard to display estimated monthly cost for each scaling profile
   - **Acceptance:** I set budget expectations before provisioning resources

## F-15.18.13 Service Admin Dashboard

| ID            | Persona                | Features | Requirements |
|---------------|------------------------|----------|--------------|
| US-15.18.13.1 | server admin (P-22)    |          |              |
| US-15.18.13.2 | DevOps engineer (P-16) |          |              |
| US-15.18.13.3 | executive (P-1)        |          |              |

1. **US-15.18.13.1** — a Grafana dashboard showing the health, CPU, memory, and disk usage of all
   services
   - **Acceptance:** I detect problems from a single page
2. **US-15.18.13.2** — alerts routed to Slack and PagerDuty via NATS
   - **Acceptance:** on-call engineers are notified within seconds of a service degradation
3. **US-15.18.13.3** — cost estimates visible on the admin dashboard
   - **Acceptance:** I monitor infrastructure spend without logging into the AWS billing console

## F-15.18.14 Scaling Profiles

| ID            | Persona                | Features | Requirements |
|---------------|------------------------|----------|--------------|
| US-15.18.14.1 | developer (P-15)       |          |              |
| US-15.18.14.2 | DevOps engineer (P-16) |          |              |
| US-15.18.14.3 | server admin (P-22)    |          |              |
| US-15.18.14.4 | executive (P-1)        |          |              |

1. **US-15.18.14.1** — a Solo scaling profile costing ~$20/month
   - **Acceptance:** I run all services on my personal AWS account affordably
2. **US-15.18.14.2** — a Team profile at ~$100/mo for 2-10 users
   - **Acceptance:** a small team shares services without over-provisioning
3. **US-15.18.14.3** — a Studio profile at ~$500/mo with multi-AZ
   - **Acceptance:** a 10-50 person studio has reliable infrastructure with failover
4. **US-15.18.14.4** — a Production profile at ~$2000+/mo with auto-scaling and read replicas
   - **Acceptance:** live game services handle thousands of concurrent players

## F-15.18.15 Self-Hosted Build Cache Service

| ID            | Persona                 | Features | Requirements |
|---------------|-------------------------|----------|--------------|
| US-15.18.15.1 | developer (P-15)        |          |              |
| US-15.18.15.2 | DevOps engineer (P-16)  |          |              |
| US-15.18.15.3 | engine developer (P-26) |          |              |

1. **US-15.18.15.1** — a build cache with a REST API returning pre-compiled assets by content hash
   - **Acceptance:** my editor skips local compilation and starts faster
2. **US-15.18.15.2** — Redis-backed cache index lookups completing in under 5 ms
   - **Acceptance:** cache hit checks do not slow down build pipelines
3. **US-15.18.15.3** — Keycloak JWT authentication on the cache API
   - **Acceptance:** only authorized users can read or write build artifacts

## F-15.18.16 Self-Hosted Collaboration Service

| ID            | Persona                 | Features | Requirements |
|---------------|-------------------------|----------|--------------|
| US-15.18.16.1 | developer (P-15)        |          |              |
| US-15.18.16.2 | server admin (P-22)     |          |              |
| US-15.18.16.3 | engine developer (P-26) |          |              |

1. **US-15.18.16.1** — self-hosted collaboration using PostgreSQL and Redis
   - **Acceptance:** real-time multi-user editing works without proprietary managed services
2. **US-15.18.16.2** — Keycloak RBAC for collaboration sessions
   - **Acceptance:** project access is controlled by my organization's identity provider
3. **US-15.18.16.3** — the self-hosted collaboration service to implement the same WebSocket
   protocol as managed hosting
   - **Acceptance:** the editor client code is identical for both deployment modes

## F-15.18.17 Self-Hosted Matchmaking Service

| ID            | Persona                | Features | Requirements |
|---------------|------------------------|----------|--------------|
| US-15.18.17.1 | server admin (P-22)    |          |              |
| US-15.18.17.2 | developer (P-15)       |          |              |
| US-15.18.17.3 | DevOps engineer (P-16) |          |              |

1. **US-15.18.17.1** — a self-hosted matchmaking service using PostgreSQL and Redis
   - **Acceptance:** player data and match history stay on my infrastructure
2. **US-15.18.17.2** — ELO/Glicko-2 skill rating with region-based matching
   - **Acceptance:** players are matched fairly with low latency
3. **US-15.18.17.3** — NATS-based match events
   - **Acceptance:** game servers receive allocation requests in real time without polling

## F-15.18.18 Self-Hosted Asset Store Service

| ID            | Persona                 | Features | Requirements |
|---------------|-------------------------|----------|--------------|
| US-15.18.18.1 | developer (P-15)        |          |              |
| US-15.18.18.2 | server admin (P-22)     |          |              |
| US-15.18.18.3 | engine developer (P-26) |          |              |

1. **US-15.18.18.1** — a self-hosted asset store with full-text search
   - **Acceptance:** I browse and download assets from my studio's private repository
2. **US-15.18.18.2** — OpenSearch-powered search returning results in under 200 ms
   - **Acceptance:** the asset browser feels responsive
3. **US-15.18.18.3** — automated asset upload validation
   - **Acceptance:** only well-formed, correctly versioned packages are accepted into the store

## F-15.18.19 Health Monitoring Stack

| ID            | Persona                 | Features | Requirements |
|---------------|-------------------------|----------|--------------|
| US-15.18.19.1 | server admin (P-22)     |          |              |
| US-15.18.19.2 | DevOps engineer (P-16)  |          |              |
| US-15.18.19.3 | engine developer (P-26) |          |              |

1. **US-15.18.19.1** — Prometheus + Grafana monitoring with pre-configured dashboards for all
   services
   - **Acceptance:** I use open-source tools instead of CloudWatch
2. **US-15.18.19.2** — Loki log aggregation
   - **Acceptance:** I search structured logs across all services from a single Grafana interface
3. **US-15.18.19.3** — pre-configured alert rules for each service (PostgreSQL replication lag,
   Redis memory, NATS slow consumers)
   - **Acceptance:** common failure modes trigger notifications without manual configuration

## F-15.18.20 Multi-Region Deployment

| ID            | Persona                | Features | Requirements |
|---------------|------------------------|----------|--------------|
| US-15.18.20.1 | server admin (P-22)    |          |              |
| US-15.18.20.2 | DevOps engineer (P-16) |          |              |
| US-15.18.20.3 | executive (P-1)        |          |              |

1. **US-15.18.20.1** — multi-region deployment with PostgreSQL read replicas and Redis replicas
   - **Acceptance:** users in different geographic regions get low-latency reads
2. **US-15.18.20.2** — DNS-based failover via Route 53 health checks
   - **Acceptance:** a region failure automatically redirects traffic to the healthy region
3. **US-15.18.20.3** — multi-region disaster recovery
   - **Acceptance:** a regional AWS outage does not halt game operations or cause data loss
