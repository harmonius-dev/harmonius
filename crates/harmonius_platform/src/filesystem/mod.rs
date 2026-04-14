//! Host filesystem surface (`R-14.6`).

mod dir;
mod error;
mod file;
mod hasher;
mod ops;
mod path;
mod watcher;

pub use dir::{enumerate_dir, DirEntry, EnumerateOptions};
pub use error::FsError;
pub use file::{AsyncFile, OpenFlags};
pub use hasher::{Blake3Hash, ContentHasher};
pub use ops::{create_dir_all, delete_batch, delete_file, stat, stat_batch, FileMetadata, FileType};
pub use path::CanonicalPath;
pub use watcher::{FileEvent, FileEventKind, FileEventStream, FileWatcher, WatchId};
