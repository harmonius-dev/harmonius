# Data Tables ↔ UI Framework Integration Test Cases

Companion test cases for [data-tables-ui.md](data-tables-ui.md).

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-4.10.8.1 | Table rows sort by column ascending | `sort=(level, Asc)` on 10-row table | List populates with rows in ascending `level` order | IR-4.10.8 |
| TC-IR-4.10.8.2 | Table rows sort by column descending | `sort=(name, Desc)` on 10-row table | List populates in descending name order | IR-4.10.8 |
| TC-IR-4.10.8.3 | Sort applied after filter before page | Filter `level>=5`, sort `(name, Asc)`, page size 5 | First page shows 5 filtered rows in ascending name order | IR-4.10.8 |
| TC-IR-4.10.1.1 | List populates from table | 10-row table bound | 10 list item widgets | IR-4.10.1 |
| TC-IR-4.10.1.2 | Empty table shows no items | 0-row table bound | Empty list, no children | IR-4.10.1 |
| TC-IR-4.10.1.3 | List recycles on rebind | Rebind to different table | Old items recycled | IR-4.10.1 |
| TC-IR-4.10.2.1 | Column binds to label text | String column | Label.text = cell value | IR-4.10.2 |
| TC-IR-4.10.2.2 | Column binds to progress | F32 column, value=0.75 | ProgressBar.value = 0.75 | IR-4.10.2 |
| TC-IR-4.10.2.3 | Column binds to icon | AssetRef column | Image.asset = ref | IR-4.10.2 |
| TC-IR-4.10.3.1 | Filter by equality | filter: name == "Sword" | Only sword rows shown | IR-4.10.3 |
| TC-IR-4.10.3.2 | Filter by range | filter: level >= 5 | Rows with level >= 5 | IR-4.10.3 |
| TC-IR-4.10.3.3 | Filter change re-populates | Change filter at runtime | List updates same frame | IR-4.10.3 |
| TC-IR-4.10.4.1 | Hot reload updates list | TableReloaded fires | Widgets reflect new data | IR-4.10.4 |
| TC-IR-4.10.4.2 | Hot reload preserves scroll | Reload mid-scroll | Scroll offset retained | IR-4.10.4 |
| TC-IR-4.10.5.1 | FK resolves display name | ForeignKey column | Label shows resolved name | IR-4.10.5 |
| TC-IR-4.10.5.2 | Dangling FK shows fallback | FK target row deleted | Label shows fallback text | IR-4.10.5 |
| TC-IR-4.10.6.1 | Stat panel reads ability row | Ability table RowRef | Damage, cooldown displayed | IR-4.10.6 |
| TC-IR-4.10.6.2 | Stat panel reads class row | Class table RowRef | HP, mana, name displayed | IR-4.10.6 |
| TC-IR-4.10.7.1 | Virtual list pages rows | page_size=20, 100 rows | 20 items visible | IR-4.10.7 |
| TC-IR-4.10.7.2 | Scroll advances page offset | Scroll down 20 items | page_offset = 20 | IR-4.10.7 |
| TC-IR-4.10.7.3 | Page clamp at end | Scroll past last row | Offset clamped to max | IR-4.10.7 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-4.10.1.B1 | Bind 1000-row table to list | < 2 ms | IR-4.10.1 |
| TC-IR-4.10.3.B1 | Filter 10k rows | < 0.5 ms | IR-4.10.3 |
| TC-IR-4.10.5.B1 | Resolve 100 foreign keys | < 0.1 ms | IR-4.10.5 |
| TC-IR-4.10.7.B1 | Virtual list scroll 60 fps | Zero steady-state allocs | IR-4.10.7 |
