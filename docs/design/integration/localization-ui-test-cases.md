# Localization ↔ UI Integration Test Cases

All tests are CI-runnable. They use a small fake `LocalizationTable` constructed in-memory and a
fake shaper that emits deterministic glyph runs with advances matching the input strings.

## Integration Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-4.4.1.1 | Resolved text matches locale entry | IR-4.4.1 |
| TC-IR-4.4.1.2 | Fallback locale on missing key | IR-4.4.1 |
| TC-IR-4.4.1.3 | `[missing:id]` when absent in fallback | IR-4.4.1 |
| TC-IR-4.4.2.1 | RTL string reverses layout alignment | IR-4.4.2 |
| TC-IR-4.4.2.2 | Mixed bidi falls back to LTR block | IR-4.4.2 |
| TC-IR-4.4.2.3 | Icon order flips in RTL | IR-4.4.2 |
| TC-IR-4.4.3.1 | Script tag selects font chain | IR-4.4.3 |
| TC-IR-4.4.3.2 | Missing glyph falls through chain | IR-4.4.3 |
| TC-IR-4.4.4.1 | IME Start shows composition underline | IR-4.4.4 |
| TC-IR-4.4.4.2 | IME Update replaces composition text | IR-4.4.4 |
| TC-IR-4.4.4.3 | IME Commit clears composition | IR-4.4.4 |
| TC-IR-4.4.4.4 | IME Cancel clears composition | IR-4.4.4 |
| TC-IR-4.4.5.1 | Locale change invalidates layout | IR-4.4.5 |
| TC-IR-4.4.5.2 | Locale change re-runs font chain | IR-4.4.5 |
| TC-IR-4.4.5.3 | Locale change propagates to VO audio | IR-4.4.5 |
| TC-IR-4.4.6.1 | Plural arg selects correct form | IR-4.4.6 |
| TC-IR-4.4.6.2 | Gender arg selects correct form | IR-4.4.6 |
| TC-IR-4.4.6.3 | Numeric arg formatted per locale | IR-4.4.6 |

## Negative Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-4.4.1.N1 | Missing string (FM-1) uses fallback | IR-4.4.1 |
| TC-IR-4.4.1.N2 | Missing in both (FM-2) | IR-4.4.1 |
| TC-IR-4.4.3.N1 | Empty font chain -> `.notdef` (FM-3) | IR-4.4.3 |
| TC-IR-4.4.4.N1 | Commit after focus lost (FM-4) | IR-4.4.4 |
| TC-IR-4.4.5.N1 | Locale change mid-layout defers (FM-7) | IR-4.4.5 |
| TC-IR-4.4.6.N1 | Missing format arg (FM-6) | IR-4.4.6 |

### Test case details

1. **TC-IR-4.4.1.1** -- Input: `LocalizedStringId(7)` in `fr`, table has "Bonjour". Expected:
   `ResolvedText.display == "Bonjour"`, `direction == Ltr`.
2. **TC-IR-4.4.1.2** -- Input: `id=7` in `de`, missing; fallback `en` has "Hello". Expected:
   `display == "Hello"`; `FM-1` counter increments.
3. **TC-IR-4.4.1.3** -- Input: `id=42` absent in both. Expected: `display == "[missing:42]"`; `FM-2`
   counter increments.
4. **TC-IR-4.4.2.1** -- Input: Arabic string with `direction=Rtl`. Expected: UI layout runs with
   right alignment; first glyph sits at right edge.
5. **TC-IR-4.4.2.2** -- Input: mixed English + Hebrew (`Auto` on template). Expected: bidi algorithm
   produces LTR/RTL runs; layout places each run correctly.
6. **TC-IR-4.4.2.3** -- Input: button with icon + label, RTL locale. Expected: icon on the right,
   label on the left.
7. **TC-IR-4.4.3.1** -- Input: `script=Hani`. Expected: `FontChain` starts with CJK font asset id.
8. **TC-IR-4.4.3.2** -- Input: glyph not in primary font, present in fallback. Expected: shaper
   emits glyph run from fallback font.
9. **TC-IR-4.4.4.1** -- Input: `ImeEvent::Start { text: "か" }` on focused TextInput. Expected:
   composition buffer shows "か" with underline style.
10. **TC-IR-4.4.4.2** -- Input: Start then Update with text "かん". Expected: composition now
    "かん".
11. **TC-IR-4.4.4.3** -- Input: Commit with "感". Expected: composition cleared; `TextInput.text`
    appended with "感".
12. **TC-IR-4.4.4.4** -- Input: Cancel. Expected: composition cleared; `TextInput.text` unchanged.
13. **TC-IR-4.4.5.1** -- Input: locale change from `en` -> `fr`. Expected: every widget with a
    `LocalizedStringId` has `layout_dirty = true`.
14. **TC-IR-4.4.5.2** -- Input: locale change to `ar`. Expected: font chain reselected for
    script=Arab.
15. **TC-IR-4.4.5.3** -- Input: locale change. Expected: audio VO subsystem receives
    `LocaleChangeEvent` on `CH-29`.
16. **TC-IR-4.4.6.1** -- Input: pattern "{n, plural, one{# item} other{# items}}", args `n=1`.
    Expected: `"1 item"`.
17. **TC-IR-4.4.6.2** -- Input: pattern "{g, gender, male{he} female{she} other{they}}", args
    `g=female`. Expected: `"she"`.
18. **TC-IR-4.4.6.3** -- Input: pattern "{n, number}" with `n=1234.5`, locale `fr`. Expected:
    `"1 234,5"`.
19. **TC-IR-4.4.1.N1** -- Covered by TC-IR-4.4.1.2.
20. **TC-IR-4.4.1.N2** -- Covered by TC-IR-4.4.1.3.
21. **TC-IR-4.4.3.N1** -- Input: widget text has glyphs none of whose fonts support. Expected:
    `.notdef` glyph rendered; `FM-3` counter increments once per unsupported glyph.
22. **TC-IR-4.4.4.N1** -- Input: IME Commit arrives after focus moved to another widget. Expected:
    commit discarded; `FM-4` counter increments.
23. **TC-IR-4.4.5.N1** -- Input: locale change arrives mid-layout. Expected: layout this frame
    completes with old locale; new locale applied next frame; `FM-7` counter increments.
24. **TC-IR-4.4.6.N1** -- Input: pattern "Hello {name}", args `{}`. Expected:
    `"Hello {missing:name}"`; `FM-6` counter increments.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-4.4.1.B1 | Resolve 500 strings | < 0.1 ms | IR-4.4.1 |
| TC-IR-4.4.3.B1 | Font chain select 500 strings | < 0.05 ms | IR-4.4.3 |
| TC-IR-4.4.5.B1 | Locale switch 500 widgets | < 2.0 ms | IR-4.4.5 |
| TC-IR-4.4.6.B1 | Format 100 pluralized strings | < 0.1 ms | IR-4.4.6 |

All benchmarks run under `cargo bench` in CI; thresholds enforced via the benchmark harness.
