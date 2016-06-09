pub fn align(addr: u64, align_to: u64) -> u64 {
    ((addr - 1) & !(align_to - 1)) + align_to
}
