# rudp2p_derive

Derive macros for serialize Struct to bits and deserialize bits to Struct.

Example :

```rust
#[derive(SerializeData, DeserializeData)]
pub MyStruct {
    name: String,
    year: i32,
}
```
