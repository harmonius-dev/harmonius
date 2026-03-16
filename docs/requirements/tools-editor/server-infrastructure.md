# R-15.18 -- Self-Hosted Server Infrastructure Requirements

## AWS CDK Deployment

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.18.1 | The engine **SHALL** provide modular AWS CDK stacks deployable via a single CLI command with Free Tier and Enterprise profiles, IAM roles with least-privilege policies, encryption at rest (KMS) and in transit (TLS), and stack outputs including endpoint URLs and API keys. [F-15.18.1](../../features/tools-editor/server-infrastructure.md) enterprise. |  | Self-hosted AWS infrastructure must be automated, secure, and scalable from solo to | Integration test: deploy a full stack and verify completion within 15 minutes. |
| R-15.18.2 | The engine **SHALL** provide a self-hosted CRDT collaboration server with PostgreSQL for persistent state, S3 for binary assets, and WebSocket connections for real-time sync. [F-15.18.2](../../features/tools-editor/server-infrastructure.md) the self-hosted server. |  | Real-time editing must stay within the organization's infrastructure. | Integration test: connect two editor instances and verify entity edits sync via |
| R-15.18.3 | The engine **SHALL** provide a self-hosted Git server with LFS support, GitHub-compatible REST API, S3-backed LFS storage with lifecycle policies, and push/pull/lock operations. [F-15.18.3](../../features/tools-editor/server-infrastructure.md) correct behavior. |  | Source code and assets must be hostable on the organization's own infrastructure. | Integration test: push, pull, and lock via the self-hosted server and verify |
| R-15.18.4 | The engine **SHALL** provide a self-hosted build farm with auto-scaling instances based on queue depth, storing compiled results in the shared build cache. [F-15.18.4](../../features/tools-editor/server-infrastructure.md) shared cache. |  | Build work offloaded from developer machines accelerates iteration. | Integration test: submit a compilation job and verify the result appears in the |
| R-15.18.5 | The engine **SHALL** provide a self-hosted code signing server with credentials stored in AWS Secrets Manager, supporting automated signing for iOS, macOS, Android, and Windows. [F-15.18.5](../../features/tools-editor/server-infrastructure.md) tools. |  | Signing credentials must never leave secure infrastructure. | Integration test: sign an artifact and verify it passes platform verification |
| R-15.18.6 | The engine **SHALL** provide a self-hosted CI/CD pipeline triggering on Git push with stages for build, cook, test, package, and deploy, where failed tests block the deployment pipeline. [F-15.18.6](../../features/tools-editor/server-infrastructure.md) test stage. |  | Automated build-test-deploy prevents regressions from reaching production. | Integration test: introduce a failing test and verify the pipeline halts at the |
| R-15.18.7 | The engine **SHALL** provide self-hosted test runners for unit, integration, and screenshot tests with golden image comparison, test results stored in DynamoDB with CloudWatch dashboards, and SNS notifications on failures. [F-15.18.7](../../features/tools-editor/server-infrastructure.md) fails and blocks the pipeline. |  | Visual regression testing requires screenshot comparison infrastructure. | Integration test: introduce a visual regression and verify the screenshot test |
| R-15.18.8 | The engine **SHALL** provide self-hosted S3, PostgreSQL, DynamoDB, and Redis with data encryption at rest and in transit and backup schedules configured in the CDK stack. [F-15.18.8](../../features/tools-editor/server-infrastructure.md) server components. |  | Centralized data services with encryption and backups are foundational for all | Unit test: verify KMS encryption is enabled on all storage resources. |
| R-15.18.9 | The engine **SHALL** provide automated RDS backups with configurable retention (7-35 days), S3 versioning with lifecycle policies, DynamoDB point-in-time recovery, cross-region replication for enterprise DR, and a single-command restore CLI. [F-15.18.9](../../features/tools-editor/server-infrastructure.md) |  | Data loss is unacceptable; disaster recovery must be automated and tested. | Integration test: delete data, restore from RDS backup, and verify recovery. |
| R-15.18.10 | The engine **SHALL** configure VPC private subnets for databases, public subnets for load balancers only, AWS WAF blocking SQL injection/XSS with rate limiting, CloudTrail audit logging, GuardDuty threat detection, and Secrets Manager credential rotation. [F-15.18.10](../../features/tools-editor/server-infrastructure.md) layers. |  | Enterprise security posture requires defense-in-depth across all infrastructure | Unit test: send a SQL injection attempt and verify WAF blocks it. |

## Open-Source Service Deployment

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.18.11 | The engine **SHALL** provide CDK stacks that deploy all services using exclusively open-source software (PostgreSQL, Redis/Valkey, Keycloak, NATS, Forgejo, Grafana, Prometheus, OpenSearch) on AWS compute, with no proprietary AWS service dependencies beyond S3, EC2, ECS, and RDS for PostgreSQL. [F-15.18.11](../../features/tools-editor/server-infrastructure.md) | F-15.18.11 | Open-source dependencies ensure portability to any cloud or on-premises deployment. | Unit test: verify no CDK constructs reference Aurora, DynamoDB, Cognito, SQS, SNS, or CloudWatch dashboards. |
| R-15.18.12 | The engine **SHALL** provide a free AWS Marketplace listing that launches a guided wizard collecting scaling profile, region, domain, and optional services, deploying all CDK stacks with zero Marketplace fees. [F-15.18.12](../../features/tools-editor/server-infrastructure.md) | F-15.18.12 | 1-click deployment lowers the barrier for non-DevOps users. | Integration test: launch the Marketplace product and verify all services are healthy within 20 minutes. |
| R-15.18.13 | The engine **SHALL** provide a web-based admin dashboard (Grafana) displaying service health, resource utilization, active users, queue depth, session count, and cost estimates, with Keycloak SSO authentication. [F-15.18.13](../../features/tools-editor/server-infrastructure.md) | F-15.18.13 | Operators need a single pane of glass for all services. | Integration test: deploy the dashboard and verify all service panels display live metrics. |
| R-15.18.14 | The engine **SHALL** provide four scaling profiles (Solo ~$20/mo, Team ~$100/mo, Studio ~$500/mo, Production ~$2000+/mo) as CDK context parameters controlling instance sizes, replica counts, and storage allocations. [F-15.18.14](../../features/tools-editor/server-infrastructure.md) | F-15.18.14 | Predictable cost tiers let studios choose infrastructure that fits their budget. | Unit test: verify each profile produces correctly sized instances in the synthesized CloudFormation template. |
| R-15.18.15 | The engine **SHALL** provide a self-hosted build cache service with S3-compatible storage, Redis/Valkey lookup index, REST API for put/get/query, LRU eviction, and Keycloak JWT authentication. [F-15.18.15](../../features/tools-editor/server-infrastructure.md) | F-15.18.15 | Content-addressable build cache eliminates redundant compilation across the team. | Integration test: store an artifact by content hash and retrieve it from a different client. |
| R-15.18.16 | The engine **SHALL** provide a self-hosted collaboration service using PostgreSQL, Redis/Valkey, NATS, and S3-compatible storage with Keycloak RBAC, implementing the same protocol as the managed collaboration service. [F-15.18.16](../../features/tools-editor/server-infrastructure.md) | F-15.18.16 | Self-hosted collaboration must be functionally identical to managed hosting. | Integration test: connect two editors and verify CRDT sync, presence, and access control. |
| R-15.18.17 | The engine **SHALL** provide a self-hosted matchmaking service using PostgreSQL, Redis/Valkey, and NATS with ELO/Glicko-2 skill rating, region-based matching, party queuing, and Keycloak authentication. [F-15.18.17](../../features/tools-editor/server-infrastructure.md) | F-15.18.17 | Multiplayer games require matchmaking infrastructure that studios control. | Integration test: submit 20 players and verify matches are created within 30 seconds with balanced skill ratings. |
| R-15.18.18 | The engine **SHALL** provide a self-hosted asset store service using PostgreSQL, OpenSearch, S3-compatible storage, and Redis/Valkey with upload validation, version management, dependency resolution, and Keycloak authentication. [F-15.18.18](../../features/tools-editor/server-infrastructure.md) | F-15.18.18 | A self-hosted asset store enables private asset sharing within studios. | Integration test: upload an asset, search by tag, and download with dependency resolution. |
| R-15.18.19 | The engine **SHALL** provide a health monitoring stack using Prometheus, Grafana, and Loki with pre-configured dashboards for all services, alert rules firing to NATS topics, and CloudWatch exporter for AWS infrastructure metrics. [F-15.18.19](../../features/tools-editor/server-infrastructure.md) | F-15.18.19 | Open-source monitoring avoids CloudWatch vendor lock-in and enables custom dashboards. | Integration test: trigger a simulated failure and verify the alert fires within 60 seconds. |
| R-15.18.20 | The engine **SHALL** support multi-region CDK deployment with read replicas (PostgreSQL, Redis), S3 cross-region replication, NATS super-cluster, and DNS-based failover via Route 53 health checks. [F-15.18.20](../../features/tools-editor/server-infrastructure.md) | F-15.18.20 | Global studios need low-latency access and disaster recovery across regions. | Integration test: deploy to 2 regions and verify data replication and DNS failover. |

---

## US-15.18.1 AWS CDK Deployment Stacks

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.18.1.1 | DevOps | I want a single CLI command to deploy all server components so that infrastructure provisioning is automated. |  |  |
| US-15.18.1.2 | DevOps | I want modular stacks for independent component deployment so that I can deploy only the services I need. |  |  |
| US-15.18.1.3 | DevOps | I want Free Tier and Enterprise deployment profiles so that infrastructure scales from solo developers to production studios. |  |  |
| US-15.18.1.4 | DevOps | I want IAM roles with least-privilege policies per service so that infrastructure follows security best practices. |  |  |
| US-15.18.1.5 | DevOps | I want encryption at rest (KMS) and in transit (TLS) configured so that all data is protected by default. |  |  |
| US-15.18.1.6 | DevOps | I want stack outputs including endpoint URLs and API keys so that services are ready to use after deployment. |  |  |
| US-15.18.1.7 | engine tester | I want to verify full stack deployment completes within 15 minutes so that deployment time is regression-tested. |  |  |

## US-15.18.2 Live Collaboration Server

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.18.2.1 | server admin | I want a self-hosted CRDT collaboration server so that real-time editing stays within my infrastructure. |  |  |
| US-15.18.2.2 | server admin | I want PostgreSQL for persistent document state so that collaboration data survives server restarts. |  |  |
| US-15.18.2.3 | server admin | I want S3 for large binary assets so that asset storage scales independently from the server. |  |  |
| US-15.18.2.4 | server admin | I want WebSocket connections for real-time sync so that edit propagation is low-latency. |  |  |
| US-15.18.2.5 | engine tester | I want to verify two editor instances sync entity edits via the self-hosted server so that collaboration server functionality is regression-tested. |  |  |

## US-15.18.3 Git and Git LFS Hosting with Locking

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.18.3.1 | DevOps | I want a self-hosted Git server with LFS support so that source code and assets are hosted on my infrastructure. |  |  |
| US-15.18.3.2 | DevOps | I want GitHub-compatible REST API so that the editor's Git integration works without modification. |  |  |
| US-15.18.3.3 | DevOps | I want S3-backed LFS storage with lifecycle policies so that old LFS objects are tiered to cheaper storage automatically. |  |  |
| US-15.18.3.4 | engine tester | I want to verify push, pull, and LFS lock operations via the self-hosted server so that Git hosting is regression-tested. |  |  |

## US-15.18.4 Asset and Shader Compilation Server

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.18.4.1 | DevOps | I want a self-hosted build farm for asset cooking and shader compilation so that build work is offloaded from developer machines. |  |  |
| US-15.18.4.2 | DevOps | I want auto-scaling EC2 instances based on queue depth so that the build farm scales with demand. |  |  |
| US-15.18.4.3 | DevOps | I want compiled results stored in the shared build cache so that other developers benefit from build farm output. |  |  |
| US-15.18.4.4 | engine tester | I want to verify a compilation job result appears in the shared cache so that build farm integration is regression-tested. |  |  |

## US-15.18.5 Signing and Distribution Server

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.18.5.1 | DevOps | I want a self-hosted code signing server so that signing credentials never leave secure infrastructure. |  |  |
| US-15.18.5.2 | DevOps | I want signing credentials stored in AWS Secrets Manager so that keys are never stored on disk. |  |  |
| US-15.18.5.3 | DevOps | I want automated signing for iOS, macOS, Android, and Windows so that all platforms are signed in a single pipeline. |  |  |
| US-15.18.5.4 | engine tester | I want to verify signed artifacts pass platform verification tools so that self-hosted signing is regression-tested. |  |  |

## US-15.18.6 Continuous Deployment Pipeline

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.18.6.1 | DevOps | I want a self-hosted CI/CD pipeline triggering on Git push so that every commit is built and tested automatically. |  |  |
| US-15.18.6.2 | DevOps | I want pipeline stages for build, cook, test, package, and deploy so that the full release workflow is automated. |  |  |
| US-15.18.6.3 | DevOps | I want failed tests to block the deployment pipeline so that regressions do not reach production. |  |  |
| US-15.18.6.4 | engine tester | I want to verify a failing test halts the pipeline at the test stage so that pipeline gating is regression-tested. |  |  |

## US-15.18.7 Test Runner Infrastructure

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.18.7.1 | DevOps | I want self-hosted test runners for unit, integration, and screenshot tests so that testing infrastructure is under my control. |  |  |
| US-15.18.7.2 | DevOps | I want screenshot comparison against golden images so that visual regressions are caught automatically. |  |  |
| US-15.18.7.3 | DevOps | I want test results stored in DynamoDB with CloudWatch dashboards so that I can track test trends over time. |  |  |
| US-15.18.7.4 | DevOps | I want SNS notifications on test failures so that the team is alerted immediately when tests fail. |  |  |
| US-15.18.7.5 | engine tester | I want to verify a visual regression fails the screenshot test and blocks the pipeline so that screenshot testing is regression-tested. |  |  |

## US-15.18.8 Shared Cache and Database Services

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.18.8.1 | server admin | I want self-hosted S3, PostgreSQL, DynamoDB, and Redis so that all server components share centralized data services. |  |  |
| US-15.18.8.2 | server admin | I want data encryption at rest and in transit enforced so that all stored data is protected. |  |  |
| US-15.18.8.3 | server admin | I want backup schedules configured in the CDK stack so that data protection is automated. |  |  |
| US-15.18.8.4 | engine tester | I want to verify KMS encryption is enabled on all storage resources so that encryption enforcement is regression-tested. |  |  |

## US-15.18.9 Backup and Disaster Recovery

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.18.9.1 | server admin | I want automated RDS backups with configurable retention so that database data is recoverable for 7-35 days. |  |  |
| US-15.18.9.2 | server admin | I want S3 versioning with lifecycle policies so that previous asset versions are retrievable. |  |  |
| US-15.18.9.3 | server admin | I want DynamoDB point-in-time recovery so that table data can be restored to any second in the past. |  |  |
| US-15.18.9.4 | server admin | I want cross-region replication for enterprise DR so that data survives a full region failure. |  |  |
| US-15.18.9.5 | server admin | I want a restore CLI tool to recover from any backup point so that disaster recovery is a single command. |  |  |
| US-15.18.9.6 | engine tester | I want to verify restore from RDS backup recovers deleted data so that backup restore is regression-tested. |  |  |

## US-15.18.10 Enterprise Security Configuration

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.18.10.1 | server admin | I want VPC private subnets for databases with public subnets for load balancers only so that databases are isolated from the public internet. |  |  |
| US-15.18.10.2 | server admin | I want AWS WAF blocking SQL injection, XSS, and rate limiting so that common attacks are stopped at the load balancer. |  |  |
| US-15.18.10.3 | server admin | I want CloudTrail audit logging for all API calls so that infrastructure changes are traceable. |  |  |
| US-15.18.10.4 | server admin | I want GuardDuty threat detection active so that suspicious activity is flagged automatically. |  |  |
| US-15.18.10.5 | server admin | I want Secrets Manager rotation for credentials so that database passwords and API keys rotate automatically. |  |  |
| US-15.18.10.6 | engine tester | I want to verify WAF blocks SQL injection attempts so that WAF enforcement is regression-tested. |  |  |

## US-15.18.11 AWS CDK Open-Source Service Stacks

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.18.11.1 | DevOps (P-16) | I want CDK stacks that deploy only open-source services so that I am not locked into proprietary AWS services. | F-15.18.11 | R-15.18.11 |
| US-15.18.11.2 | Developer (P-15) | I want the same service APIs locally (MinIO, PostgreSQL, Redis) as in production so that I can develop offline. | F-15.18.11 | R-15.18.11 |
| US-15.18.11.3 | Executive (P-1) | I want all infrastructure dependencies to be open source so that the company avoids vendor lock-in risk. | F-15.18.11 | R-15.18.11 |
| US-15.18.11.4 | Engine Dev (P-26) | I want service stacks using standard protocols (S3 API, NATS, PostgreSQL wire protocol) so that integration tests run against local containers. | F-15.18.11 | R-15.18.11 |

## US-15.18.12 1-Click AWS Marketplace Deployment

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.18.12.1 | Developer (P-15) | I want to deploy all services from the AWS Marketplace with a guided wizard so that I do not need DevOps expertise. | F-15.18.12 | R-15.18.12 |
| US-15.18.12.2 | Executive (P-1) | I want a free Marketplace listing so that my team pays only AWS infrastructure costs with no Marketplace fees. | F-15.18.12 | R-15.18.12 |
| US-15.18.12.3 | Server Admin (P-22) | I want the wizard to show estimated monthly cost per scaling profile so that I set budget expectations before deploying. | F-15.18.12 | R-15.18.12 |

## US-15.18.13 Service Admin Dashboard

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.18.13.1 | Server Admin (P-22) | I want a Grafana dashboard showing health of all services so that I detect issues from a single page. | F-15.18.13 | R-15.18.13 |
| US-15.18.13.2 | DevOps (P-16) | I want alerts routed to Slack and PagerDuty via NATS so that on-call engineers are notified immediately. | F-15.18.13 | R-15.18.13 |
| US-15.18.13.3 | Executive (P-1) | I want cost estimates on the dashboard so that I track infrastructure spend without logging into the AWS console. | F-15.18.13 | R-15.18.13 |

## US-15.18.14 Scaling Profiles

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.18.14.1 | Developer (P-15) | I want a Solo profile at ~$20/mo so that I run all services on a personal AWS account affordably. | F-15.18.14 | R-15.18.14 |
| US-15.18.14.2 | DevOps (P-16) | I want a Team profile at ~$100/mo so that a small team shares services without over-provisioning. | F-15.18.14 | R-15.18.14 |
| US-15.18.14.3 | Server Admin (P-22) | I want a Studio profile at ~$500/mo with multi-AZ so that a 10-50 person studio has reliable services. | F-15.18.14 | R-15.18.14 |
| US-15.18.14.4 | Executive (P-1) | I want a Production profile at ~$2000+/mo with auto-scaling and read replicas so that live games are supported. | F-15.18.14 | R-15.18.14 |

## US-15.18.15 Self-Hosted Build Cache Service

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.18.15.1 | Developer (P-15) | I want a build cache with REST API so that my editor fetches pre-compiled assets by content hash. | F-15.18.15 | R-15.18.15 |
| US-15.18.15.2 | DevOps (P-16) | I want Redis-backed cache lookup so that cache hit checks complete in under 5 ms. | F-15.18.15 | R-15.18.15 |
| US-15.18.15.3 | Engine Dev (P-26) | I want Keycloak JWT auth on the cache API so that only authorized users can read or write artifacts. | F-15.18.15 | R-15.18.15 |

## US-15.18.16 Self-Hosted Collaboration Service

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.18.16.1 | Developer (P-15) | I want self-hosted collaboration using PostgreSQL and Redis so that real-time editing works without managed services. | F-15.18.16 | R-15.18.16 |
| US-15.18.16.2 | Server Admin (P-22) | I want Keycloak RBAC for collaboration sessions so that project access is controlled by my identity provider. | F-15.18.16 | R-15.18.16 |
| US-15.18.16.3 | Engine Dev (P-26) | I want the self-hosted collab service to use the same protocol as managed hosting so that the editor client code is identical. | F-15.18.16 | R-15.18.16 |

## US-15.18.17 Self-Hosted Matchmaking Service

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.18.17.1 | Server Admin (P-22) | I want a self-hosted matchmaker using PostgreSQL and Redis so that player data stays on my infrastructure. | F-15.18.17 | R-15.18.17 |
| US-15.18.17.2 | Developer (P-15) | I want ELO/Glicko-2 skill rating and region-based matching so that matchmaking is fair and low-latency. | F-15.18.17 | R-15.18.17 |
| US-15.18.17.3 | DevOps (P-16) | I want NATS-based match events so that game servers receive allocation requests in real time. | F-15.18.17 | R-15.18.17 |

## US-15.18.18 Self-Hosted Asset Store Service

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.18.18.1 | Developer (P-15) | I want a self-hosted asset store with search so that I browse and download assets without an external service. | F-15.18.18 | R-15.18.18 |
| US-15.18.18.2 | Server Admin (P-22) | I want OpenSearch indexing so that full-text and tag-based search returns results in under 200 ms. | F-15.18.18 | R-15.18.18 |
| US-15.18.18.3 | Engine Dev (P-26) | I want asset upload validation so that only well-formed packages are accepted into the store. | F-15.18.18 | R-15.18.18 |

## US-15.18.19 Health Monitoring Stack

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.18.19.1 | Server Admin (P-22) | I want Prometheus + Grafana monitoring so that I use open-source dashboards instead of CloudWatch. | F-15.18.19 | R-15.18.19 |
| US-15.18.19.2 | DevOps (P-16) | I want Loki log aggregation so that I search structured logs across all services in one place. | F-15.18.19 | R-15.18.19 |
| US-15.18.19.3 | Engine Dev (P-26) | I want pre-configured alert rules for each service so that common failure modes trigger notifications automatically. | F-15.18.19 | R-15.18.19 |

## US-15.18.20 Multi-Region Deployment

| ID | Persona | Story | Features | Requirements |
|----|---------|-------|----------|--------------|
| US-15.18.20.1 | Server Admin (P-22) | I want multi-region deployment with PostgreSQL read replicas so that users in different regions get low-latency reads. | F-15.18.20 | R-15.18.20 |
| US-15.18.20.2 | DevOps (P-16) | I want DNS-based failover via Route 53 so that a region failure redirects traffic automatically. | F-15.18.20 | R-15.18.20 |
| US-15.18.20.3 | Executive (P-1) | I want multi-region DR so that a regional AWS outage does not halt game operations. | F-15.18.20 | R-15.18.20 |
