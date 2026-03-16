# AI Governance and Assistant Test Cases

Companion test cases for [ai-governance.md](ai-governance.md).

## Unit Tests

### TC-15.7.1.1 Provenance Tag Attach and Query

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `tag_asset(asset_42, TextureGenerator, "sd-xl-1.0", prompt_hash)` | `ProvenanceTag { asset_id: 42, ai_system: TextureGenerator, model_version: "sd-xl-1.0" }` stored | R-15.7.1 |
| 2 | `get_provenance(asset_42)` | Returns tag matching fields from step 1 | R-15.7.1 |
| 3 | `get_provenance(asset_99)` (untagged) | Returns `None` | R-15.7.1 |

### TC-15.7.1.2 Provenance Survives Serialization

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Tag asset, serialize to bytes, deserialize | Deserialized tag matches original fields | R-15.7.1 |
| 2 | Tag asset, save to disk, reload | Reloaded tag matches original fields | R-15.7.1 |

### TC-15.7.1.3 Provenance Removal on Full Replace

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Tag asset with 100 regions, mark all 100 modified | `get_provenance(asset)` returns `None` | R-15.7.1, R-15.7.2 |
| 2 | Tag asset with 100 regions, mark 99 modified | `get_provenance(asset)` returns `Some(tag)` | R-15.7.1, R-15.7.2 |

### TC-15.7.2.1 Modification Bitmask Granularity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `start_tracking(asset, VertexGroup)`, mark region 0 | `bitmask.regions[0] == true`, rest false | R-15.7.2 |
| 2 | `start_tracking(asset, TextureTile(256))`, mark tile 5 | `bitmask.regions[5] == true`, rest false | R-15.7.2 |
| 3 | `start_tracking(asset, DataTableRow)`, mark row 10 | `bitmask.regions[10] == true`, rest false | R-15.7.2 |
| 4 | `start_tracking(asset, LogicGraphNode)`, mark node 3 | `bitmask.regions[3] == true`, rest false | R-15.7.2 |

### TC-15.7.2.2 Bitmask Size Under 1 KB

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Bitmask with 1000 regions | `size_of(bitmask) < 1024` bytes | R-15.7.2 |
| 2 | Bitmask with 8000 regions | `size_of(bitmask) < 1024` bytes | R-15.7.2 |

### TC-15.7.2.3 Modification Percentage Calculation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 regions, 50 marked modified | `modification_percentage() == 0.50` | R-15.7.2 |
| 2 | 100 regions, 0 marked modified | `modification_percentage() == 0.0` | R-15.7.2 |
| 3 | 100 regions, 100 marked modified | `modification_percentage() == 1.0` | R-15.7.2 |

### TC-15.7.3.1 Generative Toggle Disables LLM

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `generative_ai_enabled = false`, call LLM generation | Returns `Err(FeatureDisabled)` | R-15.7.3 |
| 2 | `generative_ai_enabled = true`, call LLM generation | Returns `Ok(...)` | R-15.7.3 |

### TC-15.7.3.2 Deterministic AI Unaffected

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `generative_ai_enabled = false`, run pathfinding | Pathfinding returns valid path | R-15.7.3 |
| 2 | `generative_ai_enabled = false`, run behavior tree | BT executes normally | R-15.7.3 |
| 3 | `generative_ai_enabled = false`, run GOAP planner | GOAP returns valid plan | R-15.7.3 |

### TC-15.7.4.1 Toggle Independence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `generative=true, assistance=true` | Both LLM generation and assistant available | R-15.7.4 |
| 2 | `generative=true, assistance=false` | LLM generation available, assistant disabled | R-15.7.4 |
| 3 | `generative=false, assistance=true` | Assistant available, LLM generation disabled | R-15.7.4 |
| 4 | `generative=false, assistance=false` | Both disabled | R-15.7.4 |

### TC-15.7.4.2 Toggle Persistence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set toggles `(true, false, true, false)`, save, reload | Reloaded toggles match `(true, false, true, false)` | R-15.7.4 |

### TC-15.7.5.1 Ed25519 Signature Verification

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sign policy with key A, verify with key A | Verification succeeds | R-15.7.5 |
| 2 | Sign policy with key A, tamper 1 byte, verify with key A | Verification fails with `InvalidSignature` | R-15.7.5 |
| 3 | Sign policy with key A, verify with key B | Verification fails with `UntrustedKey` | R-15.7.5 |

### TC-15.7.5.2 Stale Policy Rejected

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Current policy version 5, receive version 4 | `Err(StaleVersion { current: 5, received: 4 })` | R-15.7.5 |
| 2 | Current policy version 5, receive version 5 | `Err(StaleVersion { current: 5, received: 5 })` | R-15.7.5 |
| 3 | Current policy version 5, receive version 6 | Policy applied successfully | R-15.7.5 |

### TC-15.7.6.1 Audit Hash Chain

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Append 100 entries, `validate_chain()` | Returns `Ok(true)` | R-15.7.6 |
| 2 | Append 100 entries, tamper entry 50, `validate_chain()` | Returns `Ok(false)`, chain breaks at entry 50 | R-15.7.6 |

### TC-15.7.6.2 Audit Rotation Preserves Chain

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Append 1000 entries, rotate at 500, `validate_chain()` | Returns `Ok(true)` across both segments | R-15.7.6 |

### TC-15.7.7.1 Review Auto-Approve Threshold

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Threshold 80%, mark 80% modified | Asset status changes to approved | R-15.7.7 |
| 2 | Threshold 80%, mark 79% modified | Asset status remains pending | R-15.7.7 |

### TC-15.7.7.2 Review Blocks Unapproved

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | AI asset with `require_review = true`, not reviewed | Asset not in production database | R-15.7.7 |
| 2 | AI asset with `require_review = true`, approved | Asset in production database | R-15.7.7 |

### TC-15.7.8.1 Packaged Provenance Minimal

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Package asset with full provenance tag | Packaged output contains only `ai_system` and `is_ai_generated` flag | R-15.7.8 |
| 2 | Package asset without provenance | Packaged output has no provenance metadata | R-15.7.8 |

### TC-15.9.10.1 Rate Limiter Token Bucket

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Bucket with 10 tokens, acquire 10 permits | All succeed | R-15.9.10 |
| 2 | Bucket with 10 tokens, acquire 11th permit | Returns `Err(RateLimited { ... })` | R-15.9.10 |
| 3 | Wait for refill period, acquire permit | Succeeds | R-15.9.10 |

### TC-15.9.10.2 Quota Requests Per Hour

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Quota `requests_per_hour = 100`, use 100 requests | 100th succeeds | R-15.9.10 |
| 2 | Quota `requests_per_hour = 100`, use 101st request | Returns `Err(QuotaExceeded { limit: "requests_per_hour" })` | R-15.9.10 |

### TC-15.9.10.3 Quota Tokens Per Day

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Quota `tokens_per_day = 50000`, record 50000 tokens | Succeeds | R-15.9.10 |
| 2 | Quota `tokens_per_day = 50000`, record 50001 tokens | Returns `Err(QuotaExceeded { limit: "tokens_per_day" })` | R-15.9.10 |

### TC-15.9.10.4 Tool Allowlist

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Allowlist `{tool_A, tool_B}`, check `tool_A` | `is_allowed` returns `true` | R-15.9.10 |
| 2 | Allowlist `{tool_A, tool_B}`, check `tool_C` | `is_allowed` returns `false` | R-15.9.10 |

### TC-15.9.10.5 Tool Blocklist

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Blocklist `{tool_X}`, check `tool_X` | `is_allowed` returns `false` | R-15.9.10 |
| 2 | Blocklist `{tool_X}`, check `tool_Y` | `is_allowed` returns `true` | R-15.9.10 |

### TC-15.9.2.1 Tool Validation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Tool expects `Range { min: 0, max: 100 }`, pass value 50 | Validation passes | R-15.9.2 |
| 2 | Tool expects `Range { min: 0, max: 100 }`, pass value 150 | Validation fails with error message | R-15.9.2 |
| 3 | Tool expects `Required { field: "name" }`, omit `name` | Validation fails with error message | R-15.9.2 |

### TC-15.9.2.2 Tool Undo Integration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Execute tool with `supports_undo = true` | `ToolResult::Success { undo_command: Some(...) }` | R-15.9.2 |
| 2 | Execute tool with `supports_undo = false` | `ToolResult::Success { undo_command: None }` | R-15.9.2 |

### TC-15.9.6b.1 Agent Isolation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create agent A and B, select entity 1 in A | Agent B selection is empty | R-15.9.6b |
| 2 | Create agent A and B, modify undo stack in A | Agent B undo stack unchanged | R-15.9.6b |

### TC-15.9.6b.2 Agent Independent Undo

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Agent A executes tool, agent B executes tool, undo in A | Agent A reverted, agent B unchanged | R-15.9.6b |

### TC-15.9.9.1 Context Build Precedence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Context with both accessibility snapshot and screenshot | Prompt contains structured data before pixel data | R-15.9.9 |
| 2 | Context with only text input | Prompt contains text, no image tokens | R-15.9.9 |

## Integration Tests

### TC-15.7.1.I1 Provenance Pipeline Survival

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Tag asset, run import/cook/package pipeline | Packaged output contains provenance metadata | R-15.7.1 |

### TC-15.7.5.I1 Policy Push via WebSocket

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mock admin server pushes signed policy via WebSocket | Editor applies new toggle state within 5 seconds | R-15.7.5 |

### TC-15.7.5.I2 Policy Pull via HTTPS

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mock admin server hosts signed policy, editor polls | Editor applies new toggle state on next poll cycle | R-15.7.5 |

### TC-15.7.6.I1 Audit Replication

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Append 500 entries locally, `replicate_to(mock_server)` | All 500 entries received by mock server | R-15.7.6 |

### TC-15.7.7.I1 Review Workflow End to End

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Generate AI asset, route to review, approve | Asset present in production database | R-15.7.7 |
| 2 | Generate AI asset, route to review, reject | Asset not in production database, reason recorded | R-15.7.7 |

### TC-15.9.2.I1 LLM Request Async

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send request to mock LLM API via IoReactor | Response parsed, tool calls extracted correctly | R-15.9.2 |

### TC-15.9.1b.I1 Voice Command Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Transcript "select the red cube", mock LLM returns `select_entity` tool call | Entity selection tool invoked with correct parameters | R-15.9.1b |

### TC-15.9.6a.I1 Headless Tool Execution

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Execute `set_property` tool via headless API | Property value matches interactive execution result | R-15.9.6a |

### TC-15.9.6c.I1 CI Agent Artifact Production

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Run agent in headless mode with build task | Build artifact produced at expected path | R-15.9.6c |

### TC-15.9.10.I1 Offline Then Reconnect

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Operate offline for 10 interactions, reconnect | All 10 local audit entries replicated to server | R-15.9.10 |

## Benchmarks

### TC-15.7.6.B1 Audit Log Append

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single `AuditLog::append` call | Latency | < 10 us | R-15.7.6 |

### TC-15.7.6.B2 Hash Chain Validation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `validate_chain()` over 10,000 entries | Latency | < 50 ms | R-15.7.6 |

### TC-15.7.1.B1 Provenance Tag Lookup

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `get_provenance(asset_id)` | Latency | < 1 us | R-15.7.1 |

### TC-15.7.2.B1 Modification Bitmask Update

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `mark_modified(asset, region)` | Latency | < 500 ns | R-15.7.2 |

### TC-15.7.3.B1 Toggle State Check

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `check(feature)` | Latency | < 100 ns | R-15.7.3 |

### TC-15.7.5.B1 Policy Signature Verification

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Ed25519 `verify(policy, signature, key)` | Latency | < 5 ms | R-15.7.5 |

### TC-15.9.2.B1 Tool Definition Lookup

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `ToolInterface::all_definitions()` with 200 tools | Latency | < 1 us | R-15.9.2 |

### TC-15.9.2.B2 Tool Parameter Validation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `validate(call)` with 10 parameters | Latency | < 50 us | R-15.9.2 |

### TC-15.9.8.B1 Accessibility Tree Query

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full accessibility snapshot with 500 nodes | Latency | < 5 ms | R-15.9.8 |

### TC-15.9.10.B1 Rate Limiter Acquire Permit

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `acquire_permit(user_id)` | Latency | < 500 ns | R-15.9.10 |

### TC-15.9.9.B1 Context Prompt Build

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `build_prompt()` with all modalities | Latency | < 2 ms | R-15.9.9 |
