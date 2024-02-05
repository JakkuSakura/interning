// copied from std::ptr::addr_eq to make it usable on stable rust versions
pub fn addr_eq<T: ?Sized, U: ?Sized>(p: *const T, q: *const U) -> bool {
    (p as *const ()) == (q as *const ())
}
