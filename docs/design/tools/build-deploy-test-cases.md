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
