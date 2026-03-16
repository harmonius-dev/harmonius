# Localization and Documentation Test Cases

Companion test cases for [localization-docs.md](localization-docs.md).

## Unit Tests

### TC-15.13.1.1 ICU Plural Selection (English)

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `PluralRules("en-US").select(0.0)` | `PluralCategory::Other` | R-15.13.1 |
| 2 | `PluralRules("en-US").select(1.0)` | `PluralCategory::One` | R-15.13.1 |
| 3 | `PluralRules("en-US").select(2.0)` | `PluralCategory::Other` | R-15.13.1 |
| 4 | `PluralRules("en-US").select(99.0)` | `PluralCategory::Other` | R-15.13.1 |

### TC-15.13.1.2 ICU Plural Selection (Arabic)

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `PluralRules("ar-SA").select(0.0)` | `PluralCategory::Zero` | R-15.13.1 |
| 2 | `PluralRules("ar-SA").select(1.0)` | `PluralCategory::One` | R-15.13.1 |
| 3 | `PluralRules("ar-SA").select(2.0)` | `PluralCategory::Two` | R-15.13.1 |
| 4 | `PluralRules("ar-SA").select(5.0)` | `PluralCategory::Few` | R-15.13.1 |
| 5 | `PluralRules("ar-SA").select(11.0)` | `PluralCategory::Many` | R-15.13.1 |
| 6 | `PluralRules("ar-SA").select(100.0)` | `PluralCategory::Other` | R-15.13.1 |

### TC-15.13.1.3 ICU Pattern Validation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `validate_pattern("{count, plural, one {# item} other {# items}}")` | `Ok(())` | R-15.13.1 |
| 2 | `validate_pattern("{count, plural, one {# item}}")` | `Err(MissingOtherCategory)` | R-15.13.1 |
| 3 | `validate_pattern("{count, plural, one {# item} other {# items}")` | `Err(UnmatchedBrace { position: _ })` | R-15.13.1 |
| 4 | `validate_pattern("{count, badtype, ...}")` | `Err(InvalidArgType { arg: "badtype" })` | R-15.13.1 |

### TC-15.13.1.4 ICU Format with Arguments

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | pattern=`"Hello {name}"`, args=`{name: String("World")}` | `"Hello World"` | R-15.13.1 |
| 2 | pattern=`"{count, plural, one {# item} other {# items}}"`, args=`{count: Number(1.0)}` | `"1 item"` | R-15.13.1 |
| 3 | pattern=`"{count, plural, one {# item} other {# items}}"`, args=`{count: Number(5.0)}` | `"5 items"` | R-15.13.1 |
| 4 | pattern=`"Hello {name}"`, args=`{}` | `Err(MissingArgument { name: "name" })` | R-15.13.1 |

### TC-15.13.1.5 ICU Variable Extraction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `extract_variables("Hello {name}, you have {count} items")` | `["name", "count"]` | R-15.13.1 |
| 2 | `extract_variables("No variables here")` | `[]` | R-15.13.1 |
| 3 | `extract_variables("{a} {b} {a}")` | `["a", "b"]` (deduplicated) | R-15.13.1 |

### TC-15.13.3.1 Locked Entry Editing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `model.lock("key1")` then `model.set_translation("key1", "fr", "...")` | `Err(EntryLocked { key: "key1" })` | R-15.13.3 |
| 2 | `model.lock("key1")` then `model.unlock("key1")` then `model.set_translation("key1", "fr", "Bonjour")` | `Ok(())`, translation = `"Bonjour"` | R-15.13.3 |
| 3 | `model.lock("nonexistent")` | `Err(KeyNotFound { key: "nonexistent" })` | R-15.13.3 |

### TC-15.13.3.2 Source Change Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert key with source `"Hello"`, export, change source to `"Hi"`, call `detect_source_changes()` | returns `1`, entry `source_dirty == true` | R-15.13.3 |
| 2 | Insert key with source `"Hello"`, export, no change, call `detect_source_changes()` | returns `0`, entry `source_dirty == false` | R-15.13.3 |

### TC-15.13.1.6 Translation Memory Fuzzy Match

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Index 100 pairs, query exact source string | first suggestion `similarity == 1.0` | R-15.13.1 |
| 2 | Index `"Save game"`, query `"Save the game"` | suggestion with `similarity > 0.6` | R-15.13.1 |
| 3 | Query completely unrelated string `"XYZ123"` | empty suggestions or all `similarity < 0.6` | R-15.13.1 |

### TC-15.13.3.3 XLIFF Round-Trip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Encode model with 10 entries to XLIFF, decode result | 10 `TransUnit` entries, sources match originals | R-15.13.3 |
| 2 | Decode malformed XML bytes `b"<not valid"` | `Err(MalformedXml { line: 1, ... })` | R-15.13.3 |
| 3 | Encode model, verify XLIFF version attribute = `"2.1"` | version attribute present and correct | R-15.13.3 |

### TC-15.13.1.7 CSV Round-Trip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Encode model with 5 entries and 3 locales, decode result | 5 `CsvRow` entries, all translations preserved | R-15.13.1 |
| 2 | Encode entry with empty translation for `"fr"`, decode | empty string preserved for `"fr"` column | R-15.13.1 |
| 3 | Decode CSV missing `"key"` header column | `Err(MissingHeader { column: "key" })` | R-15.13.1 |

### TC-15.13.2.1 Pseudo-Localization Transforms

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `transform("Hello", PseudoConfig { accented: true, .. })` | no plain ASCII `a-z` in output | R-15.13.2 |
| 2 | `transform("Hello", PseudoConfig { padded: true, .. })` | `output.len() >= input.len() * 1.3` | R-15.13.2 |
| 3 | `transform("Hello", PseudoConfig { bracketed: true, .. })` | output starts with `[` and ends with `]` | R-15.13.2 |

### TC-15.13.2.2 Locale Validation -- Missing Translations

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Model with 10 entries, `"fr"` has 5 translations | `check_missing` returns 5 findings with `MissingTranslation` | R-15.13.2 |
| 2 | Model with 10 entries, `"fr"` has 10 translations | `check_missing` returns 0 findings | R-15.13.2 |

### TC-15.13.2.3 Locale Validation -- Text Overflow

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | German translation 2x source length, widget width = 100px | finding with `TextOverflow` category | R-15.13.2 |
| 2 | Short translation fits widget | no `TextOverflow` findings | R-15.13.2 |

### TC-15.13.2.4 Locale Validation -- RTL Layout

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Arabic text in LTR-only widget | finding with `RtlLayoutIssue` | R-15.13.2 |
| 2 | Arabic text in bidi-capable widget | no `RtlLayoutIssue` findings | R-15.13.2 |

### TC-15.13.2.5 Locale Validation -- Interpolation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Source has `{name}`, translation missing `{name}` | finding with `BrokenInterpolation` | R-15.13.2 |
| 2 | Source has `{name}`, translation has `{name}` | no `BrokenInterpolation` findings | R-15.13.2 |

### TC-15.13.2.6 Bidi Algorithm Resolution

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `resolve_bidi("Hello \u{0627}\u{0644}")` (mixed LTR/RTL) | 2 `BidiRun` entries with correct directions | R-15.13.2 |
| 2 | `resolve_bidi("Hello World")` (pure LTR) | 1 `BidiRun` with `LeftToRight` | R-15.13.2 |

### TC-15.13.1.8 Font Fallback Resolution

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `resolve(0x4E00)` (CJK ideograph), chain has CJK font | returns CJK font `AssetId` | R-15.13.1 |
| 2 | `resolve(0x41)` (Latin 'A'), chain has Latin font first | returns Latin font `AssetId` | R-15.13.1 |
| 3 | `shape_runs("Hello\u{4E16}\u{754C}")` (mixed Latin/CJK) | 2 `FontRun` entries | R-15.13.1 |

### TC-15.19.1.1 Reflect Metadata Extraction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register type with 3 fields and 2 methods, `extract()` | `ApiEntry` with 3 `FieldDoc` and 2 `MethodDoc` | R-15.19.1 |
| 2 | Register type with doc comment, `extract()` | `ApiEntry.doc_comment == Some("...")` | R-15.19.1 |
| 3 | `extract("NonExistentType")` | `None` | R-15.19.1 |

### TC-15.19.1.2 Search Index Query

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Index 100 types, `query("Transform", 10)` | results contain type named `"Transform"` | R-15.19.1 |
| 2 | `query("zzzznonexistent", 10)` | empty results | R-15.19.1 |

### TC-15.19.1.3 Versioned Doc Store

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Store pages for v1.0.0 and v1.1.0, fetch from v1.0.0 | correct page content for v1.0.0 | R-15.19.1 |
| 2 | Fetch page from unstored v2.0.0 | `None` | R-15.19.1 |

### TC-15.19.2.1 Node Documentation Completeness

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `NodeDoc` with empty `description` | finding reported for missing description | R-15.19.2 |
| 2 | `NodeDoc` with empty `examples` | finding reported for missing examples | R-15.19.2 |
| 3 | Complete `NodeDoc` with description, ports, examples | no findings | R-15.19.2 |

### TC-15.19.3.1 Tutorial Step Advancement

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Start tutorial with 3 steps, fire completion event for step 1, call `try_advance()` | returns `true`, `progress() == (1, 3)` | R-15.19.3 |
| 2 | Call `try_advance()` without completing condition | returns `false`, step unchanged | R-15.19.3 |

### TC-15.19.3.2 Tutorial Pause and Resume

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Start tutorial at step 1, `pause()`, `resume()` | `state() == Running`, `progress() == (0, N)` unchanged | R-15.19.3 |
| 2 | `pause()` | `state() == Paused` | R-15.19.3 |

### TC-15.19.5.1 Contextual Help Lookup

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `lookup("TransformInspector", "en-US")` | `Some(HelpEntry)` with correct `short_desc` | R-15.19.5 |
| 2 | `lookup("TransformInspector", "ja-JP")` | `Some(HelpEntry)` with Japanese locale content | R-15.19.5 |
| 3 | `lookup("NonExistentWidget", "en-US")` | `None` | R-15.19.5 |

## Integration Tests

### TC-15.13.2.I1 Full Locale Switch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set active locale from `"en-US"` to `"ar-SA"` | all widget text re-rendered, RTL layout applied | R-15.13.2 |
| 2 | Set active locale from `"en-US"` to `"ja-JP"` | CJK font fallback chain active, text renders correctly | R-15.13.2 |

### TC-15.13.2.I2 Pseudo-Localization Full UI

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enable pseudo-locale with `accented + bracketed` | no plain ASCII `a-z` in any rendered UI string | R-15.13.2 |

### TC-15.13.3.I1 XLIFF TMS Compatibility

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Export XLIFF 2.1, import into TMS simulator, re-export, import back | all 5000 strings preserved with correct states | R-15.13.3 |

### TC-15.19.1.I1 Full API Reference Generation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Run `generate()` against full `TypeRegistry` | `DocArtifact.page_count > 0`, all registered types have pages | R-15.19.1 |

### TC-15.19.1.I2 Help Panel Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Open help panel, measure time to first content | latency < 500 ms | R-15.19.1 |

### TC-15.19.5.I1 Tooltip Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hover over widget, measure time to tooltip display | latency < 200 ms | R-15.19.5 |

### TC-15.19.3.I1 Tutorial End-to-End

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Run GettingStarted tutorial, complete all steps | `state() == Complete`, all steps visited | R-15.19.3 |

### TC-15.19.7.I1 Doc-Test Compilation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Extract all inline code examples, compile each | all examples compile without errors | R-15.19.7 |

## Benchmarks

### TC-15.13.1.B1 ICU Format Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Format simple pattern with 1 arg | latency | < 5 us | R-15.13.1 |
| 2 | Format plural pattern with nested select | latency | < 10 us | R-15.13.1 |

### TC-15.13.1.B2 Translation Memory Search

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Fuzzy search across 10k TM entries | latency | < 10 ms | R-15.13.1 |

### TC-15.13.3.B1 XLIFF Export

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Export 5000 strings to XLIFF 2.1 | latency | < 500 ms | R-15.13.3 |

### TC-15.13.2.B1 Validation Pass

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full validation of 5000 strings for one locale | latency | < 2 s | R-15.13.2 |

### TC-15.19.1.B1 Search Index Query

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Trigram query against full engine API index | latency | < 5 ms | R-15.19.1 |

### TC-15.19.5.B1 Help Tooltip Lookup

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Lookup help entry by widget type ID | latency | < 200 ms | R-15.19.5 |

### TC-15.19.1.B2 Help Panel Open

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Open help panel and render first page | latency | < 500 ms | R-15.19.1 |

### TC-15.19.1.B3 API Reference Generation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Generate full API reference for entire engine | latency | < 60 s | R-15.19.1 |
