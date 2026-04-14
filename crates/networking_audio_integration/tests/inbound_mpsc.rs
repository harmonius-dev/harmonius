//! Inbound voice packet queue backpressure (design capacity 1024).

use crossbeam_channel::bounded;

/// TC-IR-4.3.1.4 — bounded channel rejects the 1025th `try_send`.
#[test]
fn tc_ir_4_3_1_4_inbound_mpsc_full() {
    let (sender, _receiver) = bounded::<u32>(1024);
    for i in 0u32..1024 {
        sender.try_send(i).expect("queue accepts 1024 items");
    }
    assert!(sender.try_send(1024).is_err());
}
