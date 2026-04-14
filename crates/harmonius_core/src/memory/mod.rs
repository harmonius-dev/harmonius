//! Memory allocators, budgets, profiling, and deterministic numerics.

mod arena;
mod budget;
mod pool;
mod precision;
mod tag;
mod tracker;

pub use arena::{ArenaConfig, ArenaError, FrameArena, PerThreadArena, ScopedArena};
pub use budget::{BudgetError, MemoryBudget};
pub use pool::{PoolAllocator, PoolConfig, PoolSlot};
pub use precision::{BigFloat, BigInt};
pub use tag::{AllocationTag, SubsystemId};
pub use tracker::MemoryTracker;
