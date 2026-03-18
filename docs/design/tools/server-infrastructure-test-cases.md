# Server Infrastructure Test Cases

Companion test cases for [server-infrastructure.md](server-infrastructure.md).

## Unit Tests

### TC-15.18.1.1 Free Tier Instance Sizes

| # | Requirement |
|---|-------------|
| 1 | R-15.18.1   |
| 2 | R-15.18.1   |

1. **#1** — Synthesize CDK stacks with `DeploymentProfile::FreeTier`
   - **Expected:** all EC2 instances are `t3.micro` or `t4g.micro`
2. **#2** — Synthesize CDK stacks with `DeploymentProfile::FreeTier`
   - **Expected:** RDS instance class is `db.t3.micro`

### TC-15.18.1.2 Enterprise Multi-AZ

| # | Requirement |
|---|-------------|
| 1 | R-15.18.1   |
| 2 | R-15.18.1   |

1. **#1** — Synthesize CDK stacks with `DeploymentProfile::Enterprise`
   - **Expected:** VPC spans 3 availability zones
2. **#2** — Enterprise RDS configuration
   - **Expected:** `MultiAZ == true`

### TC-15.18.1.3 Stack Outputs Contain Endpoints

| # | Requirement |
|---|-------------|
| 1 | R-15.18.1   |
| 2 | R-15.18.1   |
| 3 | R-15.18.1   |

1. **#1** — Deploy full stack, inspect stack outputs
   - **Expected:** outputs contain collaboration WebSocket URL
2. **#2** — Deploy full stack, inspect stack outputs
   - **Expected:** outputs contain Git server URL
3. **#3** — Deploy full stack, inspect stack outputs
   - **Expected:** outputs contain build API endpoint

### TC-15.18.2.1 CRDT Merge Correctness

| # | Requirement |
|---|-------------|
| 1 | R-15.18.2   |
| 2 | R-15.18.2   |

1. **#1** — Client A sets field X=1, Client B sets field Y=2, merge
   - **Expected:** merged state has X=1 and Y=2
2. **#2** — Client A sets field X=1, Client B sets field X=2, merge
   - **Expected:** last-writer-wins: one value persists, no corruption

### TC-15.18.2.2 Presence Tracking

| # | Requirement |
|---|-------------|
| 1 | R-15.18.2   |

1. **#1** — Client A sends `Presence { user_id: A, position: ... }`
   - **Expected:** Client B receives presence update for user A

### TC-15.18.3.1 Git API Compatibility

| # | Requirement |
|---|-------------|
| 1 | R-15.18.3   |

1. **#1** — `GET /api/v1/repos/{owner}/{repo}` on Forgejo
   - **Expected:** response matches GitHub REST v3 schema for repo endpoint

### TC-15.18.3.2 LFS Lock and Unlock

| # | Requirement |
|---|-------------|
| 1 | R-15.18.3   |
| 2 | R-15.18.3   |

1. **#1** — User A locks `asset.fbx`, User B attempts edit
   - **Expected:** User B receives `LockHeld { owner: "A" }` error
2. **#2** — User A unlocks `asset.fbx`, User B locks
   - **Expected:** lock succeeds for User B

### TC-15.18.4.1 Build Job Cache Hit

| # | Requirement |
|---|-------------|
| 1 | R-15.18.4   |
| 2 | R-15.18.4   |

1. **#1** — Upload artifact for hash H, `check_cache(H)`
   - **Expected:** returns `Some(artifact_url)`
2. **#2** — `submit()` job with cached hash H
   - **Expected:** `BuildJobResult::CacheHit { artifact_url }`

### TC-15.18.4.2 Build Job Cache Miss

| # | Requirement |
|---|-------------|
| 1 | R-15.18.4   |
| 2 | R-15.18.4   |

1. **#1** — `check_cache(new_hash)`
   - **Expected:** returns `None`
2. **#2** — `submit()` job with new hash, poll until complete
   - **Expected:** `BuildJobResult::Compiled { artifact_url, duration_ms }`

### TC-15.18.5.1 Signing Secrets Not on Disk

| # | Requirement |
|---|-------------|
| 1 | R-15.18.5   |
| 2 | R-15.18.5   |

1. **#1** — Inspect signing EC2 instance filesystem
   - **Expected:** no signing credentials in `/tmp`, `/home`, or `/etc`
2. **#2** — Check signing code path
   - **Expected:** credentials fetched from Secrets Manager at use time only

### TC-15.18.6.1 Failed Test Blocks Pipeline

| # | Requirement |
|---|-------------|
| 1 | R-15.18.6   |

1. **#1** — Pipeline Test stage has 1 failure
   - **Expected:** `StageResult { stage: Test, status: Failed }`, Package stage not executed

### TC-15.18.7.1 Screenshot Diff Threshold

| # | Requirement |
|---|-------------|
| 1 | R-15.18.7   |
| 2 | R-15.18.7   |

1. **#1** — Golden image vs test image with 2% pixel diff, threshold = 5%
   - **Expected:** test passes
2. **#2** — Golden image vs test image with 10% pixel diff, threshold = 5%
   - **Expected:** test fails with diff URL

### TC-15.18.8.1 KMS Encryption Enabled

| # | Requirement |
|---|-------------|
| 1 | R-15.18.8   |
| 2 | R-15.18.8   |

1. **#1** — Inspect all S3 buckets in Enterprise stack
   - **Expected:** all have `ServerSideEncryptionConfiguration` with KMS
2. **#2** — Inspect RDS in Enterprise stack
   - **Expected:** `StorageEncrypted == true`, KMS key ARN present

### TC-15.18.9.1 RDS Backup Retention

| # | Requirement |
|---|-------------|
| 1 | R-15.18.9   |
| 2 | R-15.18.9   |

1. **#1** — Free Tier RDS configuration
   - **Expected:** `BackupRetentionPeriod == 7`
2. **#2** — Enterprise RDS configuration
   - **Expected:** `BackupRetentionPeriod == 35`

### TC-15.18.10.1 WAF SQL Injection Block

| # | Requirement |
|---|-------------|
| 1 | R-15.18.10  |
| 2 | R-15.18.10  |

1. **#1** — Send HTTP request with `'; DROP TABLE` in query param through Enterprise ALB
   - **Expected:** request blocked by WAF, HTTP 403
2. **#2** — Send normal HTTP request through Enterprise ALB
   - **Expected:** request passes through, HTTP 200

## Integration Tests

### TC-15.18.1.I1 Full Stack Deploy Free Tier

| # | Requirement |
|---|-------------|
| 1 | R-15.18.1   |

1. **#1** — `cdk deploy --profile free-tier`
   - **Expected:** all stacks deploy successfully, all endpoints return HTTP 200

### TC-15.18.1.I2 Full Stack Deploy Enterprise

| # | Requirement |
|---|-------------|
| 1 | R-15.18.1   |

1. **#1** — `cdk deploy --profile enterprise`
   - **Expected:** all stacks deploy, multi-AZ verified, WAF active, GuardDuty enabled

### TC-15.18.2.I1 Two Editors Sync

| # | Requirement |
|---|-------------|
| 1 | R-15.18.2   |
| 2 | R-15.18.2   |

1. **#1** — Editor A creates entity, Editor B observes
   - **Expected:** entity appears in Editor B within 100 ms
2. **#2** — Editor B modifies entity position, Editor A observes
   - **Expected:** position update visible in Editor A within 100 ms

### TC-15.18.3.I1 Git Push Pull LFS

| # | Requirement |
|---|-------------|
| 1 | R-15.18.3   |

1. **#1** — Push repo with 10 MB LFS object, pull from another client
   - **Expected:** LFS object downloaded, byte-identical to original

### TC-15.18.4.I1 Build Farm Shader Compile

| # | Requirement |
|---|-------------|
| 1 | R-15.18.4   |

1. **#1** — Submit shader compilation job, compare output to local compile
   - **Expected:** remote artifact byte-identical to local compile output

### TC-15.18.4.I2 Build Farm Auto Scale

| # | Requirement |
|---|-------------|
| 1 | R-15.18.4   |

1. **#1** — Submit 50 jobs simultaneously, monitor ASG
   - **Expected:** ASG scales out (instance count increases), then scales back in after idle

### TC-15.18.6.I1 Pipeline End-to-End

| # | Requirement |
|---|-------------|
| 1 | R-15.18.6   |

1. **#1** — Push commit to repo, monitor pipeline
   - **Expected:** all stages complete: Source, Build, Cook, Test, Package, Deploy

### TC-15.18.6.I2 Pipeline Failure Notification

| # | Requirement |
|---|-------------|
| 1 | R-15.18.6   |

1. **#1** — Push commit with compile error
   - **Expected:** pipeline fails at Build stage, SNS notification sent to `failure_topic`

### TC-15.18.7.I1 Screenshot Regression Blocks

| # | Requirement |
|---|-------------|
| 1 | R-15.18.7   |

1. **#1** — Introduce visual regression (change material color)
   - **Expected:** screenshot test fails, pipeline halted at Test stage, diff URL in report

### TC-15.18.9.I1 Restore RDS Backup

| # | Requirement |
|---|-------------|
| 1 | R-15.18.9   |

1. **#1** — Delete data from RDS, restore from latest backup
   - **Expected:** all data recovered, application queries return pre-deletion results

### TC-15.18.9.I2 Cross-Region Replication

| # | Requirement |
|---|-------------|
| 1 | R-15.18.9   |

1. **#1** — Write object to primary S3 bucket
   - **Expected:** object appears in DR region S3 bucket within 60 seconds

### TC-15.18.1.I3 Deploy Time

| # | Requirement |
|---|-------------|
| 1 | R-15.18.1   |

1. **#1** — Full stack deployment from scratch, measure wall time
   - **Expected:** total deployment time < 15 minutes

### TC-15.18.1.I4 Free Tier Cost Limits

| # | Requirement |
|---|-------------|
| 1 | R-15.18.1   |

1. **#1** — Inspect all Free Tier resources
   - **Expected:** all instances within AWS Free Tier eligible types/sizes

## CDK Snapshot Tests

### TC-15.18.1.S1 Network Stack Snapshot

| # | Requirement |
|---|-------------|
| 1 | R-15.18.1   |

1. **#1** — Synthesize `NetworkStack`
   - **Expected:** CloudFormation template matches stored snapshot

### TC-15.18.1.S2 Security Stack Snapshot

| # | Requirement |
|---|-------------|
| 1 | R-15.18.1   |

1. **#1** — Synthesize `SecurityStack`
   - **Expected:** IAM policies match stored snapshot

### TC-15.18.1.S3 Database Stack Snapshot

| # | Requirement |
|---|-------------|
| 1 | R-15.18.1   |

1. **#1** — Synthesize `DatabaseStack`
   - **Expected:** RDS configuration matches stored snapshot

### TC-15.18.1.S4 Build Farm Stack Snapshot

| # | Requirement |
|---|-------------|
| 1 | R-15.18.1   |

1. **#1** — Synthesize `BuildFarmStack`
   - **Expected:** ASG and SQS config matches stored snapshot

### TC-15.18.1.S5 Pipeline Stack Snapshot

| # | Requirement |
|---|-------------|
| 1 | R-15.18.1   |

1. **#1** — Synthesize `PipelineStack`
   - **Expected:** pipeline stages match stored snapshot

## Benchmarks

### TC-15.18.4.B1 Build Job Submission Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Submit job with cached content hash (cache hit) | latency | < 200 ms | R-15.18.4 |

### TC-15.18.2.B1 Collaboration Round-Trip

| # | Metric  | Target   | Requirement |
|---|---------|----------|-------------|
| 1 | latency | < 100 ms | R-15.18.2   |

1. **1** — Send CRDT operation from client A, measure receipt at client B

### TC-15.18.3.B1 Git LFS Download

| # | Metric     | Target                                   | Requirement |
|---|------------|------------------------------------------|-------------|
| 1 | throughput | >= 80% of available CloudFront bandwidth | R-15.18.3   |

1. **1** — Download 100 MB LFS object via CloudFront

### TC-15.18.6.B1 Pipeline Duration

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full pipeline for small project (10k LOC) | wall time | < 30 min | R-15.18.6 |

### TC-15.18.1.B1 Stack Deployment Time

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full stack deployment from scratch | wall time | < 15 min | R-15.18.1 |
