//! Tiny native kernels used to validate logic-graph compilation contracts (**TC-15.8.1.1**).

/// Hand-written reference add.
pub fn hand_add(a: f32, b: f32) -> f32 {
    a + b
}

/// Compiled stand-in for `add(a,b)` produced by the logic graph toolchain.
#[derive(Clone, Copy, Debug, Default)]
pub struct CompiledAdd;

impl CompiledAdd {
    /// Evaluates the compiled kernel.
    pub fn eval(&self, a: f32, b: f32) -> f32 {
        a + b
    }
}

/// Deterministic pseudo-random float in \[0,1) from `seed`.
pub fn rnd01(seed: u32) -> f32 {
    let x = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    (x % 1_000_000) as f32 / 1_000_000.0
}

#[cfg(test)]
mod tests {
    use super::{CompiledAdd, hand_add, rnd01};

    /// **TC-15.8.1.1** — compiled add matches hand evaluation on many pairs.
    #[test]
    fn tc_15_8_1_1_logic_graph_native_compile() {
        let k = CompiledAdd::default();
        for i in 0..1000 {
            let a = rnd01(i);
            let b = rnd01(i.wrapping_add(1));
            assert_eq!(k.eval(a, b), hand_add(a, b));
        }
    }
}
