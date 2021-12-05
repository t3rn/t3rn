// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes } from '@polkadot/api/types';
import type { Bytes, Option, U256, Vec, u32, u64 } from '@polkadot/types';
import type { BalanceStatus } from '@polkadot/types/interfaces/balances';
import type { ChainId, LaneId, MessageNonce, Parameter } from '@polkadot/types/interfaces/bridges';
import type { MessageId } from '@polkadot/types/interfaces/cumulus';
import type { EvmLog } from '@polkadot/types/interfaces/evm';
import type { AuthorityList } from '@polkadot/types/interfaces/grandpa';
import type { AccountId, Balance, BalanceOf, H160, Hash, PhantomData, Weight } from '@polkadot/types/interfaces/runtime';
import type { SessionIndex } from '@polkadot/types/interfaces/session';
import type { SpecVersion } from '@polkadot/types/interfaces/state';
import type { DispatchError, DispatchInfo, DispatchResult } from '@polkadot/types/interfaces/system';
import type { RegistryContractId } from 't3rn-circuit-typegen/interfaces/contracts_registry';
import type { AllowedSideEffect, SideEffectsDFD, XtxId } from 't3rn-circuit-typegen/interfaces/execution_delivery';
import type { ConfirmedSideEffect, GatewaySysProps, GatewayType, GatewayVendor, SideEffect } from 't3rn-circuit-typegen/interfaces/primitives';
import type { XdnsRecordId } from 't3rn-circuit-typegen/interfaces/xdns';

declare module '@polkadot/api/types/events' {
  export interface AugmentedEvents<ApiType> {
    balances: {
      /**
       * A balance was set by root. \[who, free, reserved\]
       **/
      BalanceSet: AugmentedEvent<ApiType, [AccountId, Balance, Balance]>;
      /**
       * Some amount was deposited (e.g. for transaction fees). \[who, deposit\]
       **/
      Deposit: AugmentedEvent<ApiType, [AccountId, Balance]>;
      /**
       * An account was removed whose balance was non-zero but below ExistentialDeposit,
       * resulting in an outright loss. \[account, balance\]
       **/
      DustLost: AugmentedEvent<ApiType, [AccountId, Balance]>;
      /**
       * An account was created with some free balance. \[account, free_balance\]
       **/
      Endowed: AugmentedEvent<ApiType, [AccountId, Balance]>;
      /**
       * Some balance was reserved (moved from free to reserved). \[who, value\]
       **/
      Reserved: AugmentedEvent<ApiType, [AccountId, Balance]>;
      /**
       * Some balance was moved from the reserve of the first account to the second account.
       * Final argument indicates the destination balance type.
       * \[from, to, balance, destination_status\]
       **/
      ReserveRepatriated: AugmentedEvent<ApiType, [AccountId, AccountId, Balance, BalanceStatus]>;
      /**
       * Transfer succeeded. \[from, to, value\]
       **/
      Transfer: AugmentedEvent<ApiType, [AccountId, AccountId, Balance]>;
      /**
       * Some balance was unreserved (moved from reserved to free). \[who, value\]
       **/
      Unreserved: AugmentedEvent<ApiType, [AccountId, Balance]>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    basicOutboundChannel: {
      MessageAccepted: AugmentedEvent<ApiType, [MessageNonce]>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    bridgeDispatch: {
      /**
       * Phantom member, never used. Needed to handle multiple pallet instances.
       **/
      _Dummy: AugmentedEvent<ApiType, [PhantomData]>;
      /**
       * We have failed to decode Call from the message.
       **/
      MessageCallDecodeFailed: AugmentedEvent<ApiType, [ChainId, MessageId]>;
      /**
       * The call from the message has been rejected by the call filter.
       **/
      MessageCallRejected: AugmentedEvent<ApiType, [ChainId, MessageId]>;
      /**
       * Message has been dispatched with given result.
       **/
      MessageDispatched: AugmentedEvent<ApiType, [ChainId, MessageId, DispatchResult]>;
      /**
       * Message has been rejected before reaching dispatch.
       **/
      MessageRejected: AugmentedEvent<ApiType, [ChainId, MessageId]>;
      /**
       * Message signature mismatch.
       **/
      MessageSignatureMismatch: AugmentedEvent<ApiType, [ChainId, MessageId]>;
      /**
       * Message has been rejected by dispatcher because of spec version mismatch.
       * Last two arguments are: expected and passed spec version.
       **/
      MessageVersionSpecMismatch: AugmentedEvent<ApiType, [ChainId, MessageId, SpecVersion, SpecVersion]>;
      /**
       * Message has been rejected by dispatcher because of weight mismatch.
       * Last two arguments are: expected and passed call weight.
       **/
      MessageWeightMismatch: AugmentedEvent<ApiType, [ChainId, MessageId, Weight, Weight]>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    bridgeGatewayMessages: {
      /**
       * Phantom member, never used.
       **/
      Dummy: AugmentedEvent<ApiType, [PhantomData]>;
      /**
       * Message has been accepted and is waiting to be delivered.
       **/
      MessageAccepted: AugmentedEvent<ApiType, [LaneId, MessageNonce]>;
      /**
       * Messages in the inclusive range have been delivered and processed by the bridged chain.
       **/
      MessagesDelivered: AugmentedEvent<ApiType, [LaneId, MessageNonce, MessageNonce]>;
      /**
       * Pallet parameter has been updated.
       **/
      ParameterUpdated: AugmentedEvent<ApiType, [Parameter]>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    contracts: {
      /**
       * A code with the specified hash was removed.
       * \[code_hash\]
       * 
       * This happens when the last contract that uses this code hash was removed or evicted.
       **/
      CodeRemoved: AugmentedEvent<ApiType, [Hash]>;
      /**
       * Code with the specified hash has been stored. \[code_hash\]
       **/
      CodeStored: AugmentedEvent<ApiType, [Hash]>;
      /**
       * A custom event emitted by the contract.
       * \[contract, data\]
       * 
       * # Params
       * 
       * - `contract`: The contract that emitted the event.
       * - `data`: Data supplied by the contract. Metadata generated during contract
       * compilation is needed to decode it.
       **/
      ContractEmitted: AugmentedEvent<ApiType, [AccountId, Bytes]>;
      /**
       * Contract has been evicted and is now in tombstone state. \[contract\]
       **/
      Evicted: AugmentedEvent<ApiType, [AccountId]>;
      /**
       * Contract deployed by address at the specified address. \[deployer, contract\]
       **/
      Instantiated: AugmentedEvent<ApiType, [AccountId, AccountId]>;
      /**
       * Restoration of a contract has been successful.
       * \[restorer, dest, code_hash, rent_allowance\]
       * 
       * # Params
       * 
       * - `restorer`: Account ID of the restoring contract.
       * - `dest`: Account ID of the restored contract.
       * - `code_hash`: Code hash of the restored contract.
       * - `rent_allowance`: Rent allowance of the restored contract.
       **/
      Restored: AugmentedEvent<ApiType, [AccountId, AccountId, Hash, Balance]>;
      /**
       * Triggered when the current schedule is updated.
       * \[version\]
       * 
       * # Params
       * 
       * - `version`: The version of the newly set schedule.
       **/
      ScheduleUpdated: AugmentedEvent<ApiType, [u32]>;
      /**
       * Contract has been terminated without leaving a tombstone.
       * \[contract, beneficiary\]
       * 
       * # Params
       * 
       * - `contract`: The contract that was terminated.
       * - `beneficiary`: The account that received the contracts remaining balance.
       * 
       * # Note
       * 
       * The only way for a contract to be removed without a tombstone and emitting
       * this event is by calling `seal_terminate`.
       **/
      Terminated: AugmentedEvent<ApiType, [AccountId, AccountId]>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    contractsRegistry: {
      /**
       * \[requester, contract_id\]
       **/
      ContractPurged: AugmentedEvent<ApiType, [AccountId, RegistryContractId]>;
      /**
       * \[requester, contract_id\]
       **/
      ContractStored: AugmentedEvent<ApiType, [AccountId, RegistryContractId]>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    ethereumLightClient: {
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    evm: {
      /**
       * A deposit has been made at a given address. \[sender, address, value\]
       **/
      BalanceDeposit: AugmentedEvent<ApiType, [AccountId, H160, U256]>;
      /**
       * A withdrawal has been made from a given address. \[sender, address, value\]
       **/
      BalanceWithdraw: AugmentedEvent<ApiType, [AccountId, H160, U256]>;
      /**
       * A contract has been created at given \[address\].
       **/
      Created: AugmentedEvent<ApiType, [H160]>;
      /**
       * A \[contract\] was attempted to be created, but the execution failed.
       **/
      CreatedFailed: AugmentedEvent<ApiType, [H160]>;
      /**
       * A \[contract\] has been executed successfully with states applied.
       **/
      Executed: AugmentedEvent<ApiType, [H160]>;
      /**
       * A \[contract\] has been executed with errors. States are reverted with only gas fees applied.
       **/
      ExecutedFailed: AugmentedEvent<ApiType, [H160]>;
      /**
       * Ethereum events from contracts.
       **/
      Log: AugmentedEvent<ApiType, [EvmLog]>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    execDelivery: {
      CancelledSideEffects: AugmentedEvent<ApiType, [AccountId, XtxId, Vec<SideEffect>]>;
      GatewayUpdated: AugmentedEvent<ApiType, [ChainId, Option<Vec<Bytes>>]>;
      NewGatewayRegistered: AugmentedEvent<ApiType, [ChainId, GatewayType, GatewayVendor, GatewaySysProps, Vec<AllowedSideEffect>]>;
      NewSideEffectsAvailable: AugmentedEvent<ApiType, [AccountId, XtxId, Vec<SideEffect>]>;
      SideEffectConfirmed: AugmentedEvent<ApiType, [AccountId, XtxId, ConfirmedSideEffect, u64]>;
      XTransactionReceivedForExec: AugmentedEvent<ApiType, [XtxId, SideEffectsDFD]>;
      XTransactionSuccessfullyCompleted: AugmentedEvent<ApiType, [XtxId]>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    grandpa: {
      /**
       * New authority set has been applied. \[authority_set\]
       **/
      NewAuthorities: AugmentedEvent<ApiType, [AuthorityList]>;
      /**
       * Current authority set has been paused.
       **/
      Paused: AugmentedEvent<ApiType, []>;
      /**
       * Current authority set has been resumed.
       **/
      Resumed: AugmentedEvent<ApiType, []>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    session: {
      /**
       * New session has happened. Note that the argument is the \[session_index\], not the block
       * number as the type might suggest.
       **/
      NewSession: AugmentedEvent<ApiType, [SessionIndex]>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    sudo: {
      /**
       * The \[sudoer\] just switched identity; the old key is supplied.
       **/
      KeyChanged: AugmentedEvent<ApiType, [AccountId]>;
      /**
       * A sudo just took place. \[result\]
       **/
      Sudid: AugmentedEvent<ApiType, [DispatchResult]>;
      /**
       * A sudo just took place. \[result\]
       **/
      SudoAsDone: AugmentedEvent<ApiType, [DispatchResult]>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    system: {
      /**
       * `:code` was updated.
       **/
      CodeUpdated: AugmentedEvent<ApiType, []>;
      /**
       * An extrinsic failed. \[error, info\]
       **/
      ExtrinsicFailed: AugmentedEvent<ApiType, [DispatchError, DispatchInfo]>;
      /**
       * An extrinsic completed successfully. \[info\]
       **/
      ExtrinsicSuccess: AugmentedEvent<ApiType, [DispatchInfo]>;
      /**
       * An \[account\] was reaped.
       **/
      KilledAccount: AugmentedEvent<ApiType, [AccountId]>;
      /**
       * A new \[account\] was created.
       **/
      NewAccount: AugmentedEvent<ApiType, [AccountId]>;
      /**
       * On on-chain remark happened. \[origin, remark_hash\]
       **/
      Remarked: AugmentedEvent<ApiType, [AccountId, Hash]>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    utility: {
      /**
       * Batch of dispatches completed fully with no error.
       **/
      BatchCompleted: AugmentedEvent<ApiType, []>;
      /**
       * Batch of dispatches did not complete fully. Index of first failing dispatch given, as
       * well as the error. \[index, error\]
       **/
      BatchInterrupted: AugmentedEvent<ApiType, [u32, DispatchError]>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    volatileVm: {
      /**
       * A code with the specified hash was removed.
       * \[code_hash\]
       * 
       * This happens when the last contract that uses this code hash was removed or evicted.
       **/
      CodeRemoved: AugmentedEvent<ApiType, [Hash]>;
      /**
       * Code with the specified hash has been stored. \[code_hash\]
       **/
      CodeStored: AugmentedEvent<ApiType, [Hash]>;
      /**
       * A custom event emitted by the contract.
       * \[contract, data\]
       * 
       * # Params
       * 
       * - `contract`: The contract that emitted the event.
       * - `data`: Data supplied by the contract. Metadata generated during contract
       * compilation is needed to decode it.
       **/
      ContractEmitted: AugmentedEvent<ApiType, [AccountId, Bytes]>;
      /**
       * Contract has been evicted and is now in tombstone state. \[contract\]
       **/
      Evicted: AugmentedEvent<ApiType, [AccountId]>;
      /**
       * Contract deployed by address at the specified address. \[deployer, contract\]
       **/
      Instantiated: AugmentedEvent<ApiType, [AccountId, AccountId]>;
      /**
       * Restoration of a contract has been successful.
       * \[restorer, dest, code_hash, rent_allowance\]
       * 
       * # Params
       * 
       * - `restorer`: Account ID of the restoring contract.
       * - `dest`: Account ID of the restored contract.
       * - `code_hash`: Code hash of the restored contract.
       * - `rent_allowance`: Rent allowance of the restored contract.
       **/
      Restored: AugmentedEvent<ApiType, [AccountId, AccountId, Hash, BalanceOf]>;
      /**
       * Triggered when the current schedule is updated.
       * \[version\]
       * 
       * # Params
       * 
       * - `version`: The version of the newly set schedule.
       **/
      ScheduleUpdated: AugmentedEvent<ApiType, [u32]>;
      /**
       * Contract deployed by address at the specified address. \[deployer, contract\]
       **/
      TempInstantiated: AugmentedEvent<ApiType, [AccountId, AccountId]>;
      /**
       * Contract has been terminated without leaving a tombstone.
       * \[contract, beneficiary\]
       * 
       * # Params
       * 
       * - `contract`: The contract that was terminated.
       * - `beneficiary`: The account that received the contracts remaining balance.
       * 
       * # Note
       * 
       * The only way for a contract to be removed without a tombstone and emitting
       * this event is by calling `seal_terminate`.
       **/
      Terminated: AugmentedEvent<ApiType, [AccountId, AccountId]>;
      /**
       * An event deposited upon execution of a contract from the account.
       * \[escrow_account, requester_account, data\]
       **/
      VolatileVMEmitted: AugmentedEvent<ApiType, [AccountId, AccountId, Bytes]>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    xdns: {
      /**
       * \[requester, xdns_record_id\]
       **/
      XdnsRecordPurged: AugmentedEvent<ApiType, [AccountId, XdnsRecordId]>;
      /**
       * \[requester, xdns_record_id\]
       **/
      XdnsRecordStored: AugmentedEvent<ApiType, [AccountId, XdnsRecordId]>;
      /**
       * \[xdns_record_id\]
       **/
      XdnsRecordUpdated: AugmentedEvent<ApiType, [XdnsRecordId]>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
  }

  export interface DecoratedEvents<ApiType extends ApiTypes> extends AugmentedEvents<ApiType> {
    [key: string]: ModuleEvents<ApiType>;
  }
}
