

/// Returns an i32.
/// 
/// # Arguments:
/// * x: the value wanted to quantize
/// * bits: desired amount of bits and cosine force, which is 0.3 in this case 
pub fn encodes(x: f32, bits: u32, cosine_force: f32) -> i32 {
    (scale_sat(x, cosine_force) * smaxs(bits) as f32 + 0.5).floor() as i32
}

/// Returns an u32.
/// 
/// # Arguments:
/// * x: the value wanted to quantize
/// * bits: desired amount of bits and cosine force, which is 0.3 in this case 
pub fn encodeu(x: f32, bits: u32, cosine_force: f32) -> u32 {
    (scale_sat(x, cosine_force) * smaxu(bits) as f32 + 0.5).floor() as u32
}


/// Returns a f32 which is setting the range.
/// 
/// # Arguments:
/// * x: the value passing
/// * max_magnitude: The highest/lowest you can go
pub fn scale_sat(x: f32, max_magnitude: f32) -> f32 {
    if x > max_magnitude {
        return 1.0
    } else if x < -max_magnitude {
        return -1.0
    } else {
        return x / max_magnitude
    }
}

/// Returns i32 which is used to multiply and scale the inputted value.
/// 
/// # Arguments:
/// * bits: number of bits 
pub fn smaxs(bits: u32) -> i32 {
    ((1 << bits) / 2) -1
}

/// Returns u32 which is used to multiply and scale the inputted value.
/// 
/// # Arguments:
/// * bits: number of bits 
pub fn smaxu(bits: u32) -> u32 {
    ((1 << bits) / 2) -1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encodes() {
        assert_eq!(encodes(0.3, 5, 0.3), 15);
        assert_eq!(encodes(0.1, 5, 0.3), 5);
        assert_eq!(encodes(-0.2, 5, 0.3), -10);
    }
}
