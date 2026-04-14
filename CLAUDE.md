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

Language-specific coding standards are loaded automatically as skills when reading, writing, or
designing systems in the associated language.

| Skill | Scope |
|-------|-------|
| `rust` | Primary engine language (`.rs`) |
| `hlsl` | Shader IL / DXC pipeline (`.hlsl`, `.hlsli`) |
| `markdown` | Documentation / rumdl (`.md`) |
| `json` | Configuration files (`.json`) |
| `toml` | Cargo.toml, .rumdl.toml (`.toml`) |
| `yaml` | GitHub Actions workflows (`.yml`, `.yaml`) |

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

## Philosophy

The engine's design philosophy — composition over inheritance, generic primitives, platform-native
harmony — is in the [README.md](README.md#philosophy) Philosophy section. Read it to understand
the "why" behind every design decision.

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

## Harmonize plugin

Always use the harmonize plugin's agents and skills for every part of the software
development cycle: ideation, design, testing, implementation, review, or release. Do not
perform these tasks manually — spawn the right background agent or load the right
interactive sub-skill.

### Lifecycle

| Phase | Orchestrator | Workers |
|-------|--------------|---------|
| 1 Specify | `specify-orchestrator` | `feature-author`, `requirement-author`, `user-story-author` |
| 2 Design | `design-orchestrator` | `subsystem-designer`, `interface-designer`, `component-designer`, `integration-designer`, `design-reviewer`, `design-reviser` |
| 3 Plan + TDD | `plan-orchestrator` | `plan-author`, `plan-implementer`, `pr-reviewer`, `test-writer`, `implementer`, `review-supervisor`, `correctness/standards/architecture-reviewer` |
| 4 Release | `release-orchestrator` | `release-notes-author`, `changelog-updater`, `tagger` |

The `harmonize` master agent supervises all four phases and dispatches phase orchestrators
as background tasks.

### Interactive sub-skills

Load the appropriate sub-skill when you want to give feedback at a specific level:

| Sub-skill | For |
|-----------|-----|
| `harmonize-specify` | Author or revise features, requirements, user stories |
| `harmonize-design` | Author or revise subsystem / interface / component / integration designs |
| `harmonize-plan` | Author or revise implementation plans |
| `harmonize-implement` | Step through or observe a plan's TDD loop |
| `harmonize-review` | Triage a draft PR's review findings interactively |
| `harmonize-release` | Cut a release |

Each sub-skill claims a coarse `(phase, subsystem)` lock so background workers stay away
from what the user is working on.

### Skills

| Skill | Purpose |
|-------|---------|
| `harmonize` | Master entry point — routes SDLC work, boots cron, manual merge-detect backup |
| `harmonize-specify` | Interactive Phase 1 |
| `harmonize-design` | Interactive Phase 2 |
| `harmonize-plan` | Interactive Phase 3 plan authoring |
| `harmonize-implement` | Interactive Phase 3 TDD step-through |
| `harmonize-review` | Interactive Phase 3 PR review |
| `harmonize-release` | Interactive Phase 4 |
| `document-templates` | Templates for every artifact type |

### Harmonize trigger

When the user mentions "harmonize" in any form (e.g., "harmonize", "run harmonize",
"harmonize status", "author a harmonize plan"), you MUST invoke the `/harmonize` skill via
the Skill tool BEFORE doing anything else. The skill is the operational playbook for the
full SDLC. Do not act on harmonize requests from memory — always load the skill first.

Similarly, when the user mentions a specific sub-skill name or a phase-scoped request,
invoke the matching sub-skill directly (e.g., `/harmonize-design docs/design/...`).

The `harmonize` skill (2.0.1+) runs a **manual merge-detection backup** when `/harmonize` is invoked
with no extra argument, when cron bootstrap cannot confirm a job, or when the user asks for
`/harmonize merge-detect`. Refresh the installed copy after the marketplace updates:

`claude plugin update harmonize@cjhowe-us-workflow`

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
| Markdown check | `rumdl check .` |
| Markdown fix | `rumdl fmt .` |
| Build Rust | `cargo build` |
| Test Rust | `cargo test` |

## Prerequisites

| Tool | Version |
|------|---------|
| Rust | stable (1.80+) |
| harmonize plugin | 2.0.1+ (`harmonize@cjhowe-us-workflow`) |
| jq | latest |
| rumdl | latest |
| taplo | latest |
| yq | latest |

## Key files

| File | Purpose |
|------|---------|
| `docs/architecture.md` | Engine architecture overview |
| `docs/design/constraints.md` | Design constraints |
| `Cargo.toml` | Rust workspace definition |
| `.rumdl.toml` | Markdown lint config |
