//! Encoder handle must not escape the `with_encoder` closure (compile_error).

use harmonius_rg::{with_encoder, Encoder};

fn main() {
    let _escaped: &mut Encoder<'static> = with_encoder(|enc| enc);
}
