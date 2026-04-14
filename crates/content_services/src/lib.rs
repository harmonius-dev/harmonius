#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

//! Editor-side content services: localization helpers, AI governance toggles, and assistant tool
//! dispatch. Unit tests reference `TC-*` identifiers from
//! `docs/design/tools/content-services-test-cases.md`.

pub mod assistant;
pub mod governance;
pub mod localization;

pub use assistant::tools::{
    ToolAccessControl, ToolDefinition, ToolError, ToolInterface, ToolInvocation, ToolResult,
};
pub use governance::command::{
    AssistantError, CommandHandle, CommandInterpreter, MultiModalContext, UserId,
};
pub use governance::modification::{ModificationBitmask, TrackingGranularity};
pub use governance::toggles::{FeatureToggleState, FeatureToggles};
pub use localization::icu::{
    plural_category, validate_icu_pattern, IcuPatternError, PluralCategory,
};
pub use localization::pseudo::pseudo_localize;
