# R-15.18 -- Self-Hosted Server Infrastructure Requirements

## AWS CDK Deployment

### R-15.18.1 AWS CDK Deployment Stacks

The engine **SHALL** provide modular AWS CDK stacks
deployable via a single CLI command with Free Tier and
Enterprise profiles, IAM roles with least-privilege
policies, encryption at rest (KMS) and in transit (TLS),
and stack outputs including endpoint URLs and API keys.

- **Derived from:**
  [F-15.18.1](../../features/tools-editor/server-infrastructure.md)
- **Rationale:** Self-hosted AWS infrastructure must be
  automated, secure, and scalable from solo to enterprise.
- **Verification:** Integration test: deploy a full stack
  and verify completion within 15 minutes.

### R-15.18.2 Live Collaboration Server

The engine **SHALL** provide a self-hosted CRDT
collaboration server with PostgreSQL for persistent state,
S3 for binary assets, and WebSocket connections for
real-time sync.

- **Derived from:**
  [F-15.18.2](../../features/tools-editor/server-infrastructure.md)
- **Rationale:** Real-time editing must stay within the
  organization's infrastructure.
- **Verification:** Integration test: connect two editor
  instances and verify entity edits sync via the
  self-hosted server.

### R-15.18.3 Git and Git LFS Hosting

The engine **SHALL** provide a self-hosted Git server with
LFS support, GitHub-compatible REST API, S3-backed LFS
storage with lifecycle policies, and push/pull/lock
operations.

- **Derived from:**
  [F-15.18.3](../../features/tools-editor/server-infrastructure.md)
- **Rationale:** Source code and assets must be hostable on
  the organization's own infrastructure.
- **Verification:** Integration test: push, pull, and lock
  via the self-hosted server and verify correct behavior.

### R-15.18.4 Asset and Shader Compilation Server

The engine **SHALL** provide a self-hosted build farm with
auto-scaling instances based on queue depth, storing
compiled results in the shared build cache.

- **Derived from:**
  [F-15.18.4](../../features/tools-editor/server-infrastructure.md)
- **Rationale:** Build work offloaded from developer
  machines accelerates iteration.
- **Verification:** Integration test: submit a compilation
  job and verify the result appears in the shared cache.

### R-15.18.5 Signing and Distribution Server

The engine **SHALL** provide a self-hosted code signing
server with credentials stored in AWS Secrets Manager,
supporting automated signing for iOS, macOS, Android, and
Windows.

- **Derived from:**
  [F-15.18.5](../../features/tools-editor/server-infrastructure.md)
- **Rationale:** Signing credentials must never leave
  secure infrastructure.
- **Verification:** Integration test: sign an artifact
  and verify it passes platform verification tools.

### R-15.18.6 Continuous Deployment Pipeline

The engine **SHALL** provide a self-hosted CI/CD pipeline
triggering on Git push with stages for build, cook, test,
package, and deploy, where failed tests block the
deployment pipeline.

- **Derived from:**
  [F-15.18.6](../../features/tools-editor/server-infrastructure.md)
- **Rationale:** Automated build-test-deploy prevents
  regressions from reaching production.
- **Verification:** Integration test: introduce a failing
  test and verify the pipeline halts at the test stage.

### R-15.18.7 Test Runner Infrastructure

The engine **SHALL** provide self-hosted test runners for
unit, integration, and screenshot tests with golden image
comparison, test results stored in DynamoDB with CloudWatch
dashboards, and SNS notifications on failures.

- **Derived from:**
  [F-15.18.7](../../features/tools-editor/server-infrastructure.md)
- **Rationale:** Visual regression testing requires
  screenshot comparison infrastructure.
- **Verification:** Integration test: introduce a visual
  regression and verify the screenshot test fails and
  blocks the pipeline.

### R-15.18.8 Shared Cache and Database Services

The engine **SHALL** provide self-hosted S3, PostgreSQL,
DynamoDB, and Redis with data encryption at rest and in
transit and backup schedules configured in the CDK stack.

- **Derived from:**
  [F-15.18.8](../../features/tools-editor/server-infrastructure.md)
- **Rationale:** Centralized data services with encryption
  and backups are foundational for all server components.
- **Verification:** Unit test: verify KMS encryption is
  enabled on all storage resources.

### R-15.18.9 Backup and Disaster Recovery

The engine **SHALL** provide automated RDS backups with
configurable retention (7-35 days), S3 versioning with
lifecycle policies, DynamoDB point-in-time recovery,
cross-region replication for enterprise DR, and a
single-command restore CLI.

- **Derived from:**
  [F-15.18.9](../../features/tools-editor/server-infrastructure.md)
- **Rationale:** Data loss is unacceptable; disaster
  recovery must be automated and tested.
- **Verification:** Integration test: delete data, restore
  from RDS backup, and verify recovery.

### R-15.18.10 Enterprise Security Configuration

The engine **SHALL** configure VPC private subnets for
databases, public subnets for load balancers only, AWS WAF
blocking SQL injection/XSS with rate limiting, CloudTrail
audit logging, GuardDuty threat detection, and Secrets
Manager credential rotation.

- **Derived from:**
  [F-15.18.10](../../features/tools-editor/server-infrastructure.md)
- **Rationale:** Enterprise security posture requires
  defense-in-depth across all infrastructure layers.
- **Verification:** Unit test: send a SQL injection attempt
  and verify WAF blocks it.

---

## User Stories

## US-15.18.1 AWS CDK Deployment Stacks

### US-15.18.1.1
As a **DevOps (P-16)**, I want a single CLI command to deploy all server components so that
infrastructure provisioning is automated.

### US-15.18.1.2
As a **DevOps (P-16)**, I want modular stacks for independent component deployment so that I can
deploy only the services I need.

### US-15.18.1.3
As a **DevOps (P-16)**, I want Free Tier and Enterprise deployment profiles so that infrastructure
scales from solo developers to production studios.

### US-15.18.1.4
As a **DevOps (P-16)**, I want IAM roles with least-privilege policies per service so that
infrastructure follows security best practices.

### US-15.18.1.5
As a **DevOps (P-16)**, I want encryption at rest (KMS) and in transit (TLS) configured so that all
data is protected by default.

### US-15.18.1.6
As a **DevOps (P-16)**, I want stack outputs including endpoint URLs and API keys so that services
are ready to use after deployment.

### US-15.18.1.7
As an **engine tester (P-27)**, I want to verify full stack deployment completes within 15 minutes
so that deployment time is regression-tested.

---

## US-15.18.2 Live Collaboration Server

### US-15.18.2.1
As a **server admin (P-22)**, I want a self-hosted CRDT collaboration server so that real-time
editing stays within my infrastructure.

### US-15.18.2.2
As a **server admin (P-22)**, I want PostgreSQL for persistent document state so that collaboration
data survives server restarts.

### US-15.18.2.3
As a **server admin (P-22)**, I want S3 for large binary assets so that asset storage scales
independently from the server.

### US-15.18.2.4
As a **server admin (P-22)**, I want WebSocket connections for real-time sync so that edit
propagation is low-latency.

### US-15.18.2.5
As an **engine tester (P-27)**, I want to verify two editor instances sync entity edits via the
self-hosted server so that collaboration server functionality is regression-tested.

---

## US-15.18.3 Git and Git LFS Hosting with Locking

### US-15.18.3.1
As a **DevOps (P-16)**, I want a self-hosted Git server with LFS support so that source code and
assets are hosted on my infrastructure.

### US-15.18.3.2
As a **DevOps (P-16)**, I want GitHub-compatible REST API so that the editor's Git integration works
without modification.

### US-15.18.3.3
As a **DevOps (P-16)**, I want S3-backed LFS storage with lifecycle policies so that old LFS objects
are tiered to cheaper storage automatically.

### US-15.18.3.4
As an **engine tester (P-27)**, I want to verify push, pull, and LFS lock operations via the
self-hosted server so that Git hosting is regression-tested.

---

## US-15.18.4 Asset and Shader Compilation Server

### US-15.18.4.1
As a **DevOps (P-16)**, I want a self-hosted build farm for asset cooking and shader compilation so
that build work is offloaded from developer machines.

### US-15.18.4.2
As a **DevOps (P-16)**, I want auto-scaling EC2 instances based on queue depth so that the build
farm scales with demand.

### US-15.18.4.3
As a **DevOps (P-16)**, I want compiled results stored in the shared build cache so that other
developers benefit from build farm output.

### US-15.18.4.4
As an **engine tester (P-27)**, I want to verify a compilation job result appears in the shared
cache so that build farm integration is regression-tested.

---

## US-15.18.5 Signing and Distribution Server

### US-15.18.5.1
As a **DevOps (P-16)**, I want a self-hosted code signing server so that signing credentials never
leave secure infrastructure.

### US-15.18.5.2
As a **DevOps (P-16)**, I want signing credentials stored in AWS Secrets Manager so that keys are
never stored on disk.

### US-15.18.5.3
As a **DevOps (P-16)**, I want automated signing for iOS, macOS, Android, and Windows so that all
platforms are signed in a single pipeline.

### US-15.18.5.4
As an **engine tester (P-27)**, I want to verify signed artifacts pass platform verification tools
so that self-hosted signing is regression-tested.

---

## US-15.18.6 Continuous Deployment Pipeline

### US-15.18.6.1
As a **DevOps (P-16)**, I want a self-hosted CI/CD pipeline triggering on Git push so that every
commit is built and tested automatically.

### US-15.18.6.2
As a **DevOps (P-16)**, I want pipeline stages for build, cook, test, package, and deploy so that
the full release workflow is automated.

### US-15.18.6.3
As a **DevOps (P-16)**, I want failed tests to block the deployment pipeline so that regressions do
not reach production.

### US-15.18.6.4
As an **engine tester (P-27)**, I want to verify a failing test halts the pipeline at the test stage
so that pipeline gating is regression-tested.

---

## US-15.18.7 Test Runner Infrastructure

### US-15.18.7.1
As a **DevOps (P-16)**, I want self-hosted test runners for unit, integration, and screenshot tests
so that testing infrastructure is under my control.

### US-15.18.7.2
As a **DevOps (P-16)**, I want screenshot comparison against golden images so that visual
regressions are caught automatically.

### US-15.18.7.3
As a **DevOps (P-16)**, I want test results stored in DynamoDB with CloudWatch dashboards so that I
can track test trends over time.

### US-15.18.7.4
As a **DevOps (P-16)**, I want SNS notifications on test failures so that the team is alerted
immediately when tests fail.

### US-15.18.7.5
As an **engine tester (P-27)**, I want to verify a visual regression fails the screenshot test and
blocks the pipeline so that screenshot testing is regression-tested.

---

## US-15.18.8 Shared Cache and Database Services

### US-15.18.8.1
As a **server admin (P-22)**, I want self-hosted S3, PostgreSQL, DynamoDB, and Redis so that all
server components share centralized data services.

### US-15.18.8.2
As a **server admin (P-22)**, I want data encryption at rest and in transit enforced so that all
stored data is protected.

### US-15.18.8.3
As a **server admin (P-22)**, I want backup schedules configured in the CDK stack so that data
protection is automated.

### US-15.18.8.4
As an **engine tester (P-27)**, I want to verify KMS encryption is enabled on all storage resources
so that encryption enforcement is regression-tested.

---

## US-15.18.9 Backup and Disaster Recovery

### US-15.18.9.1
As a **server admin (P-22)**, I want automated RDS backups with configurable retention so that
database data is recoverable for 7-35 days.

### US-15.18.9.2
As a **server admin (P-22)**, I want S3 versioning with lifecycle policies so that previous asset
versions are retrievable.

### US-15.18.9.3
As a **server admin (P-22)**, I want DynamoDB point-in-time recovery so that table data can be
restored to any second in the past.

### US-15.18.9.4
As a **server admin (P-22)**, I want cross-region replication for enterprise DR so that data
survives a full region failure.

### US-15.18.9.5
As a **server admin (P-22)**, I want a restore CLI tool to recover from any backup point so that
disaster recovery is a single command.

### US-15.18.9.6
As an **engine tester (P-27)**, I want to verify restore from RDS backup recovers deleted data so
that backup restore is regression-tested.

---

## US-15.18.10 Enterprise Security Configuration

### US-15.18.10.1
As a **server admin (P-22)**, I want VPC private subnets for databases with public subnets for load
balancers only so that databases are isolated from the public internet.

### US-15.18.10.2
As a **server admin (P-22)**, I want AWS WAF blocking SQL injection, XSS, and rate limiting so that
common attacks are stopped at the load balancer.

### US-15.18.10.3
As a **server admin (P-22)**, I want CloudTrail audit logging for all API calls so that
infrastructure changes are traceable.

### US-15.18.10.4
As a **server admin (P-22)**, I want GuardDuty threat detection active so that suspicious activity
is flagged automatically.

### US-15.18.10.5
As a **server admin (P-22)**, I want Secrets Manager rotation for credentials so that database
passwords and API keys rotate automatically.

### US-15.18.10.6
As an **engine tester (P-27)**, I want to verify WAF blocks SQL injection attempts so that WAF
enforcement is regression-tested.
