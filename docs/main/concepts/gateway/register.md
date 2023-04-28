---
sidebar_position: 1
---

# Register on Circuit

To enable cross-chain functionality, t3rn relies on gateways that connect the t3rn circuit to target blockchains. A gateway can be seen as an interface, connecting t3rn with external blockchains, allowing cross-chain transactions between them. In this article we will go through the process of registering a new gateway, explaining the different systems that are involved, and the rationale behind them.

## Why Gateways

Before diving into the details of the gateway, it makes sense to reiterate what functionality the gateway enables, and how it is crucial to enable trust-less cross-chain transactions. Triggering a transaction from another blockchain is rather simple. A relayer catches an event on the source blockchain and executes the specified transaction on the target blockchain. While this can be described as a cross-chain transaction, it is not trust-less and requires trusting the relayer. To remove the trust from the relayer, we need to prove the finality of the transaction on the source blockchain. While blockchains can vary in their specific implementation, generally, this is proven by:

1. Proving the transaction is included in a block
2. Proving that the block is finalized on the target blockchain

Verifying these proofs ensures that the specified transaction was finalized on the target blockchain. While this approach is generally used by most blockchains, the implementation can vary. As t3rn is a blockchain agnostic protocol, the finality proofs of all target blockchains need to be verifiable in the t3rn circuit. This requires blockchain specifications to be stored on the t3rn circuit, which is done by registering a gateway. The gateway is made up of two main components that allow the integration of a wide variety of different blockchains.

### XDNS Entry

Firstly, we will take a look at the XDNS entry that is initialized during the gateway registration process. Like with classical DNS entries, an XDNS entry contains parameters required to enable trust-less cross-chain routing of transactions. For one, each XDNS entry has a `url` field, storing an RPC endpoint used for connecting to the target blockchain. This, however, doesn’t suffice as it is also required to verify the finality of a transaction. For that reason, the XDNS entry also stores the cryptographic primitives required for verifying transaction finality proofs. As these can vary from blockchain to blockchain, they need to be specified in each gateway registration.

### Bridge Instance

The bridge instance is the other main component that is initialized on the registration of a new gateway. While the XDNS entry stores the cryptographic primitives needed for transaction finality verification, the bridge instance stores data needed for verifying incoming transaction finality proofs. For example, a gateway connecting a substrate-based blockchain, the bridge receives block headers, as well as the corresponding GRANDPA justification. If the verification of the header is successful it is stored in the circuit, enabling finality proofs for transactions of that block.

We will go through the process of registering a gateway for a substrate-based blockchain. We will look at the different parameters of the transaction and explain what they are used for.

## Create XDNS Entry

As a first step, the `registerGateway` function creates an XDNS entry for the gateway. The entry specifies the target blockchain, storing the parameters needed for verifying finality proofs, building transactions, or generating address keypairs. While these parameters will largely be the same for substrate-based blockchains, the XDNS entry allows any type of blockchain to be connected. As the XDNS entry is easily understood without much context, we will look at the different parameters that are stored.

```rust
pub fn add_new_xdns_record(
    origin: OriginFor<T>,                   // signer
    url: Vec<u8>,                           // b"ws://localhost:9944".to_vec();
    gateway_id: ChainId,                    // [0; 4]
    gateway_abi: GatewayABIConfig,
    gateway_vendor: GatewayVendor,          // GatewayVendor::Substrate;
    gateway_type: GatewayType,              // GatewayType::ProgrammableInternal(0)
    gateway_genesis: GatewayGenesisConfig,
    security_coordinates: Vec<u8>,
    allowed_side_effects: Vec<AllowedSideEffect>,
) -> DispatchResultWithPostInfo
```

### Gateway ABI

The `gateway_abi` contains the parameters required for enabling compatibility of the cryptographic operations. The specifications of the target blockchain defined here enable compatibility between the t3rn circuit and the target blockchain. For example, when verifying a signature created by a target blockchain account, the t3rn circuit will query the `crypto` field the XDNS entry, and use the specified signature scheme to verify the signature.

```rust
pub struct GatewayABIConfig {
    // block number type in bytes
    pub block_number_type_size: u16,    // 32
    // hash size in bytes
    pub hash_size: u16,                 // 32          
    // hashing algorithm
    pub hasher: HasherAlgo,             // HasherAlgo::Blake2,
    // cryptography algorithm
    pub crypto: CryptoAlgo,             // CryptoAlgo::Sr25519
    // address length in bytes
    pub address_length: u16,            // 32
    // value length in bytes
    pub value_type_size: u16,           // 64
    // value length in bytes
    pub decimals: u16,                  // 8
    // value length in bytes. ToDo: move as part of metadata.
    pub structs: Vec<StructDecl>,       // vec![]
}
```

### Gateway Genesis

\[NEED SOME INFO HERE]

```rust
pub struct GatewayGenesisConfig {
    /// SCALE-encoded modules following the format of selected     frame_metadata::RuntimeMetadataVXX
    pub modules_encoded: Option<Vec<u8>>,
    /// SCALE-encoded signed extension - see more at frame_metadata::ExtrinsicMetadata
    pub signed_extension: Option<Vec<u8>>,
    /// Runtime version
    pub runtime_version: sp_version::RuntimeVersion,
    /// Extrinsics version
    pub extrinsics_version: u8,
    /// Genesis hash - block id of the genesis block use to distinct the network and sign messages
    /// Length depending on parameter passed in abi::GatewayABIConfig
    pub genesis_hash: Vec<u8>,
}
```

### Gateway Type

In t3rn there are three different gateway types, namely `intrinsic-programmable`, `extrinsic-programmable`, and `transaction-only`. Selecting a type for the gateway is required, as it defines the functionalities that the gateway enables on the target blockchain.

This is explained in more detail here:

[Gateway Types](types.md)

**Intrinsic Programmable:** used for blockchains that are based on substrate and implement the t3rn gateway pallet.

:::caution
The gateway described in this guide is different than the gateway pallet. Currently, the naming can be a bit confusing and is subject to change.
:::

**Extrinsic Programmable:** used for blockchains that are programmable. It must be noted that substrate-based blockchains can also connect via an `extrinsic-programmable` gateway.

_Example: Ethereum, Solana, Elrond, etc._

**Transaction Only:** used for transaction-only blockchains. These do not offer any kind of programmability and are only used to transfer assets. All blockchains connected as `extrinsic-programmable` or `intrinsic-programmable` can also be connected via a `transaction-only` gateway.

_Example: Bitcoin, Litecoin, etc._

### _Remaining Parameters_

**Origin:** Signer of the transaction

**URL:** RPC endpoint to target blockchain, must be passed as byte encoded array

**Gateway ID:** Unique identifier of the gateway. Used internally to map stored values to the gateway

**Gateway Vendor:** Type of the target blockchain. Currently, Ethereum and substrate-based blockchains are supported

## The Bridge

Once the XDNS entry is created, the t3rn circuit initializes a bridge instance. Before diving into the actual initialization of the bridge, we will take a high-level look at the bridge, explaining the proofs and how they enable finality-proofs of cross-chain transactions. At that point, the general concept should be clear, which makes explaining the initialization simple. It should be especially clear why the presented parameters are required.

### Explaining the Bridge

In this section we will cover the bridge at a high level, explaining how it enables transaction finality proofs and how it fits into the entire system. In substrate-based blockchains (and most other blockchains) transaction finality verification requires two separate proofs:

#### **The transaction is included in a block**

In substrate-based blockchains, transactions are tied to a block by storing them in a Merkle Patricia Tree and storing its root in the blocks header. We can prove a transaction is part of that block, by hashing it with its merkle path. If the resulting hash equals the merkle root stored in the header, we have shown that the transaction is included in the block.

#### **The block is finalized**

After proving that the transaction is included in a block, it is required to prove that the block has been finalized. In substrate-based blockchains, the GRANDPA justification can be used to verify the finality of a block. While the details of this proof are a bit more complex, essentially it contains a block header, signed by the authorities. By verifying these signatures and ensuring that more than 2/3rd of the voters signed the block header, we can prove the finality of the block. A transaction proven to be included in this block is now considered final.

While the bridge is involved in both proofs, in the scope of this guide the focus will be on the latter, the block finality proof. As these proofs are verified in different transactions, it makes sense to explain them in their respective section. In this guide, we will focus on how the bridge verifies and stores incoming block headers, enabling them to be used for verifying the finality of transactions in the block.

When receiving a new block header, along with its GRANDPA justification, the bridge needs to verify two things, proving that the block is valid and finalized. We will explain these here briefly:

#### **Verify GRANDPA justification**

Roughly speaking, the GRANDPA justification contains a block header, signed by the blockchains authorities. Verifying these signatures proves that the authorities are in agreement about the finality of the block. As the correct signature scheme is stored in the gateways XDNS entry, the bridge can verify the proofs.&#x20;

#### **Ensure the GRANDPA justification only includes verified authorities**

While verifying the GRANDPA justification ensures the authorities included in the proof have signed the block header, we need a way of ensuring these addresses are actual authorities. After all, an attacker could create a GRANDPA justification with addresses under its control. For this reason, the t3rn circuit stores the current authorities of a gateway in a list and ensures only these authorities have signed the block header.

If these checks pass, the block header will be stored in the t3rn circuit. As this is an on-chain operation, stored block headers are proven to be correct and can be used for transaction finality verifications indefinitely.

### Initializing the Bridge

As explained above, the finality verification of blocks is dependent on storing the current authority set in the t3rn circuit. The set can be updated by already verified authorities attesting to the validity of new ones. This enables the addition or removal of authorities in a transitive fashion. This, however, results in the initial definition of authorities to be a trusted operation. It is required to set the initial set of authorities once, which begins the verification of blocks. For this reason, it is required to pass them as a parameter in the `registerGateway` function, along with the first block header. As this operation is trusted, the validity of the first block header must not be verified.

```rust
pub fn init_bridge_instance<T: pallet_multi_finality_verifier::Config<I>, I: 'static>(
    origin: T::RuntimeOrigin,
    first_header: Vec<u8>,
    authorities: Option<Vec<T::AccountId>>,
    gateway_id: bp_runtime::ChainId,
) -> DispatchResultWithPostInfo
```

The gateway is now registered and the bridge connecting the circuit to the target blockchain is active. Relayers connecting the two are now able to forward cross-chain transactions, between the two blockchains. In this section, we explained the registration of a substrate-based target blockchain, which utilizes the GRANDPA justifications as the block finality proof. Other finality-proof formats are supported and can be added to the circuit. The goal of t3rn is to be a blockchain agnostic protocol, allowing trustless cross-chain transactions. Understanding the gateway registration process is the first step of getting a deep level of understanding of how the t3rn circuit works.

## Summary

* XDNS entry is created, storing routing information and the cryptographic primitives required for verifying transaction finality
* Store initial authority set
* Store first block header
* XDNS entry and bridge are initialized, allowing transaction finality proofs from the connected blockchain to be verified in the t3rn circuit