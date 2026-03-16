# Server Infrastructure Test Cases

Companion test cases for [server-infrastructure.md](server-infrastructure.md).

## Unit Tests

### TC-15.18.1.1 Free Tier Instance Sizes

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Synthesize CDK stacks with `DeploymentProfile::FreeTier` | all EC2 instances are `t3.micro` or `t4g.micro` | R-15.18.1 |
| 2 | Synthesize CDK stacks with `DeploymentProfile::FreeTier` | RDS instance class is `db.t3.micro` | R-15.18.1 |

### TC-15.18.1.2 Enterprise Multi-AZ

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Synthesize CDK stacks with `DeploymentProfile::Enterprise` | VPC spans 3 availability zones | R-15.18.1 |
| 2 | Enterprise RDS configuration | `MultiAZ == true` | R-15.18.1 |

### TC-15.18.1.3 Stack Outputs Contain Endpoints

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Deploy full stack, inspect stack outputs | outputs contain collaboration WebSocket URL | R-15.18.1 |
| 2 | Deploy full stack, inspect stack outputs | outputs contain Git server URL | R-15.18.1 |
| 3 | Deploy full stack, inspect stack outputs | outputs contain build API endpoint | R-15.18.1 |

### TC-15.18.2.1 CRDT Merge Correctness

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Client A sets field X=1, Client B sets field Y=2, merge | merged state has X=1 and Y=2 | R-15.18.2 |
| 2 | Client A sets field X=1, Client B sets field X=2, merge | last-writer-wins: one value persists, no corruption | R-15.18.2 |

### TC-15.18.2.2 Presence Tracking

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Client A sends `Presence { user_id: A, position: ... }` | Client B receives presence update for user A | R-15.18.2 |

### TC-15.18.3.1 Git API Compatibility

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `GET /api/v1/repos/{owner}/{repo}` on Forgejo | response matches GitHub REST v3 schema for repo endpoint | R-15.18.3 |

### TC-15.18.3.2 LFS Lock and Unlock

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | User A locks `asset.fbx`, User B attempts edit | User B receives `LockHeld { owner: "A" }` error | R-15.18.3 |
| 2 | User A unlocks `asset.fbx`, User B locks | lock succeeds for User B | R-15.18.3 |

### TC-15.18.4.1 Build Job Cache Hit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Upload artifact for hash H, `check_cache(H)` | returns `Some(artifact_url)` | R-15.18.4 |
| 2 | `submit()` job with cached hash H | `BuildJobResult::CacheHit { artifact_url }` | R-15.18.4 |

### TC-15.18.4.2 Build Job Cache Miss

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `check_cache(new_hash)` | returns `None` | R-15.18.4 |
| 2 | `submit()` job with new hash, poll until complete | `BuildJobResult::Compiled { artifact_url, duration_ms }` | R-15.18.4 |

### TC-15.18.5.1 Signing Secrets Not on Disk

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Inspect signing EC2 instance filesystem | no signing credentials in `/tmp`, `/home`, or `/etc` | R-15.18.5 |
| 2 | Check signing code path | credentials fetched from Secrets Manager at use time only | R-15.18.5 |

### TC-15.18.6.1 Failed Test Blocks Pipeline

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pipeline Test stage has 1 failure | `StageResult { stage: Test, status: Failed }`, Package stage not executed | R-15.18.6 |

### TC-15.18.7.1 Screenshot Diff Threshold

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Golden image vs test image with 2% pixel diff, threshold = 5% | test passes | R-15.18.7 |
| 2 | Golden image vs test image with 10% pixel diff, threshold = 5% | test fails with diff URL | R-15.18.7 |

### TC-15.18.8.1 KMS Encryption Enabled

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Inspect all S3 buckets in Enterprise stack | all have `ServerSideEncryptionConfiguration` with KMS | R-15.18.8 |
| 2 | Inspect RDS in Enterprise stack | `StorageEncrypted == true`, KMS key ARN present | R-15.18.8 |

### TC-15.18.9.1 RDS Backup Retention

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Free Tier RDS configuration | `BackupRetentionPeriod == 7` | R-15.18.9 |
| 2 | Enterprise RDS configuration | `BackupRetentionPeriod == 35` | R-15.18.9 |

### TC-15.18.10.1 WAF SQL Injection Block

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send HTTP request with `'; DROP TABLE` in query param through Enterprise ALB | request blocked by WAF, HTTP 403 | R-15.18.10 |
| 2 | Send normal HTTP request through Enterprise ALB | request passes through, HTTP 200 | R-15.18.10 |

## Integration Tests

### TC-15.18.1.I1 Full Stack Deploy Free Tier

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `cdk deploy --profile free-tier` | all stacks deploy successfully, all endpoints return HTTP 200 | R-15.18.1 |

### TC-15.18.1.I2 Full Stack Deploy Enterprise

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `cdk deploy --profile enterprise` | all stacks deploy, multi-AZ verified, WAF active, GuardDuty enabled | R-15.18.1 |

### TC-15.18.2.I1 Two Editors Sync

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Editor A creates entity, Editor B observes | entity appears in Editor B within 100 ms | R-15.18.2 |
| 2 | Editor B modifies entity position, Editor A observes | position update visible in Editor A within 100 ms | R-15.18.2 |

### TC-15.18.3.I1 Git Push Pull LFS

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Push repo with 10 MB LFS object, pull from another client | LFS object downloaded, byte-identical to original | R-15.18.3 |

### TC-15.18.4.I1 Build Farm Shader Compile

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit shader compilation job, compare output to local compile | remote artifact byte-identical to local compile output | R-15.18.4 |

### TC-15.18.4.I2 Build Farm Auto Scale

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit 50 jobs simultaneously, monitor ASG | ASG scales out (instance count increases), then scales back in after idle | R-15.18.4 |

### TC-15.18.6.I1 Pipeline End-to-End

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Push commit to repo, monitor pipeline | all stages complete: Source, Build, Cook, Test, Package, Deploy | R-15.18.6 |

### TC-15.18.6.I2 Pipeline Failure Notification

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Push commit with compile error | pipeline fails at Build stage, SNS notification sent to `failure_topic` | R-15.18.6 |

### TC-15.18.7.I1 Screenshot Regression Blocks

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Introduce visual regression (change material color) | screenshot test fails, pipeline halted at Test stage, diff URL in report | R-15.18.7 |

### TC-15.18.9.I1 Restore RDS Backup

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Delete data from RDS, restore from latest backup | all data recovered, application queries return pre-deletion results | R-15.18.9 |

### TC-15.18.9.I2 Cross-Region Replication

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write object to primary S3 bucket | object appears in DR region S3 bucket within 60 seconds | R-15.18.9 |

### TC-15.18.1.I3 Deploy Time

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Full stack deployment from scratch, measure wall time | total deployment time < 15 minutes | R-15.18.1 |

### TC-15.18.1.I4 Free Tier Cost Limits

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Inspect all Free Tier resources | all instances within AWS Free Tier eligible types/sizes | R-15.18.1 |

## CDK Snapshot Tests

### TC-15.18.1.S1 Network Stack Snapshot

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Synthesize `NetworkStack` | CloudFormation template matches stored snapshot | R-15.18.1 |

### TC-15.18.1.S2 Security Stack Snapshot

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Synthesize `SecurityStack` | IAM policies match stored snapshot | R-15.18.1 |

### TC-15.18.1.S3 Database Stack Snapshot

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Synthesize `DatabaseStack` | RDS configuration matches stored snapshot | R-15.18.1 |

### TC-15.18.1.S4 Build Farm Stack Snapshot

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Synthesize `BuildFarmStack` | ASG and SQS config matches stored snapshot | R-15.18.1 |

### TC-15.18.1.S5 Pipeline Stack Snapshot

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Synthesize `PipelineStack` | pipeline stages match stored snapshot | R-15.18.1 |

## Benchmarks

### TC-15.18.4.B1 Build Job Submission Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Submit job with cached content hash (cache hit) | latency | < 200 ms | R-15.18.4 |

### TC-15.18.2.B1 Collaboration Round-Trip

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Send CRDT operation from client A, measure receipt at client B | latency | < 100 ms | R-15.18.2 |

### TC-15.18.3.B1 Git LFS Download

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Download 100 MB LFS object via CloudFront | throughput | >= 80% of available CloudFront bandwidth | R-15.18.3 |

### TC-15.18.6.B1 Pipeline Duration

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full pipeline for small project (10k LOC) | wall time | < 30 min | R-15.18.6 |

### TC-15.18.1.B1 Stack Deployment Time

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full stack deployment from scratch | wall time | < 15 min | R-15.18.1 |
