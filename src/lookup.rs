use crate::InternedStringHash;
use dashmap::DashMap;
use lazy_static::lazy_static;
use std::collections::{hash_map, HashMap};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LocalLookupTable {
    table: HashMap<InternedStringHash, &'static str>,
}

impl LocalLookupTable {
    pub fn new() -> Self {
        Default::default()
    }
    /// Returns if newly interned
    pub fn intern(&mut self, hash: InternedStringHash, s: &'static str) -> &'static str {
        match self.table.entry(hash) {
            hash_map::Entry::Occupied(o) => {
                if *o.get() != s {
                    panic!(
                        "InternedString::intern: duplicate string: {} != {}",
                        s,
                        o.get()
                    )
                }
                o.get()
            }
            hash_map::Entry::Vacant(v) => {
                let global = GLOBAL_LOOKUP_TABLE.intern(hash, s);
                v.insert(global);
                global
            }
        }
    }
    pub fn lookup(&mut self, hash: InternedStringHash) -> Option<&'static str> {
        match self.table.get(&hash) {
            Some(s) => Some(*s),
            None => {
                let looked_up = GLOBAL_LOOKUP_TABLE.lookup(hash)?;
                self.table.insert(hash, looked_up);
                Some(looked_up)
            }
        }
    }
}

pub fn local_lookup(hash: InternedStringHash) -> Option<&'static str> {
    LOCAL_LOOKUP_TABLE.with(|table| table.borrow_mut().lookup(hash))
}

pub fn local_intern(hash: InternedStringHash, s: String) -> &'static str {
    let leaked = Box::leak(s.into_boxed_str());
    let interned: &'static str =
        LOCAL_LOOKUP_TABLE.with(|table| table.borrow_mut().intern(hash, leaked));
    // If the interned string is different from the leaked string, then the string was already interned
    if !std::ptr::addr_eq(interned, leaked) {
        unsafe {
            let boxed = Box::from_raw(leaked as *const str as *mut str);
            drop(boxed)
        }
    }
    interned
}
pub fn local_cleanup() {
    LOCAL_LOOKUP_TABLE.with(|table| table.borrow_mut().table.clear());
}
thread_local! {
    pub static LOCAL_LOOKUP_TABLE: std::cell::RefCell<LocalLookupTable> = std::cell::RefCell::new(LocalLookupTable::new());
}
#[derive(Debug, Clone, Default)]
struct GlobalLookupTable {
    table: DashMap<InternedStringHash, &'static str>,
}
impl GlobalLookupTable {
    pub fn new() -> Self {
        Default::default()
    }
    /// Returns if newly interned
    pub fn intern(&self, hash: InternedStringHash, s: &'static str) -> &'static str {
        match self.table.entry(hash) {
            dashmap::mapref::entry::Entry::Occupied(o) => {
                if *o.get() != s {
                    panic!(
                        "InternedString::intern: duplicate string: {} != {}",
                        s,
                        o.get()
                    )
                }
                o.get()
            }
            dashmap::mapref::entry::Entry::Vacant(v) => {
                v.insert(s);
                s
            }
        }
    }
    pub fn lookup(&self, hash: InternedStringHash) -> Option<&'static str> {
        self.table.get(&hash).map(|s| *s.value())
    }
}
lazy_static! {
    static ref GLOBAL_LOOKUP_TABLE: GlobalLookupTable = GlobalLookupTable::new();
}

pub fn global_cleanup() {
    let to_be_dropped = GLOBAL_LOOKUP_TABLE
        .table
        .iter()
        .map(|x| *x.value())
        .collect::<Vec<_>>();
    GLOBAL_LOOKUP_TABLE.table.clear();
    for k in to_be_dropped {
        unsafe {
            let boxed = Box::from_raw(k as *const str as *mut str);
            drop(boxed)
        }
    }
}
