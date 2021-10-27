// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Bytes, Enum, Struct, Type, Vec, u16 } from '@polkadot/types';
import type { Parameter } from '@polkadot/types/interfaces/bridges';
import type { AccountId, Balance } from '@polkadot/types/interfaces/runtime';

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

/** @name CryptoAlgo */
export interface CryptoAlgo extends Enum {
  readonly isEd25519: boolean;
  readonly isSr25519: boolean;
  readonly isEcdsa: boolean;
}

/** @name GatewayABIConfig */
export interface GatewayABIConfig extends Struct {
  readonly block_number_type_size: u16;
  readonly hash_size: u16;
  readonly hasher: HasherAlgo;
  readonly crypto: CryptoAlgo;
  readonly address_length: u16;
  readonly value_type_size: u16;
  readonly decimals: u16;
  readonly structs: Vec<StructDecl>;
}

/** @name HasherAlgo */
export interface HasherAlgo extends Enum {
  readonly isBlake2: boolean;
  readonly isKeccak256: boolean;
}

/** @name StructDecl */
export interface StructDecl extends Struct {
  readonly name: Type;
  readonly fields: Vec<Parameter>;
  readonly offsets: Vec<u16>;
}

export type PHANTOM_PRIMITIVES = 'primitives';
