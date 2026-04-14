//! TC-IR-3.5.4.U2 — this crate stays on std-only threading contracts (no async runtimes).

use std::fs;
use std::path::PathBuf;

#[test]
fn tc_ir_3_5_4_u2_manifest_has_no_tokio_or_futures() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("Cargo.toml");
    let text = fs::read_to_string(path).expect("read Cargo.toml");
    let lower = text.to_ascii_lowercase();
    assert!(
        !lower.contains("tokio"),
        "integration crate must not depend on tokio (IR-3.5.4.U2)"
    );
    assert!(
        !lower.contains("futures"),
        "integration crate must not depend on futures (IR-3.5.4.U2)"
    );
}
