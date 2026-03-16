# User Stories: AI Editor Assistant

## F-15.9.1a Speech-to-Text Pipeline

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.9.1a.1 | designer (P-5) | live voice input captured by my microphone and processed through a speech-to-text pipeline with word-level timestamps | I can issue voice commands hands-free while working in the viewport |  |  |
| US-15.9.1a.2 | developer (P-15) | speech-to-text processing on a configurable backend (local or remote) | studios can choose between on-premise processing for privacy or cloud processing for accuracy |  |  |
| US-15.9.1a.3 | engine developer (P-26) | microphone capture using platform-native audio APIs (CoreAudio on macOS, WASAPI on Windows, PipeWire on Linux) | voice input works reliably on all supported platforms |  |  |
| US-15.9.1a.4 | engine tester (P-27) | to verify that transcription results are streamed with word-level timestamps at low latency | voice commands feel responsive |  |  |

## F-15.9.1b Voice Command Interpretation

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.9.1b.1 | designer (P-5) | transcribed voice input interpreted by an LLM agent that translates natural language into editor tool invocations (e.g., "place a directional light facing east at 45 degrees") | I can manipulate scenes without navigating menus |  |  |
| US-15.9.1b.2 | artist (P-8) | the assistant to maintain conversational context so follow-up commands like "rotate it 10 degrees north" reference the previously created or selected object | I can work naturally without repeating context |  |  |
| US-15.9.1b.3 | creative director (P-2) | to issue high-level verbal directions ("make this area darker", "add fog to the valley") | I can guide art direction during reviews without touching the editor controls |  |  |
| US-15.9.1b.4 | engine tester (P-27) | to verify that a standard set of voice commands are parsed correctly into the expected tool invocations | voice control is reliable for common operations |  |  |

## F-15.9.1c Voice Activation Modes

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.9.1c.1 | designer (P-5) | push-to-talk activation with a configurable key | the microphone only activates when I intend to speak a command |  |  |
| US-15.9.1c.2 | artist (P-8) | an always-listening mode with wake word or silence detection | I can issue commands without pressing any key while my hands are on a pen tablet |  |  |
| US-15.9.1c.3 | tech artist (P-13) | voice activation mode stored per-user in editor preferences | each team member can choose the mode that suits their work environment |  |  |
| US-15.9.1c.4 | engine tester (P-27) | to verify that switching between push-to-talk and always-listening modes works without restarting the editor | users can change modes mid-session |  |  |

## F-15.9.2 AI Assistant Tool Interface

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.9.2.1 | designer (P-5) | the assistant to access the full editor tool API (scene manipulation, asset management, graph editing, material adjustment) | any editor action I can perform manually can also be done via voice or text command |  |  |
| US-15.9.2.2 | developer (P-15) | all assistant-driven actions to participate in the standard undo/redo stack | AI-initiated changes can be reverted just like manual edits |  |  |
| US-15.9.2.3 | tech artist (P-13) | tool definitions generated from the same metadata as the editor plugin API | the assistant's capabilities stay in sync with available editor features |  |  |
| US-15.9.2.4 | engine tester (P-27) | to verify that every user-facing editor feature has a corresponding tool definition accessible to the assistant | no editor capability is missing from the AI tool interface |  |  |

## F-15.9.3 Visual and Graphical Tool Access

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.9.3.1 | artist (P-8) | the assistant to adjust color parameters in material editors, move nodes in logic graphs, and reshape splines in the viewport | I can direct visual changes verbally while focusing on the viewport |  |  |
| US-15.9.3.2 | designer (P-5) | the assistant to paint terrain heightmaps and splat maps using the same input-action APIs as mouse input | AI-driven painting follows the same snapping and constraint rules as manual painting |  |  |
| US-15.9.3.3 | tech artist (P-13) | all visual manipulations performed by the assistant recorded as undoable commands | I can undo AI-driven edits just like manual ones |  |  |
| US-15.9.3.4 | engine tester (P-27) | to verify that assistant visual manipulations produce identical results to manual mouse/keyboard input | AI editing is consistent with human editing |  |  |

## F-15.9.4 Keyboard Shortcut Recommendations

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.9.4.1 | designer (P-5) | the assistant to display a subtle tooltip showing the keyboard shortcut when I perform an action through menus | I gradually learn faster workflows without interrupting my current task |  |  |
| US-15.9.4.2 | artist (P-8) | shortcut recommendations shown at most once per shortcut per session and only when I repeat the slower interaction path | recommendations are helpful rather than annoying |  |  |
| US-15.9.4.3 | developer (P-15) | to disable shortcut recommendations globally or per- category in preferences | experienced users are not bothered by tips they already know |  |  |
| US-15.9.4.4 | engine tester (P-27) | shortcut display to adapt to platform modifier key conventions (Cmd on macOS, Ctrl on Windows/Linux) | recommendations are correct on each platform |  |  |

## F-15.9.5 Contextual Action Reminders

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.9.5.1 | designer (P-5) | "how do I..." queries answered with step-by-step visual overlays that highlight relevant UI elements in sequence | I can learn workflows by following guided visual instructions |  |  |
| US-15.9.5.2 | artist (P-8) | overlays to fade after I complete the guided action or explicitly dismiss them | help overlays do not permanently obscure my workspace |  |  |
| US-15.9.5.3 | modder (P-24) | contextual action reminders that work in the mod SDK editor | I can learn the subset of available tools without separate documentation |  |  |
| US-15.9.5.4 | engine tester (P-27) | to verify that overlay highlights target the correct UI widgets regardless of panel layout | guided instructions work with any workspace configuration |  |  |

## F-15.9.6a Headless Editor API Layer

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.9.6a.1 | DevOps engineer (P-16) | a headless API layer that mirrors the UI interaction model without a visible display | I can automate editor operations in CI/CD pipelines |  |  |
| US-15.9.6a.2 | developer (P-15) | the headless layer to expose low-level UI automation primitives (opening panels, clicking buttons, entering values) | I can write automated test scripts for editor features |  |  |
| US-15.9.6a.3 | engine developer (P-26) | headless mode to use a virtual framebuffer (EGL on Linux, headless Metal on macOS) for viewport operations | headless automation works on CI servers without a display |  |  |
| US-15.9.6a.4 | engine tester (P-27) | to verify that headless API operations produce identical results to interactive UI operations | automated tests are reliable proxies for manual testing |  |  |

## F-15.9.6b Agent Orchestration

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.9.6b.1 | DevOps engineer (P-16) | multiple concurrent AI agents with isolated command contexts (independent undo stacks, selection state, tool history) | parallel automation tasks do not interfere with each other |  |  |
| US-15.9.6b.2 | developer (P-15) | agent lifecycle management (create, run, terminate) through the headless API | I can programmatically control automation workflows |  |  |
| US-15.9.6b.3 | engine developer (P-26) | agents unable to observe or modify each other's contexts | concurrent agents cannot cause race conditions or data corruption |  |  |
| US-15.9.6b.4 | engine tester (P-27) | to verify that concurrent agents operate in fully isolated contexts with no shared mutable state | parallel automation is safe and deterministic |  |  |

## F-15.9.6c CI/CD Agent Integration

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.9.6c.1 | DevOps engineer (P-16) | agents to integrate with CI/CD pipelines for automated content generation and regression testing | repetitive asset production runs unattended as part of the build process |  |  |
| US-15.9.6c.2 | developer (P-15) | agents to produce build artifacts and test reports without human interaction | automated builds include content validation |  |  |
| US-15.9.6c.3 | server admin (P-22) | agent job status and results reported through CI dashboards | I can monitor automated content generation alongside other build steps |  |  |
| US-15.9.6c.4 | engine tester (P-27) | to verify that headless agents on CI servers with virtual framebuffers produce the same results as interactive sessions | CI-generated content matches developer expectations |  |  |

## F-15.9.7 Screenshot and Screen Recording Capture

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.9.7.1 | artist (P-8) | the assistant to capture screenshots of the viewport or specific panels | I can create documentation and bug reports without third-party tools |  |  |
| US-15.9.7.2 | tech artist (P-13) | captured screenshots fed into the AI's visual understanding pipeline | the assistant can analyze a screenshot to diagnose lighting or material setup issues |  |  |
| US-15.9.7.3 | developer (P-15) | screen recordings saved to disk with configurable resolution and format | I can attach reproduction videos to bug reports |  |  |
| US-15.9.7.4 | engine tester (P-27) | to verify that screen capture requests proper OS permissions (TCC on macOS, DXGI on Windows, PipeWire on Linux) | capture works without security violations |  |  |

## F-15.9.8 UI Accessibility Tree Analysis

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.9.8.1 | designer (P-5) | the assistant to read the UI accessibility tree to understand which panel is active, what is selected, and which tool mode is engaged | the AI responds accurately to my context without relying on screenshot analysis alone |  |  |
| US-15.9.8.2 | developer (P-15) | the accessibility tree to integrate with platform-native APIs (NSAccessibility on macOS, UIA on Windows, AT-SPI2 on Linux) | the AI leverages the same accessibility infrastructure as screen readers |  |  |
| US-15.9.8.3 | engine developer (P-26) | the editor's custom widget toolkit to expose accessibility metadata (widget type, text label, value, enabled state, hierarchy) on all platforms | the accessibility tree is complete and accurate |  |  |
| US-15.9.8.4 | engine tester (P-27) | to verify that every editor widget is represented in the accessibility tree | the AI assistant and screen readers both have complete interface information |  |  |

## F-15.9.9 Multi-Modal Understanding

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.9.9.1 | designer (P-5) | the assistant to combine voice input, screenshot analysis, accessibility tree state, and conversation history to resolve ambiguous references (e.g., "make that brighter") | I can interact naturally without specifying every detail |  |  |
| US-15.9.9.2 | artist (P-8) | structured data (accessibility tree, selection state) to take precedence over pixel analysis when both are available | the assistant's responses are fast and accurate |  |  |
| US-15.9.9.3 | tech artist (P-13) | the assistant to use viewport capture alongside the accessibility tree to understand visual context that the tree alone cannot convey (e.g., "why does this material look wrong?") | visual diagnosis is possible |  |  |
| US-15.9.9.4 | engine tester (P-27) | to verify that the assistant degrades gracefully when a modality is unavailable (e.g., no microphone) | the system works with partial input sources |  |  |

## F-15.9.10 AI Assistance Administration

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.9.10.1 | server admin (P-22) | to restrict which tools the assistant can invoke via an allowlist or blocklist | sensitive operations are protected from AI-driven changes |  |  |
| US-15.9.10.2 | DevOps engineer (P-16) | to configure the AI provider and model backing the assistant | the studio can choose between cloud and on-premise LLM providers |  |  |
| US-15.9.10.3 | server admin (P-22) | per-user or per-team usage quotas (requests per hour, tokens per day) with rate limits | AI service costs are controlled |  |  |
| US-15.9.10.4 | engine tester (P-27) | to verify that assistant interaction logs are stored in a separate audit trail from content generation logs | compliance review of AI assistance is independent from AI content review |  |  |
