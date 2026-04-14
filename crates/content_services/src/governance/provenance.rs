//! AI asset provenance tagging and filtered queries.

use std::collections::HashMap;

use super::audit::AuditLog;
use super::command::UserId;

/// Stable asset identifier used by governance and audit trails.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AssetId(pub u64);

/// Identifies which AI system produced an asset (texture vs mesh generators in tests).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AiSystemId(pub u32);

/// Well-known generator ids for traceability tests.
pub mod ai_systems {
    use super::AiSystemId;

    /// Texture pipeline id.
    pub const TEXTURE_GENERATOR: AiSystemId = AiSystemId(1);
    /// Mesh pipeline id.
    pub const MESH_GENERATOR: AiSystemId = AiSystemId(2);
}

/// 32-byte BLAKE3 digest placeholder for prompt attestation fields.
pub type Blake3Hash = [u8; 32];

/// Record stored per tagged asset.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceTag {
    /// Asset this tag describes.
    pub asset_id: AssetId,
    /// Originating AI system.
    pub ai_system: AiSystemId,
}

/// Filter for `query_assets` (subset by AI system).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProvenanceFilter {
    /// When set, only assets produced by this system match.
    pub ai_system: Option<AiSystemId>,
}

/// In-memory provenance index with synchronous audit append for unit tests.
#[derive(Clone, Debug, Default)]
pub struct ProvenanceTracker {
    tags: HashMap<AssetId, ProvenanceTag>,
}

impl ProvenanceTracker {
    /// Empty tracker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records provenance and appends a deterministic audit line.
    pub fn tag_asset(
        &mut self,
        asset_id: AssetId,
        ai_system: AiSystemId,
        _model_version: &str,
        _prompt_hash: Blake3Hash,
        _user_id: UserId,
        audit_log: &mut AuditLog,
    ) {
        self.tags.insert(
            asset_id,
            ProvenanceTag {
                asset_id,
                ai_system,
            },
        );
        let mut body = Vec::new();
        body.extend_from_slice(&asset_id.0.to_le_bytes());
        body.extend_from_slice(&ai_system.0.to_le_bytes());
        audit_log.append(body);
    }

    /// True when a provenance record exists for `asset_id`.
    pub fn has_provenance(&self, asset_id: AssetId) -> bool {
        self.tags.contains_key(&asset_id)
    }

    /// Borrows a stored tag when present.
    pub fn tag_of(&self, asset_id: AssetId) -> Option<&ProvenanceTag> {
        self.tags.get(&asset_id)
    }

    /// Returns asset ids matching `filter` (stable sort by id for determinism).
    pub fn query_assets(&self, filter: &ProvenanceFilter) -> Vec<AssetId> {
        let mut out: Vec<AssetId> = self
            .tags
            .iter()
            .filter(|(_, tag)| {
                filter
                    .ai_system
                    .map(|s| s == tag.ai_system)
                    .unwrap_or(true)
            })
            .map(|(id, _)| *id)
            .collect();
        out.sort_by_key(|a| a.0);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::ai_systems::{MESH_GENERATOR, TEXTURE_GENERATOR};
    use super::*;

    /// TC-15.7.1.1 — tagging establishes provenance visibility.
    #[test]
    fn tc_15_7_1_1_provenance_tag_create() {
        let mut tracker = ProvenanceTracker::new();
        let mut audit = AuditLog::new();
        let id = AssetId(7);
        tracker.tag_asset(
            id,
            TEXTURE_GENERATOR,
            "v1",
            [1u8; 32],
            UserId(99),
            &mut audit,
        );
        assert!(tracker.has_provenance(id));
    }

    /// TC-15.7.1.2 — filter returns only mesh-generated assets.
    #[test]
    fn tc_15_7_1_2_provenance_query_filter() {
        let mut tracker = ProvenanceTracker::new();
        let mut audit = AuditLog::new();
        for i in 0u64..50 {
            let sys = if i % 3 == 0 {
                MESH_GENERATOR
            } else {
                TEXTURE_GENERATOR
            };
            tracker.tag_asset(AssetId(i), sys, "v1", [0u8; 32], UserId(1), &mut audit);
        }
        let mesh_only = tracker.query_assets(&ProvenanceFilter {
            ai_system: Some(MESH_GENERATOR),
        });
        for id in &mesh_only {
            assert_eq!(tracker.tag_of(*id).expect("tag").ai_system, MESH_GENERATOR);
        }
        assert_eq!(mesh_only.len(), 17);
    }
}
