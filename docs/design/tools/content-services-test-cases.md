# Content Services Test Cases

Companion test cases for [content-services.md](content-services.md).

Consolidates test cases from the following former files:

- localization-docs-test-cases.md
- ai-governance-test-cases.md

See those files for the full test case definitions until
migration is complete.

## Unit Tests

| ID | Requirement | Description |
|----|-------------|-------------|
| TC-15.13.1.1 | R-15.13.1 | ICU pattern validation |
| TC-15.13.1.2 | R-15.13.1 | Plural category selection |
| TC-15.13.1.3 | R-15.13.1 | TM fuzzy match accuracy |
| TC-15.13.1.4 | R-15.13.1 | String entry lock/unlock |
| TC-15.13.2.1 | R-15.13.2 | Pseudo-loc transform |
| TC-15.13.2.2 | R-15.13.2 | RTL bidi resolution |
| TC-15.13.2.3 | R-15.13.2 | Missing translation detect |
| TC-15.13.3.1 | R-15.13.3 | XLIFF encode round-trip |
| TC-15.13.3.2 | R-15.13.3 | XLIFF import merge |
| TC-15.13.3.3 | R-15.13.3 | Source dirty detection |
| TC-15.19.1.1 | R-15.19.1 | API ref type extraction |
| TC-15.19.2.1 | R-15.19.2 | Node doc generation |
| TC-15.19.3.1 | R-15.19.3 | Tutorial step advance |
| TC-15.19.5.1 | R-15.19.5 | Contextual help lookup |
| TC-15.19.5.2 | R-15.19.5 | Search index trigram |
| TC-15.7.1.1 | R-15.7.1 | Provenance tag create |
| TC-15.7.1.2 | R-15.7.1 | Provenance tag query |
| TC-15.7.2.1 | R-15.7.2 | Bitmask mark modified |
| TC-15.7.2.2 | R-15.7.2 | Fully replaced detect |
| TC-15.7.3.1 | R-15.7.3 | Generative toggle gate |
| TC-15.7.4.1 | R-15.7.4 | Assistance toggle gate |
| TC-15.7.5.1 | R-15.7.5 | Policy signature verify |
| TC-15.7.5.2 | R-15.7.5 | Policy version monotonic |
| TC-15.7.6.1 | R-15.7.6 | Audit hash chain append |
| TC-15.7.6.2 | R-15.7.6 | Audit chain validation |
| TC-15.7.7.1 | R-15.7.7 | Review route AI asset |
| TC-15.7.7.2 | R-15.7.7 | Review approve/reject |
| TC-15.9.2.1 | R-15.9.2 | Tool registration |
| TC-15.9.2.2 | R-15.9.2 | Tool execution |
| TC-15.9.4.1 | R-15.9.4 | Shortcut recommendations |
| TC-15.9.10.1 | R-15.9.10 | Rate limiter token bucket |
| TC-15.9.10.2 | R-15.9.10 | Quota enforcement |

## Integration Tests

| ID | Requirement | Description |
|----|-------------|-------------|
| TC-15.13.I.1 | R-15.13.1--R-15.13.3 | Full loc pipeline |
| TC-15.13.I.2 | R-15.13.3 | XLIFF export/import |
| TC-15.19.I.1 | R-15.19.1 | API reference generation |
| TC-15.7.I.1 | R-15.7.1--R-15.7.7 | Provenance-to-review |
| TC-15.7.I.2 | R-15.7.5 | Policy distribution |
| TC-15.9.I.1 | R-15.9.2 | LLM tool execution |
| TC-15.9.I.2 | R-15.9.9 | Multi-modal context |

## Benchmarks

| ID | Requirement | Description |
|----|-------------|-------------|
| TC-15.13.B.1 | R-15.13.1 | TM lookup 100k entries |
| TC-15.19.B.1 | R-15.19.5 | Search query latency |
| TC-15.7.B.1 | R-15.7.6 | Audit log append |
| TC-15.7.B.2 | R-15.7.1 | Provenance query 50k |
| TC-15.9.B.1 | R-15.9.2 | LLM round-trip latency |
