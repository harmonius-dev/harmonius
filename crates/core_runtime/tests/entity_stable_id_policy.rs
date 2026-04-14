//! TC-1.10.2.1 — [`core_runtime::ids::Entity`] must not implement [`StableId`](core_runtime::ids::StableId).

use core_runtime::ids::{Entity, StableId};
use static_assertions::assert_not_impl_any;

assert_not_impl_any!(Entity: StableId);
