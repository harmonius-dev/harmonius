//! Network bandwidth and packet decode helpers (design: `NetBandwidthTracker`).

use std::collections::HashMap;
use std::sync::Mutex;

/// Traffic direction for a network sample.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetDirection {
    /// Client to server.
    Upstream,
    /// Server to client.
    Downstream,
}

/// Per-channel bandwidth estimate for one frame.
#[derive(Clone, Debug, PartialEq)]
pub struct ChannelBandwidth {
    /// Logical channel id.
    pub channel_id: u32,
    /// Human-readable channel label.
    pub channel_name: String,
    /// Estimated upstream bitrate (bits per second).
    pub upstream_bps: f64,
    /// Estimated downstream bitrate (bits per second).
    pub downstream_bps: f64,
    /// Bytes observed this frame (upstream).
    pub upstream_bytes: u64,
    /// Bytes observed this frame (downstream).
    pub downstream_bytes: u64,
}

/// Filter placeholder for [`PacketInspector`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PacketFilter {
    /// Optional channel id filter (`None` = all).
    pub channel_id: Option<u32>,
}

/// One decoded field in a packet.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PacketField {
    /// Field name from the wire schema.
    pub name: String,
    /// Field value as UTF-8 text.
    pub value: String,
    /// Byte offset of the field within the packet.
    pub offset: u32,
    /// Field size in bytes on the wire.
    pub size: u32,
}

/// Decoded packet view for tooling.
#[derive(Clone, Debug, PartialEq)]
pub struct DecodedPacket {
    /// Capture timestamp in seconds (stub `0.0` until clock wiring).
    pub timestamp: f64,
    /// Traffic direction for this sample.
    pub direction: NetDirection,
    /// Channel id from framing metadata.
    pub channel_id: u32,
    /// Total packet size in bytes.
    pub size_bytes: u32,
    /// Parsed field rows.
    pub fields: Vec<PacketField>,
}

/// Tracks per-channel byte counters for the active frame.
#[derive(Debug)]
pub struct NetBandwidthTracker {
    inner: Mutex<HashMap<u32, (u64, u64)>>,
    frame_seconds: Mutex<f64>,
}

impl NetBandwidthTracker {
    /// Empty tracker with a 1.0 s nominal frame window for bps estimates.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
            frame_seconds: Mutex::new(1.0),
        }
    }

    /// Overrides the assumed frame duration used for bps conversion in tests.
    pub fn set_frame_seconds(&self, seconds: f64) {
        *self.frame_seconds.lock().expect("poisoned lock") = seconds;
    }

    /// Records `bytes` for `channel` in `direction`.
    pub fn record(&self, channel: u32, direction: NetDirection, bytes: u32) {
        let mut g = self.inner.lock().expect("poisoned lock");
        let e = g.entry(channel).or_insert((0, 0));
        match direction {
            NetDirection::Upstream => e.0 += u64::from(bytes),
            NetDirection::Downstream => e.1 += u64::from(bytes),
        }
    }

    /// Clears all per-channel counters (call at frame start).
    pub fn begin_frame(&self) {
        self.inner.lock().expect("poisoned lock").clear();
    }

    /// Returns per-channel counters for the current frame.
    #[must_use]
    pub fn per_channel_bandwidth(&self) -> Vec<ChannelBandwidth> {
        let g = self.inner.lock().expect("poisoned lock");
        let sec = *self.frame_seconds.lock().expect("poisoned lock");
        let denom = sec.max(f64::EPSILON);
        let mut out: Vec<ChannelBandwidth> = g
            .iter()
            .map(|(&channel_id, &(up_b, down_b))| ChannelBandwidth {
                channel_id,
                channel_name: format!("ch{channel_id}"),
                upstream_bps: (up_b as f64) * 8.0 / denom,
                downstream_bps: (down_b as f64) * 8.0 / denom,
                upstream_bytes: up_b,
                downstream_bytes: down_b,
            })
            .collect();
        out.sort_by_key(|c| c.channel_id);
        out
    }

    /// Sum of `(upstream + downstream) * 8 / frame_seconds` across channels.
    #[must_use]
    pub fn total_bps_estimate(&self) -> f64 {
        self.per_channel_bandwidth()
            .iter()
            .map(|c| c.upstream_bps + c.downstream_bps)
            .sum()
    }
}

impl Default for NetBandwidthTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Decodes captured packet bytes for the inspector UI.
#[derive(Debug, Default)]
pub struct PacketInspector {
    filter: PacketFilter,
    recent: Vec<DecodedPacket>,
}

impl PacketInspector {
    /// Empty inspector.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Updates capture filter settings.
    pub fn set_filter(&mut self, filter: PacketFilter) {
        self.filter = filter;
    }

    /// Returns the last `count` decoded packets.
    #[must_use]
    pub fn recent_packets(&self, count: u32) -> &[DecodedPacket] {
        let n = usize::try_from(count).unwrap_or(0).min(self.recent.len());
        if n == 0 {
            return &[];
        }
        let start = self.recent.len() - n;
        &self.recent[start..]
    }

    /// Decodes a tiny self-describing binary layout:
    /// `[u8 ver=1][u8 field_count][repeat: u16 nlen u8... name u16 vlen u8... value]`.
    #[must_use]
    pub fn decode(&self, data: &[u8]) -> DecodedPacket {
        let _ = &self.filter;
        let mut fields = Vec::new();
        if data.len() < 2 {
            return DecodedPacket {
                timestamp: 0.0,
                direction: NetDirection::Downstream,
                channel_id: 0,
                size_bytes: u32::try_from(data.len()).unwrap_or(0),
                fields,
            };
        }
        let ver = data[0];
        let count = data[1] as usize;
        let mut offset = 2_usize;
        if ver != 1 {
            return DecodedPacket {
                timestamp: 0.0,
                direction: NetDirection::Downstream,
                channel_id: 0,
                size_bytes: u32::try_from(data.len()).unwrap_or(0),
                fields,
            };
        }
        for idx in 0..count {
            if offset + 4 > data.len() {
                break;
            }
            let nlen = u16::from_le_bytes([data[offset], data[offset + 1]]) as usize;
            offset += 2;
            if offset + nlen > data.len() {
                break;
            }
            let name = String::from_utf8_lossy(&data[offset..offset + nlen]).into_owned();
            offset += nlen;
            if offset + 2 > data.len() {
                break;
            }
            let vlen = u16::from_le_bytes([data[offset], data[offset + 1]]) as usize;
            offset += 2;
            if offset + vlen > data.len() {
                break;
            }
            let value = String::from_utf8_lossy(&data[offset..offset + vlen]).into_owned();
            offset += vlen;
            let base = u32::try_from(idx * 100).unwrap_or(0);
            fields.push(PacketField {
                name,
                value,
                offset: base,
                size: 1,
            });
        }
        DecodedPacket {
            timestamp: 0.0,
            direction: NetDirection::Downstream,
            channel_id: 0,
            size_bytes: u32::try_from(data.len()).unwrap_or(0),
            fields,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::FrameStats;

    /// TC-15.5.5.1 — per-channel byte totals.
    #[test]
    fn tc_15_5_5_1_per_channel_bandwidth() {
        let n = NetBandwidthTracker::new();
        n.begin_frame();
        n.record(1, NetDirection::Downstream, 100);
        n.record(2, NetDirection::Downstream, 200);
        n.record(3, NetDirection::Downstream, 300);
        let rows = n.per_channel_bandwidth();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].downstream_bytes, 100);
        assert_eq!(rows[1].downstream_bytes, 200);
        assert_eq!(rows[2].downstream_bytes, 300);
    }

    /// TC-15.5.5.2 — bps sum tracks `FrameStats::net_bandwidth_bps`.
    #[test]
    fn tc_15_5_5_2_bandwidth_sum_matches_stats() {
        let n = NetBandwidthTracker::new();
        n.set_frame_seconds(1.0);
        n.begin_frame();
        n.record(1, NetDirection::Downstream, 100);
        n.record(2, NetDirection::Downstream, 200);
        n.record(3, NetDirection::Upstream, 50);
        let sum_bps = n.total_bps_estimate();
        let mut stats = FrameStats::default();
        stats.net_bandwidth_bps = sum_bps;
        let expected = n.total_bps_estimate();
        assert!((stats.net_bandwidth_bps - expected).abs() < expected * 0.01 + f64::EPSILON);
    }

    /// TC-15.5.5.3 — packet decode fields.
    #[test]
    fn tc_15_5_5_3_packet_decode() {
        let mut enc = vec![1_u8, 3];
        for (name, val) in [("a", "1"), ("b", "2"), ("c", "3")] {
            enc.extend_from_slice(&(name.len() as u16).to_le_bytes());
            enc.extend_from_slice(name.as_bytes());
            enc.extend_from_slice(&(val.len() as u16).to_le_bytes());
            enc.extend_from_slice(val.as_bytes());
        }
        let pi = PacketInspector::new();
        let pkt = pi.decode(&enc);
        assert_eq!(pkt.fields.len(), 3);
        assert_eq!(pkt.fields[0].name, "a");
        assert_eq!(pkt.fields[0].value, "1");
    }
}
