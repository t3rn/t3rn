// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Bytes, Enum, Option, Struct, U256, U8aFixed, Vec, bool, u128, u32, u64 } from '@polkadot/types';
import type { AccountId, H128, H160, H256, H512, MultiAddress } from '@polkadot/types/interfaces/runtime';
import type { ITuple } from '@polkadot/types/types';

/** @name Address */
export interface Address extends MultiAddress {}

/** @name AssetId */
export interface AssetId extends Enum {
  readonly isEth: boolean;
  readonly isToken: boolean;
  readonly asToken: H160;
}

/** @name Bloom */
export interface Bloom extends U8aFixed {}

/** @name ChannelId */
export interface ChannelId extends Enum {
  readonly isBasic: boolean;
  readonly isIncentivized: boolean;
}

/** @name DispatchMessageId */
export interface DispatchMessageId extends Struct {
  readonly channelId: ChannelId;
  readonly nonce: u64;
}

/** @name EthashProofData */
export interface EthashProofData extends Struct {
  readonly dagNodes: Vec<H512>;
  readonly proof: Vec<H128>;
}

/** @name EthereumDifficultyConfig */
export interface EthereumDifficultyConfig extends Struct {
  readonly byzantiumForkBlock: u64;
  readonly constantinopleForkBlock: u64;
  readonly muirGlacierForkBlock: u64;
  readonly londonForkBlock: u64;
}

/** @name EthereumHeader */
export interface EthereumHeader extends Struct {
  readonly parentHash: H256;
  readonly timestamp: u64;
  readonly number: u64;
  readonly author: H160;
  readonly transactionsRoot: H256;
  readonly ommersHash: H256;
  readonly extraData: Bytes;
  readonly stateRoot: H256;
  readonly receiptsRoot: H256;
  readonly logBloom: Bloom;
  readonly gasUsed: U256;
  readonly gasLimit: U256;
  readonly difficulty: U256;
  readonly seal: Vec<Bytes>;
  readonly baseFee: Option<U256>;
}

/** @name EthereumHeaderId */
export interface EthereumHeaderId extends Struct {
  readonly number: u64;
  readonly hash: H256;
}

/** @name LookupSource */
export interface LookupSource extends MultiAddress {}

/** @name Message */
export interface Message extends Struct {
  readonly data: Bytes;
  readonly proof: Proof;
}

/** @name MessageNonce */
export interface MessageNonce extends u64 {}

/** @name Proof */
export interface Proof extends Struct {
  readonly blockHash: H256;
  readonly txIndex: u32;
  readonly data: ITuple<[Vec<Bytes>, Vec<Bytes>]>;
}

/** @name PruningRange */
export interface PruningRange extends Struct {
  readonly oldestUnprunedBlock: u64;
  readonly oldestBlockToKeep: u64;
}

/** @name StoredHeader */
export interface StoredHeader extends Struct {
  readonly submitter: Option<AccountId>;
  readonly header: EthereumHeader;
  readonly totalDifficulty: U256;
  readonly finalized: bool;
}

/** @name TokenData */
export interface TokenData extends Struct {
  readonly tokenContract: H160;
  readonly tokenId: U256;
}

/** @name TokenId */
export interface TokenId extends u128 {}

/** @name TokenInfoOf */
export interface TokenInfoOf extends Struct {
  readonly owner: AccountId;
  readonly metadata: Bytes;
  readonly data: TokenData;
}

export type PHANTOM_SNOWFORK = 'snowfork';
