# Swift Coding Standard

## Scope

Apple platform wrappers only. Swift wraps macOS/iOS APIs (Metal, NSWindow, GCD) and exposes them to
Rust as C ABI functions via `@_cdecl`. Rust consumes the generated C headers through bindgen. Built
with CMake. No SwiftUI, no Combine, no Objective-C, no Objective-C++, no cxx.rs, no metal-cpp.

## Naming Conventions

| Element | Convention | Example |
|---------|-----------|---------|
| Type (class, struct, enum, protocol) | `PascalCase` | `WindowManager` |
| Function / method | `camelCase` | `createWindow` |
| Variable / property | `camelCase` | `frameCount` |
| Constant | `camelCase` | `maxBufferSize` |
| Static constant | `camelCase` | `defaultConfig` |
| Enum case | `camelCase` | `metalBackend` |
| Protocol | `PascalCase` + `-able`/`-ing`/`-Protocol` | `Renderable` |

## File Organization

- One type per file
- File name matches primary type name
- Extensions in `{Type}+{Protocol}.swift`
- Group by platform module

## Formatting Rules

- 100-character line limit
- 4-space indentation
- `swift-format` with project configuration
- Trailing commas in multi-line collections

## Linting Rules

- `swiftlint` with project `.swiftlint.yml`
- Treat all warnings as errors
- Force unwrap (`!`) forbidden in production code

## Type Checking

- `swiftc -typecheck` must pass
- Strict concurrency checking enabled
- All public types must have doc comments

## How to Check and Fix

| Validation | Check command | Fix command |
|------------|--------------|-------------|
| Format | `swift-format lint` | `swift-format format -i` |
| Lint | `swiftlint` | `swiftlint --fix` |
| Type-check | `swiftc -typecheck` | (manual) |
| Test | `ctest` (via CMake) | (manual) |
| Build | `cmake --build` | (manual) |

## Project-Specific Rules

1. **Apple platform wrappers only** — no application logic in Swift
2. **No SwiftUI** — use AppKit/UIKit directly
3. **No Combine** — use GCD and completion handlers at the Swift layer
4. **C ABI via `@_cdecl`** — all Swift functions exposed to Rust use `@_cdecl` for C linkage
5. **CMake for libraries** — Swift libraries built with CMake, not SwiftPM
6. **XcodeGen for apps** — macOS/iOS app packaging uses XcodeGen + xcodebuild
7. **GCD for concurrency** — dispatch queues, not Swift concurrency (`async`/`await` in Swift)
8. **No Objective-C** — no `@objc`, no Objective-C bridging headers, no Objective-C++
9. **Metal directly from Swift** — Metal accessed directly via Swift APIs, exposed to Rust via C ABI
10. **No cxx.rs** — no C++ interop; all FFI through C ABI
11. **No metal-cpp** — no C++ Metal wrappers

## Cache-Friendly Patterns

- **Value types** — prefer `struct` over `class` for data passed across FFI boundaries
- **Contiguous arrays** — use `Array` and `UnsafeBufferPointer` for bulk data
- **Avoid ARC overhead** — minimize reference counting by using value types and `unowned`/`weak`
  sparingly
- **Batch API calls** — group platform API calls to reduce bridge overhead
- **Pre-allocate buffers** — reuse `Data` and array buffers instead of reallocating

## Testing

1. **Test-driven development** — write tests first, driven by requirements (R-X.Y.Z) and features
   (F-X.Y.Z)
2. **Testability in design** — keep platform wrappers testable by isolating API calls
3. **Use real dependencies** — test against real Apple APIs where possible
4. **NO MOCKING** — no mock libraries; no mock objects
5. **Fakes only when necessary** — write full fakes conforming to the same protocol
6. **Protocols for contracts** — define protocols at module boundaries for testability
7. **Enums for state** — prefer enums over booleans
8. **XCTest** — use XCTest for all Swift tests

## Best Practices

1. Use `let` by default; `var` only when mutation is required
2. Use `guard` for early returns
3. Prefer `enum` with associated values over class hierarchies
4. Never use `@objc` — no Objective-C interop
5. Mark classes as `final` unless inheritance is needed
6. Use `Result<T, Error>` for fallible operations
7. Document all public API with `///` comments
8. Use `typealias` for complex generic signatures
9. Use access control (`private`, `internal`, `public`) explicitly
10. Keep functions under 40 lines

## Anti-Patterns

1. **Application logic in Swift** — all logic belongs in Rust
2. **Force unwrap (`!`)** — use `guard let` or `if let`
3. **Implicit `self`** — always capture `[weak self]` or `[unowned self]` in closures
4. **SwiftUI/Combine** — not used in this project
5. **Global state** — no singletons or global mutable variables
6. **Deep class hierarchies** — use protocols and composition
7. **Stringly-typed APIs** — use enums and typed IDs
8. **Unstructured concurrency** — use GCD dispatch queues, not `Task` or `TaskGroup`
9. **Large files** — split into extensions by protocol conformance
10. **Ignoring errors** — handle all errors explicitly
