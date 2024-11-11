/// Helper to for functions implementing the Data trait
/// This sets a byte in a u32 value
pub(crate) fn set_in_u32(value: u32, byte: u8, addr: u32) -> u32 {
    let index = addr % 4;
    (value & !(0xFF << (index * 8))) | (byte as u32) << (index * 8)
}