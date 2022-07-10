# Departure
Departure is a bytecode library for Lua 5.1, written in rust. It allows you to input Lua 5.1 source code, and compile it into 5.1 bytecode, or deserialize that bytecode into a Rust `struct` that's easy to analyze. 

## Examples

```rust
use departure::deserialize;
use departure::Constant;
use departure::OpCode;

fn main() {
    let source = String::from(r#"
        print("Hello World!")
    "#);
    let deserialized = deserialize(&source).unwrap();

    assert_eq!(deserialized.instructions.len(), 4);
    assert_eq!(&deserialized.instructions[0].op_code, &OpCode::GetGlobal);
    match &deserialized.constants[0] {
        Constant::String(s) => {
            assert_eq!(s, "print");
        }
        _ => {}
    }
}
```