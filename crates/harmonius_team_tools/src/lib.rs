#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

//! Team collaboration tools: version control integration, remote editing, and related workflows.

pub mod binary_conflict;
pub mod branch_manager;
pub mod git_core;
pub mod git_status;
pub mod hosting;
pub mod lfs;
pub mod merge;
pub mod remote_collaboration;
pub mod shelf;
pub mod sparse_checkout;

pub use binary_conflict::{
    BinaryConflictResolver, ConflictPick, MergeError, PropertyConflict, PropertyTrees, PropertyRow,
};
pub use branch_manager::BranchManager;
pub use git_core::{CommitId, GitCore};
pub use git_status::{FileStatus, StatusKind, VcError};
pub use hosting::{detect_hosting, HostingKind};
pub use lfs::{LockInfo, LfsManager, QuotaInfo, TrackingRule};
pub use merge::{MergeDriver, MergeResult};
pub use remote_collaboration::{
    AiCollaboration, AssetComment, BandwidthAdapter, CommentStore, DiffHunk, GpuSessionToken,
    InlineReviewComment, PrReviewViewer, QualityTier, QuicTransport, RemoteGpuServer,
    VideoCodec, VideoEncodePipeline, VirtualUserId,
};
pub use shelf::{ShelfError, ShelfId, ShelfManager};
pub use sparse_checkout::SparseCheckoutConfig;
