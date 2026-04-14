//! SDK and build-farm integration stubs (`R-14.8`).

mod build_server;

pub use build_server::{
    compare_priority, BuildJob, BuildJobId, BuildPriority, BuildServer, BuildStatus,
    ConsolePlatform,
};
