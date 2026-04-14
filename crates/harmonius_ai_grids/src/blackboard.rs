use std::collections::BTreeMap;

/// Stable key for blackboard entries.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BlackboardKey(pub u32);

/// Scalar or vector payload stored on a blackboard.
#[derive(Clone, Debug, PartialEq)]
pub enum BlackboardValue {
    /// Single-precision scalar.
    F32(f32),
    /// Unknown / invalid sample flag per FM-1.
    Unknown,
    /// Sampled scalar with explicit uncertainty (FM-1 out-of-bounds).
    Measured {
        /// Sampled or default value.
        value: f32,
        /// Whether the sample is outside valid grid coverage.
        unknown: bool,
    },
}

/// Per-agent blackboard using a sorted map for deterministic iteration.
#[derive(Clone, Debug, Default)]
pub struct Blackboard {
    entries: BTreeMap<BlackboardKey, BlackboardValue>,
}

impl Blackboard {
    /// Creates an empty blackboard.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    /// Inserts or replaces a value.
    pub fn insert(&mut self, key: BlackboardKey, value: BlackboardValue) {
        let _ = self.entries.insert(key, value);
    }

    /// Borrows a value if present.
    #[must_use]
    pub fn get(&self, key: BlackboardKey) -> Option<&BlackboardValue> {
        self.entries.get(&key)
    }

    /// Reads world position `(x, y)` from keys `px`, `py` when both are `F32`.
    #[must_use]
    pub fn world_position_xy(&self, px: BlackboardKey, py: BlackboardKey) -> Option<(f32, f32)> {
        let x = self.get(px)?;
        let y = self.get(py)?;
        match (x, y) {
            (BlackboardValue::F32(xv), BlackboardValue::F32(yv)) => Some((*xv, *yv)),
            _ => None,
        }
    }

    /// Reads world position `(x, y, z)` from keys `px`, `py`, `pz` when all are `F32`.
    #[must_use]
    pub fn world_position_xyz(
        &self,
        px: BlackboardKey,
        py: BlackboardKey,
        pz: BlackboardKey,
    ) -> Option<(f32, f32, f32)> {
        let x = self.get(px)?;
        let y = self.get(py)?;
        let z = self.get(pz)?;
        match (x, y, z) {
            (BlackboardValue::F32(xv), BlackboardValue::F32(yv), BlackboardValue::F32(zv)) => {
                Some((*xv, *yv, *zv))
            }
            _ => None,
        }
    }
}
