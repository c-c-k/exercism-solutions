pub fn square(s: u32) -> u64 {
    1_u64.strict_shl(s - 1)
}

pub fn total() -> u64 {
    u64::MAX
}
