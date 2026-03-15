# 15.18 — Self-Hosted Server Infrastructure

## AWS CDK Deployment

### F-15.18.1 AWS CDK Deployment Stacks
Open-source AWS CDK (Cloud Development Kit) stacks in TypeScript/Rust that deploy all Harmonius
server components to a developer's own AWS account. Stacks are modular — deploy only what you need.
Each stack configures VPC networking, IAM roles, security groups, encryption (at rest and in
transit), CloudWatch monitoring, and auto-scaling. Two deployment profiles: **Free Tier** (single-AZ,
t3.micro/t4g.micro instances, minimal storage, suitable for solo developers and prototyping) and
**Enterprise** (multi-AZ, auto-scaling groups, RDS Multi-AZ, ElastiCache, CloudFront CDN, WAF,
suitable for production studios). A single `cdk deploy --profile free-tier` or
`cdk deploy --profile enterprise` command provisions the entire infrastructure. Stack outputs include
endpoint URLs, connection strings, and API keys. All stacks are version-pinned to the engine release
for compatibility.
- **Requirements:** R-15.18.1
- **Dependencies:** None (standalone infrastructure)
- **Platform notes:** AWS only. Requires AWS CLI and CDK CLI. Free tier stays within AWS Free Tier
  limits (750 hours t3.micro, 5GB S3, 20GB RDS).

### F-15.18.2 Live Collaboration Server
A self-hosted CRDT-based real-time collaboration server (the backend for F-15.12.3-14). Deployed as
an ECS Fargate container or EC2 instance. Uses PostgreSQL (RDS) for persistent document state, S3
for large binary assets, and WebSocket connections for real-time sync. The server handles: multi-user
editing sessions, presence tracking, access control, voice/text chat relay, and AI agent
collaboration channels. CDK stack configures: RDS PostgreSQL (db.t3.micro for free tier,
db.r6g.large for enterprise), S3 bucket with lifecycle policies, Application Load Balancer with
WebSocket support, and ECS task definitions.
- **Requirements:** R-15.18.2
- **Dependencies:** F-15.18.1, F-15.12.7 (Cloud Collaboration Service)
- **Platform notes:** Free tier: single Fargate task, db.t3.micro RDS. Enterprise: auto-scaling
  Fargate, Multi-AZ RDS.

### F-15.18.3 Git and Git LFS Hosting with Locking
A self-hosted Git server with Git LFS support and file locking. Deployed as a Gitea or Forgejo
instance on ECS/EC2 with S3-backed LFS storage. Supports the engine's version control features
(F-15.10.1-8): push, pull, branch, merge, LFS lock/unlock, and multi-provider API compatibility
(GitHub-compatible REST API so the editor's Git integration works without modification). LFS objects
are stored in S3 with configurable lifecycle policies (move to Glacier after 90 days). CDK stack
configures: EC2/Fargate for the Git server, S3 for LFS storage, EBS for Git repository data, and
Route 53 for DNS.
- **Requirements:** R-15.18.3
- **Dependencies:** F-15.18.1, F-15.10.1 (Git Integration), F-15.10.2 (Git LFS)
- **Platform notes:** Free tier: t3.micro EC2, 5GB S3. Enterprise: m6i.large, unlimited S3,
  CloudFront for LFS downloads.

### F-15.18.4 Asset and Shader Compilation Server
A self-hosted build farm for asset cooking and shader compilation. Deployed as an auto-scaling group
of GPU-enabled instances (for shader compilation) or CPU instances (for asset cooking). The
compilation server processes jobs from a Redis/SQS queue: texture compression, LOD generation,
meshlet building, HLSL-to-DXIL/SPIR-V/MSL compilation, and logic graph bytecode generation. Results
are stored in the shared build cache (S3-backed, F-15.11.1). CDK stack configures: SQS job queue,
auto-scaling EC2 (c6i for CPU, g5 for GPU), S3 for build cache, and CloudWatch alarms for queue
depth.
- **Requirements:** R-15.18.4
- **Dependencies:** F-15.18.1, F-15.11.1 (Shared Build Cache), F-12.2.9 (Shader Pipeline)
- **Platform notes:** Free tier: single t3.micro (CPU-only, slow). Enterprise: auto-scaling
  c6i.2xlarge + g5.xlarge spot instances.

### F-15.18.5 Signing and Distribution Server
A self-hosted code signing and distribution packaging server. Stores signing credentials in AWS
Secrets Manager (never on disk). Automates: iOS signing with provisioning profiles, macOS
notarization submission, Android keystore signing, Windows Authenticode signing, and package
generation (.dmg, .msi, .deb, AppImage). Integrates with the store distribution pipeline
(F-15.14.8) for automated submissions. CDK stack configures: EC2 build agent, Secrets Manager for
credentials, S3 for build artifacts, and CodePipeline for orchestration.
- **Requirements:** R-15.18.5
- **Dependencies:** F-15.18.1, F-15.14.4 (Code Signing), F-15.14.5 (Installers)
- **Platform notes:** iOS signing requires a macOS build agent (Mac EC2 or external).

### F-15.18.6 Continuous Deployment Pipeline
A self-hosted CI/CD pipeline for building, testing, and deploying game builds. Uses AWS CodePipeline
with CodeBuild for Rust compilation, asset cooking, test execution, and deployment to
staging/production environments. The pipeline triggers on Git push events (via webhook from the Git
server F-15.18.3). Pipeline stages: source checkout, Rust build (debug/release), asset cook
(F-15.18.4), automated tests (unit, integration, screenshot), packaging (F-15.18.5), deployment to
S3/CloudFront for distribution. CDK stack configures the entire pipeline with configurable stages.
- **Requirements:** R-15.18.6
- **Dependencies:** F-15.18.1, F-15.18.3 (Git Server), F-15.18.4 (Build Farm),
  F-15.18.5 (Signing)
- **Platform notes:** Free tier: single CodeBuild project. Enterprise: parallel builds, multiple
  environments.

### F-15.18.7 Test Runner Infrastructure
A self-hosted automated testing infrastructure for continuous quality assurance. Deploys test runner
instances that execute: unit tests (Rust cargo test), integration tests (multi-system scenarios),
screenshot comparison tests (render a scene, compare against golden images), performance benchmark
tests (measure frame time, draw calls, memory), and stress tests (spawn N entities, simulate M
seconds). Test results are stored in DynamoDB with CloudWatch dashboards for trend analysis. Failed
tests block the deployment pipeline (F-15.18.6). CDK stack configures: CodeBuild projects per test
type, DynamoDB for results, S3 for golden images, and SNS for failure notifications.
- **Requirements:** R-15.18.7
- **Dependencies:** F-15.18.1, F-15.18.6 (CI/CD Pipeline)
- **Platform notes:** GPU tests require g5 instances. Free tier uses CPU-only tests.

### F-15.18.8 Shared Cache and Database Services
Self-hosted shared cache (S3-backed) and database services (PostgreSQL, DynamoDB) for all engine
server components. The shared cache stores: compiled assets, shader bytecode, logic graph bytecode,
terrain chunks, and universe generation output. Databases store: project metadata, user accounts,
collaboration state, crash reports, telemetry, and matchmaking data. CDK stack configures: S3 with
intelligent tiering, RDS PostgreSQL, DynamoDB tables, ElastiCache Redis for hot data, and backup
schedules. Data encryption at rest (KMS) and in transit (TLS) is enforced.
- **Requirements:** R-15.18.8
- **Dependencies:** F-15.18.1, F-15.11.1 (Shared Build Cache), F-8.7.5 (Persistent World State)
- **Platform notes:** Free tier: S3 standard, db.t3.micro RDS, DynamoDB on-demand. Enterprise: S3
  Intelligent-Tiering, db.r6g.large Multi-AZ, DynamoDB provisioned.

### F-15.18.9 Backup and Disaster Recovery
Automated backup and restore for all self-hosted server data. RDS automated backups with
configurable retention (7-35 days). S3 versioning for asset storage with lifecycle policies.
DynamoDB point-in-time recovery. Cross-region replication for enterprise disaster recovery. A
restore CLI tool recovers from any backup point to a new or existing stack. Backup status is
monitored via CloudWatch alarms. CDK stack configures all backup policies and the cross-region
replication topology.
- **Requirements:** R-15.18.9
- **Dependencies:** F-15.18.1, F-15.18.8 (Databases)
- **Platform notes:** Free tier: 7-day RDS backups, S3 versioning. Enterprise: 35-day,
  cross-region.

### F-15.18.10 Enterprise Security Configuration
Enterprise-grade security for all self-hosted components. VPC with private subnets for databases and
internal services, public subnets only for load balancers. IAM roles with least-privilege policies
per service. AWS WAF on Application Load Balancers to block common attacks (SQL injection, XSS, rate
limiting). KMS encryption for all data at rest. TLS 1.3 for all data in transit. CloudTrail audit
logging for all API calls. GuardDuty for threat detection. Secrets Manager rotation for database
credentials and API keys. The CDK stack enforces these security controls by default in enterprise
profile; the free tier profile relaxes some controls (single AZ, no WAF, no GuardDuty) to stay
within cost limits.
- **Requirements:** R-15.18.10
- **Dependencies:** F-15.18.1
- **Platform notes:** Enterprise security adds ~$50-100/month in AWS costs. Free tier uses default
  VPC.
