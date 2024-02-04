use stable_hash::fast_stable_hash;

/// TODO: make Hash endianness independent from the machine
pub fn stable_hash_string(s: &str) -> u64 {
    if s.len() >= 8 {
        let mut hash = (fast_stable_hash(&s) as u64).to_be_bytes();
        hash[0] = 0;
        u64::from_be_bytes(hash)
    } else {
        let mut hash = [0u8; 8];
        hash[0] = (s.len() as u8) + 1;
        hash[1..=s.len()].copy_from_slice(s.as_bytes());
        u64::from_be_bytes(hash)
    }
}
pub fn is_hash_inlined(hash: u64) -> bool {
    hash.to_be_bytes()[0] != 0
}
pub fn get_hash_inlined_len(hash: u64) -> usize {
    hash.to_be_bytes()[0] as usize - 1
}
pub fn get_hash_inlined_str(hash: &u64) -> &str {
    debug_assert!(is_hash_inlined(*hash));
    let len = get_hash_inlined_len(*hash);
    let ptr = hash as *const u64 as *const u8;
    let ptr = unsafe { ptr.add(8) };
    unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len)) }
}
