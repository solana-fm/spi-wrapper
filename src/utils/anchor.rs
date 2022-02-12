pub fn unpack_ix_to_sighash(data: &[u8]) -> [u8; 8] {
    let mut ix_data: &[u8] = data;
    let mut sighash: [u8; 8] = [0; 8];
    sighash.copy_from_slice(&ix_data[..8]);
    ix_data = &ix_data[8..];
    sighash
}