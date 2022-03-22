// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Option, Struct, u32 } from "@polkadot/types-codec";
import type { ITuple } from "@polkadot/types-codec/types";
import type { CodeHash, TrieId } from "@polkadot/types/interfaces/contracts";
import type { Balance, BlockNumber } from "@polkadot/types/interfaces/runtime";

/** @name RawAliveContractInfo */
export interface RawAliveContractInfo extends Struct {
  readonly trie_id: TrieId;
  readonly storage_size: u32;
  readonly pair_count: u32;
  readonly code_hash: CodeHash;
  readonly rent_allowance: Balance;
  readonly rent_paid: Balance;
  readonly deduct_block: BlockNumber;
  readonly last_write: Option<BlockNumber>;
  readonly _reserved: Option<ITuple<[]>>;
}

export type PHANTOM_VOLATILE_VM = "volatile_vm";
