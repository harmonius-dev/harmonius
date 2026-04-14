//! Minimal RTL-aware layout helpers for integration tests.

use crate::types::{FallbackCounters, LocaleId, LocalizedStringId, TextDirection};

/// One laid-out run with a start `x` and total advance.
#[derive(Clone, Debug, PartialEq)]
pub struct RunLayout {
    /// Start x in layout units.
    pub start_x: f32,
    /// Total advance for the run.
    pub advance: f32,
}

/// A full line layout composed of directional runs.
#[derive(Clone, Debug, PartialEq)]
pub struct LineLayout {
    /// Ordered runs left-to-right in visual order.
    pub runs: Vec<RunLayout>,
}

fn measure_advance(text: &str) -> f32 {
    text.chars().count() as f32 * 10.0
}

/// Lays out a single-line `text` inside `width` using `direction`.
#[must_use]
pub fn layout_line(text: &str, direction: TextDirection, width: f32) -> LineLayout {
    let adv = measure_advance(text);
    match direction {
        TextDirection::Rtl => LineLayout {
            runs: vec![RunLayout {
                start_x: (width - adv).max(0.0),
                advance: adv,
            }],
        },
        TextDirection::Ltr | TextDirection::Auto => LineLayout {
            runs: vec![RunLayout {
                start_x: 0.0,
                advance: adv,
            }],
        },
    }
}

/// Splits mixed-direction text into coarse runs for TC-IR-4.4.2.2.
///
/// This is a small directional heuristic for tests, not a full UAX#9 implementation.
#[must_use]
pub fn split_mixed_runs(text: &str) -> Vec<(TextDirection, String)> {
    let mut runs: Vec<(TextDirection, String)> = Vec::new();
    let mut current_dir = TextDirection::Ltr;
    let mut buf = String::new();

    for ch in text.chars() {
        let dir =
            if ('\u{0590}'..='\u{05FF}').contains(&ch) || ('\u{0600}'..='\u{06FF}').contains(&ch) {
                TextDirection::Rtl
            } else if ch.is_ascii_alphanumeric() || ch.is_ascii_whitespace() {
                TextDirection::Ltr
            } else {
                current_dir
            };

        if !buf.is_empty() && dir != current_dir {
            runs.push((current_dir, std::mem::take(&mut buf)));
        }
        current_dir = dir;
        buf.push(ch);
    }
    if !buf.is_empty() {
        runs.push((current_dir, buf));
    }
    runs
}

/// Computes `(icon_x, label_x)` for a simple icon+label row.
#[must_use]
pub fn icon_button_layout(
    direction: TextDirection,
    icon_x: f32,
    label_x: f32,
    gap: f32,
) -> (f32, f32) {
    match direction {
        TextDirection::Rtl => (label_x, icon_x + gap),
        TextDirection::Ltr | TextDirection::Auto => (icon_x, label_x),
    }
}

/// Tracks locale changes that arrive during layout (FM-7).
#[derive(Clone, Debug)]
pub struct LayoutLocaleBridge {
    in_layout: bool,
    staged_locale: LocaleId,
    pending_next: Option<LocaleId>,
}

impl LayoutLocaleBridge {
    /// Starts in `locale`.
    #[must_use]
    pub fn new(locale: LocaleId) -> Self {
        Self {
            in_layout: false,
            staged_locale: locale,
            pending_next: None,
        }
    }

    /// Active locale visible to layout this frame.
    #[must_use]
    pub fn current(&self) -> LocaleId {
        self.staged_locale
    }

    /// Marks the start of layout work for the frame.
    pub fn begin_layout(&mut self) {
        self.in_layout = true;
    }

    /// Ends layout; applies any deferred locale switch.
    pub fn end_layout(&mut self) {
        self.in_layout = false;
        if let Some(next) = self.pending_next.take() {
            self.staged_locale = next;
        }
    }

    /// Requests a locale switch, deferring when mid-layout.
    pub fn request_locale(&mut self, next: LocaleId, counters: &mut FallbackCounters) {
        if self.in_layout {
            self.pending_next = Some(next);
            counters.fm7 += 1;
        } else {
            self.staged_locale = next;
        }
    }
}

/// Widget shell tracked for locale-driven relayout.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetShell {
    /// Optional localized label id.
    pub localized: Option<LocalizedStringId>,
    /// Dirty flag consumed by the layout scheduler.
    pub layout_dirty: bool,
}

/// Small forest of widgets used to validate locale invalidation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetForest {
    /// Flat widget list.
    pub widgets: Vec<WidgetShell>,
}

impl WidgetForest {
    /// Marks every widget carrying a localized id dirty.
    pub fn on_locale_change(&mut self) {
        for w in &mut self.widgets {
            if w.localized.is_some() {
                w.layout_dirty = true;
            }
        }
    }
}
