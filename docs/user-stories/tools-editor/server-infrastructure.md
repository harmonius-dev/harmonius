# User Stories -- 15.18 Server Infrastructure

## Stories

| ID            | Persona                 |
|---------------|-------------------------|
| US-15.18.1.1  | build engineer (P-16)   |
| US-15.18.1.2  | engine developer (P-26) |
| US-15.18.2.1  | build engineer (P-16)   |
| US-15.18.2.2  | engine developer (P-26) |
| US-15.18.3.1  | build engineer (P-16)   |
| US-15.18.3.2  | engine developer (P-26) |
| US-15.18.4.1  | build engineer (P-16)   |
| US-15.18.4.2  | technical artist (P-13) |
| US-15.18.5.1  | build engineer (P-16)   |
| US-15.18.5.2  | engine developer (P-26) |
| US-15.18.6.1  | build engineer (P-16)   |
| US-15.18.6.2  | engine developer (P-26) |
| US-15.18.7.1  | build engineer (P-16)   |
| US-15.18.7.2  | engine developer (P-26) |
| US-15.18.8.1  | build engineer (P-16)   |
| US-15.18.8.2  | engine developer (P-26) |
| US-15.18.9.1  | build engineer (P-16)   |
| US-15.18.9.2  | engine developer (P-26) |
| US-15.18.10.1 | build engineer (P-16)   |
| US-15.18.10.2 | engine developer (P-26) |

1. **US-15.18.1.1** — **As a** build engineer (P-16), **I want** modular AWS deployment templates
   provisioning all server components with a single command, **so that** infrastructure setup is
   automated.

2. **US-15.18.1.2** — **As a** engine developer (P-26), **I want** free-tier and enterprise
   deployment profiles, **so that** solo developers and large studios both have appropriate options.

3. **US-15.18.2.1** — **As a** build engineer (P-16), **I want** a self-hosted CRDT collaboration
   server on ECS Fargate with PostgreSQL and S3, **so that** real-time editing runs on our own
   infrastructure.

4. **US-15.18.2.2** — **As a** engine developer (P-26), **I want** the collaboration server to
   handle voice, text, and AI agent channels, **so that** all team features use one backend.

5. **US-15.18.3.1** — **As a** build engineer (P-16), **I want** a self-hosted Forgejo Git server
   with S3-backed LFS and file locking, **so that** source control runs on our infrastructure.

6. **US-15.18.3.2** — **As a** engine developer (P-26), **I want** the Git server to expose a
   GitHub-compatible REST API, **so that** the editor's Git integration works without changes.

7. **US-15.18.4.1** — **As a** build engineer (P-16), **I want** an auto-scaling build farm for
   asset cooking and shader compilation, **so that** builds scale with demand.

8. **US-15.18.4.2** — **As a** technical artist (P-13), **I want** GPU-enabled instances for shader
   compilation, **so that** compile times are minimized.

9. **US-15.18.5.1** — **As a** build engineer (P-16), **I want** a signing and distribution server
   with credentials in AWS Secrets Manager, **so that** signing is automated and secure.

10. **US-15.18.5.2** — **As a** engine developer (P-26), **I want** signing credentials decrypted
    only inside build containers, **so that** keys are never persisted to disk.

11. **US-15.18.6.1** — **As a** build engineer (P-16), **I want** a self-hosted CI/CD pipeline
    triggered on Git push with configurable stages, **so that** builds, tests, and deploys are
    automated.

12. **US-15.18.6.2** — **As a** engine developer (P-26), **I want** pipeline stages for Rust build,
    asset cook, tests, packaging, and deployment, **so that** every commit is validated.

13. **US-15.18.7.1** — **As a** build engineer (P-16), **I want** automated test runner
    infrastructure with unit, integration, screenshot, and performance tests, **so that** quality
    gates block bad builds.

14. **US-15.18.7.2** — **As a** engine developer (P-26), **I want** test results in DynamoDB with
    trend dashboards, **so that** regressions are visible over time.

15. **US-15.18.8.1** — **As a** build engineer (P-16), **I want** shared cache and database services
    on S3, PostgreSQL, and Redis with encryption at rest, **so that** all data is centralized and
    secure.

16. **US-15.18.8.2** — **As a** engine developer (P-26), **I want** intelligent storage tiering and
    backup schedules, **so that** storage costs scale with usage.

17. **US-15.18.9.1** — **As a** build engineer (P-16), **I want** automated backup and restore with
    configurable retention and cross-region replication, **so that** disaster recovery is reliable.

18. **US-15.18.9.2** — **As a** engine developer (P-26), **I want** a restore CLI recovering any
    backup to a new or existing stack, **so that** recovery is scriptable.

19. **US-15.18.10.1** — **As a** build engineer (P-16), **I want** enterprise security with VPC, IAM
    least-privilege, WAF, KMS, TLS 1.3, and audit logging, **so that** infrastructure meets security
    standards.

20. **US-15.18.10.2** — **As a** engine developer (P-26), **I want** the free-tier profile to relax
    controls to stay within cost limits, **so that** solo developers are not overcharged.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-15.18.1 | build engineer (P-16) |
| US-15.18.10 | build engineer (P-16) |
| US-15.18.2 | build engineer (P-16) |
| US-15.18.3 | build engineer (P-16) |
| US-15.18.4 | build engineer (P-16) |
| US-15.18.5 | build engineer (P-16) |
| US-15.18.6 | build engineer (P-16) |
| US-15.18.7 | build engineer (P-16) |
| US-15.18.8 | build engineer (P-16) |
| US-15.18.9 | build engineer (P-16) |

1. **US-15.18.1** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.18.1.1 through US-15.18.1.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

2. **US-15.18.10** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.18.10.1 through US-15.18.10.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

3. **US-15.18.2** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.18.2.1 through US-15.18.2.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

4. **US-15.18.3** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.18.3.1 through US-15.18.3.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

5. **US-15.18.4** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.18.4.1 through US-15.18.4.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

6. **US-15.18.5** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.18.5.1 through US-15.18.5.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

7. **US-15.18.6** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.18.6.1 through US-15.18.6.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

8. **US-15.18.7** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.18.7.1 through US-15.18.7.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

9. **US-15.18.8** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.18.8.1 through US-15.18.8.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

10. **US-15.18.9** -- **As a** build engineer (P-16), **I want** the capabilities defined in
    sub-stories
US-15.18.9.1 through US-15.18.9.2 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.
