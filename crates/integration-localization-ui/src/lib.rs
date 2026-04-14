//! Localization ↔ UI integration contracts and deterministic fakes used by CI tests.
//!
//! Behavioral coverage follows `docs/design/integration/localization-ui.md` and
//! `docs/design/integration/localization-ui-test-cases.md`.
//!
//! ECS wiring, worker-thread scheduling, and production `crossbeam-channel` usage live in
//! engine layers above this crate; keep integration tests here pure and single-threaded.

#![deny(clippy::all)]
#![deny(unsafe_code)]
#![warn(missing_docs)]

mod format;
mod ime;
mod layout;
mod locale_channel;
mod localization_table;
mod shaping;
mod types;

pub use format::format_message;
pub use ime::{ImeEvent, ImeEventKind, TextInputState};
pub use layout::{
    icon_button_layout, layout_line, split_mixed_runs, LayoutLocaleBridge, LineLayout, RunLayout,
    WidgetForest, WidgetShell,
};
pub use locale_channel::{LocaleChangeChannel, LocaleChangeEvent};
pub use localization_table::{LocalizationTable, ResolveArgs};
pub use shaping::{shape_line, GlyphRun, ShapedGlyph};
pub use types::{
    ArgValue, AssetId, FallbackCounters, FontChain, FontChainResolver, Gender, LocaleId,
    LocalizedStringId, MessageTemplate, ResolvedText, ScriptTag, TextDirection,
};
