//! Harmonius core entry: console SDK wiring for the open-source tree.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

pub use console_sdk::*;

/// Active console backend for this build.
///
/// The public repository always links the stub while `target_platform_console` is absent.
/// Private forks replace this alias with proprietary implementations while preserving the trait
/// surface.
pub type Console = console_stub::StubConsoleSdk;
