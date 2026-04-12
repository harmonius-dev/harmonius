# Containers/Slots ↔ UI Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-5.9.1.1 | Grid displays 20 items | Container with 20 filled slots | 20 item icons in grid cells | IR-5.9.1 |
| TC-IR-5.9.1.2 | Grid updates on add | Add item to container | New icon appears in next free cell | IR-5.9.1 |
| TC-IR-5.9.1.3 | Grid updates on remove | Remove item from container | Cell becomes empty | IR-5.9.1 |
| TC-IR-5.9.1.4 | Stack quantity display | Slot with quantity=5 | "5" label on item icon | IR-5.9.1 |
| TC-IR-5.9.2.1 | Equipment slots show attachments | 6 sockets, 4 with items | 4 slots show item icons, 2 empty | IR-5.9.2 |
| TC-IR-5.9.2.2 | Equipment slot compatibility hint | Drag incompatible item over slot | Slot border turns red | IR-5.9.2 |
| TC-IR-5.9.2.3 | Equipment slot shows socket name | Head socket | "Head" label on slot widget | IR-5.9.2 |
| TC-IR-5.9.3.1 | Drag item between containers | Drag from bag to chest | Item in chest, removed from bag | IR-5.9.3 |
| TC-IR-5.9.3.2 | Drag to full container fails | Drag to full container | Error feedback, item stays in source | IR-5.9.3 |
| TC-IR-5.9.3.3 | Drag ghost preview follows cursor | Begin drag | Translucent item icon at cursor | IR-5.9.3 |
| TC-IR-5.9.3.4 | Drop target highlight | Drag over valid slot | Slot border highlights green | IR-5.9.3 |
| TC-IR-5.9.4.1 | Stack split via modifier+drag | Shift+drag stack of 10 | Split dialog, enter 5 | IR-5.9.4 |
| TC-IR-5.9.4.2 | Split clamped to stack size | Try split 15 from stack of 10 | Clamped to 10 | IR-5.9.4 |
| TC-IR-5.9.4.3 | Split minimum is 1 | Try split 0 | Clamped to 1 | IR-5.9.4 |
| TC-IR-5.9.5.1 | 2x2 item occupies 4 cells | Place 2x2 item in grid | GridOccupancy marks 4 cells | IR-5.9.5 |
| TC-IR-5.9.5.2 | 2x2 item blocked by occupied cell | Place 2x2, 1 cell taken | Transfer denied, error feedback | IR-5.9.5 |
| TC-IR-5.9.6.1 | Hover shows item tooltip | Hover over sword icon | Tooltip with name, stats, weight | IR-5.9.6 |
| TC-IR-5.9.6.2 | Tooltip dismissed on mouse exit | Move mouse away | Tooltip disappears | IR-5.9.6 |
| TC-IR-5.9.7.1 | Sort by name | Click "Sort by Name" | Items reordered alphabetically | IR-5.9.7 |
| TC-IR-5.9.7.2 | Sort by rarity | Click "Sort by Rarity" | Items reordered by rarity tier | IR-5.9.7 |
| TC-IR-5.9.7.3 | Filter by type | Select "Weapons" filter | Only weapon items visible | IR-5.9.7 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-5.9.1.B1 | Inventory grid 500 items render | < 2 ms layout + paint | IR-5.9.1 |
| TC-IR-5.9.3.B1 | Drag-drop transfer validation | < 0.1 ms | IR-5.9.3 |
| TC-IR-5.9.5.B1 | Grid bin-pack 500 items | < 1 ms | IR-5.9.5 |
| TC-IR-5.9.7.B1 | Sort 500 items | < 1 ms | IR-5.9.7 |
