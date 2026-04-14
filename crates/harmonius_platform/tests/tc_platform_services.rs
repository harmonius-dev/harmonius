//! Integration tests mapped to `TC-14.*` rows in
//! `docs/design/platform/platform-services-test-cases.md`.

use std::fs;
use std::thread;
use std::time::Duration;

use harmonius_platform::crash::{
    GpuBreadcrumbs, GpuDevice, LogFilter, LogRecord, LogSink, Logger, PassId, PerfCounters,
    CounterName, Severity,
};
use harmonius_platform::filesystem::{
    create_dir_all, delete_batch, enumerate_dir, stat, stat_batch, AsyncFile, CanonicalPath,
    ContentHasher, EnumerateOptions, FileEventKind, FileType, FileWatcher, FsError, OpenFlags,
};
use harmonius_platform::os::Clipboard;
use harmonius_platform::sdk::{
    compare_priority, BuildJob, BuildJobId, BuildPriority, BuildServer, ConsolePlatform,
};
use harmonius_platform::services::{
    AchievementId, AchievementService, BuildCertInputs, CertificationValidator, CloudKey,
    CloudStorageService, EntitlementService, LeaderboardId, LeaderboardRow, LeaderboardScope,
    LeaderboardService, PlatformServices, PresenceState, RichPresenceService, UnlockState,
    VoiceBridge, VoicePlatform,
};
use harmonius_platform::storage::{
    GpuDriverKey, PrefKey, PrefValue, PreferencesStore, PsoCacheStore, TempFileManager,
};

fn tmp() -> tempfile::TempDir {
    tempfile::tempdir().expect("tempdir")
}

fn p(dir: &tempfile::TempDir, name: &str) -> CanonicalPath {
    CanonicalPath::from_path_unverified(&dir.path().join(name))
}

#[test]
fn tc_14_6_7_1_canonical_path_resolve() {
    let p = CanonicalPath::resolve("a/../b/./c").expect("resolve");
    assert_eq!(p.as_str(), "b/c");
    assert!(CanonicalPath::resolve("").is_err());
}

#[test]
fn tc_14_6_7_2_canonical_path_join_components() {
    let p = CanonicalPath::resolve("a/b").unwrap();
    let j = p.join("subdir").unwrap();
    assert_eq!(j.as_str(), "a/b/subdir");
    let f = CanonicalPath::resolve("a/b/file.txt").unwrap();
    assert_eq!(f.file_name(), Some("file.txt"));
    assert_eq!(f.extension(), Some("txt"));
}

#[test]
fn tc_14_6_1_1_async_file_round_trip() {
    let dir = tmp();
    let path = p(&dir, "f.bin");
    let f = AsyncFile::open(&path, OpenFlags::create_new()).expect("create");
    let payload = [0u8; 4096];
    assert_eq!(f.write(&payload, 0).unwrap(), 4096);
    f.flush().unwrap();
    f.close().unwrap();
    let r = AsyncFile::open(&path, OpenFlags::read_only()).unwrap();
    let got = r.read_to_end().unwrap();
    assert_eq!(got.len(), 4096);
}

#[test]
fn tc_14_6_1_2_async_file_read_offset() {
    let dir = tmp();
    let path = p(&dir, "big.bin");
    let f = AsyncFile::open(&path, OpenFlags::create_new()).unwrap();
    let mut buf = [0u8; 8192];
    for i in 0..8192 {
        buf[i] = (i % 256) as u8;
    }
    f.write(&buf, 0).unwrap();
    f.close().unwrap();
    let r = AsyncFile::open(&path, OpenFlags::read_only()).unwrap();
    let mut out = [0u8; 4096];
    let n = r.read(&mut out, 4096).unwrap();
    assert_eq!(n, 4096);
    assert_eq!(&out[..], &buf[4096..]);
}

#[test]
fn tc_14_6_1_3_async_file_missing() {
    let p = CanonicalPath::resolve("missing.dat").unwrap();
    let e = AsyncFile::open(&p, OpenFlags::read_only()).unwrap_err();
    assert_eq!(
        e,
        FsError::NotFound {
            path: "missing.dat".into()
        }
    );
}

#[test]
fn tc_14_6_2_1_create_dir_all() {
    let dir = tmp();
    let root = CanonicalPath::from_path_unverified(dir.path());
    let nested = root.join("a").unwrap().join("b").unwrap().join("c").unwrap().join("d").unwrap();
    create_dir_all(&nested).unwrap();
    assert!(nested.to_path_buf().is_dir());
}

#[test]
fn tc_14_6_2_2_delete_batch() {
    let dir = tmp();
    let mut paths = Vec::new();
    for i in 0..10 {
        let p = dir.path().join(format!("{i}.txt"));
        fs::write(&p, b"x").unwrap();
        paths.push(CanonicalPath::from_path_unverified(&p));
    }
    let mut batch: Vec<_> = paths.iter().take(5).cloned().collect();
    let ghost = p(&dir, "ghost.txt");
    batch.push(ghost.clone());
    let res = delete_batch(&batch);
    assert_eq!(res.iter().filter(|r| r.is_ok()).count(), 5);
    assert!(matches!(
        res.last().unwrap(),
        Err(FsError::NotFound { .. })
    ));
}

#[test]
fn tc_14_6_3_1_stat_metadata() {
    let dir = tmp();
    let fpath = dir.path().join("1k.bin");
    fs::write(&fpath, vec![0u8; 1024]).unwrap();
    let cp = CanonicalPath::from_path_unverified(&fpath);
    let m = stat(&cp).unwrap();
    assert_eq!(m.file_type, FileType::File);
    assert_eq!(m.size, 1024);
    assert!(!m.read_only);
    let dpath = dir.path().join("d");
    fs::create_dir(&dpath).unwrap();
    let dm = stat(&CanonicalPath::from_path_unverified(&dpath)).unwrap();
    assert_eq!(dm.file_type, FileType::Directory);
}

#[test]
fn tc_14_6_3_2_stat_batch() {
    let dir = tmp();
    let mut paths = Vec::new();
    for (i, sz) in [(0u32, 10u64), (1, 20), (2, 30), (3, 40), (4, 50)] {
        let p = dir.path().join(format!("{i}.dat"));
        fs::write(&p, vec![0u8; sz as usize]).unwrap();
        paths.push(CanonicalPath::from_path_unverified(&p));
    }
    let batch = stat_batch(&paths);
    assert_eq!(batch.len(), 5);
    for (r, sz) in batch.iter().zip([10u64, 20, 30, 40, 50]) {
        assert_eq!(r.as_ref().unwrap().size, sz);
    }
}

#[test]
fn tc_14_6_4_1_enumerate_glob() {
    let dir = tmp();
    for name in ["a.txt", "b.txt", "c.txt", "x.png", "y.png"] {
        fs::write(dir.path().join(name), b"x").unwrap();
    }
    let root = CanonicalPath::from_path_unverified(dir.path());
    let opts = EnumerateOptions {
        max_depth: u32::MAX,
        glob: Some("*.txt".into()),
    };
    let list = enumerate_dir(&root, &opts).unwrap();
    assert_eq!(list.len(), 3);
}

#[test]
fn tc_14_6_4_2_enumerate_max_depth_zero() {
    let dir = tmp();
    fs::write(dir.path().join("root.txt"), b"x").unwrap();
    fs::create_dir(dir.path().join("sub")).unwrap();
    fs::write(dir.path().join("sub/nested.txt"), b"x").unwrap();
    let root = CanonicalPath::from_path_unverified(dir.path());
    let opts = EnumerateOptions {
        max_depth: 0,
        glob: None,
    };
    let list = enumerate_dir(&root, &opts).unwrap();
    assert_eq!(list.len(), 1);
    assert!(list[0].name == "root.txt");
}

#[test]
fn tc_14_6_5_1_file_watcher_debounce() {
    let dir = tmp();
    let root = CanonicalPath::from_path_unverified(dir.path());
    let mut w = FileWatcher::new(100).unwrap();
    let (_id, mut stream) = w.watch(&root, false).unwrap();
    let p = dir.path().join("f.txt");
    fs::write(&p, b"0").unwrap();
    thread::sleep(Duration::from_millis(30));
    let _ = stream.poll(&mut w);
    for i in 0..5 {
        fs::write(&p, vec![b'x'; 200 + i]).unwrap();
        thread::sleep(Duration::from_millis(15));
    }
    thread::sleep(Duration::from_millis(120));
    let evs1 = stream.poll(&mut w);
    thread::sleep(Duration::from_millis(120));
    let evs2 = stream.poll(&mut w);
    let mods = evs1
        .iter()
        .chain(evs2.iter())
        .filter(|e| matches!(e.kind, FileEventKind::Modified))
        .count();
    assert!(
        (1..=5).contains(&mods),
        "expected debounced Modified events, got {mods}"
    );
}

#[test]
fn tc_14_6_5_2_file_watcher_create_delete() {
    let dir = tmp();
    let root = CanonicalPath::from_path_unverified(dir.path());
    let mut w = FileWatcher::new(0).unwrap();
    let (_id, mut stream) = w.watch(&root, false).unwrap();
    let p = dir.path().join("n.txt");
    fs::write(&p, b"x").unwrap();
    thread::sleep(Duration::from_millis(30));
    let evs = stream.poll(&mut w);
    assert!(evs.iter().any(|e| matches!(
        e.kind,
        FileEventKind::Created
    )));
    fs::remove_file(&p).unwrap();
    thread::sleep(Duration::from_millis(30));
    let evs2 = stream.poll(&mut w);
    assert!(evs2.iter().any(|e| matches!(e.kind, FileEventKind::Deleted)));
}

#[test]
fn tc_14_6_6_1_content_hash_deterministic() {
    let dir = tmp();
    let p = dir.path().join("blob.bin");
    fs::write(&p, vec![0u8; 1024 * 1024]).unwrap();
    let cp = CanonicalPath::from_path_unverified(&p);
    let h = ContentHasher::new();
    let a = h.hash_file(&cp).unwrap();
    let b = h.hash_file(&cp).unwrap();
    assert_eq!(a, b);
    let mut v = vec![0u8; 1024 * 1024];
    v[0] = 1;
    fs::write(&p, v).unwrap();
    let c = h.hash_file(&cp).unwrap();
    assert_ne!(a, c);
}

#[test]
fn tc_14_6_6_2_content_hash_change_detection() {
    let dir = tmp();
    let p = dir.path().join("x.bin");
    fs::write(&p, b"same").unwrap();
    let cp = CanonicalPath::from_path_unverified(&p);
    let h = ContentHasher::new();
    let old = h.hash_file(&cp).unwrap();
    assert!(!h.has_content_changed(&cp, &old).unwrap());
    fs::write(&p, b"diff").unwrap();
    assert!(h.has_content_changed(&cp, &old).unwrap());
}

#[test]
fn tc_14_2_1_1_clipboard_text() {
    let cb = Clipboard::new();
    cb.set_text("hello").unwrap();
    cb.publish_frame();
    assert_eq!(cb.published().text.as_deref(), Some("hello"));
}

#[test]
fn tc_14_4_4_1_logger_ring_buffer() {
    let filter = LogFilter {
        channel_levels: vec![],
        default_level: Severity::Debug,
    };
    let logger = Logger::new(
        filter,
        vec![LogSink::RingBuffer { capacity: 1024 }],
        1024,
    );
    for i in 0..10 {
        logger.log(&LogRecord {
            channel: "c",
            severity: Severity::Info,
            message: format!("m{i}"),
        });
    }
    assert_eq!(logger.ring_snapshot().len(), 10);
    for i in 0..1025 {
        logger.log(&LogRecord {
            channel: "c",
            severity: Severity::Info,
            message: format!("x{i}"),
        });
    }
    assert_eq!(logger.ring_snapshot().len(), 1024);
}

#[test]
fn tc_14_4_5_1_perf_counters_increment() {
    let c = PerfCounters::new();
    let n = CounterName("frames");
    for _ in 0..100 {
        c.increment(&n);
    }
    let s = c.flush();
    assert!(s.values.contains(&(n.clone(), 100.0)));
    let n2 = CounterName("other");
    c.increment_by(&n2, 5.0);
    c.increment_by(&n2, 5.0);
    let s2 = c.flush();
    assert!(s2.values.contains(&(n2, 10.0)));
}

#[test]
fn tc_14_4_5_2_perf_counter_gauge() {
    let c = PerfCounters::new();
    let n = CounterName("g");
    c.gauge(&n, 42.0);
    c.gauge(&n, 99.0);
    let s = c.flush();
    assert!(s.values.contains(&(n, 99.0)));
}

#[test]
fn tc_14_4_6_1_gpu_breadcrumbs() {
    let dev = GpuDevice::new();
    let gb = GpuBreadcrumbs::new(&dev).unwrap();
    gb.begin_pass(PassId(1));
    gb.end_pass(PassId(1));
    gb.begin_pass(PassId(2));
    assert_eq!(gb.read_last_completed(), Some(PassId(1)));
}

#[test]
fn tc_14_5_11_1_pso_cache_get() {
    let key = GpuDriverKey {
        vendor: 1,
        device: 2,
        driver: "1".into(),
    };
    let mut s = PsoCacheStore::new(key);
    s.store(b"k", b"blob");
    assert_eq!(s.get(b"k"), Some(b"blob".to_vec()));
    assert_eq!(s.get(b"missing"), None);
}

#[test]
fn tc_14_5_11_2_pso_invalidate_all() {
    let k = GpuDriverKey {
        vendor: 0,
        device: 0,
        driver: "x".into(),
    };
    let mut s = PsoCacheStore::new(k);
    for i in 0u8..5 {
        s.store(&[i], &[i]);
    }
    let n = s.invalidate_all();
    assert_eq!(n, 5);
    for i in 0u8..5 {
        assert_eq!(s.get(&[i]), None);
    }
}

#[test]
fn tc_14_5_11_3_pso_driver_isolation() {
    let a = GpuDriverKey {
        vendor: 0x10DE,
        device: 0x2684,
        driver: "535.183".into(),
    };
    let b = GpuDriverKey {
        vendor: 0x10DE,
        device: 0x2684,
        driver: "535.999".into(),
    };
    let mut sa = PsoCacheStore::new(a.clone());
    sa.store(b"k", b"v");
    let mut sb = PsoCacheStore::new(b);
    sb.load_all_from_store(&sa);
    assert_eq!(sb.get(b"k"), None);
    let mut sc = PsoCacheStore::new(a);
    sc.load_all_from_store(&sa);
    assert_eq!(sc.get(b"k"), Some(b"v".to_vec()));
}

#[test]
fn tc_14_5_12_1_temp_allocate_cleanup() {
    let dir = tmp();
    let root = CanonicalPath::from_path_unverified(dir.path());
    let mut m = TempFileManager::init(root, 1_000_000).unwrap();
    let h = m.allocate("scratch.tmp").unwrap();
    assert!(h.path().exists());
    drop(h);
    assert_eq!(m.cleanup_orphans().unwrap(), 1);
}

#[test]
fn tc_14_5_12_3_temp_budget() {
    let dir = tmp();
    let root = CanonicalPath::from_path_unverified(dir.path());
    let mut m = TempFileManager::init(root, 1024).unwrap();
    let e = m.allocate_sized("a.bin", 2048).unwrap_err();
    assert!(matches!(e, harmonius_platform::storage::TempError::BudgetExceeded { .. }));
}

#[test]
fn tc_14_5_8_1_preferences_round_trip() {
    let dir = tmp();
    let path = p(&dir, "p.toml");
    let mut p = PreferencesStore::load(path).unwrap();
    p.set("volume", PrefValue::Float(0.8));
    let v = p.get(&PrefKey {
        key: "volume",
        default: PrefValue::Float(1.0),
    });
    assert_eq!(v, PrefValue::Float(0.8));
    let d = p.get(&PrefKey {
        key: "missing",
        default: PrefValue::Bool(true),
    });
    assert_eq!(d, PrefValue::Bool(true));
}

#[test]
fn tc_14_5_1_1_achievement_unlock_pending() {
    let a = AchievementService::new();
    let id = AchievementId("a1".into());
    a.register(&id, 1);
    a.unlock(&id).unwrap();
    assert_eq!(a.state(&id).state, UnlockState::Pending);
}

#[test]
fn tc_14_5_2_1_leaderboard_pending_and_cache() {
    let lb = LeaderboardService::new();
    let id = LeaderboardId("lb1".into());
    lb.seed_cache(
        &id,
        vec![LeaderboardRow {
            rank: 1,
            player_name: "p".into(),
            score: 10,
            player_id: None,
        }],
    );
    lb.submit(&id, 9999).unwrap();
    assert_eq!(lb.pending_len(), 1);
    let rows = lb.query(&id, LeaderboardScope::Global, 0, 10).unwrap();
    assert_eq!(rows.rows.len(), 1);
}

#[test]
fn tc_14_5_3_1_presence_throttle() {
    let p = RichPresenceService::new();
    let s1 = PresenceState {
        activity: "a".into(),
        zone: None,
        party_size: None,
    };
    let s2 = PresenceState {
        activity: "b".into(),
        zone: None,
        party_size: None,
    };
    p.update(s1).unwrap();
    assert!(p.update(s2.clone()).is_err());
    assert_eq!(p.current(), Some(s2));
}

#[test]
fn tc_14_5_5_3_cloud_quota() {
    let c = CloudStorageService::new();
    c.set_quota(Some(10));
    let k = CloudKey("k".into());
    assert!(c.upload(&k, &[0u8; 20]).is_err());
}

#[test]
fn tc_14_5_6_1_entitlements() {
    let e = EntitlementService::new();
    e.refresh(&["base-game"], &[]);
    assert!(e.is_owned("base-game"));
    assert!(!e.is_owned("unowned-dlc"));
}

#[test]
fn tc_14_5_4_1_voice_bridge() {
    let v = VoiceBridge::new();
    assert_eq!(
        v.start_voice(VoicePlatform::PlayStation).unwrap(),
        "native"
    );
    assert_eq!(v.start_voice(VoicePlatform::Desktop).unwrap(), "vivox");
}

#[test]
fn tc_14_5_7_1_certification() {
    let v = CertificationValidator;
    let ok = BuildCertInputs {
        required_keys: vec!["save_size".into()],
        declarations: vec![("save_size".into(), "1MB".into())],
    };
    assert!(v.validate(&ok).failures.is_empty());
    let bad = BuildCertInputs {
        required_keys: vec!["save_size".into()],
        declarations: vec![],
    };
    assert!(!v.validate(&bad).failures.is_empty());
}

#[test]
fn tc_14_8_5_1_build_queue_priority() {
    let s = BuildServer::new();
    let routine = BuildJob {
        id: BuildJobId(1),
        platform: ConsolePlatform::Ps5,
        priority: BuildPriority::Routine,
        team: "t".into(),
    };
    let cert = BuildJob {
        id: BuildJobId(2),
        platform: ConsolePlatform::Ps5,
        priority: BuildPriority::Certification,
        team: "t".into(),
    };
    s.enqueue(routine);
    s.enqueue(cert);
    let next = s.peek_next().unwrap();
    assert_eq!(next.id, BuildJobId(2));
    assert_eq!(
        compare_priority(BuildPriority::Certification, BuildPriority::Routine),
        std::cmp::Ordering::Less
    );
}

#[test]
fn platform_services_init_shutdown() {
    let mut ps = PlatformServices::init().expect("init");
    ps.shutdown().unwrap();
}
