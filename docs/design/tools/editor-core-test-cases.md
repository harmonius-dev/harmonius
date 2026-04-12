# Editor Core Test Cases

Companion test cases for [editor-core.md](editor-core.md).

Consolidates test cases from the following former files:

- editor-framework-test-cases.md
- editor-plugins-test-cases.md
- vr-editor-test-cases.md

See those files for the full test case definitions until migration is complete.

## Unit Tests

| ID | Requirement | Description |
|----|-------------|-------------|
| TC-15.1.1.1 | R-15.1.1 | Dock split horizontal |
| TC-15.1.1.2 | R-15.1.1 | Dock split vertical |
| TC-15.1.1.3 | R-15.1.1 | Dock add tab |
| TC-15.1.1.4 | R-15.1.1 | Dock float and redock |
| TC-15.1.1.5 | R-15.1.1 | Dock close panel |
| TC-15.1.2.1 | R-15.1.2 | Viewport create |
| TC-15.1.2.2 | R-15.1.2 | Viewport camera modes |
| TC-15.1.2.3 | R-15.1.2 | Viewport screen-to-ray |
| TC-15.1.3.1 | R-15.1.3 | Undo single command |
| TC-15.1.3.2 | R-15.1.3 | Redo single command |
| TC-15.1.3.3 | R-15.1.3 | Transaction undo |
| TC-15.1.3.4 | R-15.1.3 | Undo clears redo |
| TC-15.1.3.5 | R-15.1.3 | Crash recovery replay |
| TC-15.1.4.1 | R-15.1.4 | Click select entity |
| TC-15.1.4.2 | R-15.1.4 | Marquee select |
| TC-15.1.4.3 | R-15.1.4 | Selection modifiers |
| TC-15.1.4.4 | R-15.1.4 | Named selection sets |
| TC-15.1.5.1 | R-15.1.5 | Translate gizmo axis |
| TC-15.1.5.2 | R-15.1.5 | Rotate gizmo |
| TC-15.1.5.3 | R-15.1.5 | Scale gizmo with snap |
| TC-15.1.5.4 | R-15.1.5 | Gizmo reference frame |
| TC-15.1.6.1 | R-15.1.6 | Measurement distance |
| TC-15.1.7.1 | R-15.1.7 | Preference get/set |
| TC-15.1.7.2 | R-15.1.7 | Preference migration |
| TC-1.6.1.1 | R-1.6.1 | Plugin discovery |
| TC-1.6.2.1 | R-1.6.2 | Plugin load lifecycle |
| TC-1.6.3.1 | R-1.6.3 | Plugin hot-reload state |
| TC-1.6.4.1 | R-1.6.4 | Plugin panic isolation |
| TC-1.6.5.1 | R-1.6.5 | Dependency resolution |
| TC-1.6.5.2 | R-1.6.5 | Circular dependency |
| TC-1.6.6.1 | R-1.6.6 | ABI version compat |
| TC-1.6.7.1 | R-1.6.7 | Custom widget register |
| TC-1.6.7.2 | R-1.6.7 | Widget lookup dispatch |
| TC-15.1.9.1 | R-15.1.9 | VR mode enter/exit |
| TC-15.16.1.1 | R-15.16.1 | Controller mapping |
| TC-15.16.1.2 | R-15.16.1 | Hand gesture recognition |
| TC-15.16.2.1 | R-15.16.2 | VR panel spawn/dismiss |
| TC-15.16.2.2 | R-15.16.2 | VR panel anchor modes |
| TC-15.16.3.1 | R-15.16.3 | VR avatar visibility |
| TC-15.16.4.1 | R-15.16.4 | Follow mode attach |
| TC-15.16.4.2 | R-15.16.4 | Follow mode break |
| TC-15.16.5.1 | R-15.16.5 | VR perf budget adjust |

## Integration Tests

| ID | Requirement | Description |
|----|-------------|-------------|
| TC-15.1.I.1 | R-15.1.1--R-15.1.8 | Full editor lifecycle |
| TC-15.1.I.2 | R-15.1.3 | Dual-world event bridge |
| TC-15.1.I.3 | R-15.1.4, R-15.1.5 | Select + gizmo flow |
| TC-1.6.I.1 | R-1.6.1--R-1.6.7 | Plugin load-to-unload |
| TC-1.6.I.2 | R-1.6.3 | Hot-reload round-trip |
| TC-1.6.I.3 | R-1.6.7 | Component editor dispatch |
| TC-15.16.I.1 | R-15.16.1--R-15.16.5 | VR mode full session |
| TC-1.6.1.I1 | US-1.6.1 | Plugin trait declares types, resources, systems end-to-end |
| TC-1.6.2.I1 | US-1.6.2 | Named plugin group (DefaultPlugins) registers all members |
| TC-1.6.3.I1 | US-1.6.3 | Individual plugin disable removes systems without crash |
| TC-1.6.4.I1 | US-1.6.4 | Plugin declares dependency and loads after dependency |
| TC-1.6.5.I1 | US-1.6.5 | Init order resolves topologically across 5 plugins |
| TC-1.6.6.I1 | US-1.6.6 | Missing and circular dependencies produce structured errors |
| TC-1.6.7.I1 | US-1.6.7 | Hot-reload plugin during running session preserves state |
| TC-15.1.1.I1 | US-15.1.1 | Dock layout save and restore across sessions |
| TC-15.1.2.I1 | US-15.1.2 | Viewport camera modes + pick ray via screen click |
| TC-15.1.4.I1 | US-15.1.4 | Click + marquee + named set selection workflow |
| TC-15.1.5.I1 | US-15.1.5 | Translate/rotate/scale gizmo with snap and reference frame |
| TC-15.1.6.I1 | US-15.1.6 | Measurement tool reports distance/angle/area |
| TC-15.1.7.I1 | US-15.1.7 | Preference edit + migration across editor versions |
| TC-15.1.8.I1 | US-15.1.8 | Extension ScriptableObject hook fires on editor event |
| TC-15.1.9.I1 | US-15.1.9 | VR edit mode enter and exit from desktop session |
| TC-15.16.1.I1 | US-15.16.1 | VR controller + hand gesture input routed to editor panels |
| TC-15.16.2.I1 | US-15.16.2 | VR spatial panels spawn, anchor, and dismiss |
| TC-15.16.3.I1 | US-15.16.3 | VR avatar visibility toggled in co-edit session |
| TC-15.16.4.I1 | US-15.16.4 | Follow-mode attach to another avatar and break on gesture |

## Benchmarks

| ID | Requirement | Description |
|----|-------------|-------------|
| TC-15.1.B.1 | US-15.1.NF1.1 | UI input ack < 16 ms |
| TC-15.1.B.2 | US-15.1.3.6 | Undo/redo < 50 ms |
| TC-15.1.B.3 | US-15.1.NF1.4 | Selection 10k entities |
| TC-1.6.B.1 | R-1.6.2 | Plugin load time |
| TC-15.16.B.1 | US-15.16.5.1 | VR >= 90 fps |
| TC-15.16.B.2 | US-15.16.5.2 | Motion-to-photon < 20 ms |
