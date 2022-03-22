// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Enum, Struct, Type, Vec, u16 } from "@polkadot/types-codec";
import type { Parameter } from "@polkadot/types/interfaces/bridges";

/** @name CryptoAlgo */
export interface CryptoAlgo extends Enum {
  readonly isEd25519: boolean;
  readonly isSr25519: boolean;
  readonly isEcdsa: boolean;
  readonly type: "Ed25519" | "Sr25519" | "Ecdsa";
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
  readonly type: "Blake2" | "Keccak256";
}

/** @name StructDecl */
export interface StructDecl extends Struct {
  readonly name: Type;
  readonly fields: Vec<Parameter>;
  readonly offsets: Vec<u16>;
}

export type PHANTOM_CIRCUIT_PORTAL = "circuit_portal";
