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
///     0b0000_0000,
///     0b0111_1111,
///     0b0111_1111,
/// ];
/// let integer = synchsafe_to_u32(&synchsafe);
/// assert_eq!(integer, Some(0b00111111_11111111));
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
