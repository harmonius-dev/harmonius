# User Stories -- 15.24 Cloud Build Service

## Stories

| ID           | Persona                 |
|--------------|-------------------------|
| US-15.24.1.1 | build engineer (P-16)   |
| US-15.24.1.2 | game designer (P-5)     |
| US-15.24.2.1 | build engineer (P-16)   |
| US-15.24.2.2 | engine developer (P-26) |
| US-15.24.3.1 | technical artist (P-13) |
| US-15.24.3.2 | build engineer (P-16)   |
| US-15.24.4.1 | build engineer (P-16)   |
| US-15.24.4.2 | engine developer (P-26) |
| US-15.24.5.1 | build engineer (P-16)   |
| US-15.24.5.2 | game designer (P-5)     |
| US-15.24.6.1 | engine developer (P-26) |
| US-15.24.6.2 | game designer (P-5)     |
| US-15.24.7.1 | game designer (P-5)     |
| US-15.24.7.2 | build engineer (P-16)   |

1. **US-15.24.1.1** — **As a** build engineer (P-16), **I want** a remote build pipeline compiling,
   signing, and packaging for all target platforms from any host OS, **so that** developers never
   install cross-platform toolchains locally.

2. **US-15.24.1.2** — **As a** game designer (P-5), **I want** to submit build requests from the
   editor UI specifying target platforms and configuration, **so that** I get builds without
   command-line tools.

3. **US-15.24.2.1** — **As a** build engineer (P-16), **I want** OCI container definitions for each
   platform toolchain with pinned versions and reproducible builds, **so that** builds are
   deterministic.

4. **US-15.24.2.2** — **As a** engine developer (P-26), **I want** containers versioned and tagged
   per engine release, **so that** toolchain versions match the engine version.

5. **US-15.24.3.1** — **As a** technical artist (P-13), **I want** cloud shader compilation to all
   target bytecode formats without local toolchains, **so that** I do not install DXC or Metal
   Shader Converter.

6. **US-15.24.3.2** — **As a** build engineer (P-16), **I want** cloud-compiled shaders stored in
   the shared build cache, **so that** other developers and CI reuse them.

7. **US-15.24.4.1** — **As a** build engineer (P-16), **I want** cloud code signing for all
   platforms with credentials in an encrypted vault, **so that** signing is automated and secure.

8. **US-15.24.4.2** — **As a** engine developer (P-26), **I want** credentials decrypted only inside
   build containers, **so that** signing keys are never exposed.

9. **US-15.24.5.1** — **As a** build engineer (P-16), **I want** build artifacts stored in S3 with
   configurable retention and BLAKE3 integrity verification, **so that** artifacts are reliable and
   manageable.

10. **US-15.24.5.2** — **As a** game designer (P-5), **I want** QR codes for mobile builds and
    direct devkit deploy for console builds, **so that** I can test from my phone.

11. **US-15.24.6.1** — **As a** engine developer (P-26), **I want** local builds for my own platform
    without the cloud service, **so that** development works offline.

12. **US-15.24.6.2** — **As a** game designer (P-5), **I want** the editor to clearly indicate which
    targets build locally versus via cloud, **so that** I understand my build options.

13. **US-15.24.7.1** — **As a** game designer (P-5), **I want** a real-time build status panel
    showing phase, shader progress, signing status, and estimated time remaining, **so that** I can
    monitor cloud builds.

14. **US-15.24.7.2** — **As a** build engineer (P-16), **I want** historical build results with
    duration trends and success/failure rates, **so that** I can track build health.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-15.24.1 | build engineer (P-16) |
| US-15.24.2 | build engineer (P-16) |
| US-15.24.3 | technical artist (P-13) |
| US-15.24.4 | build engineer (P-16) |
| US-15.24.5 | build engineer (P-16) |
| US-15.24.6 | engine developer (P-26) |
| US-15.24.7 | game designer (P-5) |

1. **US-15.24.1** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.24.1.1 through US-15.24.1.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

2. **US-15.24.2** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.24.2.1 through US-15.24.2.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

3. **US-15.24.3** -- **As a** technical artist (P-13), **I want** the capabilities defined in
   sub-stories US-15.24.3.1 through US-15.24.3.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

4. **US-15.24.4** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.24.4.1 through US-15.24.4.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

5. **US-15.24.5** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.24.5.1 through US-15.24.5.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

6. **US-15.24.6** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-15.24.6.1 through US-15.24.6.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

7. **US-15.24.7** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-15.24.7.1 through US-15.24.7.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.
