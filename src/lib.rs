use crate::hash::{get_hash_inlined_str, is_hash_inlined, stable_hash_string};
use crate::lookup::{local_intern, local_lookup};
use crate::serde_util::BorrowedStrVisitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display};
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

pub mod hash;
pub mod lookup;
mod serde_util;

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub struct InternedString {
    hash: u64,
}

impl InternedString {
    pub fn new(s: impl Into<String>) -> InternedString {
        let string = s.into();
        let hash = stable_hash_string(&string);
        if !is_hash_inlined(hash) {
            local_intern(hash, string);
        }
        InternedString { hash }
    }
    pub fn from_str(s: &str) -> InternedString {
        let hash = stable_hash_string(s);
        if !is_hash_inlined(hash) {
            let looked_up = local_lookup(hash);
            if looked_up.is_none() {
                let string = s.to_string();
                local_intern(hash, string);
            }
        }
        InternedString { hash }
    }

    /// Build a InternedString from a hash. Use with caution as the hash may not be valid.
    pub unsafe fn from_hash(hash: u64) -> InternedString {
        InternedString { hash }
    }
    pub fn as_str(&self) -> &str {
        if is_hash_inlined(self.hash) {
            get_hash_inlined_str(&self.hash)
        } else {
            local_lookup(self.hash).unwrap()
        }
    }
    pub fn hash(&self) -> u64 {
        self.hash
    }
}
impl<'a> From<&'a str> for InternedString {
    fn from(s: &'a str) -> Self {
        InternedString::from_str(s)
    }
}
impl From<String> for InternedString {
    fn from(s: String) -> Self {
        InternedString::new(s)
    }
}
impl From<Box<str>> for InternedString {
    fn from(s: Box<str>) -> Self {
        let s = String::from(s);
        InternedString::new(s)
    }
}
impl From<Arc<str>> for InternedString {
    fn from(s: Arc<str>) -> Self {
        let s = String::from(s.as_ref());
        InternedString::new(s)
    }
}
impl Into<String> for InternedString {
    fn into(self) -> String {
        self.as_str().to_string()
    }
}
impl Into<Box<str>> for InternedString {
    fn into(self) -> Box<str> {
        self.as_str().into()
    }
}
impl Into<Arc<str>> for InternedString {
    fn into(self) -> Arc<str> {
        self.as_str().into()
    }
}
impl FromStr for InternedString {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(InternedString::from_str(s))
    }
}
impl AsRef<str> for InternedString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl Deref for InternedString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
impl Display for InternedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.as_str(), f)
    }
}
impl Debug for InternedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.as_str(), f)
    }
}
impl Serialize for InternedString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for InternedString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = deserializer.deserialize_str(BorrowedStrVisitor)?;
        Ok(InternedString::from_str(val))
    }
}
