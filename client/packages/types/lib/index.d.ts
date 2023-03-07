import './augment/types-lookup';
import './augment/registry';
import './augment/augment-api';
import { AnyU8a, Codec, DetectCodec, ITuple, Observable } from '@polkadot/types/types';
import { Text, UInt, Null, bool, Option, Vec, BTreeSet, BTreeMap, Tuple, Enum, Struct, Bytes, TypeRegistry, Raw } from '@polkadot/types';
import BN from 'bn.js';
import { AugmentedQuery } from '@polkadot/api/types';
export type EnumVariant<T> = keyof T extends infer K ? K extends keyof T ? T[K] extends Null | null ? K | {
    [I in K]: T[I];
} : {
    [I in K]: T[I];
} : never : never;
type EnumAccessors<T extends string> = {
    [K in `as${T}`]?: unknown;
};
type DecoratedEnum<T extends string> = Omit<Enum, 'type'> & {
    type: T;
} & EnumAccessors<T>;
type CodecOrNull<T> = T extends Codec ? T : Null;
type EnumDefs<E extends DecoratedEnum<T>, T extends string> = {
    [K in T]: CodecOrNull<E[`as${K}`]>;
};
type StructDefs<S extends Struct> = Omit<S, keyof Struct>;
type KeyOf<T> = T extends DecoratedEnum<infer S> ? keyof EnumDefs<T, S> : T extends Struct ? keyof StructDefs<T> : unknown[];
type AsRecord<K, V> = K extends string ? Record<K & string, V> : K extends number ? Record<K & number, V> : never;
/**
 * Recursively create typesafe interface representing valid input for constructing any Codec type
 * (inlcuding complex types with a lot of nesting)
 *
 * Some examples:
 *
 * CreateInterface<Option<u128>> = Option<u128> | u128 | number | BN | null | undefined
 *
 * CreateInterface<PalletCommonBalanceKind> =
 *   PalletCommonBalanceKind |
 *   'Positive' |
 *   'Negative' |
 *   { Positive: null } |
 *   { Negative: null }
 *
 * CreateInterface<PalletContentPermissionsContentActor> =
 *   PalletContentPermissionsContentActor |
 *   'Lead' |
 *   { Lead: null } |
 *   { Curator: ITuple<[u64, u64]> | [u64 | BN | number, u64 | BN | number] }
 *   { Member: u64 | BN | number }
 *
 * CreateInterface<PalletContentLimitPerPeriod> =
 *   PalletContentLimitPerPeriod |
 *   { limit: u64 | BN | number, blockNumberPeriod: u32 | BN | number }
 */
export type CreateInterface<T> = T | (T extends Option<infer S> ? null | undefined | CreateInterface<S> : T extends DecoratedEnum<infer S> ? EnumVariant<{
    [K in keyof EnumDefs<T, S>]: CreateInterface<EnumDefs<T, S>[K]>;
}> : T extends Struct ? {
    [K in keyof StructDefs<T>]: CreateInterface<StructDefs<T>[K]>;
} : T extends Text ? string : T extends Bytes | Raw ? AnyU8a : T extends UInt ? number | BN : T extends bool ? boolean : T extends Vec<infer S> ? CreateInterface<S>[] : T extends BTreeSet<infer S> ? CreateInterface<S>[] | Set<CreateInterface<S>> : T extends ITuple<infer S> ? S extends Tuple ? unknown[] : {
    [K in keyof S]: CreateInterface<T[K]>;
} : T extends BTreeMap<infer K, infer V> ? Map<CreateInterface<K>, CreateInterface<V>> | AsRecord<CreateInterface<K>, CreateInterface<V>> : T extends Null ? null : unknown);
export declare const registry: TypeRegistry;
export declare function createType<TN extends string>(typeName: TN, value: CreateInterface<DetectCodec<Codec, TN>>): Codec extends DetectCodec<Codec, TN> ? unknown : DetectCodec<Codec, TN>;
export declare function keysOf<T extends Struct | Enum, TN extends string>(typeName: TN): KeyOf<T>[];
export declare function entriesByIds<IDType extends UInt, ValueType extends Codec>(apiMethod: AugmentedQuery<'promise', (key: IDType) => Observable<ValueType>, [IDType]>): Promise<[IDType, AsCodec<ValueType>][]>;
export type AsCodec<T> = T extends Codec ? T : Codec;
export {};
