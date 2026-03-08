# Agent Handbook for Harmonius

This handbook provides guidelines and best practices for agents working on the Harmonius project.

## Rulebook

- Put all documentation in the `docs/` directory. Use Markdown format for all documentation.
- Never create ASCII diagrams. Instead, create Mermaid diagrams. Always render Mermaid diagrams with
  the MCP to ensure correctness and readability.

## Design guidelines

- Design abstractions to be modular and reusable. Separate concerns between components and modules.
- Use domain-driven design to model the problem space and create a ubiquitous language. Define clear
  boundaries between domains and use interfaces to decouple them.
- Seek approval before adding or changing any dependencies.
- Always use the latest versions of dependencies and tools. Use the package manager CLI to install
  dependencies with the latest version automatically.

## Development guidelines

- Write implementation comments sparingly. Always write documentation comments and generate an API
  reference from them. Use implementation comments only for complex algorithms or non-obvious code.
- Use test-driven development to ensure code quality and correctness. Write unit tests, integration
  tests, and end-to-end tests to cover all aspects of the codebase. Use continuous integration to
  run tests automatically on every commit.
- Ensure all code is memory-safe and free of undefined behavior. Use safe concurrency and
  multithreading patterns when accessing shared data.
- Use functional programming and immutable data structures when feasible and performant.
- Always minimize mutable state and isolate it as much as possible. Avoid using mutable shared
  state.
