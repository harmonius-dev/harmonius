//! Benchmarks for IR-5.1.6 / IR-5.1.7 (see design test-case table).

use std::hint::black_box;
use std::time::Instant;

use harmonius_asset_build::cooked_manifest_merkle_root;

fn main() {
    let leaves: Vec<[u8; 32]> = (0u64..100_000)
        .map(|i| *blake3::hash(&i.to_le_bytes()).as_bytes())
        .collect();
    let t0 = Instant::now();
    let root = cooked_manifest_merkle_root(&leaves);
    let elapsed = t0.elapsed();
    println!(
        "merkle_100k_entries: {:?} root={:02x?}",
        elapsed,
        &root[..4]
    );
    black_box(root);
}
