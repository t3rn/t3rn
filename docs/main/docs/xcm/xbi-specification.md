---
sidebar_position: 2
---

# XBI Specification
## Global XBI Types 
```rust
pub type AccountId32 = sp_runtime::AccountId32;
pub type AccountId20 = sp_core::H160;
pub type AssetId = u32;
pub type Data = Vec<u8>;
pub type Id = sp_core::H256;
pub type Gas = u64;
pub type Value = u128; 
pub type ValueEvm = sp_core::U256;
pub type Target = u32;
pub type Timeout = u32;
```

## XBI Metadata
```rust
pub struct XBIMetadata {
    pub id: Id,
    pub dest_para_id: Target,
    pub src_para_id: Target,
    pub sent: Timeout,
    pub delivered: Timeout,
    pub executed: Timeout,
    pub max_exec_cost: Value,
    pub max_notifications_cost: Value,
    pub maybe_known_origin: Option<AccountId32>,
    pub maybe_fee_asset_id: Option<AssetId>,
}
```
### id
Assign your ID to XBI, enabling the search engines to scrape through the XBI Orders across the Parachains Storage
Only XBI Orders with distinct IDs are dispatched via XCM.

### dest_para_id
Parachain ID targeted to execute XBI.

### src_para_id
Parachain ID dispatching the XBI.

### sent
Timeout in seconds (hence independent of source Parachain block time) before XBI is sent from source Parachain via XCM.
Resolves `XBI::Result` with `ErrorSentTimeoutExceeded` if dispatch queues from the source took too long to dispatch.

### delivered
Timeout in seconds (hence independent of target/transition Parachain block time) before XBI is delivered and reaches destination Parachain.
Resolves `XBI::Result` with `ErrorDeliveryTimeoutExceeded` if before the dispatch to XBI check-in queue on the target Parachain the delivery timeout measured relatively to the sent timeout was exceeded.

### executed
Timeout in seconds (hence independent of target Parachain block time) before XBI starts executing.
Resolves `XBI::Result` with `ErrorExecutionTimeoutExceeded` if before the execution on target Parachain the executed timeout measured relatively to the delivered timeout was exceeded.

### max_exec_cost
Maximum allowed execution costs on the destination Parachain.
If no `maybe_fee_asset_id` is set, it's measured as the Parachain native currency units.
Injects automatically into `gas_limit` for Smart Contract calls.

### max_notifications_cost
Maximum allowed notification costs of the spending to retrieve a result from either source or destination Parachains.
If no `maybe_fee_asset_id` is set, it's measured in the units of native to Parachain currency.

### maybe_known_origin
Optional Metadata field allowing to specify the dispatching Origin.

### maybe_fee_asset_id
Optional Metadata field that changes `max_exec_cost` and `max_notifications_cost` as well as `actual_aggregated_costs` of XBI::Result to different currency with defined trade mechanics on source Parachain.

## XBI Instructions
Dynamically defined (without the necessity of runtime upgrades to all Parachains that communicate using XBI) set of Instructions to dispatch on target using XBI Format. It is expected for Parachains to only support a selected set of XBI Instructions, defaulting to `XBI::Unknown` if a given XBI Instruction isn't supported by the target. This is enabled thanks to the custom XBI codec.

### 0 - Unknown
Default to all of the unknown to target Parachain XBI Instructions
```rust
    // 0
    Unknown {
        identifier: u8,
        params: Vec<u8>,
    }
```

### 1 - CallNative
Very generic Instruction assuming target Parachain implements the rules of decoding `payload` into dispatchable arguments to any Substrate Runtime Call.

```rust
    // 1
    CallNative {
      payload: Data,
    }
```

### 2 - CallEvm
Call an EVM smart contract on the target Parachain.
```rust
    // 2
    CallEvm {
      source: AccountId20,
      target: AccountId20,
      value: ValueEvm,
      input: Data,
      gas_limit: Gas,
      max_fee_per_gas: ValueEvm,
      max_priority_fee_per_gas: Option<ValueEvm>,
      nonce: Option<ValueEvm>,
      access_list: Vec<(AccountId20, Vec<Id>)>,
    }
```
### 3 - CallWasm
Call a WASM smart contract (implemented by Substrate's Pallet Contracts) on the target Parachain.
```rust
    // 3
    CallWasm {
      dest: AccountId32,
      value: Value,
      gas_limit: Gas,
      storage_deposit_limit: Option<Value>,
      data: Data,
    }
```

### 4 - CallCustomVM
Call a custom smart contract Virtual Machine on target Parachain.
```rust
    // 4
    CallCustomVM {
      caller: AccountId32,
      dest: AccountId32,
      value: Value,
      input: Data,
      limit: Gas,
      additional_params: Data,
  }
```

### 5 - Transfer
Transfer native to source Parachain currency to target
```rust
    // 5
    Transfer {
        dest: AccountId32,
        value: Value,
    }
```

### 6 - TransferAssets
Transfer fungible currency of given `asset_id` supported by source Parachain to a target supporting the same currency. 
```rust
    // 6
    TransferAssets {
        currency_id: AssetId,
        dest: AccountId32,
        value: Value,
    },
```
### 7 - Swap
Swap fungible currency of the given `asset_in` to another currency of `asset_out`. Target Parachain implements asset ids' trade mechanics and conversions to fungible currencies.

```rust
    // 7
    Swap {
      asset_out: AssetId,
      asset_in: AssetId,
      amount: Value,
      max_limit: Value,
      discount: bool,
    },
```
### 8 - Add Liquidity
Add liquidity in two fungible currencies, A and B, to a DeFi pool implemented on target Parachain. Target Parachain implements the conversions of asset ids to fungible currencies.

```rust
    // 8
    AddLiquidity {
        asset_a: AssetId,
        asset_b: AssetId,
        amount_a: Value,
        amount_b_max_limit: Value,
    },
```

### 9 - Remove Liquidity
Remove liquidity of two fungible currencies, A and B, from a DeFi pool implemented on target Parachain based on the amount of LP-share (liquidity amount). Target Parachain implements the conversions of asset ids to fungible currencies.

```rust
    // 9
    RemoveLiquidity {
        asset_a: AssetId,
        asset_b: AssetId,
        liquidity_amount: Value,
    },
```

### 10 - Get Price
Gets the price exchanging `amount` of currency A to currency B on Target Parachain that implements the trade mechanics and conversions of asset ids to fungible currencies.

```rust
    // 10
    GetPrice {
        asset_a: AssetId,
        asset_b: AssetId,
        amount: Value,
    },
```

### 255 - Result
XBI Result accompanying each XBI Instruction. With XBI, users get the guarantee that each XBI order will be resolved with one of the following outcomes:

```rust
pub enum XBICheckOutStatus {
  // Success scenario
  SuccessfullyExecuted,
  
  // Failed execution scenarios
  ErrorFailedExecution,
  ErrorFailedOnXCMDispatch,
  
  // Failed with exceeded costs scenarios
  ErrorExecutionCostsExceededAllowedMax,
  ErrorNotificationsCostsExceededAllowedMax,
  
  // Failed with exceeded timeout scenarios
  ErrorSentTimeoutExceeded,
  ErrorDeliveryTimeoutExceeded,
  ErrorExecutionTimeoutExceeded,
}
```

The future extensions to XBI foresee communication with remote to Polkadot consensus systems. Therefore field `witness` is already included as part of XBI Result and defaults to an empty bytes vector for all executions that target Parachain.
```rust
    // 255
    Result {
        outcome: XBICheckOutStatus,
        output: Data,
        witness: Data,
        actual_aggregated_costs: Value,
    },
```
