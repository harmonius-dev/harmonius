//! AI asset review queue with approve transitions.

use std::collections::{HashMap, VecDeque};

use super::provenance::AssetId;

/// Lifecycle state for reviewer workflows.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReviewStatus {
    /// Waiting in the reviewer queue.
    Draft,
    /// Accepted by a reviewer.
    Approved,
    /// Rejected by a reviewer.
    Rejected,
}

/// Routes tagged assets into a human review queue.
#[derive(Clone, Debug, Default)]
pub struct ReviewWorkflow {
    queue: VecDeque<AssetId>,
    status: HashMap<AssetId, ReviewStatus>,
}

impl ReviewWorkflow {
    /// Empty workflow.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enqueues `asset` when not already queued or approved.
    pub fn route_to_review(&mut self, asset: AssetId) {
        if self.status.get(&asset) == Some(&ReviewStatus::Approved) {
            return;
        }
        if !self.queue.contains(&asset) {
            self.queue.push_back(asset);
            self.status.insert(asset, ReviewStatus::Draft);
        }
    }

    /// Snapshot of pending reviewer ids (front of queue first).
    pub fn reviewer_queue(&self) -> Vec<AssetId> {
        self.queue.iter().copied().collect()
    }

    /// Marks `asset` approved and removes it from the pending queue.
    pub fn approve(&mut self, asset: AssetId) {
        self.status.insert(asset, ReviewStatus::Approved);
        self.queue.retain(|&a| a != asset);
    }

    /// Marks `asset` rejected and removes it from the pending queue.
    pub fn reject(&mut self, asset: AssetId) {
        self.status.insert(asset, ReviewStatus::Rejected);
        self.queue.retain(|&a| a != asset);
    }

    /// Reads the stored status for `asset` when present.
    pub fn status_of(&self, asset: AssetId) -> Option<ReviewStatus> {
        self.status.get(&asset).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::super::provenance::AssetId;
    use super::*;

    /// TC-15.7.7.1 — routing places the asset on the reviewer queue.
    #[test]
    fn tc_15_7_7_1_review_route_ai_asset() {
        let mut rw = ReviewWorkflow::new();
        let asset = AssetId(42);
        rw.route_to_review(asset);
        assert_eq!(rw.reviewer_queue(), vec![asset]);
    }

    /// TC-15.7.7.2 — approval clears the queue entry and records status.
    #[test]
    fn tc_15_7_7_2_review_approve_removes_queue() {
        let mut rw = ReviewWorkflow::new();
        let asset = AssetId(3);
        rw.route_to_review(asset);
        rw.approve(asset);
        assert_eq!(rw.status_of(asset), Some(ReviewStatus::Approved));
        assert!(rw.reviewer_queue().is_empty());
    }
}
