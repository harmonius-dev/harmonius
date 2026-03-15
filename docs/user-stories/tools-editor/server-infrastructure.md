# User Stories: Self-Hosted Server Infrastructure

## F-15.18.1 AWS CDK Deployment Stacks

### US-15.18.1a Single-Command Infrastructure Provisioning
**As a** solo developer (P-15), **I want** to deploy all Harmonius server components to my own AWS
account with a single `cdk deploy --profile free-tier` command, **so that** I get a fully configured
collaboration server, Git hosting, and build cache without manually provisioning AWS resources or
writing CloudFormation templates.

### US-15.18.1b Modular Stack Selection
**As a** DevOps engineer (P-16), **I want** to deploy only the CDK stacks my team needs (e.g., Git
server and build farm but not the collaboration server), **so that** I avoid paying for unused
infrastructure and keep the deployment footprint minimal.

### US-15.18.1c Version-Pinned Infrastructure
**As a** server admin (P-22), **I want** CDK stacks to be version-pinned to the engine release,
**so that** infrastructure upgrades are coordinated with engine updates and I never run incompatible
server versions.

### US-15.18.1d Infrastructure Cost Visibility
**As an** executive (P-1), **I want** the Free Tier profile to stay within AWS Free Tier limits so
my solo developers and prototyping teams incur near-zero cloud costs, **so that** infrastructure
expenses do not gate access to the engine's server features.

## F-15.18.2 Live Collaboration Server

### US-15.18.2a Self-Hosted Collaboration Data Residency
**As a** server admin (P-22), **I want** the collaboration server deployed in my own AWS account
with all data stored in my RDS and S3 buckets, **so that** project data never leaves my
organization's infrastructure and I satisfy data residency requirements.

### US-15.18.2b Real-Time Multi-User Editing Backend
**As a** solo developer (P-15), **I want** to deploy a collaboration server that enables real-time
CRDT-based editing sessions with teammates over WebSocket, **so that** my small team can
simultaneously edit the same level without file locking or manual merge conflicts.

### US-15.18.2c Collaboration Server Scaling
**As a** DevOps engineer (P-16), **I want** the enterprise collaboration server to auto-scale
Fargate tasks based on active session count, **so that** the server handles peak usage during
crunch periods without manual capacity planning.

## F-15.18.3 Git and Git LFS Hosting with Locking

### US-15.18.3a Free Self-Hosted Git with LFS
**As a** solo developer (P-15), **I want** to deploy a self-hosted Git server with LFS support
backed by S3 storage, **so that** I can host my game project's source and large binary assets
without paying per-seat fees to GitHub or GitLab.

### US-15.18.3b GitHub-Compatible API for Editor Integration
**As a** DevOps engineer (P-16), **I want** the self-hosted Git server to expose a
GitHub-compatible REST API, **so that** the editor's built-in Git integration panel works without
any configuration changes when switching from GitHub to self-hosted.

### US-15.18.3c LFS Lifecycle Cost Management
**As a** server admin (P-22), **I want** LFS objects to be automatically moved to Glacier after a
configurable retention period, **so that** storage costs for old binary assets decrease over time
without manual archival.

### US-15.18.3d Eliminating Per-Seat Git Licensing
**As an** executive (P-1), **I want** the team to use a self-hosted Git server instead of paid
hosted services, **so that** version control costs scale with infrastructure usage rather than
headcount.

## F-15.18.4 Asset and Shader Compilation Server

### US-15.18.4a Offloading Shader Compilation
**As a** solo developer (P-15), **I want** to submit shader compilation jobs to a remote build
farm instead of compiling locally, **so that** my laptop stays responsive while shaders compile on
dedicated cloud instances.

### US-15.18.4b Auto-Scaling Build Farm for Crunch Periods
**As a** DevOps engineer (P-16), **I want** the build farm to auto-scale EC2 instances based on
SQS queue depth, **so that** compilation throughput increases automatically when the team submits
a burst of build jobs during content freezes.

### US-15.18.4c Shared Build Cache Population
**As a** server admin (P-22), **I want** compiled assets and shaders to be stored in a shared S3
build cache so that subsequent requests for the same content-hash return cached results, **so
that** each unique asset is compiled at most once across the entire team.

### US-15.18.4d GPU Spot Instance Cost Optimization
**As an** executive (P-1), **I want** the enterprise build farm to use GPU spot instances for
shader compilation, **so that** we get GPU compilation performance at a fraction of on-demand
pricing.

## F-15.18.5 Signing and Distribution Server

### US-15.18.5a Secure Credential Storage
**As a** server admin (P-22), **I want** signing credentials stored in AWS Secrets Manager with
automatic rotation rather than on disk or in version control, **so that** credentials are never
exposed in build logs, repositories, or developer workstations.

### US-15.18.5b Automated Multi-Platform Signing
**As a** DevOps engineer (P-16), **I want** the signing server to automatically code-sign builds
for iOS, macOS, Android, and Windows as part of the packaging pipeline, **so that** I do not need
to manually sign each platform's artifacts or maintain signing tools on developer machines.

### US-15.18.5c Solo Developer Release Packaging
**As a** solo developer (P-15), **I want** to trigger a single command that signs and packages my
game for all target platforms, **so that** I can publish to multiple stores without learning each
platform's signing toolchain.

### US-15.18.5d Auditable Signing Operations
**As an** executive (P-1), **I want** all signing operations logged in CloudTrail with
non-repudiation, **so that** we have an audit trail proving which builds were signed, when, and
by which pipeline run.

## F-15.18.6 Continuous Deployment Pipeline

### US-15.18.6a Push-to-Deploy Workflow
**As a** DevOps engineer (P-16), **I want** a CI/CD pipeline that triggers automatically on every
Git push, building the game, cooking assets, running tests, packaging, and deploying to
staging, **so that** every commit is validated end-to-end without manual intervention.

### US-15.18.6b Solo Developer CI on Free Tier
**As a** solo developer (P-15), **I want** a single CodeBuild project on the free tier that
builds, tests, and packages my game on every push, **so that** I get continuous integration
without configuring Jenkins or paying for a hosted CI service.

### US-15.18.6c Pipeline Stage Configuration
**As a** server admin (P-22), **I want** to enable or disable pipeline stages (e.g., skip GPU
tests, skip store submission) via CDK configuration, **so that** I can tailor the pipeline to the
project's current phase without forking the CDK code.

### US-15.18.6d Release Confidence for Stakeholders
**As an** executive (P-1), **I want** every release candidate to pass through an automated
build-test-package-deploy pipeline before reaching players, **so that** I have confidence that
shipped builds meet quality standards.

## F-15.18.7 Test Runner Infrastructure

### US-15.18.7a Screenshot Regression Detection
**As a** DevOps engineer (P-16), **I want** the test runner to compare rendered screenshots against
golden images and block the pipeline on visual regressions, **so that** unintended rendering
changes are caught before they reach players.

### US-15.18.7b Performance Trend Dashboards
**As a** server admin (P-22), **I want** test results stored in DynamoDB with CloudWatch dashboards
showing frame time, draw call, and memory trends over time, **so that** I can detect gradual
performance degradation before it becomes a player-facing issue.

### US-15.18.7c Test Failure Notifications
**As a** solo developer (P-15), **I want** to receive an SNS notification (email or Slack webhook)
when a test fails in the pipeline, **so that** I can fix the issue promptly without polling the
CI dashboard.

### US-15.18.7d Quality Gate for Releases
**As an** executive (P-1), **I want** failed tests to block the deployment pipeline from advancing
to the packaging stage, **so that** no build ships to players unless it passes all automated
quality checks.

## F-15.18.8 Shared Cache and Database Services

### US-15.18.8a Centralized Data Layer
**As a** DevOps engineer (P-16), **I want** all server components to share a single set of
database and cache services (RDS, DynamoDB, ElastiCache, S3) configured by one CDK stack, **so
that** I manage data infrastructure in one place instead of per-component.

### US-15.18.8b Fast Asset Cache Hits
**As a** solo developer (P-15), **I want** the shared S3 cache to return compiled assets and
shaders by content hash so that my editor fetches cached results on startup, **so that** I skip
local compilation and start working in minutes.

### US-15.18.8c Encrypted Data at Rest and in Transit
**As a** server admin (P-22), **I want** KMS encryption on all database and cache storage and TLS
on all connections, **so that** project data is protected against unauthorized access even if the
underlying storage is compromised.

### US-15.18.8d Intelligent Storage Tiering
**As an** executive (P-1), **I want** S3 Intelligent-Tiering to automatically move infrequently
accessed cached assets to cheaper storage classes, **so that** storage costs scale with actual
usage rather than total data volume.

## F-15.18.9 Backup and Disaster Recovery

### US-15.18.9a Automated Daily Backups
**As a** server admin (P-22), **I want** RDS automated backups, S3 versioning, and DynamoDB
point-in-time recovery enabled by default, **so that** I can recover from accidental data
deletion or corruption without manual backup scripts.

### US-15.18.9b One-Command Disaster Recovery
**As a** DevOps engineer (P-16), **I want** a restore CLI tool that recovers all server data from
any backup point to a new or existing stack, **so that** I can restore full service after a
catastrophic failure without manually importing database dumps.

### US-15.18.9c Cross-Region Replication for Business Continuity
**As an** executive (P-1), **I want** enterprise deployments to replicate data across AWS regions,
**so that** a regional outage does not cause permanent data loss or extended downtime for the
production studio.

### US-15.18.9d Backup Status Monitoring
**As a** solo developer (P-15), **I want** CloudWatch alarms to notify me if a backup fails, **so
that** I am aware of backup issues before I need to recover from one.

## F-15.18.10 Enterprise Security Configuration

### US-15.18.10a Network Isolation for Databases
**As a** server admin (P-22), **I want** databases and internal services in private subnets
accessible only via the VPC, with public subnets reserved for load balancers, **so that**
sensitive data is never directly exposed to the internet.

### US-15.18.10b WAF Protection Against Common Attacks
**As a** DevOps engineer (P-16), **I want** AWS WAF rules on all Application Load Balancers to
block SQL injection, XSS, and excessive request rates, **so that** self-hosted services are
protected against common web attacks without configuring a separate firewall.

### US-15.18.10c Credential Rotation
**As a** server admin (P-22), **I want** Secrets Manager to automatically rotate database
credentials and API keys on a configurable schedule, **so that** compromised credentials expire
before they can be exploited.

### US-15.18.10d Compliance Audit Trail
**As an** executive (P-1), **I want** CloudTrail logging for all API calls and GuardDuty threat
detection enabled in the enterprise profile, **so that** the studio has auditable security
controls for compliance with publisher and platform requirements.
