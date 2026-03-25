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

## Coding standards

Language-specific coding standards are in [docs/standards/](docs/standards/). Read the relevant
standard before writing or reviewing code in any language.

| Standard | Scope |
|----------|-------|
| [Rust](docs/standards/rust.md) | Primary engine language |
| [Swift](docs/standards/swift.md) | macOS platform (C ABI) |
| [TypeScript](docs/standards/typescript.md) | Tooling, editor, build |
| [Python](docs/standards/python.md) | Scripts and tools (ruff) |
| [HLSL](docs/standards/hlsl.md) | Shader IL (DXC pipeline) |
| [Markdown](docs/standards/markdown.md) | Documentation (rumdl) |
| [JSON](docs/standards/json.md) | Configuration files |
| [TOML](docs/standards/toml.md) | Cargo.toml, .rumdl.toml |
| [YAML](docs/standards/yaml.md) | GitHub Actions workflows |

## Testing policy

- **Test-driven development** — write tests first, driven by requirements (R-X.Y.Z), features
  (F-X.Y.Z), and user stories (US-X.Y.Z)
- **NO MOCKING** — mocking is explicitly forbidden in all languages; no mock libraries, no mock
  objects
- **Real dependencies** — always prefer real objects over fakes; only use fakes when no other option
  exists
- **Fakes only when necessary** — write a full fake that emulates real behavior; implement the same
  interface, trait, or protocol
- **Pure functions** — maximize pure functions that take input and return output with no side
  effects
- **Immutable test data** — use constants for test fixtures; never mutate shared test state

## Architecture

The engine architecture overview with clickable Mermaid diagrams and cross-references to all design
documents, test cases, features, requirements, and user stories is at
[docs/architecture.md](docs/architecture.md). Read it before starting work on any subsystem.

## Design constraints

All project-wide design constraints (language, platform I/O, architecture, async model, etc.) are
documented in [docs/design/constraints.md](docs/design/constraints.md). Read that file before
starting any design or implementation work.

## Design process

- Always write design for significant features with major new changes. For minor changes, provide
  the design in a message if feasible. The design should include references to requirements, a quick
  overview, architecture diagrams, flowcharts, sequence diagrams, and class diagrams (data
  structures, enums, relationships).
- Use lists and tables to display requirements, OS API usage, tabular data, comparisons, matrices,
  features and any other structured or semi-structured information.
- Start from requirements, and rethink the design periodically to see if it needs to be redone. In
  this case, you should stash the current state, create a new branch, then delete all of the
  existing design files in the branch. Finally, write a new design from scratch that better meets
  the requirements. Throw away any inferior design iterations until you settle on an ideal design.
- Always come up with a test plan and an explicit list of test cases. Ensure full coverage of all
  code paths.
- Seek approval before adding or changing any dependencies. If you need to add a dependency, propose
  a low-level library that is well supported and maintained. Avoid using any frameworks.
- Always use the latest versions of dependencies and tools. Use the package manager CLI to install
  dependencies with the latest version automatically.
- Think about the implementation after the design of the API is finalized. Do not design and write
  code simultaneously. Separate design, design iteration, design review, implementation planning,
  test planning, test-driven implementation, verification, documentation, and release into distinct
  phases. Do not start implementation until the design is approved and finalized.

## Markdown linting

| Action | Command |
|--------|---------|
| Check | `rumdl check .` |
| Fix | `rumdl fmt .` |

Configuration is in `.rumdl.toml`. CI runs `rumdl check` on every push and auto-fixes violations. Do
not manually rewrap prose — let rumdl handle it.

## Additional guidelines

- Write implementation comments sparingly; prefer doc comments (`///`) for public API
- Keep code and documentation in sync — update docs when changing code and vice versa

## Commands

| Task | Command |
|------|---------|
| Build all | `scripts/build.sh` |
| Test all | `scripts/test.sh` |
| Lint all | `python3 scripts/hooks.py --all` |
| Lint staged | `python3 scripts/hooks.py --staged` |
| Markdown check | `rumdl check .` |
| Markdown fix | `rumdl fmt .` |
| Build Rust | `cargo build --manifest-path src/rust/Cargo.toml` |
| Test Rust | `cargo test --manifest-path src/rust/Cargo.toml` |
| Build Swift | `swift build --package-path src/swift` |
| Test Swift | `swift test --package-path src/swift` |
| Type-check TS | `bun run tsc --noEmit` |
| Lint Python | `ruff check scripts/ && mypy --strict scripts/` |

## Prerequisites

| Tool | Version |
|------|---------|
| Rust | stable (1.80+) |
| Python | 3.13+ |
| Bun | latest |
| rumdl | latest |
| ruff | latest |
| mypy | latest |

## Key files

| File | Purpose |
|------|---------|
| `docs/architecture.md` | Engine architecture overview |
| `docs/design/constraints.md` | Design constraints |
| `src/swift/Package.swift` | Swift package definition |
| `pyproject.toml` | Python/ruff/mypy config |
| `.rumdl.toml` | Markdown lint config |
| `scripts/hooks.py` | Pre-commit lint orchestrator |
| `src/rust/Cargo.toml` | Rust package definition |
