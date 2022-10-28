pub(crate) trait ParseBigEndian<T> {
    fn parse_big_endian(self) -> T;
}

impl ParseBigEndian<i64> for [u8; 8] {
    fn parse_big_endian(self) -> i64 {
        ((self[0] as i64) << (8 * 7)) |
            ((self[1] as i64) << (8 * 6)) |
            ((self[2] as i64) << (8 * 5)) |
            ((self[3] as i64) << (8 * 4)) |
            ((self[4] as i64) << (8 * 3)) |
            ((self[5] as i64) << (8 * 2)) |
            ((self[6] as i64) << 8) |
            (self[7] as i64)
    }
}

impl ParseBigEndian<i32> for [u8; 4] {
    fn parse_big_endian(self) -> i32 {
        ((self[0] as i32) << (8 * 3)) |
            ((self[1] as i32) << (8 * 2)) |
            ((self[2] as i32) << 8) |
            (self[3] as i32)
    }
}

impl ParseBigEndian<i16> for [u8; 2] {
    fn parse_big_endian(self) -> i16 {
        ((self[0] as i16) << 8) |
            (self[1] as i16)
    }
}

impl ParseBigEndian<u64> for [u8; 8] {
    fn parse_big_endian(self) -> u64 {
        ((self[0] as u64) << (8 * 7)) |
            ((self[1] as u64) << (8 * 6)) |
            ((self[2] as u64) << (8 * 5)) |
            ((self[3] as u64) << (8 * 4)) |
            ((self[4] as u64) << (8 * 3)) |
            ((self[5] as u64) << (8 * 2)) |
            ((self[6] as u64) << 8) |
            (self[7] as u64)
    }
}

impl ParseBigEndian<u32> for [u8; 4] {
    fn parse_big_endian(self) -> u32 {
        ((self[0] as u32) << (8 * 3)) |
            ((self[1] as u32) << (8 * 2)) |
            ((self[2] as u32) << 8) |
            (self[3] as u32)
    }
}

impl ParseBigEndian<u16> for [u8; 2] {
    fn parse_big_endian(self) -> u16 {
        ((self[0] as u16) << 8) |
            (self[1] as u16)
    }
}
#[cfg(test)]
mod tests {
    use super::ParseBigEndian;
    macro_rules! parse {
    ($arr:expr, $t:ty) => {
        {
            let x: $t = $arr.parse_big_endian();
            x
        }
    };
}
    #[test]
    pub fn parse_u16(){
        assert_eq!(parse!([0xca_u8, 0xfe_u8], u16), 0xcafe_u16);
    }
    #[test]
    pub fn parse_u32(){
        assert_eq!(parse!([0xca_u8, 0xfe_u8, 0xba_u8, 0xbe_u8], u32), 0xcafebabe_u32);
    }
    #[test]
    pub fn parse_u64(){
        assert_eq!(parse!([0xca_u8, 0xfe_u8, 0xba_u8, 0xbe_u8, 0xca_u8, 0xfe_u8, 0xba_u8, 0xbe_u8], u64), 0xcafebabecafebabe_u64);
    }
    #[test]
    pub fn parse_i16(){
        assert_eq!(parse!([0x0a_u8, 0xfe_u8], i16), 0x0afe_i16);
    }
    #[test]
    pub fn parse_i32(){
        assert_eq!(parse!([0x0a_u8, 0xfe_u8, 0xba_u8, 0xbe_u8], i32), 0x0afebabe_i32);
    }
    #[test]
    pub fn parse_i64(){
        assert_eq!(parse!([0x0a_u8, 0xce_u8, 0xba_u8, 0xbe_u8, 0xca_u8, 0xfe_u8, 0xba_u8, 0xbe_u8], i64), 0x0acebabecafebabe_i64);
    }
}