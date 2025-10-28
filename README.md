# unwrap-enum
A crate to generate methods to unwrap enums as certain variants, like `is_some`
and `is_none` on `Option`.

# Example
```rust
use unwrap_enum::{EnumAs, EnumIs};

#[derive(Clone, Debug, EnumAs, EnumIs)]
enum Value {
    String(String),
    Int(i64)
}

let value = Value::String("hello world".to_string());

assert!(value.is_string());
assert!(!value.is_int());
assert!(matches!(value.as_string(), Some(string) if string == "hello world"));
assert!(matches!(value.as_int(), None));
```

# Todo
Implement EnumAsMut and EnumInto derive macros.
