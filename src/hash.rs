use stable_hash::fast_stable_hash;
use std::fmt::{Debug, Display};
use std::hash::Hash;

pub const HASH_LEN_MASK: u8 = 0x80;

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Copy)]
pub struct InternedStringHash([u8; 8]);
impl Display for InternedStringHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_inlined() {
            write!(f, "{}", self.get_inlined_str())
        } else {
            write!(f, "{:x}", self.hash())
        }
    }
}
impl Debug for InternedStringHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternedStringHash({})", self)
    }
}
impl InternedStringHash {
    pub fn new(hash: u64) -> Self {
        let mut bytes = [0; 8];
        bytes.copy_from_slice(&hash.to_be_bytes());
        Self(bytes)
    }
    pub fn from_str(s: &str) -> Self {
        if s.len() >= 8 {
            let mut hash = (fast_stable_hash(&s) as u64).to_be_bytes();
            hash[0] = 0;
            Self::from_bytes(hash)
        } else {
            let mut hash = [0u8; 8];
            hash[0] = HASH_LEN_MASK | (s.len() as u8);
            hash[1..=s.len()].copy_from_slice(s.as_bytes());
            Self::from_bytes(hash)
        }
    }
    pub fn empty() -> Self {
        Self([0; 8])
    }
    pub fn from_bytes(bytes: [u8; 8]) -> Self {
        Self(bytes)
    }
    pub fn hash(&self) -> u64 {
        u64::from_be_bytes(self.0)
    }
    pub fn is_inlined(&self) -> bool {
        self.0[0] != 0
    }
    pub fn set_inline_len(&mut self, len: usize) {
        debug_assert!(len < 7);
        self.0[0] = HASH_LEN_MASK | len as u8;
    }
    pub fn get_inlined_len(&self) -> usize {
        debug_assert!(self.is_inlined());
        (self.0[0] ^ HASH_LEN_MASK) as usize
    }
    pub fn get_inlined_str(&self) -> &str {
        debug_assert!(self.is_inlined());
        let len = self.get_inlined_len();
        let ptr = self.0.as_ptr();
        let ptr = unsafe { ptr.add(1) };
        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len)) }
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_small_strings() {
        let s = "hello";
        let interned = InternedStringHash::from_str(s);
        assert_eq!(interned.get_inlined_str(), s);
        let interned = InternedStringHash::from_str(s);
        assert_eq!(interned.get_inlined_str(), s);
    }
}
