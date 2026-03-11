# Agent Handbook for Harmonius

This handbook provides guidelines and best practices for agents working on the Harmonius project.

## Rulebook

- Put all documentation in the `docs/` directory. Use Markdown format for all documentation.
- Never create ASCII diagrams. Instead, create Mermaid diagrams. Always render Mermaid diagrams with the MCP to ensure
  correctness and readability.
- Always limit lines to 120 characters for all code, configuration, documentation, and comments. The limit may be
  exceeded if the line cannot be split without changing semantics. The line length limit does not apply to tables in
  Markdown documents.
- Always sort JSON documents by keys in lexicographical order. Do not sort JSON arrays, only JSON objects.

## Design guidelines

- Design abstractions to be modular and reusable. Separate concerns between components and modules. Prefer composition
  over inheritance.
- Always prefer static dispatch via C++20 concepts. Do not use virtual methods, vtables, abstract base classes, or
  dynamic dispatch unless necessary for runtime polymorphism.
- Use domain-driven design to model the problem space and create a ubiquitous language. Define clear boundaries between
  domains and use interfaces to decouple them.
- Seek approval before adding or changing any dependencies. If you need to add a dependency, propose a low-level
  library that is well supported and maintained, and can be accessed through vcpkg. Avoid using any frameworks.
- Always use the latest versions of dependencies and tools. Use the package manager CLI to install dependencies with the
  latest version automatically.
- Write multiplatform code that works consistently across supported operating systems. Use platform-agnostic libraries
  and tools when possible. Ensure all cross-platform interfaces are compatible with each supported platform, or use
  gating to expose platform-specific functionality.
- Do not use file I/O from the C++ standard library. Use platform-native async I/O instead: I/O completion ports on
  Windows, dispatch_async I/O (Grand Central Dispatch) on macOS, and io_uring on Linux.

## Development guidelines

- Write implementation comments sparingly. Always write documentation comments and generate an API reference from them.
  Use implementation comments only for complex algorithms or non-obvious code.
- Use test-driven development to ensure code quality and correctness. Write unit tests, integration tests, and
  end-to-end tests to cover all aspects of the codebase. Use continuous integration to run tests automatically on every
  commit.
- Ensure all code is memory-safe and free of undefined behavior. Use safe concurrency and multithreading patterns when
  accessing shared data.
- Use functional programming and immutable data structures when feasible and performant.
- Always minimize mutable state and isolate it as much as possible. Avoid using mutable shared state.
- Whenever making a change to a code file, make sure the associated documentation is updated, and vice versa.
