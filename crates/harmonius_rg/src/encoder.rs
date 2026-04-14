//! Scoped pass encoder to tie command recording lifetimes to RAII (`R-2.2.13`).

use std::marker::PhantomData;

/// Encoder handle valid only for the duration of [`with_encoder`]'s closure.
#[derive(Debug)]
pub struct Encoder<'a> {
    _p: PhantomData<&'a mut ()>,
}

impl Encoder<'_> {
    /// No-op record hook for lifetime tests.
    pub fn record_nop(&mut self) {}
}

/// Invokes `f` with a short-lived [`Encoder`].
pub fn with_encoder<F, R>(f: F) -> R
where
    F: for<'a> FnOnce(&'a mut Encoder<'a>) -> R,
{
    let mut enc = Encoder { _p: PhantomData };
    f(&mut enc)
}
