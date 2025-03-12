pub const fn murmur_hash3(mut key: u64) -> u64 {
    key ^= key >> 33;
    key *= 0xff51afd7ed558ccd;
    key ^= key >> 33;
    key *= 0xc4ceb9fe1a85ec53;
    key ^= key >> 33;
    return key;
}

// based on boost hash combine from here: https://stackoverflow.com/a/27952689
pub const fn hash_combine(lhs: u64, rhs: u64) -> u64 {
    lhs ^ (rhs + 0x517cc1b727220a95 + (lhs << 6) + (lhs >> 2))
}
