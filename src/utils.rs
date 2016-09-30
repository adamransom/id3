/// Converts from 'synchsafe' integers to regular integers.
///
/// `synchsafe_to_u32` will return `None` if the length of `buf` is not `4`.
///
/// # Examples
///
/// ```
/// # use id3::utils::synchsafe_to_u32;
/// let synchsafe: [u8; 4] = [
///     0b0000_0000,
///     0b0000_0011,
///     0b0111_1111,
///     0b0111_1111,
/// ];
/// let integer = synchsafe_to_u32(&synchsafe);
/// assert_eq!(integer, Some(0b11111111_11111111));
/// ```
#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn synchsafe_to_u32(buf: &[u8]) -> Option<u32> {
    if buf.len() == 4 {
        Some(
            (buf[0] as u32) << 21 |
            (buf[1] as u32) << 14 |
            (buf[2] as u32) << 7  |
            (buf[3] as u32)
        )
    } else {
        None
    }
}

#[cfg(test)]
mod synchsafe_to_u32_tests {
    use super::synchsafe_to_u32;

    fn assert_eq(bytes: &[u8], expected: u32) {
        let actual = synchsafe_to_u32(bytes);
        assert_eq!(actual, Some(expected));
    }

    #[test]
    fn it_converts() {
        assert_eq(&[0, 0, 0, 0], 0);
        assert_eq(&[0, 0, 0, 0x1], 0x1);
        assert_eq(&[0, 0, 0, 0x7F], 0x7F);
        assert_eq(&[0, 0, 0x1, 0], 0x80);
        assert_eq(&[0, 0, 0x1, 0x7F], 0xFF);
        assert_eq(&[0, 0x3, 0x7F, 0x7F], 0xFFFF);
        assert_eq(&[0x7, 0x7F, 0x7F, 0x7F], 0xFFFFFF);
        assert_eq(&[0x7F, 0x7F, 0x7F, 0x7F], 0xFFFFFFF);
    }

    #[test]
    fn it_returns_none() {
        assert_eq!(synchsafe_to_u32(&[]), None);
    }
}
