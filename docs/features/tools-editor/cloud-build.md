# 15.24 — Cloud Build Service

## Cloud Build Pipeline

| ID        | Feature                           | Requirements         |
|-----------|-----------------------------------|----------------------|
| F-15.24.1 | Cloud Build Service               | R-15.24.1            |
| F-15.24.2 | Platform Toolchain Containers     | R-15.24.2            |
| F-15.24.3 | Cross-Platform Shader Compilation | R-15.24.3, R-15.24.7 |
| F-15.24.4 | Remote Code Signing               | R-15.24.4, R-15.24.8 |
| F-15.24.5 | Build Artifact Distribution       | R-15.24.5            |
| F-15.24.6 | Local Development Mode            | R-15.24.6            |
| F-15.24.7 | Build Pipeline Visualization      | R-15.24.9            |

1. **F-15.24.1** — Remote build pipeline that compiles, signs, and packages for ALL target platforms
   from any developer machine. A developer on any OS submits a build request via the editor UI or
   CLI, specifying target platform(s) and build configuration (debug, development, shipping). The
   cloud service provisions a build environment with the correct toolchain, compiles the project,
   signs artifacts, and returns downloadable packages. Developers never install platform-specific
   toolchains for cross-platform targets. Local builds remain available for the developer's own
   platform without the cloud service.
   - **Deps:** F-15.14.1 (Platform Build Packaging), F-15.11.1 (Shared Build Cache), F-1.8.4 (Async
     Network I/O)
   - **Platform:** Build requests are submitted over HTTPS. Build environments run in OCI containers
     on cloud or on-premise infrastructure.
2. **F-15.24.2** — Docker/OCI container definitions for each target platform's toolchain. Each
   container bundles the complete build environment: macOS/Xcode (shader compiler, codesign,
   notarization), Windows/MSVC+DXC (cl.exe, link.exe, DXC), Android/NDK+Gradle (NDK, SDK, Gradle,
   apksigner), Linux/Clang (clang, lld, sysroot), and console-specific containers (under NDA, not
   open-sourced). Container definitions for non-NDA platforms are maintained as open-source OCI
   image definitions with pinned tool versions and reproducible builds. Containers are versioned and
   tagged per engine release.
   - **Deps:** F-15.24.1 (Cloud Build Service)
   - **Platform:** macOS containers require macOS hosts (Virtualization.framework or bare metal).
     Console containers require licensed SDK access per manufacturer agreement.
3. **F-15.24.3** — Cloud service compiles HLSL shaders to all target bytecode formats without
   requiring any shader toolchain locally. DXC compiles HLSL to DXIL (Windows/Xbox) and SPIR-V
   (Vulkan/Linux). Metal Shader Converter translates DXIL to Metal IR (macOS/iOS). Console-specific
   shader compilers produce platform-native bytecode. Shader compilation output is identical whether
   compiled locally or on the cloud service. Compiled shaders are stored in the shared build cache
   (F-15.11.2) for reuse across developers and CI.
   - **Deps:** F-12.2.9 (DXC and Metal Shader Converter Pipeline), F-15.11.2 (Shader Compilation
     Cache), F-12.2.7 (HLSL Code Generation)
   - **Platform:** Metal Shader Converter runs only on macOS. DXC is open source and runs on
     Windows, macOS, and Linux. Console shader compilers require NDA toolchains.
4. **F-15.24.4** — Cloud service handles platform code signing for all targets. Apple: codesign with
   Developer ID certificates and submit for notarization with ticket stapling. iOS: sign with
   development or distribution provisioning profiles. Android: sign APK/AAB with release keystores
   via apksigner. Windows: Authenticode sign via signtool. Console: platform-specific certification
   signing per manufacturer requirements. Developers upload signing credentials (certificates,
   keystores, provisioning profiles) to an encrypted vault; credentials are decrypted only inside
   the build container at signing time and never persisted to disk outside the vault.
   - **Deps:** F-15.14.4 (Code Signing Pipeline), F-15.24.1 (Cloud Build Service)
   - **Platform:** Apple notarization requires internet access from the build container. Windows EV
     signing may require HSM-backed keys. Console signing requires manufacturer-issued credentials.
5. **F-15.24.5** — Compiled builds are stored in S3-compatible object storage with configurable
   retention. The editor and CLI provide download links for each build artifact. Mobile builds (iOS
   IPA, Android APK) generate QR codes scannable from a phone for direct install. Console builds
   support direct deploy to connected dev kits over the network. Artifact metadata includes build
   configuration, target platform, commit hash, build duration, and artifact size. Artifacts are
   compressed with Zstd and integrity-verified via BLAKE3 hashes.
   - **Deps:** F-15.24.1 (Cloud Build Service), F-15.11.6 (Cache Transport and Storage), F-15.14.2
     (Deploy-to-Device)
   - **Platform:** S3-compatible storage includes AWS S3, MinIO, GCS (S3-compatible mode), and Azure
     Blob (S3-compatible mode).
6. **F-15.24.6** — For the developer's own platform, local builds work without the cloud service.
   The engine detects locally available toolchains and uses them for native-platform builds. The
   cloud service is only required for cross-platform targets where the developer lacks the necessary
   toolchain. A developer on macOS builds macOS and iOS targets locally (Xcode is available) but
   uses the cloud for Windows, Linux, Android, and console targets. A developer on Linux builds
   Linux targets locally but uses the cloud for all other platforms. The editor UI clearly indicates
   which targets build locally vs. via cloud.
   - **Deps:** F-15.24.1 (Cloud Build Service), F-15.14.1 (Platform Build Packaging)
   - **Platform:** Toolchain detection uses platform-native discovery: `xcrun` on macOS, `vswhere`
     on Windows, `clang --version` on Linux.
7. **F-15.24.7** — The editor shows real-time build progress for cloud builds via WebSocket
   connection to the build service. A build status panel displays: overall build phase (queued,
   compiling, linking, signing, packaging), shader compilation progress (N/M variants compiled per
   platform), code signing status, artifact upload progress, and estimated time remaining. Build
   logs stream in real-time to the editor's console panel. Failed builds highlight the error with
   click-to-navigate to the source file and line. Historical build results are browsable with
   duration trends and success/failure rates.
   - **Deps:** F-15.24.1 (Cloud Build Service), F-15.1.1 (Editor Framework), F-1.8.4 (Async Network
     I/O)
   - **Platform:** WebSocket connection uses platform-native networking: NSURLSessionWebSocketTask
     on macOS, WinHTTP WebSockets on Windows, libwebsockets on Linux.

## Platform Toolchain Coverage

| Toolchain                     | Platform Restriction                |
|-------------------------------|-------------------------------------|
| Metal Shader Converter        | macOS only                          |
| Metal developer tools         | macOS only                          |
| DXC (DirectX Shader Compiler) | Windows, macOS, Linux (open source) |
| Xcode toolchain               | macOS only                          |
| Android NDK + Gradle          | Any (complex setup)                 |
| Console shader compilers      | NDA, licensed                       |
| Platform code-signing         | Licensed                            |
| MSVC (cl.exe, link.exe)       | Windows only                        |

1. **Metal Shader Converter** — Shader compilation for iOS/macOS targets
2. **Metal developer tools** — Metal validation, GPU profiling data
3. **DXC (DirectX Shader Compiler)** — HLSL to DXIL and SPIR-V for Windows/Xbox/Vulkan
4. **Xcode toolchain** — iOS/macOS app signing, notarization
5. **Android NDK + Gradle** — Android APK/AAB builds
6. **Console shader compilers** — Platform-specific shader compilation
7. **Platform code-signing** — Signing for distribution
8. **MSVC (cl.exe, link.exe)** — Windows .exe/.dll compilation
