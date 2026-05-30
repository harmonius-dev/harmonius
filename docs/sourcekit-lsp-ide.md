# SourceKit-LSP IDE Setup

## Decision

Harmonius uses SourceKit-LSP as the default language server for Swift and
SwiftPM-owned C++ interop.

SwiftPM is the editor build model. Do not add a root `compile_commands.json`,
`.clangd`, or `buildServer.json` for the default path.

The shader headers are shared by Swift, Slang, and CPU-side C++. The CPU-side
types come from `hlsl++` through the SwiftPM `hlslpp` system-library target.
Do not replace them with manual vector shims.

`HarmoniusShaders` is a small SwiftPM C++ target that owns the shader header
directory for SourceKit-LSP. It gives direct header opens and
go-to-definition a C++ target without changing app runtime products.

## Flow

1. Bootstrap the macOS host dependencies.

   ```bash
   ./scripts/dev.sh bootstrap macos
   ```

2. Warm the SwiftPM index used by SourceKit-LSP.

   ```bash
   ./scripts/dev.sh compile-spm macos debug
   ```

3. Restart SourceKit-LSP in the editor.

4. Validate Swift imports, shader headers, Slang shaders, and breakpoints.

VS Code and Cursor can run the `dev:index` task. That task delegates to the
hidden `lsp:index` task.

## Wrapper

Editors do not always inherit the same shell environment as `scripts/dev.sh`.

The `scripts/sourcekit-lsp.sh` wrapper exports the macOS arm64 pkg-config paths
needed by `hlslpp`. It then execs SourceKit-LSP from the active
Xcode toolchain with `xcrun`.

The wrapper also exposes SwiftPM's `PackagePlugin` API module from the active
toolchain. SourceKit-LSP does not index plugin targets as package source, so
opened build-tool plugin files use fallback Swift flags for editor diagnostics.

The wrapper does not bootstrap dependencies, generate shims, or build targets.
Run `dev:index` or `./scripts/dev.sh compile-spm macos debug` for that work.

## VS Code And Cursor

The workspace recommends the Swift extension and CodeLLDB.

SourceKit-LSP is configured through the current Swift extension settings:

- `swift.sourcekit-lsp.serverPath`
- `swift.sourcekit-lsp.supported-languages`
- `swift.sourcekit-lsp.backgroundIndexing`

The Swift extension language list includes Swift, C, C++, Objective-C, and
Objective-C++. Workspace file associations force
`Sources/HarmoniusShaders/*.h` to C++ because `SwiftCompat.h` intentionally
uses `hlsl++`.

Direct header routing is backed by `HarmoniusShaders` plus the
SourceKit-LSP fallback C++ flags for the generated vcpkg include directory.
This avoids a root compilation database while keeping header navigation live.

The workspace disables generated Swift launch configurations because Harmonius
owns custom CodeLLDB launch flows in `.vscode/launch.json`.

The workspace marks clangd, Microsoft C/C++, and CMake extensions as unwanted.
It also disables known clangd, C/C++, and CMake activation settings so an
installed extension does not own shader headers in this workspace.

## Zed

Zed is configured to use SourceKit-LSP for Swift files and the project wrapper
for the SourceKit-LSP binary.

Zed's Swift extension supports SourceKit-LSP and uses `lldb-dap` through the
Swift debug adapter. Zed language-server settings also allow a project-local
binary path.

Local validation showed the relative wrapper path starts SourceKit-LSP with the
expected `PKG_CONFIG_PATH` when Zed opens this worktree from the project
environment.

Zed also routes C and C++ to SourceKit-LSP for this project. Project settings
disable clangd for C and C++ so a standalone clangd server does not own shader
headers.

Use `.zed/debug.json` for the SwiftPM unit-test debug flow. It mirrors the
working `xctest` launch shape used by CodeLLDB.

## Validation

Run these command checks before handoff:

```bash
swift --version
xcrun --find sourcekit-lsp
xcrun --find lldb-dap
./scripts/dev.sh bootstrap macos
./scripts/dev.sh package-graph
./scripts/dev.sh compile-spm macos debug
./scripts/dev.sh build-tests
./scripts/dev.sh full-check
```

After restarting SourceKit-LSP, verify these editor behaviors:

- Swift files importing `HarmoniusShaders` have no false diagnostics.
- `Sources/HarmoniusShaders/ShaderTypes.h` has no false diagnostics.
- `Sources/HarmoniusShaders/SwiftCompat.h` has no false diagnostics.
- `Sources/HarmoniusShaders/Triangle.slang` has no false diagnostics.
- Swift autocomplete sees `HarmoniusShaders` declarations.
- Go-to-definition works from Swift into `ShaderTypes.h`.
- Go-to-definition works from Swift into `SwiftCompat.h`.
- Go-to-definition works from `ShaderTypes.h` into `SwiftCompat.h`.
- Go-to-definition works from `SwiftCompat.h` into vcpkg `hlsl++` headers.

Debug validation should cover:

- SwiftPM unit tests through CodeLLDB in VS Code.
- SwiftPM unit tests through CodeLLDB in Cursor.
- SwiftPM unit tests through Zed's Swift debug adapter.
- Render and Appium test builds through `scripts/dev.sh build-tests`.
- macOS app launch and iOS simulator attach where local permissions allow it.

## Local Notes

Validation on May 29, 2026 found these editor-specific notes:

- VS Code starts SourceKit-LSP through `scripts/sourcekit-lsp.sh`.
- VS Code reports `No Problems` for Swift and opened shader headers.
- VS Code CodeLLDB stops at `TriangleGeometryTests.swift:38`.
- Cursor starts SourceKit-LSP through `scripts/sourcekit-lsp.sh`.
- Cursor reports `No Problems` for Swift and opened shader headers.
- Cursor CodeLLDB stops at `TriangleGeometryTests.swift:38`.
- Zed starts SourceKit-LSP with the expected vcpkg pkg-config path.
- Zed reports no visible diagnostics for `SwiftCompat.h` or `Triangle.slang`.
- Zed's Swift debug adapter stops at `TriangleGeometryTests.swift:38`.
- SourceKit-LSP resolves `TriangleData` and `VertexData` to `ShaderTypes.h`.
- SourceKit-LSP resolves `HarmoniusFloat2` to `SwiftCompat.h` and `hlsl++`.
- SourceKit-LSP reports zero diagnostics for `SwiftCompat.h` as C or C++.

These notes do not justify a clangd fallback. The durable path is SourceKit-LSP
through SwiftPM, with vcpkg dependencies exposed through `PKG_CONFIG_PATH`.

## Fallback Policy

Do not add clangd, CMake, root `compile_commands.json`, `.clangd`, or
`buildServer.json` for Harmonius code intelligence.

Fix C++ editor failures by improving the SwiftPM package graph and the vcpkg
`pkg-config` environment that SourceKit-LSP receives.

If a future C++ need cannot be represented through SwiftPM, document the
failure first and keep any fallback outside the default IDE path.

## References

- [SourceKit-LSP README](https://github.com/swiftlang/sourcekit-lsp)
- [SourceKit-LSP config][sourcekit-config]
- [VS Code Swift docs][vscode-swift]
- [Cursor Swift docs][cursor-swift]
- [Zed Swift docs](https://zed.dev/docs/languages/swift)
- [Zed language-server config][zed-lsp-config]
- [Swift/C++ setup][swift-cxx-setup]

[cursor-swift]: https://www.swift.org/documentation/articles/getting-started-with-cursor-swift.html
[sourcekit-config]: https://github.com/swiftlang/sourcekit-lsp/tree/main/Documentation
[swift-cxx-setup]: https://www.swift.org/documentation/cxx-interop/project-build-setup/
[vscode-swift]: https://www.swift.org/documentation/articles/getting-started-with-vscode-swift.html
[zed-lsp-config]: https://zed.dev/docs/configuring-languages
