//! Work-stealing [`ThreadPool`] with injector queue and recursive pool-backed
//! [`super::job_system::par_iter`].

use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::thread::JoinHandle as StdJoinHandle;

use crossbeam_deque::{Injector, Steal, Stealer, Worker};
use crossbeam_utils::Backoff;

use super::graph::TaskGraph;
use super::topology::CoreTopology;

type Job = Box<dyn FnOnce() + Send>;

/// Configuration for [`ThreadPool::new`].
#[derive(Clone, Debug)]
pub struct ThreadPoolConfig {
    /// Worker thread count. When `None`, uses hybrid-aware defaults from the design.
    pub worker_count: Option<u32>,
    /// Prefix applied to OS thread names for debugging.
    pub name_prefix: &'static str,
}

impl Default for ThreadPoolConfig {
    fn default() -> Self {
        Self {
            worker_count: None,
            name_prefix: "harmonius",
        }
    }
}

/// Thread priority hint (mapped to OS policies in future platform shims).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThreadPriority {
    /// High-priority worker / performance class.
    High,
    /// Default worker priority.
    Normal,
    /// Main / render style low-priority scheduling.
    Low,
}

/// Shared deque state used by worker threads and [`ThreadPool::spawn`].
pub(crate) struct PoolShared {
    injector: Injector<Job>,
    stealers: Vec<Stealer<Job>>,
    shutdown: AtomicBool,
    active_jobs: AtomicUsize,
    parked: Mutex<bool>,
    wake: Condvar,
}

impl PoolShared {
    fn notify_workers(&self) {
        let _guard = self.parked.lock().expect("park mutex poisoned");
        self.wake.notify_all();
    }

    pub(crate) fn submit_job(&self, job: Job) {
        self.active_jobs.fetch_add(1, Ordering::Relaxed);
        self.injector.push(job);
        self.notify_workers();
    }
}

fn run_job(job: Job, shared: &PoolShared) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        job();
    }));
    shared.active_jobs.fetch_sub(1, Ordering::AcqRel);
    shared.notify_workers();
}

fn worker_main(index: usize, worker: Worker<Job>, shared: Arc<PoolShared>) {
    let backoff = Backoff::new();
    'outer: loop {
        if let Some(job) = worker.pop() {
            run_job(job, &shared);
            backoff.reset();
            continue;
        }
        match shared.injector.steal() {
            Steal::Success(job) => {
                run_job(job, &shared);
                backoff.reset();
                continue;
            }
            Steal::Empty | Steal::Retry => {}
        }
        for (i, stealer) in shared.stealers.iter().enumerate() {
            if i == index {
                continue;
            }
            match stealer.steal() {
                Steal::Success(job) => {
                    run_job(job, &shared);
                    backoff.reset();
                    continue 'outer;
                }
                Steal::Empty | Steal::Retry => {}
            }
        }
        if shared.shutdown.load(Ordering::Acquire) {
            break;
        }
        if backoff.is_completed() {
            let guard = shared.parked.lock().expect("park mutex poisoned");
            let _unused = shared
                .wake
                .wait_timeout(guard, std::time::Duration::from_millis(2))
                .expect("condvar wait");
        } else {
            backoff.snooze();
        }
    }
}

/// Work-stealing thread pool backed by `crossbeam-deque`.
pub struct ThreadPool {
    topology: CoreTopology,
    worker_count: u32,
    shared: Arc<PoolShared>,
    threads: Vec<StdJoinHandle<()>>,
}

impl fmt::Debug for ThreadPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ThreadPool")
            .field("topology", &self.topology)
            .field("worker_count", &self.worker_count)
            .finish_non_exhaustive()
    }
}

fn resolve_worker_count(topology: &CoreTopology, config: &ThreadPoolConfig) -> u32 {
    if let Some(n) = config.worker_count {
        return n.max(1);
    }
    if topology.efficiency_cores.is_empty() {
        topology.total_core_count().saturating_sub(2).max(1)
    } else {
        topology.performance_core_count().max(1)
    }
}

impl ThreadPool {
    /// Spawns workers and begins stealing.
    pub fn new(config: ThreadPoolConfig) -> Self {
        let topology = CoreTopology::detect();
        let worker_count = resolve_worker_count(&topology, &config);
        let n = worker_count as usize;

        let workers: Vec<Worker<Job>> = (0..n).map(|_| Worker::new_lifo()).collect();
        let stealers: Vec<Stealer<Job>> = workers.iter().map(|w| w.stealer()).collect();

        let shared = Arc::new(PoolShared {
            injector: Injector::new(),
            stealers,
            shutdown: AtomicBool::new(false),
            active_jobs: AtomicUsize::new(0),
            parked: Mutex::new(false),
            wake: Condvar::new(),
        });

        let mut threads = Vec::with_capacity(n);
        for (index, worker) in workers.into_iter().enumerate() {
            let shared_thread = Arc::clone(&shared);
            let name = format!("{}-{}", config.name_prefix, index);
            let handle = thread::Builder::new()
                .name(name)
                .spawn(move || worker_main(index, worker, shared_thread))
                .expect("spawn worker");
            threads.push(handle);
        }

        Self {
            topology,
            worker_count,
            shared,
            threads,
        }
    }

    /// Clones the internal [`PoolShared`] handle for recursive job submission (used by
    /// [`super::job_system::par_iter`]).
    pub(crate) fn shared(&self) -> Arc<PoolShared> {
        Arc::clone(&self.shared)
    }

    /// Number of worker threads.
    pub fn worker_count(&self) -> u32 {
        self.worker_count
    }

    /// Topology snapshot used when this pool was constructed.
    pub fn topology(&self) -> &CoreTopology {
        &self.topology
    }

    /// Spawn a detached `'static` task executed on worker threads.
    pub fn spawn<F, R>(&self, f: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let (tx, rx) = crossbeam_channel::bounded(1);
        self.shared.submit_job(Box::new(move || {
            let out = f();
            let _ = tx.send(out);
        }));
        JoinHandle { rx }
    }

    /// Executes a compiled [`TaskGraph`] to completion.
    pub fn execute_graph(&self, graph: TaskGraph) -> JobHandle {
        graph.execute_on(self);
        JobHandle { _private: () }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.shared.shutdown.store(true, Ordering::Release);
        loop {
            match self.shared.injector.steal() {
                Steal::Success(job) => run_job(job, &self.shared),
                Steal::Empty => break,
                Steal::Retry => {}
            }
        }
        self.shared.notify_workers();
        for handle in self.threads.drain(..) {
            let _ = handle.join();
        }
    }
}

/// Handle returned from [`ThreadPool::execute_graph`].
#[derive(Debug)]
pub struct JobHandle {
    _private: (),
}

/// Handle to a spawned task's result.
#[derive(Debug)]
pub struct JoinHandle<R> {
    rx: crossbeam_channel::Receiver<R>,
}

impl<R> JoinHandle<R> {
    /// Returns `true` if the worker has already sent its result.
    pub fn is_complete(&self) -> bool {
        !self.rx.is_empty()
    }

    /// Blocks until the task finishes and returns its output.
    pub fn join(self) -> R {
        self.rx
            .recv()
            .expect("worker panicked without sending result")
    }
}
