#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

//! Team collaboration tools: version control integration, remote editing, and related workflows.

pub mod git_status;

pub use git_status::{FileStatus, StatusKind, VcError};
