// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Bytes, Enum, Option, Struct, U8aFixed, Vec, u16, u32, u64, u8 } from '@polkadot/types-codec';
import type { ChainId } from '@polkadot/types/interfaces/bridges';
import type { AccountId, Balance, BalanceOf, Hash } from '@polkadot/types/interfaces/runtime';

/** @name AllowedSideEffect */
export interface AllowedSideEffect extends U8aFixed {}

/** @name BlockNumber */
export interface BlockNumber extends u64 {}

/** @name CircuitOutboundMessage */
export interface CircuitOutboundMessage extends Struct {
  readonly name: Bytes;
  readonly module_name: Bytes;
  readonly method_name: Bytes;
  readonly sender: Option<Bytes>;
  readonly target: Option<Bytes>;
  readonly arguments: Vec<Bytes>;
  readonly expected_output: Vec<GatewayExpectedOutput>;
  readonly extra_payload: Option<ExtraMessagePayload>;
}

/** @name Compose */
export interface Compose extends Struct {
  readonly name: Bytes;
  readonly code_txt: Bytes;
  readonly exec_type: Bytes;
  readonly dest: AccountId;
  readonly value: Balance;
  readonly bytes: Bytes;
  readonly input_data: Bytes;
}

/** @name ConfirmedSideEffect */
export interface ConfirmedSideEffect extends Struct {
  readonly err: Option<Bytes>;
  readonly output: Option<Bytes>;
  readonly encoded_effect: Bytes;
  readonly inclusion_proof: Option<Bytes>;
  readonly executioner: AccountId;
  readonly received_at: BlockNumber;
  readonly cost: Option<BalanceOf>;
}

/** @name ContractActionDesc */
export interface ContractActionDesc extends Struct {
  readonly action_id: Hash;
  readonly target_id: Option<TargetId>;
  readonly to: Option<AccountId>;
}

/** @name ExtraMessagePayload */
export interface ExtraMessagePayload extends Struct {
  readonly signer: Bytes;
  readonly module_name: Bytes;
  readonly method_name: Bytes;
  readonly call_bytes: Bytes;
  readonly signature: Bytes;
  readonly extra: Bytes;
  readonly tx_signed: Bytes;
  readonly custom_payload: Option<Bytes>;
}

/** @name FullSideEffect */
export interface FullSideEffect extends Struct {
  readonly input: SideEffect;
  readonly confirmed: Option<ConfirmedSideEffect>;
}

/** @name GatewayExpectedOutput */
export interface GatewayExpectedOutput extends Enum {
  readonly isStorage: boolean;
  readonly asStorage: {
    readonly key: Vec<Bytes>;
    readonly value: Vec<Option<Bytes>>;
  } & Struct;
  readonly isEvents: boolean;
  readonly asEvents: {
    readonly signatures: Vec<Bytes>;
  } & Struct;
  readonly isExtrinsic: boolean;
  readonly asExtrinsic: {
    readonly block_height: Option<u64>;
  } & Struct;
  readonly isOutput: boolean;
  readonly asOutput: {
    readonly output: Bytes;
  } & Struct;
  readonly type: 'Storage' | 'Events' | 'Extrinsic' | 'Output';
}

/** @name GatewayGenesisConfig */
export interface GatewayGenesisConfig extends Struct {
  readonly modules_encoded: Option<Bytes>;
  readonly extrinsics_version: u8;
  readonly genesis_hash: Bytes;
}

/** @name GatewayPointer */
export interface GatewayPointer extends Struct {
  readonly id: ChainId;
  readonly vendor: GatewayVendor;
  readonly gateway_type: GatewayType;
}

/** @name GatewaySysProps */
export interface GatewaySysProps extends Struct {
  readonly ss58_format: u16;
  readonly token_symbol: Bytes;
  readonly token_decimals: u8;
}

/** @name GatewayType */
export interface GatewayType extends Enum {
  readonly isProgrammableInternal: boolean;
  readonly asProgrammableInternal: u32;
  readonly isProgrammableExternal: boolean;
  readonly asProgrammableExternal: u32;
  readonly isTxOnly: boolean;
  readonly asTxOnly: u32;
  readonly type: 'ProgrammableInternal' | 'ProgrammableExternal' | 'TxOnly';
}

/** @name GatewayVendor */
export interface GatewayVendor extends Enum {
  readonly isSubstrate: boolean;
  readonly isEthereum: boolean;
  readonly type: 'Substrate' | 'Ethereum';
}

/** @name ProofTriePointer */
export interface ProofTriePointer extends Enum {
  readonly isState: boolean;
  readonly isTransaction: boolean;
  readonly isReceipts: boolean;
  readonly type: 'State' | 'Transaction' | 'Receipts';
}

/** @name SideEffect */
export interface SideEffect extends Struct {
  readonly target: TargetId;
  readonly prize: BalanceOf;
  readonly ordered_at: BlockNumber;
  readonly encoded_action: Bytes;
  readonly encoded_args: Vec<Bytes>;
  readonly signature: Bytes;
  readonly enforce_executioner: Option<AccountId>;
}

/** @name SideEffectId */
export interface SideEffectId extends Hash {}

/** @name TargetId */
export interface TargetId extends U8aFixed {}

/** @name XtxId */
export interface XtxId extends Hash {}

export type PHANTOM_PRIMITIVES = 'primitives';
