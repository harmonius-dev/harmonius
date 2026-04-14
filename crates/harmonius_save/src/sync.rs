//! Cloud sync decision surface (R-13.3.5) — platform SDKs live elsewhere.

use crate::types::SaveSlotMeta;

/// Result of comparing local and remote saves.
#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum SyncResult {
    /// Local bytes were newer and uploaded.
    Uploaded,
    /// Remote was strictly newer and downloaded.
    Downloaded,
    /// Both sides changed since last sync token.
    Conflict {
        local: SaveSlotMeta,
        remote: SaveSlotMeta,
    },
    /// No-op: already aligned.
    Noop,
}

/// Pure decision helper used by platform services (TC-13.3.5.*).
pub fn decide_sync(local: &SaveSlotMeta, remote: &SaveSlotMeta) -> SyncResult {
    if local.content_hash == remote.content_hash {
        return SyncResult::Noop;
    }
    if local.playtime_seconds > remote.playtime_seconds {
        return SyncResult::Uploaded;
    }
    if remote.playtime_seconds > local.playtime_seconds {
        return SyncResult::Downloaded;
    }
    SyncResult::Conflict {
        local: local.clone(),
        remote: remote.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::SlotId;

    /// TC-13.3.5.1 Local newer uploads.
    #[test]
    fn tc_13_3_5_1_local_newer_uploads() {
        let mut a = SaveSlotMeta::fixture(SlotId(1), "a");
        let mut b = SaveSlotMeta::fixture(SlotId(1), "a");
        b.playtime_seconds = 1;
        a.playtime_seconds = 10;
        a.content_hash[0] = 1;
        b.content_hash[0] = 2;
        assert_eq!(decide_sync(&a, &b), SyncResult::Uploaded);
    }

    /// TC-13.3.5.2 Conflict when both diverged with equal playtime.
    #[test]
    fn tc_13_3_5_2_conflict_both_changed() {
        let mut a = SaveSlotMeta::fixture(SlotId(1), "a");
        let mut b = SaveSlotMeta::fixture(SlotId(1), "a");
        a.content_hash[0] = 1;
        b.content_hash[0] = 2;
        assert!(matches!(decide_sync(&a, &b), SyncResult::Conflict { .. }));
    }
}
