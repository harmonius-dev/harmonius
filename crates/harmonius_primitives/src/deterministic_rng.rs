//! Deterministic `xoshiro256**` pseudo-random number generator.

#[inline]
fn rotl(x: u64, k: u32) -> u64 {
    (x << k) | (x >> (64u32).wrapping_sub(k))
}

#[inline]
fn splitmix64(x: &mut u64) -> u64 {
    *x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Deterministic RNG based on **xoshiro256\*\***.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    rkyv_derive::Archive,
    rkyv_derive::Deserialize,
    rkyv_derive::Serialize,
)]
pub struct DeterministicRng {
    state: [u64; 4],
}

impl DeterministicRng {
    /// Seeds a generator from `seed`.
    #[must_use]
    pub fn seed(mut seed: u64) -> Self {
        let mut state = [0u64; 4];
        for entry in &mut state {
            *entry = splitmix64(&mut seed);
        }
        Self { state }
    }

    /// Returns the next `u64` in the deterministic stream.
    pub fn next_u64(&mut self) -> u64 {
        let result = rotl(self.state[1].wrapping_mul(5), 7).wrapping_mul(9);
        let t = self.state[1] << 17;
        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];
        self.state[2] ^= t;
        self.state[3] = rotl(self.state[3], 45);
        result
    }

    /// Returns the next `u32` in the deterministic stream.
    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// Returns a float in `[0, 1)` derived from the RNG stream.
    pub fn next_f32_unit(&mut self) -> f32 {
        let u = self.next_u32() >> 8;
        (u as f32) / ((1u32 << 24) as f32)
    }

    /// Returns a value in `[lo, hi)` when `lo < hi`.
    pub fn gen_range(&mut self, lo: i32, hi: i32) -> i32 {
        debug_assert!(lo < hi);
        let range = i64::from(hi) - i64::from(lo);
        debug_assert!(range > 0);
        let span = u64::try_from(range).expect("gen_range span must fit in u64");
        let offset = self.next_u64().wrapping_rem(span) as i32;
        lo.saturating_add(offset)
    }
}
