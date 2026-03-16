# R-15.24 -- Cloud Build Service Requirements

## Build Pipeline

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.24.1 | A developer **SHALL NOT** need any platform-specific toolchain installed locally to trigger builds for that platform via the cloud build service. The cloud service **SHALL** accept build requests specifying target platform and build configuration, provision the correct toolchain, compile the project, and return downloadable artifacts. | [F-15.24.1](../../features/tools-editor/cloud-build.md) | Eliminates the requirement for every developer to install and maintain platform-specific toolchains, enabling any developer on any OS to build for any target. | Integration test: trigger builds for all 8 target platforms from a machine with no platform SDKs installed; verify all builds produce valid artifacts. |
| R-15.24.2 | The cloud build service **SHALL** support building for all 8 target platforms (macOS, Windows, Linux, iOS, Android, PlayStation, Xbox, Nintendo Switch) from any host OS (macOS, Windows, Linux). Each platform's toolchain **SHALL** run in an OCI container with pinned, reproducible tool versions tagged per engine release. | [F-15.24.2](../../features/tools-editor/cloud-build.md) | Reproducible builds require version-pinned toolchains. OCI containers provide isolation and portability across cloud and on-premise infrastructure. | Integration test: build the same project for each target platform from each host OS; verify all 24 combinations produce identical artifacts (same content hash). |
| R-15.24.3 | The cloud build service **SHALL** compile HLSL shaders to all target bytecode formats: DXIL via DXC (Windows/Xbox), SPIR-V via DXC (Vulkan/Linux), Metal IR via Metal Shader Converter (macOS/iOS), and console-specific formats via platform shader compilers. Developers **SHALL NOT** need DXC, Metal Shader Converter, or any console shader compiler installed locally to compile shaders for those targets. | [F-15.24.3](../../features/tools-editor/cloud-build.md) | Shader compilation requires platform-specific tools (Metal Shader Converter is macOS-only, console compilers are NDA-licensed). Cloud compilation removes these barriers. | Integration test: compile a shader graph to all 6+ bytecode formats from a Linux machine with no shader tools installed; verify all outputs are valid. |

## Signing and Credentials

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.24.4 | The cloud build service **SHALL** handle code signing for all platforms: Apple codesign and notarization (macOS/iOS), Android APK signing (apksigner), Windows Authenticode (signtool), and console-specific signing. Developers **SHALL NOT** need platform signing tools installed locally. | [F-15.24.4](../../features/tools-editor/cloud-build.md) | Signing tools are platform-specific (codesign is macOS-only, signtool is Windows-only). Cloud signing enables any developer to produce signed builds for any platform. | Integration test: sign builds for each platform via the cloud service from a machine without any signing tools; verify all artifacts pass platform verification. |
| R-15.24.5 | Signing credentials (certificates, keystores, provisioning profiles) **SHALL** be stored in an encrypted vault. Credentials **SHALL** never be stored in plaintext. Credentials **SHALL** never reside on developer machines. Credentials **SHALL** be decrypted only inside the build container at signing time and **SHALL NOT** be persisted to disk outside the vault. | [F-15.24.4](../../features/tools-editor/cloud-build.md) | Signing credentials are high-value secrets. Centralizing them in an encrypted vault with ephemeral decryption eliminates credential sprawl and reduces breach surface. | Security audit: verify credentials are encrypted at rest in the vault; verify credentials are not present on any developer machine; verify credentials are wiped from the container after signing. |

## Artifact Distribution

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.24.6 | For the developer's own platform, local builds **SHALL** work without the cloud service. The cloud service **SHALL** only be required for cross-platform targets where the developer lacks the necessary toolchain. The editor **SHALL** detect locally available toolchains and indicate which targets build locally vs. via cloud. | [F-15.24.6](../../features/tools-editor/cloud-build.md) | Developers should not depend on cloud connectivity for their primary platform. Local builds provide faster iteration for native-platform development. | Unit test: on a macOS machine with Xcode, verify macOS and iOS targets build locally; verify Windows and Android targets route to the cloud service. |
| R-15.24.7 | Shader compilation **SHALL** produce identical output (bit-for-bit identical bytecode) regardless of whether compiled locally or on the cloud service, given the same source hash, target platform, and tool version. | [F-15.24.3](../../features/tools-editor/cloud-build.md) | Deterministic shader output is required for cache correctness and reproducible builds. A shader compiled locally must match the cached cloud result. | Unit test: compile the same shader locally and via cloud; compare content hashes; verify they are identical for all target formats. |

## Build Visualization

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.24.8 | Build artifact storage **SHALL** use S3-compatible object storage with Zstd compression and BLAKE3 integrity verification. Mobile builds **SHALL** generate QR codes for direct device install. Console builds **SHALL** support direct deploy to connected dev kits. | [F-15.24.5](../../features/tools-editor/cloud-build.md) | S3-compatible storage works across cloud providers and on-premise (MinIO). QR codes and direct deploy minimize steps between build and test. | Integration test: download an artifact, verify BLAKE3 hash matches; scan a QR code from a phone and verify the IPA/APK installs correctly. |
| R-15.24.9 | The editor **SHALL** display real-time cloud build progress via WebSocket, showing build phase, shader compilation progress, signing status, artifact upload progress, and streaming build logs with click-to-navigate error reporting. | [F-15.24.7](../../features/tools-editor/cloud-build.md) | Real-time feedback prevents developers from context-switching to check build status. Click-to-navigate errors accelerate debugging. | Integration test: trigger a cloud build and verify the editor displays each build phase transition within 2 seconds of occurrence; introduce a compile error and verify click-to-navigate opens the correct source line. |

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/cloud-build.md](../../user-stories/tools-editor/cloud-build.md).
Requirements in this document are derived from those user stories.
