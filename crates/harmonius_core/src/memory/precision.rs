//! Deterministic arbitrary-precision integers and floats.

/// Little-endian arbitrary-precision integer using `u64` limbs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BigInt {
    limbs: Vec<u64>,
}

impl BigInt {
    /// Constructs zero.
    #[must_use]
    pub fn zero() -> Self {
        Self { limbs: vec![0] }
    }

    /// Constructs from `u128`.
    #[must_use]
    pub fn from_u128(mut v: u128) -> Self {
        if v == 0 {
            return Self::zero();
        }
        let lo = v as u64;
        v >>= 64;
        if v == 0 {
            Self { limbs: vec![lo] }
        } else {
            Self {
                limbs: vec![lo, v as u64],
            }
        }
    }

    /// Serializes limbs as little-endian bytes (portable wire form).
    #[must_use]
    pub fn to_bytes_le(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for limb in &self.limbs {
            out.extend_from_slice(&limb.to_le_bytes());
        }
        out
    }

    /// Multiplies by a small positive integer.
    pub fn mul_u64(&self, rhs: u64) -> Self {
        if rhs == 0 || self.is_zero() {
            return Self::zero();
        }
        let mut carry = 0u128;
        let mut out = Vec::with_capacity(self.limbs.len() + 1);
        for &limb in &self.limbs {
            let prod = u128::from(limb) * u128::from(rhs) + carry;
            out.push(prod as u64);
            carry = prod >> 64;
        }
        if carry != 0 {
            out.push(carry as u64);
        }
        Self::normalize(out)
    }

    /// Adds another [`BigInt`].
    pub fn add(&self, rhs: &Self) -> Self {
        let mut out = Vec::new();
        let mut carry = 0u128;
        let max_len = self.limbs.len().max(rhs.limbs.len());
        for i in 0..max_len {
            let a = self.limbs.get(i).copied().unwrap_or(0);
            let b = rhs.limbs.get(i).copied().unwrap_or(0);
            let s = u128::from(a) + u128::from(b) + carry;
            out.push(s as u64);
            carry = s >> 64;
        }
        if carry != 0 {
            out.push(carry as u64);
        }
        Self::normalize(out)
    }

    fn is_zero(&self) -> bool {
        self.limbs.iter().all(|&x| x == 0)
    }

    fn normalize(mut limbs: Vec<u64>) -> Self {
        while limbs.last() == Some(&0) && limbs.len() > 1 {
            limbs.pop();
        }
        if limbs.is_empty() {
            limbs.push(0);
        }
        Self { limbs }
    }

    /// Lossy conversion to `f64`.
    #[must_use]
    pub fn to_f64(&self) -> f64 {
        let mut acc = 0.0_f64;
        let mut mul = 1.0_f64;
        const LIMB: f64 = (1u128 << 64) as f64;
        for &limb in &self.limbs {
            acc += mul * (limb as f64);
            mul *= LIMB;
        }
        acc
    }

    /// Lossy conversion to `f32`.
    #[must_use]
    pub fn to_f32(&self) -> f32 {
        self.to_f64() as f32
    }
}

/// Fixed-precision float built from a [`BigInt`] significand and power-of-two exponent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BigFloat {
    significand: BigInt,
    exponent: i32,
    _precision_bits: u32,
}

impl BigFloat {
    /// Creates a new [`BigFloat`].
    #[must_use]
    pub fn new(significand: BigInt, exponent: i32, precision_bits: u32) -> Self {
        Self {
            significand,
            exponent,
            _precision_bits: precision_bits,
        }
    }

    /// Rounds to `f32` using the significand bits then applies the exponent scale.
    #[must_use]
    pub fn to_f32(&self) -> f32 {
        let base = self.significand.to_f32();
        base * 2f32.powi(self.exponent)
    }

    /// Rounds to `f64` using the significand bits then applies the exponent scale.
    #[must_use]
    pub fn to_f64(&self) -> f64 {
        let base = self.significand.to_f64();
        base * 2f64.powi(self.exponent)
    }

    /// Significand bits for equality checks.
    #[must_use]
    pub fn significand_bytes(&self) -> Vec<u8> {
        self.significand.to_bytes_le()
    }

    /// Reconstructs a [`BigFloat`] from a positive finite `f32` IEEE-754 bit pattern.
    #[must_use]
    pub fn from_f32(value: f32) -> Self {
        let bits = value.to_bits();
        let exp = ((bits >> 23) & 0xff) as i32;
        let frac = bits & 0x7f_ffff;
        if exp == 0 && frac == 0 {
            return Self::new(BigInt::zero(), 0, 24);
        }
        let (sig, exponent) = if exp == 0 {
            (BigInt::from_u128(u128::from(frac)), -126_i32)
        } else {
            (
                BigInt::from_u128(u128::from(frac | (1 << 23))),
                exp - 127 - 23,
            )
        };
        Self::new(sig, exponent, 24)
    }

    /// Reconstructs a [`BigFloat`] from a positive finite `f64` IEEE-754 bit pattern.
    #[must_use]
    pub fn from_f64(value: f64) -> Self {
        let bits = value.to_bits();
        let exp = ((bits >> 52) & 0x7ff) as i32;
        let frac = bits & 0xf_ffff_ffff_ffff;
        if exp == 0 && frac == 0 {
            return Self::new(BigInt::zero(), 0, 53);
        }
        let frac = frac as u128;
        let (sig, exponent) = if exp == 0 {
            (BigInt::from_u128(frac), -1022_i32)
        } else {
            (BigInt::from_u128(frac | (1u128 << 52)), exp - 1023 - 52)
        };
        Self::new(sig, exponent, 53)
    }
}
