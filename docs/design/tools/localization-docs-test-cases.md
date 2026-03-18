# Localization and Documentation Test Cases

Companion test cases for [localization-docs.md](localization-docs.md).

## Unit Tests

### TC-15.13.1.1 ICU Plural Selection (English)

| # | Requirement |
|---|-------------|
| 1 | R-15.13.1   |
| 2 | R-15.13.1   |
| 3 | R-15.13.1   |
| 4 | R-15.13.1   |

1. **#1** — `PluralRules("en-US").select(0.0)`
   - **Expected:** `PluralCategory::Other`
2. **#2** — `PluralRules("en-US").select(1.0)`
   - **Expected:** `PluralCategory::One`
3. **#3** — `PluralRules("en-US").select(2.0)`
   - **Expected:** `PluralCategory::Other`
4. **#4** — `PluralRules("en-US").select(99.0)`
   - **Expected:** `PluralCategory::Other`

### TC-15.13.1.2 ICU Plural Selection (Arabic)

| # | Requirement |
|---|-------------|
| 1 | R-15.13.1   |
| 2 | R-15.13.1   |
| 3 | R-15.13.1   |
| 4 | R-15.13.1   |
| 5 | R-15.13.1   |
| 6 | R-15.13.1   |

1. **#1** — `PluralRules("ar-SA").select(0.0)`
   - **Expected:** `PluralCategory::Zero`
2. **#2** — `PluralRules("ar-SA").select(1.0)`
   - **Expected:** `PluralCategory::One`
3. **#3** — `PluralRules("ar-SA").select(2.0)`
   - **Expected:** `PluralCategory::Two`
4. **#4** — `PluralRules("ar-SA").select(5.0)`
   - **Expected:** `PluralCategory::Few`
5. **#5** — `PluralRules("ar-SA").select(11.0)`
   - **Expected:** `PluralCategory::Many`
6. **#6** — `PluralRules("ar-SA").select(100.0)`
   - **Expected:** `PluralCategory::Other`

### TC-15.13.1.3 ICU Pattern Validation

| # | Requirement |
|---|-------------|
| 1 | R-15.13.1   |
| 2 | R-15.13.1   |
| 3 | R-15.13.1   |
| 4 | R-15.13.1   |

1. **#1** — `validate_pattern("{count, plural, one {# item} other {# items}}")`
   - **Expected:** `Ok(())`
2. **#2** — `validate_pattern("{count, plural, one {# item}}")`
   - **Expected:** `Err(MissingOtherCategory)`
3. **#3** — `validate_pattern("{count, plural, one {# item} other {# items}")`
   - **Expected:** `Err(UnmatchedBrace { position: _ })`
4. **#4** — `validate_pattern("{count, badtype, ...}")`
   - **Expected:** `Err(InvalidArgType { arg: "badtype" })`

### TC-15.13.1.4 ICU Format with Arguments

| # | Requirement |
|---|-------------|
| 1 | R-15.13.1   |
| 2 | R-15.13.1   |
| 3 | R-15.13.1   |
| 4 | R-15.13.1   |

1. **#1** — pattern=`"Hello {name}"`, args=`{name: String("World")}`
   - **Expected:** `"Hello World"`
2. **#2** — pattern=`"{count, plural, one {# item} other {# items}}"`, args=`{count: Number(1.0)}`
   - **Expected:** `"1 item"`
3. **#3** — pattern=`"{count, plural, one {# item} other {# items}}"`, args=`{count: Number(5.0)}`
   - **Expected:** `"5 items"`
4. **#4** — pattern=`"Hello {name}"`, args=`{}`
   - **Expected:** `Err(MissingArgument { name: "name" })`

### TC-15.13.1.5 ICU Variable Extraction

| # | Requirement |
|---|-------------|
| 1 | R-15.13.1   |
| 2 | R-15.13.1   |
| 3 | R-15.13.1   |

1. **#1** — `extract_variables("Hello {name}, you have {count} items")`
   - **Expected:** `["name", "count"]`
2. **#2** — `extract_variables("No variables here")`
   - **Expected:** `[]`
3. **#3** — `extract_variables("{a} {b} {a}")`
   - **Expected:** `["a", "b"]` (deduplicated)

### TC-15.13.3.1 Locked Entry Editing

| # | Requirement |
|---|-------------|
| 1 | R-15.13.3   |
| 2 | R-15.13.3   |
| 3 | R-15.13.3   |

1. **#1** — `model.lock("key1")` then `model.set_translation("key1", "fr", "...")`
   - **Expected:** `Err(EntryLocked { key: "key1" })`
2. **#2** — `model.lock("key1")` then `model.unlock("key1")` then
   `model.set_translation("key1", "fr", "Bonjour")`
   - **Expected:** `Ok(())`, translation = `"Bonjour"`
3. **#3** — `model.lock("nonexistent")`
   - **Expected:** `Err(KeyNotFound { key: "nonexistent" })`

### TC-15.13.3.2 Source Change Detection

| # | Requirement |
|---|-------------|
| 1 | R-15.13.3   |
| 2 | R-15.13.3   |

1. **#1** — Insert key with source `"Hello"`, export, change source to `"Hi"`, call
   `detect_source_changes()`
   - **Expected:** returns `1`, entry `source_dirty == true`
2. **#2** — Insert key with source `"Hello"`, export, no change, call `detect_source_changes()`
   - **Expected:** returns `0`, entry `source_dirty == false`

### TC-15.13.1.6 Translation Memory Fuzzy Match

| # | Requirement |
|---|-------------|
| 1 | R-15.13.1   |
| 2 | R-15.13.1   |
| 3 | R-15.13.1   |

1. **#1** — Index 100 pairs, query exact source string
   - **Expected:** first suggestion `similarity == 1.0`
2. **#2** — Index `"Save game"`, query `"Save the game"`
   - **Expected:** suggestion with `similarity > 0.6`
3. **#3** — Query completely unrelated string `"XYZ123"`
   - **Expected:** empty suggestions or all `similarity < 0.6`

### TC-15.13.3.3 XLIFF Round-Trip

| # | Requirement |
|---|-------------|
| 1 | R-15.13.3   |
| 2 | R-15.13.3   |
| 3 | R-15.13.3   |

1. **#1** — Encode model with 10 entries to XLIFF, decode result
   - **Expected:** 10 `TransUnit` entries, sources match originals
2. **#2** — Decode malformed XML bytes `b"<not valid"`
   - **Expected:** `Err(MalformedXml { line: 1, ... })`
3. **#3** — Encode model, verify XLIFF version attribute = `"2.1"`
   - **Expected:** version attribute present and correct

### TC-15.13.1.7 CSV Round-Trip

| # | Requirement |
|---|-------------|
| 1 | R-15.13.1   |
| 2 | R-15.13.1   |
| 3 | R-15.13.1   |

1. **#1** — Encode model with 5 entries and 3 locales, decode result
   - **Expected:** 5 `CsvRow` entries, all translations preserved
2. **#2** — Encode entry with empty translation for `"fr"`, decode
   - **Expected:** empty string preserved for `"fr"` column
3. **#3** — Decode CSV missing `"key"` header column
   - **Expected:** `Err(MissingHeader { column: "key" })`

### TC-15.13.2.1 Pseudo-Localization Transforms

| # | Requirement |
|---|-------------|
| 1 | R-15.13.2   |
| 2 | R-15.13.2   |
| 3 | R-15.13.2   |

1. **#1** — `transform("Hello", PseudoConfig { accented: true, .. })`
   - **Expected:** no plain ASCII `a-z` in output
2. **#2** — `transform("Hello", PseudoConfig { padded: true, .. })`
   - **Expected:** `output.len() >= input.len() * 1.3`
3. **#3** — `transform("Hello", PseudoConfig { bracketed: true, .. })`
   - **Expected:** output starts with `[` and ends with `]`

### TC-15.13.2.2 Locale Validation -- Missing Translations

| # | Requirement |
|---|-------------|
| 1 | R-15.13.2   |
| 2 | R-15.13.2   |

1. **#1** — Model with 10 entries, `"fr"` has 5 translations
   - **Expected:** `check_missing` returns 5 findings with `MissingTranslation`
2. **#2** — Model with 10 entries, `"fr"` has 10 translations
   - **Expected:** `check_missing` returns 0 findings

### TC-15.13.2.3 Locale Validation -- Text Overflow

| # | Requirement |
|---|-------------|
| 1 | R-15.13.2   |
| 2 | R-15.13.2   |

1. **#1** — German translation 2x source length, widget width = 100px
   - **Expected:** finding with `TextOverflow` category
2. **#2** — Short translation fits widget
   - **Expected:** no `TextOverflow` findings

### TC-15.13.2.4 Locale Validation -- RTL Layout

| # | Requirement |
|---|-------------|
| 1 | R-15.13.2   |
| 2 | R-15.13.2   |

1. **#1** — Arabic text in LTR-only widget
   - **Expected:** finding with `RtlLayoutIssue`
2. **#2** — Arabic text in bidi-capable widget
   - **Expected:** no `RtlLayoutIssue` findings

### TC-15.13.2.5 Locale Validation -- Interpolation

| # | Requirement |
|---|-------------|
| 1 | R-15.13.2   |
| 2 | R-15.13.2   |

1. **#1** — Source has `{name}`, translation missing `{name}`
   - **Expected:** finding with `BrokenInterpolation`
2. **#2** — Source has `{name}`, translation has `{name}`
   - **Expected:** no `BrokenInterpolation` findings

### TC-15.13.2.6 Bidi Algorithm Resolution

| # | Requirement |
|---|-------------|
| 1 | R-15.13.2   |
| 2 | R-15.13.2   |

1. **#1** — `resolve_bidi("Hello \u{0627}\u{0644}")` (mixed LTR/RTL)
   - **Expected:** 2 `BidiRun` entries with correct directions
2. **#2** — `resolve_bidi("Hello World")` (pure LTR)
   - **Expected:** 1 `BidiRun` with `LeftToRight`

### TC-15.13.1.8 Font Fallback Resolution

| # | Requirement |
|---|-------------|
| 1 | R-15.13.1   |
| 2 | R-15.13.1   |
| 3 | R-15.13.1   |

1. **#1** — `resolve(0x4E00)` (CJK ideograph), chain has CJK font
   - **Expected:** returns CJK font `AssetId`
2. **#2** — `resolve(0x41)` (Latin 'A'), chain has Latin font first
   - **Expected:** returns Latin font `AssetId`
3. **#3** — `shape_runs("Hello\u{4E16}\u{754C}")` (mixed Latin/CJK)
   - **Expected:** 2 `FontRun` entries

### TC-15.19.1.1 Reflect Metadata Extraction

| # | Requirement |
|---|-------------|
| 1 | R-15.19.1   |
| 2 | R-15.19.1   |
| 3 | R-15.19.1   |

1. **#1** — Register type with 3 fields and 2 methods, `extract()`
   - **Expected:** `ApiEntry` with 3 `FieldDoc` and 2 `MethodDoc`
2. **#2** — Register type with doc comment, `extract()`
   - **Expected:** `ApiEntry.doc_comment == Some("...")`
3. **#3** — `extract("NonExistentType")`
   - **Expected:** `None`

### TC-15.19.1.2 Search Index Query

| # | Requirement |
|---|-------------|
| 1 | R-15.19.1   |
| 2 | R-15.19.1   |

1. **#1** — Index 100 types, `query("Transform", 10)`
   - **Expected:** results contain type named `"Transform"`
2. **#2** — `query("zzzznonexistent", 10)`
   - **Expected:** empty results

### TC-15.19.1.3 Versioned Doc Store

| # | Requirement |
|---|-------------|
| 1 | R-15.19.1   |
| 2 | R-15.19.1   |

1. **#1** — Store pages for v1.0.0 and v1.1.0, fetch from v1.0.0
   - **Expected:** correct page content for v1.0.0
2. **#2** — Fetch page from unstored v2.0.0
   - **Expected:** `None`

### TC-15.19.2.1 Node Documentation Completeness

| # | Requirement |
|---|-------------|
| 1 | R-15.19.2   |
| 2 | R-15.19.2   |
| 3 | R-15.19.2   |

1. **#1** — `NodeDoc` with empty `description`
   - **Expected:** finding reported for missing description
2. **#2** — `NodeDoc` with empty `examples`
   - **Expected:** finding reported for missing examples
3. **#3** — Complete `NodeDoc` with description, ports, examples
   - **Expected:** no findings

### TC-15.19.3.1 Tutorial Step Advancement

| # | Requirement |
|---|-------------|
| 1 | R-15.19.3   |
| 2 | R-15.19.3   |

1. **#1** — Start tutorial with 3 steps, fire completion event for step 1, call `try_advance()`
   - **Expected:** returns `true`, `progress() == (1, 3)`
2. **#2** — Call `try_advance()` without completing condition
   - **Expected:** returns `false`, step unchanged

### TC-15.19.3.2 Tutorial Pause and Resume

| # | Requirement |
|---|-------------|
| 1 | R-15.19.3   |
| 2 | R-15.19.3   |

1. **#1** — Start tutorial at step 1, `pause()`, `resume()`
   - **Expected:** `state() == Running`, `progress() == (0, N)` unchanged
2. **#2** — `pause()`
   - **Expected:** `state() == Paused`

### TC-15.19.5.1 Contextual Help Lookup

| # | Requirement |
|---|-------------|
| 1 | R-15.19.5   |
| 2 | R-15.19.5   |
| 3 | R-15.19.5   |

1. **#1** — `lookup("TransformInspector", "en-US")`
   - **Expected:** `Some(HelpEntry)` with correct `short_desc`
2. **#2** — `lookup("TransformInspector", "ja-JP")`
   - **Expected:** `Some(HelpEntry)` with Japanese locale content
3. **#3** — `lookup("NonExistentWidget", "en-US")`
   - **Expected:** `None`

## Integration Tests

### TC-15.13.2.I1 Full Locale Switch

| # | Requirement |
|---|-------------|
| 1 | R-15.13.2   |
| 2 | R-15.13.2   |

1. **#1** — Set active locale from `"en-US"` to `"ar-SA"`
   - **Expected:** all widget text re-rendered, RTL layout applied
2. **#2** — Set active locale from `"en-US"` to `"ja-JP"`
   - **Expected:** CJK font fallback chain active, text renders correctly

### TC-15.13.2.I2 Pseudo-Localization Full UI

| # | Requirement |
|---|-------------|
| 1 | R-15.13.2   |

1. **#1** — Enable pseudo-locale with `accented + bracketed`
   - **Expected:** no plain ASCII `a-z` in any rendered UI string

### TC-15.13.3.I1 XLIFF TMS Compatibility

| # | Requirement |
|---|-------------|
| 1 | R-15.13.3   |

1. **#1** — Export XLIFF 2.1, import into TMS simulator, re-export, import back
   - **Expected:** all 5000 strings preserved with correct states

### TC-15.19.1.I1 Full API Reference Generation

| # | Requirement |
|---|-------------|
| 1 | R-15.19.1   |

1. **#1** — Run `generate()` against full `TypeRegistry`
   - **Expected:** `DocArtifact.page_count > 0`, all registered types have pages

### TC-15.19.1.I2 Help Panel Latency

| # | Requirement |
|---|-------------|
| 1 | R-15.19.1   |

1. **#1** — Open help panel, measure time to first content
   - **Expected:** latency < 500 ms

### TC-15.19.5.I1 Tooltip Latency

| # | Requirement |
|---|-------------|
| 1 | R-15.19.5   |

1. **#1** — Hover over widget, measure time to tooltip display
   - **Expected:** latency < 200 ms

### TC-15.19.3.I1 Tutorial End-to-End

| # | Requirement |
|---|-------------|
| 1 | R-15.19.3   |

1. **#1** — Run GettingStarted tutorial, complete all steps
   - **Expected:** `state() == Complete`, all steps visited

### TC-15.19.7.I1 Doc-Test Compilation

| # | Requirement |
|---|-------------|
| 1 | R-15.19.7   |

1. **#1** — Extract all inline code examples, compile each
   - **Expected:** all examples compile without errors

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
