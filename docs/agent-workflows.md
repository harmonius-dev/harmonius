# Agent Workflows

LLM agents must use the Bash workflow as the public interface. Do not recreate
dependency order inside prompts, VS Code tasks, CI YAML, or ad hoc shell blocks.

No JavaScript or TypeScript may be added for test code, project scripts, or UI
automation. Appium UI tests are Swift tests using `swift-webdriver`.

## Workflow Rules

1. Start from a public VS Code task or a documented `scripts/dev.sh` command.
2. Let Bash bootstrap submodules, vcpkg triplets, and pkg-config shims.
3. Use SwiftPM for package graph, package builds, shader artifacts, and tests.
4. Use XcodeGen only for Apple bundles, signing, resources, schemes, and archives.
5. Prefer hidden VS Code tasks only as dependencies of public aggregate tasks.
6. Run `./scripts/dev.sh full-check` before handing off substantial changes.

## Skill Cards

### bootstrap

- Public task: `dev:bootstrap`
- Command: `./scripts/dev.sh bootstrap macos`
- Preconditions: Git is available and the vcpkg submodule can be initialized.
- Expected output: vcpkg installs the host triplet and pkg-config shims validate.
- Failure triage: run `git submodule update --init --recursive`, then retry.

### package-migration

- Public task: `dev:compile`
- Command: `./scripts/dev.sh compile-spm macos debug`
- Preconditions: `bootstrap` has completed for the host platform.
- Expected output: SwiftPM resolves, builds Swift targets, and compiles plugins.
- Failure triage: run `./scripts/dev.sh package-graph`, then inspect target names.

### shader-pipeline

- Public task: `dev:compile`
- Command: `./scripts/dev.sh compile-spm macos debug`
- Preconditions: `shader-slang` is installed by vcpkg for the host triplet.
- Expected output: `HarmoniusShaderPlugin` emits `default.metallib` and
  generated `ShaderTypes.swift` from Slang reflection.
- Failure triage: run `./scripts/dev.sh package-graph`, then inspect the Slang
  bridge diagnostics from `HarmoniusShaderTool`.
- Ownership rule: hand-authored shader code lives only in `.slang` files under
  `Sources/HarmoniusShaders/`.

### xcode-bundle

- Public task: `dev:compile`
- Command: `./scripts/dev.sh bundle macos debug`
- Preconditions: Xcode 26 and XcodeGen are installed on macOS.
- Expected output: `build/xcodegen/Build/Products/Debug/HarmoniusApp.app`.
- Failure triage: run `./scripts/dev.sh xcodegen`, then rerun the bundle command.

### appium-ui

- Public task: `dev:test`
- Command: `./scripts/dev.sh test-ui-macos`
- Preconditions: Appium is installed with the `mac2` and `xcuitest` drivers.
- Expected output: Swift Appium tests create a session and capture a screenshot.
- Failure triage: run `./scripts/dev.sh appium-bootstrap` and inspect Appium logs.

### snapshot-recording

- Public task: `dev:test`
- Command: `./scripts/dev.sh test-render-record`
- Preconditions: Metal is available and the shader plugin can build `default.metallib`.
- Expected output: render snapshots refresh under `Tests/HarmoniusRenderTests/`.
- Failure triage: compare render size `960x540` and snapshot size `480x270`.

### debugging

- Public task: `dev:debug:macos`
- Command: `./scripts/dev.sh debug macos`
- Preconditions: CodeLLDB is installed in VS Code and Xcode can build the app.
- Expected output: `build/debug-macos-app` points at the current debug bundle.
- Failure triage: rerun `./scripts/dev.sh bundle macos debug` outside VS Code.

### quality-gate

- Public task: `dev:full-check`
- Command: `./scripts/dev.sh full-check`
- Preconditions: `swift-format`, `jq`, SwiftPM, vcpkg, and Xcode tools are ready.
- Expected output: formatting, line length, JSON sorting, no-JS, build, and tests pass.
- Failure triage: run the first failing `scripts/dev.sh` subcommand directly.

### release-validation

- Public task: `release:validate`
- Command: `./scripts/dev.sh release-validate`
- Preconditions: Apple signing and App Store Connect environment variables are set.
- Expected output: required release secrets exist and the package graph resolves.
- Failure triage: populate the missing environment variable named by the command.

## Public VS Code Tasks

The public task list is intentionally small:

- `dev:bootstrap`
- `dev:format`
- `dev:compile`
- `dev:test`
- `dev:run:macos`
- `dev:debug:macos`
- `dev:run:ios-simulator`
- `dev:debug:ios-simulator`
- `dev:full-check`
- `archive:ios`
- `archive:macos`
- `release:validate`

Low-level tasks for vcpkg, pkg-config, SwiftPM, XcodeGen, bundles, Appium, and
individual test groups are hidden in `.vscode/tasks.json`.
