pub fn read_bit(byte: u8, bit: usize) -> bool {
    let state = (byte >> bit) & 0b1;
    if state == 1 { true } else { false }
}

pub fn read_u16(data: &Vec<u8>, start: usize) -> u16 {
    let end = start + 2;
    let slice = data.get(start..end).unwrap();
    let bytes: [u8; 2] = slice.try_into().unwrap();
    u16::from_le_bytes(bytes)
}

pub fn read_u32(data: &Vec<u8>, start: usize) -> u32 {
    let end = start + 4;
    let slice = data.get(start..end).unwrap();
    let bytes: [u8; 4] = slice.try_into().unwrap();
    u32::from_le_bytes(bytes)
}
