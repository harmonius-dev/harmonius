//! Plan-driven acceptance tests (`TC-14.3.*`).

use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

use tempfile::NamedTempFile;

static TOPOLOGY_TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

static TC_14_3_1_1_HITS: AtomicUsize = AtomicUsize::new(0);
static TC_14_3_3_1_COUNTER: AtomicUsize = AtomicUsize::new(0);
static TC_14_3_3_2_STATE: Mutex<String> = Mutex::new(String::new());

use crate::threading::{
    compio, par_iter, BufferPool, CoreTopology, IoResult, PlatformIo, TaskGraphBuilder,
    TaskGraphError, ThreadPool, ThreadPoolConfig,
};

#[test]
fn tc_14_3_1_1_par_iter_10k_jobs() {
    TC_14_3_1_1_HITS.store(0, Ordering::Relaxed);
    let pool = ThreadPool::new(ThreadPoolConfig::default());
    par_iter(&pool, 10_000, |_| {
        TC_14_3_1_1_HITS.fetch_add(1, Ordering::Relaxed);
    });
    assert_eq!(TC_14_3_1_1_HITS.load(Ordering::Relaxed), 10_000);
}

#[test]
fn tc_14_3_1_2_worker_count_matches_perf_cores() {
    let lock = TOPOLOGY_TEST_LOCK.get_or_init(|| Mutex::new(()));
    let _guard = lock.lock().expect("topology test lock poisoned");
    std::env::set_var("HARMONIUS_TEST_HYBRID_TOPOLOGY", "1");
    let topo = CoreTopology::detect();
    let pool = ThreadPool::new(ThreadPoolConfig::default());
    assert_eq!(pool.worker_count(), topo.performance_core_count());
    std::env::remove_var("HARMONIUS_TEST_HYBRID_TOPOLOGY");
}

#[test]
fn tc_14_3_3_1_graph_fan_out_fan_in() {
    TC_14_3_3_1_COUNTER.store(0, Ordering::Relaxed);
    let pool = ThreadPool::new(ThreadPoolConfig::default());
    let mut b = TaskGraphBuilder::new();
    let root = b.add_task("root", || {});
    let a = b.add_task("a", || {
        TC_14_3_3_1_COUNTER.fetch_add(1, Ordering::Relaxed);
    });
    let c = b.add_task("c", || {
        TC_14_3_3_1_COUNTER.fetch_add(1, Ordering::Relaxed);
    });
    let d = b.add_task("d", || {
        TC_14_3_3_1_COUNTER.fetch_add(1, Ordering::Relaxed);
    });
    let e = b.add_task("e", || {
        TC_14_3_3_1_COUNTER.fetch_add(1, Ordering::Relaxed);
    });
    let join = b.add_task("join", || {});
    b.add_dependency(root, a);
    b.add_dependency(root, c);
    b.add_dependency(root, d);
    b.add_dependency(root, e);
    b.add_dependency(a, join);
    b.add_dependency(c, join);
    b.add_dependency(d, join);
    b.add_dependency(e, join);
    let graph = b.build().expect("dag");
    pool.execute_graph(graph);
    assert_eq!(TC_14_3_3_1_COUNTER.load(Ordering::Relaxed), 4);
}

#[test]
fn tc_14_3_3_2_graph_nested_subgraph() {
    {
        let mut guard = TC_14_3_3_2_STATE.lock().expect("lock");
        guard.clear();
    }
    let pool = ThreadPool::new(ThreadPoolConfig::default());
    let mut inner = TaskGraphBuilder::new();
    let _inner_task = inner.add_task("inner", || {
        TC_14_3_3_2_STATE.lock().expect("lock").push('I');
    });
    let inner_graph = inner.build().expect("inner");
    let mut outer = TaskGraphBuilder::new();
    let sg = outer.add_subgraph("sub", inner_graph);
    let tail = outer.add_task("tail", || {
        TC_14_3_3_2_STATE.lock().expect("lock").push('T');
    });
    outer.add_dependency(sg, tail);
    let graph = outer.build().expect("outer");
    pool.execute_graph(graph);
    let s = TC_14_3_3_2_STATE.lock().expect("lock").clone();
    assert_eq!(s, "IT");
}

#[test]
fn tc_14_3_3_3_graph_cycle_detection() {
    let mut b = TaskGraphBuilder::new();
    let a = b.add_task("a", || {});
    let c = b.add_task("c", || {});
    let bb = b.add_task("b", || {});
    b.add_dependency(a, bb);
    b.add_dependency(bb, c);
    b.add_dependency(c, a);
    let err = match b.build() {
        Err(e) => e,
        Ok(_) => panic!("expected cycle"),
    };
    assert_eq!(err, TaskGraphError::CycleDetected { cycle: vec![] });
}

#[test]
fn tc_14_3_3_4_graph_empty() {
    let b = TaskGraphBuilder::new();
    let err = match b.build() {
        Err(e) => e,
        Ok(_) => panic!("expected empty graph error"),
    };
    assert_eq!(err, TaskGraphError::EmptyGraph);
}

#[test]
fn tc_14_3_5_1_compio_file_read() {
    let mut file = NamedTempFile::new().expect("tempfile");
    writeln!(file, "hello-plan").expect("write");
    file.flush().expect("flush");
    let bytes = compio::read_file(file.path()).expect("read");
    assert_eq!(bytes, b"hello-plan\n");
}

#[test]
fn tc_14_3_5_2_io_completions_at_frame_boundary() {
    let (tx, rx) = crossbeam_channel::unbounded::<IoResult>();
    let mut io = PlatformIo::new(tx);
    let mut file = NamedTempFile::new().expect("tempfile");
    writeln!(file, "frame").expect("write");
    file.flush().expect("flush");
    let pool = BufferPool::new(64, 1);
    let buf = pool.acquire().expect("buffer");
    io.submit_read(file.path(), buf);
    assert!(rx.try_recv().is_err());
    io.poll_completions();
    let res = rx.recv().expect("completion");
    assert!(res.status.is_ok());
    assert_eq!(res.buffer.as_slice(), b"frame\n");
}

#[test]
fn tc_14_3_5_3_channel_message_delivery() {
    let (tx, rx) = crossbeam_channel::unbounded::<u32>();
    for value in 0..100 {
        tx.send(value).expect("send");
    }
    drop(tx);
    let received: Vec<u32> = rx.iter().collect();
    assert_eq!(received.len(), 100);
    for pair in received.windows(2) {
        assert!(pair[0] < pair[1]);
    }
}
