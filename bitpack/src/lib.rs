pub mod bitpack;


#[cfg(test)]
mod tests {
    use crate::bitpack::{fitss, fitsu, gets, getu, newu, news};

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
