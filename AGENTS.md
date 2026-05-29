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

## Build and test workflow

Use `scripts/dev.sh` as the canonical interface for builds, tests, Appium, and
release validation. Do not hand-expand the dependency order in VS Code, CI, or
LLM instructions.

Bootstrap the host platform:

```bash
./scripts/dev.sh bootstrap macos
```

Build the SwiftPM package:

```bash
./scripts/dev.sh compile-spm macos debug
```

Build the macOS app bundle with XcodeGen and Xcode:

```bash
./scripts/dev.sh bundle macos debug
```

Run package tests:

```bash
./scripts/dev.sh test-unit
./scripts/dev.sh test-render
```

Run Appium UI smoke tests:

```bash
./scripts/dev.sh test-ui-macos
./scripts/dev.sh test-ui-ios-simulator
```

Run the main quality gate:

```bash
./scripts/dev.sh full-check
```

## Testing policy

- **Test-driven development** — write failing tests first, then implement until tests green
- **NO MOCKING** — mocking is explicitly forbidden in all languages; no mock libraries, no mock
  objects
- **Real dependencies** — always prefer real objects over fakes; only use fakes when no other
  option exists
- **Fakes only when necessary** — write a full fake that emulates real behavior; implement the
  same interface, trait, or protocol
- **Pure functions** — maximize pure functions that take input and return output with no side
  effects
- **Immutable test data** — use constants for test fixtures; never mutate shared test state
