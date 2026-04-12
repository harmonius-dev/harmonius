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
| TC-15.7.8.1 | R-15.7.8 | Provenance metadata in build | package build with 3 AI-tagged assets | build manifest contains provenance entries for all 3 |
| TC-15.19.4.1 | R-15.19.4 | Video chapter seek | tutorial video with 5 chapters, seek to chapter 3 | playback position at chapter 3 start timestamp |
| TC-15.19.6.1 | R-15.19.6 | Sample project instantiate | `instantiate_sample("platformer")` | project folder copied and registered |
| TC-15.19.7.1 | R-15.19.7 | Doc-test extraction | doc comment with 2 code examples | 2 `DocTest` entries extracted |
| TC-15.9.1a.1 | R-15.9.1a | STT transcribe fixed audio | 3 s WAV of "save scene" | transcript text equals "save scene" |
| TC-15.9.1b.1 | R-15.9.1b | Command interpretation | text "place a red cube at origin" | `ToolCall("place_mesh", {color: red, pos: [0,0,0]})` |
| TC-15.9.1c.1 | R-15.9.1c | Push-to-talk vs wake word | configure `VoiceActivation::PushToTalk`, press and release | STT active only while pressed |
| TC-15.9.3.1 | R-15.9.3 | Visual tool palette access | LLM issues `open_panel("NodeGraph")` | node graph panel visible and focused |
| TC-15.9.5.1 | R-15.9.5 | Contextual action reminder | user idles on paint tool 60 s | reminder suggests "try symmetry mode" once |
| TC-15.9.6a.1 | R-15.9.6a | Headless editor open scene | `headless.open_scene("menu.scene")` | returns `SceneHandle`; no window required |
| TC-15.9.6b.1 | R-15.9.6b | Agent orchestration plan | goal `"add health bar"`, agent plans | plan contains ordered tool calls ending in scene save |
| TC-15.9.6c.1 | R-15.9.6c | CI agent run artifact | headless agent runs build task in CI | produces log + build artifact path |
| TC-15.9.7.1 | R-15.9.7 | Screenshot capture | `capture_screenshot(viewport)` | PNG byte vector non-empty with viewport dimensions |
| TC-15.9.8.1 | R-15.9.8 | Accessibility tree snapshot | query active editor panel | returns `AccessibleNode` tree rooted at panel |
| TC-15.19.4.2 | R-15.19.4 | Embedded video play/pause | load chapterized video, call `play()` then `pause()` | `VideoPlaybackState::Paused` on the second frame |
| TC-15.19.6.2 | R-15.19.6 | Template list enumerate | `list_templates()` with 4 registered templates | returns 4 entries with name + preview thumbnail handle |
| TC-15.19.7.2 | R-15.19.7 | Doc-test compile | `DocTest` block `let x: i32 = 1 + 1;` | compiles without error; value equals 2 |

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
| TC-15.13.1.I1 | US-15.13.1 | Visual string table ICU + TM workflow | author 20 strings with ICU plural, link TM | all strings validate; TM suggestions shown |
| TC-15.13.2.I1 | US-15.13.2 | Pseudo-loc preview and RTL toggle | enable pseudo-loc, switch preview locale to `ar` | pseudo text displayed, layout mirrored |
| TC-15.13.3.I1 | US-15.13.3 | XLIFF round-trip with TMS | export model to XLIFF, modify externally, import | model updated; string locks preserved |
| TC-15.19.1.I1 | US-15.19.1 | API ref live update after codegen | recompile middleman .dylib, regenerate docs | API ref reflects new types within 1 reload |
| TC-15.19.2.I1 | US-15.19.2 | Logic graph node docs visible | register 10 custom nodes, open node browser | tooltip shows port descriptions for each |
| TC-15.19.3.I1 | US-15.19.3 | Interactive tutorial progress saved | start tutorial, advance 5 steps, close editor, reopen | tutorial resumes at step 5 |
| TC-15.19.4.I1 | US-15.19.4 | Embedded video with chapter nav | open 10-chapter video doc, click TOC | playback jumps to chapter boundaries exactly |
| TC-15.19.5.I1 | US-15.19.5 | Contextual help tooltip | hover property inspector row | help tooltip appears within 200 ms with doc entry body |
| TC-15.19.6.I1 | US-15.19.6 | Sample project end-to-end | instantiate `"platformer"` sample, run | sample opens, scene plays, no errors |
| TC-15.19.7.I1 | US-15.19.7 | Doc-test harness runs inline examples | doc with 10 code examples | all 10 doc-tests pass |
| TC-15.7.1.I1 | US-15.7.1 | AI-tagged asset provenance flow | AI generates texture, tag applied, open inspector | inspector shows system, version, hash, user |
| TC-15.7.2.I1 | US-15.7.2 | Modification bitmask across edits | AI generates mesh, artist edits 30%, checks | bitmask shows 30% modified, timeline preserved |
| TC-15.7.3.I1 | US-15.7.3 | Generative AI global toggle | toggle off, attempt AI generation | `Err(GenerativeDisabled)`; UI shows disabled state |
| TC-15.7.4.I1 | US-15.7.4 | AI assistant toggle during session | disable assistant, send command | `Err(AssistantError::Disabled)`; UI hides assistant panel |
| TC-15.7.5.I1 | US-15.7.5 | Enterprise policy push via QUIC | admin publishes `PolicyDocument`, 5 editors connected | all 5 apply policy within 1 frame |
| TC-15.7.6.I1 | US-15.7.6 | Audit log forensic query | 1000 logged events, query by user + date | filtered results returned; chain validated |
| TC-15.7.7.I1 | US-15.7.7 | Review queue approve path | AI asset routed, reviewer approves | asset `ReviewStatus::Approved`; removed from queue |
| TC-15.7.8.I1 | US-15.7.8 | Packaged build metadata | build with 20 AI assets, inspect manifest | all 20 provenance entries present; signed |
| TC-15.9.1.I1 | US-15.9.1 | Voice command end-to-end | speak "save scene" via STT pipeline | scene saved; transcript logged |
| TC-15.9.2.I1 | US-15.9.2 | LLM tool call execution | LLM receives command, calls registered tool | tool executes; result shown to user |
| TC-15.9.3.I1 | US-15.9.3 | Visual tool palette invocation | LLM opens 3 panels in sequence | panels visible in declared order; no flicker |
| TC-15.9.4.I1 | US-15.9.4 | Shortcut recommendation surfaced | user performs 10 repetitive actions | assistant surfaces shortcut suggestion once |
| TC-15.9.5.I1 | US-15.9.5 | Context-aware action reminder | user idles on tool 60 s | reminder toast shown; dismissable |
| TC-15.9.6.I1 | US-15.9.6 | Headless agent multi-step task | agent receives `"add health bar"` | plan executes end-to-end; scene saved |
| TC-15.9.7.I1 | US-15.9.7 | Screenshot shared with LLM | capture + attach to prompt | `LlmRequest` includes image modality |
| TC-15.9.8.I1 | US-15.9.8 | Accessibility tree analysis | LLM queries a11y tree of focused panel | tree returned; used to answer "what's selected?" |
| TC-15.9.9.I1 | US-15.9.9 | Multi-modal context (text + screenshot + a11y) | assemble full context for prompt | `LlmRequest` has 3 modalities, total tokens under budget |
| TC-15.9.10.I1 | US-15.9.10 | Administration rate-limit enforcement | set 100 req/min limit, simulate 200 req | first 100 succeed, next 100 return `RateLimitExceeded` |

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
