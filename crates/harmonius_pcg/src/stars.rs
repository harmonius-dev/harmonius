//! Stellar population sampling for acceptance tests (initial mass function).

/// Salpeter-like IMF draw in solar masses using deterministic hashing.
pub fn stellar_mass_solar(seed: u64, index: u32) -> f64 {
    let u = crate::seed::unit_float(seed, index) as f64;
    let m_min = 0.08_f64;
    let m_max = 100.0_f64;
    let alpha = 2.35_f64;
    let b = 1.0 - alpha;
    (m_min.powf(b) + u * (m_max.powf(b) - m_min.powf(b))).powf(1.0 / b)
}

/// Buckets a solar mass into a coarse spectral class bin.
pub fn spectral_class(m: f64) -> usize {
    if m >= 16.0 {
        0
    } else if m >= 2.1 {
        1
    } else if m >= 1.4 {
        2
    } else if m > 1.04 {
        3
    } else if m > 0.8 {
        4
    } else if m > 0.45 {
        5
    } else {
        6
    }
}

/// Builds a histogram of spectral bins for `n` stars.
pub fn spectral_histogram(seed: u64, n: usize) -> [usize; 7] {
    let mut h = [0usize; 7];
    for i in 0..n as u32 {
        let m = stellar_mass_solar(seed, i);
        h[spectral_class(m)] += 1;
    }
    h
}
