# Content Services Test Cases

Companion test cases for [content-services.md](content-services.md).

## Unit Tests

| ID | Requirement | Description | Input | Expected Output |
|----|-------------|-------------|-------|-----------------|
| TC-15.13.1.1 | R-15.13.1 | ICU pattern validation | `"{count, plural, one{# item} other{# items}}"` | `Ok(())` |
| TC-15.13.1.2 | R-15.13.1 | Plural category selection | `(locale="en", n=1)` | `PluralCategory::One` |
| TC-15.13.1.3 | R-15.13.1 | TM fuzzy match accuracy | source `"Save file"`, TM has `"Save files"` | suggestion score ≥ 0.85 |
| TC-15.13.1.4 | R-15.13.1 | String entry lock/unlock | lock key `"ui.save"`, then try `set_translation` | `Err(LocaleError::Locked)` |
| TC-15.13.2.1 | R-15.13.2 | Pseudo-loc transform | `"Hello"` | `"[Ħêĺĺø]"` (bracketed + accented) |
| TC-15.13.2.2 | R-15.13.2 | RTL bidi resolution | Arabic string `"مرحبا"` | `TextDirection::RightToLeft` |
| TC-15.13.2.3 | R-15.13.2 | Missing translation detect | model with `"en"` only, validate `"ar"` | `ValidationCategory::MissingTranslation` |
| TC-15.13.3.1 | R-15.13.3 | XLIFF encode round-trip | `StringTableModel` with 10 entries | decoded model equals original |
| TC-15.13.3.2 | R-15.13.3 | XLIFF import merge | XLIFF with 3 translated, 1 new key | 3 updated + 1 inserted, 0 lost |
| TC-15.13.3.3 | R-15.13.3 | Source dirty detection | change source text after export | `TransUnitState::Initial` reset on reimport |
| TC-15.19.1.1 | R-15.19.1 | API ref type extraction | middleman .dylib with 5 types | 5 `TypeDescriptor` entries produced |
| TC-15.19.2.1 | R-15.19.2 | Node doc generation | logic graph node with 3 ports | doc entry with port descriptions |
| TC-15.19.3.1 | R-15.19.3 | Tutorial step advance | tutorial with 3 steps, advance twice | `TutorialState::Running`, step index 2 |
| TC-15.19.5.1 | R-15.19.5 | Contextual help lookup | widget type `"ColorPicker"`, locale `"en"` | `Some(HelpEntry)` with non-empty body |
| TC-15.19.5.2 | R-15.19.5 | Search index trigram | index 100 entries, query `"tex"` | returns entries containing `"texture"` |
| TC-15.7.1.1 | R-15.7.1 | Provenance tag create | `tag_asset(id, TextureGenerator, "v1", hash, user)` | `has_provenance(id) == true` |
| TC-15.7.1.2 | R-15.7.1 | Provenance tag query | 50 tagged assets, filter by `AiSystem::MeshGenerator` | returns only mesh-generated asset IDs |
| TC-15.7.2.1 | R-15.7.2 | Bitmask mark modified | 10-region bitmask, mark region 3 | `modification_pct() ≈ 10.0` |
| TC-15.7.2.2 | R-15.7.2 | Fully replaced detect | mark all 10 regions modified | `is_fully_replaced() == true` |
| TC-15.7.3.1 | R-15.7.3 | Generative toggle gate | `generative_ai_enabled = false` | `is_generative_enabled() == false` |
| TC-15.7.4.1 | R-15.7.4 | Assistance toggle gate | `assistance_enabled = false` | `process_command` returns `Err(AssistantError::Disabled)` |
| TC-15.7.5.1 | R-15.7.5 | Policy signature verify | `PolicyDocument` with tampered bytes | `apply_policy` returns `Err(GovernanceError::InvalidSignature)` |
| TC-15.7.5.2 | R-15.7.5 | Policy version monotonic | apply policy v5, then try v3 | `Err(GovernanceError::StalePolicy)` |
| TC-15.7.6.1 | R-15.7.6 | Audit hash chain append | append 3 entries sequentially | each entry's `prev_hash` equals prior entry's hash |
| TC-15.7.6.2 | R-15.7.6 | Audit chain validation | tamper entry 2 in a 5-entry chain | validation fails at entry 2 |
| TC-15.7.7.1 | R-15.7.7 | Review route AI asset | tag asset, call `route_to_review` | asset appears in reviewer's queue |
| TC-15.7.7.2 | R-15.7.7 | Review approve/reject | approve asset in queue | `ReviewStatus::Approved`, removed from queue |
| TC-15.9.2.1 | R-15.9.2 | Tool registration | register `ToolDefinition` with id `"place_mesh"` | `execute("place_mesh", ...)` succeeds |
| TC-15.9.2.2 | R-15.9.2 | Tool execution | invoke registered tool with valid args | `Ok(ToolResult)` with expected output |
| TC-15.9.4.1 | R-15.9.4 | Shortcut recommendations | context `"select all"`, last 5 actions | returns relevant shortcut suggestion |
| TC-15.9.10.1 | R-15.9.10 | Rate limiter token bucket | 100 req/min limit, send 101 requests | 101st returns `Err(RateLimitExceeded)` |
| TC-15.9.10.2 | R-15.9.10 | Quota enforcement | cost budget $0.10, exceed via requests | subsequent request returns `Err(BudgetExceeded)` |

## Integration Tests

| ID | Requirement | Description | Input | Expected Output |
|----|-------------|-------------|-------|-----------------|
| TC-15.13.I.1 | R-15.13.1--R-15.13.3 | Full localization pipeline | 20 strings, validate + export XLIFF + import translations | all 20 strings resolved, 0 missing |
| TC-15.13.I.2 | R-15.13.3 | XLIFF export/import round-trip | model with metadata, export then import | model contents identical after round-trip |
| TC-15.19.I.1 | R-15.19.1 | API reference generation | middleman .dylib with 20 types | HTML output with 20 type pages + search index |
| TC-15.7.I.1 | R-15.7.1--R-15.7.7 | Provenance-to-review workflow | AI generates asset, artist modifies 50% | asset in review queue with correct bitmask |
| TC-15.7.I.2 | R-15.7.5 | Policy distribution | enterprise pushes policy via QUIC | all editor instances apply policy within 1 frame |
| TC-15.9.I.1 | R-15.9.2 | LLM tool execution round-trip | command `"place a red cube"`, tools registered | `place_mesh` tool called with correct args |
| TC-15.9.I.2 | R-15.9.9 | Multi-modal context assembly | text + screenshot + accessibility tree | `LlmRequest` contains all three modalities |

## Benchmarks

| ID | Requirement | Description | Input | Target |
|----|-------------|-------------|-------|--------|
| TC-15.13.B.1 | R-15.13.1 | TM lookup 100K entries | 100K indexed pairs, 1K random queries | p99 < 5 ms |
| TC-15.13.B.2 | R-15.13.1 | String table load 10K keys | rkyv file with 10K entries | load time < 5 ms |
| TC-15.19.B.1 | R-15.19.5 | Search query latency | index 10K doc entries, 1K queries | p99 < 10 ms |
| TC-15.7.B.1 | R-15.7.6 | Audit log append throughput | sequential appends, measure entries/s | > 100K entries/s |
| TC-15.7.B.2 | R-15.7.1 | Provenance query 50K assets | 50K tagged assets, filter by system | < 50 ms |
| TC-15.9.B.1 | R-15.9.2 | LLM round-trip latency | single request to provider endpoint | p50 < 2 s |
| TC-15.7.B.3 | R-15.7.4 | Feature toggle apply | call `apply_policy` with valid doc | < 1 ms |
