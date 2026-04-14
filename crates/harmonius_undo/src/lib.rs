#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

//! Undo and redo primitives for Harmonius editor tooling.
//!
//! This crate implements the command stack, transactions, budgeting, and
//! collaborative bookkeeping described in `docs/design/tools/undo-redo.md`.

pub mod collab;
pub mod command;
pub mod ids;
pub mod persist;
pub mod record;
pub mod selection;
pub mod stack;
pub mod transaction;
pub mod world;

pub use collab::{CollabError, CollabSession, UndoConflict};
pub use command::{CommandError, EditorCommand};
pub use ids::{CommandId, TxId, UserId};
pub use persist::{FlatCommandRecord, SessionManifest};
pub use record::CommandRecord;
pub use selection::{EntityRef, SelectionSnapshot};
pub use stack::{DiskSpill, UndoError, UndoStack};
pub use transaction::{TransactionEnd, TransactionGuard};
pub use world::TestWorld;
