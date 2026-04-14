//! Out-of-process crash monitor binary (`R-14.4.7`).
//!
//! Today this binary exposes an `idle` mode used by unit tests: it blocks on stdin so the parent
//! process can shut it down deterministically. Dump capture, upload, and rotation ship in later
//! slices.

use std::io::Read;

/// Entry point for the Harmonius crash monitor.
fn main() {
    let mut args = std::env::args().skip(1);
    if let Some("idle") = args.next().as_deref() {
        let mut buf = [0u8; 1];
        let _ = std::io::stdin().read_exact(&mut buf);
    }
}
