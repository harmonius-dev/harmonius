# User Stories — 12.4 Hot Reload

## US-12.4.1 See Changes in the Engine Instantly After Editing an Asset
**As an** artist, **I want** file watching, asset hot reload, shader hot reload, logic graph
hot reload, UI hot reload, partial re-import, and editor-runtime sync, **so that** every
change I make is reflected in the running engine within seconds without restarting.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Platform-native file watcher with debouncing | F-12.4.1 | R-12.4.1 |
| Texture swap in <2s via descriptor heap update | F-12.4.2 | R-12.4.2 |
| Shader recompile and PSO swap at frame boundary | F-12.4.3 | R-12.4.3 |
| Logic graph hot reload preserving execution state | F-12.4.4 | R-12.4.4 |
| UI hot reload preserving scroll and focus state | F-12.4.5 | R-12.4.5 |
| Partial re-import of changed sub-assets only | F-12.4.6 | R-12.4.6 |
| Bidirectional editor-runtime sync channel | F-12.4.7 | R-12.4.7 |
| Hot reload applies at sync points only (not mid-frame) | F-12.4.2 | R-X.6.1 |
| Texture <2s, mesh <3s, logic graph <500ms, shader <5s | F-12.4.2 | R-X.14.2 |
