
/// Returns true iff the signed value `n` fits into `width` signed bits.
/// 
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    let min_value = -1_i64 << (width - 1);
    let max_value = (1_i64 << (width - 1)) - 1;
    n >= min_value && n <= max_value
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
/// 
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    let max_value = (1_u64 << width) - 1;
    n <= max_value
}

/// Retrieve a signed value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {
    let mask = (1_u64 << width) - 1;
    let value = (word >> lsb) & mask;
    let sign_bit = 1_u64 << (width - 1);

    if value & sign_bit == 0 {
        value as i64
    } else {
        ((value as i64) - (1_i64 << width)) as i64
    }
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
    let mask = (1_u64 << width) - 1;
    (word >> lsb) & mask
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    let max_value = (1_u64 << width) - 1;

    if value <= max_value {
        let mask = max_value << lsb;
        let cleared_word: u64 = word & !mask;
        let packed_word: u64 = cleared_word | ((value & ((1u64 << width) - 1)) << lsb);
        Some(packed_word)
    } else {
        println!("{}", value);
        None
    }
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    let min_value = -(1_i64 << (width - 1));
    let max_value = (1_i64 << (width - 1)) - 1;

    if value >= min_value && value <= max_value {
        let signed_value_to_pack = (value & ((1 << width) - 1)) as u64;
        let mask = ((1u64 << width) - 1) << lsb;
        let cleared_word = word & !mask;
        let packed_word = cleared_word | (signed_value_to_pack << lsb);
        Some(packed_word)
    } else {
        println!("{}", value);
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fitss() {
        assert_eq!(fitss(-1, 3), true); // Should be true
        assert_eq!(fitss(5, 3), false); // Shroud be false
    }

    #[test]
    fn test_fitsu() {
        assert_eq!(fitsu(5, 3), true); // Shroud be true
    }

    #[test]
    fn test_gets() {
        assert_eq!(gets(0x3f4, 6, 2), -3);
    }

    #[test]
    fn test_getu() {
        assert_eq!(getu(0x3f4, 6, 2), 61);
    }

    #[test]
    fn test_newu() {
        assert_eq!(newu(0_u64, 4, 0, 10), Some(10_u64));
        assert_eq!(newu(0b100, 3, 8, 0b110), Some(0b011000000100));
    }

    #[test]
    fn test_news() {
        assert_eq!(news(0b0, 3, 5, -3), Some(0b000010100000));
    }
}
