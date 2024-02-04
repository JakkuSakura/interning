use dashmap::DashMap;
use lazy_static::lazy_static;
use std::collections::{hash_map, HashMap};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LocalLookupTable {
    table: HashMap<u64, &'static str>,
}

impl LocalLookupTable {
    pub fn new() -> Self {
        Default::default()
    }
    /// Returns if newly interned
    pub fn intern(&mut self, hash: u64, s: &'static str) -> bool {
        match self.table.entry(hash) {
            hash_map::Entry::Occupied(o) => {
                if *o.get() != s {
                    panic!(
                        "InternedString::intern: duplicate string: {} != {}",
                        s,
                        o.get()
                    )
                }
                false
            }
            hash_map::Entry::Vacant(v) => {
                v.insert(s);
                GLOBAL_LOOKUP_TABLE.intern(hash, s)
            }
        }
    }
    pub fn lookup(&mut self, hash: u64) -> Option<&'static str> {
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

pub fn local_lookup(hash: u64) -> Option<&'static str> {
    LOCAL_LOOKUP_TABLE.with(|table| table.borrow_mut().lookup(hash))
}

pub fn local_intern(hash: u64, s: String) -> bool {
    let leaked = Box::leak(s.into_boxed_str());
    let inserted = LOCAL_LOOKUP_TABLE.with(|table| table.borrow_mut().intern(hash, leaked));
    if !inserted {
        unsafe {
            let boxed = Box::from_raw(leaked as *const str as *mut str);
            drop(boxed)
        }
    }
    inserted
}
pub fn local_cleanup() {
    LOCAL_LOOKUP_TABLE.with(|table| table.borrow_mut().table.clear());
}
thread_local! {
    pub static LOCAL_LOOKUP_TABLE: std::cell::RefCell<LocalLookupTable> = std::cell::RefCell::new(LocalLookupTable::new());
}
#[derive(Debug, Clone, Default)]
struct GlobalLookupTable {
    table: DashMap<u64, &'static str>,
}
impl GlobalLookupTable {
    pub fn new() -> Self {
        Default::default()
    }
    /// Returns if newly interned
    pub fn intern(&self, hash: u64, s: &'static str) -> bool {
        match self.table.entry(hash) {
            dashmap::mapref::entry::Entry::Occupied(o) => {
                if *o.get() != s {
                    panic!(
                        "InternedString::intern: duplicate string: {} != {}",
                        s,
                        o.get()
                    )
                }
                false
            }
            dashmap::mapref::entry::Entry::Vacant(v) => {
                v.insert(s);
                true
            }
        }
    }
    pub fn lookup(&self, hash: u64) -> Option<&'static str> {
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
