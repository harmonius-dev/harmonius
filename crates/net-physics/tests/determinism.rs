//! Local determinism harness (TC-IR-4.5.4.1 developer mode).

use glam::{Quat, Vec3};
use net_physics::{ContactRange, MAX_ROLLBACK_TICKS, PhysicsSnapshot, SnapshotBuffer};
use sha2::{Digest, Sha256};

#[test]
fn tc_ir_4_5_4_1_local_snapshot_history_hash_stable() {
    let mut buf = SnapshotBuffer::new();
    let last = MAX_ROLLBACK_TICKS as u64 + 20;
    for t in 0u64..last {
        let p = Vec3::new(t as f32, (t / 3) as f32, (t % 7) as f32);
        buf.push(
            t,
            PhysicsSnapshot::new(
                p,
                Quat::IDENTITY,
                Vec3::X * 0.25,
                Vec3::ZERO,
                ContactRange::new((t % 256) as u32, (t % 11) as u16),
                t % 2 == 0,
            ),
        );
    }

    let mut hasher = Sha256::new();
    for t in (last - MAX_ROLLBACK_TICKS as u64)..last {
        let s = buf.get(t).expect("tick in window");
        for c in s.position().to_array() {
            hasher.update(c.to_le_bytes());
        }
        for c in s.linear_velocity().to_array() {
            hasher.update(c.to_le_bytes());
        }
        hasher.update(&[u8::from(s.sleeping())]);
    }
    let a = hasher.finalize();

    let mut hasher = Sha256::new();
    for t in (last - MAX_ROLLBACK_TICKS as u64)..last {
        let s = buf.get(t).expect("tick in window");
        for c in s.position().to_array() {
            hasher.update(c.to_le_bytes());
        }
        for c in s.linear_velocity().to_array() {
            hasher.update(c.to_le_bytes());
        }
        hasher.update(&[u8::from(s.sleeping())]);
    }
    let b = hasher.finalize();

    assert_eq!(a.as_slice(), b.as_slice());
}
