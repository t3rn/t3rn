// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/api-base/types/events";

import type { ApiTypes, AugmentedEvent } from "@polkadot/api-base/types";
import type {
  Bytes,
  Null,
  Option,
  Result,
  U8aFixed,
  Vec,
  bool,
  u128,
  u32,
  u64,
} from "@polkadot/types-codec";
import type { ITuple } from "@polkadot/types-codec/types";
import type { AccountId32, H256 } from "@polkadot/types/interfaces/runtime";
import type {
  FrameSupportTokensMiscBalanceStatus,
  FrameSupportWeightsDispatchInfo,
  SpFinalityGrandpaAppPublic,
  SpRuntimeDispatchError,
  T3rnPrimitivesGatewaySysProps,
  T3rnPrimitivesGatewayType,
  T3rnPrimitivesGatewayVendor,
  T3rnPrimitivesSideEffectFullSideEffect,
  T3rnTypesSideEffect,
} from "@polkadot/types/lookup";

export type __AugmentedEvent<ApiType extends ApiTypes> =
  AugmentedEvent<ApiType>;

declare module "@polkadot/api-base/types/events" {
  interface AugmentedEvents<ApiType extends ApiTypes> {
    accountManager: {
      DepositReceived: AugmentedEvent<
        ApiType,
        [
          executionId: u64,
          payee: AccountId32,
          recipient: AccountId32,
          amount: u128
        ],
        {
          executionId: u64;
          payee: AccountId32;
          recipient: AccountId32;
          amount: u128;
        }
      >;
      ExecutionFinalized: AugmentedEvent<
        ApiType,
        [executionId: u64],
        { executionId: u64 }
      >;
      Issued: AugmentedEvent<
        ApiType,
        [recipient: AccountId32, amount: u128],
        { recipient: AccountId32; amount: u128 }
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    balances: {
      /** A balance was set by root. */
      BalanceSet: AugmentedEvent<
        ApiType,
        [who: AccountId32, free: u128, reserved: u128],
        { who: AccountId32; free: u128; reserved: u128 }
      >;
      /** Some amount was deposited (e.g. for transaction fees). */
      Deposit: AugmentedEvent<
        ApiType,
        [who: AccountId32, amount: u128],
        { who: AccountId32; amount: u128 }
      >;
      /**
       * An account was removed whose balance was non-zero but below
       * ExistentialDeposit, resulting in an outright loss.
       */
      DustLost: AugmentedEvent<
        ApiType,
        [account: AccountId32, amount: u128],
        { account: AccountId32; amount: u128 }
      >;
      /** An account was created with some free balance. */
      Endowed: AugmentedEvent<
        ApiType,
        [account: AccountId32, freeBalance: u128],
        { account: AccountId32; freeBalance: u128 }
      >;
      /** Some balance was reserved (moved from free to reserved). */
      Reserved: AugmentedEvent<
        ApiType,
        [who: AccountId32, amount: u128],
        { who: AccountId32; amount: u128 }
      >;
      /**
       * Some balance was moved from the reserve of the first account to the
       * second account. Final argument indicates the destination balance type.
       */
      ReserveRepatriated: AugmentedEvent<
        ApiType,
        [
          from: AccountId32,
          to: AccountId32,
          amount: u128,
          destinationStatus: FrameSupportTokensMiscBalanceStatus
        ],
        {
          from: AccountId32;
          to: AccountId32;
          amount: u128;
          destinationStatus: FrameSupportTokensMiscBalanceStatus;
        }
      >;
      /** Some amount was removed from the account (e.g. for misbehavior). */
      Slashed: AugmentedEvent<
        ApiType,
        [who: AccountId32, amount: u128],
        { who: AccountId32; amount: u128 }
      >;
      /** Transfer succeeded. */
      Transfer: AugmentedEvent<
        ApiType,
        [from: AccountId32, to: AccountId32, amount: u128],
        { from: AccountId32; to: AccountId32; amount: u128 }
      >;
      /** Some balance was unreserved (moved from reserved to free). */
      Unreserved: AugmentedEvent<
        ApiType,
        [who: AccountId32, amount: u128],
        { who: AccountId32; amount: u128 }
      >;
      /** Some amount was withdrawn from the account (e.g. for transaction fees). */
      Withdraw: AugmentedEvent<
        ApiType,
        [who: AccountId32, amount: u128],
        { who: AccountId32; amount: u128 }
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    circuit: {
      CancelledSideEffects: AugmentedEvent<
        ApiType,
        [AccountId32, H256, Vec<T3rnTypesSideEffect>]
      >;
      EscrowTransfer: AugmentedEvent<ApiType, [AccountId32, AccountId32, u128]>;
      NewSideEffectsAvailable: AugmentedEvent<
        ApiType,
        [AccountId32, H256, Vec<T3rnTypesSideEffect>, Vec<H256>]
      >;
      SideEffectsConfirmed: AugmentedEvent<
        ApiType,
        [H256, Vec<Vec<T3rnPrimitivesSideEffectFullSideEffect>>]
      >;
      XTransactionReadyForExec: AugmentedEvent<ApiType, [H256]>;
      XTransactionReceivedForExec: AugmentedEvent<ApiType, [H256]>;
      XTransactionStepFinishedExec: AugmentedEvent<ApiType, [H256]>;
      XTransactionXtxFinishedExecAllSteps: AugmentedEvent<ApiType, [H256]>;
      XTransactionXtxRevertedAfterTimeOut: AugmentedEvent<ApiType, [H256]>;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    circuitPortal: {
      GatewayUpdated: AugmentedEvent<
        ApiType,
        [U8aFixed, Option<Vec<U8aFixed>>]
      >;
      NewGatewayRegistered: AugmentedEvent<
        ApiType,
        [
          U8aFixed,
          T3rnPrimitivesGatewayType,
          T3rnPrimitivesGatewayVendor,
          T3rnPrimitivesGatewaySysProps,
          Vec<U8aFixed>
        ]
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    contracts: {
      /** A code with the specified hash was removed. */
      CodeRemoved: AugmentedEvent<
        ApiType,
        [codeHash: H256],
        { codeHash: H256 }
      >;
      /** Code with the specified hash has been stored. */
      CodeStored: AugmentedEvent<ApiType, [codeHash: H256], { codeHash: H256 }>;
      /** A contract's code was updated. */
      ContractCodeUpdated: AugmentedEvent<
        ApiType,
        [contract: AccountId32, newCodeHash: H256, oldCodeHash: H256],
        { contract: AccountId32; newCodeHash: H256; oldCodeHash: H256 }
      >;
      /** A custom event emitted by the contract. */
      ContractEmitted: AugmentedEvent<
        ApiType,
        [contract: AccountId32, data: Bytes],
        { contract: AccountId32; data: Bytes }
      >;
      /** Contract deployed by address at the specified address. */
      Instantiated: AugmentedEvent<
        ApiType,
        [deployer: AccountId32, contract: AccountId32],
        { deployer: AccountId32; contract: AccountId32 }
      >;
      /**
       * Contract has been removed.
       *
       * # Note
       *
       * The only way for a contract to be removed and emitting this event is by
       * calling `seal_terminate`.
       */
      Terminated: AugmentedEvent<
        ApiType,
        [contract: AccountId32, beneficiary: AccountId32],
        { contract: AccountId32; beneficiary: AccountId32 }
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    contractsRegistry: {
      /** [requester, contract_id] */
      ContractPurged: AugmentedEvent<ApiType, [AccountId32, H256]>;
      /** [requester, contract_id] */
      ContractStored: AugmentedEvent<ApiType, [AccountId32, H256]>;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    grandpa: {
      /** New authority set has been applied. */
      NewAuthorities: AugmentedEvent<
        ApiType,
        [authoritySet: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>],
        { authoritySet: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>> }
      >;
      /** Current authority set has been paused. */
      Paused: AugmentedEvent<ApiType, []>;
      /** Current authority set has been resumed. */
      Resumed: AugmentedEvent<ApiType, []>;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    multiFinalityVerifierDefault: {
      NewHeaderRangeAvailable: AugmentedEvent<ApiType, [U8aFixed, u32, u32]>;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    multiFinalityVerifierEthereumLike: {
      NewHeaderRangeAvailable: AugmentedEvent<ApiType, [U8aFixed, u64, u32]>;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    multiFinalityVerifierGenericLike: {
      NewHeaderRangeAvailable: AugmentedEvent<ApiType, [U8aFixed, u32, u32]>;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    multiFinalityVerifierPolkadotLike: {
      NewHeaderRangeAvailable: AugmentedEvent<ApiType, [U8aFixed, u32, u32]>;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    multiFinalityVerifierSubstrateLike: {
      NewHeaderRangeAvailable: AugmentedEvent<ApiType, [U8aFixed, u32, u32]>;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    ormlTokens: {
      /** A balance was set by root. */
      BalanceSet: AugmentedEvent<
        ApiType,
        [currencyId: u32, who: AccountId32, free: u128, reserved: u128],
        { currencyId: u32; who: AccountId32; free: u128; reserved: u128 }
      >;
      /**
       * An account was removed whose balance was non-zero but below
       * ExistentialDeposit, resulting in an outright loss.
       */
      DustLost: AugmentedEvent<
        ApiType,
        [currencyId: u32, who: AccountId32, amount: u128],
        { currencyId: u32; who: AccountId32; amount: u128 }
      >;
      /** An account was created with some free balance. */
      Endowed: AugmentedEvent<
        ApiType,
        [currencyId: u32, who: AccountId32, amount: u128],
        { currencyId: u32; who: AccountId32; amount: u128 }
      >;
      /** Some reserved balance was repatriated (moved from reserved to another account). */
      RepatriatedReserve: AugmentedEvent<
        ApiType,
        [
          currencyId: u32,
          from: AccountId32,
          to: AccountId32,
          amount: u128,
          status: FrameSupportTokensMiscBalanceStatus
        ],
        {
          currencyId: u32;
          from: AccountId32;
          to: AccountId32;
          amount: u128;
          status: FrameSupportTokensMiscBalanceStatus;
        }
      >;
      /** Some balance was reserved (moved from free to reserved). */
      Reserved: AugmentedEvent<
        ApiType,
        [currencyId: u32, who: AccountId32, amount: u128],
        { currencyId: u32; who: AccountId32; amount: u128 }
      >;
      /** Transfer succeeded. */
      Transfer: AugmentedEvent<
        ApiType,
        [currencyId: u32, from: AccountId32, to: AccountId32, amount: u128],
        { currencyId: u32; from: AccountId32; to: AccountId32; amount: u128 }
      >;
      /** Some balance was unreserved (moved from reserved to free). */
      Unreserved: AugmentedEvent<
        ApiType,
        [currencyId: u32, who: AccountId32, amount: u128],
        { currencyId: u32; who: AccountId32; amount: u128 }
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    portal: {
      /**
       * Event documentation should end with an array that provides descriptive
       * names for event Gateway was registered successsfully. [ChainId]
       */
      GatewayRegistered: AugmentedEvent<ApiType, [U8aFixed]>;
      /** Header was successfully added */
      HeaderSubmitted: AugmentedEvent<ApiType, [U8aFixed, Bytes]>;
      /** Gateway was set operational. [ChainId, bool] */
      SetOperational: AugmentedEvent<ApiType, [U8aFixed, bool]>;
      /** Gateway owner was set successfully. [ChainId, Vec<u8>] */
      SetOwner: AugmentedEvent<ApiType, [U8aFixed, Bytes]>;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    sudo: {
      /** The [sudoer] just switched identity; the old key is supplied if one existed. */
      KeyChanged: AugmentedEvent<
        ApiType,
        [oldSudoer: Option<AccountId32>],
        { oldSudoer: Option<AccountId32> }
      >;
      /** A sudo just took place. [result] */
      Sudid: AugmentedEvent<
        ApiType,
        [sudoResult: Result<Null, SpRuntimeDispatchError>],
        { sudoResult: Result<Null, SpRuntimeDispatchError> }
      >;
      /** A sudo just took place. [result] */
      SudoAsDone: AugmentedEvent<
        ApiType,
        [sudoResult: Result<Null, SpRuntimeDispatchError>],
        { sudoResult: Result<Null, SpRuntimeDispatchError> }
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    system: {
      /** `:code` was updated. */
      CodeUpdated: AugmentedEvent<ApiType, []>;
      /** An extrinsic failed. */
      ExtrinsicFailed: AugmentedEvent<
        ApiType,
        [
          dispatchError: SpRuntimeDispatchError,
          dispatchInfo: FrameSupportWeightsDispatchInfo
        ],
        {
          dispatchError: SpRuntimeDispatchError;
          dispatchInfo: FrameSupportWeightsDispatchInfo;
        }
      >;
      /** An extrinsic completed successfully. */
      ExtrinsicSuccess: AugmentedEvent<
        ApiType,
        [dispatchInfo: FrameSupportWeightsDispatchInfo],
        { dispatchInfo: FrameSupportWeightsDispatchInfo }
      >;
      /** An account was reaped. */
      KilledAccount: AugmentedEvent<
        ApiType,
        [account: AccountId32],
        { account: AccountId32 }
      >;
      /** A new account was created. */
      NewAccount: AugmentedEvent<
        ApiType,
        [account: AccountId32],
        { account: AccountId32 }
      >;
      /** On on-chain remark happened. */
      Remarked: AugmentedEvent<
        ApiType,
        [sender: AccountId32, hash_: H256],
        { sender: AccountId32; hash_: H256 }
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    utility: {
      /** Batch of dispatches completed fully with no error. */
      BatchCompleted: AugmentedEvent<ApiType, []>;
      /**
       * Batch of dispatches did not complete fully. Index of first failing
       * dispatch given, as well as the error.
       */
      BatchInterrupted: AugmentedEvent<
        ApiType,
        [index: u32, error: SpRuntimeDispatchError],
        { index: u32; error: SpRuntimeDispatchError }
      >;
      /** A call was dispatched. */
      DispatchedAs: AugmentedEvent<
        ApiType,
        [result: Result<Null, SpRuntimeDispatchError>],
        { result: Result<Null, SpRuntimeDispatchError> }
      >;
      /** A single item within a Batch of dispatches has completed with no error. */
      ItemCompleted: AugmentedEvent<ApiType, []>;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    xdns: {
      /** [requester, xdns_record_id] */
      XdnsRecordPurged: AugmentedEvent<ApiType, [AccountId32, U8aFixed]>;
      /** [xdns_record_id] */
      XdnsRecordStored: AugmentedEvent<ApiType, [U8aFixed]>;
      /** [xdns_record_id] */
      XdnsRecordUpdated: AugmentedEvent<ApiType, [U8aFixed]>;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
  } // AugmentedEvents
} // declare module
