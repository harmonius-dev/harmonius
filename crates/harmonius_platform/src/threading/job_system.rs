//! Data-parallel helpers built on the work-stealing pool.

use super::pool::ThreadPool;

/// Parallel `for` over `0..count` by submitting one job per index and waiting for all to finish.
///
/// Each index runs exactly once. Intended for [`TC-14.3.1.1`](../../design/platform/threading-test-cases.md)
/// style throughput tests.
pub fn par_iter<F>(pool: &ThreadPool, count: usize, f: F)
where
    F: Fn(usize) + Send + Sync + Copy + 'static,
{
    if count == 0 {
        return;
    }
    let (tx, rx) = crossbeam_channel::bounded(count);
    for i in 0..count {
        let tx_i = tx.clone();
        pool.shared().submit_job(Box::new(move || {
            f(i);
            let _ = tx_i.send(());
        }));
    }
    drop(tx);
    for _ in 0..count {
        rx.recv().expect("worker dropped completion");
    }
}
