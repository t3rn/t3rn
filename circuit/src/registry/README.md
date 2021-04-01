# Pallet Registry

This FRAME pallet contours the onchain contract repository.

Apart from System it has no pallet dependencies. Latest known Substrate commit compatible with this Registry pallet is [`d5f992fc3987301c67c1deccb1bbdccc8ae63cea`](https://github.com/paritytech/substrate/tree/d5f992fc3987301c67c1deccb1bbdccc8ae63cea).

## Interface

#### RegistryContract

```rust
/// A preliminary representation of a contract in the onchain registry.
#[derive(PartialEq, Eq, Encode, Decode, Default, Clone, Debug)]
pub struct RegistryContract {
    code_txt: Vec<u8>,
    bytes: Vec<u8>,
    abi: Option<Vec<u8>>,
}
```

#### store
##### Parameters
```rust
pub fn store(
    origin, // Root origin
    requester: T::AccountId, // Account that requested the execution.
    contract_name: Vec<u8>, // Human-readable contract name.
    contract: RegistryContract // Contract artifacts.
) -> dispatch::DispatchResult
```
##### Description
Stores provided contract in the onchain registry, separately per requester. Fails if a contract's storage key already exists. Root only access.

#### purge
##### Parameters
```rust
pub fn purge(
    origin, // Root origin
    requester: T::AccountId, // Account that requested the execution.
    contract_name: Vec<u8>, // Human-readable contract name.
) -> dispatch::DispatchResult
```
##### Description
Purges a contract from the onchain registry. Fails if a contract's storage key does not exist. Root only access.

#### contract
##### Parameters
```rust
pub fn get(
    requester: T::AccountId, // Account that requested the execution.
    contract_name_hash: T::Hash // Hash of the contract name.
) -> Option<RegistryContract>
```

##### Description
A pallet-level getter exposing the contract registry onchain storage.

## Events

#### ContractStored

##### Parameters
```rust
/// [requester, contract_name]
ContractStored(AccountId, Vec<u8>)
```

##### Description
Emitted for a every stored contract.

#### ContractPurged

##### Parameters
```rust
/// [requester, contract_name]
ContractPurged(AccountId, Vec<u8>)
```

##### Description
Emitted for a every purged contract.

## Errors

#### KeyDoesNotExist

##### Description
The storage key for the contract does not exist. Possibly encountered when purging a contract.

#### KeyAlreadyExists

##### Description
The storage key for the contract does already exist. Possibly encountered when storing a contract.