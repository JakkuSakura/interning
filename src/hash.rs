use stable_hash::fast_stable_hash;

pub fn stable_hash_string(s: &str) -> u64 {
    fast_stable_hash(&s) as u64
}
