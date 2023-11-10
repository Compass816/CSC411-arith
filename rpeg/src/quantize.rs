pub fn encode(x: f32, bits: u32, cosine_force: f32) -> i32 {
    (scale_sat(x, cosine_force) * smax(bits) as f32).floor() as i32
}

pub fn scale_sat(x: f32, max_magnitude: f32) -> f32 {
    if x > max_magnitude {
        return 1.0
    } else if x < -max_magnitude {
        return -1.0
    } else {
        return x / max_magnitude
    }
}

pub fn smax(bits: u32) -> i32 {
    ((1 << bits) / 2) -1
}