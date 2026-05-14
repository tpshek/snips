fn get_bit(data: &[u8], index: usize) -> bool {
    let byte_index = index / 8;
    let bit_offset = index % 8;
    (data[byte_index] & (1 << bit_offset)) != 0
}

fn set_bit(data: &mut [u8], index: usize) {
    let byte_index = index / 8;
    let bit_offset = index % 8;
    data[byte_index] |= 1 << bit_offset;
}

fn unset_bit(data: &mut [u8], index: usize) {
    let byte_index = index / 8;
    let bit_offset = index % 8;
    data[byte_index] &= !(1 << bit_offset);
}
