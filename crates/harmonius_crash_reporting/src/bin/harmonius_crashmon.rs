//! Out-of-process crash monitor binary (`R-14.4.7`).

#![allow(missing_docs)]

use std::io::Read;

fn main() {
    let mut args = std::env::args().skip(1);
    if let Some("idle") = args.next().as_deref() {
        let mut buf = [0u8; 1];
        let _ = std::io::stdin().read_exact(&mut buf);
    }
}
