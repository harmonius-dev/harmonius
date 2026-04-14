//! Threshold scanning for event logs.
//!
//! Production runs resolve [`PredicateId`](crate::PredicateId) via editor codegen; the `pred`
//! argument to [`check_thresholds`] is the in-process bridge until that pipeline is wired.

use core::marker::PhantomData;

use rkyv::{Archive, Deserialize, Serialize};
use smallvec::SmallVec;
use smol_str::SmolStr;

use crate::log::EventLog;
use crate::types::{AssetId, GameplayTag, PredicateId};

/// Action executed when a threshold fires.
#[derive(Clone, Debug, PartialEq, Eq, Archive, Serialize, Deserialize)]
pub enum ThresholdAction {
    /// Fire a named engine event.
    FireEvent(SmolStr),
    /// Apply an effect asset.
    ApplyEffect(AssetId),
    /// Apply a gameplay tag.
    SetTag(GameplayTag),
}

/// Declarative threshold evaluated against a log.
#[derive(Clone, Debug, Archive, Serialize, Deserialize)]
pub struct ThresholdTrigger<T> {
    /// Predicate resolved by codegen.
    pub predicate: PredicateId,
    /// Minimum matching entries within the window.
    pub count: u32,
    /// Inclusive look-back window length in ticks.
    pub window_ticks: u64,
    /// Action emitted when the threshold is satisfied.
    pub action: ThresholdAction,
    #[archive(skip)]
    _marker: PhantomData<T>,
}

impl<T> ThresholdTrigger<T> {
    /// Constructs a trigger for `T`-typed logs.
    pub fn new(
        predicate: PredicateId,
        count: u32,
        window_ticks: u64,
        action: ThresholdAction,
    ) -> Self {
        Self {
            predicate,
            count,
            window_ticks,
            action,
            _marker: PhantomData,
        }
    }
}

/// Returns every action whose predicate matches at least `count` entries inside the window ending
/// at `current_tick`.
pub fn check_thresholds<T, P>(
    log: &EventLog<T>,
    triggers: &[ThresholdTrigger<T>],
    current_tick: u64,
    pred: &mut P,
) -> SmallVec<[ThresholdAction; 4]>
where
    T: Clone + rkyv::Archive,
    P: FnMut(PredicateId, &T) -> bool,
{
    let mut out = SmallVec::new();
    for trigger in triggers {
        let ws = current_tick.saturating_sub(trigger.window_ticks);
        let mut matched = 0_u32;
        for entry in &log.entries {
            if entry.timestamp < ws || entry.timestamp > current_tick {
                continue;
            }
            if pred(trigger.predicate, &entry.data) {
                matched = matched.saturating_add(1);
            }
        }
        if matched >= trigger.count {
            out.push(trigger.action.clone());
        }
    }
    out
}
