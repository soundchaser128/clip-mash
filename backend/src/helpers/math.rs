use num_traits::Float;

/// Clamp a value between 0 and 1
pub fn clamp01<N: Float>(value: N) -> N {
    value.max(N::zero()).min(N::one())
}

/// Linear interpolation between two values
pub fn lerp<N: Float>(from: N, to: N, t: N) -> N {
    return from + (to - from) * clamp01(t);
}

pub fn lerp_arrays<const N: usize, F: Float>(a: [F; N], b: [F; N], spread: F) -> [F; N] {
    let mut result = [F::zero(); N];
    for i in 0..N {
        result[i] = lerp(a[i], b[i], spread)
    }
    result
}

pub fn ease_in(t: f64) -> f64 {
    return t.powf(2.5);
}

pub fn ease_out(t: f64) -> f64 {
    let t = 1.0 - t;
    return t.powf(2.5);
}
