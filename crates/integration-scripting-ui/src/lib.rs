//! Scripting ↔ UI integration contracts and a headless harness for CI tests.
//!
//! Coverage follows `docs/design/integration/scripting-ui.md` and
//! `docs/design/integration/scripting-ui-test-cases.md`.

#![deny(clippy::all)]
#![deny(unsafe_code)]
#![warn(missing_docs)]

mod harness;
mod types;

pub use harness::{ScriptingUiHarness, WidgetEventHandler, WidgetSnapshot};
pub use types::{
    ArgValue, AssetId, BindingSource, BindingUpdateMode, BoundExpr, DataBinding, DialogueChoice,
    Entity, ExprKind, FallbackCounters, FormattedString, LocalizedStringId, ScriptValue,
    SetWidgetState, VisibilityEval, WidgetEvent, WidgetEventKind, WidgetStateKind,
};
