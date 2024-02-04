# Interning
Interning is the process of storing only one copy of each distinct string value, which must be immutable. This process is used to save memory space and improve performance.

## Usage
Add dependencies to your `Cargo.toml`:
```toml
[dependencies]
interning = "0.2"
```

```rust
use interning::InternedString;
fn main() {
    let s1 = InternedString::new("hello");
    let s2 = InternedString::new("hello");
    assert_eq!(s1, s2);
}
```

## Change Log
- 0.1.0
  - Initial release
- 0.2.0
  - Inline small strings
- 0.2.1
  - Add InternedStringHash for endianness-independent hashing and user-friendly API

