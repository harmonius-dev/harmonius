# R-15.18 -- Server Infrastructure Requirements

## Requirements

1. **R-15.18.1** — The engine **SHALL** provide modular AWS deployment templates provisioning all
   server components with free-tier and enterprise profiles.
   - **Rationale:** Modular stacks let teams deploy only the services they need.
   - **Verification:** Deploy the free-tier profile and verify all components start and pass health
     checks.

2. **R-15.18.2** — The engine **SHALL** provide a self-hosted CRDT collaboration server on ECS
   Fargate with PostgreSQL, S3, and WebSocket support.
   - **Rationale:** Self-hosted collaboration avoids vendor dependency for real-time editing.
   - **Verification:** Deploy the collaboration server and verify two editors synchronize in real
     time.

3. **R-15.18.3** — The engine **SHALL** provide a self-hosted Forgejo Git server with S3-backed LFS,
   file locking, and GitHub-compatible REST API.
   - **Rationale:** Self-hosted Git avoids hosting vendor lock-in.
   - **Verification:** Push, pull, and lock a file via the editor and verify all operations succeed.

4. **R-15.18.4** — The engine **SHALL** provide an auto-scaling build farm with GPU-enabled
   instances for shader compilation and CPU instances for asset cooking.
   - **Rationale:** Auto-scaling matches build capacity to demand.
   - **Verification:** Submit 10 concurrent build jobs and verify the auto-scaling group provisions
     additional instances.

5. **R-15.18.5** — The engine **SHALL** provide a signing and distribution server with credentials
   in AWS Secrets Manager, decrypted only inside build containers.
   - **Rationale:** Automated signing with secure credential handling is required for distribution.
   - **Verification:** Sign a build and verify credentials are not persisted to disk outside Secrets
     Manager.

6. **R-15.18.6** — The engine **SHALL** provide a self-hosted CI/CD pipeline with configurable
   stages triggered on Git push events.
   - **Rationale:** Automated CI/CD validates every commit and deploys builds.
   - **Verification:** Push a commit and verify the pipeline runs through all stages.

7. **R-15.18.7** — The engine **SHALL** provide test runner infrastructure for unit, integration,
   screenshot, and performance tests with result storage and trend dashboards.
   - **Rationale:** Comprehensive testing infrastructure catches regressions automatically.
   - **Verification:** Run a screenshot test and verify the result appears in the dashboard.

8. **R-15.18.8** — The engine **SHALL** provide shared cache and database services on S3,
   PostgreSQL, and Redis with encryption at rest and intelligent storage tiering.
   - **Rationale:** Centralized data services support all server components efficiently.
   - **Verification:** Store and retrieve a cache entry and verify encryption at rest is enabled.

9. **R-15.18.9** — The engine **SHALL** provide automated backup and restore with configurable
   retention, cross-region replication, and a restore CLI.
   - **Rationale:** Disaster recovery protects against data loss.
   - **Verification:** Create a backup, destroy the stack, restore from backup, and verify data
     integrity.

10. **R-15.18.10** — The engine **SHALL** enforce enterprise security (VPC, IAM, WAF, KMS, TLS 1.3,
    CloudTrail) in the enterprise profile, with relaxed controls in the free-tier.
    - **Rationale:** Enterprise environments require defense in depth; free-tier must be affordable.
    - **Verification:** Deploy enterprise profile and verify WAF blocks a SQL injection attempt.
