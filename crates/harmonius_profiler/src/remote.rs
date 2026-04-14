//! Remote profiling transport stubs (design: `RemoteServer`, `RemoteClient`, `BinaryProtocol`).

use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::frame_collector::FrameCapture;
use crate::ring_buffer::CpuEvent;
use crate::types::{FrameStats, GpuPassTiming};

const WIRE_MAGIC: u32 = 0x48_50_52_31;
const WIRE_VERSION: u32 = 1;

/// Errors decoding [`BinaryProtocol`] payloads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeError {
    /// Frame header or payload is inconsistent.
    InvalidHeader,
    /// Input buffer ends before a complete frame.
    TruncatedData,
    /// Wire version mismatch.
    UnsupportedVersion(u32),
    /// Optional integrity check failed (unused in v1).
    ChecksumMismatch,
}

/// Remote editor ↔ target connection failures.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RemoteError {
    /// No listener accepted the connection.
    ConnectionRefused,
    /// Peer reset an established session.
    ConnectionLost,
    /// Handshake or read deadline exceeded.
    Timeout,
    /// Protocol version mismatch.
    ProtocolMismatch {
        /// Expected wire version.
        expected: u32,
        /// Observed wire version.
        received: u32,
    },
    /// Session exceeded configured bitrate cap.
    BandwidthExceeded,
}

/// Capture granularity for bandwidth control.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaptureGranularity {
    /// Full capture: all events, all stacks.
    Full,
    /// Summary: per-system totals, no call stacks.
    Summary,
    /// Minimal: frame stats only.
    Minimal,
}

/// Varint frame codec for [`FrameCapture`].
#[derive(Debug, Default)]
pub struct BinaryProtocol;

impl BinaryProtocol {
    /// Encodes `capture` to a self-contained byte blob.
    #[must_use]
    pub fn encode(capture: &FrameCapture) -> Vec<u8> {
        let mut out = Vec::new();
        push_u32(&mut out, WIRE_MAGIC);
        push_u32(&mut out, WIRE_VERSION);
        push_u64(&mut out, capture.frame_number);
        push_f64(&mut out, capture.frame_time_ms);
        push_u32(&mut out, u32::try_from(capture.cpu_events.len()).unwrap_or(0));
        for e in &capture.cpu_events {
            push_u64(&mut out, e.begin_tsc);
            push_u64(&mut out, e.end_tsc);
            push_u32(&mut out, e.thread_id);
            push_u32(&mut out, e.zone_name_hash);
            push_u16(&mut out, e.depth);
        }
        push_u32(&mut out, u32::try_from(capture.gpu_passes.len()).unwrap_or(0));
        for g in &capture.gpu_passes {
            push_u32(&mut out, g.pass_id);
            push_f64(&mut out, g.begin_ms);
            push_f64(&mut out, g.end_ms);
            push_f64(&mut out, g.duration_ms);
            let name = g.pass_name.as_bytes();
            push_u32(&mut out, u32::try_from(name.len()).unwrap_or(0));
            out.extend_from_slice(name);
        }
        push_f64(&mut out, capture.stats.cpu_frame_ms);
        push_f64(&mut out, capture.stats.gpu_frame_ms);
        push_u32(&mut out, capture.stats.draw_calls);
        push_u32(&mut out, capture.stats.triangles);
        push_u64(&mut out, capture.stats.gpu_memory_bytes);
        push_u32(&mut out, capture.stats.entity_count);
        push_f64(&mut out, capture.stats.net_bandwidth_bps);
        out
    }

    /// Decodes a [`FrameCapture`] produced by [`BinaryProtocol::encode`].
    pub fn decode(data: &[u8]) -> Result<FrameCapture, DecodeError> {
        let mut i = 0_usize;
        if take_u32(data, &mut i)? != WIRE_MAGIC {
            return Err(DecodeError::InvalidHeader);
        }
        let ver = take_u32(data, &mut i)?;
        if ver != WIRE_VERSION {
            return Err(DecodeError::UnsupportedVersion(ver));
        }
        let frame_number = take_u64(data, &mut i)?;
        let frame_time_ms = take_f64(data, &mut i)?;
        let cpu_len = take_u32(data, &mut i)? as usize;
        let mut cpu_events = Vec::with_capacity(cpu_len.min(4096));
        for _ in 0..cpu_len {
            let begin_tsc = take_u64(data, &mut i)?;
            let end_tsc = take_u64(data, &mut i)?;
            let thread_id = take_u32(data, &mut i)?;
            let zone_name_hash = take_u32(data, &mut i)?;
            let depth = take_u16(data, &mut i)?;
            cpu_events.push(CpuEvent {
                begin_tsc,
                end_tsc,
                thread_id,
                zone_name_hash,
                depth,
            });
        }
        let gpu_len = take_u32(data, &mut i)? as usize;
        let mut gpu_passes = Vec::with_capacity(gpu_len.min(256));
        for _ in 0..gpu_len {
            let pass_id = take_u32(data, &mut i)?;
            let begin_ms = take_f64(data, &mut i)?;
            let end_ms = take_f64(data, &mut i)?;
            let duration_ms = take_f64(data, &mut i)?;
            let nlen = take_u32(data, &mut i)? as usize;
            let name_bytes = take_bytes(data, &mut i, nlen)?;
            let name = String::from_utf8_lossy(name_bytes).into_owned();
            let pass_name: &'static str = Box::leak(name.into_boxed_str());
            gpu_passes.push(GpuPassTiming {
                pass_id,
                pass_name,
                begin_ms,
                end_ms,
                duration_ms,
            });
        }
        let cpu_frame_ms = take_f64(data, &mut i)?;
        let gpu_frame_ms = take_f64(data, &mut i)?;
        let draw_calls = take_u32(data, &mut i)?;
        let triangles = take_u32(data, &mut i)?;
        let gpu_memory_bytes = take_u64(data, &mut i)?;
        let entity_count = take_u32(data, &mut i)?;
        let net_bandwidth_bps = take_f64(data, &mut i)?;
        let stats = FrameStats {
            cpu_frame_ms,
            gpu_frame_ms,
            draw_calls,
            triangles,
            gpu_memory_bytes,
            entity_count,
            net_bandwidth_bps,
        };
        Ok(FrameCapture {
            frame_number,
            frame_time_ms,
            cpu_events,
            gpu_passes,
            stats,
        })
    }
}

fn take_bytes<'a>(
    data: &'a [u8],
    cur: &mut usize,
    n: usize,
) -> Result<&'a [u8], DecodeError> {
    if *cur + n > data.len() {
        return Err(DecodeError::TruncatedData);
    }
    let s = &data[*cur..*cur + n];
    *cur += n;
    Ok(s)
}

fn take_u32(data: &[u8], cur: &mut usize) -> Result<u32, DecodeError> {
    Ok(u32::from_le_bytes(
        take_bytes(data, cur, 4)?
            .try_into()
            .map_err(|_| DecodeError::InvalidHeader)?,
    ))
}

fn take_u64(data: &[u8], cur: &mut usize) -> Result<u64, DecodeError> {
    Ok(u64::from_le_bytes(
        take_bytes(data, cur, 8)?
            .try_into()
            .map_err(|_| DecodeError::InvalidHeader)?,
    ))
}

fn take_u16(data: &[u8], cur: &mut usize) -> Result<u16, DecodeError> {
    Ok(u16::from_le_bytes(
        take_bytes(data, cur, 2)?
            .try_into()
            .map_err(|_| DecodeError::InvalidHeader)?,
    ))
}

fn take_f64(data: &[u8], cur: &mut usize) -> Result<f64, DecodeError> {
    Ok(f64::from_le_bytes(
        take_bytes(data, cur, 8)?
            .try_into()
            .map_err(|_| DecodeError::InvalidHeader)?,
    ))
}

fn push_u32(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn push_u64(out: &mut Vec<u8>, v: u64) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn push_u16(out: &mut Vec<u8>, v: u16) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn push_f64(out: &mut Vec<u8>, v: f64) {
    out.extend_from_slice(&v.to_le_bytes());
}

#[derive(Debug)]
struct RemoteServerInner {
    port: u16,
    listener: Option<TcpListener>,
    granularity: CaptureGranularity,
}

/// TCP accept loop stub for editor connections (QUIC planned).
#[derive(Clone, Debug)]
pub struct RemoteServer {
    inner: Arc<Mutex<RemoteServerInner>>,
}

impl RemoteServer {
    /// Prepares a server for `port` (`0` selects an ephemeral port after `start`).
    #[must_use]
    pub fn new(port: u16) -> Self {
        Self {
            inner: Arc::new(Mutex::new(RemoteServerInner {
                port,
                listener: None,
                granularity: CaptureGranularity::Full,
            })),
        }
    }

    /// Binds `127.0.0.1:self.port`.
    pub fn start(&self) -> std::io::Result<()> {
        let mut g = self.inner.lock().expect("poisoned lock");
        let l = TcpListener::bind(("127.0.0.1", g.port))?;
        g.listener = Some(l);
        Ok(())
    }

    /// Drops the listener.
    pub fn stop(&self) {
        let mut g = self.inner.lock().expect("poisoned lock");
        g.listener = None;
    }

    /// Non-blocking enqueue hook (real send lands with QUIC integration).
    pub fn publish_frame(&self, _capture: &FrameCapture) {}

    /// Sets capture granularity for future sends.
    pub fn set_granularity(&self, granularity: CaptureGranularity) {
        let mut g = self.inner.lock().expect("poisoned lock");
        g.granularity = granularity;
    }

    /// Local socket after [`RemoteServer::start`] (`127.0.0.1:port`).
    pub fn local_addr(&self) -> std::io::Result<SocketAddr> {
        let g = self.inner.lock().expect("poisoned lock");
        let l = g.listener.as_ref().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotConnected, "server not started")
        })?;
        l.local_addr()
    }

    /// Blocks until one client connects (tests spawn this on a worker thread).
    pub fn accept_one(&self) -> std::io::Result<TcpStream> {
        let l = {
            let g = self.inner.lock().expect("poisoned lock");
            g.listener
                .as_ref()
                .ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::NotConnected, "server not started")
                })?
                .try_clone()?
        };
        let (s, _) = l.accept()?;
        Ok(s)
    }
}

/// Editor-side remote profiling client (TCP handshake stub).
#[derive(Debug)]
pub struct RemoteClient {
    stream: Option<TcpStream>,
}

impl RemoteClient {
    /// Disconnected client.
    #[must_use]
    pub fn new() -> Self {
        Self { stream: None }
    }

    /// Attempts a TCP connection to `host:port`.
    pub fn connect(&mut self, host: &str, port: u16) -> Result<(), RemoteError> {
        let addr = (host, port)
            .to_socket_addrs()
            .map_err(|_| RemoteError::ConnectionLost)?
            .next()
            .ok_or(RemoteError::ConnectionLost)?;
        match TcpStream::connect_timeout(&addr, Duration::from_secs(2)) {
            Ok(s) => {
                self.stream = Some(s);
                Ok(())
            }
            Err(e) if e.kind() == std::io::ErrorKind::ConnectionRefused => {
                Err(RemoteError::ConnectionRefused)
            }
            Err(_) => Err(RemoteError::ConnectionLost),
        }
    }

    /// Closes any active stream.
    pub fn disconnect(&mut self) {
        self.stream = None;
    }

    /// Poll hook for decoded frames (not wired for TCP yet).
    #[must_use]
    pub fn poll_frame(&mut self) -> Option<Result<FrameCapture, RemoteError>> {
        None
    }

    /// `true` after a successful [`RemoteClient::connect`].
    #[must_use]
    pub fn is_connected(&self) -> bool {
        self.stream.is_some()
    }

    /// Placeholder bitrate readout.
    #[must_use]
    pub fn bandwidth_usage(&self) -> f64 {
        let _ = &self.stream;
        0.0
    }
}

impl Default for RemoteClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    /// TC-15.5.7.1 — binary roundtrip for a small capture.
    #[test]
    fn tc_15_5_7_1_remote_encode_decode() {
        let cap = FrameCapture {
            frame_number: 7,
            frame_time_ms: 8.0,
            cpu_events: vec![CpuEvent {
                begin_tsc: 1,
                end_tsc: 2,
                thread_id: 3,
                zone_name_hash: 4,
                depth: 0,
            }],
            gpu_passes: vec![GpuPassTiming {
                pass_id: 9,
                pass_name: "main",
                begin_ms: 0.0,
                end_ms: 1.0,
                duration_ms: 1.0,
            }],
            stats: FrameStats {
                cpu_frame_ms: 8.0,
                gpu_frame_ms: 1.0,
                draw_calls: 0,
                triangles: 0,
                gpu_memory_bytes: 0,
                entity_count: 0,
                net_bandwidth_bps: 0.0,
            },
        };
        let bytes = BinaryProtocol::encode(&cap);
        let decoded = BinaryProtocol::decode(&bytes).expect("decode");
        assert_eq!(decoded.frame_number, cap.frame_number);
        assert_eq!(decoded.cpu_events, cap.cpu_events);
        assert_eq!(decoded.gpu_passes.len(), cap.gpu_passes.len());
        assert_eq!(decoded.gpu_passes[0].pass_id, cap.gpu_passes[0].pass_id);
        assert_eq!(decoded.gpu_passes[0].pass_name, cap.gpu_passes[0].pass_name);
        assert!((decoded.stats.cpu_frame_ms - cap.stats.cpu_frame_ms).abs() < f64::EPSILON);
    }

    /// TC-15.5.7.1 #2 — truncated payload.
    #[test]
    fn tc_15_5_7_1_decode_truncated() {
        let err = BinaryProtocol::decode(&[0, 1, 2]).expect_err("truncated");
        assert_eq!(err, DecodeError::TruncatedData);
    }

    /// TC-15.5.7.2 — successful connect vs refused.
    #[test]
    fn tc_15_5_7_2_remote_connect() {
        let srv = RemoteServer::new(0);
        srv.start().expect("bind");
        let port = srv.local_addr().expect("addr").port();
        let acc = srv.clone();
        let handle = thread::spawn(move || acc.accept_one().expect("accept"));
        thread::sleep(Duration::from_millis(30));
        let mut client = RemoteClient::new();
        client.connect("127.0.0.1", port).expect("connect");
        assert!(client.is_connected());
        let _ = handle.join();
        srv.stop();
        let mut bad = RemoteClient::new();
        let refuse_port = {
            let l = TcpListener::bind(("127.0.0.1", 0)).expect("bind");
            let p = l.local_addr().unwrap().port();
            drop(l);
            p
        };
        let err = bad.connect("127.0.0.1", refuse_port).expect_err("refused");
        assert_eq!(err, RemoteError::ConnectionRefused);
    }
}
