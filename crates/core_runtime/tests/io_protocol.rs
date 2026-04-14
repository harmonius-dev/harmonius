//! Integration coverage for `docs/design/core-runtime/io-test-cases.md` (`TC-*` anchors).

use core_runtime::error::IoError;
use core_runtime::io::{
    IoBackendKind, IoBufferPool, IoDispatcher, IoDispatcherConfig, IoRequest, IoRequestId,
    IoRequestKind, IoResponse, MountBackend, SocketEndpoint, SocketHandle, SwapchainHandle, VPath,
    VirtualFileSystem,
};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

fn temp_root(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "harmonius_core_runtime_io_{}_{}",
        name,
        std::process::id()
    ))
}

// TC-1.8.1.1
#[test]
fn tc_1_8_1_1_io_request_read_file_construction() {
    let mut pool = IoBufferPool::new();
    let buffer = pool.acquire(4096);
    let expected_id = IoRequestId(7);
    let req = IoRequest::ReadFile {
        id: expected_id,
        path: VPath("asset://mesh.bin".to_string()),
        buffer,
    };
    assert_eq!(req.kind(), IoRequestKind::Read);
    assert_eq!(req.id(), expected_id);
}

// TC-1.8.1.2
#[test]
fn tc_1_8_1_2_io_request_send_packet_construction() {
    let mut pool = IoBufferPool::new();
    let mut buffer = pool.acquire(2048);
    let payload = vec![0x55_u8; 1500];
    pool.write_all(&mut buffer, &payload);
    let req = IoRequest::SendPacket {
        id: IoRequestId(1),
        socket: SocketHandle(9),
        buffer,
    };
    assert_eq!(req.kind(), IoRequestKind::Socket);
    match req {
        IoRequest::SendPacket { buffer, .. } => assert_eq!(buffer.len(), 1500),
        _ => panic!("wrong variant"),
    }
}

// TC-1.8.1.3
#[test]
fn tc_1_8_1_3_io_request_swap_chain_present_construction() {
    let req = IoRequest::SwapChainPresent {
        id: IoRequestId(3),
        swapchain: SwapchainHandle(11),
        image_index: 0,
    };
    match req {
        IoRequest::SwapChainPresent {
            swapchain,
            image_index,
            ..
        } => {
            assert_eq!(swapchain, SwapchainHandle(11));
            assert_eq!(image_index, 0);
        }
        _ => panic!("wrong variant"),
    }
}

// TC-1.8.1.4
#[test]
fn tc_1_8_1_4_io_request_cancel_request_carries_target() {
    let req = IoRequest::CancelRequest {
        target: IoRequestId(42),
    };
    assert_eq!(req.cancel_target(), Some(IoRequestId(42)));
}

// TC-1.8.2.1
#[test]
fn tc_1_8_2_1_io_response_read_ok_parsing() {
    let mut pool = IoBufferPool::new();
    let mut buffer = pool.acquire(4096);
    pool.write_all(&mut buffer, &vec![0xCC; 2048]);
    let resp = IoResponse::ReadOk {
        id: IoRequestId(2),
        bytes_read: 2048,
        buffer,
    };
    match resp {
        IoResponse::ReadOk {
            bytes_read, buffer, ..
        } => {
            assert_eq!(bytes_read, 2048);
            assert_eq!(buffer.len(), 2048);
        }
        _ => panic!("wrong variant"),
    }
}

// TC-1.8.2.2
#[test]
fn tc_1_8_2_2_io_response_failed_carries_io_error() {
    let resp = IoResponse::Failed {
        id: IoRequestId(5),
        error: IoError::NotFound {
            path: "missing".to_string(),
        },
    };
    match resp {
        IoResponse::Failed { error, .. } => {
            assert_eq!(
                error,
                IoError::NotFound {
                    path: "missing".to_string()
                }
            );
        }
        _ => panic!("wrong variant"),
    }
}

// TC-1.8.2.3
#[test]
fn tc_1_8_2_3_io_response_cancelled_only_carries_id() {
    let resp = IoResponse::Cancelled {
        id: IoRequestId(99),
    };
    match resp {
        IoResponse::Cancelled { id } => assert_eq!(id, IoRequestId(99)),
        _ => panic!("wrong variant"),
    }
}

// TC-1.8.3.1
#[test]
fn tc_1_8_3_1_virtual_file_system_resolve_asset_mount() {
    let mut vfs = VirtualFileSystem::new();
    let root = temp_root("tc1831");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("deep")).unwrap();
    vfs.mount(
        "asset://",
        MountBackend::LooseFiles {
            path: root.join("deep"),
        },
    );
    let resolved = vfs.resolve(&VPath("asset://mesh.bin".to_string()));
    assert_eq!(resolved, Some(root.join("deep").join("mesh.bin")));
}

// TC-1.8.3.2
#[test]
fn tc_1_8_3_2_virtual_file_system_unknown_prefix() {
    let vfs = VirtualFileSystem::new();
    assert!(vfs.resolve(&VPath("unknown://foo".to_string())).is_none());
}

// TC-1.8.3.3
#[test]
fn tc_1_8_3_3_virtual_file_system_mount_shadowing() {
    let mut vfs = VirtualFileSystem::new();
    let first = temp_root("tc1833a");
    let second = temp_root("tc1833b");
    let _ = fs::remove_dir_all(&first);
    let _ = fs::remove_dir_all(&second);
    fs::create_dir_all(&first).unwrap();
    fs::create_dir_all(&second).unwrap();
    vfs.mount(
        "asset://",
        MountBackend::PackedArchive {
            path: first.clone(),
        },
    );
    vfs.mount(
        "asset://",
        MountBackend::LooseFiles {
            path: second.clone(),
        },
    );
    let resolved = vfs.resolve(&VPath("asset://mesh.bin".to_string()));
    assert_eq!(resolved, Some(second.join("mesh.bin")));
}

// TC-1.8.3.4
#[test]
fn tc_1_8_3_4_asset_loader_end_to_end() {
    let root = temp_root("tc1834");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let mesh_path = root.join("mesh.bin");
    let magic = [0xDE_u8, 0xAD, 0xBE, 0xEF];
    fs::write(&mesh_path, magic).unwrap();

    let mut vfs = VirtualFileSystem::new();
    vfs.mount("asset://", MountBackend::LooseFiles { path: root.clone() });
    let config = IoDispatcherConfig {
        vfs,
        buffers: IoBufferPool::new(),
        backend: IoBackendKind::Loopback,
    };
    let (mut dispatcher, ch) = IoDispatcher::new(config);
    let buffer = dispatcher.buffers_mut().acquire(64);
    let req = IoRequest::ReadFile {
        id: IoRequestId(1),
        path: VPath("asset://mesh.bin".to_string()),
        buffer,
    };
    ch.request_tx.send(req).unwrap();
    dispatcher.poll_completions(0);
    let resp = ch.response_rx.recv().unwrap();
    match resp {
        IoResponse::ReadOk { buffer, .. } => {
            let slice = dispatcher.buffers().as_slice(&buffer);
            assert_eq!(&slice[..4], &magic);
            dispatcher.buffers_mut().release(buffer);
        }
        other => panic!("unexpected response: {other:?}"),
    }
    assert_eq!(dispatcher.buffers().leaked_buffer_count(), 0);
}

// TC-1.8.3.5 — throughput smoke (release builds get tighter budgets).
#[test]
fn tc_1_8_3_5_vfs_resolve_throughput_smoke() {
    let mut vfs = VirtualFileSystem::new();
    let root = temp_root("tc1835");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    for i in 0..16_usize {
        let prefix = Box::leak(format!("v{i:02x}://").into_boxed_str());
        fs::create_dir_all(root.join(format!("slot{i}"))).unwrap();
        vfs.mount(
            prefix,
            MountBackend::LooseFiles {
                path: root.join(format!("slot{i}")),
            },
        );
    }
    let vpath = VPath("v0f://file.bin".to_string());
    let iterations: u64 = 50_000;
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = vfs.resolve(&vpath);
    }
    let elapsed = start.elapsed();
    let per_ns = elapsed.as_nanos() / u128::from(iterations);
    let budget_ns: u128 = if cfg!(debug_assertions) { 2_000 } else { 500 };
    assert!(
        per_ns <= budget_ns,
        "mean resolve latency {per_ns} ns exceeds {budget_ns} ns budget"
    );
}

// TC-1.8.4.1
#[test]
fn tc_1_8_4_1_cancel_moves_ticket_to_cancelled_state() {
    let vfs = VirtualFileSystem::new();
    let config = IoDispatcherConfig {
        vfs,
        buffers: IoBufferPool::new(),
        backend: IoBackendKind::StallReads,
    };
    let (mut dispatcher, ch) = IoDispatcher::new(config);
    let buffer = dispatcher.buffers_mut().acquire(16);
    ch.request_tx
        .send(IoRequest::ReadFile {
            id: IoRequestId(1),
            path: VPath("mem://x".to_string()),
            buffer,
        })
        .unwrap();
    ch.request_tx
        .send(IoRequest::CancelRequest {
            target: IoRequestId(1),
        })
        .unwrap();
    dispatcher.poll_completions(0);
    let resp = ch.response_rx.recv().unwrap();
    assert!(matches!(resp, IoResponse::Cancelled { id } if id == IoRequestId(1)));
    assert_eq!(dispatcher.in_flight_len(), 0);
}

// TC-1.8.4.2
#[test]
fn tc_1_8_4_2_cancel_before_completion() {
    let vfs = VirtualFileSystem::new();
    let config = IoDispatcherConfig {
        vfs,
        buffers: IoBufferPool::new(),
        backend: IoBackendKind::StallReads,
    };
    let (mut dispatcher, ch) = IoDispatcher::new(config);
    let buffer = dispatcher.buffers_mut().acquire(16);
    ch.request_tx
        .send(IoRequest::ReadFile {
            id: IoRequestId(2),
            path: VPath("mem://slow".to_string()),
            buffer,
        })
        .unwrap();
    dispatcher.poll_completions(0);
    assert_eq!(dispatcher.in_flight_len(), 1);
    ch.request_tx
        .send(IoRequest::CancelRequest {
            target: IoRequestId(2),
        })
        .unwrap();
    dispatcher.poll_completions(1);
    let resp = ch.response_rx.recv().unwrap();
    assert!(matches!(resp, IoResponse::Cancelled { id } if id == IoRequestId(2)));
    assert!(ch.response_rx.is_empty());
}

// TC-1.8.5.1
#[test]
fn tc_1_8_5_1_dispatcher_in_flight_ticket_bookkeeping() {
    let vfs = VirtualFileSystem::new();
    let config = IoDispatcherConfig {
        vfs,
        buffers: IoBufferPool::new(),
        backend: IoBackendKind::StallReads,
    };
    let (mut dispatcher, ch) = IoDispatcher::new(config);
    for id in [3_u64, 1, 2] {
        let buffer = dispatcher.buffers_mut().acquire(8);
        ch.request_tx
            .send(IoRequest::ReadFile {
                id: IoRequestId(id),
                path: VPath("mem://q".to_string()),
                buffer,
            })
            .unwrap();
    }
    dispatcher.poll_completions(0);
    assert_eq!(dispatcher.in_flight_len(), 3);
    let ids = dispatcher.in_flight_ids();
    assert_eq!(ids, vec![IoRequestId(1), IoRequestId(2), IoRequestId(3)]);
}

// TC-1.8.5.2
#[test]
fn tc_1_8_5_2_main_thread_drain_single_completion() {
    let vfs = VirtualFileSystem::new();
    let config = IoDispatcherConfig {
        vfs,
        buffers: IoBufferPool::new(),
        backend: IoBackendKind::Loopback,
    };
    let (mut dispatcher, ch) = IoDispatcher::new(config);
    let buffer = dispatcher.buffers_mut().acquire(2048);
    ch.request_tx
        .send(IoRequest::ReadFile {
            id: IoRequestId(4),
            path: VPath("mem://one".to_string()),
            buffer,
        })
        .unwrap();
    dispatcher.poll_completions(0);
    let resp = ch.response_rx.recv().unwrap();
    assert!(matches!(
        resp,
        IoResponse::ReadOk {
            id: IoRequestId(4),
            bytes_read: 1024,
            ..
        }
    ));
}

// TC-1.8.5.3
#[test]
fn tc_1_8_5_3_main_thread_drain_multiple_subsystems() {
    let vfs = VirtualFileSystem::new();
    let config = IoDispatcherConfig {
        vfs,
        buffers: IoBufferPool::new(),
        backend: IoBackendKind::Loopback,
    };
    let (mut dispatcher, ch) = IoDispatcher::new(config);
    let paths = ["mem://asset.bin", "mem://save.bin", "mem://net.bin"];
    let mut ids = Vec::new();
    for (i, p) in paths.iter().enumerate() {
        let id = IoRequestId(10 + i as u64);
        ids.push(id);
        let buffer = dispatcher.buffers_mut().acquire(512);
        ch.request_tx
            .send(IoRequest::ReadFile {
                id,
                path: VPath((*p).to_string()),
                buffer,
            })
            .unwrap();
    }
    dispatcher.poll_completions(0);
    let mut seen = Vec::new();
    for _ in 0..3 {
        seen.push(ch.response_rx.recv().unwrap());
    }
    let read_ids: Vec<IoRequestId> = seen
        .into_iter()
        .map(|r| match r {
            IoResponse::ReadOk { id, .. } => id,
            other => panic!("unexpected {other:?}"),
        })
        .collect();
    assert_eq!(read_ids, ids);
}

// TC-1.8.5.4
#[test]
fn tc_1_8_5_4_main_thread_drain_backpressure() {
    let vfs = VirtualFileSystem::new();
    let config = IoDispatcherConfig {
        vfs,
        buffers: IoBufferPool::new(),
        backend: IoBackendKind::Loopback,
    };
    let (mut dispatcher, ch) = IoDispatcher::new(config);
    for i in 0..10_000_u64 {
        let buffer = dispatcher.buffers_mut().acquire(2048);
        ch.request_tx
            .send(IoRequest::ReadFile {
                id: IoRequestId(i),
                path: VPath("mem://flood".to_string()),
                buffer,
            })
            .unwrap();
    }
    dispatcher.poll_completions(0);
    for i in 0..10_000_u64 {
        let resp = ch.response_rx.recv().unwrap();
        match resp {
            IoResponse::ReadOk { id, buffer, .. } => {
                assert_eq!(id, IoRequestId(i));
                dispatcher.buffers_mut().release(buffer);
            }
            other => panic!("unexpected {other:?}"),
        }
    }
}

// TC-1.8.5.5
#[test]
fn tc_1_8_5_5_ten_k_small_file_reads_under_budget() {
    if cfg!(debug_assertions) {
        // Timing budgets are validated under `--release` to avoid debug noise.
        return;
    }
    let vfs = VirtualFileSystem::new();
    let config = IoDispatcherConfig {
        vfs,
        buffers: IoBufferPool::new(),
        backend: IoBackendKind::Loopback,
    };
    let (mut dispatcher, ch) = IoDispatcher::new(config);
    let start = Instant::now();
    for i in 0..10_000_u64 {
        let buffer = dispatcher.buffers_mut().acquire(2048);
        ch.request_tx
            .send(IoRequest::ReadFile {
                id: IoRequestId(i),
                path: VPath("mem://bench".to_string()),
                buffer,
            })
            .unwrap();
    }
    dispatcher.poll_completions(0);
    for _ in 0..10_000_u64 {
        let resp = ch.response_rx.recv().unwrap();
        if let IoResponse::ReadOk { buffer, .. } = resp {
            dispatcher.buffers_mut().release(buffer);
        }
    }
    assert!(
        start.elapsed() < std::time::Duration::from_millis(250),
        "10k reads exceeded soft budget"
    );
}

// TC-1.8.5.6 — statistical smoke; strict ns targets are host-dependent.
#[test]
fn tc_1_8_5_6_request_submission_throughput_smoke() {
    if cfg!(debug_assertions) {
        return;
    }
    let vfs = VirtualFileSystem::new();
    let config = IoDispatcherConfig {
        vfs,
        buffers: IoBufferPool::new(),
        backend: IoBackendKind::Loopback,
    };
    let (mut dispatcher, ch) = IoDispatcher::new(config);
    let iterations = 50_000_u64;
    let start = Instant::now();
    for i in 0..iterations {
        let buffer = dispatcher.buffers_mut().acquire(512);
        ch.request_tx
            .send(IoRequest::ReadFile {
                id: IoRequestId(i),
                path: VPath("mem://thr".to_string()),
                buffer,
            })
            .unwrap();
    }
    dispatcher.poll_completions(0);
    for _ in 0..iterations {
        if let IoResponse::ReadOk { buffer, .. } = ch.response_rx.recv().unwrap() {
            dispatcher.buffers_mut().release(buffer);
        }
    }
    let elapsed = start.elapsed();
    let per_ns = elapsed.as_nanos() / u128::from(iterations);
    assert!(
        per_ns < 50_000,
        "submission path slower than expected: {per_ns} ns/iter"
    );
}

// TC-1.8.1.5
#[test]
fn tc_1_8_1_5_networking_send_receive_round_trip() {
    let vfs = VirtualFileSystem::new();
    let config = IoDispatcherConfig {
        vfs,
        buffers: IoBufferPool::new(),
        backend: IoBackendKind::Loopback,
    };
    let (mut dispatcher, ch) = IoDispatcher::new(config);

    ch.request_tx
        .send(IoRequest::OpenSocket {
            id: IoRequestId(30),
            endpoint: SocketEndpoint {
                host: "127.0.0.1".to_string(),
                port: 9,
            },
        })
        .unwrap();
    dispatcher.poll_completions(0);
    let open = ch.response_rx.recv().unwrap();
    let socket = match open {
        IoResponse::OpenSocketOk { socket, .. } => socket,
        other => panic!("unexpected {other:?}"),
    };

    let payload = vec![0x5Au8; 1024];
    let mut send_buf = dispatcher.buffers_mut().acquire(2048);
    dispatcher.buffers_mut().write_all(&mut send_buf, &payload);
    ch.request_tx
        .send(IoRequest::SendPacket {
            id: IoRequestId(31),
            socket,
            buffer: send_buf,
        })
        .unwrap();
    dispatcher.poll_completions(1);
    let send_ok = ch.response_rx.recv().unwrap();
    assert!(matches!(
        send_ok,
        IoResponse::SendPacketOk {
            id: IoRequestId(31),
            bytes_sent: 1024
        }
    ));

    let recv_buf = dispatcher.buffers_mut().acquire(2048);
    ch.request_tx
        .send(IoRequest::RecvPacket {
            id: IoRequestId(32),
            socket,
            buffer: recv_buf,
        })
        .unwrap();
    dispatcher.poll_completions(2);
    let recv_ok = ch.response_rx.recv().unwrap();
    match recv_ok {
        IoResponse::RecvPacketOk {
            id,
            bytes_recv,
            buffer,
        } => {
            assert_eq!(id, IoRequestId(32));
            assert_eq!(bytes_recv, 1024);
            assert_eq!(dispatcher.buffers().as_slice(&buffer), payload.as_slice());
            dispatcher.buffers_mut().release(buffer);
        }
        other => panic!("unexpected {other:?}"),
    }
}

// TC-1.8.1.6
#[test]
fn tc_1_8_1_6_save_system_write_file() {
    let root = temp_root("tc1816");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();

    let mut vfs = VirtualFileSystem::new();
    vfs.mount("save://", MountBackend::LooseFiles { path: root.clone() });
    let config = IoDispatcherConfig {
        vfs,
        buffers: IoBufferPool::new(),
        backend: IoBackendKind::Loopback,
    };
    let (mut dispatcher, ch) = IoDispatcher::new(config);
    let mut buffer = dispatcher.buffers_mut().acquire(20_000);
    let bytes = vec![0x77_u8; 16 * 1024];
    dispatcher.buffers_mut().write_all(&mut buffer, &bytes);
    ch.request_tx
        .send(IoRequest::WriteFile {
            id: IoRequestId(40),
            path: VPath("save://slot0.bin".to_string()),
            buffer,
        })
        .unwrap();
    dispatcher.poll_completions(0);
    let resp = ch.response_rx.recv().unwrap();
    assert!(matches!(
        resp,
        IoResponse::WriteOk {
            id: IoRequestId(40),
            bytes_written: 16384
        }
    ));
}

// TC-1.8.6.1
#[test]
fn tc_1_8_6_1_io_buffer_pool_acquire_release() {
    let mut pool = IoBufferPool::new();
    let first = pool.acquire(4096);
    let idx = first.pool_slot_index();
    assert!(first.capacity() >= 4096);
    pool.release(first);
    let second = pool.acquire(4096);
    assert_eq!(second.pool_slot_index(), idx);
    assert_eq!(pool.allocation_count(), 1);
}

// TC-1.8.6.2
#[test]
fn tc_1_8_6_2_io_buffer_pool_bucketed_sizes() {
    let sizes = [64_usize, 512, 4096, 65_536];
    let mut buckets = Vec::new();
    for s in sizes {
        buckets.push(IoBufferPool::bucket_index_for_size(s));
    }
    assert_eq!(buckets, vec![0, 1, 2, 3]);
}

// TC-1.8.6.3
#[test]
fn tc_1_8_6_3_io_buffer_pool_acquire_throughput_smoke() {
    if cfg!(debug_assertions) {
        return;
    }
    let mut pool = IoBufferPool::new();
    let iterations: u64 = 200_000;
    let start = Instant::now();
    for _ in 0..iterations {
        let buf = pool.acquire(4096);
        pool.release(buf);
    }
    let per_ns = start.elapsed().as_nanos() / u128::from(iterations);
    assert!(
        per_ns < 5_000,
        "pool acquire/release slower than expected: {per_ns} ns/iter"
    );
}
