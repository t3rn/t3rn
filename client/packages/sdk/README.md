# t3rn ts-sdk

The t3rn ts-sdk enables easy client side development with the t3rn blockchain. The main goal is to abstract away a lot of the cross-chain complexity, providing a simple interface to invoke XTX transactions on t3rn. 

This library is under still under development, so proceed with caution.

## SDK
The SDK class used when looking to initialize the entire class, instead of single components. The constructor accepts two parameters, the signer and the circuit RPC endpoint.

```typescript
const keyring = new Keyring({ type: "sr25519" })
this.signer = process.env.CIRCUIT_KEY === undefined
        ? keyring.addFromUri("//Alice")
        : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)

this.sdk = new Sdk("ws://127.0.0.1:9944", this.signer)
// @ts-ignore
this.circuit = await this.sdk.init();
```

## Circuit
The Circuit class is mainly used for dealing with amount conversions and batching and sending transaction objects to the circuit. It deals with error decoding, nonce incrementation and aims to provide a baseline implementation that deals with common situations.

## Gateways
The gateways class is used for exposing functionality of the chains connected to t3rn. When the SDK is initialized, the t3rn blockchain is queried for all XDNS records that are stored on chain. These records contain chain specific constants like value types, address formats, hashing functions, etc. 

For example, when creating a Side Effect to be executed on the Ethereum blockchain, the correct value types, decimals and address format must be ensured. Manually dealing with these validations and conversions is error-prone, potentially resulting in loss of funds.

With the SDK, creating a transfer SideEffect using floats can be done the following way:

```typescript
const data = {
	target: "eth",
    type: "tran",
    to: "0x71C7656EC7ab88b098defB751B7401B5f6d8976F",
    amount: 0.0756,
    maxReward: 1.456,
    insurance: 0.55
}

const obj: T3rnTypesSideEffect = sdk.gateways[data.target].createSfx[data.type]({
    from: sender.toString(),
    to: data.to,
    // createSfx expects Int/BN, passing a float will result in an error
    value: sdk.gateways[data.target].floatToBn(data.amount), 
    maxReward: sdk.circuit.floatToBn(data.reward), // These assets are on circuit
    insurance: sdk.circuit.floatToBn(data.insurance), // This one too
})
```
The gateway class will validate and convert all parameters to the correct encodings and create the SideEffect object, compatible with the t3rn blockchain. The amount conversion is required, as we're passing floats.  


## Helpers
A variety of helpers are exposed via the SDK, that deal with decodings, conversions and validations. 

### AmountConverter
This class is used for doing amount conversions of different types. When dealing with circuit, there are three main encodings are used for representing amounts:

- `LittleEndian` - Used by Substrate as part of the SCALE encoding. The amount is LittleEndian encoded
- `Float` - Floats are the human-readable format of amounts. This is very user facing.
- `Integer` - In combination with decimals, used to represent amounts. These are represented as TS number or BN object

This class enables the conversion between these different types. The gateway class heavily relies on this class, passing parameters like decimals and value types out of the box. This for example is the implementation for `toFloat` in the `Gateway` class. 

```typescript
toFloat(value: BN | number): number {
    return new AmountConverter({
      value,
      decimals: this.decimals,
      valueTypeSize: this.valueTypeSize
    })
    .toFloat()
}
```



It is important to note, that the constructor of the `AmountConverter` only accepts `Integer` or `LittleEndian` types. This needs to be enforced, as TypeScript represents integers as floats. For this reason, it's not possible to differentiate between 1.0 and 1, which is a problem. 

To get around this problem, the `AmountConverter` can also be constructed without passing the value. This enables the `Gateway` class to do conversion from float to int:


```typescript
floatToBn(value: number): BN {
    return new AmountConverter({
      decimals: this.decimals,
      valueTypeSize: this.valueTypeSize,
    }).floatToBn(value);
}
```

### Addresses
Contains helper functions for address related functionality. At this stage this is only available for substrate address, enabling the conversion and validation of addresses and public keys.

### Decoders
A collection of decoding functions is included. This contains a rather arbitrary collection of functions, containing decoders for things we ran into frequently. Currently, this includes only Substrate specific types, but will be extended to other blockchains in the future.  
