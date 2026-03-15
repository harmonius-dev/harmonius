# R-15.18 -- Self-Hosted Server Infrastructure Requirements

## R-15.18.1 AWS CDK Deployment Stacks
The engine **SHALL** provide open-source AWS CDK stacks that deploy all Harmonius server components
to a developer's own AWS account with a single CLI command. Stacks SHALL be modular, allowing
independent deployment of individual components. Each stack SHALL configure VPC networking, IAM roles
with least-privilege policies, security groups, encryption at rest (KMS) and in transit (TLS),
CloudWatch monitoring, and auto-scaling. Two deployment profiles SHALL be supported: **Free Tier**
(single-AZ, t3.micro/t4g.micro instances, minimal storage) and **Enterprise** (multi-AZ,
auto-scaling groups, RDS Multi-AZ, ElastiCache, CloudFront CDN, WAF). Stack outputs SHALL include
endpoint URLs, connection strings, and API keys. All stacks SHALL be version-pinned to the engine
release.
- **Derived from:** [F-15.18.1](../../features/tools-editor/server-infrastructure.md)
- **Rationale:** Self-hosted infrastructure gives developers full control over their data, avoids
  vendor lock-in to a managed service, and allows the engine to ship without requiring Harmonius to
  operate cloud infrastructure on behalf of users.
- **Verification:** Run `cdk deploy --profile free-tier` against a clean AWS account and confirm all
  stacks provision successfully within 15 minutes; confirm stack outputs contain valid endpoint URLs
  and connection strings; confirm IAM roles use least-privilege policies; confirm encryption is
  enabled on all storage resources; repeat with `--profile enterprise` and confirm multi-AZ, WAF,
  and auto-scaling are configured; destroy and redeploy to confirm idempotency.

## R-15.18.2 Live Collaboration Server
The engine **SHALL** deploy a self-hosted CRDT-based real-time collaboration server as an ECS
Fargate container or EC2 instance. The server SHALL handle multi-user editing sessions, presence
tracking, access control, voice/text chat relay, and AI agent collaboration channels. The server
SHALL use RDS PostgreSQL for persistent document state, S3 for large binary assets, and WebSocket
connections for real-time sync. The CDK stack SHALL configure RDS, S3 with lifecycle policies,
Application Load Balancer with WebSocket support, and ECS task definitions.
- **Derived from:** [F-15.18.2](../../features/tools-editor/server-infrastructure.md)
- **Rationale:** Self-hosted collaboration keeps all project data within the developer's
  infrastructure, satisfying data residency requirements and eliminating dependency on third-party
  collaboration services.
- **Verification:** Deploy the collaboration stack; connect two editor instances to the server and
  confirm real-time CRDT sync of entity edits; confirm presence indicators show both users; confirm
  chat messages relay between users; disconnect one user and confirm the other continues without
  interruption; confirm data persists in RDS after server restart.

## R-15.18.3 Git and Git LFS Hosting with Locking
The engine **SHALL** deploy a self-hosted Git server (Gitea or Forgejo) with Git LFS support and
file locking on ECS/EC2 with S3-backed LFS storage. The server SHALL expose a GitHub-compatible REST
API so the editor's Git integration (F-15.10.1-8) works without modification. LFS objects SHALL be
stored in S3 with configurable lifecycle policies. The CDK stack SHALL configure the Git server
instance, S3 for LFS storage, EBS for repository data, and Route 53 for DNS.
- **Derived from:** [F-15.18.3](../../features/tools-editor/server-infrastructure.md)
- **Rationale:** Self-hosted Git with LFS eliminates per-seat licensing costs for hosted Git
  services and keeps all source code and binary assets within the developer's infrastructure.
- **Verification:** Deploy the Git stack; push a repository with LFS-tracked binary assets from the
  editor; confirm push, pull, branch, and merge operations work via the editor's Git integration
  panel; lock an LFS file and confirm another user cannot push changes to it; confirm LFS objects
  are stored in S3; confirm lifecycle policies move old LFS objects to the configured storage tier.

## R-15.18.4 Asset and Shader Compilation Server
The engine **SHALL** deploy a self-hosted build farm for asset cooking and shader compilation as an
auto-scaling group of EC2 instances. The compilation server SHALL process jobs from an SQS queue:
texture compression, LOD generation, meshlet building, HLSL-to-DXIL/SPIR-V/MSL compilation, and
logic graph bytecode generation. Compiled results SHALL be stored in the shared build cache
(S3-backed, F-15.11.1). The CDK stack SHALL configure the SQS job queue, auto-scaling EC2 instances,
S3 for build cache, and CloudWatch alarms for queue depth.
- **Derived from:** [F-15.18.4](../../features/tools-editor/server-infrastructure.md)
- **Rationale:** A shared compilation server eliminates redundant builds across team members,
  accelerates iteration by offloading heavy compilation to dedicated hardware, and centralizes the
  build cache for the entire team.
- **Verification:** Deploy the build farm stack; submit a shader compilation job from the editor and
  confirm the compiled result appears in the shared build cache; submit a texture compression job
  and confirm the output is cached; confirm a second request for the same job returns the cached
  result without recompilation; confirm auto-scaling adds instances when queue depth exceeds the
  configured threshold.

## R-15.18.5 Signing and Distribution Server
The engine **SHALL** deploy a self-hosted code signing and distribution packaging server on EC2. The
server SHALL store signing credentials in AWS Secrets Manager (never on disk) and automate: iOS
signing with provisioning profiles, macOS notarization submission, Android keystore signing, Windows
Authenticode signing, and package generation (.dmg, .msi, .deb, AppImage). The CDK stack SHALL
configure the EC2 build agent, Secrets Manager for credentials, S3 for build artifacts, and
CodePipeline for orchestration.
- **Derived from:** [F-15.18.5](../../features/tools-editor/server-infrastructure.md)
- **Rationale:** Centralizing signing credentials in Secrets Manager eliminates the risk of
  credentials being committed to version control or stored on developer workstations, and
  automating signing removes a manual error-prone step from the release process.
- **Verification:** Deploy the signing stack; submit a build for macOS signing and confirm the
  output is code-signed and notarized; confirm signing credentials are stored in Secrets Manager
  and not on disk; confirm the signed artifact is uploaded to S3; repeat for Windows and Android
  signing and confirm valid signatures via platform verification tools.

## R-15.18.6 Continuous Deployment Pipeline
The engine **SHALL** deploy a self-hosted CI/CD pipeline using AWS CodePipeline with CodeBuild. The
pipeline SHALL trigger on Git push events via webhook from the Git server (F-15.18.3). Pipeline
stages SHALL include: source checkout, Rust build (debug/release), asset cook (F-15.18.4), automated
tests (unit, integration, screenshot), packaging (F-15.18.5), and deployment to S3/CloudFront for
distribution. The CDK stack SHALL configure the entire pipeline with configurable stages.
- **Derived from:** [F-15.18.6](../../features/tools-editor/server-infrastructure.md)
- **Rationale:** A fully automated CI/CD pipeline ensures every commit is built, tested, and
  deployable, reducing manual release overhead and catching regressions before they reach players.
- **Verification:** Deploy the CI/CD stack; push a commit to the Git server and confirm the pipeline
  triggers automatically; confirm each stage (build, cook, test, package, deploy) executes
  successfully; push a commit with a failing test and confirm the pipeline halts at the test stage;
  confirm the final artifact is deployed to S3/CloudFront.

## R-15.18.7 Test Runner Infrastructure
The engine **SHALL** deploy self-hosted test runner instances that execute: unit tests (Rust cargo
test), integration tests, screenshot comparison tests (against golden images), performance benchmark
tests, and stress tests. Test results SHALL be stored in DynamoDB with CloudWatch dashboards for
trend analysis. Failed tests SHALL block the deployment pipeline (F-15.18.6). The CDK stack SHALL
configure CodeBuild projects per test type, DynamoDB for results, S3 for golden images, and SNS for
failure notifications.
- **Derived from:** [F-15.18.7](../../features/tools-editor/server-infrastructure.md)
- **Rationale:** Automated test infrastructure catches visual regressions, performance degradations,
  and functional bugs continuously, and trend dashboards reveal gradual quality drift that
  individual test runs might miss.
- **Verification:** Deploy the test runner stack; trigger a test run and confirm unit, integration,
  and screenshot tests execute; introduce a visual regression and confirm the screenshot comparison
  test fails and blocks the pipeline; confirm test results appear in DynamoDB and on the CloudWatch
  dashboard; confirm SNS sends a failure notification.

## R-15.18.8 Shared Cache and Database Services
The engine **SHALL** deploy self-hosted shared cache (S3-backed) and database services (RDS
PostgreSQL, DynamoDB, ElastiCache Redis) for all server components. The shared cache SHALL store
compiled assets, shader bytecode, logic graph bytecode, terrain chunks, and universe generation
output. Databases SHALL store project metadata, user accounts, collaboration state, crash reports,
telemetry, and matchmaking data. Data encryption at rest (KMS) and in transit (TLS) SHALL be
enforced. The CDK stack SHALL configure S3 with intelligent tiering, RDS, DynamoDB tables,
ElastiCache, and backup schedules.
- **Derived from:** [F-15.18.8](../../features/tools-editor/server-infrastructure.md)
- **Rationale:** Centralized cache and database services eliminate per-component storage
  configuration, provide consistent encryption and backup policies, and enable cross-service data
  sharing (e.g., the build farm and CI pipeline share the same cache).
- **Verification:** Deploy the database stack; confirm RDS, DynamoDB, ElastiCache, and S3 are
  provisioned; write data to each service from a server component and confirm it persists; confirm
  KMS encryption is enabled on RDS, DynamoDB, and S3; confirm TLS is enforced on all connections;
  confirm backup schedules are active.

## R-15.18.9 Backup and Disaster Recovery
The engine **SHALL** provide automated backup and restore for all self-hosted server data. RDS SHALL
have automated backups with configurable retention (7-35 days). S3 SHALL have versioning enabled
with lifecycle policies. DynamoDB SHALL have point-in-time recovery enabled. Enterprise profile SHALL
support cross-region replication. A restore CLI tool SHALL recover from any backup point to a new or
existing stack. Backup status SHALL be monitored via CloudWatch alarms.
- **Derived from:** [F-15.18.9](../../features/tools-editor/server-infrastructure.md)
- **Rationale:** Automated backups with tested restore procedures protect against data loss from
  accidental deletion, corruption, or infrastructure failure, and cross-region replication provides
  business continuity for production studios.
- **Verification:** Deploy the backup stack; confirm RDS automated backups are enabled with the
  configured retention period; delete a record from RDS and restore from backup to confirm data
  recovery; confirm S3 versioning allows retrieval of previous object versions; confirm DynamoDB
  point-in-time recovery restores to a specific timestamp; for enterprise profile, confirm
  cross-region replication by verifying data appears in the replica region.

## R-15.18.10 Enterprise Security Configuration
The engine **SHALL** enforce enterprise-grade security for all self-hosted components in the
enterprise profile. VPC SHALL use private subnets for databases and internal services with public
subnets only for load balancers. IAM roles SHALL use least-privilege policies per service. AWS WAF
SHALL block common attacks (SQL injection, XSS, rate limiting) on Application Load Balancers. KMS
SHALL encrypt all data at rest. TLS 1.3 SHALL encrypt all data in transit. CloudTrail SHALL log all
API calls. GuardDuty SHALL monitor for threats. Secrets Manager SHALL rotate database credentials and
API keys automatically.
- **Derived from:** [F-15.18.10](../../features/tools-editor/server-infrastructure.md)
- **Rationale:** Defense-in-depth security protects studio intellectual property and player data,
  meets compliance requirements for published games, and prevents unauthorized access to build
  infrastructure.
- **Verification:** Deploy the enterprise stack; confirm databases are in private subnets
  inaccessible from the public internet; attempt SQL injection and XSS via the load balancer and
  confirm WAF blocks both; confirm KMS encryption is enabled on all storage; confirm TLS 1.3 is
  enforced on all endpoints; confirm CloudTrail logs API calls; confirm GuardDuty is active;
  confirm Secrets Manager rotates credentials on schedule.

## Non-Functional Requirements

### R-15.18.NF1 Deployment Time
A full `cdk deploy` of all server stacks SHALL complete within 15 minutes on both Free Tier and
Enterprise profiles, measured from command invocation to all stack outputs being available. Individual
stack updates (adding or modifying a single component) SHALL complete within 5 minutes. Stack
destruction via `cdk destroy` SHALL complete within 10 minutes.
- **Derived from:** F-15.18.1 through F-15.18.10 (all server infrastructure features)
- **Rationale:** Fast deployment enables rapid iteration on infrastructure changes, quick disaster
  recovery, and practical CI/CD pipeline testing. Deployments exceeding 15 minutes discourage
  infrastructure-as-code adoption and slow down incident response.
- **Verification:** Run `cdk deploy --profile free-tier --all` against a clean AWS account and
  measure wall-clock time to completion; assert under 15 minutes. Repeat with `--profile
  enterprise`; assert under 15 minutes. Modify a single stack parameter and run `cdk deploy`;
  assert the update completes within 5 minutes. Run `cdk destroy --all`; assert completion within
  10 minutes.

### R-15.18.NF2 Free Tier Monthly Cost
The Free Tier deployment profile SHALL incur no more than $5.00 USD per month in AWS charges when
running all server components continuously (24/7). This budget assumes AWS Free Tier eligibility
(first 12 months) and typical solo-developer usage (fewer than 10 Git pushes per day, fewer than 100
asset compilation jobs per month, fewer than 5 concurrent collaboration sessions). Cost estimates
SHALL be documented per stack and validated monthly via AWS Cost Explorer.
- **Derived from:** F-15.18.1 through F-15.18.10 (all server infrastructure features)
- **Rationale:** A sub-$5 monthly cost makes self-hosted infrastructure accessible to solo
  developers and small indie teams who cannot justify enterprise cloud spending, ensuring the
  engine's server features are not gated behind significant recurring costs.
- **Verification:** Deploy the full Free Tier stack; run simulated solo-developer workloads for 30
  days (10 Git pushes/day, 100 compilation jobs/month, 5 collaboration sessions); review AWS Cost
  Explorer and assert total charges are under $5.00 USD. Document per-stack cost breakdown.
