# User Stories: Self-Hosted Server Infrastructure

## F-15.18.1 AWS CDK Deployment Stacks

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.18.1.1 | developer (P-15) | to deploy all Harmonius server components to my own AWS account with a single `cdk deploy --profile free-tier` command | I get collaboration server, Git hosting, and build cache without manually provisioning resources |  |  |
| US-15.18.1.2 | DevOps engineer (P-16) | to deploy only the CDK stacks my team needs (e.g., Git server and build farm but not collaboration) | I avoid paying for unused infrastructure |  |  |
| US-15.18.1.3 | server admin (P-22) | CDK stacks version-pinned to the engine release | infrastructure upgrades are coordinated with engine updates |  |  |
| US-15.18.1.4 | engine tester (P-27) | to verify that the Free Tier profile stays within AWS Free Tier limits (750h t3.micro, 5GB S3, 20GB RDS) | solo developers incur near-zero cloud costs |  |  |

## F-15.18.2 Live Collaboration Server

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.18.2.1 | server admin (P-22) | the collaboration server deployed in my own AWS account with all data in my RDS and S3 buckets | project data satisfies data residency requirements |  |  |
| US-15.18.2.2 | developer (P-15) | a collaboration server enabling CRDT-based editing sessions over WebSocket | my team edits simultaneously without file locking |  |  |
| US-15.18.2.3 | DevOps engineer (P-16) | the enterprise server to auto-scale Fargate tasks based on active session count | the server handles peak usage without manual capacity planning |  |  |
| US-15.18.2.4 | engine tester (P-27) | to verify that the collaboration server correctly handles multi-user editing sessions, presence tracking, access control, and chat relay | all collaboration features work on self-hosted infrastructure |  |  |

## F-15.18.3 Git and Git LFS Hosting with Locking

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.18.3.1 | developer (P-15) | a self-hosted Git server with LFS support backed by S3 storage | I host source and binary assets without per-seat hosting fees |  |  |
| US-15.18.3.2 | DevOps engineer (P-16) | the self-hosted Git server to expose a GitHub-compatible REST API | the editor's Git integration works without configuration changes |  |  |
| US-15.18.3.3 | server admin (P-22) | LFS objects automatically moved to Glacier after a configurable retention period | storage costs for old assets decrease over time |  |  |
| US-15.18.3.4 | engine tester (P-27) | to verify that LFS lock and unlock operations work correctly through the self-hosted server | file locking prevents concurrent binary edits |  |  |

## F-15.18.4 Asset and Shader Compilation Server

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.18.4.1 | developer (P-15) | to submit shader compilation and asset cooking jobs to a remote build farm | my laptop stays responsive while builds run on dedicated cloud instances |  |  |
| US-15.18.4.2 | DevOps engineer (P-16) | the build farm to auto-scale EC2 instances based on SQS queue depth | compilation throughput increases automatically during content freezes |  |  |
| US-15.18.4.3 | server admin (P-22) | compiled results stored in the S3-backed shared build cache | each asset is compiled at most once across the team |  |  |
| US-15.18.4.4 | engine tester (P-27) | to verify that GPU-enabled instances (g5) compile shaders correctly for all target platforms | remote compilation matches local output |  |  |

## F-15.18.5 Signing and Distribution Server

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.18.5.1 | server admin (P-22) | signing credentials stored in AWS Secrets Manager with automatic rotation | credentials are never on disk or in version control |  |  |
| US-15.18.5.2 | DevOps engineer (P-16) | the signing server to sign builds for iOS, macOS, Android, and Windows as part of the packaging pipeline | manual signing is eliminated |  |  |
| US-15.18.5.3 | developer (P-15) | a single command that signs and packages my game for all target platforms | I publish to multiple stores without learning each signing toolchain |  |  |
| US-15.18.5.4 | engine tester (P-27) | to verify that all signing operations are logged in CloudTrail with non-repudiation | the studio has a complete audit trail |  |  |

## F-15.18.6 Continuous Deployment Pipeline

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.18.6.1 | DevOps engineer (P-16) | a CI/CD pipeline triggering on every Git push that builds, cooks assets, runs tests, packages, and deploys to staging | every commit is validated without manual intervention |  |  |
| US-15.18.6.2 | developer (P-15) | a single CodeBuild project on the free tier that builds, tests, and packages on every push | I get CI without configuring Jenkins or paying for hosted CI |  |  |
| US-15.18.6.3 | server admin (P-22) | to enable or disable pipeline stages (skip GPU tests, skip store submission) via CDK configuration | the pipeline adapts to the project's current phase |  |  |
| US-15.18.6.4 | engine tester (P-27) | to verify that the full pipeline (source checkout, build, asset cook, test, package, deploy) completes successfully | automated releases are reliable |  |  |

## F-15.18.7 Test Runner Infrastructure

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.18.7.1 | DevOps engineer (P-16) | screenshot comparison tests comparing rendered scenes against golden images | unintended rendering changes are caught before reaching players |  |  |
| US-15.18.7.2 | server admin (P-22) | test results stored in DynamoDB with CloudWatch dashboards showing frame time, draw call, and memory trends | gradual performance degradation is detected early |  |  |
| US-15.18.7.3 | developer (P-15) | SNS notifications (email or Slack webhook) when tests fail | I can fix issues without polling the CI dashboard |  |  |
| US-15.18.7.4 | engine tester (P-27) | to verify that failed tests block the deployment pipeline from advancing to packaging | no build ships without passing all quality checks |  |  |

## F-15.18.8 Shared Cache and Database Services

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.18.8.1 | DevOps engineer (P-16) | all server components to share a single set of database and cache services (RDS, DynamoDB, ElastiCache, S3) configured by one CDK stack | data infrastructure is managed in one place |  |  |
| US-15.18.8.2 | developer (P-15) | the shared S3 cache to return compiled assets by content hash on editor startup | I skip local compilation and start working in minutes |  |  |
| US-15.18.8.3 | server admin (P-22) | KMS encryption on all storage and TLS on all connections | project data is protected at rest and in transit |  |  |
| US-15.18.8.4 | engine tester (P-27) | to verify that S3 Intelligent-Tiering moves infrequently accessed assets to cheaper storage automatically | costs scale with actual usage |  |  |

## F-15.18.9 Backup and Disaster Recovery

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.18.9.1 | server admin (P-22) | RDS automated backups, S3 versioning, and DynamoDB point-in-time recovery enabled by default | data is recoverable without manual backup scripts |  |  |
| US-15.18.9.2 | DevOps engineer (P-16) | a restore CLI tool that recovers all server data from any backup point | full service is restored after catastrophic failure without manual database imports |  |  |
| US-15.18.9.3 | developer (P-15) | CloudWatch alarms notifying me if a backup fails | I am aware of backup issues before I need to recover |  |  |
| US-15.18.9.4 | engine tester (P-27) | to verify that enterprise cross-region replication works correctly | a regional outage does not cause permanent data loss |  |  |

## F-15.18.10 Enterprise Security Configuration

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.18.10.1 | server admin (P-22) | databases in private subnets accessible only via VPC with public subnets reserved for load balancers | sensitive data is never directly exposed to the internet |  |  |
| US-15.18.10.2 | DevOps engineer (P-16) | AWS WAF rules on all Application Load Balancers blocking SQL injection, XSS, and excessive request rates | services are protected without a separate firewall |  |  |
| US-15.18.10.3 | server admin (P-22) | Secrets Manager automatic rotation for database credentials and API keys | compromised credentials expire before exploitation |  |  |
| US-15.18.10.4 | engine tester (P-27) | to verify that CloudTrail logging and GuardDuty threat detection are enabled in the enterprise profile | security controls are auditable |  |  |

## F-15.18.11 AWS CDK Open-Source Service Stacks

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.18.11.1 | DevOps engineer (P-16) | CDK stacks that deploy only open-source services (PostgreSQL, Redis, Keycloak, NATS, Forgejo) | my infrastructure is portable to any cloud or on-premises deployment |  |  |
| US-15.18.11.2 | developer (P-15) | local Docker Compose configs using the same open-source services (MinIO, PostgreSQL, Redis, NATS) | I develop and test without an AWS account |  |  |
| US-15.18.11.3 | executive (P-1) | all infrastructure dependencies to be open source | my studio can migrate to another cloud provider or self-host on bare metal without rewriting services |  |  |
| US-15.18.11.4 | engine developer (P-26) | service stacks using standard protocols (S3 API, NATS, PostgreSQL wire protocol) | integration tests run against local containers in CI without AWS credentials |  |  |

## F-15.18.12 1-Click AWS Marketplace Deployment

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.18.12.1 | developer (P-15) | to deploy all Harmonius services from the AWS Marketplace with a guided wizard | I get a working backend without learning AWS CDK or CloudFormation |  |  |
| US-15.18.12.2 | executive (P-1) | a free Marketplace listing with no Marketplace fees | my team pays only the underlying AWS infrastructure costs |  |  |
| US-15.18.12.3 | server admin (P-22) | the wizard to display estimated monthly cost for each scaling profile | I set budget expectations before provisioning resources |  |  |

## F-15.18.13 Service Admin Dashboard

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.18.13.1 | server admin (P-22) | a Grafana dashboard showing the health, CPU, memory, and disk usage of all services | I detect problems from a single page |  |  |
| US-15.18.13.2 | DevOps engineer (P-16) | alerts routed to Slack and PagerDuty via NATS | on-call engineers are notified within seconds of a service degradation |  |  |
| US-15.18.13.3 | executive (P-1) | cost estimates visible on the admin dashboard | I monitor infrastructure spend without logging into the AWS billing console |  |  |

## F-15.18.14 Scaling Profiles

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.18.14.1 | developer (P-15) | a Solo scaling profile costing ~$20/month | I run all services on my personal AWS account affordably |  |  |
| US-15.18.14.2 | DevOps engineer (P-16) | a Team profile at ~$100/mo for 2-10 users | a small team shares services without over-provisioning |  |  |
| US-15.18.14.3 | server admin (P-22) | a Studio profile at ~$500/mo with multi-AZ | a 10-50 person studio has reliable infrastructure with failover |  |  |
| US-15.18.14.4 | executive (P-1) | a Production profile at ~$2000+/mo with auto-scaling and read replicas | live game services handle thousands of concurrent players |  |  |

## F-15.18.15 Self-Hosted Build Cache Service

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.18.15.1 | developer (P-15) | a build cache with a REST API returning pre-compiled assets by content hash | my editor skips local compilation and starts faster |  |  |
| US-15.18.15.2 | DevOps engineer (P-16) | Redis-backed cache index lookups completing in under 5 ms | cache hit checks do not slow down build pipelines |  |  |
| US-15.18.15.3 | engine developer (P-26) | Keycloak JWT authentication on the cache API | only authorized users can read or write build artifacts |  |  |

## F-15.18.16 Self-Hosted Collaboration Service

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.18.16.1 | developer (P-15) | self-hosted collaboration using PostgreSQL and Redis | real-time multi-user editing works without proprietary managed services |  |  |
| US-15.18.16.2 | server admin (P-22) | Keycloak RBAC for collaboration sessions | project access is controlled by my organization's identity provider |  |  |
| US-15.18.16.3 | engine developer (P-26) | the self-hosted collaboration service to implement the same WebSocket protocol as managed hosting | the editor client code is identical for both deployment modes |  |  |

## F-15.18.17 Self-Hosted Matchmaking Service

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.18.17.1 | server admin (P-22) | a self-hosted matchmaking service using PostgreSQL and Redis | player data and match history stay on my infrastructure |  |  |
| US-15.18.17.2 | developer (P-15) | ELO/Glicko-2 skill rating with region-based matching | players are matched fairly with low latency |  |  |
| US-15.18.17.3 | DevOps engineer (P-16) | NATS-based match events | game servers receive allocation requests in real time without polling |  |  |

## F-15.18.18 Self-Hosted Asset Store Service

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.18.18.1 | developer (P-15) | a self-hosted asset store with full-text search | I browse and download assets from my studio's private repository |  |  |
| US-15.18.18.2 | server admin (P-22) | OpenSearch-powered search returning results in under 200 ms | the asset browser feels responsive |  |  |
| US-15.18.18.3 | engine developer (P-26) | automated asset upload validation | only well-formed, correctly versioned packages are accepted into the store |  |  |

## F-15.18.19 Health Monitoring Stack

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.18.19.1 | server admin (P-22) | Prometheus + Grafana monitoring with pre-configured dashboards for all services | I use open-source tools instead of CloudWatch |  |  |
| US-15.18.19.2 | DevOps engineer (P-16) | Loki log aggregation | I search structured logs across all services from a single Grafana interface |  |  |
| US-15.18.19.3 | engine developer (P-26) | pre-configured alert rules for each service (PostgreSQL replication lag, Redis memory, NATS slow consumers) | common failure modes trigger notifications without manual configuration |  |  |

## F-15.18.20 Multi-Region Deployment

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.18.20.1 | server admin (P-22) | multi-region deployment with PostgreSQL read replicas and Redis replicas | users in different geographic regions get low-latency reads |  |  |
| US-15.18.20.2 | DevOps engineer (P-16) | DNS-based failover via Route 53 health checks | a region failure automatically redirects traffic to the healthy region |  |  |
| US-15.18.20.3 | executive (P-1) | multi-region disaster recovery | a regional AWS outage does not halt game operations or cause data loss |  |  |
