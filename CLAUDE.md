# Agent Handbook for Harmonius

This handbook provides guidelines and best practices for agents working on the Harmonius project.

## Rulebook

- Put all documentation in the `docs/` directory. Use Markdown format for all documentation.
- Never create ASCII diagrams. Instead, create Mermaid diagrams. Always render Mermaid diagrams with
  the MCP to ensure correctness and readability.
- Always limit lines to 100 characters for all code, configuration, documentation, and comments. The
  limit may be exceeded if the line cannot be split without changing semantics. The line length
  limit does not apply to tables in Markdown documents.
- Always sort JSON documents by keys in lexicographical order. Do not sort JSON arrays, only JSON
  objects.
- Be visual when possible. Use Mermaid diagrams frequently to illustrate concepts to me during
  conversation. Include them in documentation and README files when they illustrate a point well. I
  prefer visual information over prose.
- Keep documentation prose structured using headings, subheadings, numbered lists, bulleted lists,
  and tables. Keep paragraphs under 5 sentences and 50 words.
- Wrap git commit messages to a 50 character line length limit for the first line, and a 72
  character line length limit for the rest of the commit message.

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

## Development guidelines

- Write implementation comments sparingly. Always write documentation comments and generate an API
  reference from them. Use implementation comments only for complex algorithms or non-obvious code.
- Use test-driven development to ensure code quality and correctness. Always write unit tests,
  integration tests, and/or end-to-end tests to cover all aspects of the codebase. Use continuous
  integration to run tests automatically on every commit.
- Ensure all code is memory-safe and free of undefined behavior. Use safe concurrency and
  multithreading patterns when accessing shared data.
- Use functional programming and immutable data structures when feasible and performant.
- Always minimize mutable state and isolate it as much as possible. Avoid using mutable shared
  state.
- Whenever making a change to a code file, make sure the associated documentation is updated, and
  vice versa.
