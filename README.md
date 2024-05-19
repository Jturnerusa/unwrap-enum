# Generate methods to access enum variants.

## Examples
```rust
enum Value {
    String(String),
    Int(i64),
}

fn foo() {
    let val = Value::String("hello world".to_string());

    assert!(val.is_string());
    assert!(!val.is_int());
    
    assert!(matches!(
        val.as_string(),
        Some(Value::String(string) if string == "hello world")
    ));
    
    assert!(matches!(
        val.as_int(),
        None
    ));
}

## Todo
Implement EnumAsMut and EnumInto.
