use crate::hash::stable_hash_string;
use crate::lookup::{local_intern, local_lookup, LOCAL_LOOKUP_TABLE};
use crate::serde_util::BorrowedStrVisitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display};
use std::ops::Deref;
use std::str::FromStr;

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
        local_intern(hash, string);

        InternedString { hash }
    }
    pub fn from_str(s: &str) -> InternedString {
        let hash = stable_hash_string(s);

        let looked_up = local_lookup(hash);
        if looked_up.is_none() {
            let string = s.to_string();
            local_intern(hash, string);
        }
        InternedString { hash }
    }
    pub fn from_str_static(s: &'static str) -> InternedString {
        let hash = stable_hash_string(s);

        LOCAL_LOOKUP_TABLE.with(|table| table.borrow_mut().intern(hash, s));

        InternedString { hash }
    }
    pub fn as_str(&self) -> &str {
        local_lookup(self.hash).unwrap()
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
