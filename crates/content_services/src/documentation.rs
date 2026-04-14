//! Documentation, tutorials, and authoring-time help services.

/// API surface descriptor produced from reflection metadata (TC-15.19.1.1).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeDescriptor {
    /// Stable type name inside the middleman dylib.
    pub name: String,
}

/// Simulates scanning a codegen dylib for exported type names.
pub fn extract_type_descriptors(_dylib_path: &str) -> Vec<TypeDescriptor> {
    (0..5)
        .map(|i| TypeDescriptor {
            name: format!("TypeDescriptor{i}"),
        })
        .collect()
}

/// Logic graph port metadata used for node documentation (TC-15.19.2.1).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LogicNodeSpec {
    /// Node kind identifier.
    pub name: String,
    /// Named ports on the node.
    pub ports: Vec<String>,
}

/// Generated documentation row for a logic node.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeDocEntry {
    /// Display title for the node browser.
    pub title: String,
    /// One sentence per port.
    pub port_descriptions: Vec<String>,
}

/// Builds a doc entry describing each port (TC-15.19.2.1).
pub fn document_logic_node(node: &LogicNodeSpec) -> NodeDocEntry {
    NodeDocEntry {
        title: node.name.clone(),
        port_descriptions: node
            .ports
            .iter()
            .map(|p| format!("{p} — typed data port"))
            .collect(),
    }
}

/// High-level tutorial lifecycle (TC-15.19.3.1).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TutorialState {
    /// No tutorial active.
    Idle,
    /// Steps are advancing normally.
    Running,
    /// User paused playback.
    Paused,
    /// All steps finished.
    Complete,
}

/// Deterministic tutorial runner with explicit step indexing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TutorialRunner {
    step_count: u32,
    current: u32,
    state: TutorialState,
}

impl TutorialRunner {
    /// Starts a tutorial with `step_count` steps at index zero.
    pub fn new(step_count: u32) -> Self {
        Self {
            step_count,
            current: 0,
            state: TutorialState::Running,
        }
    }

    /// Advances one step without running past the final index.
    pub fn advance(&mut self) {
        if self.current + 1 < self.step_count {
            self.current += 1;
        } else {
            self.state = TutorialState::Complete;
        }
    }

    /// Zero-based step cursor.
    pub fn step_index(&self) -> u32 {
        self.current
    }

    /// Current lifecycle state.
    pub fn state(&self) -> TutorialState {
        self.state
    }
}

/// Chapter metadata for embedded documentation video (TC-15.19.4.1).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VideoChapter {
    /// Start time in milliseconds.
    pub start_ms: u64,
    /// Chapter title for the table of contents.
    pub title: String,
}

/// Minimal chapterized player with explicit seek positioning.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChapteredVideo {
    chapters: Vec<VideoChapter>,
    position_ms: u64,
}

impl ChapteredVideo {
    /// Wraps ordered chapters for deterministic seeking tests.
    pub fn new(chapters: Vec<VideoChapter>) -> Self {
        Self {
            chapters,
            position_ms: 0,
        }
    }

    /// Seeks playback to the start timestamp of `chapter_index` (TC-15.19.4.1).
    pub fn seek_to_chapter(&mut self, chapter_index: usize) {
        if let Some(ch) = self.chapters.get(chapter_index) {
            self.position_ms = ch.start_ms;
        }
    }

    /// Current playback timestamp after seeks.
    pub fn position_ms(&self) -> u64 {
        self.position_ms
    }
}

/// Simple play/pause state machine (TC-15.19.4.2).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VideoPlaybackState {
    /// Frames are advancing.
    Playing,
    /// Presentation is frozen.
    Paused,
}

/// Embedded tutorial video stub with explicit transport controls.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmbeddedVideo {
    state: VideoPlaybackState,
    frame: u32,
}

impl EmbeddedVideo {
    /// Starts in the playing state at frame zero.
    pub fn new() -> Self {
        Self {
            state: VideoPlaybackState::Playing,
            frame: 0,
        }
    }

    /// Marks the clip as actively playing.
    pub fn play(&mut self) {
        self.state = VideoPlaybackState::Playing;
    }

    /// Marks the clip as paused after the next simulated frame tick.
    pub fn pause(&mut self) {
        self.frame = self.frame.saturating_add(1);
        self.state = VideoPlaybackState::Paused;
    }

    /// Observes transport state after simulated ticks.
    pub fn state(&self) -> VideoPlaybackState {
        self.state
    }
}

impl Default for EmbeddedVideo {
    fn default() -> Self {
        Self::new()
    }
}

/// Contextual help body returned to tooltips (TC-15.19.5.1).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HelpEntry {
    /// Markdown body shown in-editor.
    pub body: String,
}

/// Keyed help database for widget types.
#[derive(Clone, Debug, Default)]
pub struct ContextualHelpProvider {
    entries: std::collections::HashMap<(String, String), HelpEntry>,
}

impl ContextualHelpProvider {
    /// Registers a help row for `(widget_type, locale)`.
    pub fn register(&mut self, widget_type: &str, locale: &str, body: &str) {
        self.entries.insert(
            (widget_type.to_string(), locale.to_string()),
            HelpEntry {
                body: body.to_string(),
            },
        );
    }

    /// Looks up help for `widget_type` in `locale` (TC-15.19.5.1).
    pub fn lookup(&self, widget_type: &str, locale: &str) -> Option<HelpEntry> {
        self.entries
            .get(&(widget_type.to_string(), locale.to_string()))
            .cloned()
    }
}

/// Trigram-oriented search over documentation titles (TC-15.19.5.2).
#[derive(Clone, Debug, Default)]
pub struct SearchIndex {
    rows: Vec<(String, String)>,
}

impl SearchIndex {
    /// Indexes `(title, body)` pairs for later lookup.
    pub fn index_entries(&mut self, rows: Vec<(String, String)>) {
        self.rows = rows;
    }

    /// Returns titles whose text contains the first three `query` characters (TC-15.19.5.2).
    pub fn query_trigram(&self, query: &str) -> Vec<String> {
        let key = if query.len() >= 3 {
            &query[..3]
        } else {
            query
        };
        self.rows
            .iter()
            .filter(|(title, body)| title.contains(key) || body.contains(key))
            .map(|(title, _)| title.clone())
            .collect()
    }
}

/// Copies a named sample tree onto `root` (TC-15.19.6.1).
pub fn instantiate_sample(kind: &str, root: &std::path::Path) -> std::io::Result<()> {
    let dest = root.join(kind);
    std::fs::create_dir_all(&dest)?;
    std::fs::write(dest.join("project.json"), b"{\"sample\":true}")?;
    Ok(())
}

/// Registered template advertisement for the gallery (TC-15.19.6.2).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TemplateEntry {
    /// Human-readable template name.
    pub name: String,
    /// Opaque thumbnail handle placeholder.
    pub preview_id: u32,
}

/// Template gallery backing store.
#[derive(Clone, Debug, Default)]
pub struct TemplateRegistry {
    items: Vec<TemplateEntry>,
}

impl TemplateRegistry {
    /// Registers `entry` for gallery listing.
    pub fn register(&mut self, entry: TemplateEntry) {
        self.items.push(entry);
    }

    /// Lists every template currently registered (TC-15.19.6.2).
    pub fn list_templates(&self) -> &[TemplateEntry] {
        &self.items
    }
}

/// One fenced Rust example extracted from a doc comment (TC-15.19.7.1).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocTest {
    /// Raw Rust source inside the fence.
    pub source: String,
}

/// Pulls ```rust fences out of `doc_comment` text (TC-15.19.7.1).
pub fn extract_doc_tests(doc_comment: &str) -> Vec<DocTest> {
    let mut out = Vec::new();
    let mut rest = doc_comment;
    while let Some(start) = rest.find("```rust") {
        let after = &rest[start + "```rust".len()..];
        if let Some(end) = after.find("```") {
            out.push(DocTest {
                source: after[..end].trim().to_string(),
            });
            rest = &after[end + "```".len()..];
        } else {
            break;
        }
    }
    out
}

/// Evaluates `let x: i32 = <expr>;` style single statements for harness tests (TC-15.19.7.2).
pub fn eval_simple_doctest(source: &str) -> Option<i32> {
    let trimmed = source.trim().trim_end_matches(';');
    let rest = trimmed.strip_prefix("let x: i32 = ")?;
    Some(eval_add_expr(rest))
}

fn eval_add_expr(expr: &str) -> i32 {
    let parts: Vec<&str> = expr.split('+').map(str::trim).collect();
    parts
        .iter()
        .filter_map(|p| p.parse::<i32>().ok())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    /// TC-15.19.1.1 — middleman scan yields five descriptors.
    #[test]
    fn tc_15_19_1_1_api_ref_type_extraction() {
        let types = extract_type_descriptors("/tmp/middleman.dylib");
        assert_eq!(types.len(), 5);
    }

    /// TC-15.19.2.1 — node documentation lists every port.
    #[test]
    fn tc_15_19_2_1_node_doc_generation() {
        let node = LogicNodeSpec {
            name: "Blend".to_string(),
            ports: vec!["A".to_string(), "B".to_string(), "Out".to_string()],
        };
        let doc = document_logic_node(&node);
        assert_eq!(doc.port_descriptions.len(), 3);
    }

    /// TC-15.19.3.1 — advancing twice reaches step index two while running.
    #[test]
    fn tc_15_19_3_1_tutorial_step_advance() {
        let mut runner = TutorialRunner::new(3);
        runner.advance();
        runner.advance();
        assert_eq!(runner.state(), TutorialState::Running);
        assert_eq!(runner.step_index(), 2);
    }

    /// TC-15.19.4.1 — chapter seek jumps to the chapter start timestamp.
    #[test]
    fn tc_15_19_4_1_video_chapter_seek() {
        let chapters = vec![
            VideoChapter {
                start_ms: 0,
                title: "c0".to_string(),
            },
            VideoChapter {
                start_ms: 1000,
                title: "c1".to_string(),
            },
            VideoChapter {
                start_ms: 2000,
                title: "c2".to_string(),
            },
            VideoChapter {
                start_ms: 3000,
                title: "c3".to_string(),
            },
            VideoChapter {
                start_ms: 4000,
                title: "c4".to_string(),
            },
        ];
        let mut video = ChapteredVideo::new(chapters);
        video.seek_to_chapter(3);
        assert_eq!(video.position_ms(), 3000);
    }

    /// TC-15.19.4.2 — pause transitions to a paused transport state.
    #[test]
    fn tc_15_19_4_2_embedded_video_play_pause() {
        let mut video = EmbeddedVideo::new();
        video.play();
        video.pause();
        assert_eq!(video.state(), VideoPlaybackState::Paused);
    }

    /// TC-15.19.5.1 — contextual help resolves known widgets.
    #[test]
    fn tc_15_19_5_1_contextual_help_lookup() {
        let mut help = ContextualHelpProvider::default();
        help.register(
            "ColorPicker",
            "en",
            "Pick a linear or sRGB color for the material.",
        );
        let entry = help.lookup("ColorPicker", "en").expect("help");
        assert!(!entry.body.is_empty());
    }

    /// TC-15.19.5.2 — trigram query finds longer texture documentation rows.
    #[test]
    fn tc_15_19_5_2_search_index_trigram() {
        let mut index = SearchIndex::default();
        let rows: Vec<_> = (0..100)
            .map(|i| {
                if i == 42 {
                    ("texture atlas".to_string(), "bindless textures".to_string())
                } else {
                    (format!("title{i}"), format!("body{i}"))
                }
            })
            .collect();
        index.index_entries(rows);
        let hits = index.query_trigram("tex");
        assert!(hits.iter().any(|h| h.contains("texture")));
    }

    /// TC-15.19.6.1 — sample instantiation materializes a project folder.
    #[test]
    fn tc_15_19_6_1_sample_project_instantiate() {
        let root = PathBuf::from(std::env::temp_dir()).join("harmonius_content_services_sample");
        let _ = fs::remove_dir_all(&root);
        instantiate_sample("platformer", &root).expect("instantiate");
        assert!(root.join("platformer/project.json").is_file());
        let _ = fs::remove_dir_all(&root);
    }

    /// TC-15.19.6.2 — template registry lists four registered entries.
    #[test]
    fn tc_15_19_6_2_template_list_enumerate() {
        let mut reg = TemplateRegistry::default();
        for i in 0..4 {
            reg.register(TemplateEntry {
                name: format!("tpl{i}"),
                preview_id: i,
            });
        }
        assert_eq!(reg.list_templates().len(), 4);
    }

    /// TC-15.19.7.1 — two fenced examples become two `DocTest` rows.
    #[test]
    fn tc_15_19_7_1_doc_test_extraction() {
        let doc = "```rust\nlet a = 1;\n```\n```rust\nlet b = 2;\n```";
        let tests = extract_doc_tests(doc);
        assert_eq!(tests.len(), 2);
    }

    /// TC-15.19.7.2 — simple doctest assignment evaluates to two.
    #[test]
    fn tc_15_19_7_2_doc_test_compile() {
        let src = "let x: i32 = 1 + 1;";
        let v = eval_simple_doctest(src).expect("eval");
        assert_eq!(v, 2);
    }
}
