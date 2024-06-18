/// Clamp a value between 0 and 1
pub fn clamp01(value: f64) -> f64 {
    return 0.0f64.max(1.0f64.min(value));
}

/// Linear interpolation between two values
pub fn lerp(from: f64, to: f64, t: f64) -> f64 {
    return from + (to - from) * clamp01(t);
}

pub fn ease_in(t: f64) -> f64 {
    return t.powf(2.5);
}

pub fn ease_out(t: f64) -> f64 {
    let t = 1.0 - t;
    return t.powf(2.5);
}
