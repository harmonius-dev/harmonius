# R-15.24 -- Cloud Build Service Requirements

## Requirements

1. **R-15.24.1** — The engine **SHALL** provide a remote build pipeline compiling, signing, and
   packaging for all target platforms from any host OS, submittable from the editor UI or CLI.
   - **Rationale:** Cross-platform builds without local toolchains reduce developer setup burden.
   - **Verification:** Submit a build from a macOS host targeting Windows and verify a signed .exe
     is produced.

2. **R-15.24.2** — The engine **SHALL** maintain OCI container definitions per platform toolchain
   with pinned tool versions, tagged per engine release.
   - **Rationale:** Reproducible build environments eliminate toolchain drift.
   - **Verification:** Build the same commit twice in the same container and verify identical output
     hashes.

3. **R-15.24.3** — The engine **SHALL** compile GLSL shaders to all target formats (SPIR-V, SPIR-V,
   SPIR-V) in the cloud with output stored in the shared build cache.
   - **Rationale:** Cloud shader compilation eliminates local toolchain requirements.
   - **Verification:** Compile a shader in the cloud and verify the cache entry is reused by a local
     build.

4. **R-15.24.4** — The engine **SHALL** handle code signing for all platforms in the cloud with
   credentials stored in an encrypted vault, decrypted only inside build containers.
   - **Rationale:** Secure automated signing is required for distribution.
   - **Verification:** Sign a build and verify credentials are not present on disk after the
     container exits.

5. **R-15.24.5** — The engine **SHALL** store build artifacts in S3-compatible storage with
   configurable retention, BLAKE3 integrity, and QR code download links for mobile builds.
   - **Rationale:** Reliable artifact storage and easy mobile install accelerate testing.
   - **Verification:** Download an artifact, verify its hash, and scan a QR code to install on a
     phone.

6. **R-15.24.6** — The engine **SHALL** support local builds for the developer's own platform
   without the cloud service, with clear UI indication of local versus cloud targets.
   - **Rationale:** Offline development must remain possible.
   - **Verification:** Disconnect from the network, build for the local platform, and verify
     success.

7. **R-15.24.7** — The engine **SHALL** display real-time cloud build progress via WebSocket with
   phase, shader progress, signing status, estimated time, and historical trend data.
   - **Rationale:** Build visibility enables monitoring and diagnosing slow or failed builds.
   - **Verification:** Submit a build and verify the status panel updates in real time as each phase
     completes.
