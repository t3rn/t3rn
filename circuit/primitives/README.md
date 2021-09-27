# Circuit Primitives 

A crate that hosts a common definitions that are relevant for the Circuit.

### Runtime Structure `Compose`
```rust
/// Single step of composable execution.
/// Chunk of code bytes instantiated with given input on chain via selected gateway.
pub struct Compose<Account, Balance> {
    pub name: Vec<u8>, /// Compose name
    pub code_txt: Vec<u8>, /// Code in plain text
    pub gateway_id: Account, /// Selected Gateway
    pub exec_type: Vec<u8>, /// Type of execution at Gateway
    pub dest: Option<Account>, /// Optional destination at target chain (either contract or recipient)
    pub value: Option<Balance>, /// Optional value to be transferred to recipient
    pub bytes: Vec<u8>, /// Code bytes
    pub input_data: Vec<u8>, /// Input data 
}
```

License: Apache-2.0