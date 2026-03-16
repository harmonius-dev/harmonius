# User Stories: Cloud Build Service

## F-15.24.1 Cloud Build Service

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.24.1.1 | game developer (P-15) | As a game developer on Linux, I want to trigger an iOS build from my Linux workstation so that I can test my game on iPhones without owning a Mac or installing Xcode. | the iOS IPA is built, signed, and downloadable from the cloud build service | F-15.24.1 | R-15.24.1 |
| US-15.24.1.2 | designer (P-5) | As a designer, I want to trigger a PlayStation build from the editor without knowing what a console SDK is so that I can test my level designs on target hardware. | the build is submitted via a single button click; no SDK installation or configuration is required | F-15.24.1 | R-15.24.1 |
| US-15.24.1.3 | game developer (P-15) | As a game developer on Windows, I want to build for macOS and Linux from my Windows machine so that I do not need separate machines for each platform. | macOS .app and Linux AppImage artifacts are produced by the cloud service from a Windows-initiated build request | F-15.24.1 | R-15.24.1 |

## F-15.24.2 Platform Toolchain Containers

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.24.2.1 | DevOps engineer (P-16) | As a DevOps engineer, I want to set up cloud build containers for each target platform with pinned tool versions so that builds are reproducible across all developers and CI. | OCI container images are built from version-pinned definitions; the same source produces identical images on rebuild | F-15.24.2 | R-15.24.2 |
| US-15.24.2.2 | DevOps engineer (P-16) | As a DevOps engineer, I want container definitions for non-NDA platforms to be open-source so that I can audit, customize, and self-host the build infrastructure. | Dockerfiles for macOS, Windows, Linux, and Android toolchains are publicly available with build instructions | F-15.24.2 | R-15.24.2 |
| US-15.24.2.3 | DevOps engineer (P-16) | As a DevOps engineer, I want each container tagged per engine release so that I can roll back to a previous toolchain version if a new version introduces regressions. | containers are tagged with the engine version; previous tags remain available in the registry | F-15.24.2 | R-15.24.2 |

## F-15.24.3 Cross-Platform Shader Compilation

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.24.3.1 | technical artist (P-13) | As a technical artist on Windows, I want to compile my shader graphs for all platforms (Metal, Vulkan, D3D12, console) from my Windows machine so that I can verify shader correctness across all targets without switching machines. | all target bytecode formats are produced by the cloud service; compilation errors report the graph node and HLSL line | F-15.24.3 | R-15.24.3, R-15.24.7 |
| US-15.24.3.2 | game developer (P-15) | As a game developer on Linux, I want shader compilation for Metal targets to happen in the cloud so that I do not need a macOS machine just for Metal Shader Converter. | Metal IR bytecode is compiled in the cloud and cached; local Linux builds use the cached result | F-15.24.3 | R-15.24.3 |
| US-15.24.3.3 | engine developer (P-26) | As an engine developer, I want cloud-compiled shaders to be bit-for-bit identical to locally compiled shaders so that the shared build cache is consistent regardless of compilation location. | content hashes of cloud-compiled and locally compiled shaders match for the same source and tool version | F-15.24.3 | R-15.24.7 |

## F-15.24.4 Remote Code Signing

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.24.4.1 | DevOps engineer (P-16) | As a DevOps engineer, I want to upload signing credentials to an encrypted vault so that all cloud builds are automatically signed without credentials on developer machines. | credentials are encrypted at rest; decrypted only inside the build container at signing time; wiped after use | F-15.24.4 | R-15.24.4, R-15.24.5 |
| US-15.24.4.2 | game developer (P-15) | As a game developer, I want my iOS builds signed with the team's distribution profile automatically so that I can distribute test builds to testers without managing certificates myself. | the IPA is signed with the correct provisioning profile; installs on registered test devices | F-15.24.4 | R-15.24.4 |

## F-15.24.5 Build Artifact Distribution

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.24.5.1 | QA tester (P-19) | As a QA tester, I want to download an Android test build by scanning a QR code with my phone so that I can start testing a new build in seconds without USB cables or ADB commands. | the QR code links to a signed APK; scanning and installing takes under 30 seconds on a test device | F-15.24.5 | R-15.24.8 |
| US-15.24.5.2 | QA tester (P-19) | As a QA tester, I want to see build metadata (commit hash, build config, platform, duration) for each artifact so that I can file accurate bug reports tied to exact builds. | artifact metadata is displayed in the editor and included in download links | F-15.24.5 | R-15.24.8 |
| US-15.24.5.3 | game developer (P-15) | As a game developer, I want console builds deployed directly to my connected dev kit from the cloud build service so that I can test on console hardware without manual file transfer. | the cloud-built console package is deployed to the dev kit over the network; the game launches remotely | F-15.24.5 | R-15.24.8 |

## F-15.24.6 Local Development Mode

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.24.6.1 | game developer (P-15) | As a game developer on macOS, I want local builds for macOS and iOS targets without needing the cloud service so that I can iterate quickly on my primary platform. | macOS and iOS builds complete locally when Xcode is detected; no cloud dependency for native-platform builds | F-15.24.6 | R-15.24.6 |
| US-15.24.6.2 | designer (P-5) | As a designer, I want the editor to clearly indicate which targets build locally vs. via cloud so that I understand when cloud connectivity is required. | the build target selector shows a local or cloud icon next to each platform based on detected toolchains | F-15.24.6 | R-15.24.6 |

## F-15.24.7 Build Pipeline Visualization

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.24.7.1 | designer (P-5) | As a designer, I want to see real-time build progress in the editor so that I know when my cloud build will be ready without checking a separate dashboard. | build phase, progress bar, and estimated time remaining update in real-time via WebSocket | F-15.24.7 | R-15.24.9 |
| US-15.24.7.2 | game developer (P-15) | As a game developer, I want build errors to be click-to-navigate so that I can jump directly to the source file and line causing the failure. | clicking a build error in the log opens the source file at the correct line in the editor | F-15.24.7 | R-15.24.9 |
| US-15.24.7.3 | DevOps engineer (P-16) | As a DevOps engineer, I want historical build results with duration trends and success/failure rates so that I can identify infrastructure bottlenecks and flaky builds. | a build history panel shows per-platform duration trends and failure rates over time | F-15.24.7 | R-15.24.9 |
