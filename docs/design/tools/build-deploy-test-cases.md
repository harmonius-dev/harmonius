# Build and Deploy Test Cases

Companion test cases for [build-deploy.md](build-deploy.md).

Consolidates test cases from the following former files:

- deployment-test-cases.md
- server-infrastructure-test-cases.md
- shared-cache-test-cases.md
- launcher-test-cases.md
- asset-store-test-cases.md

See those files for the full test case definitions until migration is complete.

## Unit Tests

| ID | Requirement | Description |
|----|-------------|-------------|
| TC-15.14.1.1 | R-15.14.1 | Asset cooker platform dispatch |
| TC-15.14.1.2 | R-15.14.1 | Bundle builder BLAKE3 hash |
| TC-15.14.3.1 | R-15.14.3 | Cert rule pass/fail |
| TC-15.14.4.1 | R-15.14.4 | Code signing dispatch |
| TC-15.14.5.1 | R-15.14.5 | Installer format selection |
| TC-15.14.6.1 | R-15.14.6 | DLC entitlement gating |
| TC-15.14.7.1 | R-15.14.7 | CDC chunking determinism |
| TC-15.14.7.2 | R-15.14.7 | Delta patch generation |
| TC-15.14.7.3 | R-15.14.7 | Patch apply + verify |
| TC-15.14.8.1 | R-15.14.8 | Store config validation |
| TC-15.16.1.1 | R-15.16.1 | Mod SDK constraints |
| TC-15.16.2.1 | R-15.16.2 | Mod policy validation |
| TC-15.16.4.1 | R-15.16.4 | Mod sandbox budget check |
| TC-15.16.4.2 | R-15.16.4 | Mod deactivation on exceed |
| TC-15.11.1.1 | R-15.11.1 | Cache key compute |
| TC-15.11.1.2 | R-15.11.1 | Cache key determinism |
| TC-15.11.2.1 | R-15.11.2 | Shader cache key |
| TC-15.11.5.1 | R-15.11.5 | L1 LRU eviction |
| TC-15.11.5.2 | R-15.11.5 | GC retention policy |
| TC-15.15.1.1 | R-15.15.1 | Version install/uninstall |
| TC-15.15.2.1 | R-15.15.2 | Migration chain execution |
| TC-15.15.3.1 | R-15.15.3 | Project create from template |
| TC-15.15.4.1 | R-15.15.4 | Project file read/write |
| TC-15.17.1.1 | R-15.17.1 | Asset catalog search |
| TC-15.17.2.1 | R-15.17.2 | Dependency resolution |
| TC-15.17.3.1 | R-15.17.3 | Review CRUD |
| TC-15.17.5.1 | R-15.17.5 | Compat test trigger |
| TC-15.18.1.1 | R-15.18.1 | Deployment stack synthesis |
| TC-14.8.1.1 | R-14.8.1 | Server-side console build service accepts job |
| TC-14.8.2.1 | R-14.8.2 | Console build enqueues against isolated worker pool |
| TC-14.8.3.1 | R-14.8.3 | Proprietary SDK isolated in sandboxed worker |
| TC-14.8.4.1 | R-14.8.4 | SDK secrets never reach client editor |
| TC-14.8.5.1 | R-14.8.5 | Shared build server assigns queue slot |
| TC-14.8.6.1 | R-14.8.6 | Build server auto-scales on queue depth |
| TC-14.8.7.1 | R-14.8.7 | Remote console deploy transfers artifact |
| TC-14.8.8.1 | R-14.8.8 | Remote deploy reports progress/completion |
| TC-14.8.9.1 | R-14.8.9 | Console build artifact produced in expected format |
| TC-14.8.10.1 | R-14.8.10 | Artifact retained for policy-configured window |
| TC-15.11.3.1 | R-15.11.3 | Logic graph compile cache key |
| TC-15.11.6.1 | R-15.11.6 | Cache backend switch (Garage/TiKV) |
| TC-15.11.8.1 | R-15.11.8 | Cache hit/miss metric counter |
| TC-15.14.2.1 | R-15.14.2 | Deploy-to-device incremental transfer |
| TC-15.14.9.1 | R-15.14.9 | Host/target build matrix dispatch |
| TC-15.15.5.1 | R-15.15.5 | Cross-game preferences roam |
| TC-15.16.3.1 | R-15.16.3 | Mod package publish |
| TC-15.16.5.1 | R-15.16.5 | Mod workshop sync |
| TC-15.17.4.1 | R-15.17.4 | Publisher dashboard metrics |
| TC-15.17.6.1 | R-15.17.6 | Revenue share calculation |
| TC-15.17.7.1 | R-15.17.7 | Asset type manifest validation |
| TC-15.18.2.1 | R-15.18.2 | Collaboration server session open |
| TC-15.18.3.1 | R-15.18.3 | Git/LFS host accepts push |
| TC-15.18.4.1 | R-15.18.4 | Build farm dispatch job |
| TC-15.18.5.1 | R-15.18.5 | Signing server produces signed artifact |
| TC-15.18.6.1 | R-15.18.6 | Continuous deployment pipeline trigger |
| TC-15.18.7.1 | R-15.18.7 | Test runner infra executes test job |
| TC-15.18.8.1 | R-15.18.8 | Shared cache/DB service ready probe |
| TC-15.18.9.1 | R-15.18.9 | Backup + restore cycle |
| TC-15.18.10.1 | R-15.18.10 | Enterprise security config applied |

## Integration Tests

| ID | Requirement | Description |
|----|-------------|-------------|
| TC-15.14.I.1 | R-15.14.1--R-15.14.8 | Full packaging pipeline |
| TC-15.14.I.2 | R-15.14.7 | Delta patch round-trip |
| TC-15.16.I.1 | R-15.16.1--R-15.16.6 | Mod load/unload cycle |
| TC-15.11.I.1 | R-15.11.1--R-15.11.7 | Cache round-trip L1+L2 |
| TC-15.11.I.2 | R-15.11.7 | CI/CD cache population |
| TC-15.15.I.1 | R-15.15.1--R-15.15.6 | Launcher version install |
| TC-15.15.I.2 | R-15.15.2 | Project migration chain |
| TC-15.17.I.1 | R-15.17.1--R-15.17.8 | Store purchase flow |
| TC-15.18.I.1 | R-15.18.1 | Deploy free-tier stack |

## Benchmarks

| ID | Requirement | Description |
|----|-------------|-------------|
| TC-15.14.B.1 | R-15.14.1 | Cook time 10k assets |
| TC-15.14.B.2 | R-15.14.7 | Patch size vs full |
| TC-15.11.B.1 | R-15.11.1 | Cache hit vs miss latency |
| TC-15.11.B.2 | R-15.11.4 | Prefetch throughput |
| TC-15.16.B.1 | R-15.16.4 | Mod load time |
