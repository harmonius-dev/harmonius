use std::marker::PhantomData;

/// Interned string handle for UI copy (see `ui-framework.md` in design docs).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StringId(pub u32);

/// Marker type for `AssetHandle<UiIcon>` in the integration design.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct UiIcon;

/// Untyped asset reference for UI icons.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AssetHandle<T> {
    /// Stable asset index from the content database.
    pub index: u32,
    _marker: PhantomData<T>,
}

impl<T> AssetHandle<T> {
    /// Builds a handle; `index == 0` conventionally means unresolved / missing.
    #[must_use]
    pub const fn new(index: u32) -> Self {
        Self {
            index,
            _marker: PhantomData,
        }
    }
}

/// Data-layer transfer outcome mirrored for `UiTransferFeedback`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TransferResult {
    /// Transfer applied successfully.
    Success,
    /// Destination cannot accept more items or stacks.
    InsufficientCapacity,
    /// Tag or socket rule rejected the operation.
    ConstraintViolation,
    /// Operation not permitted for this actor.
    PermissionDenied,
    /// Slot index out of range or sentinel misuse.
    InvalidSlot,
    /// Stack cannot grow further for this item type.
    StackFull,
    /// Bag-in-bag depth exceeded configured maximum.
    NestingDepthExceeded,
    /// Source stack missing or already moved.
    ItemNotFound,
    /// Total weight would exceed container capacity.
    WeightExceeded,
}

/// Sort axis for `UiSortRequest` / `SortRequest`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SortCriteria {
    /// Lexicographic on display name.
    Name,
    /// Ascending mass.
    Weight,
    /// Higher tier first.
    Rarity,
    /// Category / item family.
    Type,
    /// Designer-defined comparator id.
    Custom(u32),
}
