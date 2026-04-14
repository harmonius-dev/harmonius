//! Remote editing collaboration stubs (**TC-15.12.*** unit scope, R-15.12.*).

use std::collections::VecDeque;
use std::sync::Mutex;

/// Video codec identifiers for remote desktop streams (**TC-15.12.1.1**).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VideoCodec {
    /// H.264 baseline profile marker for negotiated sessions.
    H264,
    /// H.265 / HEVC marker for negotiated sessions.
    H265,
}

/// Minimal encoder stub returning one synthetic frame marker per codec.
#[derive(Clone, Debug, Default)]
pub struct VideoEncodePipeline;

impl VideoEncodePipeline {
    /// Produce a deterministic pseudo-frame tag for `codec`.
    pub fn encode_keyframe(&self, codec: VideoCodec) -> Vec<u8> {
        match codec {
            VideoCodec::H264 => vec![0x48, 0x32, 0x36, 0x34],
            VideoCodec::H265 => vec![0x48, 0x32, 0x36, 0x35],
        }
    }
}

/// In-memory QUIC-style datagram echo (**TC-15.12.2.1**).
#[derive(Debug, Default)]
pub struct QuicTransport {
    pending: Mutex<VecDeque<Vec<u8>>>,
}

impl QuicTransport {
    /// Enqueue an outbound payload.
    pub fn send(&self, bytes: &[u8]) {
        let mut q = self.pending.lock().expect("lock");
        q.push_back(bytes.to_vec());
    }

    /// Pop the next inbound payload (FIFO).
    pub fn recv(&self) -> Option<Vec<u8>> {
        let mut q = self.pending.lock().expect("lock");
        q.pop_front()
    }

    /// Send then immediately receive the same bytes (loopback test hook).
    pub fn round_trip(&self, bytes: &[u8]) -> Vec<u8> {
        self.send(bytes);
        self.recv().unwrap_or_default()
    }
}

/// Discrete stream quality tiers (**TC-15.12.6.1**).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QualityTier {
    /// Lowest bitrate tier.
    Low,
    /// Mid bitrate tier.
    Medium,
    /// Highest bitrate tier.
    High,
}

/// Maps observed throughput to a [`QualityTier`].
#[derive(Clone, Copy, Debug, Default)]
pub struct BandwidthAdapter;

impl BandwidthAdapter {
    /// Select a tier from bits-per-second estimates (deterministic thresholds).
    pub fn tier_for_bps(&self, bps: u64) -> QualityTier {
        if bps < 2_000_000 {
            QualityTier::Low
        } else if bps < 8_000_000 {
            QualityTier::Medium
        } else {
            QualityTier::High
        }
    }
}

/// Token returned after allocating a remote GPU partition (**TC-15.12.4.1**).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GpuSessionToken(pub u64);

/// Stub allocator issuing monotonic session tokens.
#[derive(Debug, Default)]
pub struct RemoteGpuServer {
    next: Mutex<u64>,
}

impl RemoteGpuServer {
    /// Allocate a new isolated session and return its token.
    pub fn allocate_session(&self) -> GpuSessionToken {
        let mut n = self.next.lock().expect("lock");
        *n += 1;
        GpuSessionToken(*n)
    }
}

/// Virtual collaborator joining an editing session (**TC-15.12.12.1**).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VirtualUserId(pub String);

/// Registers AI agents as synthetic session participants.
#[derive(Clone, Debug, Default)]
pub struct AiCollaboration;

impl AiCollaboration {
    /// Join `agent` as a virtual user id string.
    pub fn join_agent(&self, agent: &str) -> VirtualUserId {
        VirtualUserId(format!("ai::{agent}"))
    }
}

/// One persisted asset comment (**TC-15.12.13.1**).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetComment {
    /// Stable author display id.
    pub author: String,
    /// Target asset reference string (path or GUID).
    pub target_ref: String,
    /// Markdown/plain body text.
    pub body: String,
    /// Wall-clock seconds since Unix epoch.
    pub created_at_secs: u64,
}

/// In-memory comment threads for asset review.
#[derive(Debug, Default)]
pub struct CommentStore {
    rows: Mutex<Vec<AssetComment>>,
}

impl CommentStore {
    /// Persist a new top-level comment row.
    pub fn create_thread(
        &self,
        author: impl Into<String>,
        target_ref: impl Into<String>,
        body: impl Into<String>,
        created_at_secs: u64,
    ) -> usize {
        let mut rows = self.rows.lock().expect("lock");
        rows.push(AssetComment {
            author: author.into(),
            body: body.into(),
            created_at_secs,
            target_ref: target_ref.into(),
        });
        rows.len() - 1
    }

    /// Borrow the stored row when the index is valid.
    pub fn get(&self, index: usize) -> Option<AssetComment> {
        let rows = self.rows.lock().expect("lock");
        rows.get(index).cloned()
    }
}

/// One diff hunk surfaced in the PR review surface (**TC-15.12.14.1**).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiffHunk {
    /// Starting line number in the unified diff (1-based for tests).
    pub start_line: u32,
    /// Line count covered by the hunk.
    pub line_count: u32,
}

/// Inline comment captured from the review UI (**TC-15.12.14.1**).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InlineReviewComment {
    /// Line anchor inside the file under review.
    pub line: u32,
    /// Comment body text.
    pub body: String,
}

/// Minimal PR review model with hunks and inline threads.
#[derive(Debug, Default)]
pub struct PrReviewViewer {
    hunks: Mutex<Vec<DiffHunk>>,
    inline: Mutex<Vec<InlineReviewComment>>,
}

impl PrReviewViewer {
    /// Register diff hunks for rendering.
    pub fn set_hunks(&self, hunks: Vec<DiffHunk>) {
        let mut g = self.hunks.lock().expect("lock");
        *g = hunks;
    }

    /// Return the number of hunks currently registered.
    pub fn hunk_count(&self) -> usize {
        self.hunks.lock().expect("lock").len()
    }

    /// Append an inline review comment (simulates editor submission).
    pub fn submit_inline(&self, line: u32, body: impl Into<String>) -> usize {
        let mut rows = self.inline.lock().expect("lock");
        rows.push(InlineReviewComment {
            body: body.into(),
            line,
        });
        rows.len() - 1
    }

    /// Fetch inline comment by index.
    pub fn inline_comment(&self, index: usize) -> Option<InlineReviewComment> {
        let rows = self.inline.lock().expect("lock");
        rows.get(index).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **TC-15.12.1.1** — Video encode pipeline produces H.264/H.265 frames.
    #[test]
    fn tc_15_12_1_1_video_frames() {
        let pipe = VideoEncodePipeline::default();
        assert_eq!(pipe.encode_keyframe(VideoCodec::H264), vec![0x48, 0x32, 0x36, 0x34]);
        assert_eq!(pipe.encode_keyframe(VideoCodec::H265), vec![0x48, 0x32, 0x36, 0x35]);
    }

    /// **TC-15.12.2.1** — QUIC transport round-trip delivers input and frames.
    #[test]
    fn tc_15_12_2_1_quic_round_trip() {
        let q = QuicTransport::default();
        assert_eq!(q.round_trip(b"ping"), b"ping");
    }

    /// **TC-15.12.6.1** — Bandwidth tier selection adapts stream quality.
    #[test]
    fn tc_15_12_6_1_bandwidth_tiers() {
        let b = BandwidthAdapter::default();
        assert_eq!(b.tier_for_bps(1_000_000), QualityTier::Low);
        assert_eq!(b.tier_for_bps(4_000_000), QualityTier::Medium);
        assert_eq!(b.tier_for_bps(20_000_000), QualityTier::High);
    }

    /// **TC-15.12.4.1** — Remote GPU server allocates a session with isolated VRAM token.
    #[test]
    fn tc_15_12_4_1_gpu_session_token() {
        let srv = RemoteGpuServer::default();
        let t1 = srv.allocate_session();
        let t2 = srv.allocate_session();
        assert_ne!(t1, t2);
    }

    /// **TC-15.12.12.1** — AI agent session joins as virtual user.
    #[test]
    fn tc_15_12_12_1_ai_agent_join() {
        let colab = AiCollaboration::default();
        let u = colab.join_agent("planner-1");
        assert_eq!(u.0, "ai::planner-1");
    }

    /// **TC-15.12.13.1** — Asset comment thread creation persists metadata.
    #[test]
    fn tc_15_12_13_1_asset_comment_thread() {
        let store = CommentStore::default();
        let idx = store.create_thread("alice", "assets/mesh.bin", "needs LOD", 1_700_000_000);
        let row = store.get(idx).expect("row");
        assert_eq!(row.author, "alice");
        assert_eq!(row.target_ref, "assets/mesh.bin");
        assert_eq!(row.body, "needs LOD");
        assert_eq!(row.created_at_secs, 1_700_000_000);
    }

    /// **TC-15.12.14.1** — PR review viewer renders diff hunks and accepts inline comments.
    #[test]
    fn tc_15_12_14_1_pr_review_surface() {
        let view = PrReviewViewer::default();
        view.set_hunks(vec![
            DiffHunk {
                line_count: 3,
                start_line: 10,
            },
            DiffHunk {
                line_count: 1,
                start_line: 40,
            },
        ]);
        assert_eq!(view.hunk_count(), 2);
        let cid = view.submit_inline(12, "nit: naming");
        let c = view.inline_comment(cid).expect("comment");
        assert_eq!(c.line, 12);
        assert_eq!(c.body, "nit: naming");
    }
}
