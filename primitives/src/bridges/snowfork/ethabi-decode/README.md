# ethabi-decode

This library decodes ABI-encoded data and event logs. It is a fork of [ethabi](https://github.com/openethereum/ethabi) with a focus on providing decode functionality in environments where `libstd` may not be available.

For compatibility with constrained `no_std` environments, the design of this library differs from the the upstream [ethabi](https://github.com/openethereum/ethabi) in several respects, including:
* ABI's need to be specified as code rather than being loaded from JSON (No SERDE support).
* Use of `Vec<u8>` instead of `std::string::String` for owned strings.
* Anything to do with human-readable error and display output was excised.


## Building

- Build without `libstd`

  ```
  cargo build --no-default-features
  ```

- Build with `libstd`

  ```
  cargo build
  ```

## Example

Decode an event log:
```rust
use ethabi_decode::{Event, ParamKind, Token};

fn decode_event_log(topics: Vec<H256>, data: Vec<u8>) -> Vec<Token> {

    let event = Event {
      signature: "SomeEvent(address,int256)",
      inputs: vec![
        Param { kind: ParamKind::Address, indexed: true },
        Param { kind: ParamKind::Int(256), indexed: false },
      ],
      anonymous: false,
    };

    event.decode(topics, data).unwrap()
}
```