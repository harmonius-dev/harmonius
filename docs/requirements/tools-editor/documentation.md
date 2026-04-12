# R-15.19 -- Documentation and Learning Requirements

## Requirements

1. **R-15.19.1** — The engine **SHALL** auto-generate API reference documentation from Rust doc
   comments, published as a searchable static site accessible from the editor help panel.
   - **Rationale:** Auto-generation ensures documentation stays in sync with source code.
   - **Verification:** Add a doc comment, regenerate, and verify it appears in the reference.

2. **R-15.19.2** — The engine **SHALL** include inline documentation on every logic graph node with
   port descriptions, examples, and related node links.
   - **Rationale:** Node-level documentation is critical for visual authoring discoverability.
   - **Verification:** Hover over a node and verify the tooltip shows description, ports, and
     examples.

3. **R-15.19.3** — The engine **SHALL** provide interactive in-editor tutorials with spotlight
   overlays, step-by-step instructions, and community distribution via the asset store.
   - **Rationale:** Guided tutorials reduce onboarding time.
   - **Verification:** Start a tutorial, follow steps, and verify the tutorial advances on
     completion.

4. **R-15.19.4** — The engine **SHALL** embed a video player for tutorial videos with chapter links
   and a curated library.
   - **Rationale:** Video tutorials complement hands-on learning.
   - **Verification:** Play a video, click a chapter link, and verify the corresponding editor panel
     opens.

5. **R-15.19.5** — The engine **SHALL** provide contextual help on every UI element via tooltips, F1
   key, and a "What's This?" mode with localized content.
   - **Rationale:** Instant contextual help reduces reliance on external documentation.
   - **Verification:** Press F1 on an inspector field and verify the correct help page opens.

6. **R-15.19.6** — The engine **SHALL** include sample projects across genres with annotated logic
   graphs and companion walkthrough documents.
   - **Rationale:** Working examples are the most effective learning resource.
   - **Verification:** Open a sample project, build, and verify it runs correctly.

7. **R-15.19.7** — The engine **SHALL** test all documentation code examples in CI, failing the
   build when an example does not compile against the current engine version.
   - **Rationale:** Stale documentation is worse than no documentation.
   - **Verification:** Break an example intentionally and verify the CI build fails.

8. **R-15.19.8** — The engine **SHALL** store generated API documentation keyed by engine version
   with navigation between versions.
   - **Rationale:** Teams on older engine versions need documentation matching their version, not
     just the latest.
   - **Verification:** Generate docs for versions 1.0 and 1.1, navigate to a type page, switch
     versions, and verify the content reflects the selected version.
