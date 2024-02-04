use serde::de::Error;
use std::fmt;

pub struct BorrowedStrVisitor;
impl<'de> serde::de::Visitor<'de> for BorrowedStrVisitor {
    type Value = &'de str;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a borrowed str")
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v)
    }
}
