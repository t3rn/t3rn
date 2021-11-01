// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Bytes, Option, Struct, Vec } from '@polkadot/types';
import type { ContractMetadata } from '@polkadot/types/interfaces/contractsAbi';
import type { AccountId, BalanceOf, Hash } from '@polkadot/types/interfaces/runtime';
import type { RawAliveContractInfo } from 't3rn-circuit-typegen/interfaces/volatile_vm';

/** @name RegistryContract */
export interface RegistryContract extends Struct {
  readonly code_txt: Bytes;
  readonly bytes: Bytes;
  readonly author: AccountId;
  readonly author_fees_per_single_use: Option<BalanceOf>;
  readonly abi: Option<Bytes>;
  readonly action_descriptions: Vec<ContractActionDesc>;
  readonly info: Option<RawAliveContractInfo>;
  readonly meta: ContractMetadata;
}

/** @name RegistryContractId */
export interface RegistryContractId extends Hash {}

export type PHANTOM_CONTRACTS_REGISTRY = 'contracts_registry';
