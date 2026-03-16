# User Stories: Self-Hosted Server Infrastructure

## F-15.18.1 AWS CDK Deployment Stacks

## US-15.18.1.1 Developer Deploys with Single Command

**As a** developer (P-15), **I want** to deploy all Harmonius server components to my own AWS
account with a single `cdk deploy --profile free-tier` command, **so that** I get collaboration
server, Git hosting, and build cache without manually provisioning resources.

## US-15.18.1.2 DevOps Selects Modular Stacks

**As a** DevOps engineer (P-16), **I want** to deploy only the CDK stacks my team needs (e.g., Git
server and build farm but not collaboration), **so that** I avoid paying for unused infrastructure.

## US-15.18.1.3 Server Admin Pins Stack Versions

**As a** server admin (P-22), **I want** CDK stacks version-pinned to the engine release,
**so that** infrastructure upgrades are coordinated with engine updates.

## US-15.18.1.4 Engine Tester Validates Free Tier Limits

**As an** engine tester (P-27), **I want** to verify that the Free Tier profile stays within AWS
Free Tier limits (750h t3.micro, 5GB S3, 20GB RDS), **so that** solo developers incur near-zero
cloud costs.

## F-15.18.2 Live Collaboration Server

## US-15.18.2.1 Server Admin Keeps Data On-Premise

**As a** server admin (P-22), **I want** the collaboration server deployed in my own AWS account
with all data in my RDS and S3 buckets, **so that** project data satisfies data residency
requirements.

## US-15.18.2.2 Developer Enables Real-Time Editing

**As a** developer (P-15), **I want** a collaboration server enabling CRDT-based editing sessions
over WebSocket, **so that** my team edits simultaneously without file locking.

## US-15.18.2.3 DevOps Configures Auto-Scaling

**As a** DevOps engineer (P-16), **I want** the enterprise server to auto-scale Fargate tasks based
on active session count, **so that** the server handles peak usage without manual capacity planning.

## US-15.18.2.4 Engine Tester Validates Session Handling

**As an** engine tester (P-27), **I want** to verify that the collaboration server correctly handles
multi-user editing sessions, presence tracking, access control, and chat relay, **so that** all
collaboration features work on self-hosted infrastructure.

## F-15.18.3 Git and Git LFS Hosting with Locking

## US-15.18.3.1 Developer Hosts Git Locally

**As a** developer (P-15), **I want** a self-hosted Git server with LFS support backed by S3
storage, **so that** I host source and binary assets without per-seat hosting fees.

## US-15.18.3.2 DevOps Ensures API Compatibility

**As a** DevOps engineer (P-16), **I want** the self-hosted Git server to expose a GitHub-compatible
REST API, **so that** the editor's Git integration works without configuration changes.

## US-15.18.3.3 Server Admin Archives Old LFS Objects

**As a** server admin (P-22), **I want** LFS objects automatically moved to Glacier after a
configurable retention period, **so that** storage costs for old assets decrease over time.

## US-15.18.3.4 Engine Tester Validates Lock/Unlock

**As an** engine tester (P-27), **I want** to verify that LFS lock and unlock operations work
correctly through the self-hosted server, **so that** file locking prevents concurrent binary edits.

## F-15.18.4 Asset and Shader Compilation Server

## US-15.18.4.1 Developer Offloads Compilation

**As a** developer (P-15), **I want** to submit shader compilation and asset cooking jobs to a
remote build farm, **so that** my laptop stays responsive while builds run on dedicated cloud
instances.

## US-15.18.4.2 DevOps Scales Build Farm

**As a** DevOps engineer (P-16), **I want** the build farm to auto-scale EC2 instances based on SQS
queue depth, **so that** compilation throughput increases automatically during content freezes.

## US-15.18.4.3 Server Admin Populates Shared Cache

**As a** server admin (P-22), **I want** compiled results stored in the S3-backed shared build
cache, **so that** each asset is compiled at most once across the team.

## US-15.18.4.4 Engine Tester Validates GPU Compilation

**As an** engine tester (P-27), **I want** to verify that GPU-enabled instances (g5) compile shaders
correctly for all target platforms, **so that** remote compilation matches local output.

## F-15.18.5 Signing and Distribution Server

## US-15.18.5.1 Server Admin Secures Credentials

**As a** server admin (P-22), **I want** signing credentials stored in AWS Secrets Manager with
automatic rotation, **so that** credentials are never on disk or in version control.

## US-15.18.5.2 DevOps Automates Multi-Platform Signing

**As a** DevOps engineer (P-16), **I want** the signing server to sign builds for iOS, macOS,
Android, and Windows as part of the packaging pipeline, **so that** manual signing is eliminated.

## US-15.18.5.3 Developer Triggers One-Command Release

**As a** developer (P-15), **I want** a single command that signs and packages my game for all
target platforms, **so that** I publish to multiple stores without learning each signing toolchain.

## US-15.18.5.4 Engine Tester Validates Audit Trail

**As an** engine tester (P-27), **I want** to verify that all signing operations are logged in
CloudTrail with non-repudiation, **so that** the studio has a complete audit trail.

## F-15.18.6 Continuous Deployment Pipeline

## US-15.18.6.1 DevOps Gets Push-to-Deploy

**As a** DevOps engineer (P-16), **I want** a CI/CD pipeline triggering on every Git push that
builds, cooks assets, runs tests, packages, and deploys to staging, **so that** every commit is
validated without manual intervention.

## US-15.18.6.2 Developer Uses Free Tier CI

**As a** developer (P-15), **I want** a single CodeBuild project on the free tier that builds,
tests, and packages on every push, **so that** I get CI without configuring Jenkins or paying for
hosted CI.

## US-15.18.6.3 Server Admin Configures Pipeline Stages

**As a** server admin (P-22), **I want** to enable or disable pipeline stages (skip GPU tests, skip
store submission) via CDK configuration, **so that** the pipeline adapts to the project's current
phase.

## US-15.18.6.4 Engine Tester Validates End-to-End Pipeline

**As an** engine tester (P-27), **I want** to verify that the full pipeline (source checkout, build,
asset cook, test, package, deploy) completes successfully, **so that** automated releases are
reliable.

## F-15.18.7 Test Runner Infrastructure

## US-15.18.7.1 DevOps Detects Visual Regressions

**As a** DevOps engineer (P-16), **I want** screenshot comparison tests comparing rendered scenes
against golden images, **so that** unintended rendering changes are caught before reaching players.

## US-15.18.7.2 Server Admin Tracks Performance Trends

**As a** server admin (P-22), **I want** test results stored in DynamoDB with CloudWatch dashboards
showing frame time, draw call, and memory trends, **so that** gradual performance degradation is
detected early.

## US-15.18.7.3 Developer Gets Failure Notifications

**As a** developer (P-15), **I want** SNS notifications (email or Slack webhook) when tests fail,
**so that** I can fix issues without polling the CI dashboard.

## US-15.18.7.4 Engine Tester Validates Quality Gates

**As an** engine tester (P-27), **I want** to verify that failed tests block the deployment pipeline
from advancing to packaging, **so that** no build ships without passing all quality checks.

## F-15.18.8 Shared Cache and Database Services

## US-15.18.8.1 DevOps Centralizes Data Layer

**As a** DevOps engineer (P-16), **I want** all server components to share a single set of database
and cache services (RDS, DynamoDB, ElastiCache, S3) configured by one CDK stack, **so that** data
infrastructure is managed in one place.

## US-15.18.8.2 Developer Gets Fast Cache Hits

**As a** developer (P-15), **I want** the shared S3 cache to return compiled assets by content hash
on editor startup, **so that** I skip local compilation and start working in minutes.

## US-15.18.8.3 Server Admin Enforces Encryption

**As a** server admin (P-22), **I want** KMS encryption on all storage and TLS on all connections,
**so that** project data is protected at rest and in transit.

## US-15.18.8.4 Engine Tester Validates Intelligent Tiering

**As an** engine tester (P-27), **I want** to verify that S3 Intelligent-Tiering moves infrequently
accessed assets to cheaper storage automatically, **so that** costs scale with actual usage.

## F-15.18.9 Backup and Disaster Recovery

## US-15.18.9.1 Server Admin Gets Automated Backups

**As a** server admin (P-22), **I want** RDS automated backups, S3 versioning, and DynamoDB
point-in-time recovery enabled by default, **so that** data is recoverable without manual backup
scripts.

## US-15.18.9.2 DevOps Restores with One Command

**As a** DevOps engineer (P-16), **I want** a restore CLI tool that recovers all server data from
any backup point, **so that** full service is restored after catastrophic failure without manual
database imports.

## US-15.18.9.3 Developer Gets Backup Alerts

**As a** developer (P-15), **I want** CloudWatch alarms notifying me if a backup fails, **so that**
I am aware of backup issues before I need to recover.

## US-15.18.9.4 Engine Tester Validates Cross-Region Replication

**As an** engine tester (P-27), **I want** to verify that enterprise cross-region replication works
correctly, **so that** a regional outage does not cause permanent data loss.

## F-15.18.10 Enterprise Security Configuration

## US-15.18.10.1 Server Admin Isolates Networks

**As a** server admin (P-22), **I want** databases in private subnets accessible only via VPC with
public subnets reserved for load balancers, **so that** sensitive data is never directly exposed to
the internet.

## US-15.18.10.2 DevOps Enables WAF Protection

**As a** DevOps engineer (P-16), **I want** AWS WAF rules on all Application Load Balancers blocking
SQL injection, XSS, and excessive request rates, **so that** services are protected without a
separate firewall.

## US-15.18.10.3 Server Admin Rotates Credentials

**As a** server admin (P-22), **I want** Secrets Manager automatic rotation for database credentials
and API keys, **so that** compromised credentials expire before exploitation.

## US-15.18.10.4 Engine Tester Validates Security Controls

**As an** engine tester (P-27), **I want** to verify that CloudTrail logging and GuardDuty threat
detection are enabled in the enterprise profile, **so that** security controls are auditable.
