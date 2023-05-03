# Configuration Documentation

The configuration file provides information about the t3rn circuit and gateways. It defines the properties of the circuit, such as its RPC endpoint, decimals, and value type size, and the properties of each gateway, such as its name, ID, token ID, transfer data, and registration data.

The file exports an object with the following properties:

- `circuit`: Object that defines the properties of the t3rn circuit.

  - `rpc`: String that represents the RPC endpoint of the circuit.
  - `decimals`: Number that represents the number of decimal places used by the circuit.
  - `valueTypeSize`: Number that represents the size of the value type in bytes.

- `gateways`: Array of objects that define the properties of each gateway.
  - `name`: String that represents the name of the gateway.
  - `id`: String that represents the ID of the gateway.
  - `rpc`: String that represents the RPC endpoint of the gateway.
  - `subscan`: String that represents the Subscan URL of the gateway.
  - `tokenId`: String that represents the token ID of the gateway.
  - `transferData`: Object that defines the transfer data of the gateway.
    - `receiver`: String that represents the receiver address.
    - `fee`: Number that represents the transfer fee (optional).
  - `registrationData`: Object that defines the registration data of the gateway.
    - `owner`: String that represents the owner address.
    - `parachain`: Object that defines the properties of the parachain (optional).
      - `relayChainId`: String that represents the relay chain ID.
      - `id`: Number that represents the parachain ID.
    - `verificationVendor`: String that represents the verification vendor.
    - `executionVendor`: String that represents the execution vendor.
    - `runtimeCodec`: String that represents the runtime codec.
    - `tokenInfo`: Object that defines the token information.
      - `symbol`: String that represents the token symbol.
      - `decimals`: Number that represents the number of decimal places used by the token.
      - `id`: Number that represents the ID of the token (optional).
      - `address`: String that represents the address of the token (optional).
    - `allowedSideEffects`: Array of tuples that define the allowed side effects. Each tuple consists of a string and a number.
