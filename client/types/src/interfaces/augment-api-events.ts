// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes } from "@polkadot/api-base/types";
import type {
  Null,
  Option,
  Result,
  U8aFixed,
  Vec,
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
  T3rnPrimitivesSideEffect,
  T3rnPrimitivesSideEffectFullSideEffect,
} from "@polkadot/types/lookup";

declare module "@polkadot/api-base/types/events" {
  export interface AugmentedEvents<ApiType extends ApiTypes> {
    balances: {
      /** A balance was set by root. */
      BalanceSet: AugmentedEvent<ApiType, [AccountId32, u128, u128]>;
      /** Some amount was deposited (e.g. for transaction fees). */
      Deposit: AugmentedEvent<ApiType, [AccountId32, u128]>;
      /**
       * An account was removed whose balance was non-zero but below
       * ExistentialDeposit, resulting in an outright loss.
       */
      DustLost: AugmentedEvent<ApiType, [AccountId32, u128]>;
      /** An account was created with some free balance. */
      Endowed: AugmentedEvent<ApiType, [AccountId32, u128]>;
      /** Some balance was reserved (moved from free to reserved). */
      Reserved: AugmentedEvent<ApiType, [AccountId32, u128]>;
      /**
       * Some balance was moved from the reserve of the first account to the
       * second account. Final argument indicates the destination balance type.
       */
      ReserveRepatriated: AugmentedEvent<
        ApiType,
        [AccountId32, AccountId32, u128, FrameSupportTokensMiscBalanceStatus]
      >;
      /** Some amount was removed from the account (e.g. for misbehavior). */
      Slashed: AugmentedEvent<ApiType, [AccountId32, u128]>;
      /** Transfer succeeded. */
      Transfer: AugmentedEvent<ApiType, [AccountId32, AccountId32, u128]>;
      /** Some balance was unreserved (moved from reserved to free). */
      Unreserved: AugmentedEvent<ApiType, [AccountId32, u128]>;
      /** Some amount was withdrawn from the account (e.g. for transaction fees). */
      Withdraw: AugmentedEvent<ApiType, [AccountId32, u128]>;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    circuit: {
      CancelledSideEffects: AugmentedEvent<
        ApiType,
        [AccountId32, H256, Vec<T3rnPrimitivesSideEffect>]
      >;
      NewSideEffectsAvailable: AugmentedEvent<
        ApiType,
        [AccountId32, H256, Vec<T3rnPrimitivesSideEffect>]
      >;
      SideEffectsConfirmed: AugmentedEvent<
        ApiType,
        [H256, Vec<Vec<T3rnPrimitivesSideEffectFullSideEffect>>]
      >;
      XTransactionFinishedExec: AugmentedEvent<ApiType, [H256]>;
      XTransactionReadyForExec: AugmentedEvent<ApiType, [H256]>;
      XTransactionReceivedForExec: AugmentedEvent<ApiType, [H256]>;
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
        [Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>]
      >;
      /** Current authority set has been paused. */
      Paused: AugmentedEvent<ApiType, []>;
      /** Current authority set has been resumed. */
      Resumed: AugmentedEvent<ApiType, []>;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    ormlTokens: {
      /** A balance was set by root. */
      BalanceSet: AugmentedEvent<ApiType, [u32, AccountId32, u128, u128]>;
      /**
       * An account was removed whose balance was non-zero but below
       * ExistentialDeposit, resulting in an outright loss.
       */
      DustLost: AugmentedEvent<ApiType, [u32, AccountId32, u128]>;
      /** An account was created with some free balance. */
      Endowed: AugmentedEvent<ApiType, [u32, AccountId32, u128]>;
      /** Some reserved balance was repatriated (moved from reserved to another account). */
      RepatriatedReserve: AugmentedEvent<
        ApiType,
        [
          u32,
          AccountId32,
          AccountId32,
          u128,
          FrameSupportTokensMiscBalanceStatus
        ]
      >;
      /** Some balance was reserved (moved from free to reserved). */
      Reserved: AugmentedEvent<ApiType, [u32, AccountId32, u128]>;
      /** Transfer succeeded. */
      Transfer: AugmentedEvent<ApiType, [u32, AccountId32, AccountId32, u128]>;
      /** Some balance was unreserved (moved from reserved to free). */
      Unreserved: AugmentedEvent<ApiType, [u32, AccountId32, u128]>;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    sudo: {
      /** The [sudoer] just switched identity; the old key is supplied if one existed. */
      KeyChanged: AugmentedEvent<ApiType, [Option<AccountId32>]>;
      /** A sudo just took place. [result] */
      Sudid: AugmentedEvent<ApiType, [Result<Null, SpRuntimeDispatchError>]>;
      /** A sudo just took place. [result] */
      SudoAsDone: AugmentedEvent<
        ApiType,
        [Result<Null, SpRuntimeDispatchError>]
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
        [SpRuntimeDispatchError, FrameSupportWeightsDispatchInfo]
      >;
      /** An extrinsic completed successfully. */
      ExtrinsicSuccess: AugmentedEvent<
        ApiType,
        [FrameSupportWeightsDispatchInfo]
      >;
      /** An account was reaped. */
      KilledAccount: AugmentedEvent<ApiType, [AccountId32]>;
      /** A new account was created. */
      NewAccount: AugmentedEvent<ApiType, [AccountId32]>;
      /** On on-chain remark happened. */
      Remarked: AugmentedEvent<ApiType, [AccountId32, H256]>;
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
