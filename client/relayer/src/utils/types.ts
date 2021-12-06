import { EventRecord } from '@polkadot/types/interfaces/system';
import { ApiPromise } from '@polkadot/api';
import { SideEffect } from './../../../types/src/interfaces/primitives/types';
import { XtxId } from './../../../types/src/interfaces/execution_delivery/types';
import type { Hash, AccountId } from '@polkadot/types/interfaces/runtime';
import type { Vec, Compact, u128, StorageKey, Bytes } from '@polkadot/types';
import type { AnyNumber } from '@polkadot/types/types';
import events from 'events';

export interface TransactionResult {
  blockHash: Hash;
  status: boolean;
  // assumption : we relay multiple events
  events: EventRecord[];
}

export interface StorageResult {
  value: string;
  status: boolean;
}

export interface NewSideEffectsAvailableEvent {
  requester: AccountId;
  xtx_id: XtxId;
  sideEffects: Vec<SideEffect>;
}

export declare interface Emitter {
  on(
    event: 'NewSideEffect',
    listener: (payload: NewSideEffectsAvailableEvent, circuitApi: ApiPromise, rococoApi: ApiPromise) => void
  ): this;
}

export class Emitter extends events.EventEmitter {
  emitSideEffect(payload: NewSideEffectsAvailableEvent, circuitApi: ApiPromise, rococoApi: ApiPromise): void {
    this.emit('NewSideEffect', payload, circuitApi, rococoApi);
  }
}

export interface TransferArguments {
  to: AccountId;
  from: AccountId;
  amount: Compact<u128> | AnyNumber | Uint8Array;
}

export interface GetStorageArguments {
  key: StorageKey;
}
