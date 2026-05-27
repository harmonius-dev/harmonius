# Agent Handbook for Harmonius

This handbook provides guidelines and best practices for agents working on the Harmonius project.

## Rulebook

- Put all documentation in the `docs/` directory. Use Markdown format for all documentation.
- Never create ASCII diagrams. Instead, create Mermaid diagrams. Always render Mermaid diagrams with
  the MCP to ensure correctness and readability.
- Always limit lines to 100 characters for all code, configuration, documentation, and comments. The
  limit may be exceeded if the line cannot be split without changing semantics.
- Tables in Markdown documents must also fit within 100 characters per line. Keep short identifiers
  (IDs, names, refs) in the table and move long content (descriptions, rationale, verification) into
  a numbered detail list below the table. Split wide tables into two tables sharing column 0 as key.
- Always sort JSON documents by keys in lexicographical order. Do not sort JSON arrays, only JSON
  objects.
- Be visual when possible. Use Mermaid diagrams frequently to illustrate concepts to me during
  conversation. Include them in documentation and README files when they illustrate a point well. I
  prefer visual information over prose.
- Keep documentation prose structured using headings, subheadings, numbered lists, bulleted lists,
  and tables. Keep paragraphs under 5 sentences and 50 words.
- Wrap git commit messages to a 50 character line length limit for the first line, and a 72
  character line length limit for the rest of the commit message.

## macOS build and test (xcodebuild)

Use the XcodeGen project and `xcodebuild` on macOS. Generate the project first:

```bash
xcodegen generate
```

Build the app:

```bash
xcodebuild build \
  -project Harmonius.xcodeproj \
  -scheme HarmoniusApp \
  -destination "platform=macOS" \
  -derivedDataPath build/xcodegen
```

Built app: `build/xcodegen/Build/Products/Debug/HarmoniusApp.app`

Run all unit and UI tests (same as CI):

```bash
xcodebuild test \
  -project Harmonius.xcodeproj \
  -scheme HarmoniusApp \
  -destination "platform=macOS" \
  -derivedDataPath build/xcodegen \
  -resultBundlePath build/xcodegen/test-results/HarmoniusApp.xcresult
```

Unit tests only:

```bash
xcodebuild test \
  -project Harmonius.xcodeproj \
  -scheme HarmoniusApp \
  -only-testing:HarmoniusUnitTests \
  -destination "platform=macOS" \
  -derivedDataPath build/xcodegen
```

UI snapshot tests only (prepend `SNAPSHOT_RECORD=1` to record baselines with SnapshotTesting):

```bash
xcodebuild test \
  -project Harmonius.xcodeproj \
  -scheme HarmoniusApp \
  -only-testing:HarmoniusUITests \
  -destination "platform=macOS" \
  -derivedDataPath build/xcodegen
```

The `HarmoniusApp` target runs a CMake pre-build script. Prefer `xcodebuild` over invoking CMake
directly when building the app or running tests.

## Testing policy

- **Test-driven development** — write failing tests first, then implement until tests green
- **NO MOCKING** — mocking is explicitly forbidden in all languages; no mock libraries, no mock
  objects
- **Real dependencies** — always prefer real objects over fakes; only use fakes when no other option
  exists
- **Fakes only when necessary** — write a full fake that emulates real behavior; implement the same
  interface, trait, or protocol
- **Pure functions** — maximize pure functions that take input and return output with no side
  effects
- **Immutable test data** — use constants for test fixtures; never mutate shared test state
