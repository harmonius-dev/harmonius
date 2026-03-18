# AI Governance and Assistant Test Cases

Companion test cases for [ai-governance.md](ai-governance.md).

## Unit Tests

### TC-15.7.1.1 Provenance Tag Attach and Query

| # | Requirement |
|---|-------------|
| 1 | R-15.7.1    |
| 2 | R-15.7.1    |
| 3 | R-15.7.1    |

1. **#1** — `tag_asset(asset_42, TextureGenerator, "sd-xl-1.0", prompt_hash)`
   - **Expected:**
     `ProvenanceTag { asset_id: 42, ai_system: TextureGenerator, model_version: "sd-xl-1.0" }`
     stored
2. **#2** — `get_provenance(asset_42)`
   - **Expected:** Returns tag matching fields from step 1
3. **#3** — `get_provenance(asset_99)` (untagged)
   - **Expected:** Returns `None`

### TC-15.7.1.2 Provenance Survives Serialization

| # | Requirement |
|---|-------------|
| 1 | R-15.7.1    |
| 2 | R-15.7.1    |

1. **#1** — Tag asset, serialize to bytes, deserialize
   - **Expected:** Deserialized tag matches original fields
2. **#2** — Tag asset, save to disk, reload
   - **Expected:** Reloaded tag matches original fields

### TC-15.7.1.3 Provenance Removal on Full Replace

| # | Requirement        |
|---|--------------------|
| 1 | R-15.7.1, R-15.7.2 |
| 2 | R-15.7.1, R-15.7.2 |

1. **#1** — Tag asset with 100 regions, mark all 100 modified
   - **Expected:** `get_provenance(asset)` returns `None`
2. **#2** — Tag asset with 100 regions, mark 99 modified
   - **Expected:** `get_provenance(asset)` returns `Some(tag)`

### TC-15.7.2.1 Modification Bitmask Granularity

| # | Requirement |
|---|-------------|
| 1 | R-15.7.2    |
| 2 | R-15.7.2    |
| 3 | R-15.7.2    |
| 4 | R-15.7.2    |

1. **#1** — `start_tracking(asset, VertexGroup)`, mark region 0
   - **Expected:** `bitmask.regions[0] == true`, rest false
2. **#2** — `start_tracking(asset, TextureTile(256))`, mark tile 5
   - **Expected:** `bitmask.regions[5] == true`, rest false
3. **#3** — `start_tracking(asset, DataTableRow)`, mark row 10
   - **Expected:** `bitmask.regions[10] == true`, rest false
4. **#4** — `start_tracking(asset, LogicGraphNode)`, mark node 3
   - **Expected:** `bitmask.regions[3] == true`, rest false

### TC-15.7.2.2 Bitmask Size Under 1 KB

| # | Requirement |
|---|-------------|
| 1 | R-15.7.2    |
| 2 | R-15.7.2    |

1. **#1** — Bitmask with 1000 regions
   - **Expected:** `size_of(bitmask) < 1024` bytes
2. **#2** — Bitmask with 8000 regions
   - **Expected:** `size_of(bitmask) < 1024` bytes

### TC-15.7.2.3 Modification Percentage Calculation

| # | Requirement |
|---|-------------|
| 1 | R-15.7.2    |
| 2 | R-15.7.2    |
| 3 | R-15.7.2    |

1. **#1** — 100 regions, 50 marked modified
   - **Expected:** `modification_percentage() == 0.50`
2. **#2** — 100 regions, 0 marked modified
   - **Expected:** `modification_percentage() == 0.0`
3. **#3** — 100 regions, 100 marked modified
   - **Expected:** `modification_percentage() == 1.0`

### TC-15.7.3.1 Generative Toggle Disables LLM

| # | Requirement |
|---|-------------|
| 1 | R-15.7.3    |
| 2 | R-15.7.3    |

1. **#1** — `generative_ai_enabled = false`, call LLM generation
   - **Expected:** Returns `Err(FeatureDisabled)`
2. **#2** — `generative_ai_enabled = true`, call LLM generation
   - **Expected:** Returns `Ok(...)`

### TC-15.7.3.2 Deterministic AI Unaffected

| # | Requirement |
|---|-------------|
| 1 | R-15.7.3    |
| 2 | R-15.7.3    |
| 3 | R-15.7.3    |

1. **#1** — `generative_ai_enabled = false`, run pathfinding
   - **Expected:** Pathfinding returns valid path
2. **#2** — `generative_ai_enabled = false`, run behavior tree
   - **Expected:** BT executes normally
3. **#3** — `generative_ai_enabled = false`, run GOAP planner
   - **Expected:** GOAP returns valid plan

### TC-15.7.4.1 Toggle Independence

| # | Requirement |
|---|-------------|
| 1 | R-15.7.4    |
| 2 | R-15.7.4    |
| 3 | R-15.7.4    |
| 4 | R-15.7.4    |

1. **#1** — `generative=true, assistance=true`
   - **Expected:** Both LLM generation and assistant available
2. **#2** — `generative=true, assistance=false`
   - **Expected:** LLM generation available, assistant disabled
3. **#3** — `generative=false, assistance=true`
   - **Expected:** Assistant available, LLM generation disabled
4. **#4** — `generative=false, assistance=false`
   - **Expected:** Both disabled

### TC-15.7.4.2 Toggle Persistence

| # | Requirement |
|---|-------------|
| 1 | R-15.7.4    |

1. **#1** — Set toggles `(true, false, true, false)`, save, reload
   - **Expected:** Reloaded toggles match `(true, false, true, false)`

### TC-15.7.5.1 Ed25519 Signature Verification

| # | Requirement |
|---|-------------|
| 1 | R-15.7.5    |
| 2 | R-15.7.5    |
| 3 | R-15.7.5    |

1. **#1** — Sign policy with key A, verify with key A
   - **Expected:** Verification succeeds
2. **#2** — Sign policy with key A, tamper 1 byte, verify with key A
   - **Expected:** Verification fails with `InvalidSignature`
3. **#3** — Sign policy with key A, verify with key B
   - **Expected:** Verification fails with `UntrustedKey`

### TC-15.7.5.2 Stale Policy Rejected

| # | Requirement |
|---|-------------|
| 1 | R-15.7.5    |
| 2 | R-15.7.5    |
| 3 | R-15.7.5    |

1. **#1** — Current policy version 5, receive version 4
   - **Expected:** `Err(StaleVersion { current: 5, received: 4 })`
2. **#2** — Current policy version 5, receive version 5
   - **Expected:** `Err(StaleVersion { current: 5, received: 5 })`
3. **#3** — Current policy version 5, receive version 6
   - **Expected:** Policy applied successfully

### TC-15.7.6.1 Audit Hash Chain

| # | Requirement |
|---|-------------|
| 1 | R-15.7.6    |
| 2 | R-15.7.6    |

1. **#1** — Append 100 entries, `validate_chain()`
   - **Expected:** Returns `Ok(true)`
2. **#2** — Append 100 entries, tamper entry 50, `validate_chain()`
   - **Expected:** Returns `Ok(false)`, chain breaks at entry 50

### TC-15.7.6.2 Audit Rotation Preserves Chain

| # | Requirement |
|---|-------------|
| 1 | R-15.7.6    |

1. **#1** — Append 1000 entries, rotate at 500, `validate_chain()`
   - **Expected:** Returns `Ok(true)` across both segments

### TC-15.7.7.1 Review Auto-Approve Threshold

| # | Requirement |
|---|-------------|
| 1 | R-15.7.7    |
| 2 | R-15.7.7    |

1. **#1** — Threshold 80%, mark 80% modified
   - **Expected:** Asset status changes to approved
2. **#2** — Threshold 80%, mark 79% modified
   - **Expected:** Asset status remains pending

### TC-15.7.7.2 Review Blocks Unapproved

| # | Requirement |
|---|-------------|
| 1 | R-15.7.7    |
| 2 | R-15.7.7    |

1. **#1** — AI asset with `require_review = true`, not reviewed
   - **Expected:** Asset not in production database
2. **#2** — AI asset with `require_review = true`, approved
   - **Expected:** Asset in production database

### TC-15.7.8.1 Packaged Provenance Minimal

| # | Requirement |
|---|-------------|
| 1 | R-15.7.8    |
| 2 | R-15.7.8    |

1. **#1** — Package asset with full provenance tag
   - **Expected:** Packaged output contains only `ai_system` and `is_ai_generated` flag
2. **#2** — Package asset without provenance
   - **Expected:** Packaged output has no provenance metadata

### TC-15.9.10.1 Rate Limiter Token Bucket

| # | Requirement |
|---|-------------|
| 1 | R-15.9.10   |
| 2 | R-15.9.10   |
| 3 | R-15.9.10   |

1. **#1** — Bucket with 10 tokens, acquire 10 permits
   - **Expected:** All succeed
2. **#2** — Bucket with 10 tokens, acquire 11th permit
   - **Expected:** Returns `Err(RateLimited { ... })`
3. **#3** — Wait for refill period, acquire permit
   - **Expected:** Succeeds

### TC-15.9.10.2 Quota Requests Per Hour

| # | Requirement |
|---|-------------|
| 1 | R-15.9.10   |
| 2 | R-15.9.10   |

1. **#1** — Quota `requests_per_hour = 100`, use 100 requests
   - **Expected:** 100th succeeds
2. **#2** — Quota `requests_per_hour = 100`, use 101st request
   - **Expected:** Returns `Err(QuotaExceeded { limit: "requests_per_hour" })`

### TC-15.9.10.3 Quota Tokens Per Day

| # | Requirement |
|---|-------------|
| 1 | R-15.9.10   |
| 2 | R-15.9.10   |

1. **#1** — Quota `tokens_per_day = 50000`, record 50000 tokens
   - **Expected:** Succeeds
2. **#2** — Quota `tokens_per_day = 50000`, record 50001 tokens
   - **Expected:** Returns `Err(QuotaExceeded { limit: "tokens_per_day" })`

### TC-15.9.10.4 Tool Allowlist

| # | Requirement |
|---|-------------|
| 1 | R-15.9.10   |
| 2 | R-15.9.10   |

1. **#1** — Allowlist `{tool_A, tool_B}`, check `tool_A`
   - **Expected:** `is_allowed` returns `true`
2. **#2** — Allowlist `{tool_A, tool_B}`, check `tool_C`
   - **Expected:** `is_allowed` returns `false`

### TC-15.9.10.5 Tool Blocklist

| # | Requirement |
|---|-------------|
| 1 | R-15.9.10   |
| 2 | R-15.9.10   |

1. **#1** — Blocklist `{tool_X}`, check `tool_X`
   - **Expected:** `is_allowed` returns `false`
2. **#2** — Blocklist `{tool_X}`, check `tool_Y`
   - **Expected:** `is_allowed` returns `true`

### TC-15.9.2.1 Tool Validation

| # | Requirement |
|---|-------------|
| 1 | R-15.9.2    |
| 2 | R-15.9.2    |
| 3 | R-15.9.2    |

1. **#1** — Tool expects `Range { min: 0, max: 100 }`, pass value 50
   - **Expected:** Validation passes
2. **#2** — Tool expects `Range { min: 0, max: 100 }`, pass value 150
   - **Expected:** Validation fails with error message
3. **#3** — Tool expects `Required { field: "name" }`, omit `name`
   - **Expected:** Validation fails with error message

### TC-15.9.2.2 Tool Undo Integration

| # | Requirement |
|---|-------------|
| 1 | R-15.9.2    |
| 2 | R-15.9.2    |

1. **#1** — Execute tool with `supports_undo = true`
   - **Expected:** `ToolResult::Success { undo_command: Some(...) }`
2. **#2** — Execute tool with `supports_undo = false`
   - **Expected:** `ToolResult::Success { undo_command: None }`

### TC-15.9.6b.1 Agent Isolation

| # | Requirement |
|---|-------------|
| 1 | R-15.9.6b   |
| 2 | R-15.9.6b   |

1. **#1** — Create agent A and B, select entity 1 in A
   - **Expected:** Agent B selection is empty
2. **#2** — Create agent A and B, modify undo stack in A
   - **Expected:** Agent B undo stack unchanged

### TC-15.9.6b.2 Agent Independent Undo

| # | Requirement |
|---|-------------|
| 1 | R-15.9.6b   |

1. **#1** — Agent A executes tool, agent B executes tool, undo in A
   - **Expected:** Agent A reverted, agent B unchanged

### TC-15.9.9.1 Context Build Precedence

| # | Requirement |
|---|-------------|
| 1 | R-15.9.9    |
| 2 | R-15.9.9    |

1. **#1** — Context with both accessibility snapshot and screenshot
   - **Expected:** Prompt contains structured data before pixel data
2. **#2** — Context with only text input
   - **Expected:** Prompt contains text, no image tokens

## Integration Tests

### TC-15.7.1.I1 Provenance Pipeline Survival

| # | Requirement |
|---|-------------|
| 1 | R-15.7.1    |

1. **#1** — Tag asset, run import/cook/package pipeline
   - **Expected:** Packaged output contains provenance metadata

### TC-15.7.5.I1 Policy Push via WebSocket

| # | Requirement |
|---|-------------|
| 1 | R-15.7.5    |

1. **#1** — Mock admin server pushes signed policy via WebSocket
   - **Expected:** Editor applies new toggle state within 5 seconds

### TC-15.7.5.I2 Policy Pull via HTTPS

| # | Requirement |
|---|-------------|
| 1 | R-15.7.5    |

1. **#1** — Mock admin server hosts signed policy, editor polls
   - **Expected:** Editor applies new toggle state on next poll cycle

### TC-15.7.6.I1 Audit Replication

| # | Requirement |
|---|-------------|
| 1 | R-15.7.6    |

1. **#1** — Append 500 entries locally, `replicate_to(mock_server)`
   - **Expected:** All 500 entries received by mock server

### TC-15.7.7.I1 Review Workflow End to End

| # | Requirement |
|---|-------------|
| 1 | R-15.7.7    |
| 2 | R-15.7.7    |

1. **#1** — Generate AI asset, route to review, approve
   - **Expected:** Asset present in production database
2. **#2** — Generate AI asset, route to review, reject
   - **Expected:** Asset not in production database, reason recorded

### TC-15.9.2.I1 LLM Request Async

| # | Requirement |
|---|-------------|
| 1 | R-15.9.2    |

1. **#1** — Send request to mock LLM API via IoReactor
   - **Expected:** Response parsed, tool calls extracted correctly

### TC-15.9.1b.I1 Voice Command Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-15.9.1b   |

1. **#1** — Transcript "select the red cube", mock LLM returns `select_entity` tool call
   - **Expected:** Entity selection tool invoked with correct parameters

### TC-15.9.6a.I1 Headless Tool Execution

| # | Requirement |
|---|-------------|
| 1 | R-15.9.6a   |

1. **#1** — Execute `set_property` tool via headless API
   - **Expected:** Property value matches interactive execution result

### TC-15.9.6c.I1 CI Agent Artifact Production

| # | Requirement |
|---|-------------|
| 1 | R-15.9.6c   |

1. **#1** — Run agent in headless mode with build task
   - **Expected:** Build artifact produced at expected path

### TC-15.9.10.I1 Offline Then Reconnect

| # | Requirement |
|---|-------------|
| 1 | R-15.9.10   |

1. **#1** — Operate offline for 10 interactions, reconnect
   - **Expected:** All 10 local audit entries replicated to server

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
